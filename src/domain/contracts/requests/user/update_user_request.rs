use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::{AppErrors, AppRequest};
use crate::domain::contracts::requests::common::{validate_regex, Regexes};

#[derive(Clone, Deserialize, Serialize)]
pub struct UpdateUserRequest {
    pub user_id: Uuid,
    pub email: String,
    pub name: String,
    pub last_name:String
}

impl AppRequest<UpdateUserRequest, UpdateUserRequest> for UpdateUserRequest{
    fn validate(&self) -> Result<(), AppErrors> {
        let mut validation_errors:HashMap<String,String> = HashMap::new();
        if !validate_regex(Regexes::Email, &self.email){
            validation_errors.insert("InvalidEmail".to_string(), "Invalid e-mail provided".to_string());
        }
        if(validation_errors.len()>0){
            Err(AppErrors::FailedContractValidation(validation_errors))
        }
        else {
            Ok(())
        }
    }

    fn build_model(&self) -> Result<UpdateUserRequest, AppErrors> {
        Ok(self.clone())
    }
}