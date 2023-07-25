use super::modules::{Modules, ModulesExt};
use crate::{
    domain::model::session::AuthPayload, usecase::session::gen_cookie_jwt,
    utils::gen_rand_alphanumeric,
};
use axum::{
    extract::{Query, State},
    headers::Header,
    http::HeaderMap,
    response::{IntoResponse, Redirect},
};
use oauth2::{basic::BasicClient, reqwest::async_http_client, AuthorizationCode, TokenResponse};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Deserialize)]
pub struct CallbackParams {
    code: String,
    state: String,
}

pub async fn callback_handler(
    Query(query): Query<CallbackParams>,
    cookies: Cookies,
    State((modules, oauth_client)): State<(Arc<Modules>, BasicClient)>,
) -> Result<Redirect, StatusCode> {
    let token = oauth_client.exchange_code(AuthorizationCode::new(query.code.clone()));
    // .set_pkce_verifier(pkce_verifier)
    let token = token.request_async(async_http_client).await.unwrap();
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
    let user = me["id"].as_str().unwrap();
    modules.user_use_case().register_user(user).await;
    let auth_payload = AuthPayload {
        id: user.to_string(),
    };
    let jwt = gen_cookie_jwt(auth_payload).await.unwrap();
    cookies.add(Cookie::new("blog_session_jwt", jwt));
    Ok(Redirect::to("http://localhost:3030/"))
}
