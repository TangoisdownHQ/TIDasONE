use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Package {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub inventory_item_id: Option<Uuid>,
    pub status: String,
    pub destination: String,
    pub nft_token: Option<String>,
    pub created_at: DateTime<Utc>,
}

