use std::sync::Arc;

use crate::{
    infrastructure::{
        module::{RepositoriesModule, RepositoriesModuleExt},
        repository::connect_db,
    },
    usecase::{post::PostUseCase, user::UserUseCase},
};

pub struct Modules {
    pub post_use_case: PostUseCase<RepositoriesModule>,
    pub user_use_case: UserUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RemositoriesModule: RepositoriesModuleExt;
    fn post_use_case(&self) -> &PostUseCase<RepositoriesModule>;
    fn user_use_case(&self) -> &UserUseCase<RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RemositoriesModule = RepositoriesModule;
    fn post_use_case(&self) -> &PostUseCase<RepositoriesModule> {
        &self.post_use_case
    }
    fn user_use_case(&self) -> &UserUseCase<RepositoriesModule> {
        &self.user_use_case
    }
}

impl Modules {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = Arc::new(connect_db().await?);
        let repositories_module = Arc::new(RepositoriesModule::new(pool));

        let post_use_case = PostUseCase::new(repositories_module.clone()).await;
        let user_use_case = UserUseCase::new(repositories_module).await;

        Ok(Self {
            post_use_case,
            user_use_case,
        })
    }
}
