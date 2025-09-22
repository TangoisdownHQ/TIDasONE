use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub nft_token_id: Option<String>,
    pub identity_hash: Option<String>,
    pub created_at: DateTime<Utc>, // ✅ Make this optional
}

