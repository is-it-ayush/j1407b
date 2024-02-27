use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
#[repr(u8)]
enum MessageType {
    Request = 01,
    Response = 02,
}

#[derive(Debug, Deserialize, Serialize)]
#[repr(u8)]
enum ClientCommand {
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
struct Message<T> {
    _type: MessageType,
    command: ClientCommand,
    payload: T,
}
