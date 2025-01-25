use std::collections::HashMap;
use crate::domain::{AppErrors, AppRequest};
use crate::domain::contracts::requests::common::{validate_regex, Regexes};
use crate::domain::models::client::Client;
use crate::domain::models::user::User;

pub struct CreateUserAndClient {
    email: String,
    name: String,
    last_name: String
}

impl AppRequest<CreateUserAndClient,(User, Client)> for CreateUserAndClient {
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

    fn build_model(&self) -> Result<(User, Client), AppErrors> {
        let user = User::new(self.email.clone(), self.name.clone(), self.last_name.clone());
        let client = Client::new(user.user_id.clone(), user.user_scopes.clone(), "user first client".to_string(),None);
        Ok((user, client))
    }
}