use std::sync::Arc;

use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    Extension,
};
use oauth2::basic::BasicClient;

use self::{mutation::Mutation, query::Query};

use super::modules::Modules;

pub mod model;
pub mod mutation;
pub mod query;

pub fn init_schema(
    modules: Arc<Modules>,
    oauth_client: BasicClient,
) -> Schema<Query, Mutation, EmptySubscription> {
    Schema::build(
        Query {
            modules: modules.clone(),
        },
        Mutation {
            modules,
            oauth_client,
        },
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
