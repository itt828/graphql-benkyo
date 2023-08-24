use chrono::{DateTime, Utc};
use sqlx::{Executor, MySqlPool};
use std::sync::Arc;
use uuid::Uuid;

use crate::models::Post;

pub async fn get_post(pool: Arc<MySqlPool>, id: Uuid) -> anyhow::Result<Option<Post>> {
    let post: Option<Post> = sqlx::query_as(r"select * from post where id=?")
        .bind(id.to_string())
        .fetch_optional(&*pool)
        .await
        .expect("hoge");
    Ok(post)
}
pub async fn all_posts(pool: Arc<MySqlPool>) -> anyhow::Result<Vec<Post>> {
    let posts: Vec<Post> = sqlx::query_as(r"SELECT * FROM post")
        .fetch_all(&*pool)
        .await?;
    Ok(posts)
}
pub async fn create_post(
    pool: Arc<MySqlPool>,
    avater_id: Uuid,
    emoji_id: Uuid,
    place_id: Uuid,
    title: String,
    comment: String,
    visited_at: Option<DateTime<Utc>>,
) -> anyhow::Result<Post> {
    let created_at = Utc::now();
    let updated_at = Utc::now();
    let visited_at = visited_at.map_or(Utc::now(), |v| v);
    let post = Post {
        id: Uuid::new_v4(),
        avater_id,
        emoji_id,
        title,
        place_id,
        comment,
        created_at,
        updated_at,
        visited_at,
    };
    let query = sqlx::query(r"insert into post (id, avater_id, emoji_id, place_id, title, comment, visited_at, created_at, updated_at) values (?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(post.id.to_string())
            .bind(post.avater_id.clone().to_string())
            .bind(post.emoji_id.clone().to_string())
            .bind(post.place_id.clone().to_string())
            .bind(post.title.clone())
            .bind(post.comment.clone())
            .bind(post.visited_at)
            .bind(post.created_at)
            .bind(post.updated_at) ;
    pool.execute(query).await?;
    Ok(post)
}
async fn edit_post(
    pool: Arc<MySqlPool>,
    post_id: Uuid,
    params: &EditPostParams,
) -> anyhow::Result<Post> {
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
    Ok(post)
}

pub async fn delete_post(pool: Arc<MySqlPool>, post_id: Uuid) -> anyhow::Result<()> {
    let query = sqlx::query(r"delete from post where id = ?").bind(post_id.to_string());
    pool.execute(query).await?;
    Ok(())
}

#[derive(Debug)]
pub struct EditPostParams {
    pub emoji_id: Option<Uuid>,
    pub place_id: Option<Uuid>,
    pub title: Option<String>,
    pub comment: Option<String>,
    pub visited_at: Option<String>,
}
