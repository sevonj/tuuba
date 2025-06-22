use serde::Deserialize;

use crate::client::api;

use api::types::AccountSummary;
use api::types::VideoChannelSummary;

#[derive(Debug, Clone, Deserialize)]
pub struct Video {
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
    account: AccountSummary,
    channel: VideoChannelSummary,
    // TODO: userhistory
}

impl Video {
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

    pub fn account(&self) -> &AccountSummary {
        &self.account
    }

    pub fn channel(&self) -> &VideoChannelSummary {
        &self.channel
    }

    pub fn publish_timestamp(&self) -> &str {
        if let Some(timestamp) = &self.originally_published_at {
            return timestamp;
        }
        &self.published_at
    }
}
