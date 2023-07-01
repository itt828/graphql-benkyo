use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlogError {
    #[error("ValidationError: {0}")]
    ValidationError(String),
}
