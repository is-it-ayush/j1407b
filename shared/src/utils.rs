//! Utility functions for the shared library.

use crate::error::SharedError;

const HOME_VARIABLE: &str = "HOME";

/// Get the home directory of the current user.
pub fn get_home_dir() -> Result<String, SharedError> {
    std::env::var(HOME_VARIABLE).map_err(|err| SharedError::InvalidVariable {
        variable: HOME_VARIABLE.to_string(),
        source: err,
    })
}

/// Get the path to the config file.
pub fn ensure_directory(directory: &str) -> Result<(), SharedError> {
    std::fs::create_dir_all(directory).map_err(|err| SharedError::IO { source: err })
}
