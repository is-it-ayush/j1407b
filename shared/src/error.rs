//! Error types for the shared library. It uses `thiserror` for error handling.

use std::os::fd::RawFd;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SharedError {
    #[error("Invalid variable: {variable}")]
    InvalidVariable {
        variable: String,
        source: std::env::VarError,
    },

    #[error("IO error: {source}")]
    IO { source: std::io::Error },

    #[error("Failed to create socket: {errno}")]
    CreateSocket {
        #[source]
        errno: nix::errno::Errno,
    },

    #[error("Failed to create socket address (sockaddr_un): {errno}")]
    CreateUnixAddr {
        #[source]
        errno: nix::errno::Errno,
    },

    #[error(
        "Failed to read a connection fd {conn_fd} from socket descriptor {socket_fd}: {errno}"
    )]
    ReadSocketConnection {
        socket_fd: RawFd,
        conn_fd: RawFd,
        #[source]
        errno: nix::errno::Errno,
    },

    #[error("Failed to serialize message: {0}")]
    MessageSerialize(String),

    #[error("Failed to deserialize message: {0}")]
    MessageDeserialize(String),

    #[error("Deserialize error: {source}")]
    ConfigDeserialize { source: toml::de::Error },

    #[error("Serialize error: {source}")]
    ConfigSerialize { source: toml::ser::Error },
}
