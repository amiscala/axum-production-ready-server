use crate::api_adapter::CustomHttpStatusCode;
use http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct AppErrorStruct {
    pub error: Vec<AppErrorInnerResponse>,
    pub status_code: CustomHttpStatusCode,
}

impl AppErrorStruct {
    pub fn new(errors: Vec<AppErrorInnerResponse>, status_code: StatusCode) -> Self{
        Self{
            error: errors,
            status_code: CustomHttpStatusCode(status_code)
        }
    }

    pub fn from_single_error(code: &str, message:&str, status_code: StatusCode) -> Self{
        Self{
            error: vec!(AppErrorInnerResponse::new(code.to_string(),message.to_string())),
            status_code: CustomHttpStatusCode(status_code)
        }
    }
}


#[derive(Serialize)]
pub struct AppErrorInnerResponse {
    pub code: String,
    pub message: String,
}

impl AppErrorInnerResponse{
    pub fn new(code: String, message:String) -> Self{
        Self{
            code,
            message
        }
    }
}