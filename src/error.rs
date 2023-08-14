use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationErrorResponse {
    pub error: String,
    pub error_description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagementErrorResponse {
    pub status_code: u16,
    pub error: String,
    pub message: String,
    pub attributes: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, thiserror::Error, Deserialize, Serialize)]
#[serde(tag = "error")]
pub enum AuthenticationError {
    #[error("Bad request: {0:?}")]
    InvalidRequest(AuthenticationErrorResponse),

    #[error("Bad request: {0:?}")]
    InvalidScope(AuthenticationErrorResponse),

    #[error("Unauthorized: {0:?}")]
    InvalidClient(AuthenticationErrorResponse),

    #[error("Unauthorized: {0:?}")]
    RequestValidation(AuthenticationErrorResponse),

    #[error("Forbidden: {0:?}")]
    UnauthorizedClient(AuthenticationErrorResponse),

    #[error("Forbidden: {0:?}")]
    AccessDenied(AuthenticationErrorResponse),

    #[error("Forbidden: {0:?}")]
    InvalidGrant(AuthenticationErrorResponse),

    #[error("Not found: {0:?}")]
    EndpointDisabled(AuthenticationErrorResponse),

    #[error("Method not allowed: {0:?}")]
    MethodNotAllowed(AuthenticationErrorResponse),

    #[error("Too many requests: {0:?}")]
    TooManyRequests(AuthenticationErrorResponse),

    #[error("Not implemented: {0:?}")]
    UnsupportedResponseType(AuthenticationErrorResponse),

    #[error("Not implemented: {0:?}")]
    UnsupportedGrantType(AuthenticationErrorResponse),

    #[error("Service unavailable: {0:?}")]
    TemporarilyUnavailable(AuthenticationErrorResponse),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Reqwest error: {0:?}")]
    RequestError(#[from] reqwest::Error),

    #[error("Authentication error: {0:?}")]
    AuthenticationError(#[from] AuthenticationError),

    #[error("Management error: {0:?}")]
    ManagementError(ManagementErrorResponse),
}
