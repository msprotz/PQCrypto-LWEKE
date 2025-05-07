#![allow(unused_variables)]

mod ffi {
    extern "C" {
        pub(crate) fn AES128_load_schedule(key: *const u8, schedule: *const u8);

        pub(crate) fn AES128_ECB_enc_sch(plaintext: *const u8, plaintext_len: usize, schedule: *const u8, ciphertext: *mut u8);

        pub(crate) fn AES128_free_schedule(schedule: *const u8);

        pub(crate) fn AES256_load_schedule(key: *const u8, schedule: *mut u8);

        pub(crate) fn AES256_ECB_enc_sch(plaintext: *const u8, plaintext_len: usize, schedule: *const u8, ciphertext:*mut u8);

        pub(crate) fn AES256_free_schedule(schedule: *const u8);
    }
}

pub(crate) fn AES128_load_schedule(key: &[u8], schedule: &mut [u8]) {
    unsafe {
        ffi::AES128_load_schedule(key.as_ptr(), schedule.as_mut_ptr())
    }
}

pub(crate) fn AES128_ECB_enc_sch(plaintext: &[u8], plaintext_len: usize, schedule: &[u8], ciphertext: &mut[u8]) {
    unsafe {
        ffi::AES128_ECB_enc_sch(plaintext.as_ptr(), plaintext_len, schedule.as_ptr(), ciphertext.as_mut_ptr())
    }
}

pub(crate) fn AES128_free_schedule(schedule: &[u8]) {
}

pub(crate) fn AES256_load_schedule(key: &[u8], schedule: &mut[u8]) {
    unsafe {
        ffi::AES256_load_schedule(key.as_ptr(), schedule.as_mut_ptr())
    }
}

pub(crate) fn AES256_ECB_enc_sch(plaintext: &[u8], plaintext_len: usize, schedule: &[u8], ciphertext:&mut[u8]) {
    unsafe {
        ffi::AES256_ECB_enc_sch(plaintext.as_ptr(), plaintext_len, schedule.as_ptr(), ciphertext.as_mut_ptr())
    }
}

pub(crate) fn AES256_free_schedule(schedule:&[u8]) {
}
