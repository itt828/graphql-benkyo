use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{domain::blog::Blog, repository::blog::BlogRepository};

#[async_trait]
pub trait BlogService {
    async fn get_blog(&self, id: Uuid) -> anyhow::Result<Option<Blog>>;
    async fn get_blogs(&self) -> anyhow::Result<Vec<Blog>>;
    async fn create_blog(&self, title: &str, content: &str) -> anyhow::Result<Blog>;
    // fn delete_blog(&self, id: Uuid);
}

pub struct BlogServiceImpl<R: BlogRepository> {
    pub repository: Arc<R>,
}

#[async_trait]
impl<R: BlogRepository + Send + Sync> BlogService for BlogServiceImpl<R> {
    async fn get_blog(&self, id: Uuid) -> anyhow::Result<Option<Blog>> {
        self.repository.get_blog(id).await
    }
    async fn get_blogs(&self) -> anyhow::Result<Vec<Blog>> {
        self.repository.get_blogs().await
    }
    async fn create_blog(&self, title: &str, content: &str) -> anyhow::Result<Blog> {
        let blog = Blog::new(Uuid::new_v4(), title, content)?;
        self.repository.insert_blog(&blog).await?;
        Ok(blog)
    }
    // fn delete_blog(&self, id: Uuid) {
    //     let _ = self.repository.delete_blog(id);
    // }
}
