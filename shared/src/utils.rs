//! Utility functions for the shared library.

use crate::{
    comms::{Command, Header, Type},
    error::SharedError,
};

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

/// Utility function to build a header for a request.
pub fn build_header(_type: Type, command: Command, body_size: u64) -> Result<Vec<u8>, SharedError> {
    let header = Header::new(_type, command, body_size);
    let header_bytes = rust_fr::serializer::to_bytes(&header)
        .map_err(|e| SharedError::MessageSerialize(e.to_string()))?;
    Ok(header_bytes)
}

/// Utility function to build a body for a request.
pub fn build_body<T: serde::Serialize>(body: T) -> Result<Vec<u8>, SharedError> {
    let body_bytes = rust_fr::serializer::to_bytes(&body)
        .map_err(|e| SharedError::MessageSerialize(e.to_string()))?;
    Ok(body_bytes)
}

/// Utility function to build a message from a header and a body.
pub fn build_message(header: Vec<u8>, body: Vec<u8>) -> Vec<u8> {
    [header, body].concat()
}
