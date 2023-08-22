use crate::domain::{model::post::Emoji, repository::emoji::EmojiRepository};
use sqlx::MySqlPool;
use std::{str::FromStr, sync::Arc};
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
        let pool = self.pool.clone();
        let emoji: Option<Emoji> = sqlx::query_as(r"select * from emoji where id=?")
            .bind(emoji_id.to_string())
            .fetch_optional(&*pool)
            .await?;
        Ok(emoji.map(|v| v.into()))
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

// #[derive(Clone, Debug, sqlx::FromRow)]
// pub struct EmojiRecord {
//     id: String,
//     name: String,
// }

// impl Into<EmojiRecord> for Emoji {
//     fn into(self) -> EmojiRecord {
//         EmojiRecord {
//             id: self.id.to_string(),
//             name: self.name,
//         }
//     }
// }

// impl Into<Emoji> for EmojiRecord {
//     fn into(self) -> Emoji {
//         Emoji {
//             id: Uuid::from_str(&self.id).unwrap(),
//             name: self.name,
//         }
//     }
// }
