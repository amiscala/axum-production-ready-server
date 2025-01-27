use std::collections::HashMap;
use serde::Deserialize;
use crate::domain::{AppErrors, AppRequest};
use crate::domain::contracts::requests::common::{validate_regex, Regexes};
use crate::domain::models::client::Client;
use crate::domain::models::user::User;

#[derive(Deserialize, Clone)]
pub struct CreateUserAndClient {
    email: String,
    name: String,
    last_name: String
}

impl CreateUserAndClient {
    pub fn to_model(self) -> (User, Client){
        let user = User::new(self.email, self.name, self.last_name);
        let client = Client::new(user.user_id.clone(), user.user_scopes.clone(), "Default client".to_owned(), None);
        (user,client)
    }
}

impl AppRequest<CreateUserAndClient> for CreateUserAndClient {
    fn validate(&self) -> Result<(), AppErrors> {
        let mut validation_errors:HashMap<String,String> = HashMap::new();
        if !validate_regex(Regexes::Email, &self.email){
            validation_errors.insert("InvalidEmail".to_string(), "Invalid e-mail provided".to_string());
        }
        if(validation_errors.len() > 0){
            Err(AppErrors::FailedContractValidation(validation_errors))
        }
        else{
            Ok(())
        }
    }
}