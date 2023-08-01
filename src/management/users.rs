use super::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AssignRolesRequestParameters {
    pub roles: Vec<String>,
}

pub trait Users {
    fn assign_roles(&self, id: String, request: AssignRolesRequestParameters) -> RequestBuilder;
}

impl Users for Api {
    fn assign_roles(&self, id: String, request: AssignRolesRequestParameters) -> RequestBuilder {
        let endpoint = format!("/api/v2/users/{}/roles", id);
        let url = self.base_url.join(&endpoint).unwrap();
        self.apply_auth(self.client.post(url).json(&request))
    }
}
