mod api_adapter;
mod domain;
mod persistance;
mod routers;

use crate::persistance::Database;
use crate::persistance::Database::Postgres;
use crate::routers::{authentication_router, client_router, todo_router, user_router};
use axum_production_ready_observability::ObservabilityGuard;
use axum_production_ready_security::get_jwt_configuration;
use axum_server::tls_rustls::RustlsConfig;
use sqlx::{migrate, PgPool};
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::Level;

#[derive(Clone)]
struct AppState {
    database: Database,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("is the env file on the project root?");
    let otel_url = env::var("OTLP_COLLECTOR_URL").expect("OTLP_COLLECTOR_URL not set");
    let _guard = ObservabilityGuard::new(Level::INFO, "tracer", otel_url);
    let jwt_config = Arc::new(
        get_jwt_configuration(
            &format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "private_key.pem"),
            &format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "public_key.pem"),
            1200,
        )
        .expect(&format!("Working dir: {}", env!("CARGO_MANIFEST_DIR"))),
    );

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = PgPool::connect(&db_url).await.expect(&format!(
        "Error while creating migration connection pool with db url {}",
        db_url
    ));
    migrate!("./migrations").run(&pool).await.expect(&format!(
        "Error while running migrations with this db url :{}",
        db_url
    ));

    let app_state = Arc::new(AppState {
        database: Postgres(pool),
    });

    let user_router = user_router::route(jwt_config.clone(), app_state.clone());
    let app = user_router
        .merge(authentication_router::route(
            jwt_config.clone(),
            app_state.clone(),
        ))
        .merge(client_router::route(jwt_config.clone(), app_state.clone()))
        .merge(todo_router::route(jwt_config.clone(), app_state.clone()));

    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("server.crt"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("server.key"),
    )
    .await
    .unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
