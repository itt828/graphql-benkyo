use sqlx::MySqlPool;
use std::sync::Arc;
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
        Ok(None)
    }
}
