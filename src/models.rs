use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthenticationResponse<T> {
    Success(T),
    Error(super::error::AuthenticationError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_deserialize_success_auth_response() {
        let json = r#"{
  "access_token": "....",
  "scope": "update:roles",
  "expires_in": 86400,
  "token_type": "Bearer"
}
"#;

        let response = serde_json::from_str::<
            super::AuthenticationResponse<
                crate::authentication::get_token::client_credentials_flow::Response,
            >,
        >(json);

        assert_eq!(response.is_ok(), true);
    }

    #[test]
    fn test_deserialize_error_auth_response() {
        let json = r#"{"error":"access_denied","error_description":"Unauthorized"}"#;

        let response = serde_json::from_str::<
            super::AuthenticationResponse<
                crate::authentication::get_token::client_credentials_flow::Response,
            >,
        >(json);

        assert_eq!(response.is_ok(), true);
    }
}
