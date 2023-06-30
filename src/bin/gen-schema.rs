use std::sync::Arc;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use blog::{
    handler::graphql::{blog::BlogQuery, GQLSchema},
    repository::blog::MockBlogRepository,
    service::blog::BlogServiceImpl,
};

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
    println!("{}", &schema.sdl());
}
