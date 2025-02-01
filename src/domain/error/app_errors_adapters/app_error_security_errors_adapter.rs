use crate::domain::error::app_errors::LibType;
use crate::domain::AppErrors;
use axum_production_ready_security::SecurityErrors;

impl From<SecurityErrors> for AppErrors {
    fn from(value: SecurityErrors) -> Self {
        match value {
            SecurityErrors::Unauthorized(val) => AppErrors::Unauthorized(val),
            SecurityErrors::Forbidden(val) => AppErrors::Forbidden(val),
            SecurityErrors::MissingAuthorizationHeader(val) => {
                AppErrors::MissingAuthorizationHeader(val)
            }
            SecurityErrors::GenericError(val) => AppErrors::ExternalLibError {
                lib_type: LibType::JWT,
                message: val.to_string(),
            },
        }
    }
}
