use sqlx::MySqlPool;
use uuid::Uuid;

use std::sync::Arc;

use crate::models::Avater;

pub async fn get_avater(pool: Arc<MySqlPool>, avater_id: Uuid) -> anyhow::Result<Option<Avater>> {
    let avater: Option<Avater> = sqlx::query_as(r"select * from avater where id=?")
        .bind(avater_id.to_string())
        .fetch_optional(&*pool)
        .await?;
    Ok(avater)
}
pub async fn get_avaters(
    pool: Arc<MySqlPool>,
    avater_ids: Option<Vec<Uuid>>,
) -> anyhow::Result<Vec<Avater>> {
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
pub async fn register_avater(pool: Arc<MySqlPool>, name: String) -> anyhow::Result<Avater> {
    let avater = Avater {
        id: Uuid::new_v4(),
        name,
    };
    sqlx::query(r"insert into avater (id, name, account_id) values (?, ?, null)")
        .bind(avater.id.to_string())
        .bind(avater.name.clone())
        .execute(&*pool)
        .await?;
    Ok(avater)
}
