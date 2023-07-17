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
use std::env;

use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

pub async fn oauth_client() -> anyhow::Result<BasicClient> {
    let client_id = env::var("CLIENT_ID")?;
    let client_secret = env::var("CLIENT_SECRET")?;
    let auth_url = env::var("AUTH_URL")?;
    let token_url = env::var("TOKEN_URL")?;
    let redirect_url = env::var("REDIRECT_URL")?;
    Ok(BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url)?,
        Some(TokenUrl::new(token_url)?),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url)?))
}