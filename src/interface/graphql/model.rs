use async_graphql::{SimpleObject, ID};

#[derive(SimpleObject)]
pub struct Post {
    id: ID,
    avater: Avater,
    emoji: Emoji,
    place: Place,
    title: String,
    comment: String,
    visited_at: String,
    created_at: String,
    updated_at: String,
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
