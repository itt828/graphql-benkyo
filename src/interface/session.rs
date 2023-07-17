use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use oauth2::{reqwest::async_http_client, AuthorizationCode, TokenResponse};
use serde::Deserialize;
use serde_json::Value;
use tower_cookies::{Cookie, Cookies};

use crate::{modules::Container, service::user::UserService, utils::gen_rand_alphanumeric};

#[derive(Deserialize)]
pub struct CallbackParams {
    code: String,
}

pub async fn callback_handler(
    Query(query): Query<CallbackParams>,
    cookies: Cookies,
    State(container): State<Arc<Container>>,
) -> impl IntoResponse {
    let token = container
        .oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        // .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
        .unwrap();
    let client = reqwest::Client::new();
    let me = client
        .get("https://api.github.com/user")
        .header(
            "Authorization",
            format!("token {}", token.access_token().secret()),
        )
        .header("User-Agent", "itt-blog-server")
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();
    let user = container
        .user_service
        .add_user(me["id"].as_str().unwrap())
        .await
        .unwrap();
    println!("{:?}", user);
    cookies.add(Cookie::new("blog_session", gen_rand_alphanumeric(36)));
    Redirect::to("http://localhost:3030/")
}