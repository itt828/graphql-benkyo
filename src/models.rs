use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::fmt::Hyphenated;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub avaters: Vec<Uuid>,
}

#[derive(Debug, Clone, Deserialize, sqlx::FromRow)]
pub struct Avater {
    #[sqlx(try_from = "Hyphenated")]
    pub id: Uuid,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, sqlx::FromRow)]
pub struct Post {
    #[sqlx(try_from = "Hyphenated")]
    pub id: Uuid,
    #[sqlx(try_from = "Hyphenated")]
    pub avater_id: Uuid,
    #[sqlx(try_from = "Hyphenated")]
    pub emoji_id: Uuid,
    #[sqlx(try_from = "Hyphenated")]
    pub place_id: Uuid,
    pub title: String,
    pub comment: String,
    pub visited_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, sqlx::FromRow)]
pub struct Emoji {
    #[sqlx(try_from = "Hyphenated")]
    pub id: Uuid,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, sqlx::FromRow)]

pub struct Place {
    #[sqlx(try_from = "Hyphenated")]
    pub id: Uuid,
    pub name: String,
    pub address: String,
}
