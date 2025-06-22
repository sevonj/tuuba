use serde::Deserialize;

use crate::client::api;

use api::{Api, ApiError};

use api::types::Video;

#[derive(Debug, Clone, Deserialize)]
pub struct Videos {
    data: Vec<Video>,
    total: usize,
}

impl Videos {
    pub fn data(&self) -> &Vec<Video> {
        &self.data
    }

    pub fn total(&self) -> usize {
        self.total
    }

    pub async fn get(api: Api) -> Result<Self, ApiError> {
        api.get("/api/v1/videos").await
    }
}
