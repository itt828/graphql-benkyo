use crate::domain::{model::blog::BlogTitle, repository::blog::BlogRepository};
use sqlx::{Executor, MySqlPool};
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

type DomainBlog = crate::domain::model::blog::Blog;

#[derive(sqlx::FromRow)]
struct Blog {
    id: String,
    title: String,
    content: String,
}

impl From<Blog> for DomainBlog {
    fn from(value: Blog) -> Self {
        Self {
            id: Uuid::from_str(&value.id).unwrap(),
            title: BlogTitle::new(&value.title).unwrap(),
            content: value.content,
            authors: vec![],
            tags: vec![],
        }
    }
}

pub struct BlogRepositoryImpl {
    pub pool: Arc<MySqlPool>,
}

impl BlogRepositoryImpl {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl BlogRepository for BlogRepositoryImpl {
    async fn get_blog(&self, id: uuid::Uuid) -> anyhow::Result<Option<DomainBlog>> {
        let pool = self.pool.clone();
        let blog: Option<Blog> = sqlx::query_as(r"select * from `blog` where id=?")
            .bind(id.to_string())
            .fetch_optional(&*pool)
            .await?;
        Ok(blog.map(|b| b.into()))
    }
    async fn get_blogs(&self) -> anyhow::Result<Vec<DomainBlog>> {
        let pool = self.pool.clone();
        let blog: Vec<Blog> = sqlx::query_as(r"SELECT * FROM `blog`")
            .fetch_all(&*pool)
            .await?;
        Ok(blog.into_iter().map(|b| b.into()).collect())
    }
    async fn insert_blog(&self, blog: &DomainBlog) -> anyhow::Result<()> {
        let pool = self.pool.clone();
        let query = sqlx::query(r"insert into blog (`id`, `title`, `content`) values (?, ?, ?)")
            .bind(blog.id.to_string())
            .bind(blog.title.0.clone())
            .bind(blog.content.clone());
        pool.execute(query).await?;
        Ok(())
    }
    async fn delete_blog(&self, _id: uuid::Uuid) -> anyhow::Result<()> {
        Ok(())
    }
}