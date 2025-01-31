use crate::domain::AppErrors;
use axum_production_ready_security::JwtClaims;
use axum::Extension;
use axum::extract::State;
use std::sync::Arc;
use crate::api_adapter::{AppErrorResponse, AppSuccessResponse};
use crate::{created_response, AppState};
use http::StatusCode;
use axum_production_ready_authorization_macros::require_scopes;
use crate::domain::{CreateUserRequest, JsonExtractor, Queries};
use crate::domain::models::user::User;
use crate::persistance::database::execute_query;


#[require_scopes("admin")]
#[axum::debug_handler]
async fn create_user(State(state): State<Arc<AppState>>, JsonExtractor(request): JsonExtractor<CreateUserRequest>) -> Result<AppSuccessResponse<User>, AppErrorResponse> {
    let user = request.to_model();
    let query = Queries::CreateUser {
        user
    };

    let res: User = execute_query(&state.database,query).await?;
    created_response!(res)
}