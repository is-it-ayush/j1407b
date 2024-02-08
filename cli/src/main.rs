mod clap;
mod cli;
mod config;
mod error;

use cli::Cli;
use error::CliError;

fn main() -> Result<(), CliError> {
    let mut cli = Cli::new()?;
    match cli.execute() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: {}", err.to_string());
        }
    }
    Ok(())
}
