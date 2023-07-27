use super::model::Blog;
use crate::interface::modules::Modules;
use async_graphql::Object;
use openidconnect::core::CoreClient;
use std::sync::Arc;

pub struct Mutation {
    pub modules: Arc<Modules>,
    pub oidc_client: Arc<CoreClient>,
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
