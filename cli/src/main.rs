mod error;
mod utils;

use error::CliError;
use serde::{Deserialize, Serialize};
use shared::config::ConfigHolder;

const CONFIG_FILE_NAME: &str = "cli";
#[derive(Debug, Default, Serialize, Deserialize)]
struct Config {
    name: String,
}

fn main() -> Result<(), CliError> {
    let config = ConfigHolder::<Config>::new(CONFIG_FILE_NAME)?;
    Ok(())
}
