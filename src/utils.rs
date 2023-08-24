use async_graphql::Schema;
use rand::{distributions::Alphanumeric, Rng};
use serde_json::Value;
use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
    MySqlPool,
};
use std::{fs::File, io::Write, sync::Arc};
use uuid::Uuid;

use crate::{models::Emoji, usecase::emoji::register_emojis};

pub fn gen_rand_alphanumeric(n: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}

pub fn gen_graphql_schema_file<Query, Mutation, Subscription>(
    schema: &Schema<Query, Mutation, Subscription>,
) where
    Mutation: async_graphql::ObjectType + 'static,
    Query: async_graphql::ObjectType + 'static,
    Subscription: async_graphql::SubscriptionType + 'static,
{
    let mut file = File::create("schema.graphql").unwrap();
    let schema_text = format!(
        r"# Auto generated. DO NOT EDIT.
{}",
        schema.sdl()
    );
    let _ = file.write_all(schema_text.as_bytes());
}

pub async fn init_emojis(db: Arc<MySqlPool>) {
    let emojis =
        reqwest::get("https://raw.githubusercontent.com/iamcal/emoji-data/master/emoji.json")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    let emojis: Value = serde_json::from_str(&emojis).unwrap();
    let emojis = emojis
        .as_array()
        .unwrap()
        .iter()
        .map(|emoji| Emoji {
            id: Uuid::new_v4(),
            name: emoji["short_name"].as_str().unwrap().to_string(),
        })
        .collect::<Vec<_>>();
    register_emojis(db, &emojis).await.unwrap();
}

pub async fn connect_db() -> anyhow::Result<MySqlPool> {
    let mysql_config = MySqlConnectOptions::new()
        .host("mysql")
        .port(3306)
        .username("root")
        .password("password")
        .database("blog-db");
    let pool = MySqlPoolOptions::new().connect_with(mysql_config).await?;
    Ok(pool)
}
