use std::future::Future;
use std::sync::Arc;
use crate::api_adapter::{AppErrorResponse};
use crate::domain::AppErrors;
use axum::extract::rejection::{FormRejection, JsonRejection, PathRejection, QueryRejection};
use axum::extract::{FromRef, FromRequest, FromRequestParts, Path, Query, Request};
use axum::{Form, Json};
use http::request::Parts;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::AppState;

// Using option instead of return because if the return is valid, there will be no action to take.
pub trait AppRequest<R, M>
{
    fn validate(&self) -> Result<(), AppErrors>;
    fn build_model(&self) -> Result<M, AppErrors>;
}


pub struct JsonExtractor<R,M>(pub R, pub M);
impl<R,M, S> FromRequest<S> for JsonExtractor<R,M>
where
    R: AppRequest<R,M>,
    S: Send + Sync,
    Json<R>: FromRequest<S, Rejection = JsonRejection>,
    Arc<AppState>: FromRef<S>
{
    type Rejection = AppErrorResponse;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(val) = Json::<R>::from_request(req, state).await?;
        val.validate()?;
        let model = val.build_model()?;
        Ok(JsonExtractor(val, model))
    }
}

pub struct FormExtractor<R,M>(pub R, pub M);
impl<R,M, S> FromRequest<S> for FormExtractor<R,M>
where
    R: AppRequest<R,M>,
    S: Send + Sync,
    Form<R>: FromRequest<S, Rejection = FormRejection>,
    Arc<AppState>: FromRef<S>
{
    type Rejection = AppErrorResponse;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(val) = Form::<R>::from_request(req, state).await?;
        val.validate()?;
        let model = val.build_model()?;
        Ok(FormExtractor(val, model))
    }
}

pub struct PathExtractor<R>(pub R);
impl<R, S> FromRequestParts<S> for PathExtractor<R>
where
    R: DeserializeOwned + Send,
    S: Send + Sync,
    Path<R>: FromRequestParts<S, Rejection = PathRejection>,
    Arc<AppState>: FromRef<S>
{
    type Rejection = AppErrorResponse;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(val) = Path::<R>::from_request_parts(parts, state).await?;
        Ok(PathExtractor(val))
    }
}

pub struct QueryExtractor<R>(pub R);
impl<R,S> FromRequestParts<S> for QueryExtractor<R>
where
    S: Send + Sync,
    Query<R>: FromRequestParts<S, Rejection = QueryRejection>,
    Arc<AppState>: FromRef<S>
{
    type Rejection = AppErrorResponse;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection>
    {
        let Query(val) = Query::<R>::from_request_parts(parts, state).await?;
        Ok(Self(val))
    }
}