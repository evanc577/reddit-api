use serde::Serialize;

use super::search_comments_sort::SearchCommentsSort;
use crate::constants::request::SEARCH_COMMENTS_ID;
use crate::traits::Request;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SearchCommentsRequest {
    id: &'static str,
    variables: Variables,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Variables {
    query: String,
    product_surface: &'static str,
    after_cursor: Option<String>,
    sort: SearchCommentsSort,
    filters: Vec<Filter>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Filter {
    key: String,
    value: String,
}

impl SearchCommentsRequest {
    pub(crate) fn new(query: String, sort: SearchCommentsSort) -> Self {
        Self {
            id: SEARCH_COMMENTS_ID,
            variables: Variables {
                query,
                product_surface: "android",
                after_cursor: None,
                sort,
                filters: vec![Filter {
                    key: "time_range".into(),
                    value: "all".into(),
                }],
            },
        }
    }
}

impl Request for SearchCommentsRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.variables.after_cursor = Some(cursor)
    }
}
