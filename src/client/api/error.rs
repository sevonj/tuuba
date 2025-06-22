use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum ApiError {
    RequestFail,
    ParseFail { url: String },
    InvalidUrl { url: String },
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ApiError::*;

        match self {
            RequestFail => write!(f, "Request failed."),
            ParseFail { url } => write!(f, "Couldn't parse response from '{url}'"),
            InvalidUrl { url } => write!(f, "Invalid URL: '{url}'"),
        }
    }
}

impl Error for ApiError {}

impl From<reqwest::Error> for ApiError {
    fn from(_value: reqwest::Error) -> Self {
        ApiError::RequestFail
    }
}
