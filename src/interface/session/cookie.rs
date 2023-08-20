use jsonwebtoken::{encode, EncodingKey, Header};

pub async fn gen_cookie_jwt(id_token: String) -> anyhow::Result<String> {
    let keys = EncodingKey::from_secret(&[b'a']);
    let token = encode(&Header::default(), &id_token, &keys)?;
    Ok(token)
}
