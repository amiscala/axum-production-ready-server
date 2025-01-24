use core::fmt::{Display, Formatter};
use core::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::domain::AppErrors;

#[derive(Debug,Serialize, Deserialize, Clone, PartialEq)]
pub enum AppScope {
    Read,
    Write,
    Admin,
    All,
}

impl FromStr for AppScope {
    type Err = AppErrors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "read" => Ok(AppScope::Read),
            "write" => Ok(AppScope::Write),
            "admin" => Ok(AppScope::Admin),
            _ => {
                let message = format!("Could not find enum for value {}",{s});
                Err(AppErrors::FromStringToAppScopeConversionError(message))
            }
        }
    }
}


impl Display for AppScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let value = match self {
            AppScope::Read => {
                "read"
            }
            AppScope::Write => {
                "write"
            }
            AppScope::Admin => {
                "admin"
            },
            &AppScope::All => {
                "all"
            },

        };
        write!(f, "{}", value)
    }
}