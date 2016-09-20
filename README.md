# rust-sizedbytes
Rust macro annotating the constant size value to a constant bytestring

The feature allows to embed a constant byte string in source code and use its
byte-size during compile time to declare other buffer of specific preset 
size.

Example Cargo.toml
```init

[dependencies]
bytestool = "0.2.0"
sizedbytes = "0.1.0"
```

Example main.rs
```rust
#![feature(plugin)]
#![plugin(bytestool)]
#[macro_use(sized_bytes)]

extern crate sizedbytes;

use sizedbytes::SizedBytes;

fn main() {
   const CONST_DATA : SizedBytes = sized_bytes!(b"0123456789");
   let runtime_bytes = b"0123456789";

   let mut buffer :  [u8; 2 * CONST_DATA.size ] = [0; 2 * CONST_DATA.size ];
   for (i, char_byte) in CONST_DATA.bytes.into_iter().enumerate() {
       buffer[i * 2] = *char_byte;
   }
}
```
