use std::{io::Write, sync::Arc};

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use blog::{
    handler::graphql::{blog::BlogQuery, GQLSchema},
    repository::blog::MockBlogRepository,
    service::blog::BlogServiceImpl,
};
use std::fs::File;

fn main() {
    let repository = Arc::new(MockBlogRepository);
    let schema: GQLSchema<MockBlogRepository> = Schema::build(
        BlogQuery {
            service: Arc::new(BlogServiceImpl { repository }),
        },
        EmptyMutation,
        EmptySubscription,
    )
    .finish();
    let mut file = File::create("schema.graphql").unwrap();
    let schema_text = format!(
        r"# Auto generated. DO NOT EDIT.
{}",
        schema.sdl()
    );
    let _ = file.write_all(schema_text.as_bytes());
}
