use std::time::Duration;

use futures::Stream;
use page_turner::PageTurner;
use reqwest::{header, Client, ClientBuilder};

use crate::access_token::AccessToken;
use crate::constants;
use crate::error::Error;
use crate::structs::*;

/// The main client which all Reddit APIs are called through.
#[derive(Clone)]
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

    pub(crate) async fn authentication(&self) -> Result<String, Error> {
        self.access_token.authentication(&self.client).await
    }

    pub(crate) fn client(&self) -> &Client {
        &self.client
    }
}

/// Builders for Reddit API endpoints
#[buildstructor::buildstructor]
impl RedditClient {
    #[builder(entry = "subreddit_posts_query", exit = "build", visibility = "pub")]
    fn subreddit_posts(
        &self,
        subreddit: String,
        sort: SubredditSort,
    ) -> SubredditPostsQueryBuilder {
        SubredditPostsQueryBuilder {
            client: self.clone(),
            subreddit,
            sort,
        }
    }

    #[builder(entry = "search_posts_query", exit = "build", visibility = "pub")]
    fn search_posts(
        &self,
        query: String,
        sort: SearchPostsSort,
        subreddit: Option<String>,
        nsfw: Option<bool>,
    ) -> SearchPostsQueryBuilder {
        SearchPostsQueryBuilder {
            client: self.clone(),
            query,
            sort,
            subreddit,
            nsfw: nsfw.unwrap_or(false),
        }
    }

    #[builder(entry = "search_comments_query", exit = "build", visibility = "pub")]
    fn search_comments(
        &self,
        query: String,
        sort: SearchCommentsSort,
        subreddit: Option<String>,
        nsfw: Option<bool>,
    ) -> SearchCommentsQueryBuilder {
        SearchCommentsQueryBuilder {
            client: self.clone(),
            query,
            sort,
            subreddit,
            nsfw: nsfw.unwrap_or(false),
        }
    }
}

pub struct SubredditPostsQueryBuilder {
    client: RedditClient,
    subreddit: String,
    sort: SubredditSort,
}

impl SubredditPostsQueryBuilder {
    pub async fn execute(&self) -> impl '_ + Send + Stream<Item = Result<Post, Error>> {
        self.client
            .pages(SubredditPostsRequest::new(
                self.subreddit.clone(),
                self.sort.clone(),
            ))
            .items()
    }
}

pub struct SearchPostsQueryBuilder {
    client: RedditClient,
    query: String,
    sort: SearchPostsSort,
    subreddit: Option<String>,
    nsfw: bool,
}

impl SearchPostsQueryBuilder {
    pub async fn execute(&self) -> impl '_ + Send + Stream<Item = Result<Post, Error>> {
        self.client
            .pages(SearchPostsRequest::new(
                self.query.clone(),
                self.sort.clone(),
                self.subreddit.clone(),
                self.nsfw,
            ))
            .items()
    }
}

pub struct SearchCommentsQueryBuilder {
    client: RedditClient,
    query: String,
    sort: SearchCommentsSort,
    subreddit: Option<String>,
    nsfw: bool,
}

impl SearchCommentsQueryBuilder {
    pub async fn execute(&self) -> impl '_ + Send + Stream<Item = Result<Comment, Error>> {
        self.client
            .pages(SearchCommentsRequest::new(
                self.query.clone(),
                self.sort.clone(),
                self.subreddit.clone(),
                self.nsfw,
            ))
            .items()
    }
}
