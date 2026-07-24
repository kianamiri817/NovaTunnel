use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Provider not available: {0}")]
    ProviderNotAvailable(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Session expired")]
    SessionExpired,

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("Tunnel not connected")]
    NotConnected,

    #[error("Already connected")]
    AlreadyConnected,

    #[error("Timeout")]
    Timeout,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
