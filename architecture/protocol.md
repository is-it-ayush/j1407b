### J1047b Protocols

```
<message> ::= <type> <command> <payload>
<type> ::= REQ | RES
<command> ::= 1 | 2 | 3 | 4 | 5 ...
<payload> = <string>*
<string> ::= <char>
<char> ::= any printable ASCII character
```

### Protocol Examples

- REQ: REQ 3 hello
- RES: RES 3 world
