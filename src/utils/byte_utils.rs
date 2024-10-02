use std::{mem, ptr};

pub fn bytes_to_struct<T>(s: &[u8]) -> T {
    unsafe {
        let src = s.as_ptr() as *const T;

        std::ptr::read(src)
    }
}

pub fn struct_to_bytes<T>(s: &T, buffer: &mut [u8]) {
    // Get a raw pointer to the struct and cast it to a byte pointer
    let struct_ptr: *const u8 = &s as *const _ as *const u8;

    // Use unsafe to copy the struct memory into the byte array
    unsafe {
        ptr::copy_nonoverlapping(struct_ptr, buffer.as_mut_ptr(), mem::size_of::<T>());
    }
}

pub fn create_empty<T>() -> T {
    unsafe { mem::zeroed() }
}

pub fn bytes_to_partial_struct<T>(s: &mut T, buffer: &[u8]) {
    unsafe {
        // Get unsafe mutable raw pointer
        let struct_ptr = s as *mut T as *mut u8;

        // Similar to memcpy
        ptr::copy_nonoverlapping(buffer.as_ptr(), struct_ptr, buffer.len());
    };
}

pub fn bytes_to_struct_mut<T>(buf: &mut [u8]) -> &mut T {
    unsafe { &mut *(buf.as_mut_ptr() as *mut T) }
}
