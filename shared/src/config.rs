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
    T: DefaultConfig + Serialize + for<'de> Deserialize<'de>,
{
    path: String,
    pub config: T,
}

pub trait DefaultConfig {
    /// A default implementation defined by the consumer.
    fn default(path: &str) -> Self;
}

impl<T> ConfigHolder<T>
where
    T: DefaultConfig + Serialize + for<'de> Deserialize<'de>,
{
    /// initialize the config holder.
    pub fn new(file_name: &str) -> Result<Self, SharedError> {
        let base_path = Self::get_base_path()?;
        let config_path = format!("{}/{}.toml", base_path, file_name);
        let config = Self::read(base_path.as_str(), config_path.as_str())?;
        Ok(ConfigHolder {
            path: config_path,
            config: config,
        })
    }

    /// read from disk.
    pub fn read(base_path: &str, config_path: &str) -> Result<T, SharedError> {
        ensure_config_dir()?;
        let config_string = match fs::read_to_string(config_path) {
            Ok(config_string) => config_string,
            Err(err) => {
                if err.kind() == io::ErrorKind::NotFound {
                    String::new()
                } else {
                    return Err(SharedError::IO { source: err })?;
                }
            }
        };

        // if empty, return default.
        if config_string.is_empty() {
            return Ok(T::default(base_path)); // call the default implementation.
        }

        // parse & return.
        let config = toml::from_str::<T>(&config_string)
            .map_err(|err| SharedError::Deserialize { source: err })?;
        Ok(config)
    }

    /// write to disk.
    pub fn write(&self) -> Result<(), SharedError> {
        let config_string =
            toml::to_string(&self.config).map_err(|err| SharedError::Serialize { source: err })?;
        fs::write(&self.path, config_string).map_err(|err| SharedError::IO { source: err })
    }

    /// get the path to the config file.
    fn get_base_path() -> Result<String, SharedError> {
        let home = utils::get_home_dir()?;
        Ok(format!("{}/.config/j1407b", home))
    }
}
