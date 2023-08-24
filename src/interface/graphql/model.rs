use async_graphql::{SimpleObject, ID};

use crate::models::{Avater as DomainAvater, Emoji as DomainEmoji, Place as DomainPlace};

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

impl From<DomainEmoji> for Emoji {
    fn from(value: DomainEmoji) -> Self {
        Emoji {
            id: ID(value.id.to_string()),
            name: value.name.to_string(),
        }
    }
}

impl From<DomainPlace> for Place {
    fn from(value: DomainPlace) -> Self {
        Place {
            id: ID(value.id.to_string()),
            name: value.name,
            address: value.address,
        }
    }
}

impl From<DomainAvater> for Avater {
    fn from(value: DomainAvater) -> Self {
        Avater {
            id: ID(value.id.to_string()),
            name: value.name,
        }
    }
}
