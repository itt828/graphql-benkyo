use uuid::Uuid;

use super::{tag::Tag, user::User};

pub struct Blog {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub authors: Vec<User>,
    pub tags: Vec<Tag>,
}
