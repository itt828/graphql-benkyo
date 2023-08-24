use std::sync::Arc;

use uuid::Uuid;

use crate::{
    domain::{model::post::Emoji, repository::emoji::EmojiRepository},
    infrastructure::module::RepositoriesModuleExt,
};

pub struct EmojiUseCase<R: RepositoriesModuleExt> {
    pub repositories: Arc<R>,
}

impl<R> EmojiUseCase<R>
where
    R: RepositoriesModuleExt,
{
    pub async fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }
    pub async fn get_emoji(&self, id: Uuid) -> anyhow::Result<Option<Emoji>> {
        self.repositories.emoji_repository().get_emoji(id).await
    }
    pub async fn get_emojis(&self, emoji_ids: Option<Vec<Uuid>>) -> anyhow::Result<Vec<Emoji>> {
        self.repositories
            .emoji_repository()
            .get_emojis(emoji_ids)
            .await
    }
    pub async fn register_emojis(&self, emojis: &[Emoji]) -> anyhow::Result<()> {
        self.repositories
            .emoji_repository()
            .register_emojis(emojis)
            .await
    }
}
