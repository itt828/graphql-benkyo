use super::model::Post;
use crate::{
    models::{Avater as DomainAvater, Emoji, Place},
    usecase::{
        emoji::{get_emoji, get_emojis},
        place::{get_place, get_places},
        post::{all_posts, get_post},
        user::{get_avater, get_avaters},
    },
};
use async_graphql::{Object, ID};
use sqlx::MySqlPool;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use uuid::Uuid;

pub struct Query {
    pub pool: Arc<MySqlPool>,
}

#[Object]
impl Query {
    pub async fn post(&self, id: ID) -> anyhow::Result<Option<Post>> {
        let post = get_post(self.pool.clone(), Uuid::parse_str(&id.0)?).await?;
        if let Some(post) = post {
            let avater = get_avater(self.pool.clone(), post.avater_id).await?;

            let emoji = get_emoji(self.pool.clone(), post.emoji_id).await?;
            let place = get_place(self.pool.clone(), post.place_id).await?;
            Ok(Some(Post {
                id: ID(post.id.to_string()),
                avater: avater.unwrap().into(),
                emoji: emoji.unwrap().into(),
                place: place.unwrap().into(),
                title: post.title,
                comment: post.comment,
                visited_at: post.visited_at.to_rfc3339(),
                created_at: post.created_at.to_rfc3339(),
                updated_at: post.updated_at.to_rfc3339(),
            }))
        } else {
            Ok(None)
        }
    }
    pub async fn posts(&self) -> anyhow::Result<Vec<Post>> {
        let posts = all_posts(self.pool.clone()).await?;
        let avater_ids = posts
            .iter()
            .map(|item| item.avater_id)
            .collect::<HashSet<Uuid>>()
            .into_iter()
            .collect::<Vec<_>>();
        let avaters = get_avaters(self.pool.clone(), Some(avater_ids)).await?;
        let avaters = avaters
            .iter()
            .map(|item| (item.id, item))
            .collect::<HashMap<Uuid, &DomainAvater>>();
        let emoji_ids = posts
            .iter()
            .map(|item| item.emoji_id)
            .collect::<HashSet<Uuid>>()
            .into_iter()
            .collect::<Vec<_>>();
        let emojis = get_emojis(self.pool.clone(), Some(emoji_ids)).await?;
        let emojis = emojis
            .iter()
            .map(|item| (item.id, item))
            .collect::<HashMap<Uuid, &Emoji>>();
        let place_ids = posts
            .iter()
            .map(|item| item.place_id)
            .collect::<HashSet<Uuid>>()
            .into_iter()
            .collect::<Vec<_>>();
        let places = get_places(self.pool.clone(), Some(place_ids)).await?;
        let places = places
            .iter()
            .map(|item| (item.id, item))
            .collect::<HashMap<Uuid, &Place>>();
        Ok(posts
            .iter()
            .map(|item| Post {
                id: ID(item.id.to_string()),
                avater: avaters[&item.avater_id].clone().into(),
                emoji: emojis[&item.emoji_id].clone().into(),
                place: places[&item.place_id].clone().into(),
                title: item.title.to_string(),
                comment: item.comment.to_string(),
                visited_at: item.visited_at.to_rfc3339(),
                created_at: item.created_at.to_rfc3339(),
                updated_at: item.updated_at.to_rfc3339(),
            })
            .collect())
    }
}
