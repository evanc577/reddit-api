use serde::Serialize;

use super::subreddit_sort::SubredditSort;
use crate::constants::request::SUBREDDIT_POSTS_ID;
use crate::traits::Request;

#[derive(Debug, Serialize)]
pub(crate) struct SubredditPostsRequest {
    id: &'static str,
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
            id: SUBREDDIT_POSTS_ID,
            variables: Variables {
                subreddit_name,
                sort,
                after: None,
                include_subreddit_in_posts: false,
            },
        }
    }
}

impl Request for SubredditPostsRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.variables.after = Some(cursor);
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
