use uuid::Uuid;

use crate::{domain::blog::Blog, repository::blog::BlogRepository};

pub trait BlogService {
    fn get_blog(&self, id: Uuid) -> Blog;
    // fn post_blog(&self, new_blog: Blog);
    // fn delete_blog(&self, id: Uuid);
}

pub struct BlogServiceImpl<R: BlogRepository> {
    pub repository: R,
}

impl<R: BlogRepository> BlogService for BlogServiceImpl<R> {
    fn get_blog(&self, id: Uuid) -> Blog {
        self.repository.get_blog(id)
    }
    // fn post_blog(&self, new_blog: Blog) {
    //     self.repository.create_blog(new_blog);
    // }
    // fn delete_blog(&self, id: Uuid) {
    //     let _ = self.repository.delete_blog(id);
    // }
}
