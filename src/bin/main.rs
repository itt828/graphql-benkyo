use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    http::HeaderValue,
    response::{self, IntoResponse},
    routing::get,
    Extension, Router,
};
use blog::{
    handler::graphql::{blog::BlogQuery, GQLSchema},
    repository::blog::MockBlogRepository,
    service::blog::BlogServiceImpl,
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

async fn graphql_handler(schema: Extension<GQLSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let schema: GQLSchema = Schema::build(
        BlogQuery {
            service: Arc::new(BlogServiceImpl {
                repository: MockBlogRepository,
            }),
        },
        EmptyMutation,
        EmptySubscription,
    )
    .finish();
    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::exact(HeaderValue::from_static(
                    "http://localhost:3030",
                )))
                .allow_methods(Any)
                .allow_headers(Any),
        );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
