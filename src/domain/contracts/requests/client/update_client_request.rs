use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;
use crate::domain::{AppErrors, AppRequest};
use crate::domain::contracts::requests::common::{validate_regex, Regexes};
use crate::domain::contracts::requests::common::validation_helpers::validate_scopes;

#[derive(Deserialize)]
pub struct UpdateClientRequest {
    pub client_id: Uuid,
    pub client_description: String,
    pub client_scopes: String,
    pub expires_at: Option<DateTime<Utc>>,
}

impl AppRequest<UpdateClientRequest> for UpdateClientRequest {
    fn validate(&self) -> Result<(), AppErrors> {
        let mut validation_errors:HashMap<String,String> = HashMap::new();
        if !validate_regex(Regexes::Scopes, &self.client_scopes){
            validation_errors.insert("InvalidScopes".to_string(), "Invalid scopes provided, they must be split by a single space ".to_string());
        }
        if let Err(err) = validate_scopes(&self.client_scopes) {
            validation_errors.insert(err.0, err.1);
        }
        if validation_errors.len() > 0 {
            Err(AppErrors::FailedContractValidation(validation_errors))
        }
        else{
            Ok(())
        }
    }
}