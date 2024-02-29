### J1047b Message Protocol

- The protocol is binary and is serialized & deserialized by the `rust-fr`
library.
- It's a simple request-response protocol.
- It is defined in the `comms` module in the `shared` crate.
- The serialized header is concatenated with the serilized body and sent as a
single message. It's the daemon's job to separate the header from the body & deserialize
it. Then read the body with the given header information & perform the required
action.

```bnf
<message> ::= <header> <body>
<header> ::= <type> <command> <length>
<type> ::= 1 | 2
<command> ::= 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
<length> ::= <int>+
<body> ::= <string>
<string> ::= <char>+
<char> ::= any printable ASCII character
<int> ::= any integer value
```

### Protocol Structures

- Within `./shared/src/comms.rs`.
- Header structure:
```rust
struct Header {
    _type: Type,
    command: Command,
    length: u64,
}
pub enum MessageType {
    Request = 01,
    Response = 02,
}
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
```
