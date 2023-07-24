use super::modules::Modules;
use crate::utils::gen_rand_alphanumeric;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use oauth2::{reqwest::async_http_client, AuthorizationCode, TokenResponse};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CallbackParams {
    code: String,
}

pub async fn callback_handler(
    Query(query): Query<CallbackParams>,
    jar: CookieJar,
    State(modules): State<Arc<Modules>>,
) -> Result<(CookieJar, Redirect), StatusCode> {
    let token = modules
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
    // let user = modules.add_user(me["id"].as_str().unwrap()).await.unwrap();
    // println!("{:?}", user);
    Ok((
        jar.add(Cookie::new("blog_session", gen_rand_alphanumeric(36))),
        Redirect::to("http://localhost:3030/"),
    ))
}
