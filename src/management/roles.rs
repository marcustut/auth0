use super::Api;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListRolesRequestParameters {
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
pub struct CreateRoleRequestParameters {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateRoleRequestParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AssignUsersToRoleRequestParameters {
    pub users: Vec<String>,
}

pub trait Roles {
    fn list_roles(&self, request: ListRolesRequestParameters) -> RequestBuilder;

    fn create_role(&self, request: CreateRoleRequestParameters) -> RequestBuilder;

    fn get_role(&self, id: String) -> RequestBuilder;

    fn delete_role(&self, id: String) -> RequestBuilder;

    fn update_role(&self, id: String, request: UpdateRoleRequestParameters) -> RequestBuilder;

    fn assign_users_to_role(
        &self,
        id: String,
        request: AssignUsersToRoleRequestParameters,
    ) -> RequestBuilder;
}

impl Roles for Api {
    fn list_roles(&self, request: ListRolesRequestParameters) -> RequestBuilder {
        let endpoint = String::from("/api/v2/roles");
        let url = self.base_url.join(&endpoint).unwrap();
        self.apply_auth(self.client.get(url)).query(&request)
    }

    fn create_role(&self, request: CreateRoleRequestParameters) -> RequestBuilder {
        let endpoint = String::from("/api/v2/roles");
        let url = self.base_url.join(&endpoint).unwrap();
        self.apply_auth(self.client.post(url)).json(&request)
    }

    fn get_role(&self, id: String) -> RequestBuilder {
        let endpoint = format!("/api/v2/roles/{}", id);
        let url = self.base_url.join(&endpoint).unwrap();
        self.apply_auth(self.client.get(url))
    }

    fn delete_role(&self, id: String) -> RequestBuilder {
        let endpoint = format!("/api/v2/roles/{}", id);
        let url = self.base_url.join(&endpoint).unwrap();
        self.apply_auth(self.client.delete(url))
    }

    fn update_role(&self, id: String, request: UpdateRoleRequestParameters) -> RequestBuilder {
        let endpoint = format!("/api/v2/roles/{}", id);
        let url = self.base_url.join(&endpoint).unwrap();
        self.apply_auth(self.client.patch(url).json(&request))
    }

    fn assign_users_to_role(
        &self,
        id: String,
        request: AssignUsersToRoleRequestParameters,
    ) -> RequestBuilder {
        let endpoint = format!("/api/v2/roles/{}/users", id);
        let url = self.base_url.join(&endpoint).unwrap();
        self.apply_auth(self.client.post(url).json(&request))
    }
}
