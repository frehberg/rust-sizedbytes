#![feature(plugin)]
#![plugin(bytestool)]

pub struct SizedBytes {
     pub size : usize,
     pub bytes : &'static [u8],
}

// macro allows to declare byte-array and corresponding length during compile time
// eg. if second array 'buffer' depends on its length
// const SB : SizedBytes = sized_bytes!(b"hello world");
// let mut buffer :  [u8; 2 * SB.size ] = [0; 2 * SB.size ];
#[macro_export]
macro_rules! sized_bytes {
    ( $bstr:expr ) => {{
        SizedBytes {size :  byte_size_of!($bstr), bytes : $bstr, }
    }};
}


#[cfg(test)]
mod tests {
    use super::SizedBytes;

    #[test]
    fn test_sizedbytes() {
	const CONST_DATA : SizedBytes = sized_bytes!(b"0123456789");
        let runtime_bytes = b"0123456789";

        assert_eq!(CONST_DATA.size,  runtime_bytes.len()); // size of u8 array
        assert_eq!(CONST_DATA.bytes, runtime_bytes); // equal arrays
	
	let mut buffer :  [u8; 2 * CONST_DATA.size ] = [0; 2 * CONST_DATA.size ];
	for (i, char_byte) in CONST_DATA.bytes.into_iter().enumerate() {
           buffer[i * 2] = *char_byte;
        }
    }
}
