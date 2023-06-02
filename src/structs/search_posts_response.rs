use serde::Deserialize;

use super::{PageInfo, Post};
use crate::traits::Response;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SearchPostsResponse {
    data: Data,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    search: Search,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Search {
    general: General,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct General {
    posts: Posts,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Posts {
    page_info: PageInfo,
    edges: Vec<Edge>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Edge {
    node: PostType,
}

#[allow(clippy::large_enum_variant)]
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "__typename")]
pub(crate) enum PostType {
    SubredditPost(Post),
    ProfilePost(Post),
    AdPost,
}

impl Response<Post> for SearchPostsResponse {
    fn page_info(&self) -> &PageInfo {
        &self.data.search.general.posts.page_info
    }

    fn items(self) -> Vec<Post> {
        self.data
            .search
            .general
            .posts
            .edges
            .into_iter()
            .filter_map(|e| match e.node {
                PostType::SubredditPost(post) => Some(post),
                PostType::ProfilePost(post) => Some(post),
                _ => None,
            })
            .collect()
    }
}
