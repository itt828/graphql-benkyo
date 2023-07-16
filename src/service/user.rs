use async_trait::async_trait;

use crate::domain::user::User;

#[async_trait]
pub trait UserService {
    // async fn get_me(&self) -> anyhow::Result<User>;
    // async fn get_user(&self, id: Uuid) -> anyhow::Result<User>;
    async fn add_user(&self, name: &str) -> anyhow::Result<User>;
}
pub struct UserServiceImpl;

#[async_trait]
impl UserService for UserServiceImpl {
    // async fn get_me(&self) -> anyhow::Result<User> {}
    // // async fn get_user(&self, id: Uuid) -> anyhow::Result<User> {}
    async fn add_user(&self, _name: &str) -> anyhow::Result<User> {
        // if exist -> error
        // not -> return new user
        anyhow::bail!("unimplemented")
    }
}
