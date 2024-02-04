use thiserror::Error;

#[derive(Error, Debug)]
pub enum SharedError {
    #[error("Invalid variable: {variable}")]
    InvalidVariable {
        variable: String,
        source: std::env::VarError,
    },

    #[error("IO error: {source}")]
    IoError { source: std::io::Error },

    #[error("Deserialize error: {source}")]
    DeserializeError { source: toml::de::Error },

    #[error("Serialize error: {source}")]
    SerializeError { source: toml::ser::Error },
}
