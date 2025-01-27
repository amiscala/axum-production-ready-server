mod api_adapter;
mod domain;
mod persistance;
mod routers;

use crate::domain::CreateUserRequest;
use crate::domain::{AppRequest, Queries};
use crate::domain::create_uuid_v7;
use axum_production_ready_security::{issue_jwt_token, validate_jwt_token, SecurityErrors};
use crate::domain::AppScope;
use axum_production_ready_security::JwtConfig;
use crate::domain::AppErrors;
use crate::api_adapter::AppErrorResponse;
use crate::api_adapter::AppSuccessResponse;
use crate::domain::JsonExtractor;
use axum_production_ready_security::JwtClaims;
use axum_production_ready_security::authentication_middleware;
use axum_production_ready_observability::logging_middleware;
use crate::Database::Postgres;
use axum_production_ready_security::get_jwt_configuration;
use axum_production_ready_observability::ObservabilityGuard;
use axum_production_ready_authorization_macros::require_scopes;
use std::any::type_name;
use axum::routing::get;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{middleware, Extension, Router};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPoolOptions, PgRow};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use axum::extract::{Path, State};
use sqlx::{migrate, PgPool};
use tower::layer::util::Stack;
use tower::ServiceBuilder;
use tracing::{Level};
use uuid::Uuid;
use crate::domain::contracts::requests::common::app_contract::PathExtractor;
use crate::domain::models::status::AppStatus;
use crate::domain::models::user::User;
use crate::persistance::Database;
use crate::persistance::database::execute_query;
use crate::routers::{authentication_router, user_router, RouterExtensions};

#[derive(Clone)]
struct AppState {
    database: Database,
}

#[tokio::main]
async fn main() {
    let _guard = ObservabilityGuard::new(Level::ERROR, "tracer", "http://localhost:4317");
    let jwt_config = Arc::new(
        get_jwt_configuration(
            "/home/arthur/RustroverProjects/production-ready-axum/server/private_key.pem",
            "/home/arthur/RustroverProjects/production-ready-axum/server/public_key.pem",
            1200,
        )
            .unwrap(),
    );
    dotenv::dotenv().ok();
    let migration_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = PgPool::connect(&migration_url).await.expect("Error while creating migration connection pool");
    migrate!("./migrations").run(&pool).await.expect("Error while running migrations");

    let app_state = Arc::new(AppState {
        database: Postgres(pool),
    });

    let public_router: Router<()> = Router::new()
        .route("/token", post(token_issuer))
        .layer(Extension(jwt_config.clone()));
    let user_router = user_router::route(jwt_config.clone(), app_state.clone());
    let app = public_router
        .merge(user_router)
        .merge(authentication_router::route(jwt_config.clone(),app_state.clone()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct InputPayload {
    name: String,
    age: u32,
}

// Define a struct for the JSON response
#[derive(Serialize)]
struct OutputPayload {
    message: String,
    name: String,
    age: u32,
    claims: JwtClaims,
}
#[require_scopes("read", "write")]
async fn handler(

    JsonExtractor(payload): JsonExtractor<MyRequest>,
) -> Result<AppSuccessResponse<OutputPayload>, AppErrorResponse> {
    // let test = fail().await?;
    // let scopes = vec!(AppScope::Read, AppScope::Write);
    // let tok = issue_jwt_token(&jwt_config, create_uuid_v7(), create_uuid_v7(), scopes)?;
    // let res = tok.to_string();
    // let val = validate_jwt_token(&jwt_config, &res)?;
    // "string";
    // Process the input and prepare a response

    created_response!(OutputPayload {
        message: "Foi".to_string(),
        name: payload.name,
        age: 45,
        claims: jwt_claims.0.as_ref().clone()
    })
}

#[derive(Deserialize, Serialize)]
pub struct Test {
    uuid: Uuid,
}
// #[axum::debug_handler]
// async fn path_test(PathExtractor(uuid): PathExtractor<Uuid>) -> Result<AppSuccessResponse<Test>, AppErrorResponse> {
//     ok_response!(Test{uuid})
// }

#[axum::debug_handler]
async fn create_user(State(state): State<Arc<AppState>>, JsonExtractor(request): JsonExtractor<CreateUserRequest>) -> Result<AppSuccessResponse<User>, AppErrorResponse> {
    let user = request.to_model();
    let query = Queries::CreateUser {
        user
    };

    let res: User = execute_query(&state.database,query).await?;
    created_response!(res)
}
#[axum::debug_handler]
async fn token_issuer(
    jwt_config: Extension<Arc<JwtConfig>>,
) -> Result<AppSuccessResponse<OutputPayload>, AppErrorResponse> {
    // let test = fail().await?;
    let scopes = AppScope::Read.to_string();
    let tok = issue_jwt_token(&jwt_config, create_uuid_v7(), create_uuid_v7(), scopes)?;
    let res = tok.to_string();
    let val = validate_jwt_token(&jwt_config, &res)?;

    // "string";
    // Process the input and prepare a response
    created_response!(OutputPayload {
        message: res,
        name: "".to_string(),
        age: 45,
        claims: val
    })
}

/* TEMP BLOCK*/
#[derive(Deserialize)]
pub struct MyRequest {
    pub name: String,
}

impl AppRequest<MyRequest> for MyRequest {
    fn validate(&self) -> Result<(), AppErrors> {
        let mut test = Ok(());
        test = Err(AppErrors::InsertConflict);
        if (self.name == "alou") {
            Err(AppErrors::FailedContractValidation(HashMap::new()))
        } else {
            Ok(())
        }
    }
}

fn test() -> Result<AppSuccessResponse<OutputPayload>, AppErrors> {
    Ok(other()?)
}

fn other() -> Result<AppSuccessResponse<OutputPayload>, AppErrors> {
    Err(OtherError::A)?
}
enum OtherError {
    A,
    B,
    C,
    D,
}

impl From<OtherError> for AppErrors {
    fn from(other: OtherError) -> AppErrors {
        match other {
            _ => AppErrors::InsertConflict("other".to_string()),
        }
    }
}

async fn sleep() {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
}

pub async fn fail() -> Result<PgRow, AppErrors> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/test")
        .await?;
    let row = sqlx::query("SELECT $1 Update")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;
    Ok(row)
}

/*END TEMP BLOCK*/

