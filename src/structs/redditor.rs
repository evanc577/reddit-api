use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "__typename")]
pub enum Redditor {
    Redditor {
        id: String,
        name: String,
    },
    DeletedRedditor,
    UnavailableRedditor,
}
