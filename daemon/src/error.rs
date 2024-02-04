use shared::error::SharedError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DaemonError {
    #[error("Failed to create new child process: {errno}")]
    CloneError {
        #[source]
        errno: nix::Error,
    },

    #[error("Failed to wait for child process: {errno}")]
    WaitError {
        #[source]
        errno: nix::Error,
    },

    #[error("Shared Errror: {0}")]
    SharedError(#[from] SharedError),
}
