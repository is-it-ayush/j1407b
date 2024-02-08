use nix::sys::socket::UnixAddr;
use shared::error::SharedError;
use std::os::fd::RawFd;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DaemonError {
    #[error("Shared Errror: {0}")]
    Shared(#[from] SharedError),

    #[error("Failed to set socket option on socket descriptor {fd} : {errno}")]
    SetSocketOpt {
        fd: RawFd,
        #[source]
        errno: nix::errno::Errno,
    },

    #[error("Failed to bind socket descriptor {fd} to address {addr}: {errno}")]
    BindSocket {
        fd: RawFd,
        addr: UnixAddr,
        #[source]
        errno: nix::errno::Errno,
    },

    #[error("Failed to listen on socket descriptor {fd}: {errno}")]
    ListenSocket {
        fd: RawFd,
        #[source]
        errno: nix::errno::Errno,
    },

    #[error("Failed to accept a connection from socket descriptor {fd}: {errno}")]
    AcceptSocketConnection {
        fd: RawFd,
        #[source]
        errno: nix::errno::Errno,
    },

    #[error(
        "Failed to read a connection fd {conn_fd} from socket descriptor {socket_fd}: {errno}"
    )]
    ReadSocketConnection {
        socket_fd: RawFd,
        conn_fd: RawFd,
        #[source]
        errno: nix::errno::Errno,
    },

    #[error(
        "Failed to close a connection fd {conn_fd} from socket descriptor {socket_fd}: {errno}"
    )]
    CloseSocketConnection {
        socket_fd: RawFd,
        conn_fd: RawFd,
        #[source]
        errno: nix::errno::Errno,
    },
}
