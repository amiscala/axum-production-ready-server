use crate::api_adapter::{AppErrorResponse, AppSuccessResponse};
use crate::domain::contracts::requests::common::app_contract::FormExtractor;
use crate::domain::contracts::responses::TokenResponse;
use crate::domain::contracts::IssueTokenRequest;
use crate::domain::models::client::Client;
use crate::domain::models::common::string_to_sha_256;
use crate::domain::{AppErrors, Queries};
use crate::persistance::database::execute_query;
use crate::routers::RouterExtensions;
use crate::{ok_response, AppState};
use axum::extract::State;
use axum::routing::post;
use axum::{Extension, Router};
use axum_production_ready_security::{issue_jwt_token, JwtConfig};
use http::StatusCode;
use std::sync::Arc;

#[axum::debug_handler]
async fn authenticate(
    jwt_config: Extension<Arc<JwtConfig>>,
    State(state): State<Arc<AppState>>,
    FormExtractor(token_request): FormExtractor<IssueTokenRequest>,
) -> Result<AppSuccessResponse<TokenResponse>, AppErrorResponse> {
    let client_query = Queries::GetClientWithClientIdAndClientSecret {
        client_id: token_request.client_id.clone(),
        client_secret: string_to_sha_256(token_request.client_secret.clone()),
    };
    let client: Client = execute_query(&state.database, client_query).await?;

    let mut requested_scopes = token_request.scope.split(" ");
    let mut client_scopes = client.client_scopes.split(" ");

    if !requested_scopes
        .all(|requested_scope| client_scopes.any(|client_scope| client_scope == requested_scope))
    {
        return Err(AppErrors::InvalidScopes(format!(
            "Requested Scopes: {}, available scopes: {}",
            token_request.scope, client.client_scopes
        )))?;
    }

    let token = issue_jwt_token(
        &jwt_config,
        token_request.client_id,
        client.user_id,
        token_request.scope.clone(),
    )?;
    ok_response!(TokenResponse {
        scope: token_request.scope,
        access_token: token.to_string(),
        token_type: "Bearer".to_owned(),
        expires_in: jwt_config.token_expiration_in_seconds
    })
}

pub fn route(jwt_config: Arc<JwtConfig>,app_state: Arc<AppState>) -> Router{
    let public = Router::new()
        .route("/token", post(authenticate))
        .add_logging_and_jwt_config(jwt_config)
        .with_state(app_state);
    Router::new().nest("/authentication", public)
}
