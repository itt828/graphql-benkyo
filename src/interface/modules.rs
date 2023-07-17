use oauth2::basic::BasicClient;
use std::sync::Arc;

use crate::{
    infrastructure::{
        module::{RepositoriesModule, RepositoriesModuleExt},
        repository::connect_db,
    },
    usecase::blog::BlogUseCase,
    utils::oauth_client,
};

pub struct Modules {
    pub blog_use_case: BlogUseCase<RepositoriesModule>,
    pub oauth_client: BasicClient,
}

pub trait ModulesExt {
    type RemositoriesModule: RepositoriesModuleExt;
    fn blog_use_case(&self) -> &BlogUseCase<RepositoriesModule>;
    fn oauth_client(&self) -> &BasicClient;
}

impl ModulesExt for Modules {
    type RemositoriesModule = RepositoriesModule;
    fn blog_use_case(&self) -> &BlogUseCase<RepositoriesModule> {
        &self.blog_use_case
    }
    fn oauth_client(&self) -> &BasicClient {
        &self.oauth_client
    }
}

impl Modules {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = Arc::new(connect_db().await?);
        let repositories_module = Arc::new(RepositoriesModule::new(pool));

        let blog_use_case = BlogUseCase::new(repositories_module).await;
        let oauth_client = oauth_client().await?;

        Ok(Self {
            blog_use_case,
            oauth_client,
        })
    }
}
