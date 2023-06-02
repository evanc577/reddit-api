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

impl std::fmt::Display for SortRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SortRange::Hour => "hour",
            SortRange::Day => "day",
            SortRange::Week => "week",
            SortRange::Month => "month",
            SortRange::Year => "year",
            SortRange::All => "all",
        };
        write!(f, "{s}")
    }
}
