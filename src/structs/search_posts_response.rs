use serde::Deserialize;

use super::{PageInfo, SubredditPost};
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

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "__typename")]
pub enum PostType {
    SubredditPost(SubredditPost),
    AdPost,
}

impl Response<SubredditPost> for SearchPostsResponse {
    fn page_info(&self) -> &PageInfo {
        &self.data.search.general.posts.page_info
    }

    fn items(self) -> Vec<SubredditPost> {
        self.data
            .search
            .general
            .posts
            .edges
            .into_iter()
            .filter_map(|e| match e.node {
                PostType::SubredditPost(post) => Some(post),
                _ => None,
            })
            .collect()
    }
}
