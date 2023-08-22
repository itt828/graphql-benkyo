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
