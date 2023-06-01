use serde::{Deserialize, Serialize};

use super::{MediaSource, Dimensions};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub preview_media_id: String,
    pub still: StillMedia,
    pub streaming: Option<StreamingMedia>,
    pub video: Option<VideoMedia>,
    pub packaged_media: Option<PackagedMedia>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StillMedia {
    pub source: Option<MediaSource>,
    pub small: Option<MediaSource>,
    pub medium: Option<MediaSource>,
    pub large: Option<MediaSource>,
    pub xlarge: Option<MediaSource>,
    pub xxlarge: Option<MediaSource>,
    pub xxxlarge: Option<MediaSource>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingMedia {
    pub hls_url: String,
    pub dash_url: String,
    pub scrubber_media_url: String,
    pub dimensions: Dimensions,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoMedia {
    pub embed_html: String,
    pub url: String,
    pub dimensions: Dimensions,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackagedMedia {
    pub muxed_mp4s: Option<MuxedMp4s>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MuxedMp4s {
    pub low: MuxedMp4,
    pub medium: MuxedMp4,
    pub high: MuxedMp4,
    pub highest: MuxedMp4,
    pub recommended: MuxedMp4,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MuxedMp4 {
    pub url: String,
}
