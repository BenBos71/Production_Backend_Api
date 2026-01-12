use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub quantity: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct ItemResponse {
    pub id: i64,
    pub name: String,
    pub quantity: i32,
    pub created_at: String,
}

impl From<Item> for ItemResponse {
    fn from(item: Item) -> Self {
        Self {
            id: item.id,
            name: item.name,
            quantity: item.quantity,
            created_at: item.created_at.to_rfc3339(),
        }
    }
}