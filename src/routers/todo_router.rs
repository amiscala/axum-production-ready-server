use crate::api_adapter::{AppErrorResponse, AppSuccessResponse};
use crate::domain::contracts::requests::common::app_contract::PathExtractor;
use crate::domain::contracts::requests::todo::create_todo_request::CreateTodoRequest;
use crate::domain::contracts::requests::todo::update_todo_request::UpdateTodoRequest;
use crate::domain::models::queries::VecQueries;
use crate::domain::models::todo::Todo;
use crate::domain::Queries;
use crate::domain::{AppErrors, JsonExtractor};
use crate::persistance::database::{execute_query, execute_vec_query};
use crate::routers::RouterExtensions;
use crate::{created_response, ok_response, AppState};
use axum::extract::State;
use axum::routing::{delete, get, post, put};
use axum::{Extension, Router};
use axum_production_ready_authorization_macros::require_scopes;
use axum_production_ready_security::{JwtClaims, JwtConfig};
use http::StatusCode;
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;

#[require_scopes("write")]
#[axum::debug_handler]
async fn create_todo(
    State(state): State<Arc<AppState>>,
    Extension(jwt_claims): Extension<Arc<JwtClaims>>,
    JsonExtractor(request): JsonExtractor<CreateTodoRequest>,
) -> Result<AppSuccessResponse<Todo>, AppErrorResponse> {
    let todo = request.to_model(jwt_claims.sub);
    let query = Queries::CreateTodo { todo };
    let res: Todo = execute_query(&state.database, query).await?;
    created_response!(res)
}
#[require_scopes("write")]
#[axum::debug_handler]
async fn update_todo(
    State(state): State<Arc<AppState>>,
    Extension(jwt_claims): Extension<Arc<JwtClaims>>,
    JsonExtractor(request): JsonExtractor<UpdateTodoRequest>,
) -> Result<AppSuccessResponse<Todo>, AppErrorResponse> {
    let query = Queries::UpdateTodo {
        todo_id: request.todo_id,
        user_id: jwt_claims.sub,
        title: request.title,
        body: request.body,
        category: request.category,
        status: request.status,
    };
    let res: Todo = execute_query(&state.database, query).await?;
    ok_response!(res)
}

#[require_scopes("write")]
#[axum::debug_handler]
async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Extension(jwt_claims): Extension<Arc<JwtClaims>>,
    PathExtractor(todo_id): PathExtractor<Uuid>,
) -> Result<AppSuccessResponse<Todo>, AppErrorResponse> {
    let query = Queries::DeleteTodo {
        todo_id,
        user_id: jwt_claims.sub,
    };
    let res: Todo = execute_query(&state.database, query).await?;
    ok_response!(res)
}

#[require_scopes("read")]
#[axum::debug_handler]
async fn get_todo(
    State(state): State<Arc<AppState>>,
    Extension(jwt_claims): Extension<Arc<JwtClaims>>,
    PathExtractor(todo_id): PathExtractor<Uuid>,
) -> Result<AppSuccessResponse<Todo>, AppErrorResponse> {
    let query = Queries::GetTodo {
        todo_id,
        user_id: jwt_claims.sub,
    };
    let res: Todo = execute_query(&state.database, query).await?;
    ok_response!(res)
}

#[require_scopes("read")]
#[axum::debug_handler]
async fn get_todos(
    State(state): State<Arc<AppState>>,
    Extension(jwt_claims): Extension<Arc<JwtClaims>>,
) -> Result<AppSuccessResponse<Vec<Todo>>, AppErrorResponse> {
    let query = VecQueries::GetTodos {
        user_id: jwt_claims.sub,
    };
    let res: Vec<Todo> = execute_vec_query(&state.database, query).await?;
    ok_response!(res)
}

pub fn route(jwt_config: Arc<JwtConfig>, app_state: Arc<AppState>) -> Router {
    let private_router = Router::new()
        .route("/", post(create_todo))
        .route("/", put(update_todo))
        .route("/{todo_id}", delete(delete_todo))
        .route("/{todo_id}", get(get_todo))
        .route("/todos", get(get_todos))
        .add_logging_and_security(jwt_config)
        .with_state(app_state);
    Router::new().nest("/todo", private_router)
}
