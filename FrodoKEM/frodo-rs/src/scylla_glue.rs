// HAND-WRITTEN FILE

pub(crate)
fn scylla_u8_of_u16(x: & [u16]) -> & [u8] {
    unsafe {
        std::slice::from_raw_parts((&raw const *x).cast::<u8>(), x.len()*2)
    }
}

pub(crate)
fn scylla_u8_of_u32(x: & [u32]) -> & [u8] {
    unsafe {
        std::slice::from_raw_parts((&raw const *x).cast::<u8>(), x.len()*4)
    }
}

pub(crate)
fn scylla_u16_of_u32(x: & [u32]) -> & [u16] {
    unsafe {
        std::slice::from_raw_parts((&raw const *x).cast::<u16>(), x.len()*2)
    }
}

pub(crate)
fn scylla_u16_of_u8(x: & [u8]) -> & [u16] {
    unsafe {
        std::slice::from_raw_parts((&raw const *x).cast::<u16>(), x.len()/2)
    }
}

pub(crate)
fn scylla_u32_of_u16(x: & [u16]) -> & [u32] {
    unsafe {
        std::slice::from_raw_parts((&raw const *x).cast::<u32>(), x.len()/2)
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate)
fn scylla_u8_of_u16_mut(x: &mut [u16]) -> &mut [u8] {
    unsafe {
        std::slice::from_raw_parts_mut((&raw mut *x).cast::<u8>(), x.len()*2)
    }
}

pub(crate)
fn scylla_u8_of_i16_mut(x: &mut [i16]) -> &mut [u8] {
    unsafe {
        std::slice::from_raw_parts_mut((&raw mut *x).cast::<u8>(), x.len()*2)
    }
}

pub(crate)
fn scylla_u8_of_u32_mut(x: & mut [u32]) -> & mut [u8] {
    unsafe {
        std::slice::from_raw_parts_mut((&raw mut *x).cast::<u8>(), x.len()*4)
    }
}

pub(crate)
fn scylla_u16_of_u32_mut(x: & mut [u32]) -> & mut [u16] {
    unsafe {
        std::slice::from_raw_parts_mut((&raw mut *x).cast::<u16>(), x.len()*2)
    }
}

pub(crate)
fn scylla_u16_of_u8_mut(x: & mut [u8]) -> & mut [u16] {
    unsafe {
        std::slice::from_raw_parts_mut((&raw mut *x).cast::<u16>(), x.len()/2)
    }
}

pub(crate)
fn scylla_u32_of_u16_mut(x: & mut [u16]) -> & mut [u32] {
    unsafe {
        std::slice::from_raw_parts_mut((&raw mut *x).cast::<u32>(), x.len()/2)
    }
}
