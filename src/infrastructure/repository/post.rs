use sqlx::{Executor, MySqlPool};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    model::post::Post,
    repository::post::{EditPostParams, PostRepository},
};

pub struct PostRepositoryImpl {
    pub pool: Arc<MySqlPool>,
}

impl PostRepositoryImpl {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl PostRepository for PostRepositoryImpl {
    async fn get_post(&self, id: uuid::Uuid) -> anyhow::Result<Option<Post>> {
        let pool = self.pool.clone();
        let post: Option<Post> = sqlx::query_as(r"select * from post where id=?")
            .bind(id.to_string())
            .fetch_optional(&*pool)
            .await
            .expect("hoge");
        Ok(post)
    }
    async fn get_posts(&self) -> anyhow::Result<Vec<Post>> {
        let pool = self.pool.clone();
        let blog: Vec<Post> = sqlx::query_as(r"SELECT * FROM post")
            .fetch_all(&*pool)
            .await?;
        Ok(blog.into_iter().map(|b| b.into()).collect())
    }
    async fn add_post(&self, post: &Post) -> anyhow::Result<()> {
        let pool = self.pool.clone();
        let query = sqlx::query(r"insert into post (id, avater_id, emoji_id, place_id, title, comment, visited_at, created_at, updated_at) values (?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(post.id.to_string())
            .bind(post.avater_id.clone().to_string())
            .bind(post.emoji_id.clone().to_string())
            .bind(post.place_id.clone().to_string())
            .bind(post.title.clone())
            .bind(post.comment.clone())
            .bind(post.visited_at.clone())
            .bind(post.created_at.clone())
            .bind(post.updated_at.clone()) ;
        pool.execute(query).await?;
        Ok(())
    }
    async fn edit_post(&self, post_id: Uuid, params: &EditPostParams) -> anyhow::Result<Post> {
        let pool = self.pool.clone();
        let mut query = sqlx::QueryBuilder::new(r"update post set");
        if let Some(emoji_id) = params.emoji_id {
            query.push(" emoji_id = ");
            query.push_bind(emoji_id.to_string());
        }
        if let Some(place_id) = params.place_id {
            query.push(" place_id = ");
            query.push_bind(place_id.to_string());
        }
        if let Some(title) = &params.title {
            query.push(" title = ");
            query.push_bind(title);
        }
        if let Some(comment) = &params.comment {
            query.push(" comment = ");
            query.push_bind(comment);
        }
        if let Some(visited_at) = &params.visited_at {
            query.push(" visited_at = ");
            query.push_bind(visited_at);
        }
        query.push("where id =");
        query.push_bind(post_id.to_string());
        let query = query.build_query_as::<Post>();

        let post = query.fetch_one(&*pool).await?;
        Ok(post.into())
    }

    async fn delete_post(&self, post_id: Uuid) -> anyhow::Result<()> {
        let pool = self.pool.clone();
        let query = sqlx::query(r"delete from post where id = ?").bind(post_id.to_string());
        pool.execute(query).await?;
        Ok(())
    }
}

// #[derive(Clone, Debug, sqlx::FromRow)]
// pub struct PostRecord {
//     pub id: String,
//     pub avater_id: String,
//     pub emoji_id: String,
//     pub place_id: String,
//     pub title: String,
//     pub comment: String,
//     pub visited_at: DateTime<Utc>,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,
// }

// impl Into<PostRecord> for Post {
//     fn into(self) -> PostRecord {
//         PostRecord {
//             id: self.id.to_string(),
//             avater_id: self.id.to_string(),
//             emoji_id: self.emoji_id.to_string(),
//             place_id: self.place_id.to_string(),
//         }
//     }
// }

// impl Into<Post> for PostRecord {
//     fn into(self) -> Post {
//         Post {
//             id: Uuid::try_parse(&self.id).unwrap(),
//             avater_id: Uuid::try_parse(&self.avater_id).unwrap(),
//             emoji_id: Uuid::try_parse(&self.emoji_id).unwrap(),
//             place_id: Uuid::try_parse(&self.place_id).unwrap(),
//             title: self.title,
//             comment: self.comment,
//             visited_at: self.visited_at,
//             created_at: self.created_at,
//             updated_at: self.updated_at,
//         }
//     }
// }
