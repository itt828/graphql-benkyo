use async_graphql::{http::GraphiQLSource, *};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    http::HeaderValue,
    response::{self, IntoResponse},
    routing::get,
    Extension, Router,
};
use std::net::SocketAddr;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use uuid::Uuid;

#[derive(SimpleObject)]
struct User {
    id: ID,
    name: String,
}

#[derive(SimpleObject)]
struct Tag {
    id: ID,
    name: String,
}

#[derive(SimpleObject)]
struct Blog {
    id: ID,
    title: String,
    tags: Vec<Tag>,
    content: String,
}

struct Query;
#[Object]
impl Query {
    async fn user(&self) -> anyhow::Result<Option<User>> {
        Ok(Some(User {
            id: ID(Uuid::new_v4().to_string()),
            name: String::from("itt"),
        }))
    }
    async fn tag(&self) -> anyhow::Result<Option<Tag>> {
        Ok(Some(Tag {
            id: ID(Uuid::new_v4().to_string()),
            name: String::from("tag"),
        }))
    }
}
type GQLSchema = Schema<Query, EmptyMutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<GQLSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let schema: GQLSchema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();
    println!("{}", &schema.sdl());
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
