use uuid::Uuid;

use crate::domain::model::user::Avater;

#[async_trait::async_trait]
pub trait UserRepository {
    async fn get_avater(&self, avater_id: Uuid) -> anyhow::Result<Option<Avater>>;
    async fn get_avaters(&self, avater_ids: Option<Vec<Uuid>>) -> anyhow::Result<Vec<Avater>>;
    async fn register_avater(&self, avater: &Avater) -> anyhow::Result<()>;
}
