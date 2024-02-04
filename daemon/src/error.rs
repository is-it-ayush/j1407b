use shared::error::SharedError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DaemonError {
    #[error("Failed to create new child process: {errno}")]
    Clone {
        #[source]
        errno: nix::Error,
    },

    #[error("Failed to wait for child process: {errno}")]
    Wait {
        #[source]
        errno: nix::Error,
    },

    #[error("Shared Errror: {0}")]
    Shared(#[from] SharedError),
}
