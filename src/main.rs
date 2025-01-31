mod api_adapter;
mod domain;
mod persistance;
mod routers;

use crate::domain::CreateUserRequest;
use crate::domain::{Queries};
use crate::domain::AppScope;
use crate::api_adapter::AppErrorResponse;
use crate::api_adapter::AppSuccessResponse;
use crate::domain::JsonExtractor;
use axum_production_ready_security::get_jwt_configuration;
use axum_production_ready_observability::ObservabilityGuard;
use axum::http::StatusCode;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use axum::extract::State;
use sqlx::{migrate, PgPool};
use tracing::{info_span, Level};
use crate::domain::models::user::User;
use crate::persistance::Database;
use crate::persistance::database::execute_query;
use crate::persistance::Database::Postgres;
use crate::routers::{authentication_router, user_router};

#[derive(Clone)]
struct AppState {
    database: Database,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("is the env file on the project root?");
    let _guard = ObservabilityGuard::new(Level::INFO, "tracer", "http://localhost:4317");
    let jwt_config = Arc::new(
        get_jwt_configuration(
            &format!("{}/{}", env!("CARGO_MANIFEST_DIR"),"private_key.pem"),
            &format!("{}/{}", env!("CARGO_MANIFEST_DIR"),"public_key.pem"),
            1200,
        )
            .expect(&format!("Working dir: {}", env!("CARGO_MANIFEST_DIR"))),
    );

    let migration_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = PgPool::connect(&migration_url).await.expect("Error while creating migration connection pool");
    migrate!("./migrations").run(&pool).await.expect("Error while running migrations");

    let app_state = Arc::new(AppState {
        database: Postgres(pool),
    });

    let user_router = user_router::route(jwt_config.clone(), app_state.clone());
    let app = user_router
        .merge(authentication_router::route(jwt_config.clone(),app_state.clone()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()
}