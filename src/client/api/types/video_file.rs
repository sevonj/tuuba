use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct VideoFile {
    #[serde(rename = "fileUrl")]
    file_url: String,
}

impl VideoFile {
    pub fn file_url(&self) -> &str {
        &self.file_url
    }
}
