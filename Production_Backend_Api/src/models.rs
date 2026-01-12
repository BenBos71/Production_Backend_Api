use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub quantity: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateItemRequest {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    
    #[validate(range(min = 0, message = "Quantity must be 0 or greater"))]
    pub quantity: i32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemResponse {
    pub id: i64,
    pub name: String,
    pub quantity: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: std::collections::HashMap<String, Vec<String>>,
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

impl From<validator::ValidationErrors> for ErrorResponse {
    fn from(errors: validator::ValidationErrors) -> Self {
        let mut error_map = std::collections::HashMap::new();
        
        for (field, errors) in errors.field_errors() {
            let error_messages: Vec<String> = errors
                .iter()
                .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                .collect();
            
            if !error_messages.is_empty() {
                error_map.insert(field.to_string(), error_messages);
            }
        }
        
        Self {
            error: "validation_error".to_string(),
            details: error_map,
        }
    }
}