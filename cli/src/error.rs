use nix::sys::socket::UnixAddr;
use shared::error::SharedError;
use std::os::fd::RawFd;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Shared Errror: {0}")]
    Shared(#[from] SharedError),

    #[error("Failed to connect to socket at {addr}: {errno}")]
    ConnectSocket {
        addr: UnixAddr,
        #[source]
        errno: nix::errno::Errno,
    },

    #[error("Failed to write to socket descriptor {fd}: {errno}")]
    WriteSocket {
        fd: RawFd,
        #[source]
        errno: nix::errno::Errno,
    },
}
