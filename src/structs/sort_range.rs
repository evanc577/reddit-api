use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SortRange {
    Hour,
    Day,
    Week,
    Month,
    Year,
    All,
}
