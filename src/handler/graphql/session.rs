use oauth2::{basic::BasicClient, CsrfToken, PkceCodeChallenge, Scope};

pub async fn session_handler(client: &BasicClient) -> String {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (authrize_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read:user".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();
    authrize_url.to_string()
}
