use std::sync::Arc;

use crate::authentication::{
    self,
    get_token::{client_credentials_flow, GetToken},
};
use reqwest::{header, Client, RequestBuilder, Url};
use tokio::sync::Mutex;

pub mod roles;
pub mod users;

pub struct Api {
    pub base_url: Url,
    access_token: Arc<Mutex<client_credentials_flow::Response>>,
    client: Client,
}

impl Api {
    pub async fn init(
        base_url: Url,
        client_id: String,
        client_secret: String,
    ) -> Result<Self, reqwest::Error> {
        let client = Self::build_client();

        // Instantiate a Authentication API client for getting client_credentials token
        let auth = authentication::Api::init(
            base_url.clone(),
            authentication::AuthenticationMethod::ClientIDClientSecret(
                client_id.clone(),
                client_secret.clone(),
            ),
        );
        let params = authentication::get_token::client_credentials_flow::RequestParameters {
            grant_type: "client_credentials".to_string(),
            client_id: client_id.clone(),
            client_secret: client_secret.clone(),
            audience: base_url.join("/api/v2/").unwrap().to_string(),
        };
        let access_token = Arc::new(Mutex::new(
            auth.client_credentials_flow(params.clone())
                .send()
                .await?
                .json::<client_credentials_flow::Response>()
                .await?,
        ));
        tracing::info!("Fetched access token (client_credentials) from Auth0");

        // Spawn a thread to keep the access token up to date
        let token = access_token.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(3600));
            interval.tick().await;

            // Fetch a new token every hour
            loop {
                interval.tick().await;
                let mut token = token.lock().await;
                *token = auth
                    .client_credentials_flow(params.clone())
                    .send()
                    .await
                    .unwrap()
                    .json::<client_credentials_flow::Response>()
                    .await
                    .unwrap();
                tracing::info!("Refreshed access token (client_credentials) from Auth0");
            }
        });

        Ok(Api {
            base_url,
            client,
            access_token,
        })
    }

    fn apply_auth(&self, rb: RequestBuilder) -> RequestBuilder {
        let token = self.access_token.clone();
        tokio::task::block_in_place(move || {
            rb.header(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&token.blocking_lock().access_token.clone()).unwrap(),
            )
        })
    }

    fn build_client() -> Client {
        static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
        reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn authentication_api_init() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let api = Api::init(
            base_url,
            String::from("some_awesome_id"),
            String::from("some_awesome_token"),
        )
        .await
        .unwrap();
        let request = api
            .client
            .request(reqwest::Method::GET, api.base_url)
            .build()
            .unwrap();
        let test_url = String::from("https://your_domain/");
        assert_eq!(request.method(), reqwest::Method::GET);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().len(), 0);
        assert_eq!(request.body().is_none(), true);
    }
}
