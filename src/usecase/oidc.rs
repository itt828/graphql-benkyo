use anyhow::anyhow;
use derive_new::new;
use openidconnect::{
    core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata},
    reqwest::{async_http_client, http_client},
    AccessTokenHash, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce,
    OAuth2TokenResponse, PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse,
};
use std::env;

pub fn init_google_oidc_client() -> anyhow::Result<CoreClient> {
    let client_id = env::var("CLIENT_ID")?;
    let client_secret = env::var("CLIENT_SECRET")?;
    let provider_metadata = CoreProviderMetadata::discover(
        &IssuerUrl::new("https://accounts.google.com".to_string())?,
        http_client,
    )?;
    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
    );
    Ok(client)
}

#[derive(Debug, new)]
pub struct OidcLoginElms {
    pub url: String,
    pub pkce_veriifier: PkceCodeVerifier,
    pub nonce: Nonce,
}

pub async fn google_oidc_login(client: &CoreClient) -> anyhow::Result<OidcLoginElms> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, _csrf_token, nonce) = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new("read".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();
    Ok(OidcLoginElms::new(
        auth_url.to_string(),
        pkce_verifier,
        nonce,
    ))
}

pub async fn google_oidc_callback(
    client: &CoreClient,
    pkce_verifier: PkceCodeVerifier,
    nonce: &Nonce,
) -> anyhow::Result<()> {
    let token_response = client
        .exchange_code(AuthorizationCode::new("".to_string()))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await?;
    let id_token = token_response.id_token().ok_or_else(|| anyhow!(""))?;
    let claims = id_token.claims(&client.id_token_verifier(), nonce)?;
    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let actual_access_token_hash =
            AccessTokenHash::from_token(token_response.access_token(), &id_token.signing_alg()?)?;
        if actual_access_token_hash != *expected_access_token_hash {
            return Err(anyhow!("Invalid access token"));
        }
    }
    println!(
        "User {} with e-mail address {} has authenticated successfully",
        claims.subject().as_str(),
        claims
            .email()
            .map(|email| email.as_str())
            .unwrap_or("<not provided>"),
    );

    Ok(())
}
