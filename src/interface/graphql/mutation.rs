use std::sync::Arc;

use async_graphql::Object;

use crate::interface::modules::Modules;

use super::model::Blog;

pub struct Mutation {
    pub modules: Arc<Modules>,
}

#[Object]
impl Mutation {
    pub async fn add_blog(&self, title: String, content: String) -> anyhow::Result<Blog> {
        let blog = self
            .modules
            .blog_use_case
            .create_blog(&title, &content)
            .await?;
        Ok(blog.into())
    }
}
