use std::sync::Arc;

use sqlx::MySqlPool;
use uuid::Uuid;

use crate::models::Emoji;

pub async fn get_emoji(pool: Arc<MySqlPool>, emoji_id: Uuid) -> anyhow::Result<Option<Emoji>> {
    let emoji: Option<Emoji> = sqlx::query_as(r"select * from emoji where id=?")
        .bind(emoji_id.to_string())
        .fetch_optional(&*pool)
        .await?;
    Ok(emoji)
}

pub async fn get_emojis(
    pool: Arc<MySqlPool>,
    emoji_ids: Option<Vec<Uuid>>,
) -> anyhow::Result<Vec<Emoji>> {
    let emojis = match emoji_ids {
        Some(emoji_ids) => {
            let query_string = format!(
                "select * from emoji where id in (?{})",
                ", ?".repeat(emoji_ids.len() - 1)
            );
            let mut query = sqlx::query_as(&query_string);

            for emoji_id in emoji_ids.iter() {
                query = query.bind(emoji_id.to_string());
            }

            query.fetch_all(&*pool).await?
        }
        None => {
            let query = sqlx::query_as("select * from emoji");
            query.fetch_all(&*pool).await?
        }
    };
    Ok(emojis)
}

pub async fn register_emojis(pool: Arc<MySqlPool>, emojis: &[Emoji]) -> anyhow::Result<()> {
    let mut query = sqlx::QueryBuilder::new(r"insert into emoji (id, name)");
    query.push_values(emojis.iter(), |mut b, emoji| {
        b.push_bind(emoji.id.to_string())
            .push_bind(emoji.name.to_string());
    });
    let query = query.build();
    query.execute(&*pool).await?;
    Ok(())
}
