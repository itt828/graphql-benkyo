use uuid::Uuid;

use crate::domain::model::post::Place;

#[async_trait::async_trait]
pub trait PlaceRepository {
    async fn get_place(&self, place_id: Uuid) -> anyhow::Result<Option<Place>>;
}
