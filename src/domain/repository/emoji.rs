use uuid::Uuid;

use crate::domain::model::post::Emoji;

#[async_trait::async_trait]
pub trait EmojiRepository {
    async fn get_emoji(&self, emoji_id: Uuid) -> anyhow::Result<Option<Emoji>>;
    async fn get_emojis(&self, emoji_ids: Option<Vec<Uuid>>) -> anyhow::Result<Vec<Emoji>>;
    async fn register_emojis(&self, emojis: &[Emoji]) -> anyhow::Result<()>;
}
