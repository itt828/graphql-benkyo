use crate::interface::modules::Modules;
use async_graphql::Object;
use openidconnect::core::CoreClient;
use std::sync::Arc;
use uuid::Uuid;

use super::model::Post;

pub struct Mutation {
    pub modules: Arc<Modules>,
    pub oidc_client: Arc<CoreClient>,
}

#[Object]
impl Mutation {
    pub async fn add_post(&self, title: String, comment: String) -> anyhow::Result<Post> {
        let post = self
            .modules
            .post_use_case
            .create_post(
                Uuid::new_v4(),
                Uuid::new_v4(),
                Uuid::new_v4(),
                title,
                comment,
                None,
            )
            .await?;
        unimplemented!()
    }
}
