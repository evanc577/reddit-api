use serde::Serialize;

use super::subreddit_sort::SubredditSort;

#[derive(Debug, Serialize)]
pub(crate) struct SubredditPostsRequest {
    id: String,
    variables: Variables,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Variables {
    subreddit_name: String,
    #[serde(flatten)]
    sort: SubredditSort,
    after: Option<String>,
    include_subreddit_in_posts: bool,
}

impl SubredditPostsRequest {
    pub(crate) fn new(subreddit_name: String, sort: SubredditSort) -> Self {
        Self {
            id: "3496a5858eb9".into(),
            variables: Variables {
                subreddit_name,
                sort,
                after: None,
                include_subreddit_in_posts: false,
            },
        }
    }

    pub(crate) fn set_cursor(&mut self, after: String) {
        self.variables.after = Some(after);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::structs::sort_range::SortRange;

    #[test]
    fn serialize() {
        let sort = SubredditSort::Top(SortRange::Month);
        let request = SubredditPostsRequest::new("dreamcatcher".into(), sort);
        serde_json::to_string(&request).unwrap();
    }
}
