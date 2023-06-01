use serde::{Deserialize, Serialize};

use super::{Redditor, Media};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubredditPost {
    pub id: String,
    pub created_at: String,
    pub title: String,
    pub domain: String,
    pub url: String,
    pub permalink: String,

    pub author_info: Redditor,
    pub author_flair: Option<AuthorFlair>,
    pub flair: Option<PostFlair>,

    pub content: Option<PostContent>,
    pub media: Option<Media>,

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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostContent {
    pub markdown: String,
    pub richtext: String,
    pub html: String,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostFlair {
    #[serde(rename = "type")]
    pub flair_type: PostFlairType,
    pub text: String,
    pub rich_text: Option<String>,
    pub text_color: TextColor,
    pub template: PostFlairTemplate,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PostFlairType {
    Text,
    Richtext,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostFlairTemplate {
    pub id: Option<String>,
    pub background_color: Option<String>,
    pub is_editable: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorFlair {
    pub text: String,
    pub rich_text: Option<String>,
    pub text_color: TextColor,
    pub template: AuthorFlairTemplate,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorFlairTemplate {
    pub id: Option<String>,
    pub background_color: Option<String>,
    pub is_mod_only: bool,
    pub is_editable: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextColor {
    Light,
    Dark,
}

