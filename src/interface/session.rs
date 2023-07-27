use super::modules::Modules;
use crate::{
    domain::model::session::AuthPayload,
    usecase::{
        oidc::{google_oidc_callback, google_oidc_login},
        session::gen_cookie_jwt,
    },
};
use axum::{
    extract::{Query, State},
    response::Redirect,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use openidconnect::{core::CoreClient, Nonce, PkceCodeVerifier};
use reqwest::StatusCode;
use serde::Deserialize;

use std::sync::Arc;
pub async fn login_handler(
    State((_modules, oidc_client)): State<(Arc<Modules>, Arc<CoreClient>)>,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), StatusCode> {
    let a = match google_oidc_login(&oidc_client).await {
        Ok(v) => v,
        Err(_e) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    let jar = jar.add(Cookie::new(
        "code_verifier",
        a.pkce_veriifier.secret().to_string(),
    ));
    let jar = jar.add(Cookie::new("nonce", a.nonce.secret().to_string()));
    Ok((jar, Redirect::to(&a.url)))
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CallbackParams {
    code: String,
    state: String,
}

pub async fn callback_handler(
    Query(_query): Query<CallbackParams>,
    jar: CookieJar,
    State((_modules, oidc_client)): State<(Arc<Modules>, Arc<CoreClient>)>,
) -> Result<(CookieJar, Redirect), StatusCode> {
    let code_verifier = jar
        .get("code_verifier")
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();
    let nonce = jar
        .get("nonce")
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();
    google_oidc_callback(
        &oidc_client,
        PkceCodeVerifier::new(code_verifier),
        &Nonce::new(nonce),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let auth_payload = AuthPayload {
        id: "sample".to_string(),
    };
    let jwt = gen_cookie_jwt(auth_payload).await.unwrap();
    Ok((
        jar.add(Cookie::new("blog_session_jwt", jwt)),
        Redirect::to("http://localhost:3030/"),
    ))
}
