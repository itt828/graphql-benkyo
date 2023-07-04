use super::error::BlogError;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Blog {
    pub id: Uuid,
    pub title: BlogTitle,
    pub content: String,
    pub authors: Vec<Uuid>,
    pub tags: Vec<Uuid>,
}

impl Blog {
    pub fn new(id: Uuid, title: &str, content: &str) -> anyhow::Result<Self> {
        Ok(Self {
            id,
            title: BlogTitle::new(title)?,
            content: content.to_string(),
            authors: vec![],
            tags: vec![],
        })
    }
    pub fn change_title(&mut self, new_title: &str) -> anyhow::Result<()> {
        self.title = BlogTitle::new(new_title)?;
        Ok(())
    }
    pub fn change_content(&mut self, new_content: &str) {
        self.content = new_content.to_string();
    }
    pub fn add_author(&mut self, new_author: Uuid) -> Result<(), BlogError> {
        if self.authors.iter().all(|&author| author != new_author) {
            self.authors.push(new_author);
            Ok(())
        } else {
            Err(BlogError::ValidationError(
                "同じauthorを登録することはできません".to_string(),
            ))
        }
    }
    pub fn remove_author(&mut self, author_id: Uuid) -> Result<(), BlogError> {
        if self.authors.iter().all(|&author| author != author_id) {
            Err(BlogError::UserNotFoundError(author_id))
        } else if self.authors.len() == 1 {
            Err(BlogError::ValidationError(
                "作者は1人以上にして下さい".to_string(),
            ))
        } else {
            // self.authors = self.authors.iter().
            Ok(())
        }
    }
    pub fn add_tag(&mut self, new_tag: Uuid) -> Result<(), BlogError> {
        if self.tags.iter().all(|&tag| tag != new_tag) {
            self.authors.push(new_tag);
            Ok(())
        } else {
            Err(BlogError::ValidationError(
                "同じtagを登録することはできません".to_string(),
            ))
        }
    }
    pub fn remove_tag(&mut self, tag_id: Uuid) -> Result<(), BlogError> {
        if self.tags.iter().all(|&tag| tag != tag_id) {
            Err(BlogError::UserNotFoundError(tag_id))
        } else {
            // let _ = self.tags.
            Ok(())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlogTitle(pub String);
impl BlogTitle {
    pub fn new(s: &str) -> Result<Self, BlogError> {
        if s.chars().count() < 50 {
            Ok(BlogTitle(s.to_string()))
        } else {
            Err(BlogError::ValidationError(
                "ブログタイトルの長さは50文字以下にしてください。".to_string(),
            ))
        }
    }
}
