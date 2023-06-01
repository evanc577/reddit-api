use serde::{Deserialize, Serialize};

use super::MediaSource;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Redditor {
    pub id: String,
    pub name: String,

    pub icon: MediaSource,
    pub icon_small: MediaSource,

    pub is_cake_day_now: bool,
}
