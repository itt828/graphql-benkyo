use uuid::Uuid;

use crate::domain::model::post::Emoji;

#[async_trait::async_trait]
pub trait EmojiRepository {
    async fn get_emoji(&self, emoji_id: Uuid) -> anyhow::Result<Option<Emoji>>;
}
