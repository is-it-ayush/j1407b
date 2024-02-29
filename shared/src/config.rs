//! The config holder is a wrapper around a configuration struct that is
//! generic over T. This means if you implement the `DefaultConfig` trait
//! over any struct, you can grant it several methods for reading and writing
//! to disk. By default, the config holder will look for a file in the
//! user's home directory under `~/.config/j1407b`. If the file does not
//! exist, it will create it and write the default configuration to it.
//! If the file does exist, it will read the configuration from it and
//! deserialize it into the struct. It uses `toml` for serialization and
//! deserialization.
//!
//! # Example
//! ```rust
//! use j1407b_shared::config::{ConfigHolder, DefaultConfig};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, Deserialize, Serialize)]
//! struct MyConfig {
//!    value: i32,
//! }
//!
//! impl DefaultConfig for MyConfig {
//!   fn default(path: &str) -> Self {
//!   MyConfig { value: 42 }
//! }
//!
//! // create a new config holder. `my_config.toml` will be created in the
//! // user's home directory if it does not exist.
//! let mut config = ConfigHolder::<MyConfig>::new("my_config").unwrap();
//!
//! // update the value.
//! config.config.value = 43;
//!
//! // write the config to disk.
//! config.write().unwrap();
//! ```

#![allow(clippy::redundant_field_names)]

use crate::{
    error::SharedError,
    utils::{self, ensure_directory},
};
use serde::{Deserialize, Serialize};
use std::{fs, io};

#[derive(Debug, Clone)]
pub struct ConfigHolder<T> {
    path: String,
    pub config: T,
}

/// A trait for defining a default implementation of a configuration.
pub trait DefaultConfig {
    /// A default implementation defined by the consumer.
    fn default(path: &str) -> Self;
}

impl<T> ConfigHolder<T>
where
    T: DefaultConfig + Serialize + for<'de> Deserialize<'de>,
{
    /// Initialize the config holder.
    pub fn new(file_name: &str) -> Result<Self, SharedError> {
        let base_path = Self::get_base_path()?;
        let config_path = format!("{}/{}.toml", base_path, file_name);
        let config = Self::read(base_path.as_str(), config_path.as_str())?;
        Ok(ConfigHolder {
            path: config_path,
            config: config,
        })
    }

    /// Read from disk.
    pub fn read(base_path: &str, config_path: &str) -> Result<T, SharedError> {
        ensure_directory(base_path)?;
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
            .map_err(|err| SharedError::ConfigDeserialize { source: err })?;
        Ok(config)
    }

    /// Write to disk.
    pub fn write(&self) -> Result<(), SharedError> {
        let config_string = toml::to_string(&self.config)
            .map_err(|err| SharedError::ConfigSerialize { source: err })?;
        fs::write(&self.path, config_string).map_err(|err| SharedError::IO { source: err })
    }

    /// Get the path to the config file.
    fn get_base_path() -> Result<String, SharedError> {
        let home = utils::get_home_dir()?;
        Ok(format!("{}/.config/j1407b", home))
    }
}
