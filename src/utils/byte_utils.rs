use std::{
    mem::{self, MaybeUninit},
    ptr,
};

use crate::constants::BUF_SIZE;

pub fn struct_to_bytes<T: Copy>(s: &T, buffer: &mut [u8]) -> usize {
    let mut size = std::mem::size_of::<T>();

    // Ensure the buffer is large enough
    if buffer.len() < size {
        size = buffer.len();
    }

    // Get a pointer to the value
    let ptr = s as *const T as *const u8;

    unsafe {
        // Copy the bytes into the buffer
        std::ptr::copy_nonoverlapping(ptr, buffer.as_mut_ptr(), size);
    }

    size
}

pub fn struct_to_bytes_heap<T>(src: T, dst: &mut [u8]) -> usize {
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

pub fn bytes_to_struct<T>(buff: &[u8]) -> T {
    let buff_ptr = buff.as_ptr() as *const T;

    unsafe { std::ptr::read(buff_ptr) }
}

pub fn uninit_to_buf(src: &[MaybeUninit<u8>; BUF_SIZE]) -> [u8; BUF_SIZE] {
    unsafe { std::mem::transmute::<[MaybeUninit<u8>; BUF_SIZE], [u8; BUF_SIZE]>(*src) }
}

pub fn create_empty<T>() -> T {
    unsafe { mem::zeroed() }
}

pub fn create_uninit<T>() -> T {
    unsafe { MaybeUninit::uninit().assume_init() }
}

pub fn bytes_to_partial_struct<T: Copy>(s: &mut T, buffer: &[u8]) {
    // Get unsafe mutable raw pointer
    let struct_ptr = s as *mut T as *mut u8;

    // Copy lowest size
    // This is to prevent overflow
    let mut min_size = mem::size_of::<T>();

    if min_size > buffer.len() {
        min_size = buffer.len();
    }

    unsafe {
        // Similar to memcpy
        ptr::copy_nonoverlapping(buffer.as_ptr(), struct_ptr, min_size);
    };
}

pub fn bytes_to_struct_mut<T: Copy>(buf: &mut [u8]) -> &mut T {
    unsafe { &mut *(buf.as_mut_ptr() as *mut T) }
}
