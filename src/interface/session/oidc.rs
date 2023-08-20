use anyhow::anyhow;
use derive_new::new;
use openidconnect::{
    core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    AccessTokenHash, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce,
    OAuth2TokenResponse, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse,
};
use std::{borrow::Cow, env, fmt::format};

pub async fn init_google_oidc_client() -> anyhow::Result<CoreClient> {
    let client_id = env::var("CLIENT_ID")?;
    let client_secret = env::var("CLIENT_SECRET")?;
    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new("https://accounts.google.com".to_string())?,
        async_http_client,
    )
    .await?;
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
    pub csrf_token: String,
    pub pkce_veriifier: PkceCodeVerifier,
    pub nonce: Nonce,
}

pub async fn google_oidc_login(client: &CoreClient) -> anyhow::Result<OidcLoginElms> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let redirect_url = Cow::Owned(
        RedirectUrl::new("http://localhost:3000/auth/callback".to_string())
            .map_err(|e| format!("{}", e))
            .unwrap()
            .clone(),
    );

    let (auth_url, csrf_token, nonce) = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .set_redirect_uri(redirect_url)
        .url();
    Ok(OidcLoginElms::new(
        auth_url.to_string(),
        csrf_token.secret().to_string(),
        pkce_verifier,
        nonce,
    ))
}

pub async fn google_oidc_callback(
    client: &CoreClient,
    authz_code: String,
    pkce_verifier: PkceCodeVerifier,
    nonce: &Nonce,
) -> anyhow::Result<()> {
    let redirect_url = Cow::Owned(
        RedirectUrl::new("http://localhost:3000/auth/callback".to_string())
            .map_err(|e| format!("{}", e))
            .unwrap()
            .clone(),
    );
    let token_response = client
        .exchange_code(AuthorizationCode::new(authz_code))
        .set_pkce_verifier(pkce_verifier)
        .set_redirect_uri(redirect_url)
        .request_async(async_http_client)
        .await
        .expect("bug");
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
