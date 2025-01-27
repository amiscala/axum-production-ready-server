use chrono::{DateTime, Utc};
use crate::domain::models::common::create_uuid_v7;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::domain::models::status::AppStatus;

// Later if it makes sense, add all the part for a user registration itself, now it is just a "placeholder" for later if necessary to create users it will be possible.

pub const USER_SCOPES: &str = "read write admin";
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub email: String,
    pub name: String,
    pub last_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_scopes: String,
    pub status:String
}

impl User {
    pub fn new(email: String, name:String, last_name: String) -> Self {
        let now = Utc::now();
        Self {
            email,
            name,
            last_name,
            created_at: now,
            updated_at: now,
            user_id: create_uuid_v7(),
            user_scopes: USER_SCOPES.to_owned(),
            status: AppStatus::Active.to_string(),
        }
    }
}
