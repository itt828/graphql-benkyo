use std::sync::Arc;

use crate::{
    infrastructure::{
        module::{RepositoriesModule, RepositoriesModuleExt},
        repository::connect_db,
    },
    usecase::{emoji::EmojiUseCase, place::PlaceUseCase, post::PostUseCase, user::UserUseCase},
};

pub struct Modules {
    pub post_use_case: PostUseCase<RepositoriesModule>,
    pub user_use_case: UserUseCase<RepositoriesModule>,
    pub emoji_use_case: EmojiUseCase<RepositoriesModule>,
    pub place_use_case: PlaceUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RemositoriesModule: RepositoriesModuleExt;
    fn post_use_case(&self) -> &PostUseCase<RepositoriesModule>;
    fn user_use_case(&self) -> &UserUseCase<RepositoriesModule>;
    fn emoji_use_case(&self) -> &EmojiUseCase<RepositoriesModule>;
    fn place_use_case(&self) -> &PlaceUseCase<RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RemositoriesModule = RepositoriesModule;
    fn post_use_case(&self) -> &PostUseCase<RepositoriesModule> {
        &self.post_use_case
    }
    fn user_use_case(&self) -> &UserUseCase<RepositoriesModule> {
        &self.user_use_case
    }
    fn emoji_use_case(&self) -> &EmojiUseCase<RepositoriesModule> {
        &self.emoji_use_case
    }
    fn place_use_case(&self) -> &PlaceUseCase<RepositoriesModule> {
        &self.place_use_case
    }
}

impl Modules {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = Arc::new(connect_db().await?);
        let repositories_module = Arc::new(RepositoriesModule::new(pool));

        let post_use_case = PostUseCase::new(repositories_module.clone()).await;
        let user_use_case = UserUseCase::new(repositories_module.clone()).await;
        let place_use_case = PlaceUseCase::new(repositories_module.clone()).await;
        let emoji_use_case = EmojiUseCase::new(repositories_module).await;

        Ok(Self {
            post_use_case,
            user_use_case,
            place_use_case,
            emoji_use_case,
        })
    }
}
