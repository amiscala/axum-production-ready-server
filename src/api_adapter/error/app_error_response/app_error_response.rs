use http::StatusCode;
use crate::api_adapter::AppErrorStruct;
use axum::extract::rejection::{FormRejection, JsonRejection, PathRejection, QueryRejection};
use tracing::error_span;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use axum_production_ready_security::SecurityErrors;

pub enum AppErrorResponse {
    // JsonRejection is a wrapper for the Axum JsonRejection error, has this value to be able to handle correctly the errors
    JsonRejection(JsonRejection),
    FormRejection(FormRejection),
    PathRejection(PathRejection),
    QueryRejection(QueryRejection),
    AppContractError(AppErrorStruct),
    AppBusinessError(AppErrorStruct),
}
impl IntoResponse for AppErrorResponse {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        match self {
            AppErrorResponse::JsonRejection(rejection) => {
                (rejection.status(), Json(AppErrorStruct::from_single_error("JsonRejection", &rejection.body_text(), StatusCode::BAD_REQUEST))).into_response()
            }
            AppErrorResponse::AppContractError(app_error) => {
                (StatusCode::BAD_REQUEST, Json(app_error)).into_response()
            }
            AppErrorResponse::AppBusinessError(app_error_response) => {
                // error_span!("test", message=app_error_response.error.message, code=app_error_response.error.code);
                (app_error_response.status_code.0, Json(app_error_response)).into_response()
            }
            AppErrorResponse::FormRejection(rejection) => {
                (rejection.status(), Json(AppErrorStruct::from_single_error("FormRejection", &rejection.body_text(), StatusCode::BAD_REQUEST))).into_response()
            }
            AppErrorResponse::PathRejection(rejection) => {
                (rejection.status(), Json(AppErrorStruct::from_single_error("PathRejection", &rejection.body_text(), StatusCode::BAD_REQUEST))).into_response()
            }
            AppErrorResponse::QueryRejection(rejection) => {
                (rejection.status(), Json(AppErrorStruct::from_single_error("QueryRejection", &rejection.body_text(), StatusCode::BAD_REQUEST))).into_response()
            }
        }
    }
}

impl From<JsonRejection> for AppErrorResponse {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}

impl From<FormRejection> for AppErrorResponse {
    fn from(rejection: FormRejection) -> Self {
        Self::FormRejection(rejection)
    }
}

impl From<PathRejection> for AppErrorResponse {
    fn from(value: PathRejection) -> Self {
        Self::PathRejection(value)
    }
}
impl From<QueryRejection> for AppErrorResponse {
    fn from(value: QueryRejection) -> Self {
        Self::QueryRejection(value)
    }
}

impl From<SecurityErrors> for AppErrorResponse {
    fn from(value: SecurityErrors) -> Self {
        let code_as_string: String = value.to_string();
        const ERROR_MESSAGE: &str = "Error while handling request";
        match value {
            SecurityErrors::Unauthorized(val) => {
                error_span!(ERROR_MESSAGE, message = val, code = &code_as_string);
                AppErrorResponse::AppBusinessError(AppErrorStruct::from_single_error(
                    &code_as_string,
                    &code_as_string,
                    StatusCode::UNAUTHORIZED,
                ))
            }
            SecurityErrors::Forbidden(val) => {
                error_span!(ERROR_MESSAGE, message = val, code = &code_as_string);
                AppErrorResponse::AppBusinessError(AppErrorStruct::from_single_error(
                    &code_as_string,
                    &code_as_string,
                    StatusCode::FORBIDDEN,
                ))
            }
            SecurityErrors::MissingAuthorizationHeader(val) => {
                error_span!(ERROR_MESSAGE, message = val, code = &code_as_string);
                AppErrorResponse::AppBusinessError(AppErrorStruct::from_single_error(
                    &code_as_string,
                    &code_as_string,
                    StatusCode::FORBIDDEN,
                ))
            }
            SecurityErrors::GenericError(val) => {
                error_span!(ERROR_MESSAGE, message = val, code = &code_as_string);
                AppErrorResponse::AppBusinessError(AppErrorStruct::from_single_error(
                    &code_as_string,
                    &code_as_string,
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        }
    }
}