use serde::Serialize;

use super::sort_range::SortRange;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "sort", content = "range")]
pub enum SearchPostsSort {
    Relevance(SortRange),
    Hot,
    New,
    Top(SortRange),
    Comments(SortRange),
}

impl std::fmt::Display for SearchPostsSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SearchPostsSort::Relevance(_) => "RELEVANCE",
            SearchPostsSort::Hot => "HOT",
            SearchPostsSort::New => "NEW",
            SearchPostsSort::Top(_) => "TOP",
            SearchPostsSort::Comments(_) => "COMMENTS",
        };
        write!(f, "{s}")
    }
}
