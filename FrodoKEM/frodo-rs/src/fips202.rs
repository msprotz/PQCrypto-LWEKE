#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(unreachable_patterns)]
#![allow(unused_mut)]

pub fn KeccakF1600_StatePermute(state: &mut [u64])
{
  let mut round: i32;
  let mut Aba: u64;
  let mut Abe: u64;
  let mut Abi: u64;
  let mut Abo: u64;
  let mut Abu: u64;
  let mut Aga: u64;
  let mut Age: u64;
  let mut Agi: u64;
  let mut Ago: u64;
  let mut Agu: u64;
  let mut Aka: u64;
  let mut Ake: u64;
  let mut Aki: u64;
  let mut Ako: u64;
  let mut Aku: u64;
  let mut Ama: u64;
  let mut Ame: u64;
  let mut Ami: u64;
  let mut Amo: u64;
  let mut Amu: u64;
  let mut Asa: u64;
  let mut Ase: u64;
  let mut Asi: u64;
  let mut Aso: u64;
  let mut Asu: u64;
  let mut BCa: u64;
  let mut BCe: u64;
  let mut BCi: u64;
  let mut BCo: u64;
  let mut BCu: u64;
  let mut Da: u64;
  let mut De: u64;
  let mut Di: u64;
  let mut Do: u64;
  let mut Du: u64;
  let mut Eba: u64;
  let mut Ebe: u64;
  let mut Ebi: u64;
  let mut Ebo: u64;
  let mut Ebu: u64;
  let mut Ega: u64;
  let mut Ege: u64;
  let mut Egi: u64;
  let mut Ego: u64;
  let mut Egu: u64;
  let mut Eka: u64;
  let mut Eke: u64;
  let mut Eki: u64;
  let mut Eko: u64;
  let mut Eku: u64;
  let mut Ema: u64;
  let mut Eme: u64;
  let mut Emi: u64;
  let mut Emo: u64;
  let mut Emu: u64;
  let mut Esa: u64;
  let mut Ese: u64;
  let mut Esi: u64;
  let mut Eso: u64;
  let mut Esu: u64;
  Aba = state[0usize];
  Abe = state[1usize];
  Abi = state[2usize];
  Abo = state[3usize];
  Abu = state[4usize];
  Aga = state[5usize];
  Age = state[6usize];
  Agi = state[7usize];
  Ago = state[8usize];
  Agu = state[9usize];
  Aka = state[10usize];
  Ake = state[11usize];
  Aki = state[12usize];
  Ako = state[13usize];
  Aku = state[14usize];
  Ama = state[15usize];
  Ame = state[16usize];
  Ami = state[17usize];
  Amo = state[18usize];
  Amu = state[19usize];
  Asa = state[20usize];
  Ase = state[21usize];
  Asi = state[22usize];
  Aso = state[23usize];
  Asu = state[24usize];
  {
    round = 0i32;
    while
    round < 24i32
    {
      {
        BCa = Aba ^ Aga ^ Aka ^ Ama ^ Asa;
        BCe = Abe ^ Age ^ Ake ^ Ame ^ Ase;
        BCi = Abi ^ Agi ^ Aki ^ Ami ^ Asi;
        BCo = Abo ^ Ago ^ Ako ^ Amo ^ Aso;
        BCu = Abu ^ Agu ^ Aku ^ Amu ^ Asu;
        Da = BCu ^ (BCe.wrapping_shl(1u32) ^ BCe.wrapping_shr(64i32.wrapping_sub(1i32) as u32));
        De = BCa ^ (BCi.wrapping_shl(1u32) ^ BCi.wrapping_shr(64i32.wrapping_sub(1i32) as u32));
        Di = BCe ^ (BCo.wrapping_shl(1u32) ^ BCo.wrapping_shr(64i32.wrapping_sub(1i32) as u32));
        Do = BCi ^ (BCu.wrapping_shl(1u32) ^ BCu.wrapping_shr(64i32.wrapping_sub(1i32) as u32));
        Du = BCo ^ (BCa.wrapping_shl(1u32) ^ BCa.wrapping_shr(64i32.wrapping_sub(1i32) as u32));
        Aba ^= Da;
        BCa = Aba;
        Age ^= De;
        BCe = Age.wrapping_shl(44u32) ^ Age.wrapping_shr(64i32.wrapping_sub(44i32) as u32);
        Aki ^= Di;
        BCi = Aki.wrapping_shl(43u32) ^ Aki.wrapping_shr(64i32.wrapping_sub(43i32) as u32);
        Amo ^= Do;
        BCo = Amo.wrapping_shl(21u32) ^ Amo.wrapping_shr(64i32.wrapping_sub(21i32) as u32);
        Asu ^= Du;
        BCu = Asu.wrapping_shl(14u32) ^ Asu.wrapping_shr(64i32.wrapping_sub(14i32) as u32);
        Eba = BCa ^ ! BCe & BCi;
        Eba ^= KeccakF_RoundConstants[round as usize] as u64;
        Ebe = BCe ^ ! BCi & BCo;
        Ebi = BCi ^ ! BCo & BCu;
        Ebo = BCo ^ ! BCu & BCa;
        Ebu = BCu ^ ! BCa & BCe;
        Abo ^= Do;
        BCa = Abo.wrapping_shl(28u32) ^ Abo.wrapping_shr(64i32.wrapping_sub(28i32) as u32);
        Agu ^= Du;
        BCe = Agu.wrapping_shl(20u32) ^ Agu.wrapping_shr(64i32.wrapping_sub(20i32) as u32);
        Aka ^= Da;
        BCi = Aka.wrapping_shl(3u32) ^ Aka.wrapping_shr(64i32.wrapping_sub(3i32) as u32);
        Ame ^= De;
        BCo = Ame.wrapping_shl(45u32) ^ Ame.wrapping_shr(64i32.wrapping_sub(45i32) as u32);
        Asi ^= Di;
        BCu = Asi.wrapping_shl(61u32) ^ Asi.wrapping_shr(64i32.wrapping_sub(61i32) as u32);
        Ega = BCa ^ ! BCe & BCi;
        Ege = BCe ^ ! BCi & BCo;
        Egi = BCi ^ ! BCo & BCu;
        Ego = BCo ^ ! BCu & BCa;
        Egu = BCu ^ ! BCa & BCe;
        Abe ^= De;
        BCa = Abe.wrapping_shl(1u32) ^ Abe.wrapping_shr(64i32.wrapping_sub(1i32) as u32);
        Agi ^= Di;
        BCe = Agi.wrapping_shl(6u32) ^ Agi.wrapping_shr(64i32.wrapping_sub(6i32) as u32);
        Ako ^= Do;
        BCi = Ako.wrapping_shl(25u32) ^ Ako.wrapping_shr(64i32.wrapping_sub(25i32) as u32);
        Amu ^= Du;
        BCo = Amu.wrapping_shl(8u32) ^ Amu.wrapping_shr(64i32.wrapping_sub(8i32) as u32);
        Asa ^= Da;
        BCu = Asa.wrapping_shl(18u32) ^ Asa.wrapping_shr(64i32.wrapping_sub(18i32) as u32);
        Eka = BCa ^ ! BCe & BCi;
        Eke = BCe ^ ! BCi & BCo;
        Eki = BCi ^ ! BCo & BCu;
        Eko = BCo ^ ! BCu & BCa;
        Eku = BCu ^ ! BCa & BCe;
        Abu ^= Du;
        BCa = Abu.wrapping_shl(27u32) ^ Abu.wrapping_shr(64i32.wrapping_sub(27i32) as u32);
        Aga ^= Da;
        BCe = Aga.wrapping_shl(36u32) ^ Aga.wrapping_shr(64i32.wrapping_sub(36i32) as u32);
        Ake ^= De;
        BCi = Ake.wrapping_shl(10u32) ^ Ake.wrapping_shr(64i32.wrapping_sub(10i32) as u32);
        Ami ^= Di;
        BCo = Ami.wrapping_shl(15u32) ^ Ami.wrapping_shr(64i32.wrapping_sub(15i32) as u32);
        Aso ^= Do;
        BCu = Aso.wrapping_shl(56u32) ^ Aso.wrapping_shr(64i32.wrapping_sub(56i32) as u32);
        Ema = BCa ^ ! BCe & BCi;
        Eme = BCe ^ ! BCi & BCo;
        Emi = BCi ^ ! BCo & BCu;
        Emo = BCo ^ ! BCu & BCa;
        Emu = BCu ^ ! BCa & BCe;
        Abi ^= Di;
        BCa = Abi.wrapping_shl(62u32) ^ Abi.wrapping_shr(64i32.wrapping_sub(62i32) as u32);
        Ago ^= Do;
        BCe = Ago.wrapping_shl(55u32) ^ Ago.wrapping_shr(64i32.wrapping_sub(55i32) as u32);
        Aku ^= Du;
        BCi = Aku.wrapping_shl(39u32) ^ Aku.wrapping_shr(64i32.wrapping_sub(39i32) as u32);
        Ama ^= Da;
        BCo = Ama.wrapping_shl(41u32) ^ Ama.wrapping_shr(64i32.wrapping_sub(41i32) as u32);
        Ase ^= De;
        BCu = Ase.wrapping_shl(2u32) ^ Ase.wrapping_shr(64i32.wrapping_sub(2i32) as u32);
        Esa = BCa ^ ! BCe & BCi;
        Ese = BCe ^ ! BCi & BCo;
        Esi = BCi ^ ! BCo & BCu;
        Eso = BCo ^ ! BCu & BCa;
        Esu = BCu ^ ! BCa & BCe;
        BCa = Eba ^ Ega ^ Eka ^ Ema ^ Esa;
        BCe = Ebe ^ Ege ^ Eke ^ Eme ^ Ese;
        BCi = Ebi ^ Egi ^ Eki ^ Emi ^ Esi;
        BCo = Ebo ^ Ego ^ Eko ^ Emo ^ Eso;
        BCu = Ebu ^ Egu ^ Eku ^ Emu ^ Esu;
        Da = BCu ^ (BCe.wrapping_shl(1u32) ^ BCe.wrapping_shr(64i32.wrapping_sub(1i32) as u32));
        De = BCa ^ (BCi.wrapping_shl(1u32) ^ BCi.wrapping_shr(64i32.wrapping_sub(1i32) as u32));
        Di = BCe ^ (BCo.wrapping_shl(1u32) ^ BCo.wrapping_shr(64i32.wrapping_sub(1i32) as u32));
        Do = BCi ^ (BCu.wrapping_shl(1u32) ^ BCu.wrapping_shr(64i32.wrapping_sub(1i32) as u32));
        Du = BCo ^ (BCa.wrapping_shl(1u32) ^ BCa.wrapping_shr(64i32.wrapping_sub(1i32) as u32));
        Eba ^= Da;
        BCa = Eba;
        Ege ^= De;
        BCe = Ege.wrapping_shl(44u32) ^ Ege.wrapping_shr(64i32.wrapping_sub(44i32) as u32);
        Eki ^= Di;
        BCi = Eki.wrapping_shl(43u32) ^ Eki.wrapping_shr(64i32.wrapping_sub(43i32) as u32);
        Emo ^= Do;
        BCo = Emo.wrapping_shl(21u32) ^ Emo.wrapping_shr(64i32.wrapping_sub(21i32) as u32);
        Esu ^= Du;
        BCu = Esu.wrapping_shl(14u32) ^ Esu.wrapping_shr(64i32.wrapping_sub(14i32) as u32);
        Aba = BCa ^ ! BCe & BCi;
        Aba ^= KeccakF_RoundConstants[round.wrapping_add(1i32) as usize] as u64;
        Abe = BCe ^ ! BCi & BCo;
        Abi = BCi ^ ! BCo & BCu;
        Abo = BCo ^ ! BCu & BCa;
        Abu = BCu ^ ! BCa & BCe;
        Ebo ^= Do;
        BCa = Ebo.wrapping_shl(28u32) ^ Ebo.wrapping_shr(64i32.wrapping_sub(28i32) as u32);
        Egu ^= Du;
        BCe = Egu.wrapping_shl(20u32) ^ Egu.wrapping_shr(64i32.wrapping_sub(20i32) as u32);
        Eka ^= Da;
        BCi = Eka.wrapping_shl(3u32) ^ Eka.wrapping_shr(64i32.wrapping_sub(3i32) as u32);
        Eme ^= De;
        BCo = Eme.wrapping_shl(45u32) ^ Eme.wrapping_shr(64i32.wrapping_sub(45i32) as u32);
        Esi ^= Di;
        BCu = Esi.wrapping_shl(61u32) ^ Esi.wrapping_shr(64i32.wrapping_sub(61i32) as u32);
        Aga = BCa ^ ! BCe & BCi;
        Age = BCe ^ ! BCi & BCo;
        Agi = BCi ^ ! BCo & BCu;
        Ago = BCo ^ ! BCu & BCa;
        Agu = BCu ^ ! BCa & BCe;
        Ebe ^= De;
        BCa = Ebe.wrapping_shl(1u32) ^ Ebe.wrapping_shr(64i32.wrapping_sub(1i32) as u32);
        Egi ^= Di;
        BCe = Egi.wrapping_shl(6u32) ^ Egi.wrapping_shr(64i32.wrapping_sub(6i32) as u32);
        Eko ^= Do;
        BCi = Eko.wrapping_shl(25u32) ^ Eko.wrapping_shr(64i32.wrapping_sub(25i32) as u32);
        Emu ^= Du;
        BCo = Emu.wrapping_shl(8u32) ^ Emu.wrapping_shr(64i32.wrapping_sub(8i32) as u32);
        Esa ^= Da;
        BCu = Esa.wrapping_shl(18u32) ^ Esa.wrapping_shr(64i32.wrapping_sub(18i32) as u32);
        Aka = BCa ^ ! BCe & BCi;
        Ake = BCe ^ ! BCi & BCo;
        Aki = BCi ^ ! BCo & BCu;
        Ako = BCo ^ ! BCu & BCa;
        Aku = BCu ^ ! BCa & BCe;
        Ebu ^= Du;
        BCa = Ebu.wrapping_shl(27u32) ^ Ebu.wrapping_shr(64i32.wrapping_sub(27i32) as u32);
        Ega ^= Da;
        BCe = Ega.wrapping_shl(36u32) ^ Ega.wrapping_shr(64i32.wrapping_sub(36i32) as u32);
        Eke ^= De;
        BCi = Eke.wrapping_shl(10u32) ^ Eke.wrapping_shr(64i32.wrapping_sub(10i32) as u32);
        Emi ^= Di;
        BCo = Emi.wrapping_shl(15u32) ^ Emi.wrapping_shr(64i32.wrapping_sub(15i32) as u32);
        Eso ^= Do;
        BCu = Eso.wrapping_shl(56u32) ^ Eso.wrapping_shr(64i32.wrapping_sub(56i32) as u32);
        Ama = BCa ^ ! BCe & BCi;
        Ame = BCe ^ ! BCi & BCo;
        Ami = BCi ^ ! BCo & BCu;
        Amo = BCo ^ ! BCu & BCa;
        Amu = BCu ^ ! BCa & BCe;
        Ebi ^= Di;
        BCa = Ebi.wrapping_shl(62u32) ^ Ebi.wrapping_shr(64i32.wrapping_sub(62i32) as u32);
        Ego ^= Do;
        BCe = Ego.wrapping_shl(55u32) ^ Ego.wrapping_shr(64i32.wrapping_sub(55i32) as u32);
        Eku ^= Du;
        BCi = Eku.wrapping_shl(39u32) ^ Eku.wrapping_shr(64i32.wrapping_sub(39i32) as u32);
        Ema ^= Da;
        BCo = Ema.wrapping_shl(41u32) ^ Ema.wrapping_shr(64i32.wrapping_sub(41i32) as u32);
        Ese ^= De;
        BCu = Ese.wrapping_shl(2u32) ^ Ese.wrapping_shr(64i32.wrapping_sub(2i32) as u32);
        Asa = BCa ^ ! BCe & BCi;
        Ase = BCe ^ ! BCi & BCo;
        Asi = BCi ^ ! BCo & BCu;
        Aso = BCo ^ ! BCu & BCa;
        Asu = BCu ^ ! BCa & BCe
      };
      round = round.wrapping_add(2i32)
    }
  };
  state[0usize] = Aba;
  state[1usize] = Abe;
  state[2usize] = Abi;
  state[3usize] = Abo;
  state[4usize] = Abu;
  state[5usize] = Aga;
  state[6usize] = Age;
  state[7usize] = Agi;
  state[8usize] = Ago;
  state[9usize] = Agu;
  state[10usize] = Aka;
  state[11usize] = Ake;
  state[12usize] = Aki;
  state[13usize] = Ako;
  state[14usize] = Aku;
  state[15usize] = Ama;
  state[16usize] = Ame;
  state[17usize] = Ami;
  state[18usize] = Amo;
  state[19usize] = Amu;
  state[20usize] = Asa;
  state[21usize] = Ase;
  state[22usize] = Asi;
  state[23usize] = Aso;
  state[24usize] = Asu
}

pub const KeccakF_RoundConstants: [u64; 24] =
    [1u64, 32898u64, 9223372036854808714u64, 9223372039002292224u64, 32907u64, 2147483649u64,
        9223372039002292353u64, 9223372036854808585u64, 138u64, 136u64, 2147516425u64, 2147483658u64,
        2147516555u64, 9223372036854775947u64, 9223372036854808713u64, 9223372036854808579u64,
        9223372036854808578u64, 9223372036854775936u64, 32778u64, 9223372039002259466u64,
        9223372039002292353u64, 9223372036854808704u64, 2147483649u64, 9223372039002292232u64];

pub fn keccak_absorb(s: &mut [u64], r: u32, mut m: &[u8], mut mlen: u64, p: u8)
{
  let mut i: u64;
  let mut t: [u8; 200] = [0u8; 200usize];
  while
  mlen >= r as u64
  {
    {
      i = 0u64;
      while
      i < r.wrapping_div(8u32) as u64
      {
        s[i as usize] ^= load64(&m[8u64.wrapping_mul(i) as usize..]);
        i = i.wrapping_add(1u64)
      }
    };
    KeccakF1600_StatePermute(s);
    mlen = mlen.wrapping_sub(r as u64);
    m = &m[r as usize..]
  };
  {
    i = 0u64;
    while
    i < r as u64
    {
      t[i as usize] = 0u8;
      i = i.wrapping_add(1u64)
    }
  };
  {
    i = 0u64;
    while
    i < mlen
    {
      t[i as usize] = m[i as usize];
      i = i.wrapping_add(1u64)
    }
  };
  t[i as usize] = p;
  t[r.wrapping_sub(1u32) as usize] = (t[r.wrapping_sub(1u32) as usize] as u32 | 128u32) as u8;
  i = 0u64;
  while
  i < r.wrapping_div(8u32) as u64
  {
    s[i as usize] ^= load64(&t[8u64.wrapping_mul(i) as usize..]);
    i = i.wrapping_add(1u64)
  }
}

pub fn keccak_squeezeblocks(mut h: &mut [u8], mut nblocks: u64, s: &mut [u64], r: u32)
{
  let mut i: u32;
  while
  nblocks > 0u64
  {
    KeccakF1600_StatePermute(s);
    {
      i = 0u32;
      while
      i < r.wrapping_shr(3u32)
      {
        store64(&mut h[8u32.wrapping_mul(i) as usize..], s[i as usize]);
        i = i.wrapping_add(1u32)
      }
    };
    h = &mut h[r as usize..];
    nblocks = nblocks.wrapping_sub(1u64)
  }
}

pub fn load64(x: &[u8]) -> u64
{
  let mut r: u64 = 0u64;
  let mut i: u64;
  {
    i = 0u64;
    while
    i < 8u64
    {
      r |= (x[i as usize] as u64).wrapping_shl(8u64.wrapping_mul(i) as u32);
      i = i.wrapping_add(1u64)
    }
  };
  return r
}

pub fn shake128(mut output: &mut [u8], mut outlen: u64, input: &[u8], inlen: u64)
{
  let mut s: [u64; 25] = [0u64; 25usize];
  let mut t: [u8; 168] = [0u8; 168usize];
  let nblocks: u64 = outlen.wrapping_div(168u64);
  let mut i: usize;
  keccak_absorb(&mut s, 168u32, input, inlen, 31u8);
  keccak_squeezeblocks(output, nblocks, &mut s, 168u32);
  output = &mut output[nblocks.wrapping_mul(168u64) as usize..];
  outlen = outlen.wrapping_sub(nblocks.wrapping_mul(168u64));
  if outlen != 0u64
  {
    keccak_squeezeblocks(&mut t, 1u64, &mut s, 168u32);
    i = 0usize;
    while
    i < outlen as usize
    {
      output[i] = t[i];
      i = i.wrapping_add(1usize)
    }
  }
}

pub fn shake128_absorb(s: &mut [u64], input: &[u8], inputByteLen: u32)
{ keccak_absorb(s, 168u32, input, inputByteLen as u64, 31u8) }

pub fn shake128_squeezeblocks(output: &mut [u8], nblocks: u64, s: &mut [u64])
{ keccak_squeezeblocks(output, nblocks, s, 168u32) }

pub fn shake256(mut output: &mut [u8], mut outlen: u64, input: &[u8], inlen: u64)
{
  let mut s: [u64; 25] = [0u64; 25usize];
  let mut t: [u8; 136] = [0u8; 136usize];
  let nblocks: u64 = outlen.wrapping_div(136u64);
  let mut i: usize;
  {
    i = 0usize;
    while
    i < 25usize
    {
      s[i] = 0u64;
      i = i.wrapping_add(1usize)
    }
  };
  keccak_absorb(&mut s, 136u32, input, inlen, 31u8);
  keccak_squeezeblocks(output, nblocks, &mut s, 136u32);
  output = &mut output[nblocks.wrapping_mul(136u64) as usize..];
  outlen = outlen.wrapping_sub(nblocks.wrapping_mul(136u64));
  if outlen != 0u64
  {
    keccak_squeezeblocks(&mut t, 1u64, &mut s, 136u32);
    i = 0usize;
    while
    i < outlen as usize
    {
      output[i] = t[i];
      i = i.wrapping_add(1usize)
    }
  }
}

pub fn shake256_absorb(s: &mut [u64], input: &[u8], inputByteLen: u32)
{ keccak_absorb(s, 136u32, input, inputByteLen as u64, 31u8) }

pub fn shake256_squeezeblocks(output: &mut [u8], nblocks: u64, s: &mut [u64])
{ keccak_squeezeblocks(output, nblocks, s, 136u32) }

pub fn store64(x: &mut [u8], mut u: u64)
{
  let mut i: u32 = 0u32;
  while
  i < 8u32
  {
    {
      x[i as usize] = u as u8;
      u = u.wrapping_shr(8u32)
    };
    i = i.wrapping_add(1u32)
  }
}
