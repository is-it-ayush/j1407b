use crate::{
    config::{Config, CONFIG_FILE_NAME},
    error::DaemonError,
};
use nix::{
    sys::socket::{
        accept, bind, listen, setsockopt, socket, sockopt, AddressFamily, SockFlag, SockType,
        UnixAddr,
    },
    unistd::{close, read},
};
use shared::{config::ConfigHolder, error::SharedError};
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

        let _ = std::fs::remove_file(DAEMON_SOCKET);
        // bind the socket to the address
        bind(socket_fd.as_raw_fd(), &socket_addr).map_err(|e| DaemonError::BindSocket {
            fd: socket_fd.as_raw_fd(),
            addr: socket_addr,
            errno: e,
        })?;

        println!("Socket bound to address: {:?}", socket_addr);

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

        println!("Listening for incoming connections");

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

            println!("Accepted new connection: {:?}", conn_fd);

            let mut buffer = [0; 512]; // 1024 bytes buffer

            loop {
                // read from the connection
                let bytes_read =
                    read(conn_fd, &mut buffer).map_err(|e| DaemonError::ReadSocketConnection {
                        socket_fd: self.socket_fd.as_raw_fd(),
                        conn_fd: conn_fd,
                        errno: e,
                    })?;

                println!("Read {} bytes", bytes_read);

                if bytes_read == 0 {
                    break;
                }

                let message = std::str::from_utf8(&buffer[..bytes_read]).unwrap();
                println!("Received message: {}", message);
            }

            // close the connection
            close(conn_fd).map_err(|e| DaemonError::CloseSocketConnection {
                socket_fd: self.socket_fd.as_raw_fd(),
                conn_fd: conn_fd,
                errno: e,
            })?;
        }
    }
}

impl Drop for Daemon {
    fn drop(&mut self) {
        // close the socket
        close(self.socket_fd.as_raw_fd()).unwrap();
    }
}
