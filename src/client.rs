use std::time::Duration;

use futures::Stream;
use page_turner::PageTurner;
use reqwest::{header, Client, ClientBuilder};

use crate::access_token::AccessToken;
use crate::constants;
use crate::error::Error;
use crate::structs::{
    SearchPostsRequest, SearchPostsSort, SubredditPost, SubredditPostsRequest, SubredditSort,
};

/// The main client which all Reddit APIs are called through.
pub struct RedditClient {
    pub(crate) client: Client,
    pub(crate) access_token: AccessToken,
}

impl RedditClient {
    /// Create a new RedditClient.
    pub fn new() -> Result<Self, Error> {
        #[rustfmt::skip]
        let headers = {
            use constants::header as ch;
            use header as h;
            use header::HeaderName as HN;
            use header::HeaderValue as HV;

            let mut headers = header::HeaderMap::new();
            headers.insert(h::ACCEPT_ENCODING, HV::from_static(ch::ACCEPT_ENCODING));
            headers.insert(h::USER_AGENT, HV::from_static(ch::USER_AGENT));
            headers.insert(HN::from_static("client-vendor-id"), HV::from_static(ch::CLIENT_VENDOR_ID));
            headers.insert(HN::from_static("x-reddit-compression"), HV::from_static(ch::X_REDDIT_COMPRESSION));
            headers.insert(HN::from_static("x-reddit-media-codecs"), HV::from_static(ch::X_REDDIT_MEDIA_CODECS));
            headers.insert(HN::from_static("x-reddit-qos"), HV::from_static(ch::X_REDDIT_QOS));
            headers.insert(HN::from_static("x-reddit-retry"), HV::from_static(ch::X_REDDIT_RETRY));
            headers
        };

        let client = ClientBuilder::new()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .http1_title_case_headers()
            .build()?;

        Ok(Self {
            client,
            access_token: Default::default(),
        })
    }

    /// Get a stream of posts from a subreddit.
    pub async fn subreddit_posts(
        &self,
        subreddit: impl AsRef<str>,
        sort: SubredditSort,
    ) -> impl '_ + Send + Stream<Item = Result<SubredditPost, Error>> {
        self.pages(SubredditPostsRequest::new(
            subreddit.as_ref().to_owned(),
            sort,
        ))
        .items()
    }

    /// Search posts
    pub async fn search_posts(
        &self,
        query: impl AsRef<str>,
        sort: SearchPostsSort,
    ) -> impl '_ + Send + Stream<Item = Result<SubredditPost, Error>> {
        self.pages(SearchPostsRequest::new(query.as_ref().to_owned(), sort))
            .items()
    }

    pub(crate) async fn authentication(&self) -> Result<String, Error> {
        self.access_token.authentication(&self.client).await
    }

    pub(crate) fn client(&self) -> &Client {
        &self.client
    }
}
