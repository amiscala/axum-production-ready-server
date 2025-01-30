#[macro_export] macro_rules! ok_response {
    ($response_body:expr) => {
        Ok(AppSuccessResponse::new($response_body, StatusCode::OK))
    };
}

#[macro_export] macro_rules! created_response {
    ($response_body:expr) => {
        Ok(AppSuccessResponse::new($response_body, StatusCode::CREATED))
    };
}

#[macro_export] macro_rules! success_response_custom_status_code {
    ($response_body:expr, $status_code:expr) => {
        Ok(AppSuccessResponse::new($response_body, $status_code))
    };
}
