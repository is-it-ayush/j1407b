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
pub fn ensure_config_dir() -> Result<(), SharedError> {
    let home = get_home_dir()?;
    let config_dir = format!("{}/.config/j1407b", home);
    std::fs::create_dir_all(config_dir).map_err(|err| SharedError::IoError { source: err })
}
