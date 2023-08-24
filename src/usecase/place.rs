use std::sync::Arc;

use sqlx::MySqlPool;
use uuid::Uuid;

use crate::models::Place;

pub async fn get_place(pool: Arc<MySqlPool>, place_id: Uuid) -> anyhow::Result<Option<Place>> {
    let place: Option<Place> = sqlx::query_as(r"select * from place where id=?")
        .bind(place_id.to_string())
        .fetch_optional(&*pool)
        .await?;
    Ok(place)
}
pub async fn get_places(
    pool: Arc<MySqlPool>,
    place_ids: Option<Vec<Uuid>>,
) -> anyhow::Result<Vec<Place>> {
    let places = match place_ids {
        Some(place_ids) => {
            let query_string = format!(
                "select * from place where id in (?{})",
                ", ?".repeat(place_ids.len() - 1)
            );
            let mut query = sqlx::query_as(&query_string);

            for place_id in place_ids.iter() {
                query = query.bind(place_id.to_string());
            }

            query.fetch_all(&*pool).await?
        }
        None => {
            let query = sqlx::query_as("select * from place");
            query.fetch_all(&*pool).await?
        }
    };
    Ok(places)
}
pub async fn add_place(
    pool: Arc<MySqlPool>,
    name: String,
    address: String,
) -> anyhow::Result<Place> {
    let place = Place {
        id: Uuid::new_v4(),
        name,
        address,
    };
    sqlx::query(r"insert into place (id, name, address) values (?, ?, ?)")
        .bind(place.id.to_string())
        .bind(place.name.to_string())
        .bind(place.address.to_string())
        .execute(&*pool)
        .await?;
    Ok(place)
}
