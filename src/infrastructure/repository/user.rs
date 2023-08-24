use crate::domain::{model::user::Avater, repository::user::UserRepository};
use sqlx::MySqlPool;
use std::{str::FromStr, sync::Arc};
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
        let pool = self.pool.clone();
        let avater: Option<Avater> = sqlx::query_as(r"select * from avater where id=?")
            .bind(avater_id.to_string())
            .fetch_optional(&*pool)
            .await?;
        Ok(avater.map(|v| v.into()))
    }
    async fn get_avaters(&self, avater_ids: Option<Vec<Uuid>>) -> anyhow::Result<Vec<Avater>> {
        let pool = self.pool.clone();
        let avaters = match avater_ids {
            Some(avater_ids) => {
                let query_string = format!(
                    "select id, name from avater where id in (?{})",
                    ", ?".repeat(avater_ids.len() - 1)
                );
                let mut query = sqlx::query_as(&query_string);

                for avater_id in avater_ids.iter() {
                    query = query.bind(avater_id.to_string());
                }
                query.fetch_all(&*pool).await?
            }
            None => {
                let query = sqlx::query_as("select id, name from avater");
                query.fetch_all(&*pool).await?
            }
        };
        Ok(avaters)
    }
    async fn register_avater(&self, avater: &Avater) -> anyhow::Result<()> {
        let pool = self.pool.clone();
        sqlx::query(r"insert into avater (id, name, account_id) values (?, ?, null)")
            .bind(avater.id.to_string())
            .bind(avater.name.clone())
            .execute(&*pool)
            .await?;
        Ok(())
    }
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct AvaterRecord {
    id: String,
    name: String,
}

impl Into<AvaterRecord> for Avater {
    fn into(self) -> AvaterRecord {
        AvaterRecord {
            id: self.id.to_string(),
            name: self.name,
        }
    }
}

impl Into<Avater> for AvaterRecord {
    fn into(self) -> Avater {
        Avater {
            id: Uuid::from_str(&self.id).unwrap(),
            name: self.name,
        }
    }
}
