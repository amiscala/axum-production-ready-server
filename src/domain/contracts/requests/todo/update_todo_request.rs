use crate::domain::contracts::requests::common::validation_helpers::{
    validate_scopes, validate_status,
};
use crate::domain::contracts::requests::common::{validate_regex, Regexes};
use crate::domain::{AppErrors, AppRequest};
use serde::Deserialize;
use std::collections::HashMap;
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
        let mut validation_errors: HashMap<String, String> = HashMap::new();
        if let Err(err) = validate_status(&self.status) {
            validation_errors.insert(err.0, err.1);
        }
        if validation_errors.len() > 0 {
            Err(AppErrors::FailedContractValidation(validation_errors))
        } else {
            Ok(())
        }
    }
}
