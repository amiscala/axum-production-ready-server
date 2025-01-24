use crate::AppErrors;
use std::sync::Arc;
use axum::extract::State;
use axum::{Extension, Router};
use axum::routing::{delete, post, put};
use crate::api_adapter::{AppErrorResponse, AppSuccessResponse};
use crate::{created_response, ok_response, AppState};
use crate::domain::{CreateUserRequest, JsonExtractor, Queries};
use crate::domain::models::user::User;
use crate::persistance::database::execute_query;
use http::StatusCode;
use uuid::Uuid;
use axum_production_ready_authorization_macros::require_scopes;
use axum_production_ready_security::{JwtClaims, JwtConfig};
use crate::domain::contracts::requests::common::app_contract::PathExtractor;
use crate::domain::contracts::requests::user::update_user_request::UpdateUserRequest;
use crate::routers::RouterExtensions;

#[axum::debug_handler]
async fn create_user(State(state): State<Arc<AppState>>, JsonExtractor(request, user): JsonExtractor<CreateUserRequest, User>) -> Result<AppSuccessResponse<User>, AppErrorResponse> {
    let query = Queries::CreateUser {
        user
    };
    let res: User = execute_query(&state.database, query).await?;
    created_response!(res)
}

#[require_scopes("write")]
async fn update_user(State(state): State<Arc<AppState>>, JsonExtractor(request, user): JsonExtractor<UpdateUserRequest, UpdateUserRequest>) -> Result<AppSuccessResponse<User>, AppErrorResponse> {
    let query = Queries::UpdateUser {
        user_id: request.user_id,
        name: request.name,
        email: request.email,
        last_name: request.last_name,
    };
    let res: User = execute_query(&state.database, query).await?;
    ok_response!(res)
}

#[axum::debug_handler]
#[require_scopes("admin")]
async fn delete_user(State(state): State<Arc<AppState>>, Extension(jwt_claims): Extension<Arc<JwtClaims>>) -> Result<AppSuccessResponse<User>, AppErrorResponse> {

    let query = Queries::DeleteUser {
        user_id: jwt_claims.sub,
    };
    let res: User = execute_query(&state.database, query).await?;
    ok_response!(res)
}

#[axum::debug_handler]
#[require_scopes("read")]
async fn get_user_by_id(State(state): State<Arc<AppState>>, Extension(jwt_claims): Extension<Arc<JwtClaims>>) -> Result<AppSuccessResponse<User>, AppErrorResponse> {
    let query = Queries::GetUser {
        user_id: jwt_claims.sub,
    };
    let res: User = execute_query(&state.database, query).await?;

    ok_response!(res)
}

pub fn Route(jwt_config: Arc<JwtConfig>, app_state: Arc<AppState>) -> Router {
    let public_router = Router::new()
        .route("/user", post(create_user))
        .add_logging()
        .with_state(app_state.clone());

    let private_router = Router::new()
        .route("/user", put(update_user))
        .route("/user/{user_id}", delete(delete_user))
        .add_logging_and_security(jwt_config)
        .with_state(app_state);
    public_router.merge(private_router)
}


