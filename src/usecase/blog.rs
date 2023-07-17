use crate::{
    domain::{model::blog::Blog, repository::blog::BlogRepository},
    infrastructure::module::RepositoriesModuleExt,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct BlogUseCase<R: RepositoriesModuleExt> {
    pub repositories: Arc<R>,
}

impl<R> BlogUseCase<R>
where
    R: RepositoriesModuleExt,
{
    pub async fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }
    pub async fn find_blog(&self, id: Uuid) -> anyhow::Result<Option<Blog>> {
        // self.repositories.blog_repository().get_blog(id).await
        self.repositories.blog_repository().get_blog(id).await
    }
    pub async fn all_blogs(&self) -> anyhow::Result<Vec<Blog>> {
        self.repositories.blog_repository().get_blogs().await
    }
    pub async fn create_blog(&self, title: &str, content: &str) -> anyhow::Result<Blog> {
        let blog = Blog::new(Uuid::new_v4(), title, content)?;
        self.repositories
            .blog_repository()
            .insert_blog(&blog)
            .await?;
        Ok(blog)
    }
    pub async fn delete_blog(&self, id: Uuid) -> anyhow::Result<()> {
        self.repositories.blog_repository().delete_blog(id).await
    }
}
