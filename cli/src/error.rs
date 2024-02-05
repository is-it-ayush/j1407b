use shared::error::SharedError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Shared Errror: {0}")]
    Shared(#[from] SharedError),

    #[error("Failed to connect to socket: {errno}")]
    Connect {
        #[source]
        errno: std::io::Error,
    },

    #[error("Failed to write to socket: {errno}")]
    Write {
        #[source]
        errno: std::io::Error,
    },
}
