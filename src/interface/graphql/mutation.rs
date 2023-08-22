use crate::interface::modules::{Modules, ModulesExt};
use async_graphql::{Object, ID};
use openidconnect::core::CoreClient;
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

use super::model::{Avater, Place, Post};

pub struct Mutation {
    pub modules: Arc<Modules>,
    pub oidc_client: Arc<CoreClient>,
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
        let post = self
            .modules
            .post_use_case
            .create_post(
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
            avater: self
                .modules
                .user_use_case
                .get_avater(post.avater_id)
                .await?
                .unwrap()
                .into(),
            emoji: self
                .modules
                .emoji_use_case
                .get_emoji(post.emoji_id)
                .await?
                .unwrap()
                .into(),
            place: self
                .modules
                .place_use_case
                .get_place(post.place_id)
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
        let avater = self.modules.user_use_case().register_avater(name).await?;
        Ok(avater.into())
    }
    pub async fn add_place(&self, name: String, address: String) -> anyhow::Result<Place> {
        let place = self.modules.place_use_case.add_place(name, address).await?;
        Ok(place.into())
    }
}
