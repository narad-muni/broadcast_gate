use std::{mem::{self, MaybeUninit}, ptr};

#[derive(Debug)]
struct St {
    vec: Vec<u8>,
    str: String,
}

impl St {
    fn new() -> Self {
        Self {
            vec: vec![1,4,6,2,7],
            str: "Hello World".to_string(),
        }
    }
}

impl Drop for St {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

pub fn bytes_to_struct_bincode<T>(buff: &[u8]) -> T {
    let buff_ptr = buff.as_ptr() as *const T;

    unsafe { std::ptr::read(buff_ptr) }
}

pub fn struct_to_bytes_bincode<T>(src: T, dst: &mut [u8]) -> usize {
    let struct_ptr = &src as *const T as *const u8;
    // Use `ManuallyDrop` to take ownership of `input` without dropping it
    mem::forget(src);

    let mut size = size_of::<T>();
    
    size = size.min(dst.len());

    unsafe {
        // Similar to memcpy
        ptr::copy_nonoverlapping(struct_ptr, dst.as_mut_ptr(), size);
    };

    size
}
fn main() {
    let mut buf: [u8; 1024] = [0;1024];

    {
        let st = St::new();
        
        struct_to_bytes_bincode(st, &mut buf);
    }

    let st: St = bytes_to_struct_bincode(&buf);

    buf = [0;1024];

    println!("{:?}", st);
}
