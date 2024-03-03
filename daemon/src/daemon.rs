use crate::{
    config::{Config, CONFIG_FILE_NAME},
    error::DaemonError,
};
use nix::{
    sys::socket::{
        accept, bind, listen, setsockopt, socket, sockopt, AddressFamily, SockFlag, SockType,
        UnixAddr,
    },
    unistd::close,
};
use shared::{
    config::ConfigHolder,
    error::SharedError,
    protocol::{self, Header, Protocol},
    requests::PullRequest,
};
use std::os::fd::{AsRawFd, OwnedFd};

pub struct Daemon {
    pub config: ConfigHolder<Config>,
    pub socket_fd: OwnedFd,
}

const DAEMON_SOCKET: &str = "/tmp/j1047b.sock";

impl Daemon {
    /// Create a new Daemon instance
    pub fn new() -> Result<Self, DaemonError> {
        let config = ConfigHolder::<Config>::new(CONFIG_FILE_NAME)?;

        // create a new socket address (sockaddr_un)
        let socket_addr = UnixAddr::new(DAEMON_SOCKET.as_bytes())
            .map_err(|e| SharedError::CreateUnixAddr { errno: e })?;

        println!("[Info] Socket address: {:?}", socket_addr);

        // create a new socket
        let socket_fd = socket(
            AddressFamily::Unix,
            SockType::Stream,
            SockFlag::empty(),
            None,
        )
        .map_err(|e| SharedError::CreateSocket { errno: e })?;

        println!("[INFO] Socket file descriptor: {:?}", socket_fd.as_raw_fd());

        let _ = std::fs::remove_file(DAEMON_SOCKET);
        // bind the socket to the address
        bind(socket_fd.as_raw_fd(), &socket_addr).map_err(|e| DaemonError::BindSocket {
            fd: socket_fd.as_raw_fd(),
            addr: socket_addr,
            errno: e,
        })?;

        println!("[INFO] Socket bound to address: {:?}", socket_addr);

        // set SO_REUSEADDR option
        setsockopt(&socket_fd, sockopt::ReuseAddr, &true).map_err(|e| {
            DaemonError::SetSocketOpt {
                fd: socket_fd.as_raw_fd(),
                errno: e,
            }
        })?;

        // listen for incoming connections
        listen(&socket_fd, 5).map_err(|e| DaemonError::ListenSocket {
            fd: socket_fd.as_raw_fd(),
            errno: e,
        })?;

        println!("[INFO] Listening for incoming connections");

        Ok(Daemon {
            config: config,
            socket_fd: socket_fd,
        })
    }

    /// Run the daemon
    pub fn run(&self) -> Result<(), DaemonError> {
        loop {
            // accept a new connection (connection queue is 5)
            let conn_fd = accept(self.socket_fd.as_raw_fd()).map_err(|e| {
                DaemonError::AcceptSocketConnection {
                    fd: self.socket_fd.as_raw_fd(),
                    errno: e,
                }
            })?;
            println!("[INFO] Accepted new connection: {:?}", conn_fd);

            // read the header
            let header = Protocol::read_header(self.socket_fd.as_raw_fd(), conn_fd)?;
            if header._type != protocol::Type::Request {
                println!("[ERROR] Invalid message type: {:?}", header._type);
                continue;
            }
            self.execute_command(header, conn_fd)?;

            // close the connection
            close(conn_fd).map_err(|e| DaemonError::CloseSocketConnection {
                socket_fd: self.socket_fd.as_raw_fd(),
                conn_fd: conn_fd,
                errno: e,
            })?;
        }
    }

    /// Execute a command based on the parsed CLI arguments.
    pub fn execute_command(&self, header: Header, conn_fd: i32) -> Result<(), DaemonError> {
        match header.command {
            protocol::Command::Pull => self.pull(header, conn_fd),
            _ => Ok(()),
        }
    }

    /// The `pull` command
    pub fn pull(&self, header: Header, conn_fd: i32) -> Result<(), DaemonError> {
        // read the body
        let body =
            Protocol::read_body::<PullRequest>(self.socket_fd.as_raw_fd(), conn_fd, header.length)?;

        Ok(())
    }
}

impl Drop for Daemon {
    fn drop(&mut self) {
        // close the socket
        close(self.socket_fd.as_raw_fd()).unwrap();
    }
}
