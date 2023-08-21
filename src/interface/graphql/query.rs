use super::model::{Avater, Post};
use crate::interface::modules::{Modules, ModulesExt};
use async_graphql::{Object, ID};
use std::{str::FromStr, sync::Arc};
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
            .get_post(Uuid::from_str(&id.0).unwrap())
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
        self.modules.post_use_case.all_posts().await?;
        Ok(vec![])
    }
}
