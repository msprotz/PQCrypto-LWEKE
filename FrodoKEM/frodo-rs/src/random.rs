#![allow(unused_variables)]

mod ffi {
    extern "C" {
        pub(crate) fn randombytes(random_array: *mut u8, nbytes: u64) -> i32;
    }
}

pub fn randombytes(random_array: &mut[u8], nbytes: u64) -> i32 {
    unsafe {
        ffi::randombytes(random_array.as_mut_ptr(), nbytes)
    }
}

