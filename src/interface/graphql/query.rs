use super::model::Blog;
use crate::interface::modules::Modules;
use async_graphql::{Object, ID};
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

pub struct Query {
    pub modules: Arc<Modules>,
}

#[Object]
impl Query {
    pub async fn blog(&self, id: ID) -> anyhow::Result<Option<Blog>> {
        self.modules
            .blog_use_case
            .find_blog(Uuid::from_str(&id.0).unwrap())
            .await
            .map(|b| b.map(|b| b.into()))
    }
    pub async fn blogs(&self) -> anyhow::Result<Vec<Blog>> {
        self.modules
            .blog_use_case
            .all_blogs()
            .await
            .map(|blogs| blogs.into_iter().map(|blog| blog.into()).collect())
    }
}
