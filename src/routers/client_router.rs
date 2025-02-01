use crate::domain::AppErrors;
use axum_production_ready_security::JwtClaims;
use axum::Extension;
use axum::extract::State;
use std::sync::Arc;
use crate::api_adapter::{AppErrorResponse, AppSuccessResponse};
use crate::{created_response, ok_response, AppState};
use http::StatusCode;
use uuid::Uuid;
use axum_production_ready_authorization_macros::require_scopes;
use crate::domain::{CreateUserRequest, JsonExtractor, Queries};
use crate::domain::contracts::requests::client::create_client_request::CreateClientRequest;
use crate::domain::contracts::requests::client::update_client_request::UpdateClientRequest;
use crate::domain::contracts::requests::common::app_contract::PathExtractor;
use crate::domain::models::client::Client;
use crate::domain::models::user::User;
use crate::persistance::database::execute_query;


#[require_scopes("admin")]
#[axum::debug_handler]
async fn create_client(State(state): State<Arc<AppState>>, Extension(jwt_claims): Extension<Arc<JwtClaims>>,JsonExtractor(request): JsonExtractor<CreateClientRequest>) -> Result<AppSuccessResponse<Client>, AppErrorResponse> {
    let client = request.to_model(jwt_claims.sub);
    let query = Queries::CreateClient {
        client
    };
    let res: Client = execute_query(&state.database,query).await?;
    created_response!(res)
}

#[require_scopes("admin")]
#[axum::debug_handler]
async fn update_client(State(state): State<Arc<AppState>>, Extension(jwt_claims): Extension<Arc<JwtClaims>>,JsonExtractor(request): JsonExtractor<UpdateClientRequest>) -> Result<AppSuccessResponse<Client>, AppErrorResponse> {

    let query = Queries::UpdateClient {
        client_id: request.client_id,
        client_scopes: request.client_scopes,
        client_description: request.client_description,
        expires_at: request.expires_at,
        user_id:jwt_claims.sub
    };
    let res: Client = execute_query(&state.database,query).await?;
    ok_response!(res)
}

#[require_scopes("admin")]
#[axum::debug_handler]
async fn delete_client(State(state): State<Arc<AppState>>, Extension(jwt_claims): Extension<Arc<JwtClaims>>,PathExtractor(client_id): PathExtractor<Uuid>) -> Result<AppSuccessResponse<Client>, AppErrorResponse> {
    let query = Queries::DeleteClient {
        client_id,
        user_id: jwt_claims.sub
    };
    let res: Client = execute_query(&state.database,query).await?;
    ok_response!(res)
}


#[require_scopes("admin")]
#[axum::debug_handler]
async fn get_client(State(state): State<Arc<AppState>>, Extension(jwt_claims): Extension<Arc<JwtClaims>>,PathExtractor(client_id): PathExtractor<Uuid>) -> Result<AppSuccessResponse<Client>, AppErrorResponse> {
    let query = Queries::GetClient {
        client_id,
        user_id: jwt_claims.sub
    };
    let res: Client = execute_query(&state.database,query).await?;
    ok_response!(res)
}

#[require_scopes("admin")]
#[axum::debug_handler]
async fn get_clients(State(state): State<Arc<AppState>>, Extension(jwt_claims): Extension<Arc<JwtClaims>>) -> Result<AppSuccessResponse<Client>, AppErrorResponse> {
    let query = Queries::GetClients {
        user_id: jwt_claims.sub
    };
    let res: Client = execute_query(&state.database,query).await?;
    ok_response!(res)
}


