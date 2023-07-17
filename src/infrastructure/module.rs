use super::repository::blog::BlogRepositoryImpl;
use crate::domain::repository::blog::BlogRepository;
use sqlx::MySqlPool;
use std::sync::Arc;

pub struct RepositoriesModule {
    pub blog_repository: BlogRepositoryImpl,
    // pub user_repository: UserRepositoryImpl
}
pub trait RepositoriesModuleExt {
    type BlogRepo: BlogRepository;
    fn blog_repository(&self) -> &Self::BlogRepo;
}
impl RepositoriesModuleExt for RepositoriesModule {
    type BlogRepo = BlogRepositoryImpl;
    fn blog_repository(&self) -> &Self::BlogRepo {
        &self.blog_repository
    }
}
impl RepositoriesModule {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        let blog_repository = BlogRepositoryImpl::new(pool.clone());
        Self { blog_repository }
    }
}
