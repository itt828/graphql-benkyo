use std::sync::Arc;

use crate::{
    domain::blog,
    repository::mysql::{blog::BlogRepositoryImpl, connect_db},
    service::blog::BlogServiceImpl,
    session::oauth_client,
};
use oauth2::basic::BasicClient;

#[derive(Clone)]
pub struct Container {
    pub blog_service: Arc<BlogServiceImpl<BlogRepositoryImpl>>,
    pub oauth_client: BasicClient,
}

impl Container {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = connect_db().await?;
        let repository = Arc::new(BlogRepositoryImpl {
            pool: Arc::new(pool),
        });
        let blog_service = Arc::new(BlogServiceImpl { repository });
        let oauth_client = oauth_client().await?;
        Ok(Self {
            blog_service,
            oauth_client,
        })
    }
}
