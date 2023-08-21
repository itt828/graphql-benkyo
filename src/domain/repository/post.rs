use crate::domain::model::post::Post;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait PostRepository {
    async fn get_post(&self, post_id: Uuid) -> anyhow::Result<Option<Post>>;
    async fn get_posts(&self) -> anyhow::Result<Vec<Post>>;
    async fn add_post(&self, post: &Post) -> anyhow::Result<()>;
    async fn edit_post(&self, post_id: Uuid, params: &EditPostParams) -> anyhow::Result<Post>;
    async fn delete_post(&self, post_id: Uuid) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct EditPostParams {
    pub emoji_id: Option<Uuid>,
    pub place_id: Option<Uuid>,
    pub title: Option<String>,
    pub comment: Option<String>,
    pub visited_at: Option<String>,
}
