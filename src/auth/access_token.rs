use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use crate::constants;
use crate::error::Error;

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AccessToken {
    pub(crate) access_token: String,
    #[serde(rename = "expiry_ts")]
    #[serde(deserialize_with = "crate::utils::deserialize_timestamp")]
    pub(crate) expiry: OffsetDateTime,
    #[serde(deserialize_with = "crate::utils::deserialize_duration")]
    expires_in: Duration,
    scope: Vec<String>,
    token_type: String,
}

#[derive(Serialize)]
struct Body {
    scopes: Vec<String>,
}

pub(crate) async fn access_token(client: &Client) -> Result<AccessToken, Error> {
    static ENDPOINT: &str = "https://accounts.reddit.com/api/access_token";
    let body = Body {
        scopes: vec!["*".into(), "email".into(), "pii".into()],
    };
    let response = client
        .post(ENDPOINT)
        .header(header::AUTHORIZATION, constants::header::AUTHORIZATION)
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

#[cfg(test)]
mod test {
    use crate::RedditClient;

    #[tokio::test]
    async fn works() {
        RedditClient::init().await.unwrap();
    }
}
