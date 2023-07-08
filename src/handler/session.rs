use std::{fmt::format, sync::Arc};

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use oauth2::{reqwest::async_http_client, AuthorizationCode, TokenResponse};
use serde::Deserialize;
use serde_json::Value;

use crate::container::Container;

#[derive(Deserialize)]
pub struct CallbackParams {
    code: String,
}

pub async fn callback_handler(
    Query(query): Query<CallbackParams>,
    State(container): State<Arc<Container>>,
) -> impl IntoResponse {
    let token = container
        .oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
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
        .header("User-Agent", "itt-blog-server");
    println!("{:?}", me);
    let me = me.send().await.unwrap().json::<Value>().await.unwrap();
    println!("{:?}", me);
    Redirect::to("http://localhost:3030/")
}
