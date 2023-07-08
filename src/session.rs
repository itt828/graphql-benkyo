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
