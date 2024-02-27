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

    #[error("Failed to serialize message: {0}")]
    MessageSerialize(String),

    #[error("Failed to deserialize message: {0}")]
    MessageDeserialize(String),

    #[error("Deserialize error: {source}")]
    ConfigDeserialize { source: toml::de::Error },

    #[error("Serialize error: {source}")]
    ConfigSerialize { source: toml::ser::Error },
}

impl serde::ser::Error for SharedError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        SharedError::MessageSerialize(msg.to_string())
    }
}

impl serde::de::Error for SharedError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        SharedError::MessageDeserialize(msg.to_string())
    }
}
