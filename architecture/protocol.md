### J1047b Message Protocol

- The protocol makes use of local unix sockets for communication.
- The protocol is a binary request-response protocol.
- Here `Message = Serialize(Header) + Serialize(Body)` where `Header = Type + Command + Length` & `Body = T`.
- Header is 16 bytes long. Serialized Header is 41 bytes long.
    - The `Type` is either a `Request` or a `Response`.
    - The `Command` is the type of command being executed. This is an enum.
    - The `Length` is the length of the body. This is a `u64`.
- The body is `Length` bytes long. This information is stored in the header.
- The header is first read from the connection. It is deserialized and the body length is extracted.
This body length is then used to read the body from the connection which is then deserialized into a `T` type.

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

- Important Structures: `./shared/src/protocol.rs`.
```rust
pub enum Type {
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
pub struct Header {
    pub _type: Type,      // 1 byte
    pub command: Command, // 1 byte
    pub length: u64,      // 8 bytes
}
pub struct Protocol;
```
