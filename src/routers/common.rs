use crate::api_adapter::AppErrorResponse;
use axum::{middleware, Extension, Router};
use axum_production_ready_observability::logging_middleware;
use axum_production_ready_security::{authentication_middleware, JwtConfig};
use std::sync::Arc;
use tower::ServiceBuilder;


pub trait RouterExtensions<S>
{
    // Used when want to guard the router
    fn add_logging_and_security(self, jwt_config: Arc<JwtConfig>) -> Router<S>;
    // Used for when the router is public
    fn add_logging(self) -> Router<S>;
    fn add_logging_and_jwt_config(self,jwt_config: Arc<JwtConfig>) -> Router<S>;
}

impl<S> RouterExtensions<S> for Router<S>
where
    S: Send + Sync + Clone + 'static,
{
    fn add_logging_and_security(self, jwt_config: Arc<JwtConfig>) -> Router<S> {
        self.layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(logging_middleware))
                .layer(Extension(jwt_config))
                .layer(middleware::from_fn(authentication_middleware::<AppErrorResponse>))
        )
    }

    fn add_logging(self) -> Router<S> {
        self.layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(logging_middleware)))
    }
    fn add_logging_and_jwt_config(self,jwt_config: Arc<JwtConfig>) -> Router<S> {
        self.layer(
            ServiceBuilder::new()
                .layer(Extension(jwt_config))
                .layer(middleware::from_fn(logging_middleware)))
    }
}

// pub fn add_logging_and_auth_layers(jwt_config: Arc<JwtConfig>) -> ServiceBuilder<T> {
//     ServiceBuilder::new()
//         .layer(middleware::from_fn(logging_middleware))
//         .layer(Extension(jwt_config))
//         .layer(middleware::from_fn(authentication_middleware::<AppErrorResponse>))
// }