//! This module contains the message types and the message struct.
//! These define the messages that can be sent between the cli
//! and the daemon. The protoocol is a simple request/response
//! protocol over local unix sockets. The message types are serialized
//! and deserialized using `rust-fr`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[repr(u8)]
pub enum Type {
    Request = 01,
    Response = 02,
}

#[derive(Debug, Deserialize, Serialize)]
#[repr(u8)]
pub enum Command {
    Pull = 01,
    Run = 02,
    Stop = 03,
    Rm = 04,
    Ps = 05,
    Images = 06,
    Logs = 07,
    Exec = 08,
    Tag = 09,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Header {
    pub _type: Type,      // 1 byte
    pub command: Command, // 1 byte
    pub length: u64,      // 8 bytes
}

impl Header {
    /// Create a new header
    pub fn new(_type: Type, command: Command, length: u64) -> Self {
        Header {
            _type,
            command,
            length,
        }
    }
}
