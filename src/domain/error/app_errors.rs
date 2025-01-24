use crate::api_adapter::error::{AppErrorResponse, AppErrorStruct,app_error_response::app_error_struct::AppErrorInnerResponse};
use axum::body::Body;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use core::fmt::{Display, Formatter};
use std::collections::HashMap;
use tracing::error_span;

#[derive(Debug)]
pub enum AppErrors {
    InsertConflict(String),
    RegisterNotFound(String),
    FailedContractValidation(HashMap<String, String>),
    JsonParseError(String),
    Unauthorized(String),
    MissingAuthorizationHeader(String),
    WrongAuthorizationHeader(String),
    Forbidden(String),
    ExternalLibError {
        lib_type: LibType,
        message: String,
    },
    FromStringToAppScopeConversionError(String)
}
#[derive(Debug)]
pub enum LibType {
    Postgresql,
    JWT,
    StdSystemTime,
    StdIO
}
impl From<AppErrors> for AppErrorResponse {
    fn from(value: AppErrors) -> Self {
        let code_as_string: String = value.to_string();
        const ERROR_MESSAGE: &str = "Error while handling request";
        match value {
            AppErrors::InsertConflict(val) => {
                error_span!(
                    "Error while handling request",
                    message = val,
                    code = code_as_string
                );
                AppErrorResponse::AppBusinessError(AppErrorStruct::from_single_error(
                    &code_as_string,
                    "Duplicated correlation_key",
                    StatusCode::CONFLICT,
                ))
            }
            AppErrors::FailedContractValidation(message) => {
                error_span!(
                    ERROR_MESSAGE,
                    message = "Errors on response log".to_string(),
                    code = &code_as_string
                );
                let errors = message
                    .into_iter()
                    .map(|(key, value)| AppErrorInnerResponse::new(key, value))
                    .collect();
                AppErrorResponse::AppBusinessError(AppErrorStruct::new(
                    errors,
                    StatusCode::BAD_REQUEST,
                ))
            }
            AppErrors::JsonParseError(val) => {
                error_span!(ERROR_MESSAGE, message = val, code = &code_as_string);
                AppErrorResponse::AppContractError(AppErrorStruct::from_single_error(
                    &code_as_string,
                    &val,
                    StatusCode::BAD_REQUEST,
                ))
            }
            AppErrors::Unauthorized(val) => {
                error_span!(ERROR_MESSAGE, message = val, code = &code_as_string);
                AppErrorResponse::AppBusinessError(AppErrorStruct::from_single_error(
                    &code_as_string,
                    &code_as_string,
                    StatusCode::UNAUTHORIZED,
                ))
            }
            AppErrors::Forbidden(val) => {
                error_span!(ERROR_MESSAGE, message = val, code = &code_as_string);
                AppErrorResponse::AppBusinessError(AppErrorStruct::from_single_error(
                    &code_as_string,
                    &code_as_string,
                    StatusCode::FORBIDDEN,
                ))
            }
            AppErrors::ExternalLibError { lib_type: infrastructure_type, message} => {
                let message = format!("Type {}: Error: {}", infrastructure_type, message);
                error_span!(ERROR_MESSAGE, message = message, code = &code_as_string);
                AppErrorResponse::AppBusinessError(AppErrorStruct::from_single_error(
                    &code_as_string,
                    &code_as_string,
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
            AppErrors::RegisterNotFound(val) => {
                error_span!(ERROR_MESSAGE, message = val, code = &code_as_string);
                AppErrorResponse::AppBusinessError(AppErrorStruct::from_single_error(
                    &code_as_string,
                    &code_as_string,
                    StatusCode::FORBIDDEN,
                ))
            }
            AppErrors::MissingAuthorizationHeader(val) => {
                error_span!(ERROR_MESSAGE, message = val, code = &code_as_string);
                AppErrorResponse::AppBusinessError(AppErrorStruct::from_single_error(
                    &code_as_string,
                    &code_as_string,
                    StatusCode::FORBIDDEN,
                ))
            }
            AppErrors::WrongAuthorizationHeader(val) => {
                error_span!(ERROR_MESSAGE, message = val, code = &code_as_string);
                AppErrorResponse::AppBusinessError(AppErrorStruct::from_single_error(
                    &code_as_string,
                    &code_as_string,
                    StatusCode::FORBIDDEN,
                ))
            }
            AppErrors::FromStringToAppScopeConversionError(val) => {
                error_span!(ERROR_MESSAGE, message = val, code = &code_as_string);
                AppErrorResponse::AppBusinessError(AppErrorStruct::from_single_error(
                    "InternalServerError",
                    "InternalServerError",
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        }
    }
}

impl Display for AppErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let variant_name = match self {
            AppErrors::InsertConflict(_) => "InsertConflict",
            AppErrors::FailedContractValidation(_) => "FailedContractValidation",
            AppErrors::JsonParseError(_) => "JsonParseError",
            AppErrors::Unauthorized(_) => "Unauthorized",
            AppErrors::Forbidden(_) => "Forbidden",
            AppErrors::RegisterNotFound(_) => "RegisterNotFound",
            AppErrors::ExternalLibError { .. } => "InfrastructureError",
            AppErrors::MissingAuthorizationHeader(_) => "MissingAuthorizationHeader",
            AppErrors::WrongAuthorizationHeader(_) => "WrongAuthorizationHeader",
            &AppErrors::FromStringToAppScopeConversionError(_) => "FromStringToAppScopeConversionError"
        };
        write!(f, "{}", variant_name)
    }
}

impl Display for LibType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let variant_name = match self {
            LibType::Postgresql => "Postgresql",
            LibType::JWT => "JWT",
            LibType::StdSystemTime => "StdSystemTime",
            &LibType::StdIO => "StdIO"
        };
        write!(f, "{}", variant_name)
    }
}
