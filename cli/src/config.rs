use serde::{Deserialize, Serialize};
use shared::config::DefaultConfig;

pub const CONFIG_FILE_NAME: &str = "cli";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    something: i32,
}

impl DefaultConfig for Config {
    fn default(_path: &str) -> Self {
        Self { something: 0 }
    }
}
