use crate::domain::models::common::create_uuid_v7;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct Todo {
    pub todo_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub body: String,
    pub category: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: String,
}

impl Todo {
    pub fn new(
        user_id: Uuid,
        title: String,
        body: String,
        category: String,
        status: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            todo_id: create_uuid_v7(),
            user_id,
            title,
            body,
            category,
            created_at: now,
            updated_at: now,
            status,
        }
    }
}
