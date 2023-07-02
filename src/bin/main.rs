use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    http::HeaderValue,
    response::{self, IntoResponse},
    routing::get,
    Extension, Router,
};
use blog::{
    handler::graphql::{GQLSchema, Query},
    repository::{
        blog::{BlogRepository, MockBlogRepository},
        mysql::{blog::BlogRepositoryImpl, connect_db},
    },
    service::blog::BlogServiceImpl,
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

async fn graphql_handler<R: BlogRepository + Send + Sync + 'static>(
    schema: Extension<GQLSchema<R>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("db connecting");
    let pool = connect_db().await?;
    println!("db connected!");
    let repository = Arc::new(BlogRepositoryImpl {
        pool: Arc::new(pool),
    });
    let schema: GQLSchema<BlogRepositoryImpl> = Schema::build(
        Query {
            blog_service: Arc::new(BlogServiceImpl {
                // repository: Arc::new(MockBlogRepository),
                repository,
            }),
        },
        EmptyMutation,
        EmptySubscription,
    )
    .finish();
    let app = Router::new()
        .route(
            "/",
            get(graphiql).post(graphql_handler::<BlogRepositoryImpl>),
        )
        .layer(Extension(schema))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::exact(HeaderValue::from_static(
                    "http://localhost:3030",
                )))
                .allow_methods(Any)
                .allow_headers(Any),
        );
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
