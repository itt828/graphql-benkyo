use crate::{
    domain::{model::post::Post, repository::post::PostRepository},
    infrastructure::module::RepositoriesModuleExt,
};
use chrono::{DateTime, Local, Utc};
use std::sync::Arc;
use uuid::Uuid;

pub struct PostUseCase<R: RepositoriesModuleExt> {
    pub repositories: Arc<R>,
}

impl<R> PostUseCase<R>
where
    R: RepositoriesModuleExt,
{
    pub async fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }
    pub async fn get_post(&self, id: Uuid) -> anyhow::Result<Option<Post>> {
        self.repositories.post_repository().get_post(id).await
    }
    pub async fn all_posts(&self) -> anyhow::Result<Vec<Post>> {
        self.repositories.post_repository().get_posts().await
    }
    pub async fn create_post(
        &self,
        avater_id: Uuid,
        emoji_id: Uuid,
        place_id: Uuid,
        title: String,
        comment: String,
        visited_at: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Post> {
        let created_at = Utc::now();
        let updated_at = Utc::now();
        let visited_at = visited_at.map_or(Utc::now(), |v| v);
        let post = Post {
            id: Uuid::new_v4(),
            avater_id,
            emoji_id,
            title,
            place_id,
            comment,
            created_at,
            updated_at,
            visited_at,
        };
        self.repositories.post_repository().add_post(&post).await?;
        Ok(post)
    }
    pub async fn delete_blog(&self, id: Uuid) -> anyhow::Result<()> {
        self.repositories.post_repository().delete_post(id).await
    }
}
