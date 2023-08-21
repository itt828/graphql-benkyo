use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub avater_id: Uuid,
    pub emoji_id: Uuid,
    pub place_id: Uuid,
    pub title: String,
    pub comment: String,
    pub visited_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, sqlx::FromRow)]
pub struct Emoji {
    pub id: Uuid,
    pub name: Uuid,
}

#[derive(Clone, Debug, Deserialize, sqlx::FromRow)]

pub struct Place {
    pub id: Uuid,
    pub name: String,
    pub address: String,
}
