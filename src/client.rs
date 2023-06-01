use std::sync::Arc;
use std::time::Duration;

use futures::Stream;
use page_turner::PageTurner;
use reqwest::{header, Client, ClientBuilder};
use time::OffsetDateTime;
use tokio::sync::Mutex;

use crate::error::Error;
use crate::structs::{SubredditPost, SubredditPostsRequest, SubredditSort};
use crate::{auth, constants};

/// The main client which all Reddit APIs are called through.
pub struct RedditClient {
    pub(crate) client: Client,
    pub(crate) access_token: Token,
}

impl RedditClient {
    /// Create a new RedditClient.
    /// This method authenticates with Reddit.
    pub async fn init() -> Result<Self, Error> {
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
        let access_token = auth::access_token(&client).await?;

        Ok(Self {
            client,
            access_token: Token::new(access_token),
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
}

#[derive(Debug, Clone)]
pub(crate) struct Token {
    token: Arc<Mutex<auth::AccessToken>>,
}

impl Token {
    pub(crate) fn new(access_token: auth::AccessToken) -> Self {
        Self {
            token: Arc::new(Mutex::new(access_token)),
        }
    }

    /// Return stored authorization, refresh it if needed
    pub(crate) async fn authentication(&self, client: &Client) -> Result<String, Error> {
        let mut access_token_guard = self.token.lock().await;
        let expiry = access_token_guard.expiry;
        let buffer_time = Duration::new(4 * 60 * 60, 0); // 4 hours
        if expiry - OffsetDateTime::now_utc() < buffer_time {
            let new_token = auth::access_token(client).await?;
            *access_token_guard = new_token;
        }
        Ok(format!(
            "Bearer {}",
            access_token_guard.access_token.clone()
        ))
    }
}
