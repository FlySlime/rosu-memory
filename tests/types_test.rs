use std::fs::File;
use std::io::Read;
use rand::prelude::*;
use paste::paste;

use rosu_memory::memory::process::ProcessTraits;
use rosu_memory::memory::error::*;

macro_rules! prim_read_test {
    ($t: ident) => {
        paste! {
            #[test]
            fn [<test_ $t>]() {
                let mut rng = rand::thread_rng();
                let num: $t = rng.gen();

                let buff: Vec<u8> = num.to_le_bytes().to_vec();
                let p = FakeProccess {
                    buff
                };

                let tmp = p.[<read_ $t>](0).unwrap();
                assert_eq!(tmp, num);
            }
            
        }
    }
}

macro_rules! prim_read_array_test {
    ($t: ident) => {
        paste! {
            #[test]
            fn [<test_ $t _array>]() {
                // Constructing fake array identical how arrays 
                // stored inside osu!
                // Type for this example is i32
                // | 00 00 00 00 | 0C 00 00 00 | 00 00 00 00 |
                // ^ [1]         ^ [2]         ^ [1]
                // | 00 00 00 01 | 00 00 00 00 |
                // ^ [3]         ^ [4]
                //
                // [1] - Bytes we don't care about
                // [2] - Pointer to our data, since we 
                // are inside test then it's
                // just a index to our test array
                // [3] - Size of array
                // [4] - Array of our desired type
                // Result of this example gonna be -> [0; 1]
                let mut rng = rand::thread_rng();

                let mut synthetic_buff = Vec::new();
                let items_start_idx: i32 = 8;
                let length: i32 = rng.gen_range(0..1024);
                let items: Vec<$t> = (0..length)
                    .map(|_| {
                        rand::random::<$t>()
                    }).collect();

                synthetic_buff.extend(
                    [0u8; 4]
                ); // Bytes we don't care about
                synthetic_buff.extend(
                    items_start_idx.to_le_bytes()
                ); // Index to array data
                synthetic_buff.extend(
                    [0u8; 4]
                ); // Bytes we don't care about
                synthetic_buff.extend(
                    length.to_le_bytes()
                ); // Size of array

                // Filling our synthetic array with random generated
                // numbers
                for item in &items {
                    synthetic_buff.extend(item.to_le_bytes())
                }

                let p = FakeProccess {
                    buff: synthetic_buff
                };

                let mut fake_output_buff = Vec::new();

                p.[<read_ $t _array>](0, &mut fake_output_buff)
                    .unwrap();

                assert_eq!(fake_output_buff.len(), length as usize);
                assert_eq!(fake_output_buff, items);
            }
        }
    }
}

pub struct FakeProccess {
    buff: Vec<u8>,
}

impl ProcessTraits for FakeProccess {
    fn initialize(_proc_name: &str) -> Result<Self, ProcessError> {
        todo!()
    }

    fn find_process(_proc_name: &str) -> Result<Self, ProcessError> {
        todo!()
    }

    fn read_regions(self) -> Result<Self, ProcessError> {
        todo!()
    }

    fn read_signature(
        &self, 
        _sign: &rosu_memory::memory::signature::Signature
    ) -> Result<usize, ProcessError> {
        todo!()
    }

    fn read(
        &self, 
        addr: usize, 
        len: usize, 
        buff: &mut [u8]
    ) -> Result<(), ProcessError> {
        // Addr - starting index
        // self.buff.set_position(addr as u64);
        //self.buff.read(buff);

        let mut slice = &self.buff[addr..addr+len];
        let _ = slice.read(buff);

        Ok(())
    }
}

#[test]
fn test_uleb() {
    let mut file = File::open("./tests/files/test_uleb")
        .unwrap();

    let mut buff = Vec::new();

    file.read_to_end(&mut buff).unwrap();

    let p = FakeProccess {
        buff
    };
    
    // Skipping that one 0x0b byte
    assert_eq!(p.read_uleb128(1).unwrap(), 4);
}

#[test]
fn test_string() {
    // TODO rewrite this test by generating random string

    let mut buff: Vec<u8> = vec![0x0, 0x0, 0x0, 0x0]; // Random 4 bytes

    let s = 0x08u32.to_le_bytes();
    buff.extend_from_slice(&s);

    let test_string_bytes: Vec<u8> = vec![
        83,
        0,
        101,
        0,
        114,
        0,
        101,
        0,
        110,
        0,
        105,
        0,
        116,
        0,
        121,
        0,
    ];

    buff.extend_from_slice(&test_string_bytes);

    let p = FakeProccess {
        buff
    };

    let len = p.read_u32(0x4).unwrap();
    assert_eq!(len, 8);

    let s = p.read_string(0).unwrap();
    assert_eq!(s, "Serenity");
}

prim_read_test!(i8);
prim_read_test!(i16);
prim_read_test!(i32);
prim_read_test!(i64);
prim_read_test!(i128);

prim_read_test!(u8);
prim_read_test!(u16);
prim_read_test!(u32);
prim_read_test!(u64);
prim_read_test!(u128);

prim_read_test!(f32);
prim_read_test!(f64);

prim_read_array_test!(i8);
prim_read_array_test!(i16);
prim_read_array_test!(i32);
prim_read_array_test!(i64);
prim_read_array_test!(i128);

prim_read_array_test!(u8);
prim_read_array_test!(u16);
prim_read_array_test!(u32);
prim_read_array_test!(u64);
prim_read_array_test!(u128);

prim_read_array_test!(f32);
prim_read_array_test!(f64);
