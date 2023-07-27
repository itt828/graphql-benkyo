use std::sync::Arc;

use crate::{
    infrastructure::{
        module::{RepositoriesModule, RepositoriesModuleExt},
        repository::connect_db,
    },
    usecase::{blog::BlogUseCase, user::UserUseCase},
};

pub struct Modules {
    pub blog_use_case: BlogUseCase<RepositoriesModule>,
    pub user_use_case: UserUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RemositoriesModule: RepositoriesModuleExt;
    fn blog_use_case(&self) -> &BlogUseCase<RepositoriesModule>;
    fn user_use_case(&self) -> &UserUseCase<RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RemositoriesModule = RepositoriesModule;
    fn blog_use_case(&self) -> &BlogUseCase<RepositoriesModule> {
        &self.blog_use_case
    }
    fn user_use_case(&self) -> &UserUseCase<RepositoriesModule> {
        &self.user_use_case
    }
}

impl Modules {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = Arc::new(connect_db().await?);
        let repositories_module = Arc::new(RepositoriesModule::new(pool));

        let blog_use_case = BlogUseCase::new(repositories_module.clone()).await;
        let user_use_case = UserUseCase::new(repositories_module).await;

        Ok(Self {
            blog_use_case,
            user_use_case,
        })
    }
}
