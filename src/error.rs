use thiserror::Error;

pub type LockBoxResult<T> = Result<T, LockBoxError>;

#[derive(Debug, Error)]
pub enum LockBoxError {
    #[error("Keyring error: {0}")]
    Keyring(String),

    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Format error: {0}")]
    Format(String),
}
