use thiserror::Error;

/// The Result type used across this binary
pub type MResult<T> = Result<T, MError>;

/// Custom error enum used across this binary, contains both native error types and wraps external ones.
#[derive(Debug, Error)]
pub enum MError {
    #[error("Error reading from sled database: {0:?}")]
    DatabaseError(#[from] sled::Error),

    #[error("Error deserializing save data: {0:?}")]
    DeserializeError(#[from] rmp_serde::decode::Error),

    #[error("Error serializing save data: {0:?}")]
    SerializeError(#[from] rmp_serde::encode::Error),

    #[error("I/O Error: {0:?}")]
    IOError(#[from] std::io::Error),
}
