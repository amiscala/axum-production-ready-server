use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter};

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