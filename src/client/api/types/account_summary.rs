use serde::Deserialize;

use crate::client::api;

use api::types::ActorImage;

#[derive(Debug, Clone, Deserialize)]
pub struct AccountSummary {
    id: usize,
    name: String,
    #[serde(rename = "displayName")]
    display_name: String,
    url: String,
    host: String,
    avatars: Vec<ActorImage>,
}

impl AccountSummary {
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
}
