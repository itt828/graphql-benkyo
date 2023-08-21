use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub avaters: Vec<Uuid>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Avater {
    pub id: Uuid,
    pub name: String,
}
