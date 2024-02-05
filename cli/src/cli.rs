use crate::{
    clap::{ClapCli, Commands},
    config::{Config, CONFIG_FILE_NAME},
    error::CliError,
};
use clap::Parser;
use shared::config::ConfigHolder;
use std::{io::Write, os::unix::net::UnixStream};

pub struct Cli {
    pub config: ConfigHolder<Config>,
    pub cli: ClapCli,
    pub socket: UnixStream,
}

impl Cli {
    pub fn new() -> Result<Self, CliError> {
        let config = ConfigHolder::<Config>::new(CONFIG_FILE_NAME)?;
        let mothership = UnixStream::connect("/tmp/j1047b.sock")
            .map_err(|err| CliError::Connect { errno: err })?;
        Ok(Cli {
            config,
            cli: ClapCli::parse(),
            socket: mothership,
        })
    }

    pub fn execute(&mut self) -> Result<(), CliError> {
        match &self.cli.command {
            Some(Commands::Pull { image }) => self.pull(image.clone()),
            None => Ok(()),
        }
    }

    fn pull(&mut self, image: String) -> Result<(), CliError> {
        self
            .socket
            .write_all(image.as_bytes())
            .map_err(|err| CliError::Write { errno: err })?;
        Ok(())
    }
}
