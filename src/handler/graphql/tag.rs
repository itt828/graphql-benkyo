use async_graphql::{SimpleObject, ID};

#[derive(SimpleObject)]
pub struct Tag {
    id: ID,
    name: String,
}
