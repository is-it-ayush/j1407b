#![allow(clippy::redundant_field_names)]

use crate::{
    error::SharedError,
    utils::{self, ensure_config_dir},
};
use serde::{Deserialize, Serialize};
use std::{fs, io};

#[derive(Debug, Clone)]
pub struct ConfigHolder<T>
where
    T: Default + Serialize + for<'de> Deserialize<'de>,
{
    path: String,
    pub config: T,
}

impl<T> ConfigHolder<T>
where
    T: Default + Serialize + for<'de> Deserialize<'de>,
{
    /// initialize the config holder.
    pub fn new(file_name: &str) -> Result<Self, SharedError> {
        let path = Self::get_config_path(file_name)?;
        let config = Self::read(&path)?;
        Ok(ConfigHolder {
            path: path,
            config: config,
        })
    }

    /// read from disk.
    pub fn read(path: &String) -> Result<T, SharedError> {
        ensure_config_dir()?;
        let config_string = match fs::read_to_string(path) {
            Ok(config_string) => config_string,
            Err(err) => {
                if err.kind() == io::ErrorKind::NotFound {
                    String::new()
                } else {
                    return Err(SharedError::IoError { source: err })?;
                }
            }
        };

        // if empty, return default.
        if config_string.is_empty() {
            return Ok(T::default());
        }

        // parse & return.
        let config = toml::from_str::<T>(&config_string)
            .map_err(|err| SharedError::DeserializeError { source: err })?;
        Ok(config)
    }

    /// write to disk.
    pub fn write(&self) -> Result<(), SharedError> {
        let config_string = toml::to_string(&self.config)
            .map_err(|err| SharedError::SerializeError { source: err })?;
        fs::write(&self.path, config_string).map_err(|err| SharedError::IoError { source: err })
    }

    /// get the path to the config file.
    fn get_config_path(file_name: &str) -> Result<String, SharedError> {
        let home = utils::get_home_dir()?;
        Ok(format!("{}/.config/j1407b/{}.toml", home, file_name))
    }
}
