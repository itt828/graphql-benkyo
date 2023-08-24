use sqlx::MySqlPool;
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

use crate::domain::{model::post::Place, repository::place::PlaceRepository};

pub struct PlaceRepositoryImpl {
    pub pool: Arc<MySqlPool>,
}

impl PlaceRepositoryImpl {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl PlaceRepository for PlaceRepositoryImpl {
    async fn get_place(&self, place_id: Uuid) -> anyhow::Result<Option<Place>> {
        let pool = self.pool.clone();
        let emoji: Option<Place> = sqlx::query_as(r"select * from place where id=?")
            .bind(place_id.to_string())
            .fetch_optional(&*pool)
            .await?;
        Ok(emoji.map(|v| v.into()))
    }
    async fn get_places(&self, place_ids: Option<Vec<Uuid>>) -> anyhow::Result<Vec<Place>> {
        let pool = self.pool.clone();
        let places = match place_ids {
            Some(place_ids) => {
                let query_string = format!(
                    "select * from place where id in (?{})",
                    ", ?".repeat(place_ids.len() - 1)
                );
                let mut query = sqlx::query_as(&query_string);

                for place_id in place_ids.iter() {
                    query = query.bind(place_id);
                }

                query.fetch_all(&*pool).await?
            }
            None => {
                let mut query = sqlx::query_as("select * from place");
                query.fetch_all(&*pool).await?
            }
        };
        Ok(places)
    }
    async fn add_place(&self, place: &Place) -> anyhow::Result<()> {
        let pool = self.pool.clone();
        sqlx::query(r"insert into place (id, name, address) values (?, ?, ?)")
            .bind(place.id.to_string())
            .bind(place.name.to_string())
            .bind(place.address.to_string())
            .execute(&*pool)
            .await?;
        Ok(())
    }
}

// #[derive(Clone, Debug, sqlx::FromRow)]
// pub struct PlaceRecord {
//     id: String,
//     name: String,
//     address: String,
// }

// impl Into<PlaceRecord> for Place {
//     fn into(self) -> PlaceRecord {
//         PlaceRecord {
//             id: self.id.to_string(),
//             name: self.name,
//             address: self.address,
//         }
//     }
// }

// impl Into<Place> for PlaceRecord {
//     fn into(self) -> Place {
//         Place {
//             id: Uuid::from_str(&self.id).unwrap(),
//             name: self.name,
//             address: self.address,
//         }
//     }
// }
