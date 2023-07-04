use crate::service::blog::{BlogService, BlogServiceImpl};
use async_graphql::{EmptySubscription, Object, Schema, SimpleObject, ID};
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

impl From<crate::domain::blog::Blog> for Blog {
    fn from(blog: crate::domain::blog::Blog) -> Self {
        Blog {
            id: ID(blog.id.to_string()),
            title: blog.title.0,
            tags: vec![],
            content: blog.content,
        }
    }
}
