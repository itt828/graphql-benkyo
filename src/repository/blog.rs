use std::vec;

use uuid::Uuid;

use crate::domain::blog::Blog;
use crate::domain::user::User;

pub trait BlogRepository {
    fn get_blog(&self, id: Uuid) -> Blog;
    // fn create_blog(&self, blog: Blog);
    // fn delete_blog(&self, id: Uuid);
}

pub struct MockBlogRepository;
impl BlogRepository for MockBlogRepository {
    fn get_blog(&self, id: Uuid) -> Blog {
        Blog {
            id,
            title: "test title".to_string(),
            content: "test content".to_string(),
            authors: vec![
                User {
                    id: uuid::Uuid::new_v4(),
                    name: "itt".to_string(),
                },
                User {
                    id: uuid::Uuid::new_v4(),
                    name: "iitt".to_string(),
                },
            ],
            tags: vec![],
        }
    }
}
