use serde::{Deserialize, Serialize};

use super::Dimensions;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaSource {
    pub url: String,
    pub dimensions: Dimensions,
}
