use serde::{Deserialize, Serialize};

use super::{Redditor, SubredditPost};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: String,
    pub created_at: String,
    pub edited_at: Option<String>,
    pub parent: Option<CommentParent>,
    pub post_info: SubredditPost,

    pub author_info: Redditor,

    #[serde(deserialize_with = "crate::utils::deserialize_option_float_to_int")]
    pub score: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentParent {
    pub id: String,
}
