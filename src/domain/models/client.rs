use crate::domain::models::common::{generate_client_secret, create_uuid_v7};
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::models::status::AppStatus;

#[derive(Deserialize, Serialize, Clone)]
pub struct Client {
    client_id: Uuid,
    client_description: String,
    user_id: Uuid,
    client_secret: String,
    client_scopes: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    expires_at: Option<DateTime<Utc>>,
    status: String,
}

impl Client {
    pub fn new(user_id: Uuid, client_scopes: String, client_description: String, expires_at: Option<DateTime<Utc>>) -> Self {
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
