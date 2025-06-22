use serde::Deserialize;

use crate::client::api;

use api::{Api, ApiError};

use api::types::Account;
use api::types::VideoChannel;
use api::types::VideoFile;
use api::types::VideoStreamingPlaylist;

#[derive(Debug, Clone, Deserialize)]
pub struct VideoDetails {
    id: usize,
    uuid: String,
    #[serde(rename = "shortUUID")]
    short_uuid: String,
    #[serde(rename = "isLive")]
    is_live: bool,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    #[serde(rename = "publishedAt")]
    published_at: String,
    #[serde(rename = "originallyPublishedAt")]
    originally_published_at: Option<String>,
    // TODO: Category
    // TODO: License
    // TODO: Language
    // TODO: Privacy
    #[serde(rename = "truncatedDescription")]
    truncated_description: Option<String>,
    duration: usize,
    #[serde(rename = "aspectRatio")]
    aspect_ratio: Option<f32>,
    #[serde(rename = "isLocal")]
    is_local: bool,
    name: String,
    #[serde(rename = "thumbnailPath")]
    thumbnail_path: String,
    #[serde(rename = "previewPath")]
    preview_path: String,
    #[serde(rename = "embedPath")]
    embed_path: String,
    views: usize,
    likes: usize,
    dislikes: usize,
    comments: usize,
    nsfw: bool,
    #[serde(rename = "nsfwFlags")]
    nsfw_flags: usize,
    /// TODO: Remove opt. PeerTube >= 7.2
    #[serde(rename = "nsfwSummary")]
    nsfw_summary: Option<String>,
    #[serde(rename = "waitTranscoding")]
    wait_transcoding: Option<bool>,
    // TODO: state
    // TODO: scheduledupdate
    blacklisted: Option<bool>,
    #[serde(rename = "blacklistedReason")]
    blacklisted_reason: Option<String>,
    account: Account,
    channel: VideoChannel,
    viewers: usize,
    description: Option<String>,
    support: Option<String>,
    tags: Vec<String>,
    // commentsEnabled
    // comments_policy: usize,
    #[serde(rename = "downloadEnabled")]
    download_enabled: bool,
    input_file_updated_at: Option<String>,
    #[serde(rename = "trackerUrls")]
    tracker_urls: Vec<String>,
    #[serde(rename = "streamingPlaylists")]
    streaming_playlists: Option<Vec<VideoStreamingPlaylist>>,
    files: Option<Vec<VideoFile>>,
}

impl VideoDetails {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn short_uuid(&self) -> &str {
        &self.short_uuid
    }

    pub fn is_live(&self) -> bool {
        self.is_live
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }

    pub fn published_at(&self) -> &str {
        &self.published_at
    }

    pub fn originally_published_at(&self) -> Option<&str> {
        self.originally_published_at.as_deref()
    }

    // Category
    // License
    // Language
    // Privacy

    pub fn truncated_description(&self) -> Option<&str> {
        self.truncated_description.as_deref()
    }

    pub fn duration(&self) -> usize {
        self.duration
    }

    pub fn aspect_ratio(&self) -> Option<f32> {
        self.aspect_ratio
    }

    pub fn is_local(&self) -> bool {
        self.is_local
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn thumbnail_path(&self) -> &str {
        &self.thumbnail_path
    }

    pub fn preview_path(&self) -> &str {
        &self.preview_path
    }

    pub fn embed_path(&self) -> &str {
        &self.embed_path
    }

    pub fn views(&self) -> usize {
        self.views
    }

    pub fn likes(&self) -> usize {
        self.likes
    }

    pub fn dislikes(&self) -> usize {
        self.dislikes
    }

    pub fn comments(&self) -> usize {
        self.comments
    }

    pub fn nsfw(&self) -> bool {
        self.nsfw
    }

    pub fn nsfw_flags(&self) -> usize {
        self.nsfw_flags
    }

    pub fn nsfw_summary(&self) -> Option<&str> {
        self.nsfw_summary.as_deref()
    }

    pub fn wait_transcoding(&self) -> Option<bool> {
        self.wait_transcoding
    }

    pub fn blacklisted(&self) -> Option<bool> {
        self.blacklisted
    }

    pub fn blacklisted_reason(&self) -> Option<&str> {
        self.blacklisted_reason.as_deref()
    }

    pub fn account(&self) -> &Account {
        &self.account
    }

    pub fn channel(&self) -> &VideoChannel {
        &self.channel
    }

    pub fn viewers(&self) -> usize {
        self.viewers
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn support(&self) -> Option<&str> {
        self.support.as_deref()
    }

    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    // commentsEnabled
    // comments_policy: usize,

    pub fn download_enabled(&self) -> bool {
        self.download_enabled
    }

    pub fn input_file_updated_at(&self) -> Option<&str> {
        self.input_file_updated_at.as_deref()
    }

    pub fn tracker_urls(&self) -> &Vec<String> {
        &self.tracker_urls
    }

    pub fn streaming_playlists(&self) -> Option<&Vec<VideoStreamingPlaylist>> {
        self.streaming_playlists.as_ref()
    }

    pub fn files(&self) -> Option<&Vec<VideoFile>> {
        self.files.as_ref()
    }

    pub fn publish_timestamp(&self) -> &str {
        if let Some(timestamp) = &self.originally_published_at {
            return timestamp;
        }
        &self.published_at
    }

    pub fn playback_url(&self) -> &str {
        if let Some(playlists) = &self.streaming_playlists {
            if !playlists.is_empty() {
                return playlists[0].playlist_url();
            }
        }
        self.files.as_ref().unwrap()[0].file_url()
        // "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4"
    }

    pub async fn get(api: Api, id: usize) -> Result<Self, ApiError> {
        api.get(format!("/api/v1/videos/{id}")).await
    }
}
