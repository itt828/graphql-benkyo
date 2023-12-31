use crate::interface::graphql::{graphiql, graphql_handler, init_schema};
use crate::interface::session::oidc::init_google_oidc_client;
use crate::interface::session::{callback_handler, login_handler};
use crate::utils::gen_graphql_schema_file;

use axum::http::HeaderValue;
use axum::routing::get;
use axum::{Extension, Router};
use sqlx::MySqlPool;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

pub mod graphql;
pub mod session;

pub async fn startup(pool: Arc<MySqlPool>) -> anyhow::Result<()> {
    let oidc_client = Arc::new(init_google_oidc_client().await?);
    let schema = init_schema(pool.clone());
    gen_graphql_schema_file(&schema);
    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
        .route("/auth/login", get(login_handler))
        .route("/auth/callback", get(callback_handler))
        .with_state(oidc_client)
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
        .await?;
    Ok(())
}
