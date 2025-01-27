use std::collections::HashMap;
use serde::Deserialize;
use uuid::Uuid;
use crate::domain::models::user::User;
use crate::domain::{AppRequest, AppErrors};
use crate::domain::contracts::requests::common::{validate_regex, Regexes};
use crate::domain::contracts::requests::common::validation_helpers::validate_scopes;
use crate::domain::models::client::Client;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub name: String,
    pub last_name: String,
}

impl CreateUserRequest {
    pub fn to_model(self) -> User{
        User::new(self.email, self.name, self.last_name)
    }
}

impl AppRequest<CreateUserRequest> for CreateUserRequest {
    fn validate(&self) -> Result<(), AppErrors> {
        let mut validation_errors:HashMap<String,String> = HashMap::new();
        if !validate_regex(Regexes::Email, &self.email){
            validation_errors.insert("InvalidEmail".to_string(), "Invalid e-mail provided".to_string());
        }
        // if !validate_regex(Regexes::Scopes, &self.scopes){
        //     validation_errors.insert("InvalidScopes".to_string(), "Invalid scopes they must only lower case letters splitted by a single space".to_string());
        // }

        if(validation_errors.len() > 0){
            Err(AppErrors::FailedContractValidation(validation_errors))
        }
        else{
            Ok(())
        }
    }
}