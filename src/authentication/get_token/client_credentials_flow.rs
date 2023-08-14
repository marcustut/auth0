pub use crate::authentication::get_token::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestParameters {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub audience: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub access_token: String,
    pub scope: Option<String>,
    pub expires_in: u64,
    pub token_type: String,
}
