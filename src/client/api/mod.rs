mod error;

pub mod instance;
pub mod types;

use serde::de::DeserializeOwned;
use std::str::FromStr;

pub use error::ApiError;

// https://docs.joinpeertube.org/api-rest-reference.html

#[derive(Debug, Clone)]
pub struct Api {
    client: reqwest::Client,
    base_url: String,
}

impl Api {
    pub fn new<S: Into<String>>(base_url: S) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.into(),
        }
    }

    /// Path is in form "/api/v1/config"
    pub async fn get<S, T>(&self, path: S) -> Result<T, ApiError>
    where
        S: Into<String>,
        T: DeserializeOwned,
    {
        let url_str = format!("{}{}", self.base_url, path.into());
        let Ok(url) = reqwest::Url::from_str(&url_str) else {
            return Err(ApiError::InvalidUrl { url: url_str });
        };
        let text = self.client.get(url.clone()).send().await?.text().await?;
        serde_json::from_str(&text).map_err(|_| ApiError::ParseFail {
            url: url_str.clone(),
        })
    }
}
