use crate::domain::models::status::AppStatus;
use crate::domain::models::todo::Todo;
use crate::domain::{AppErrors, AppRequest};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub body: String,
    pub category: String,
}

impl AppRequest<CreateTodoRequest> for CreateTodoRequest {
    fn validate(&self) -> Result<(), AppErrors> {
        Ok(())
    }
}

impl CreateTodoRequest {
    pub fn to_model(self, user_id: Uuid) -> Todo {
        Todo::new(
            user_id,
            self.title,
            self.body,
            self.category,
            AppStatus::Created.to_string(),
        )
    }
}
