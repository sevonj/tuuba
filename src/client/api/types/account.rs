use serde::Deserialize;

use crate::client::api;

use api::{Api, ApiError};

use api::types::ActorImage;

#[derive(Debug, Clone, Deserialize)]
pub struct Account {
    id: usize,
    name: String,
    #[serde(rename = "displayName")]
    display_name: String,
    url: String,
    host: String,
    avatars: Vec<ActorImage>,
    #[serde(rename = "hostRedundancyAllowed")]
    host_redundancy_allowed: Option<bool>,
    #[serde(rename = "followingCount")]
    following_count: usize,
    #[serde(rename = "followersCount")]
    followers_count: usize,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    #[serde(rename = "userId")]
    user_id: Option<usize>,
    description: Option<String>,
}

impl Account {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn avatars(&self) -> &Vec<ActorImage> {
        &self.avatars
    }

    pub fn host_redundancy_allowed(&self) -> Option<&bool> {
        self.host_redundancy_allowed.as_ref()
    }

    pub fn following_count(&self) -> usize {
        self.following_count
    }

    pub fn followers_count(&self) -> usize {
        self.followers_count
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }

    pub fn user_id(&self) -> Option<usize> {
        self.user_id
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub async fn get(_api: Api, _handle: String) -> Result<Self, ApiError> {
        todo!()
    }
}
