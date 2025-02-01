use crate::api_adapter::{AppErrorResponse, AppSuccessResponse};
use crate::domain::contracts::requests::user::create_user_and_client::CreateUserAndClient;
use crate::domain::contracts::requests::user::update_user_request::UpdateUserRequest;
use crate::domain::contracts::CreateUserAndClientResponse;
use crate::domain::models::queries::Transactions;
use crate::domain::models::user::User;
use crate::domain::AppErrors;
use crate::domain::{CreateUserRequest, JsonExtractor, Queries};
use crate::persistance::database::{execute_query, execute_transaction};
use crate::routers::RouterExtensions;
use crate::{created_response, ok_response, AppState};
use axum::extract::State;
use axum::routing::{delete, get, post, put};
use axum::{Extension, Router};
use axum_production_ready_authorization_macros::require_scopes;
use axum_production_ready_security::{JwtClaims, JwtConfig};
use http::StatusCode;
use std::sync::Arc;

#[axum::debug_handler]
async fn create_user(
    State(state): State<Arc<AppState>>,
    JsonExtractor(request): JsonExtractor<CreateUserRequest>,
) -> Result<AppSuccessResponse<User>, AppErrorResponse> {
    let user = request.to_model();
    let query = Queries::CreateUser { user };
    let res: User = execute_query(&state.database, query).await?;
    created_response!(res)
}

#[axum::debug_handler]
async fn create_user_and_client(
    State(state): State<Arc<AppState>>,
    JsonExtractor(request): JsonExtractor<CreateUserAndClient>,
) -> Result<AppSuccessResponse<CreateUserAndClientResponse>, AppErrorResponse> {
    let (user, client) = request.to_model();
    let transaction = Transactions::CreateClientAndUser {
        user: user.clone(),
        client: client.clone(),
    };
    let () = execute_transaction(&state.database, transaction).await?;
    let response = CreateUserAndClientResponse { user, client };
    ok_response!(response)
}

#[require_scopes("write")]
async fn update_user(
    State(state): State<Arc<AppState>>,
    Extension(jwt_claims): Extension<Arc<JwtClaims>>,
    JsonExtractor(request): JsonExtractor<UpdateUserRequest>,
) -> Result<AppSuccessResponse<User>, AppErrorResponse> {
    let query = Queries::UpdateUser {
        user_id: jwt_claims.sub,
        name: request.name,
        email: request.email,
        last_name: request.last_name,
    };
    let res: User = execute_query(&state.database, query).await?;
    ok_response!(res)
}

#[axum::debug_handler]
#[require_scopes("admin")]
async fn delete_user(
    State(state): State<Arc<AppState>>,
    Extension(jwt_claims): Extension<Arc<JwtClaims>>,
) -> Result<AppSuccessResponse<User>, AppErrorResponse> {
    let query = Queries::DeleteUser {
        user_id: jwt_claims.sub,
    };
    let res: User = execute_query(&state.database, query).await?;
    ok_response!(res)
}

#[axum::debug_handler]
#[require_scopes("read")]
async fn get_user(
    State(state): State<Arc<AppState>>,
    Extension(jwt_claims): Extension<Arc<JwtClaims>>,
) -> Result<AppSuccessResponse<User>, AppErrorResponse> {
    let query = Queries::GetUser {
        user_id: jwt_claims.sub,
    };
    let res: User = execute_query(&state.database, query).await?;

    ok_response!(res)
}

pub fn route(jwt_config: Arc<JwtConfig>, app_state: Arc<AppState>) -> Router {
    let public_router = Router::new()
        .route("/", post(create_user))
        .route("/create-user-and-client", post(create_user_and_client))
        .add_logging()
        .with_state(app_state.clone());

    let private_router = Router::new()
        .route("/", put(update_user))
        .route("/", delete(delete_user))
        .route("/", get(get_user))
        .add_logging_and_security(jwt_config)
        .with_state(app_state);
    let merged = public_router.merge(private_router);
    Router::new().nest("/user", merged)
}
