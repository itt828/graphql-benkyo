use std::{io::Write, sync::Arc};

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use blog::{
    handler::graphql::{blog::BlogQuery, GQLSchema},
    repository::blog::MockBlogRepository,
    service::blog::BlogServiceImpl,
};
use std::fs::File;

fn main() {
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

    let mut file = File::create("schema.graphql").unwrap();
    let _ = file.write_all(schema.sdl().as_bytes());
}
