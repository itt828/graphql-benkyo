use async_graphql::{SimpleObject, ID};

#[derive(SimpleObject)]
struct User {
    id: ID,
    name: String,
}
