use std::{str::FromStr, sync::Arc};

use sqlx::MySqlPool;
use uuid::Uuid;

use crate::repository::blog::BlogRepository;

#[derive(sqlx::FromRow)]
struct Blog {
    id: String,
    title: String,
    content: String,
}

impl From<Blog> for crate::domain::blog::Blog {
    fn from(value: Blog) -> Self {
        Self {
            id: Uuid::from_str(&value.id).unwrap(),
            title: value.title,
            content: value.content,
            authors: vec![],
            tags: vec![],
        }
    }
}

pub struct BlogRepositoryImpl {
    pub pool: Arc<MySqlPool>,
}

#[async_trait::async_trait]
impl BlogRepository for BlogRepositoryImpl {
    async fn get_blog(&self, id: uuid::Uuid) -> anyhow::Result<Option<crate::domain::blog::Blog>> {
        let pool = self.pool.clone();
        let blog: Option<Blog> = sqlx::query_as(r"SELECT * FROM `blog`")
            .fetch_optional(&*pool)
            .await?;
        Ok(blog.map(|b| b.into()))
    }
}
