use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SearchCommentsSort {
    Relevance,
    New,
    Top,
}
