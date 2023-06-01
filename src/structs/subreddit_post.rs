use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubredditPost {
    #[serde(rename = "__typename")]
    pub typename: PostType,
    pub id: String,
    pub created_at: String,
    pub title: String,
    pub url: String,
    pub permalink: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PostType {
    AdPost,
    SubredditPost,
}
