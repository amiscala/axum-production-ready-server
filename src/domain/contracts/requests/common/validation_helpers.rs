use core::str::FromStr;
use regex::Regex;
use crate::domain::AppScope;

pub enum Regexes {
    Email,
    Scopes,
}

const EMAIL_REGEX: &str = "[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+";
const SCOPES_REGEX: &str = "^([a-z]+)( [a-z]+)*$";


pub fn validate_regex(regex: Regexes, value: &str) -> bool {
    let regex_expression = match regex {
        Regexes::Email => {
            Regex::new(EMAIL_REGEX).expect("Wrong Email Regex")
        }
        Regexes::Scopes => {
            Regex::new(SCOPES_REGEX).expect("Wrong Scopes Regex")
        }
    };
    if !regex_expression.is_match(value) {
        return false;
    }
    true
}

pub fn validate_scopes(scopes: &str) -> Result<(), (String, String)> {
    let mut valid = true;
    let scopes_vec = scopes.split(" ");
    let mut error_string = String::from("Could not find the given scopes: ");
    for scope in scopes_vec {
        if let Err(_) = AppScope::from_str(scope) {
            valid = false;
            error_string = format!("{}, {}", error_string,scope);
        }
    }
    if valid
    {
        Ok(())
    }
    else{
        Err(("ScopesNotFound".to_string(),error_string))
    }
}