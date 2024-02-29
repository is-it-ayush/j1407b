use serde::{Deserialize, Serialize};
use shared::config::DefaultConfig;

pub const CONFIG_FILE_NAME: &str = "daemon";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub images_dir: String,
    pub containers_dir: String,
    pub registry: Registry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Registry {
    pub url: String,
    pub token: String,
    pub https: bool,
}

impl Registry {
    /// Get the URL of the registry.
    pub fn get_url(&self) -> String {
        if self.https {
            format!("https://{}", self.url)
        } else {
            format!("http://{}", self.url)
        }
    }
}

impl DefaultConfig for Config {
    // we have access to the path here.
    fn default(path: &str) -> Self {
        Config {
            images_dir: format!("{}/images", path),
            containers_dir: format!("{}/containers", path),
            registry: Registry {
                url: "hub.docker.com".to_string(),
                token: "".to_string(),
                https: true,
            },
        }
    }
}
