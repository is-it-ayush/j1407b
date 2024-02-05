mod clap;
mod cli;
mod config;
mod error;

use cli::Cli;
use error::CliError;

fn main() -> Result<(), CliError> {
    let mut cli = Cli::new()?;
    cli.execute()?;
    Ok(())
}
