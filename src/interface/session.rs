use self::{
    cookie::gen_cookie_jwt,
    oidc::{google_oidc_callback, google_oidc_login},
};

use super::modules::Modules;
use axum::{
    extract::{Query, State},
    response::Redirect,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use openidconnect::{core::CoreClient, Nonce, PkceCodeVerifier};
use reqwest::StatusCode;
use serde::Deserialize;
use std::sync::Arc;

pub mod cookie;
pub mod oidc;

pub async fn login_handler(
    State((_modules, oidc_client)): State<(Arc<Modules>, Arc<CoreClient>)>,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), StatusCode> {
    let oidc_elms = match google_oidc_login(&oidc_client).await {
        Ok(v) => v,
        Err(_e) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    let mut jar = jar;
    jar = jar.add(Cookie::new("state", oidc_elms.csrf_token));
    jar = jar.add(Cookie::new(
        "code_verifier",
        oidc_elms.pkce_veriifier.secret().to_string(),
    ));
    jar = jar.add(Cookie::new("nonce", oidc_elms.nonce.secret().to_string()));
    Ok((jar, Redirect::to(&oidc_elms.url)))
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CallbackParams {
    code: String,
    state: String,
}

pub async fn callback_handler(
    Query(query): Query<CallbackParams>,
    jar: CookieJar,
    State((_modules, oidc_client)): State<(Arc<Modules>, Arc<CoreClient>)>,
) -> Result<Redirect, StatusCode> {
    let state = jar
        .get("state")
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .value()
        .to_owned();
    let code_verifier = jar
        .get("code_verifier")
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .value()
        .to_owned();
    let nonce = jar
        .get("nonce")
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .value()
        .to_owned();
    let authz_code = query.code;
    if state != query.state {
        println!("Err");
    }

    google_oidc_callback(
        &oidc_client,
        authz_code,
        PkceCodeVerifier::new(code_verifier),
        &Nonce::new(nonce),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    println!("oisu5");
    Ok(Redirect::to("http://localhost:3030/"))
}
