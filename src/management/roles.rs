use super::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListRequestParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_totals: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_filter: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRequestParameters {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateRequestParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AssignUsersRequestParameters {
    pub users: Vec<String>,
}

pub trait Roles {
    fn list(&self, request: ListRequestParameters) -> RequestBuilder;

    fn create(&self, request: CreateRequestParameters) -> RequestBuilder;

    fn get(&self, id: String) -> RequestBuilder;

    fn delete(&self, id: String) -> RequestBuilder;

    fn update(&self, id: String, request: UpdateRequestParameters) -> RequestBuilder;

    fn assign_users(&self, id: String, request: AssignUsersRequestParameters) -> RequestBuilder;
}

impl Roles for Api {
    fn list(&self, request: ListRequestParameters) -> RequestBuilder {
        let endpoint = String::from("/api/v2/roles");
        let url = self.base_url.join(&endpoint).unwrap();
        self.apply_auth(self.client.get(url)).query(&request)
    }

    fn create(&self, request: CreateRequestParameters) -> RequestBuilder {
        let endpoint = String::from("/api/v2/roles");
        let url = self.base_url.join(&endpoint).unwrap();
        self.apply_auth(self.client.post(url)).json(&request)
    }

    fn get(&self, id: String) -> RequestBuilder {
        let endpoint = format!("/api/v2/roles/{}", id);
        let url = self.base_url.join(&endpoint).unwrap();
        self.apply_auth(self.client.get(url))
    }

    fn delete(&self, id: String) -> RequestBuilder {
        let endpoint = format!("/api/v2/roles/{}", id);
        let url = self.base_url.join(&endpoint).unwrap();
        self.apply_auth(self.client.delete(url))
    }

    fn update(&self, id: String, request: UpdateRequestParameters) -> RequestBuilder {
        let endpoint = format!("/api/v2/roles/{}", id);
        let url = self.base_url.join(&endpoint).unwrap();
        self.apply_auth(self.client.patch(url).json(&request))
    }

    fn assign_users(&self, id: String, request: AssignUsersRequestParameters) -> RequestBuilder {
        let endpoint = format!("/api/v2/roles/{}/users", id);
        let url = self.base_url.join(&endpoint).unwrap();
        self.apply_auth(self.client.post(url).json(&request))
    }
}
