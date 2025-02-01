use crate::domain::models::client::Client;
use crate::domain::models::user::User;
use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUserAndClientResponse {
    pub user: User,
    pub client: Client,
}
