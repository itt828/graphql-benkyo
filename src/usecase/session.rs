use jsonwebtoken::{encode, EncodingKey, Header};

use crate::domain::model::session::AuthPayload;

pub async fn gen_cookie_jwt(payload: AuthPayload) -> anyhow::Result<String> {
    let keys = EncodingKey::from_secret(&[b'a']);
    let token = encode(&Header::default(), &payload, &keys)?;
    Ok(token)
}
