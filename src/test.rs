mod constants;
use utils::byte_utils::{bytes_to_partial_struct, bytes_to_struct, create_empty};

mod utils;

#[derive(Debug)]
#[repr(C, packed(2))]
struct X {
    a: u32,
    b: i16,
}

fn main() {
    let mut x: X = create_empty();

    let buf = [1, 0, 0, 0];

    bytes_to_partial_struct(&mut x, &buf);

    println!("{x:?}");

    x = bytes_to_struct(&buf);

    let z = u16::from_be_bytes([210,146]);

    println!("{}", z);

    println!("{x:?}");
}
