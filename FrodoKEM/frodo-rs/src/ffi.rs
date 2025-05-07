const CRYPTO_SECRETKEYBYTES: usize =  19888;    // sizeof(s) + CRYPTO_PUBLICKEYBYTES + 2*PARAMS_N*PARAMS_NBAR + BYTES_PKHASH;
const CRYPTO_PUBLICKEYBYTES: usize =   9616;    // sizeof(seed_A) + (PARAMS_LOGQ*PARAMS_N*PARAMS_NBAR)/8;
const CRYPTO_BYTES: usize =              16;
const CRYPTO_CIPHERTEXTBYTES: usize =  9752;    // (PARAMS_LOGQ*PARAMS_N*PARAMS_NBAR)/8 + (PARAMS_LOGQ*PARAMS_NBAR*PARAMS_NBAR)/8 + BYTES_SALT;
const PARAMS_N: usize = 640;
const PARAMS_NBAR: usize = 8;
const PARAMS_LOGQ: usize = 15;
const PARAMS_Q: usize = 1 << PARAMS_LOGQ;
const PARAMS_EXTRACTED_BITS: usize = 2;
const PARAMS_STRIPE_STEP: usize = 8;
const PARAMS_PARALLEL: usize = 4;
const BYTES_SEED_A: usize = 16;
const BYTES_MU: usize = (PARAMS_EXTRACTED_BITS*PARAMS_NBAR*PARAMS_NBAR)/8;
const BYTES_SALT: usize = 2*CRYPTO_BYTES;
const BYTES_SEED_SE: usize = 2*CRYPTO_BYTES;
const BYTES_PKHASH: usize = CRYPTO_BYTES;

// FrodoKEM's key generation
// Outputs: public key pk = pk_seedA||pk_b                      (               BYTES_SEED_A + (PARAMS_LOGQ*PARAMS_N*PARAMS_NBAR)/8 bytes)
//          secret key sk = sk_s||pk_seedA||pk_b||sk_S||sk_pkh  (CRYPTO_BYTES + BYTES_SEED_A + (PARAMS_LOGQ*PARAMS_N*PARAMS_NBAR)/8 + 2*PARAMS_N*PARAMS_NBAR + BYTES_PKHASH bytes)
#[no_mangle]
pub extern "C" fn crypto_kem_keypair_Frodo640(pk: *mut u8, sk: *mut u8) -> i32
{
    unsafe {
        let pk = std::slice::from_raw_parts_mut(pk, BYTES_SEED_A + (PARAMS_LOGQ*PARAMS_N*PARAMS_NBAR)/8);
        let sk = std::slice::from_raw_parts_mut(sk, (CRYPTO_BYTES + BYTES_SEED_A + (PARAMS_LOGQ*PARAMS_N*PARAMS_NBAR)/8 + 2*PARAMS_N*PARAMS_NBAR + BYTES_PKHASH));
        crate::frodo640::crypto_kem_keypair_Frodo640(pk, sk)
    }
}

// FrodoKEM's key encapsulation
// Input:   public key pk = pk_seedA||pk_b      (BYTES_SEED_A + (PARAMS_LOGQ*PARAMS_N*PARAMS_NBAR)/8 bytes)
// Outputs: ciphertext ct = ct_c1||ct_c2||salt  (               (PARAMS_LOGQ*PARAMS_N*PARAMS_NBAR)/8 + (PARAMS_LOGQ*PARAMS_NBAR*PARAMS_NBAR)/8 + BYTES_SALT bytes)
//          shared key ss                       (CRYPTO_BYTES bytes)
#[no_mangle]
pub extern "C" fn crypto_kem_enc_Frodo640(ct: *mut u8, ss: *mut u8, pk: *const u8) -> i32
{
    unsafe {
        let pk = std::slice::from_raw_parts(pk, (BYTES_SEED_A + (PARAMS_LOGQ*PARAMS_N*PARAMS_NBAR)/8));
        let ct = std::slice::from_raw_parts_mut(ct, (               (PARAMS_LOGQ*PARAMS_N*PARAMS_NBAR)/8 + (PARAMS_LOGQ*PARAMS_NBAR*PARAMS_NBAR)/8 + BYTES_SALT));
        let ss = std::slice::from_raw_parts_mut(ss, (CRYPTO_BYTES));
        crate::frodo640::crypto_kem_enc_Frodo640(ct, ss, pk)
    }
}

// FrodoKEM's key decapsulation
// Inputs: ciphertext ct = ct_c1||ct_c2||salt                  (                              (PARAMS_LOGQ*PARAMS_N*PARAMS_NBAR)/8 + (PARAMS_LOGQ*PARAMS_NBAR*PARAMS_NBAR)/8 + BYTES_SALT bytes)
//         secret key sk = sk_s||pk_seedA||pk_b||sk_S||sk_pkh  (CRYPTO_BYTES + BYTES_SEED_A + (PARAMS_LOGQ*PARAMS_N*PARAMS_NBAR)/8 + 2*PARAMS_N*PARAMS_NBAR + BYTES_PKHASH bytes)
// Output: shared key ss                                       (CRYPTO_BYTES bytes)
#[no_mangle]
pub extern "C" fn crypto_kem_dec_Frodo640(ss: *mut u8, ct: *const u8, sk: *const u8) -> i32
{
    unsafe {
        let ss = std::slice::from_raw_parts_mut(ss, CRYPTO_BYTES);
        let ct = std::slice::from_raw_parts(ct, (                              (PARAMS_LOGQ*PARAMS_N*PARAMS_NBAR)/8 + (PARAMS_LOGQ*PARAMS_NBAR*PARAMS_NBAR)/8 + BYTES_SALT));
        let sk = std::slice::from_raw_parts(sk, (CRYPTO_BYTES + BYTES_SEED_A + (PARAMS_LOGQ*PARAMS_N*PARAMS_NBAR)/8 + 2*PARAMS_N*PARAMS_NBAR + BYTES_PKHASH));
        crate::frodo640::crypto_kem_dec_Frodo640(ss, ct, sk)
    }
}
