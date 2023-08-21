use async_graphql::{SimpleObject, ID};

use crate::domain::model::{
    post::{Emoji as DomainEmoji, Place as DomainPlace},
    user::Avater as DomainAvater,
};

#[derive(SimpleObject)]
pub struct Post {
    pub id: ID,
    pub avater: Avater,
    pub emoji: Emoji,
    pub place: Place,
    pub title: String,
    pub comment: String,
    pub visited_at: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(SimpleObject)]
pub struct Avater {
    id: ID,
    name: String,
}

#[derive(SimpleObject)]
pub struct Emoji {
    id: ID,
    name: String,
}

#[derive(SimpleObject)]
pub struct Place {
    id: ID,
    name: String,
    address: String,
}

#[derive(SimpleObject)]
pub struct Account {
    id: ID,
    email: String,
    avaters: Vec<Avater>,
}

impl Into<Emoji> for DomainEmoji {
    fn into(self) -> Emoji {
        Emoji {
            id: ID(self.id.to_string()),
            name: self.name.to_string(),
        }
    }
}

impl Into<Place> for DomainPlace {
    fn into(self) -> Place {
        Place {
            id: ID(self.id.to_string()),
            name: self.name,
            address: self.address,
        }
    }
}

impl Into<Avater> for DomainAvater {
    fn into(self) -> Avater {
        Avater {
            id: ID(self.id.to_string()),
            name: self.name,
        }
    }
}
