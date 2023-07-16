use async_graphql::Schema;
use rand::{distributions::Alphanumeric, Rng};
use std::{fs::File, io::Write};

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
