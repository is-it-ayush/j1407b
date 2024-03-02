use crate::{
    clap::{ClapCli, Commands},
    error::CliError,
};
use clap::Parser;
use nix::{
    sys::socket::{connect, socket, AddressFamily, SockFlag, SockType, UnixAddr},
    unistd::write,
};
use shared::{
    comms::{Command, Header, Type},
    error::SharedError,
};
use std::os::fd::{AsRawFd, OwnedFd};

pub struct Cli {
    pub cli: ClapCli,
    pub socket_fd: OwnedFd,
}

const DAEMON_SOCKET: &str = "/tmp/j1047b.sock";

impl Cli {
    /// create a new Cli instance
    pub fn new() -> Result<Self, CliError> {
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
        let body_bytes = rust_fr::serializer::to_bytes(&image)
            .map_err(|e| SharedError::MessageSerialize(e.to_string()))?;

        let header = Header::new(Type::Request, Command::Pull, body_bytes.len() as u64);
        let header_bytes = rust_fr::serializer::to_bytes(&header)
            .map_err(|e| SharedError::MessageSerialize(e.to_string()))?;

        //println!("header_type_size: {}", std::mem::size_of::<Header>());
        //println!("header_size: {}", header_bytes.len());
        //println!("body_size: {}", body_bytes.len());

        let message = [header_bytes, body_bytes].concat();
        write(self.socket_fd.as_raw_fd(), &message).map_err(|e| CliError::WriteSocket {
            fd: self.socket_fd.as_raw_fd(),
            errno: e,
        })?;
        Ok(())
    }
}
