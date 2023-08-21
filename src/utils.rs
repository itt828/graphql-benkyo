use async_graphql::Schema;
use rand::{distributions::Alphanumeric, Rng};
use serde_json::Value;
use std::{fs::File, io::Write};
use uuid::Uuid;

use crate::{domain::model::post::Emoji, interface::modules::Modules};

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

pub async fn init_emoji(modules: &Modules) -> () {
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
        .into_iter()
        .map(|emoji| Emoji {
            id: Uuid::new_v4(),
            name: emoji["short_name"].as_str().unwrap().to_string(),
        })
        .collect::<Vec<_>>();
    modules
        .emoji_use_case
        .register_emojis(&emojis)
        .await
        .unwrap();
}
