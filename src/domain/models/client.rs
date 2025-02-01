use crate::domain::models::common::{create_uuid_v7, generate_client_secret};
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::models::status::AppStatus;

#[derive(Deserialize, Serialize, Clone, FromRow)]
pub struct Client {
    pub client_id: Uuid,
    pub client_description: String,
    pub user_id: Uuid,
    pub client_secret: String,
    pub client_scopes: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: String,
}

impl Client {
    pub fn new(
        user_id: Uuid,
        client_scopes: String,
        client_description: String,
        expires_at: Option<DateTime<Utc>>,
    ) -> Self {
        let now = Utc::now();
        Self {
            client_id: create_uuid_v7(),
            client_description,
            user_id,
            client_scopes,
            client_secret: generate_client_secret(),
            expires_at,
            updated_at: now,
            created_at: now,
            status: AppStatus::Active.to_string(),
        }
    }
}
