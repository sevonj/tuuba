use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ActorImage {
    path: String,
    width: usize,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
}

impl ActorImage {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }
}
