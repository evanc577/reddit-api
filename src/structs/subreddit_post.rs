use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubredditPost {
    pub id: String,
    pub created_at: String,
    pub title: String,
    pub domain: String,
    pub url: String,
    pub permalink: String,

    #[serde(deserialize_with = "crate::utils::deserialize_option_float_to_int")]
    pub score: Option<i64>,
    #[serde(deserialize_with = "crate::utils::deserialize_float_to_int")]
    pub comment_count: i64,

    pub is_archived: bool,
    pub is_contest_mode: bool,
    pub is_crosspostable: bool,
    pub is_gildable: bool,
    pub is_locked: bool,
    pub is_nsfw: bool,
    pub is_poll_included: bool,
    pub is_react_allowed: bool,
    pub is_self_post: bool,
    pub is_spoiler: bool,
    pub is_stickied: bool,
}
