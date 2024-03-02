
use crate::{
    comms::{Command, Header, Type},
    error::SharedError,
};
use nix::unistd::read;
use std::os::fd::RawFd;

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
