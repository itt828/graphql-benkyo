use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{domain::blog::Blog, repository::blog::BlogRepository};

#[async_trait]
pub trait BlogService {
    async fn get_blog(&self, id: Uuid) -> anyhow::Result<Option<Blog>>;
    // fn post_blog(&self, new_blog: Blog);
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
    // fn post_blog(&self, new_blog: Blog) {
    //     self.repository.create_blog(new_blog);
    // }
    // fn delete_blog(&self, id: Uuid) {
    //     let _ = self.repository.delete_blog(id);
    // }
}
