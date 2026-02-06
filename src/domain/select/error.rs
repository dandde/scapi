use thiserror::Error;

#[derive(Error, Debug)]
pub enum SelectError {
    #[error("Invalid selector: {0}")]
    InvalidSelector(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),
}
