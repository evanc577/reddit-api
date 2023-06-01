use std::sync::Arc;

use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use tokio::sync::Mutex;

use crate::error::Error;

#[derive(Default)]
pub(crate) struct AccessToken {
    token: Arc<Mutex<Option<AccessTokenInternal>>>,
}

impl AccessToken {
    /// Return stored authorization, refresh it if needed
    pub(crate) async fn authentication(&self, client: &Client) -> Result<String, Error> {
        let mut access_token_guard = self.token.lock().await;
        if access_token_guard.is_none() {
            // Get a new token if none exists
            let new_token = AccessTokenInternal::access_token(client).await?;
            *access_token_guard = Some(new_token);
        } else if let Some(token) = &*access_token_guard {
            // Get a new token if current one is expiring soon
            let expiry = token.expiry;
            let buffer_time = Duration::new(4 * 60 * 60, 0); // 4 hours
            if expiry - OffsetDateTime::now_utc() < buffer_time {
                let new_token = AccessTokenInternal::access_token(client).await?;
                *access_token_guard = Some(new_token);
            }
        }
        Ok(format!(
            "Bearer {}",
            access_token_guard.as_ref().unwrap().access_token.clone()
        ))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct AccessTokenInternal {
    access_token: String,
    #[serde(rename = "expiry_ts")]
    #[serde(deserialize_with = "crate::utils::deserialize_timestamp")]
    expiry: OffsetDateTime,
    #[serde(deserialize_with = "crate::utils::deserialize_duration")]
    expires_in: Duration,
    scope: Vec<String>,
    token_type: String,
}

#[derive(Serialize)]
struct Body {
    scopes: Vec<String>,
}

impl AccessTokenInternal {
    async fn access_token(client: &Client) -> Result<Self, Error> {
        static ENDPOINT: &str = "https://accounts.reddit.com/api/access_token";
        static AUTHORIZATION: &str = "basic b2hYcG9xclpZdWIxa2c6";
        let body = Body {
            scopes: vec!["*".into(), "email".into(), "pii".into()],
        };
        let response = client
            .post(ENDPOINT)
            .header(header::AUTHORIZATION, AUTHORIZATION)
            .json(&body)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(Error::Reddit(response.text().await?))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::RedditClient;

    #[tokio::test]
    async fn works() {
        RedditClient::new().unwrap();
    }
}
