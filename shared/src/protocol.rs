//! This module deals with the protocol used to communicate between the client and the server. The
//! serialization and deserialization of messages is done using the `rust-fr` crate.
//!
//! - The protocol makes use of local unix sockets for communication.
//! - The protocol is a binary request-response protocol.
//! - Here `Message = Serialize(Header) + Serialize(Body)` where `Header = Type + Command + Length` & `Body = T`.
//! - Header is 16 bytes long. Serialized Header is 41 bytes long.
//!     - The `Type` is either a `Request` or a `Response`.
//!     - The `Command` is the type of command being executed. This is an enum.
//!     - The `Length` is the length of the body. This is a `u64`.
//! - The body is `Length` bytes long. This information is stored in the header.
//! - The header is first read from the connection. It is deserialized and the body length is extracted.
//! This body length is then used to read the body from the connection which is then deserialized into a `T` type.
//!
//! - The grammer for the protocol is as follows:
//! ```bnf
//! <message> ::= <header> <body>
//! <header> ::= <type> <command> <length>
//! <type> ::= 1 | 2
//! <command> ::= 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
//! <length> ::= <int>+
//! <body> ::= <string>
//! <string> ::= <char>+
//! <char> ::= any printable ASCII character
//! <int> ::= any integer value
//! ```
//!
//! This is also defined under [architecture/protocol.md](../../architecture/protocol.md) file. The
//! communication is shown in the diagram at [architecture/communication.png](../../architecture/communication.png).

use crate::error::SharedError;
use nix::unistd::read;
use serde::{Deserialize, Serialize};
use std::os::fd::RawFd;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[repr(u8)]
pub enum Type {
    Request = 01,
    Response = 02,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

pub struct Protocol;

impl Protocol {
    /// Read a header from a connection.
    /// - The serialized header is only `41` bytes long.
    /// - The header itself is 16 bytes long.
    /// - It contains the type, command, and body size.
    pub fn read_header(socket_fd: RawFd, conn_fd: i32) -> Result<Header, SharedError> {
        let mut header_buffer = [0u8; 41];
        loop {
            let bytes_read = read(conn_fd, &mut header_buffer).map_err(|e| {
                SharedError::ReadSocketConnection {
                    socket_fd: socket_fd,
                    conn_fd: conn_fd,
                    errno: e,
                }
            })?;
            if bytes_read == 41 {
                break;
            }
        }
        rust_fr::deserializer::from_bytes::<Header>(&header_buffer)
            .map_err(|e| SharedError::MessageDeserialize(e.to_string()))
    }

    /// Read a body from a connection.
    /// - The body is `message_length` bytes long. This information is stored in the header.
    /// - The body is deserialized into a `T` type (which implements `serde::de::Deserialize`).
    pub fn read_body<T: serde::de::DeserializeOwned>(
        socket_fd: RawFd,
        conn_fd: i32,
        message_length: u64,
    ) -> Result<T, SharedError> {
        let mut body_buffer = vec![0u8; message_length as usize];
        loop {
            let bytes_read =
                read(conn_fd, &mut body_buffer).map_err(|e| SharedError::ReadSocketConnection {
                    socket_fd: socket_fd,
                    conn_fd: conn_fd,
                    errno: e,
                })?;
            if bytes_read == 0 {
                break;
            }
        }
        rust_fr::deserializer::from_bytes::<T>(&body_buffer)
            .map_err(|e| SharedError::MessageDeserialize(e.to_string()))
    }

    /// Write a header for a request/response.
    pub fn write_header(
        _type: Type,
        command: Command,
        body_size: u64,
    ) -> Result<Vec<u8>, SharedError> {
        let header = Header::new(_type, command, body_size);
        let header_bytes = rust_fr::serializer::to_bytes(&header)
            .map_err(|e| SharedError::MessageSerialize(e.to_string()))?;
        Ok(header_bytes)
    }

    /// Write a body for a request/response.
    pub fn write_body<T: serde::Serialize>(body: T) -> Result<Vec<u8>, SharedError> {
        let body_bytes = rust_fr::serializer::to_bytes(&body)
            .map_err(|e| SharedError::MessageSerialize(e.to_string()))?;
        Ok(body_bytes)
    }

    /// Write a message from a header and a body.
    pub fn write_message(header: Vec<u8>, body: Vec<u8>) -> Vec<u8> {
        [header, body].concat()
    }
}
