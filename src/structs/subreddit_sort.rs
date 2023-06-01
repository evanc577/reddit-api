use serde::Serialize;

use super::sort_range::SortRange;

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "sort", content = "range")]
pub enum SubredditSort {
    Hot,
    New,
    Top(SortRange),
    Controversial(SortRange),
    Rising,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize() {
        let sort = SubredditSort::New;
        assert_eq!(r#"{"sort":"NEW"}"#, serde_json::to_string(&sort).unwrap());
        let sort = SubredditSort::Top(SortRange::Week);
        assert_eq!(
            r#"{"sort":"TOP","range":"WEEK"}"#,
            serde_json::to_string(&sort).unwrap()
        );
    }
}
