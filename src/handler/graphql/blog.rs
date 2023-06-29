use std::{str::FromStr, sync::Arc};

use async_graphql::{Object, SimpleObject, ID};
use uuid::{uuid, Uuid};

use crate::{domain, service::blog::BlogService};

use super::tag::Tag;

#[derive(SimpleObject)]
pub struct Blog {
    id: ID,
    title: String,
    tags: Vec<Tag>,
    content: String,
}

pub struct BlogQuery<S: BlogService> {
    pub service: Arc<S>,
}

#[Object]
impl<S: BlogService + Sync + Send> BlogQuery<S> {
    pub async fn blog(&self, id: ID) -> anyhow::Result<Blog> {
        Ok(self.service.get_blog(Uuid::from_str(&id.0).unwrap()).into())
    }
    // pub async fn post_blog(blog: Blog) -> anyhow::Result<Blog> {}
}

// impl Into<domain::blog::Blog> for Blog {
//     fn into(self) -> domain::blog::Blog {
//         let x = self.id.0;
//         domain::blog::Blog {
//             id: Uuid::from_str(&x).unwrap(),
//             title: self.title,
//             content: self.content,
//             authors:
//         }
//     }
// }

impl Into<Blog> for domain::blog::Blog {
    fn into(self) -> Blog {
        Blog {
            id: ID(self.id.to_string()),
            title: self.title,
            tags: vec![],
            content: self.content,
        }
    }
}
