use serde::Deserialize;

use super::{Api, ApiError};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    instance: Instance,
}

impl Config {
    pub fn instance(&self) -> &Instance {
        &self.instance
    }
}

impl Config {
    pub async fn get(api: Api) -> Result<Self, ApiError> {
        api.get("/api/v1/config").await
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Instance {
    // avatars
    // banners
    // customizations
    #[serde(rename = "defaultClientRoute")]
    default_client_route: String,
    #[serde(rename = "defaultNSFWPolicy")]
    default_nsfw_policy: String,
    #[serde(rename = "isNSFW")]
    is_nsfw: bool,
    name: String,
    #[serde(rename = "serverCountry")]
    server_country: String,
    #[serde(rename = "shortDescription")]
    short_description: String,
    // social
    // support
}

impl Instance {
    pub fn default_client_route(&self) -> &str {
        &self.default_client_route
    }
    pub fn default_nsfw_policy(&self) -> &str {
        &self.default_nsfw_policy
    }
    pub fn is_nsfw(&self) -> bool {
        self.is_nsfw
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn server_country(&self) -> &str {
        &self.server_country
    }
    pub fn short_description(&self) -> &str {
        &self.short_description
    }
}
