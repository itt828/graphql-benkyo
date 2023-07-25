use crate::interface::graphql::{graphiql, graphql_handler, init_schema};
use crate::interface::session::callback_handler;
use crate::utils::{gen_graphql_schema_file, oauth_client};

use self::modules::Modules;
use axum::http::HeaderValue;
use axum::routing::get;
use axum::{Extension, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

pub mod graphql;
pub mod modules;
pub mod session;

pub async fn startup(modules: Arc<Modules>) -> anyhow::Result<()> {
    let oauth_client = oauth_client().await?;
    let schema = init_schema(modules.clone(), oauth_client.clone());
    gen_graphql_schema_file(&schema);
    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
        .route("/auth/callback", get(callback_handler))
        .layer(CookieManagerLayer::new())
        .with_state((modules, oauth_client))
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
