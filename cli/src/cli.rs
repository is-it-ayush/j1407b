use crate::{
    clap::{ClapCli, Commands},
    config::{Config, CONFIG_FILE_NAME},
    error::CliError,
};
use clap::Parser;
use nix::{
    sys::socket::{connect, socket, AddressFamily, SockFlag, SockType, UnixAddr},
    unistd::write,
};
use shared::{config::ConfigHolder, error::SharedError};
use std::os::fd::{AsRawFd, OwnedFd};

pub struct Cli {
    pub config: ConfigHolder<Config>,
    pub cli: ClapCli,
    pub socket_fd: OwnedFd,
}

const DAEMON_SOCKET: &str = "/tmp/j1047b.sock";

impl Cli {
    /// create a new Cli instance
    pub fn new() -> Result<Self, CliError> {
        let config = ConfigHolder::<Config>::new(CONFIG_FILE_NAME)?;

        // create a new socket address (sockaddr_un)
        let socket_addr = UnixAddr::new(DAEMON_SOCKET.as_bytes())
            .map_err(|e| SharedError::CreateUnixAddr { errno: e })?;

        println!("Socket address: {:?}", socket_addr);

        // create a new socket
        let socket_fd = socket(
            AddressFamily::Unix,
            SockType::Stream,
            SockFlag::empty(),
            None,
        )
        .map_err(|e| SharedError::CreateSocket { errno: e })?;

        println!("Socket file descriptor: {:?}", socket_fd.as_raw_fd());

        // connect to the socket fd.
        connect(socket_fd.as_raw_fd(), &socket_addr).map_err(|e| CliError::ConnectSocket {
            addr: socket_addr,
            errno: e,
        })?;

        Ok(Cli {
            config,
            cli: ClapCli::parse(),
            socket_fd: socket_fd,
        })
    }

    /// execute a command based on the parsed CLI arguments.
    pub fn execute(&mut self) -> Result<(), CliError> {
        match &self.cli.command {
            Some(Commands::Pull { image }) => self.pull(image.clone()),
            None => Ok(()),
        }
    }

    /// `pull`: Send a pull command to the daemon over the socket connection
    /// with the image name as the payload.
    fn pull(&mut self, image: String) -> Result<(), CliError> {
        write(self.socket_fd.as_raw_fd(), image.as_bytes()).map_err(|e| CliError::WriteSocket {
            fd: self.socket_fd.as_raw_fd(),
            errno: e,
        })?;
        Ok(())
    }
}
