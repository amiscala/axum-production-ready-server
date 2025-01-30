use serde::{Serialize};
use crate::domain::models::client::Client;
use crate::domain::models::user::User;

#[derive(Serialize)]
pub struct CreateUserAndClientResponse{
    pub user: User,
    pub client: Client
}