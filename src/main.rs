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
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use axum_server::tls_rustls::RustlsConfig;
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
    println!("");
    println!("Starting connection to db at {}", migration_url);
    let pool = PgPool::connect(&migration_url).await.expect(&format!("Error while creating migration connection pool with db url {}", migration_url));
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("Start running migrations");
    migrate!("./migrations").run(&pool).await.expect( &format!("Error while running migrations with this db url :{}", migration_url));
    println!("Finished");

    let app_state = Arc::new(AppState {
        database: Postgres(pool),
    });

    let user_router = user_router::route(jwt_config.clone(), app_state.clone());
    let app = user_router
        .merge(authentication_router::route(jwt_config.clone(),app_state.clone()));

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:4000")
    //     .await
    //     .unwrap();
    // tracing::info!("listening on {}", listener.local_addr().unwrap());
    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("server.crt"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("server.key"),
    ).await.unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}