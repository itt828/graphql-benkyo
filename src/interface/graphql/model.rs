use async_graphql::{SimpleObject, ID};

#[derive(SimpleObject)]
pub struct Blog {
    id: ID,
    title: String,
    tags: Vec<Tag>,
    content: String,
}

#[derive(SimpleObject)]
struct User {
    id: ID,
    name: String,
}
#[derive(SimpleObject)]
pub struct Tag {
    id: ID,
    name: String,
}

impl From<crate::domain::model::blog::Blog> for Blog {
    fn from(blog: crate::domain::model::blog::Blog) -> Self {
        Blog {
            id: ID(blog.id.to_string()),
            title: blog.title.0,
            tags: vec![],
            content: blog.content,
        }
    }
}
