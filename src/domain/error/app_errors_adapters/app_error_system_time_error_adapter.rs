use std::time::SystemTimeError;
use crate::domain::AppErrors;
use crate::domain::error::app_errors::LibType;

impl From<SystemTimeError> for AppErrors{
    fn from(value: SystemTimeError) -> Self {
        match value {
            _system_time_error => {
                AppErrors::ExternalLibError {
                    lib_type: LibType::StdSystemTime,
                    message: format!("Error while getting System Time")
                }
            }
        }
    }
}