use crate::domain::{AppErrors, AppRequest};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateTodoRequest {
    pub todo_id: Uuid,
    pub title: String,
    pub category: String,
    pub body: String,
    pub status: String,
}

impl AppRequest<UpdateTodoRequest> for UpdateTodoRequest {
    fn validate(&self) -> Result<(), AppErrors> {
        Ok(())
    }
}
