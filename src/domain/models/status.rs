use crate::domain::AppErrors;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize, Clone, Type)]
pub enum AppStatus {
    Active,
    Inactive,
    Error,
    Deleted,
    Created,
    Completed,
}

impl Display for AppStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let variant_name = match self {
            AppStatus::Active => "ACTIVE",
            AppStatus::Inactive => "INACTIVE",
            AppStatus::Error => "ERROR",
            AppStatus::Deleted => "DELETED",
            AppStatus::Created => "CREATED",
            AppStatus::Completed => "COMPLETED",
        };
        write!(f, "{}", variant_name)
    }
}

impl FromStr for AppStatus {
    type Err = AppErrors;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "ACTIVE" => Ok(AppStatus::Active),
            "INACTIVE" => Ok(AppStatus::Inactive),
            "ERROR" => Ok(AppStatus::Error),
            "DELETED" => Ok(AppStatus::Deleted),
            "CREATED" => Ok(AppStatus::Created),
            "COMPLETED" => Ok(AppStatus::Completed),
            _ => Err(AppErrors::InvalidScopes(input.to_string())),
        }
    }
}
