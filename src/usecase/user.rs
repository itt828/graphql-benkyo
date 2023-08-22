use uuid::Uuid;

use crate::{
    domain::{model::user::Avater, repository::user::UserRepository},
    infrastructure::module::RepositoriesModuleExt,
};
use std::sync::Arc;

pub struct UserUseCase<R: RepositoriesModuleExt> {
    pub repositories: Arc<R>,
}

impl<R> UserUseCase<R>
where
    R: RepositoriesModuleExt,
{
    pub async fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }
    pub async fn get_avater(&self, avater_id: Uuid) -> anyhow::Result<Option<Avater>> {
        self.repositories
            .user_repository()
            .get_avater(avater_id)
            .await
    }
    pub async fn register_avater(&self, name: String) -> anyhow::Result<Avater> {
        let avater = Avater {
            id: Uuid::new_v4(),
            name,
        };
        self.repositories
            .user_repository()
            .register_avater(&avater)
            .await?;
        Ok(avater)
    }
}
