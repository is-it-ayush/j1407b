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

    #[error("Deserialize error: {source}")]
    Deserialize { source: toml::de::Error },

    #[error("Serialize error: {source}")]
    Serialize { source: toml::ser::Error },
}
