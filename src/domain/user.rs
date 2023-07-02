use uuid::Uuid;

use super::error::BlogError;

pub struct User {
    pub id: Uuid,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserName(String);

impl UserName {
    pub fn new(s: &str) -> Result<Self, BlogError> {
        if s.chars().count() < 20 {
            Ok(UserName(s.to_string()))
        } else {
            Err(BlogError::ValidationError(
                "ユーザー名は20文字以下にしてください。".to_string(),
            ))
        }
    }
}
