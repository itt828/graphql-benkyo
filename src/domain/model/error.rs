use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum BlogError {
    #[error("バリデーションに失敗しました: {0}")]
    ValidationError(String),
    #[error("idが{0}のユーザーは存在しません")]
    UserNotFoundError(Uuid),
    #[error("idが{0}のタグは存在しません")]
    TagNotFoundError(Uuid),
}
