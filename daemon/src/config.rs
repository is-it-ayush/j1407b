use serde::{Deserialize, Serialize};
use shared::config::DefaultConfig;

pub const CONFIG_FILE_NAME: &str = "daemon";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub images_dir: String,
    pub containers_dir: String,
}

impl DefaultConfig for Config {
    // we have access to the path here.
    fn default(path: &str) -> Self {
        Config {
            images_dir: format!("{}/images", path),
            containers_dir: format!("{}/containers", path),
        }
    }
}
