use crate::domain::error::app_errors::LibType;
use crate::domain::AppErrors;
use sqlx::{Error, PgPool, Pool, Postgres};

impl From<Error> for AppErrors {
    fn from(value: Error) -> Self {
        match value {
            Error::Database(ref e) => {
                if let Some(pg_error) = e.try_downcast_ref::<sqlx::postgres::PgDatabaseError>() {
                    // Postgresql uses the code 23505 for when it has a PK violation, using this as the value to capture the error
                    // for when there is a insertion conflict
                    if pg_error.code() == "23505" {
                        AppErrors::InsertConflict(format!("Error {}", value))
                    } else {
                        let message =
                            format!("Error {}", value);
                        AppErrors::ExternalLibError {
                            lib_type: LibType::Postgresql,
                            message
                        }
                    }
                } else {
                    let message =
                        format!("Error {}", value);
                    AppErrors::ExternalLibError {
                        lib_type: LibType::Postgresql,
                        message,
                    }
                }
            }
            Error::RowNotFound => {
                AppErrors::RegisterNotFound(format!("Error {}", value))
            }
            _ => {
                let message =
                    format!("Error {}", value);
                AppErrors::ExternalLibError {
                    lib_type: LibType::Postgresql,
                    message,
                }
            }
        }
    }
}