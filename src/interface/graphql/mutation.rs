use super::model::{Avater, Place, Post};
use crate::usecase::{
    emoji::get_emoji,
    place::{add_place, get_place},
    post::create_post,
    user::{get_avater, register_avater},
};
use async_graphql::{Object, ID};
use sqlx::MySqlPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct Mutation {
    pub pool: Arc<MySqlPool>,
}

#[Object]
impl Mutation {
    pub async fn add_post(
        &self,
        avater_id: ID,
        emoji_id: ID,
        place_id: ID,
        title: String,
        comment: String,
    ) -> anyhow::Result<Post> {
        let post = create_post(
            self.pool.clone(),
            Uuid::parse_str(&avater_id.0)?,
            Uuid::parse_str(&emoji_id.0)?,
            Uuid::parse_str(&place_id.0)?,
            title,
            comment,
            None,
        )
        .await?;
        Ok(Post {
            id: post.id.into(),
            avater: get_avater(self.pool.clone(), post.avater_id)
                .await?
                .unwrap()
                .into(),
            emoji: get_emoji(self.pool.clone(), post.emoji_id)
                .await?
                .unwrap()
                .into(),
            place: get_place(self.pool.clone(), post.place_id)
                .await?
                .unwrap()
                .into(),
            title: post.title,
            comment: post.comment,
            visited_at: post.visited_at.to_rfc3339(),
            created_at: post.created_at.to_rfc3339(),
            updated_at: post.updated_at.to_rfc3339(),
        })
    }
    pub async fn add_avater(&self, name: String) -> anyhow::Result<Avater> {
        let avater = register_avater(self.pool.clone(), name).await?;
        Ok(avater.into())
    }
    pub async fn add_place(&self, name: String, address: String) -> anyhow::Result<Place> {
        let place = add_place(self.pool.clone(), name, address).await?;
        Ok(place.into())
    }
}
