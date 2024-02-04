mod config;
mod error;
mod utils;

use config::{Config, CONFIG_FILE_NAME};
use error::CliError;
use shared::config::ConfigHolder;

fn main() -> Result<(), CliError> {
    let _config = ConfigHolder::<Config>::new(CONFIG_FILE_NAME)?;
    Ok(())
}
