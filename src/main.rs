use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    http::HeaderValue,
    response::{self, IntoResponse},
    routing::get,
    Extension, Router,
};
use blog::{
    container::Container,
    handler::{
        graphql::{GQLSchema, Mutation, Query},
        session::callback_handler,
    },
    repository::{blog::BlogRepository, mysql::blog::BlogRepositoryImpl},
    utils::gen_graphql_schema_file,
};
use std::{net::SocketAddr, sync::Arc};
use tower_cookies::CookieManagerLayer;
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
    dotenvy::dotenv()?;

    let container = Arc::new(Container::new().await?);
    let service = container.blog_service.clone();
    let oauth_client = container.oauth_client.clone();
    let schema: GQLSchema<BlogRepositoryImpl> = Schema::build(
        Query {
            blog_service: service.clone(),
        },
        Mutation {
            blog_service: service.clone(),
            client: oauth_client.clone(),
        },
        EmptySubscription,
    )
    .finish();
    gen_graphql_schema_file(&schema);

    let app = Router::new()
        .route(
            "/",
            get(graphiql).post(graphql_handler::<BlogRepositoryImpl>),
        )
        .layer(Extension(schema))
        .route("/callback", get(callback_handler))
        .layer(CookieManagerLayer::new())
        .with_state(container)
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
