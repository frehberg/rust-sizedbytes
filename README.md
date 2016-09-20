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

This feature is analog to the feature in C, where the function 'sizeof' is deriving the const length of the embedded string during compile time, which can be used to initialize other buffer of specific constant size.
```C
char CONST_DATA[] = "0123456789";

char buffer[sizeof(CONST_DATA) * 2];

int i=0;
for (i=0; i<sizeof(CONST_DATA); ++i)
{
    buffer[i*2] = CONST_DATA[i];
}
```
