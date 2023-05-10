use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Song {
    pub title: String,
    pub author: String,
    pub image: Option<String>,
}
