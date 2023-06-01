pub(crate) mod header {
    pub static ACCEPT_ENCODING: &str = "gzip, deflate";
    pub static USER_AGENT: &str = "Reddit/Version 2023.21.0/Build 956283/Android 9";

    pub static CLIENT_VENDOR_ID: &str = "64f84305-0fc1-4643-b156-d61b35a882d9";
    pub static X_REDDIT_COMPRESSION: &str = "1";
    pub static X_REDDIT_MEDIA_CODECS: &str = "available-codecs=video/avc, video/hevc";
    pub static X_REDDIT_QOS: &str = "down-rate-mbps=3.200";
    pub static X_REDDIT_RETRY: &str = "algo=no-retries";
}

pub(crate) static GRAPHQL_URL: &str = "https://gql.reddit.com/";
