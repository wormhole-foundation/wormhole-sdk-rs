## Cool Things

- configurable keccak backend

## Features

- `serde` - adds serde support to VAA types. Unlike previous iterations of the 
SDK, this support is for JSON only. We do not use serde for binary encoding/
decoding
- `anchor` - enables the anchor-lang keccak256 backend


### Solana development

Solana devs should turn off `serde` and on `anchor`.

For Solana applications use
- `$ cargo add wormhole-vaas --features anchor`