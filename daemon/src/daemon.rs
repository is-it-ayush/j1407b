use crate::{
    config::{Config, CONFIG_FILE_NAME},
    error::DaemonError,
};
use shared::config::ConfigHolder;
use std::{
    io::Read,
    os::unix::net::{UnixListener, UnixStream},
    thread,
};

pub struct Daemon {
    pub config: ConfigHolder<Config>,
    pub listener: UnixListener,
}

const DAEMON_SOCKET: &str = "/tmp/j1047b.sock";

impl Daemon {
    /// Create a new Daemon instance
    pub fn new() -> Result<Self, DaemonError> {
        let config = ConfigHolder::<Config>::new(CONFIG_FILE_NAME)?;
        let listener = UnixListener::bind(DAEMON_SOCKET)
            .map_err(|err| DaemonError::CreateSocket { errno: err })?;
        Ok(Daemon {
            config,
            listener,
        })
    }

    /// Run the daemon
    pub fn run(&self) -> Result<(), DaemonError> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    thread::spawn(move || {
                        if let Err(err) = Self::handle_client(&mut stream) {
                            eprintln!("[Error] Failed to handle client: {}", err);
                        }
                    });
                }
                Err(err) => {
                    eprintln!("[Error] Failed to accept client: {}", err);
                }
            }
        }
        Ok(())
    }

    pub fn handle_client(stream: &mut UnixStream) -> Result<(), DaemonError> {
        let mut buffer = [0; 1024];
        let mut data = Vec::new();
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    data.extend_from_slice(&buffer[..n]);
                }
                Err(err) => {
                    eprintln!("[Error] Failed to read from client: {}", err);
                    break;
                }
            }
        }
        let message =
            String::from_utf8(data).map_err(|err| DaemonError::InvalidUtf8 { source: err })?;
        println!("Received message: {}", message);
        Ok(())
    }

    //pub fn get_listener() -> Result<UnixListener, DaemonError> {
    //    if fs::metadata(DAEMON_SOCKET).is_ok() {
    //        let stream = UnixStream::connect(DAEMON_SOCKET)
    //            .map_err(|err| DaemonError::ReadSocket { source: err })?;
    //    } else {
    //        UnixListener::bind(DAEMON_SOCKET)
    //            .map_err(|err| DaemonError::CreateSocket { errno: err })
    //    }
    //}
}
