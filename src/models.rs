use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Chunk {
    pub id: i32,
    pub filename: String,
    pub file_path: String,
    pub size: i64,
    pub content_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewChunk {
    pub filename: String,
    pub file_path: String,
    pub size: i64,
    pub content_type: Option<String>,
}
