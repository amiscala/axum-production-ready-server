use std::collections::HashMap;
use serde::Deserialize;
use crate::domain::models::user::User;
use crate::domain::{AppRequest, AppErrors};
use crate::domain::contracts::requests::common::{validate_regex, Regexes};
use crate::domain::contracts::requests::common::validation_helpers::validate_scopes;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub name: String,
    pub last_name: String,
}

impl AppRequest<CreateUserRequest, User> for CreateUserRequest {
    fn validate(&self) -> Result<(), AppErrors> {
        let mut validation_errors:HashMap<String,String> = HashMap::new();
        if !validate_regex(Regexes::Email, &self.email){
            validation_errors.insert("InvalidEmail".to_string(), "Invalid e-mail provided".to_string());
        }
        // if !validate_regex(Regexes::Scopes, &self.scopes){
        //     validation_errors.insert("InvalidScopes".to_string(), "Invalid scopes they must only lower case letters splitted by a single space".to_string());
        // }
        // if let Err(err) = validate_scopes(&self.scopes){
        //     validation_errors.insert(err.0, err.1);
        // }
        if(validation_errors.len() > 0){
            Err(AppErrors::FailedContractValidation(validation_errors))
        }
        else{
            Ok(())
        }
    }

    fn build_model(&self) -> Result<User, AppErrors> {
        Ok(User::new(
            self.email.clone(),
            self.name.clone(),
            self.last_name.clone(),
        ))
    }
}