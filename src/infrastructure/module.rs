use sqlx::MySqlPool;
use std::sync::Arc;

use crate::domain::repository::post::PostRepository;

use super::repository::post::PostRepositoryImpl;

pub struct RepositoriesModule {
    pub post_repository: PostRepositoryImpl,
    // pub user_repository: UserRepositoryImpl
}
pub trait RepositoriesModuleExt {
    type PostRepo: PostRepository;
    fn post_repository(&self) -> &Self::PostRepo;
}
impl RepositoriesModuleExt for RepositoriesModule {
    type PostRepo = PostRepositoryImpl;
    fn post_repository(&self) -> &Self::PostRepo {
        &self.post_repository
    }
}
impl RepositoriesModule {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        let post_repository = PostRepositoryImpl::new(pool);
        Self { post_repository }
    }
}
