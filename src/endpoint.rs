use async_trait::async_trait;
use page_turner::prelude::*;
use reqwest::header;

use crate::constants::GRAPHQL_URL;
use crate::error::Error;
use crate::traits::{Request, Response};
use crate::{structs, RedditClient};

macro_rules! impl_page_turner {
    ($req:ty,$resp:ty,$item:ty) => {
        #[async_trait]
        impl PageTurner<$req> for RedditClient {
            type PageItem = $item;
            type PageError = Error;

            async fn turn_page(&self, mut request_data: $req) -> PageTurnerOutput<Self, $req> {
                // Send request
                let auth = self.authentication().await?;
                let request = self
                    .client()
                    .post(GRAPHQL_URL)
                    .header(header::AUTHORIZATION, auth)
                    .json(&request_data);
                let response = request.send().await?;
                let status = response.status();
                if !status.is_success() {
                    return Err(Error::Reddit(response.text().await?));
                }

                // Parse response
                // let text = response.text().await?;
                // eprintln!("{}", &text);
                // let parsed_response: $resp = serde_json::from_str(&text).unwrap();
                let parsed_response: $resp = response.json().await?;
                let page_info = parsed_response.page_info().clone();
                let items = parsed_response.items();
                if let Some(cursor) = page_info.end_cursor {
                    request_data.set_cursor(cursor);
                    Ok(TurnedPage::next(items, request_data))
                } else {
                    Ok(TurnedPage::last(items))
                }
            }
        }
    };
}

impl_page_turner!(
    structs::SubredditPostsRequest,
    structs::SubredditPostsResponse,
    structs::Post
);
impl_page_turner!(
    structs::SearchPostsRequest,
    structs::SearchPostsResponse,
    structs::Post
);
impl_page_turner!(
    structs::SearchCommentsRequest,
    structs::SearchCommentsResponse,
    structs::Comment
);
