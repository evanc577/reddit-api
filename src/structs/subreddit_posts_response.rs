use serde::Deserialize;

use super::post::Post;
use super::PageInfo;
use crate::traits::Response;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SubredditPostsResponse {
    data: Data,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    post_feed: PostFeed,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PostFeed {
    elements: Elements,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Elements {
    page_info: PageInfo,
    edges: Vec<Edge>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Edge {
    node: PostType,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "__typename")]
pub enum PostType {
    SubredditPost(Post),
    AdPost,
}

impl Response<Post> for SubredditPostsResponse {
    fn page_info(&self) -> &PageInfo {
        &self.data.post_feed.elements.page_info
    }

    fn items(self) -> Vec<Post> {
        self.data
            .post_feed
            .elements
            .edges
            .into_iter()
            .filter_map(|e| match e.node {
                PostType::SubredditPost(post) => Some(post),
                _ => None,
            })
            .collect()
    }
}
