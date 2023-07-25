use crate::{
    domain::{model::blog::Blog, repository::blog::BlogRepository},
    infrastructure::module::RepositoriesModuleExt,
};
use std::sync::Arc;
use uuid::Uuid;

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
    pub async fn register_user(&self, id: &str) {}
}
