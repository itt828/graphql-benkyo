use std::sync::Arc;

use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    Extension,
};
use sqlx::MySqlPool;

use self::{mutation::Mutation, query::Query};

pub mod model;
pub mod mutation;
pub mod query;

pub fn init_schema(pool: Arc<MySqlPool>) -> Schema<Query, Mutation, EmptySubscription> {
    Schema::build(
        Query { pool: pool.clone() },
        Mutation { pool: pool.clone() },
        EmptySubscription,
    )
    .finish()
}

pub async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

pub async fn graphql_handler(
    schema: Extension<Schema<Query, Mutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
