use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct VideoStreamingPlaylist {
    #[serde(rename = "playlistUrl")]
    playlist_url: String,
}

impl VideoStreamingPlaylist {
    pub fn playlist_url(&self) -> &str {
        &self.playlist_url
    }
}
