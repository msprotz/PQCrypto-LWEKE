/********************************************************************************************
* FrodoKEM: Learning with Errors Key Encapsulation
*
* Abstract: parameters and API for FrodoKEM-640
*********************************************************************************************/

#ifndef _API_Frodo640_H_
#define _API_Frodo640_H_


#define CRYPTO_SECRETKEYBYTES  19888U    // sizeof(s) + CRYPTO_PUBLICKEYBYTES + 2*PARAMS_N*PARAMS_NBAR + BYTES_PKHASH
#define CRYPTO_PUBLICKEYBYTES   9616U    // sizeof(seed_A) + (PARAMS_LOGQ*PARAMS_N*PARAMS_NBAR)/8
#define CRYPTO_BYTES              16U
#define CRYPTO_CIPHERTEXTBYTES  9752U    // (PARAMS_LOGQ*PARAMS_N*PARAMS_NBAR)/8 + (PARAMS_LOGQ*PARAMS_NBAR*PARAMS_NBAR)/8 + BYTES_SALT

// Algorithm name
#define CRYPTO_ALGNAME "FrodoKEM-640"


int crypto_kem_keypair_Frodo640(unsigned char *pk, unsigned char *sk);
int crypto_kem_enc_Frodo640(unsigned char *ct, unsigned char *ss, const unsigned char *pk);
int crypto_kem_dec_Frodo640(unsigned char *ss, const unsigned char *ct, const unsigned char *sk);


#endif
