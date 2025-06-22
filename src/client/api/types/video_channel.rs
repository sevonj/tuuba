use serde::Deserialize;

use crate::client::api;

use api::{Api, ApiError};

use api::types::Account;
use api::types::ActorImage;

#[derive(Debug, Clone, Deserialize)]
pub struct VideoChannel {
    id: usize,
    url: String,
    name: String,
    avatars: Vec<ActorImage>,
    host: String,
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
    #[serde(rename = "displayName")]
    display_name: String,
    description: Option<String>,
    support: Option<String>,
    #[serde(rename = "isLocal")]
    is_local: bool,
    banners: Vec<ActorImage>,
    #[serde(rename = "ownerAccount")]
    owner_account: Account,
}

impl VideoChannel {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn avatars(&self) -> &Vec<ActorImage> {
        &self.avatars
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn host_redundancy_allowed(&self) -> Option<bool> {
        self.host_redundancy_allowed
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

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn support(&self) -> Option<&String> {
        self.support.as_ref()
    }

    pub fn is_local(&self) -> bool {
        self.is_local
    }

    pub fn banners(&self) -> &Vec<ActorImage> {
        &self.banners
    }

    pub fn owner_account(&self) -> &Account {
        &self.owner_account
    }

    pub async fn get(api: Api, channel_handle: &str) -> Result<Self, ApiError> {
        api.get(format!("/api/v1/video-channels/{channel_handle}"))
            .await
    }
}
