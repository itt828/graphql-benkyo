pub mod blog;
pub mod tag;
pub mod user;
use crate::{
    repository::blog::{BlogRepository, MockBlogRepository},
    service::blog::BlogServiceImpl,
};

use self::blog::{Blog, BlogQuery};
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

// struct Query;
// #[Object]
// impl Query {
//     async fn blog(&self) -> anyhow::Result<Blog> {
//         a
//     }
//     async fn user(&self) -> anyhow::Result<Option<User>> {
//         Ok(Some(User {
//             id: ID(Uuid::new_v4().to_string()),
//             name: String::from("itt"),
//         }))
//     }
//     async fn tag(&self) -> anyhow::Result<Option<Tag>> {
//         Ok(Some(Tag {
//             id: ID(Uuid::new_v4().to_string()),
//             name: String::from("tag"),
//         }))
//     }
// }

pub type GQLSchema<R> = Schema<BlogQuery<BlogServiceImpl<R>>, EmptyMutation, EmptySubscription>;

// impl GQLSchema {
//     pub fn new() -> Self {
//         Schema::new(
//             BlogQuery {
//                 service: BlogServiceImpl {
//                     repository: MockBlogRepository,
//                 },
//             },
//             EmptyMutation,
//             EmptySubscription,
//         )
//     }
// }
