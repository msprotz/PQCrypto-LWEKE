#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(unreachable_patterns)]
#![allow(unused_mut)]

pub fn clear_bytes(mem: &mut [u8], n: usize)
{
  let v: (&mut [u8], &mut [u8]) = mem.split_at_mut(0usize);
  for i in 0usize..n { v.1[i] = 0u8 }
}

pub fn ct_select(r: &mut [u8], a: &[u8], b: &[u8], len: usize, selector: i8)
{ for i in 0usize..len { r[i] = ! selector as u8 & a[i] | selector as u8 & b[i] } }

pub fn ct_verify(a: &[u16], b: &[u16], len: usize) -> i8
{
  let mut r: u16 = 0u16;
  for i in 0usize..len { r |= a[i] ^ b[i] };
  r =
      ((0i16.wrapping_sub((r as i32).wrapping_shr(1u32) as i16)
      |
      0i16.wrapping_sub((r as u32 & 1u32) as i16))
      as
      i32).wrapping_shr(8u64.wrapping_mul(2u64).wrapping_sub(1u64) as u32)
      as
      u16;
  return r as i8
}

pub fn frodo_pack(out: &mut [u8], outlen: usize, r#in: &[u16], inlen: usize, lsb: u8)
{
  (out[0usize..outlen]).copy_from_slice(&vec![0u8; outlen].into_boxed_slice());
  let mut i: usize = 0usize;
  let mut j: usize = 0usize;
  let mut w: u16 = 0u16;
  let mut bits: u8 = 0u8;
  while
  i < outlen && (j < inlen || j == inlen && bits as u32 > 0u32)
  {
    let mut b: u8 = 0u8;
    while
    (b as u32) < 8u32
    {
      let nbits: i32 =
          if 8u32.wrapping_sub(b as u32) < bits as u32
          { 8u32.wrapping_sub(b as u32) as u8 }
          else
          { bits }
          as
          i32;
      let mask: u16 = 1i32.wrapping_shl(nbits as u32).wrapping_sub(1i32) as u16;
      let t: u8 =
          ((w as i32).wrapping_shr((bits as u32).wrapping_sub(nbits as u32)) as u32 & mask as u32)
          as
          u8;
      out[i] =
          (out[i] as u32).wrapping_add(
            (t as i32).wrapping_shl(8u32.wrapping_sub(b as u32).wrapping_sub(nbits as u32)) as u32
          )
          as
          u8;
      b = (b as u32).wrapping_add(nbits as u32) as u8;
      bits = (bits as u32).wrapping_sub(nbits as u32) as u8;
      w = (w as u32 & ! (mask as i32).wrapping_shl(bits as u32) as u32) as u16;
      if bits as u32 == 0u32
      {
        if j < inlen
        {
          w = r#in[j];
          bits = lsb;
          j = j.wrapping_add(1usize)
        }
        else
        { break }
      }
    };
    if b as u32 == 8u32 { i = i.wrapping_add(1usize) }
  }
}

pub fn frodo_unpack(out: &mut [u16], outlen: usize, r#in: &[u8], inlen: usize, lsb: u8)
{
  (out[0usize..outlen]).copy_from_slice(&vec![0u16; outlen].into_boxed_slice());
  let mut i: usize = 0usize;
  let mut j: usize = 0usize;
  let mut w: u8 = 0u8;
  let mut bits: u8 = 0u8;
  while
  i < outlen && (j < inlen || j == inlen && bits as u32 > 0u32)
  {
    let mut b: u8 = 0u8;
    while
    b < lsb
    {
      let nbits: i32 = if lsb.wrapping_sub(b) < bits { lsb.wrapping_sub(b) } else { bits } as i32;
      let mask: u16 = 1i32.wrapping_shl(nbits as u32).wrapping_sub(1i32) as u16;
      let t: u8 =
          ((w as i32).wrapping_shr((bits as u32).wrapping_sub(nbits as u32)) as u32 & mask as u32)
          as
          u8;
      out[i] =
          (out[i] as u32).wrapping_add(
            (t as i32).wrapping_shl((lsb.wrapping_sub(b) as u32).wrapping_sub(nbits as u32)) as u32
          )
          as
          u16;
      b = (b as u32).wrapping_add(nbits as u32) as u8;
      bits = (bits as u32).wrapping_sub(nbits as u32) as u8;
      w = (w as u32 & ! (mask as i32).wrapping_shl(bits as u32) as u32) as u8;
      if bits as u32 == 0u32
      {
        if j < inlen
        {
          w = r#in[j];
          bits = 8u8;
          j = j.wrapping_add(1usize)
        }
        else
        { break }
      }
    };
    if b == lsb { i = i.wrapping_add(1usize) }
  }
}
