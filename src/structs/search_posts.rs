use serde::Serialize;

use crate::constants::request::SEARCH_POSTS_ID;

use super::search_posts_sort::SearchPostsSort;

#[derive(Debug, Serialize)]
pub(crate) struct SearchPostsRequest {
    id: &'static str,
    variables: Variables,
}

#[derive(Debug)]
struct Variables {
    query: String,
    product_surface: &'static str,
    after_cursor: Option<String>,
    sort: SearchPostsSort,
    filters: Vec<Filter>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Filter {
    key: String,
    value: String,
}

impl Serialize for Variables {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        VariablesSerialized::from(self).serialize(serializer)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VariablesSerialized {
    query: String,
    product_surface: &'static str,
    after_cursor: Option<String>,
    sort: String,
    filters: Vec<Filter>,
}

impl From<&Variables> for VariablesSerialized {
    fn from(value: &Variables) -> Self {
        let mut filters = value.filters.clone();
        match &value.sort {
            SearchPostsSort::Relevance(r)
            | SearchPostsSort::Top(r)
            | SearchPostsSort::Comments(r) => {
                filters.push(Filter {
                    key: "time_range".into(),
                    value: r.to_string(),
                });
            }
            _ => (),
        }
        Self {
            query: value.query.clone(),
            product_surface: value.product_surface.clone(),
            after_cursor: value.after_cursor.clone(),
            sort: value.sort.to_string(),
            filters,
        }
    }
}

impl SearchPostsRequest {
    pub(crate) fn new(query: String, sort: SearchPostsSort) -> Self {
        Self {
            id: SEARCH_POSTS_ID,
            variables: Variables {
                query,
                product_surface: "android",
                after_cursor: None,
                sort,
                filters: Vec::new(),
            },
        }
    }

    pub(crate) fn set_cursor(&mut self, after: String) {
        self.variables.after_cursor = Some(after)
    }
}
