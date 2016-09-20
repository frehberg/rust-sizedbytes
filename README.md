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
   const CONST_DATA : SizedBytes = sized_bytes!(b"0123456789"); // size 10, not nul-terminated
   // equivalant  to               sized_bytes!([ 48u8, 49u8, 50u8, 51u8, 52u8 53u8 54u8, 55u8, 56u8, 57u8 ])
   let runtime_bytes = b"0123456789";

   let mut buffer :  [u8; 2 * CONST_DATA.size ] = [0; 2 * CONST_DATA.size ];
   for (i, char_byte) in CONST_DATA.bytes.into_iter().enumerate() {
       buffer[i * 2] = *char_byte;
   }
}
```

This feature is analog to the feature in C, where the function 'sizeof' is deriving the const length of the embedded string during compile time, which can be used to initialize other buffer of specific constant size.
```C
const char CONST_DATA[] = "0123456789"; // size 11, as nul-terminated
/* equivalant to: char CONST_DATA[] = ['0','1','2','3','4','5','6','7','8','9', '\0'] */
char buffer[sizeof(CONST_DATA) * 2];
const int N = sizeof(CONST_DATA) - 1 ;

int i=0;
for (i=0; i<N; ++i)
{
    buffer[i*2] = CONST_DATA[i];
}
```
