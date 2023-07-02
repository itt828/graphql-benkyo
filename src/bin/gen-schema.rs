use std::{io::Write, sync::Arc};

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use blog::{
    handler::graphql::{GQLSchema, Mutation, Query},
    repository::blog::MockBlogRepository,
    service::blog::BlogServiceImpl,
};
use std::fs::File;

fn main() {
    let repository = Arc::new(MockBlogRepository);
    let schema: GQLSchema<MockBlogRepository> = Schema::build(
        Query {
            blog_service: Arc::new(BlogServiceImpl {
                repository: repository.clone(),
            }),
        },
        Mutation {
            blog_service: Arc::new(BlogServiceImpl { repository }),
        },
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
