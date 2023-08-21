use crate::domain::{model::user::Avater, repository::user::UserRepository};
use sqlx::MySqlPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserRepositoryImpl {
    pub pool: Arc<MySqlPool>,
}

impl UserRepositoryImpl {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get_avater(&self, avater_id: Uuid) -> anyhow::Result<Option<Avater>> {
        Ok(None)
    }
}
