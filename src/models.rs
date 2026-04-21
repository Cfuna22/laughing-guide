use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, FromRow)]
pub struct Link {
    pub id: Uuid,
    pub slug: String,
    pub original_url: String,
    pub clicks: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLinkRequest {
    pub original_url: String,
    pub custom_slug: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLinkRequest {
    pub original_url: Option<String>,
    pub slug: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateLinkResponse {
    pub short_url: String,
    pub slug: String,
    pub original_url: String,
}
