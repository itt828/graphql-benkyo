use super::model::Post;
use crate::interface::modules::{Modules, ModulesExt};
use async_graphql::{Object, ID};
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

pub struct Query {
    pub modules: Arc<Modules>,
}

#[Object]
impl Query {
    pub async fn post(&self, id: ID) -> anyhow::Result<Option<Post>> {
        let post = self
            .modules
            .post_use_case()
            .get_post(Uuid::from_str(&id.0).unwrap())
            .await?;
        // let avater = self.modules.user_use_case()
        // let emoji =
        // let place =
        Ok(None)
    }
    pub async fn posts(&self) -> anyhow::Result<Vec<Post>> {
        self.modules.post_use_case.all_posts().await?;
        Ok(vec![])
    }
}
