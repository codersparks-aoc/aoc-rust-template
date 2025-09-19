use thiserror::Error;

#[derive(Debug, Error)]
pub enum AOCError {
    #[error("Error when running part {0}: {1}")]
    PartError(i8, String),
    #[error("Multiple Errors detected: {0:?}")]
    Multiple(Vec<AOCError>)
}