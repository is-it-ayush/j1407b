use shared::error::SharedError;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DaemonError {
    #[error("Failed to create new child process: {errno}")]
    CloneSyscall {
        #[source]
        errno: nix::Error,
    },
    #[error("Failed to wait for child process: {errno}")]
    WaitSyscall {
        #[source]
        errno: nix::Error,
    },

    #[error("Shared Errror: {0}")]
    Shared(#[from] SharedError),

    #[error("Failed to create socket: {errno}")]
    CreateSocket {
        #[source]
        errno: io::Error,
    },

    #[error("Failed to read from socket: {source}")]
    ReadSocket {
        #[source]
        source: io::Error,
    },

    #[error("Could not convert to an UTF8 string: {source}")]
    InvalidUtf8 {
        #[from]
        source: std::string::FromUtf8Error,
    },

    #[error("Failed to clone stream: {source}")]
    CloneStream {
        #[source]
        source: io::Error,
    },
}
