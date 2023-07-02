use crate::service::blog::{BlogService, BlogServiceImpl};
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, ID};
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

pub type GQLSchema<R> =
    Schema<Query<BlogServiceImpl<R>>, Mutation<BlogServiceImpl<R>>, EmptySubscription>;

pub struct Query<BS: BlogService> {
    pub blog_service: Arc<BS>,
}

#[Object]
impl<BS: BlogService + Sync + Send> Query<BS> {
    pub async fn blog(&self, id: ID) -> anyhow::Result<Option<Blog>> {
        self.blog_service
            .get_blog(Uuid::from_str(&id.0).unwrap())
            .await
            .map(|b| b.map(|b| b.into()))
    }
    pub async fn blogs(&self) -> anyhow::Result<Vec<Blog>> {
        self.blog_service
            .get_blogs()
            .await
            .map(|blogs| blogs.into_iter().map(|blog| blog.into()).collect())
    }
}

pub struct Mutation<BS: BlogService> {
    pub blog_service: Arc<BS>,
}

#[Object]
impl<BS: BlogService + Sync + Send> Mutation<BS> {
    pub async fn add_blog(&self, title: String, content: String) -> anyhow::Result<Blog> {
        let blog = self.blog_service.create_blog(&title, &content).await?;
        Ok(blog.into())
    }
}

#[derive(SimpleObject)]
pub struct Blog {
    id: ID,
    title: String,
    tags: Vec<Tag>,
    content: String,
}

#[derive(SimpleObject)]
struct User {
    id: ID,
    name: String,
}
#[derive(SimpleObject)]
pub struct Tag {
    id: ID,
    name: String,
}

impl Into<Blog> for crate::domain::blog::Blog {
    fn into(self) -> Blog {
        Blog {
            id: ID(self.id.to_string()),
            title: self.title.0,
            tags: vec![],
            content: self.content,
        }
    }
}
