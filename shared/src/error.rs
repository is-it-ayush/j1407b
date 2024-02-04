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

    #[error("Deserialize error: {source}")]
    Deserialize { source: toml::de::Error },

    #[error("Serialize error: {source}")]
    Serialize { source: toml::ser::Error },
}
