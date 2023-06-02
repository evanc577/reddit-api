use serde::Deserialize;

use super::{Comment, PageInfo};
use crate::traits::Response;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SearchCommentsResponse {
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
    comments: Comments,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Comments {
    page_info: PageInfo,
    edges: Vec<Edge>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Edge {
    node: CommentType,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "__typename")]
pub enum CommentType {
    Comment(Comment),
}

impl Response<Comment> for SearchCommentsResponse {
    fn page_info(&self) -> &PageInfo {
        &self.data.search.general.comments.page_info
    }

    fn items(self) -> Vec<Comment> {
        self.data
            .search
            .general
            .comments
            .edges
            .into_iter()
            .filter_map(|e| match e.node {
                CommentType::Comment(comment) => Some(comment),
            })
            .collect()
    }
}
