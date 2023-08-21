use sqlx::MySqlPool;
use std::sync::Arc;

use crate::domain::repository::{
    emoji::EmojiRepository, place::PlaceRepository, post::PostRepository, user::UserRepository,
};

use super::repository::{
    emoji::EmojiRepositoryImpl, place::PlaceRepositoryImpl, post::PostRepositoryImpl,
    user::UserRepositoryImpl,
};

pub struct RepositoriesModule {
    pub post_repository: PostRepositoryImpl,
    pub emoji_repository: EmojiRepositoryImpl,
    pub place_repository: PlaceRepositoryImpl,
    pub user_repository: UserRepositoryImpl,
}
pub trait RepositoriesModuleExt {
    type PostRepo: PostRepository;
    type EmojiRepo: EmojiRepository;
    type PlaceRepo: PlaceRepository;
    type UserRepo: UserRepository;

    fn post_repository(&self) -> &Self::PostRepo;
    fn emoji_repository(&self) -> &Self::EmojiRepo;
    fn place_repository(&self) -> &Self::PlaceRepo;
    fn user_repository(&self) -> &Self::UserRepo;
}
impl RepositoriesModuleExt for RepositoriesModule {
    type PostRepo = PostRepositoryImpl;
    type EmojiRepo = EmojiRepositoryImpl;
    type PlaceRepo = PlaceRepositoryImpl;
    type UserRepo = UserRepositoryImpl;
    fn post_repository(&self) -> &Self::PostRepo {
        &self.post_repository
    }
    fn emoji_repository(&self) -> &Self::EmojiRepo {
        &self.emoji_repository
    }
    fn place_repository(&self) -> &Self::PlaceRepo {
        &self.place_repository
    }
    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }
}
impl RepositoriesModule {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        let post_repository = PostRepositoryImpl::new(pool.clone());
        let emoji_repository = EmojiRepositoryImpl::new(pool.clone());
        let place_repository = PlaceRepositoryImpl::new(pool.clone());
        let user_repository = UserRepositoryImpl::new(pool.clone());
        Self {
            post_repository,
            emoji_repository,
            place_repository,
            user_repository,
        }
    }
}
