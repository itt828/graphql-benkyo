use super::model::{Avater, Post};
use crate::interface::modules::{Modules, ModulesExt};
use async_graphql::{Object, ID};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    sync::Arc,
};
use uuid::Uuid;

pub struct Query {
    pub modules: Arc<Modules>,
}

#[Object]
impl Query {
    pub async fn post(&self, id: ID) -> anyhow::Result<Option<Post>> {
        let post = self
            .modules
            .post_use_case()
            .get_post(Uuid::parse_str(&id.0)?)
            .await?;
        if let Some(post) = post {
            let avater = self
                .modules
                .user_use_case()
                .get_avater(post.avater_id)
                .await?;

            let emoji = self
                .modules
                .emoji_use_case()
                .get_emoji(post.emoji_id)
                .await?;
            let place = self
                .modules
                .place_use_case()
                .get_place(post.place_id)
                .await?;
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
        let posts = self.modules.post_use_case.all_posts().await?;
        let avater_ids = posts
            .iter()
            .map(|item| item.avater_id)
            .collect::<HashSet<Uuid>>()
            .into_iter()
            .collect::<Vec<_>>();
        let avaters = self
            .modules
            .user_use_case()
            .get_avaters(Some(avater_ids))
            .await?;
        let avaters = avaters
            .iter()
            .map(|item| (item.id, item))
            .collect::<HashMap<Uuid, &crate::domain::model::user::Avater>>();
        let emoji_ids = posts
            .iter()
            .map(|item| item.emoji_id)
            .collect::<HashSet<Uuid>>()
            .into_iter()
            .collect::<Vec<_>>();
        let emojis = 
        // let emojis: HashMap<Uuid, &crate::domain::model::post::Emoji> = self
        self.
            modules
            .emoji_use_case()
            .get_emojis(Some(emoji_ids))
            .await?;
            let emojis = emojis.iter()
            .map(|item| (item.id, item))
            .collect::<HashMap<Uuid, &crate::domain::model::post::Emoji>>();
        let place_ids = posts
            .iter()
            .map(|item| item.place_id)
            .collect::<HashSet<Uuid>>()
            .into_iter()
            .collect::<Vec<_>>();
        let places = self
            .modules
            .place_use_case()
            .get_places(Some(place_ids))
            .await?;
            let places = places.iter()
            .map(|item| (item.id, item))
            .collect::<HashMap<Uuid, &crate::domain::model::post::Place>>();
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
