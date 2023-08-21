use crate::domain::{model::post::Emoji, repository::emoji::EmojiRepository};
use sqlx::MySqlPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct EmojiRepositoryImpl {
    pub pool: Arc<MySqlPool>,
}

impl EmojiRepositoryImpl {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl EmojiRepository for EmojiRepositoryImpl {
    async fn get_emoji(&self, avater_id: Uuid) -> anyhow::Result<Option<Emoji>> {
        Ok(None)
    }
}
