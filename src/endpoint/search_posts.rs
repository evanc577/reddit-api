use async_trait::async_trait;
use page_turner::prelude::*;
use reqwest::header;

use crate::constants::GRAPHQL_URL;
use crate::error::Error;
use crate::structs::{SubredditPost, SearchPostsRequest, SearchPostsResponse};
use crate::RedditClient;

#[async_trait]
impl PageTurner<SearchPostsRequest> for RedditClient {
    type PageItem = SubredditPost;
    type PageError = Error;

    async fn turn_page(
        &self,
        mut request: SearchPostsRequest,
    ) -> PageTurnerOutput<Self, SearchPostsRequest> {
        // Send request
        let auth = self.access_token.authentication(&self.client).await?;
        let response = self
            .client
            .post(GRAPHQL_URL)
            .header(header::AUTHORIZATION, auth)
            .json(&request)
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            return Err(Error::Reddit(response.text().await?));
        }

        // Parse response
        let parsed_response: SearchPostsResponse = response.json().await?;
        let page_info = parsed_response.page_info().clone();
        let posts = parsed_response.posts();
        if page_info.has_next_page {
            request.set_cursor(page_info.end_cursor);
            Ok(TurnedPage::next(posts, request))
        } else {
            Ok(TurnedPage::last(posts))
        }
    }
}
