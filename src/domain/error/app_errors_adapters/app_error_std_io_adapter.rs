use std::io::Error;
use crate::domain::AppErrors;
use crate::domain::error::app_errors::LibType;

impl From<std::io::Error> for AppErrors{
    fn from(value: Error) -> Self {
        match value {
            _ => AppErrors::ExternalLibError {
                lib_type: LibType::StdIO,
                message: format!("{}", value),
            }
        }
    }
}