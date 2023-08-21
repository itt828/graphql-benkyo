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
    async fn get_emoji(&self, emoji_id: Uuid) -> anyhow::Result<Option<Emoji>> {
        Ok(None)
    }
    async fn register_emojis(&self, emojis: &[Emoji]) -> anyhow::Result<()> {
        let pool = self.pool.clone();
        let mut query = sqlx::QueryBuilder::new(r"insert into emoji (id, name)");
        query.push_values(emojis.iter(), |mut b, emoji| {
            b.push_bind(emoji.id.to_string())
                .push_bind(emoji.name.to_string());
        });
        let query = query.build();
        query.execute(&*pool).await?;
        // query.fetch(&*pool).await?;
        Ok(())
    }
}
