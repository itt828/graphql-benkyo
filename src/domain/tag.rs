use uuid::Uuid;

use super::error::BlogError;

pub struct Tag {
    pub id: Uuid,
    pub name: String,
}

impl Tag {
    pub fn new(id: Uuid, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TagName(String);

impl TagName {
    pub fn new(s: &str) -> Result<Self, BlogError> {
        if s.chars().count() < 20 {
            Ok(TagName(s.to_string()))
        } else {
            Err(BlogError::ValidationError(
                "タグの長さは20文字以下にしてください。".to_string(),
            ))
        }
    }
}
