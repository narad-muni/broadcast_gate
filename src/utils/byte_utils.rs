use std::{
    any::type_name,
    mem::{self, ManuallyDrop, MaybeUninit},
    ptr,
};

use crate::constants::BUF_SIZE;

pub fn bytes_to_struct<T>(s: &[u8]) -> T {
    let src = s.as_ptr() as *const T;

    unsafe {
        std::ptr::read(src)
    }
}

pub fn struct_to_bytes<T>(s: &T, buffer: &mut [u8]) -> usize {
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

// Safe because of assert
pub fn cast<T, F>(input: F) -> T {
    // Ensure that T and U have the same size to avoid undefined behavior
    assert_eq!(
        std::mem::size_of::<T>(),
        std::mem::size_of::<F>(),
        "Cannot cast from smaller struct {} to larger struct {}",
        type_name::<F>(),
        type_name::<T>()
    );

    // Use `ManuallyDrop` to take ownership of `input` without dropping it
    let input = ManuallyDrop::new(input);

    // Use `ptr::read` to reinterpret the bytes of `input` as type `U`
    unsafe { ptr::read(&*input as *const F as *const T) }
}

// Allows to cast from smaller structure to larger structure
// Accessing field outside of F's size will cause segfault
pub unsafe fn cast_unsafe<F, T>(input: F) -> T {
    // Use `ManuallyDrop` to take ownership of `input` without dropping it
    let input = ManuallyDrop::new(input);

    // Use `ptr::read` to reinterpret the bytes of `input` as type `U`
    ptr::read(&*input as *const F as *const T)
}

pub fn uninit_to_buf(src: &[MaybeUninit<u8>; BUF_SIZE]) -> [u8; BUF_SIZE] {
    unsafe { std::mem::transmute::<[MaybeUninit<u8>; BUF_SIZE], [u8; BUF_SIZE]>(*src) }
}

pub fn create_empty<T>() -> T {
    unsafe { mem::zeroed() }
}

pub fn bytes_to_partial_struct<T>(s: &mut T, buffer: &[u8]) {
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

pub fn bytes_to_struct_mut<T>(buf: &mut [u8]) -> &mut T {
    unsafe { &mut *(buf.as_mut_ptr() as *mut T) }
}
