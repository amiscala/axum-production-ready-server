use crate::domain::models::status::AppStatus;
use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;
use crate::domain::models::client::Client;
use crate::domain::models::user::User;

pub enum Queries {
    CreateUser {
        user: User,
    },
    UpdateUser {
        user_id: Uuid,
        email: String,
        name: String,
        last_name: String,
        updated_at: DateTime<Utc>,
    },
    DeleteUser {
        user_id: Uuid,
    },
    GetUser {
        user_id: Uuid,
    },
    CreateClientAndUser{
        user: User,
        client: Client,
    },
    CreateClient {
        client: Client,
    },
    UpdateClient {
        client: Client,
        status: AppStatus,
    },
    DeleteClient {
        user_id: Uuid,
        client: Uuid,
    },
    GetClient {
        user_id: Uuid,
        client: Uuid,
    },
    GetClients {
        user_id: Uuid,
    },
}
