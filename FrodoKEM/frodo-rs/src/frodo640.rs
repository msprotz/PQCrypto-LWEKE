#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(unreachable_patterns)]
#![allow(unused_mut)]

pub const CDF_TABLE: [u16; 13] =
    [4643u16, 13363u16, 20579u16, 25843u16, 29227u16, 31145u16, 32103u16, 32525u16, 32689u16,
        32745u16, 32762u16, 32766u16, 32767u16];

pub const CDF_TABLE_LEN: u16 = 13u16;

pub fn crypto_kem_dec_Frodo640(ss: &mut [u8], ct: &[u8], sk: &[u8]) -> i32
{
  let mut B: [u16; 5120] = [0u16; 5120usize];
  let mut Bp: [u16; 5120] = [0u16; 5120usize];
  let mut W: [u16; 64] = [0u16; 64usize];
  let mut C: [u16; 64] = [0u16; 64usize];
  let mut CC: [u16; 64] = [0u16; 64usize];
  let mut BBp: [u16; 5120] = [0u16; 5120usize];
  let mut Sp: [u16; 10304] = [0u16; 10304usize];
  let ct_c1: (&[u8], &[u8]) = ct.split_at(0usize);
  let ct_c2: (&[u8], &[u8]) =
      (ct_c1.1).split_at(
        (15u32.wrapping_mul(640u32).wrapping_mul(8u32) as usize).wrapping_div(8usize)
      );
  let salt: (&[u8], &[u8]) =
      (ct_c2.1).split_at(
        9752u32.wrapping_sub(2u32.wrapping_mul(16u32)) as usize
        -
        (15u32.wrapping_mul(640u32).wrapping_mul(8u32) as usize).wrapping_div(8usize)
      );
  let sk_s: (&[u8], &[u8]) = sk.split_at(0usize);
  let sk_pk: (&[u8], &[u8]) = (sk_s.1).split_at(16usize);
  let sk_S: &[u16] =
      crate::scylla_glue::scylla_u16_of_u8(&sk[16u32.wrapping_add(9616u32) as usize..]);
  let mut S: [u16; 5120] = [0u16; 5120usize];
  let sk_pkh: (&[u8], &[u8]) =
      sk.split_at(
        16u32.wrapping_add(9616u32).wrapping_add(2u32.wrapping_mul(640u32).wrapping_mul(8u32))
        as
        usize
      );
  let pk_seedA: (&[u8], &[u8]) = (sk_pk.1).split_at(0usize);
  let pk_b: (&[u8], &[u8]) = (pk_seedA.1).split_at(16usize);
  let mut G2in: [u8; 64] = [0u8; 64usize];
  let mut G2out: [u8; 48] = [0u8; 48usize];
  let mut Fin: [u8; 9768] = [0u8; 9768usize];
  let mut shake_input_seedSEprime: [u8; 33] = [0u8; 33usize];
  for i in 0u32..640u32.wrapping_mul(8u32) { S[i as usize] = sk_S[i as usize] };
  crate::util::frodo_unpack(
    &mut Bp,
    640u32.wrapping_mul(8u32) as usize,
    ct_c2.0,
    15u32.wrapping_mul(640u32).wrapping_mul(8u32).wrapping_div(8u32) as usize,
    15u8
  );
  crate::util::frodo_unpack(
    &mut C,
    8u32.wrapping_mul(8u32) as usize,
    salt.0,
    15u32.wrapping_mul(8u32).wrapping_mul(8u32).wrapping_div(8u32) as usize,
    15u8
  );
  frodo_mul_bs(&mut W, &Bp, &S);
  frodo_sub_inplace(&mut W, &C);
  frodo_key_decode(crate::scylla_glue::scylla_u16_of_u8_mut(&mut G2in[16usize..]), &W);
  ((&mut G2in[0usize..])[0usize..16usize]).copy_from_slice(&sk_pkh.1[0usize..16usize]);
  ((&mut
  G2in[16u32.wrapping_add(2u32.wrapping_mul(8u32).wrapping_mul(8u32).wrapping_div(8u32)) as usize..])[0usize..2u32.wrapping_mul(
    16u32
  )
  as
  usize]).copy_from_slice(&salt.1[0usize..2u32.wrapping_mul(16u32) as usize]);
  crate::fips202::shake128(
    &mut G2out,
    2u32.wrapping_mul(16u32).wrapping_add(16u32) as u64,
    &G2in,
    16u32.wrapping_add(2u32.wrapping_mul(8u32).wrapping_mul(8u32).wrapping_div(8u32)).wrapping_add(
      2u32.wrapping_mul(16u32)
    )
    as
    u64
  );
  shake_input_seedSEprime[0usize] = 150u8;
  ((&mut shake_input_seedSEprime[1usize..])[0usize..2u32.wrapping_mul(16u32) as usize]).copy_from_slice(
    &(&G2out[0usize..])[0usize..2u32.wrapping_mul(16u32) as usize]
  );
  crate::fips202::shake128(
    crate::scylla_glue::scylla_u8_of_u16_mut(&mut Sp),
    (2u32.wrapping_mul(640u32).wrapping_add(8u32).wrapping_mul(8u32) as usize).wrapping_mul(2usize)
    as
    u64,
    &shake_input_seedSEprime,
    1u32.wrapping_add(2u32.wrapping_mul(16u32)) as u64
  );
  for i in 0u32..2u32.wrapping_mul(640u32).wrapping_add(8u32).wrapping_mul(8u32)
  { Sp[i as usize] = Sp[i as usize] };
  frodo_sample_n(&mut Sp, 640u32.wrapping_mul(8u32) as usize);
  frodo_sample_n(
    &mut Sp[640u32.wrapping_mul(8u32) as usize..],
    640u32.wrapping_mul(8u32) as usize
  );
  let Sp0: (&mut [u16], &mut [u16]) = Sp.split_at_mut(0usize);
  let Ep0: (&mut [u16], &mut [u16]) = (Sp0.1).split_at_mut(640u32.wrapping_mul(8u32) as usize);
  let _ignored_stmt: i32 = frodo_mul_add_sa_plus_e(&mut BBp, Ep0.0, Ep0.1, pk_b.0);
  frodo_sample_n(
    &mut Sp[2u32.wrapping_mul(640u32).wrapping_mul(8u32) as usize..],
    8u32.wrapping_mul(8u32) as usize
  );
  crate::util::frodo_unpack(
    &mut B,
    640u32.wrapping_mul(8u32) as usize,
    pk_b.1,
    9616u32.wrapping_sub(16u32) as usize,
    15u8
  );
  frodo_mul_add_sb_plus_e(
    &mut W,
    &B,
    &Sp,
    &Sp[2u32.wrapping_mul(640u32).wrapping_mul(8u32) as usize..]
  );
  frodo_key_encode(&mut CC, crate::scylla_glue::scylla_u16_of_u8_mut(&mut G2in[16usize..]));
  frodo_add_inplace(&mut CC, &W);
  ((&mut Fin[0usize..])[0usize..9752usize]).copy_from_slice(&ct[0usize..9752usize]);
  for i in 0u32..640u32.wrapping_mul(8u32)
  {
    BBp[i as usize] =
        (BBp[i as usize] as u32 & 1i32.wrapping_shl(15u32).wrapping_sub(1i32) as u32) as u16
  };
  let selector: i8 =
      crate::util::ct_verify(&Bp, &BBp, 640u32.wrapping_mul(8u32) as usize)
      |
      crate::util::ct_verify(&C, &CC, 8u32.wrapping_mul(8u32) as usize);
  crate::util::ct_select(
    &mut Fin[9752usize..],
    &G2out[2u32.wrapping_mul(16u32) as usize..],
    sk_pkh.1,
    16usize,
    selector
  );
  crate::fips202::shake128(ss, 16u64, &Fin, 9752u32.wrapping_add(16u32) as u64);
  crate::util::clear_bytes(
    crate::scylla_glue::scylla_u8_of_u16_mut(&mut W),
    (8u32.wrapping_mul(8u32) as usize).wrapping_mul(2usize)
  );
  crate::util::clear_bytes(
    crate::scylla_glue::scylla_u8_of_u16_mut(&mut Sp),
    (640u32.wrapping_mul(8u32) as usize).wrapping_mul(2usize)
  );
  crate::util::clear_bytes(
    crate::scylla_glue::scylla_u8_of_u16_mut(&mut S),
    (640u32.wrapping_mul(8u32) as usize).wrapping_mul(2usize)
  );
  crate::util::clear_bytes(
    crate::scylla_glue::scylla_u8_of_u16_mut(&mut Sp[640u32.wrapping_mul(8u32) as usize..]),
    (640u32.wrapping_mul(8u32) as usize).wrapping_mul(2usize)
  );
  crate::util::clear_bytes(
    crate::scylla_glue::scylla_u8_of_u16_mut(
      &mut Sp[2u32.wrapping_mul(640u32).wrapping_mul(8u32) as usize..]
    ),
    (8u32.wrapping_mul(8u32) as usize).wrapping_mul(2usize)
  );
  crate::util::clear_bytes(
    &mut G2in[16usize..],
    2u32.wrapping_mul(8u32).wrapping_mul(8u32).wrapping_div(8u32) as usize
  );
  crate::util::clear_bytes(&mut G2out, 2u32.wrapping_mul(16u32).wrapping_add(16u32) as usize);
  crate::util::clear_bytes(&mut Fin[9752usize..], 16usize);
  crate::util::clear_bytes(
    &mut shake_input_seedSEprime,
    1u32.wrapping_add(2u32.wrapping_mul(16u32)) as usize
  );
  return 0i32
}

pub fn crypto_kem_enc_Frodo640(ct: &mut [u8], ss: &mut [u8], pk: &[u8]) -> i32
{
  let pk_seedA: (&[u8], &[u8]) = pk.split_at(0usize);
  let pk_b: (&[u8], &[u8]) = (pk_seedA.1).split_at(16usize);
  let ct_c1: (&mut [u8], &mut [u8]) = ct.split_at_mut(0usize);
  let ct_c2: (&mut [u8], &mut [u8]) =
      (ct_c1.1).split_at_mut(
        15u32.wrapping_mul(640u32).wrapping_mul(8u32).wrapping_div(8u32) as usize
      );
  let mut B: [u16; 5120] = [0u16; 5120usize];
  let mut V: [u16; 64] = [0u16; 64usize];
  let mut C: [u16; 64] = [0u16; 64usize];
  let mut Bp: [u16; 5120] = [0u16; 5120usize];
  let mut Sp: [u16; 10304] = [0u16; 10304usize];
  let mut G2in: [u8; 64] = [0u8; 64usize];
  let mut G2out: [u8; 48] = [0u8; 48usize];
  let k: (&[u8], &[u8]) = G2out.split_at(2u32.wrapping_mul(16u32) as usize);
  let mut Fin: [u8; 9768] = [0u8; 9768usize];
  let mut shake_input_seedSE: [u8; 33] = [0u8; 33usize];
  crate::fips202::shake128(&mut G2in[0usize..], 16u64, pk, 9616u64);
  if
  crate::random::randombytes(
    &mut G2in[16usize..],
    2u32.wrapping_mul(8u32).wrapping_mul(8u32).wrapping_div(8u32).wrapping_add(
      2u32.wrapping_mul(16u32)
    )
    as
    u64
  )
  !=
  0i32
  { return 1i32 };
  crate::fips202::shake128(
    &mut G2out,
    2u32.wrapping_mul(16u32).wrapping_add(16u32) as u64,
    &G2in,
    16u32.wrapping_add(2u32.wrapping_mul(8u32).wrapping_mul(8u32).wrapping_div(8u32)).wrapping_add(
      2u32.wrapping_mul(16u32)
    )
    as
    u64
  );
  let seedSE: (&[u8], &[u8]) = G2out.split_at(0usize);
  shake_input_seedSE[0usize] = 150u8;
  ((&mut shake_input_seedSE[1usize..])[0usize..2u32.wrapping_mul(16u32) as usize]).copy_from_slice(
    &seedSE.1[0usize..2u32.wrapping_mul(16u32) as usize]
  );
  crate::fips202::shake128(
    crate::scylla_glue::scylla_u8_of_u16_mut(&mut Sp),
    (2u32.wrapping_mul(640u32).wrapping_add(8u32).wrapping_mul(8u32) as usize).wrapping_mul(2usize)
    as
    u64,
    &shake_input_seedSE,
    1u32.wrapping_add(2u32.wrapping_mul(16u32)) as u64
  );
  for i in 0usize..2u32.wrapping_mul(640u32).wrapping_add(8u32).wrapping_mul(8u32) as usize
  { Sp[i] = Sp[i] };
  frodo_sample_n(&mut Sp, 640u32.wrapping_mul(8u32) as usize);
  frodo_sample_n(
    &mut Sp[640u32.wrapping_mul(8u32) as usize..],
    640u32.wrapping_mul(8u32) as usize
  );
  let Sp0: (&mut [u16], &mut [u16]) = Sp.split_at_mut(0usize);
  let Ep0: (&mut [u16], &mut [u16]) = (Sp0.1).split_at_mut(640u32.wrapping_mul(8u32) as usize);
  let _ignored_stmt: i32 = frodo_mul_add_sa_plus_e(&mut Bp, Ep0.0, Ep0.1, pk_b.0);
  crate::util::frodo_pack(
    ct_c2.0,
    15u32.wrapping_mul(640u32).wrapping_mul(8u32).wrapping_div(8u32) as usize,
    &Bp,
    640u32.wrapping_mul(8u32) as usize,
    15u8
  );
  frodo_sample_n(
    &mut Sp[2u32.wrapping_mul(640u32).wrapping_mul(8u32) as usize..],
    8u32.wrapping_mul(8u32) as usize
  );
  crate::util::frodo_unpack(
    &mut B,
    640u32.wrapping_mul(8u32) as usize,
    pk_b.1,
    9616u32.wrapping_sub(16u32) as usize,
    15u8
  );
  frodo_mul_add_sb_plus_e(
    &mut V,
    &B,
    &Sp,
    &Sp[2u32.wrapping_mul(640u32).wrapping_mul(8u32) as usize..]
  );
  frodo_key_encode(&mut C, crate::scylla_glue::scylla_u16_of_u8_mut(&mut G2in[16usize..]));
  frodo_add_inplace(&mut C, &V);
  crate::util::frodo_pack(
    ct_c2.1,
    15u32.wrapping_mul(8u32).wrapping_mul(8u32).wrapping_div(8u32) as usize,
    &C,
    8u32.wrapping_mul(8u32) as usize,
    15u8
  );
  ((&mut ct[9752u32.wrapping_sub(2u32.wrapping_mul(16u32)) as usize..])[0usize..2u32.wrapping_mul(
    16u32
  )
  as
  usize]).copy_from_slice(
    &(&G2in[16u32.wrapping_add(2u32.wrapping_mul(8u32).wrapping_mul(8u32).wrapping_div(8u32))
    as
    usize..])[0usize..2u32.wrapping_mul(16u32) as usize]
  );
  ((&mut Fin[0usize..])[0usize..9752usize]).copy_from_slice(&ct[0usize..9752usize]);
  ((&mut Fin[9752usize..])[0usize..16usize]).copy_from_slice(&seedSE.1[0usize..16usize]);
  crate::fips202::shake128(ss, 16u64, &Fin, 9752u32.wrapping_add(16u32) as u64);
  crate::util::clear_bytes(
    crate::scylla_glue::scylla_u8_of_u16_mut(&mut V),
    (8u32.wrapping_mul(8u32) as usize).wrapping_mul(2usize)
  );
  crate::util::clear_bytes(
    crate::scylla_glue::scylla_u8_of_u16_mut(&mut Sp),
    (640u32.wrapping_mul(8u32) as usize).wrapping_mul(2usize)
  );
  crate::util::clear_bytes(
    crate::scylla_glue::scylla_u8_of_u16_mut(&mut Sp[640u32.wrapping_mul(8u32) as usize..]),
    (640u32.wrapping_mul(8u32) as usize).wrapping_mul(2usize)
  );
  crate::util::clear_bytes(
    crate::scylla_glue::scylla_u8_of_u16_mut(
      &mut Sp[2u32.wrapping_mul(640u32).wrapping_mul(8u32) as usize..]
    ),
    (8u32.wrapping_mul(8u32) as usize).wrapping_mul(2usize)
  );
  crate::util::clear_bytes(
    &mut G2in[16usize..],
    2u32.wrapping_mul(8u32).wrapping_mul(8u32).wrapping_div(8u32) as usize
  );
  crate::util::clear_bytes(&mut G2out, 2u32.wrapping_mul(16u32).wrapping_add(16u32) as usize);
  crate::util::clear_bytes(&mut Fin[9752usize..], 16usize);
  crate::util::clear_bytes(
    &mut shake_input_seedSE,
    1u32.wrapping_add(2u32.wrapping_mul(16u32)) as usize
  );
  return 0i32
}

pub fn crypto_kem_keypair_Frodo640(pk: &mut [u8], sk: &mut [u8]) -> i32
{
  let pk_seedA: (&mut [u8], &mut [u8]) = pk.split_at_mut(0usize);
  let sk_s: (&mut [u8], &mut [u8]) = sk.split_at_mut(0usize);
  let sk_pk: (&mut [u8], &mut [u8]) = (sk_s.1).split_at_mut(16usize);
  let sk_S: (&mut [u8], &mut [u8]) = (sk_pk.1).split_at_mut(16usize + 9600usize);
  let sk_pkh: (&mut [u8], &mut [u8]) =
      (sk_pk.0).split_at_mut(
        16usize.wrapping_add(9616usize).wrapping_add(
          2usize.wrapping_mul(640usize).wrapping_mul(8usize)
        )
      );
  let mut B: [u16; 5120] = [0u16; 5120usize];
  let mut S: [u16; 10240] = [0u16; 10240usize];
  let mut randomness: [u8; 64] = [0u8; 64usize];
  let mut shake_input_seedSE: [u8; 33] = [0u8; 33usize];
  if
  crate::random::randombytes(
    &mut randomness,
    16u32.wrapping_add(2u32.wrapping_mul(16u32)).wrapping_add(16u32) as u64
  )
  !=
  0i32
  { return 1i32 };
  let randomness_s: (&[u8], &[u8]) = randomness.split_at(0usize);
  let randomness_seedSE: (&[u8], &[u8]) = (randomness_s.1).split_at(16usize);
  let randomness_z: (&[u8], &[u8]) =
      (randomness_seedSE.0).split_at(16u32.wrapping_add(2u32.wrapping_mul(16u32)) as usize);
  crate::fips202::shake128(pk_seedA.1, 16u64, randomness_z.1, 16u64);
  shake_input_seedSE[0usize] = 95u8;
  ((&mut shake_input_seedSE[1usize..])[0usize..2u32.wrapping_mul(16u32) as usize]).copy_from_slice(
    &randomness_seedSE.1[0usize..2u32.wrapping_mul(16u32) as usize]
  );
  crate::fips202::shake128(
    crate::scylla_glue::scylla_u8_of_u16_mut(&mut S),
    (2u32.wrapping_mul(640u32).wrapping_mul(8u32) as usize).wrapping_mul(2usize) as u64,
    &shake_input_seedSE,
    1u32.wrapping_add(2u32.wrapping_mul(16u32)) as u64
  );
  for i in 0usize..2u32.wrapping_mul(640u32).wrapping_mul(8u32) as usize { S[i] = S[i] };
  frodo_sample_n(&mut S, 640u32.wrapping_mul(8u32) as usize);
  frodo_sample_n(
    &mut S[640u32.wrapping_mul(8u32) as usize..],
    640u32.wrapping_mul(8u32) as usize
  );
  let _ignored_stmt: i32 =
      frodo_mul_add_as_plus_e(&mut B, &S, &S[640u32.wrapping_mul(8u32) as usize..], pk);
  crate::util::frodo_pack(
    &mut pk[16usize..],
    9616u32.wrapping_sub(16u32) as usize,
    &B,
    640u32.wrapping_mul(8u32) as usize,
    15u8
  );
  (sk_pkh.0[0usize..16usize]).copy_from_slice(&randomness_z.0[0usize..16usize]);
  (sk_S.0[0usize..9616usize]).copy_from_slice(&pk[0usize..9616usize]);
  for i in 0usize..640u32.wrapping_mul(8u32) as usize { S[i] = S[i] };
  (sk_S.1[0usize..2u32.wrapping_mul(640u32).wrapping_mul(8u32) as usize]).copy_from_slice(
    &crate::scylla_glue::scylla_u8_of_u16_mut(&mut S)[0usize..2u32.wrapping_mul(640u32).wrapping_mul(
      8u32
    )
    as
    usize]
  );
  crate::fips202::shake128(sk_pkh.1, 16u64, pk, 9616u64);
  crate::util::clear_bytes(
    crate::scylla_glue::scylla_u8_of_u16_mut(&mut S),
    (640u32.wrapping_mul(8u32) as usize).wrapping_mul(2usize)
  );
  crate::util::clear_bytes(
    crate::scylla_glue::scylla_u8_of_u16_mut(&mut S[640u32.wrapping_mul(8u32) as usize..]),
    (640u32.wrapping_mul(8u32) as usize).wrapping_mul(2usize)
  );
  crate::util::clear_bytes(
    &mut randomness,
    16u32.wrapping_add(2u32.wrapping_mul(16u32)) as usize
  );
  crate::util::clear_bytes(
    &mut shake_input_seedSE,
    1u32.wrapping_add(2u32.wrapping_mul(16u32)) as usize
  );
  return 0i32
}

pub fn frodo_add(out: &mut [u16], a: &[u16], b: &[u16])
{
  for i in 0u32..8u32.wrapping_mul(8u32)
  {
    out[i as usize] =
        ((a[i as usize]).wrapping_add(b[i as usize]) as u32
        &
        1i32.wrapping_shl(15u32).wrapping_sub(1i32) as u32)
        as
        u16
  }
}

pub fn frodo_add_inplace(out: &mut [u16], a: &[u16])
{
  for i in 0u32..8u32.wrapping_mul(8u32)
  {
    out[i as usize] =
        ((a[i as usize]).wrapping_add(out[i as usize]) as u32
        &
        1i32.wrapping_shl(15u32).wrapping_sub(1i32) as u32)
        as
        u16
  }
}

pub fn frodo_key_decode(out: &mut [u16], r#in: &[u16])
{
  let mut i: u32;
  let mut j: u32;
  let mut index: u32 = 0u32;
  let npieces_word: u32 = 8u32;
  let nwords: u32 = 8u32.wrapping_mul(8u32).wrapping_div(8u32);
  let mut temp: u16;
  let maskex: u16 = 1i32.wrapping_shl(2u32).wrapping_sub(1i32) as u16;
  let maskq: u16 = 1i32.wrapping_shl(15u32).wrapping_sub(1i32) as u16;
  let pos: &mut [u8] = crate::scylla_glue::scylla_u8_of_u16_mut(out);
  let mut templong: u64;
  i = 0u32;
  while
  i < nwords
  {
    {
      templong = 0u64;
      {
        j = 0u32;
        while
        j < npieces_word
        {
          {
            temp =
                ((r#in[index as usize] & maskq) as u32).wrapping_add(
                  1i32.wrapping_shl(15i32.wrapping_sub(2i32).wrapping_sub(1i32) as u32) as u32
                ).wrapping_shr(15i32.wrapping_sub(2i32) as u32)
                as
                u16;
            templong |= ((temp & maskex) as u64).wrapping_shl(2u32.wrapping_mul(j));
            index = index.wrapping_add(1u32)
          };
          j = j.wrapping_add(1u32)
        }
      };
      j = 0u32;
      while
      j < 2u32
      {
        pos[i.wrapping_mul(2u32).wrapping_add(j) as usize] =
            (templong.wrapping_shr(8u32.wrapping_mul(j)) & 255u64) as u8;
        j = j.wrapping_add(1u32)
      }
    };
    i = i.wrapping_add(1u32)
  }
}

pub fn frodo_key_encode(out: &mut [u16], r#in: &[u16])
{
  let mut i: u32;
  let mut j: u32;
  let npieces_word: u32 = 8u32;
  let nwords: u32 = 8u32.wrapping_mul(8u32).wrapping_div(8u32);
  let mut temp: u64;
  let mask: u64 = 1u64.wrapping_shl(2u32).wrapping_sub(1u64);
  let mut pos: (&mut [u16], &mut [u16]) = out.split_at_mut(0usize);
  i = 0u32;
  while
  i < nwords
  {
    {
      temp = 0u64;
      {
        j = 0u32;
        while
        j < 2u32
        {
          temp |=
              (crate::scylla_glue::scylla_u8_of_u16(r#in)[i.wrapping_mul(2u32).wrapping_add(j)
              as
              usize]
              as
              u64).wrapping_shl(8u32.wrapping_mul(j));
          j = j.wrapping_add(1u32)
        }
      };
      j = 0u32;
      while
      j < npieces_word
      {
        {
          pos.1[0usize] = (temp & mask).wrapping_shl(15i32.wrapping_sub(2i32) as u32) as u16;
          temp = temp.wrapping_shr(2u32);
          pos.1 = &mut pos.1[1usize..]
        };
        j = j.wrapping_add(1u32)
      }
    };
    i = i.wrapping_add(1u32)
  }
}

pub fn frodo_mul_add_as_plus_e(out: &mut [u16], s: &[u16], e: &[u16], seed_A: &[u8]) -> i32
{
  let mut i: i32;
  let mut j: i32;
  let mut k: i32;
  let mut a_row: [i16; 2560] = [0i16; 2560usize];
  {
    i = 0i32;
    while
    (i as u32) < 640u32.wrapping_mul(8u32)
    {
      crate::scylla_glue::scylla_u32_of_u16_mut(&mut out[i as usize..])[0usize] =
          crate::scylla_glue::scylla_u32_of_u16(&e[i as usize..])[0usize];
      i = i.wrapping_add(2i32)
    }
  };
  let mut a_row_temp: [i16; 2560] = [0i16; 2560usize];
  let mut aes_key_schedule: [u8; 176] = [0u8; 176usize];
  crate::aes::AES128_load_schedule(seed_A, &mut aes_key_schedule);
  {
    j = 0i32;
    while
    (j as u32) < 640u32
    {
      {
        a_row_temp[(j.wrapping_add(1i32) as u32).wrapping_add(0u32.wrapping_mul(640u32)) as usize] =
            j as i16;
        a_row_temp[(j.wrapping_add(1i32) as u32).wrapping_add(1u32.wrapping_mul(640u32)) as usize] =
            j as i16;
        a_row_temp[(j.wrapping_add(1i32) as u32).wrapping_add(2u32.wrapping_mul(640u32)) as usize] =
            j as i16;
        a_row_temp[(j.wrapping_add(1i32) as u32).wrapping_add(3u32.wrapping_mul(640u32)) as usize] =
            j as i16
      };
      j = j.wrapping_add(8i32)
    }
  };
  {
    i = 0i32;
    while
    (i as u32) < 640u32
    {
      {
        {
          j = 0i32;
          while
          (j as u32) < 640u32
          {
            {
              a_row_temp[(j as u32).wrapping_add(0u32.wrapping_mul(640u32)) as usize] =
                  i.wrapping_add(0i32) as i16;
              a_row_temp[(j as u32).wrapping_add(1u32.wrapping_mul(640u32)) as usize] =
                  i.wrapping_add(1i32) as i16;
              a_row_temp[(j as u32).wrapping_add(2u32.wrapping_mul(640u32)) as usize] =
                  i.wrapping_add(2i32) as i16;
              a_row_temp[(j as u32).wrapping_add(3u32.wrapping_mul(640u32)) as usize] =
                  i.wrapping_add(3i32) as i16
            };
            j = j.wrapping_add(8i32)
          }
        };
        crate::aes::AES128_ECB_enc_sch(
          crate::scylla_glue::scylla_u8_of_i16_mut(&mut a_row_temp),
          (4u32.wrapping_mul(640u32) as usize).wrapping_mul(2usize),
          &aes_key_schedule,
          crate::scylla_glue::scylla_u8_of_i16_mut(&mut a_row)
        );
        {
          k = 0i32;
          while
          (k as u32) < 4u32.wrapping_mul(640u32)
          {
            a_row[k as usize] = a_row[k as usize];
            k = k.wrapping_add(1i32)
          }
        };
        k = 0i32;
        while
        (k as u32) < 8u32
        {
          {
            let mut sum: [u16; 4] = [0u16; 4usize];
            {
              j = 0i32;
              while
              (j as u32) < 640u32
              {
                {
                  let sp: u16 = s[(k as u32).wrapping_mul(640u32).wrapping_add(j as u32) as usize];
                  sum[0usize] =
                      (sum[0usize]).wrapping_add(
                        (a_row[0u32.wrapping_mul(640u32).wrapping_add(j as u32) as usize] as u16).wrapping_mul(
                          sp
                        )
                      );
                  sum[1usize] =
                      (sum[1usize]).wrapping_add(
                        (a_row[1u32.wrapping_mul(640u32).wrapping_add(j as u32) as usize] as u16).wrapping_mul(
                          sp
                        )
                      );
                  sum[2usize] =
                      (sum[2usize]).wrapping_add(
                        (a_row[2u32.wrapping_mul(640u32).wrapping_add(j as u32) as usize] as u16).wrapping_mul(
                          sp
                        )
                      );
                  sum[3usize] =
                      (sum[3usize]).wrapping_add(
                        (a_row[3u32.wrapping_mul(640u32).wrapping_add(j as u32) as usize] as u16).wrapping_mul(
                          sp
                        )
                      )
                };
                j = j.wrapping_add(1i32)
              }
            };
            out[(i.wrapping_add(0i32) as u32).wrapping_mul(8u32).wrapping_add(k as u32) as usize] =
                (out[(i.wrapping_add(0i32) as u32).wrapping_mul(8u32).wrapping_add(k as u32)
                as
                usize]).wrapping_add(sum[0usize]);
            out[(i.wrapping_add(2i32) as u32).wrapping_mul(8u32).wrapping_add(k as u32) as usize] =
                (out[(i.wrapping_add(2i32) as u32).wrapping_mul(8u32).wrapping_add(k as u32)
                as
                usize]).wrapping_add(sum[2usize]);
            out[(i.wrapping_add(1i32) as u32).wrapping_mul(8u32).wrapping_add(k as u32) as usize] =
                (out[(i.wrapping_add(1i32) as u32).wrapping_mul(8u32).wrapping_add(k as u32)
                as
                usize]).wrapping_add(sum[1usize]);
            out[(i.wrapping_add(3i32) as u32).wrapping_mul(8u32).wrapping_add(k as u32) as usize] =
                (out[(i.wrapping_add(3i32) as u32).wrapping_mul(8u32).wrapping_add(k as u32)
                as
                usize]).wrapping_add(sum[3usize])
          };
          k = k.wrapping_add(1i32)
        }
      };
      i = i.wrapping_add(4i32)
    }
  };
  crate::aes::AES128_free_schedule(&aes_key_schedule);
  return 1i32
}

pub fn frodo_mul_add_sa_plus_e(out: &mut [u16], s: &[u16], e: &mut [u16], seed_A: &[u8]) -> i32
{
  let mut i: i32;
  let mut j: i32;
  let mut q: i32;
  let mut p: i32;
  let mut A: [u16; 5120] = [0u16; 5120usize];
  let mut aes_key_schedule: [u8; 176] = [0u8; 176usize];
  crate::aes::AES128_load_schedule(seed_A, &mut aes_key_schedule);
  let mut Ainit: [u16; 5120] = [0u16; 5120usize];
  {
    j = 0i32;
    while
    (j as u32) < 640u32
    {
      {
        Ainit[0u32.wrapping_mul(640u32).wrapping_add(j as u32).wrapping_add(1u32) as usize] =
            j as u16;
        Ainit[1u32.wrapping_mul(640u32).wrapping_add(j as u32).wrapping_add(1u32) as usize] =
            j as u16;
        Ainit[2u32.wrapping_mul(640u32).wrapping_add(j as u32).wrapping_add(1u32) as usize] =
            j as u16;
        Ainit[3u32.wrapping_mul(640u32).wrapping_add(j as u32).wrapping_add(1u32) as usize] =
            j as u16;
        Ainit[4u32.wrapping_mul(640u32).wrapping_add(j as u32).wrapping_add(1u32) as usize] =
            j as u16;
        Ainit[5u32.wrapping_mul(640u32).wrapping_add(j as u32).wrapping_add(1u32) as usize] =
            j as u16;
        Ainit[6u32.wrapping_mul(640u32).wrapping_add(j as u32).wrapping_add(1u32) as usize] =
            j as u16;
        Ainit[7u32.wrapping_mul(640u32).wrapping_add(j as u32).wrapping_add(1u32) as usize] =
            j as u16
      };
      j = j.wrapping_add(8i32)
    }
  };
  {
    i = 0i32;
    while
    (i as u32) < 640u32
    {
      {
        {
          q = 0i32;
          while
          q < 8i32
          {
            {
              p = 0i32;
              while
              (p as u32) < 640u32
              {
                Ainit[(q as u32).wrapping_mul(640u32).wrapping_add(p as u32) as usize] =
                    i.wrapping_add(q) as u16;
                p = p.wrapping_add(8i32)
              }
            };
            q = q.wrapping_add(1i32)
          }
        };
        let A_len: usize = (8u32.wrapping_mul(640u32) as usize).wrapping_mul(2usize);
        crate::aes::AES128_ECB_enc_sch(
          crate::scylla_glue::scylla_u8_of_u16_mut(&mut Ainit),
          A_len,
          &aes_key_schedule,
          crate::scylla_glue::scylla_u8_of_u16_mut(&mut A)
        );
        j = 0i32;
        while
        (j as u32) < 8u32
        {
          {
            let mut sum: u16 = 0u16;
            let mut sp: [i16; 8] = [0i16; 8usize];
            {
              p = 0i32;
              while
              p < 8i32
              {
                sp[p as usize] =
                    s[(j as u32).wrapping_mul(640u32).wrapping_add(i as u32).wrapping_add(p as u32)
                    as
                    usize]
                    as
                    i16;
                p = p.wrapping_add(1i32)
              }
            };
            q = 0i32;
            while
            (q as u32) < 640u32
            {
              {
                sum = e[(j as u32).wrapping_mul(640u32).wrapping_add(q as u32) as usize];
                {
                  p = 0i32;
                  while
                  p < 8i32
                  {
                    sum =
                        sum.wrapping_add(
                          (sp[p as usize] as u16).wrapping_mul(
                            A[(p as u32).wrapping_mul(640u32).wrapping_add(q as u32) as usize]
                          )
                        );
                    p = p.wrapping_add(1i32)
                  }
                };
                e[(j as u32).wrapping_mul(640u32).wrapping_add(q as u32) as usize] = sum
              };
              q = q.wrapping_add(1i32)
            }
          };
          j = j.wrapping_add(1i32)
        }
      };
      i = i.wrapping_add(8i32)
    }
  };
  (crate::scylla_glue::scylla_u8_of_u16_mut(out)[0usize..2u32.wrapping_mul(640u32).wrapping_mul(
    8u32
  )
  as
  usize]).copy_from_slice(
    &crate::scylla_glue::scylla_u8_of_u16_mut(e)[0usize..2u32.wrapping_mul(640u32).wrapping_mul(
      8u32
    )
    as
    usize]
  );
  crate::aes::AES128_free_schedule(&aes_key_schedule);
  return 1i32
}

pub fn frodo_mul_add_sb_plus_e(out: &mut [u16], b: &[u16], s: &[u16], e: &[u16])
{
  let mut i: i32;
  let mut j: i32;
  let mut k: i32 = 0i32;
  while
  (k as u32) < 8u32
  {
    {
      i = 0i32;
      while
      (i as u32) < 8u32
      {
        {
          out[(k as u32).wrapping_mul(8u32).wrapping_add(i as u32) as usize] =
              e[(k as u32).wrapping_mul(8u32).wrapping_add(i as u32) as usize];
          {
            j = 0i32;
            while
            (j as u32) < 640u32
            {
              out[(k as u32).wrapping_mul(8u32).wrapping_add(i as u32) as usize] =
                  (out[(k as u32).wrapping_mul(8u32).wrapping_add(i as u32) as usize]).wrapping_add(
                    (s[(k as u32).wrapping_mul(640u32).wrapping_add(j as u32) as usize] as i16
                    as
                    u16).wrapping_mul(
                      b[(j as u32).wrapping_mul(8u32).wrapping_add(i as u32) as usize]
                    )
                  );
              j = j.wrapping_add(1i32)
            }
          };
          out[(k as u32).wrapping_mul(8u32).wrapping_add(i as u32) as usize] =
              (out[(k as u32).wrapping_mul(8u32).wrapping_add(i as u32) as usize] as u32
              &
              1i32.wrapping_shl(15u32).wrapping_sub(1i32) as u32)
              as
              u16
        };
        i = i.wrapping_add(1i32)
      }
    };
    k = k.wrapping_add(1i32)
  }
}

pub fn frodo_mul_bs(out: &mut [u16], b: &[u16], s: &[u16])
{
  let mut i: i32;
  let mut j: i32;
  let mut k: i32;
  i = 0i32;
  while
  (i as u32) < 8u32
  {
    {
      j = 0i32;
      while
      (j as u32) < 8u32
      {
        {
          out[(i as u32).wrapping_mul(8u32).wrapping_add(j as u32) as usize] = 0u16;
          {
            k = 0i32;
            while
            (k as u32) < 640u32
            {
              out[(i as u32).wrapping_mul(8u32).wrapping_add(j as u32) as usize] =
                  (out[(i as u32).wrapping_mul(8u32).wrapping_add(j as u32) as usize]).wrapping_add(
                    (b[(i as u32).wrapping_mul(640u32).wrapping_add(k as u32) as usize]).wrapping_mul(
                      s[(j as u32).wrapping_mul(640u32).wrapping_add(k as u32) as usize] as i16
                      as
                      u16
                    )
                  );
              k = k.wrapping_add(1i32)
            }
          };
          out[(i as u32).wrapping_mul(8u32).wrapping_add(j as u32) as usize] =
              (out[(i as u32).wrapping_mul(8u32).wrapping_add(j as u32) as usize] as u32
              &
              1i32.wrapping_shl(15u32).wrapping_sub(1i32) as u32)
              as
              u16
        };
        j = j.wrapping_add(1i32)
      }
    };
    i = i.wrapping_add(1i32)
  }
}

pub fn frodo_sample_n(s: &mut [u16], n: usize)
{
  let mut i: u32;
  let mut j: u32;
  i = 0u32;
  while
  (i as usize) < n
  {
    {
      let mut sample: u16 = 0u16;
      let prnd: u16 = (s[i as usize] as i32).wrapping_shr(1u32) as u16;
      let sign: u16 = (s[i as usize] as u32 & 1u32) as u16;
      {
        j = 0u32;
        while
        j < (CDF_TABLE_LEN as u32).wrapping_sub(1u32) as u32
        {
          sample =
              (sample as u32).wrapping_add(
                ((CDF_TABLE[j as usize]).wrapping_sub(prnd) as u16 as i32).wrapping_shr(15u32)
                as
                u32
              )
              as
              u16;
          j = j.wrapping_add(1u32)
        }
      };
      s[i as usize] = (0u16.wrapping_sub(sign) ^ sample).wrapping_add(sign)
    };
    i = i.wrapping_add(1u32)
  }
}

pub fn frodo_sub(out: &mut [u16], a: &[u16], b: &[u16])
{
  for i in 0u32..8u32.wrapping_mul(8u32)
  {
    out[i as usize] =
        ((a[i as usize]).wrapping_sub(b[i as usize]) as u32
        &
        1i32.wrapping_shl(15u32).wrapping_sub(1i32) as u32)
        as
        u16
  }
}

pub fn frodo_sub_inplace(out: &mut [u16], a: &[u16])
{
  for i in 0u32..8u32.wrapping_mul(8u32)
  {
    out[i as usize] =
        ((a[i as usize]).wrapping_sub(out[i as usize]) as u32
        &
        1i32.wrapping_shl(15u32).wrapping_sub(1i32) as u32)
        as
        u16
  }
}
