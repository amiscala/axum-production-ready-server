use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::Serialize;
use crate::api_adapter::CustomHttpStatusCode;

#[derive(Serialize)]
pub struct AppSuccessResponse<T>
where
    T: Serialize,
{
    pub body: T,
    pub status_code: CustomHttpStatusCode
}

impl<T> AppSuccessResponse<T>
where T: Serialize
{
    pub fn new (body: T, status_code: StatusCode) -> AppSuccessResponse<T>{
        Self{
            body,
            status_code: CustomHttpStatusCode(status_code)
        }
    }
}

impl<T> IntoResponse for AppSuccessResponse<T>
where T: Serialize
{
    fn into_response(self) -> Response {
        (self.status_code.0, Json(self)).into_response()
    }
}