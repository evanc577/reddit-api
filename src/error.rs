/// Reddit errors.
#[derive(Debug)]
pub enum Error {
    /// Errors from Reddit API.
    Reddit(String),
    /// Errors from reqwest, such as network errors.
    Reqwest(reqwest::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reddit(e) => write!(f, "{}", e),
            Self::Reqwest(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}
