use shared::error::SharedError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Shared Errror: {0}")]
    Shared(#[from] SharedError),
}
