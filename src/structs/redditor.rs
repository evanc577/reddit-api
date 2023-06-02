use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Redditor {
    pub id: String,
    pub name: String,
}
