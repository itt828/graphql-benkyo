use super::model::Blog;
use crate::interface::modules::Modules;
use async_graphql::Object;
use oauth2::{basic::BasicClient, CsrfToken, PkceCodeChallenge, Scope};
use std::sync::Arc;

pub struct Mutation {
    pub modules: Arc<Modules>,
    pub oauth_client: BasicClient,
}

#[Object]
impl Mutation {
    pub async fn add_blog(&self, title: String, content: String) -> anyhow::Result<Blog> {
        let blog = self
            .modules
            .blog_use_case
            .create_blog(&title, &content)
            .await?;
        Ok(blog.into())
    }
    pub async fn login(&self) -> String {
        let client = &self.oauth_client;
        // let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let (authrize_url, _csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("read:user".to_string()))
            // .set_pkce_challenge(pkce_challenge)
            .url();
        authrize_url.to_string()
    }
}
