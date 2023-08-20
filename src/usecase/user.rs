use crate::infrastructure::module::RepositoriesModuleExt;
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
    pub async fn register_user(&self, _id: &str) {}
}
