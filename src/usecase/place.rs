use std::sync::Arc;

use uuid::Uuid;

use crate::{
    domain::{model::post::Place, repository::place::PlaceRepository},
    infrastructure::module::RepositoriesModuleExt,
};

pub struct PlaceUseCase<R: RepositoriesModuleExt> {
    pub repositories: Arc<R>,
}

impl<R> PlaceUseCase<R>
where
    R: RepositoriesModuleExt,
{
    pub async fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }
    pub async fn get_place(&self, id: Uuid) -> anyhow::Result<Option<Place>> {
        self.repositories.place_repository().get_place(id).await
    }
    pub async fn get_places(&self, place_ids: Option<Vec<Uuid>>) -> anyhow::Result<Vec<Place>> {
        self.repositories
            .place_repository()
            .get_places(place_ids)
            .await
    }
    pub async fn add_place(&self, name: String, address: String) -> anyhow::Result<Place> {
        let place = Place {
            id: Uuid::new_v4(),
            name,
            address,
        };
        self.repositories
            .place_repository()
            .add_place(&place)
            .await?;
        Ok(place)
    }
}
