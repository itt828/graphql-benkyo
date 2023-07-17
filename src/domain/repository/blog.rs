use crate::domain::model::blog::Blog;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait BlogRepository {
    async fn get_blog(&self, id: Uuid) -> anyhow::Result<Option<Blog>>;
    async fn get_blogs(&self) -> anyhow::Result<Vec<Blog>>;
    async fn insert_blog(&self, blog: &Blog) -> anyhow::Result<()>;
    async fn delete_blog(&self, id: Uuid) -> anyhow::Result<()>;
}
