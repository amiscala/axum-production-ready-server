use crate::api_adapter::{AppErrorResponse, AppSuccessResponse};
use crate::domain::contracts::requests::client::create_client_request::CreateClientRequest;
use crate::domain::contracts::requests::client::update_client_request::UpdateClientRequest;
use crate::domain::contracts::requests::common::app_contract::PathExtractor;
use crate::domain::models::client::Client;
use crate::domain::models::queries::VecQueries;
use crate::domain::AppErrors;
use crate::domain::{JsonExtractor, Queries};
use crate::persistance::database::{execute_query, execute_vec_query};
use crate::routers::RouterExtensions;
use crate::{created_response, ok_response, AppState};
use axum::extract::State;
use axum::routing::{delete, get, post, put};
use axum::{Extension, Router};
use axum_production_ready_authorization_macros::require_scopes;
use axum_production_ready_security::{JwtClaims, JwtConfig};
use http::StatusCode;
use std::sync::Arc;
use uuid::Uuid;

#[require_scopes("admin")]
#[axum::debug_handler]
async fn create_client(
    State(state): State<Arc<AppState>>,
    Extension(jwt_claims): Extension<Arc<JwtClaims>>,
    JsonExtractor(request): JsonExtractor<CreateClientRequest>,
) -> Result<AppSuccessResponse<Client>, AppErrorResponse> {
    let client = request.to_model(jwt_claims.sub);
    let query = Queries::CreateClient { client };
    let res: Client = execute_query(&state.database, query).await?;
    created_response!(res)
}

#[require_scopes("admin")]
#[axum::debug_handler]
async fn update_client(
    State(state): State<Arc<AppState>>,
    Extension(jwt_claims): Extension<Arc<JwtClaims>>,
    JsonExtractor(request): JsonExtractor<UpdateClientRequest>,
) -> Result<AppSuccessResponse<Client>, AppErrorResponse> {
    let query = Queries::UpdateClient {
        client_id: request.client_id,
        client_scopes: request.client_scopes,
        client_description: request.client_description,
        expires_at: request.expires_at,
        user_id: jwt_claims.sub,
    };
    let res: Client = execute_query(&state.database, query).await?;
    ok_response!(res)
}

#[require_scopes("admin")]
#[axum::debug_handler]
async fn delete_client(
    State(state): State<Arc<AppState>>,
    Extension(jwt_claims): Extension<Arc<JwtClaims>>,
    PathExtractor(client_id): PathExtractor<Uuid>,
) -> Result<AppSuccessResponse<Client>, AppErrorResponse> {
    let query = Queries::DeleteClient {
        client_id,
        user_id: jwt_claims.sub,
    };
    let res: Client = execute_query(&state.database, query).await?;
    ok_response!(res)
}

#[require_scopes("admin")]
#[axum::debug_handler]
async fn get_client(
    State(state): State<Arc<AppState>>,
    Extension(jwt_claims): Extension<Arc<JwtClaims>>,
    PathExtractor(client_id): PathExtractor<Uuid>,
) -> Result<AppSuccessResponse<Client>, AppErrorResponse> {
    let query = Queries::GetClient {
        client_id,
        user_id: jwt_claims.sub,
    };
    let res: Client = execute_query(&state.database, query).await?;
    ok_response!(res)
}

#[require_scopes("admin")]
#[axum::debug_handler]
async fn get_clients(
    State(state): State<Arc<AppState>>,
    Extension(jwt_claims): Extension<Arc<JwtClaims>>,
) -> Result<AppSuccessResponse<Vec<Client>>, AppErrorResponse> {
    let query = VecQueries::GetClients {
        user_id: jwt_claims.sub,
    };
    let res: Vec<Client> = execute_vec_query(&state.database, query).await?;
    ok_response!(res)
}

pub fn route(jwt_config: Arc<JwtConfig>, app_state: Arc<AppState>) -> Router {
    let private_router = Router::new()
        .route("/", post(create_client))
        .route("/", put(update_client))
        .route("/{client_id}", delete(delete_client))
        .route("/{client_id}", get(get_client))
        .route("/clients", get(get_clients))
        .add_logging_and_security(jwt_config)
        .with_state(app_state);
    Router::new().nest("/client", private_router)
}
