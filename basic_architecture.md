# THE BELOW IS NOT YET FULLY IMPLEMENTED

Preliminary API documented in `lib.rs`. Feel free to add any wanted features or change the architecture. Also, I know that this documentation is barebones, sorry.

## Issues
- Untested
- Blind relay (forwards to everyone)
- No handling of messages

## Goals & Guidelines

`server.rs` and `client.rs` are main replacements, please do not implement functions in them.

Keys are generated using `key_exchange.rs`, and relayed using `relay.rs`.

`relay.rs` forwards messages and public keys to the relevant parties.

`network_interface.rs` sends data to other client runners where it will be parsed and handled.

`crypto.rs` handles encrypting and decrypting plaintext.