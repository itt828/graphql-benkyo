use uuid::Uuid;

use super::{tag::Tag, user::User};

pub struct Blog {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub authors: Vec<User>,
    pub tags: Vec<Tag>,
}

impl Blog {
    pub fn new(id: Uuid, title: &str, content: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            content: content.to_string(),
            authors: vec![],
            tags: vec![],
        }
    }
}
