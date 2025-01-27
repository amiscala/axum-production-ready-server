use uuid::Uuid;
use std::collections::HashMap;
use serde::Deserialize;
use crate::domain::{AppErrors, AppRequest};
use crate::domain::contracts::requests::common::{validate_regex, Regexes};
use crate::domain::contracts::requests::common::validation_helpers::validate_scopes;

#[derive(Deserialize, Clone)]
pub struct IssueTokenRequest {
    pub client_id: Uuid,
    pub client_secret: String,
    pub scope: String,
    pub grant_type: String
}

impl AppRequest<IssueTokenRequest> for IssueTokenRequest{
    fn validate(&self) -> Result<(), AppErrors> {
        let mut validation_errors:HashMap<String,String> = HashMap::new();
        if !validate_regex(Regexes::Scopes, &self.scope){
            validation_errors.insert("InvalidScopes".to_string(), "Invalid scopes provided, they must be split by a single space ".to_string());
        }
        if let Err(err) = validate_scopes(&self.scope) {
            validation_errors.insert(err.0, err.1);
        }
        if self.grant_type != "client_credentials" {
            validation_errors.insert("InvalidGrantType".to_string(), "Invalid grant type".to_string());
        }
        if(validation_errors.len() > 0){
            Err(AppErrors::FailedContractValidation(validation_errors))
        }
        else{
            Ok(())
        }
    }
}