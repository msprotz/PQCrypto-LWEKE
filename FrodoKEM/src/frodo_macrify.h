/********************************************************************************************
* FrodoKEM: Learning with Errors Key Encapsulation
*
* Abstract: header for internal functions
*********************************************************************************************/

#ifndef _FRODO_MACRIFY_H_
#define _FRODO_MACRIFY_H_

#include <stddef.h>
#include <stdint.h>
#include "config.h"
#include <inttypes.h>


void frodo_pack(unsigned char *out, const size_t outlen, const uint16_t *in, const size_t inlen, const unsigned char lsb);
void frodo_unpack(uint16_t *out, const size_t outlen, const unsigned char *in, const size_t inlen, const unsigned char lsb);
void frodo_sample_n(uint16_t *s, const size_t n);
int8_t ct_verify(const uint16_t *a, const uint16_t *b, size_t len);
void ct_select(uint8_t *r, const uint8_t *a, const uint8_t *b, size_t len, int8_t selector);
void clear_bytes(uint8_t *mem, size_t n);

int frodo_mul_add_as_plus_e(uint16_t *b, const uint16_t *s, const uint16_t *e, const uint8_t *seed_A);
int frodo_mul_add_sa_plus_e(uint16_t *b, const uint16_t *s, uint16_t *e, const uint8_t *seed_A);
void frodo_mul_add_sb_plus_e(uint16_t *out, const uint16_t *b, const uint16_t *s, const uint16_t *e);
void frodo_mul_bs(uint16_t *out, const uint16_t *b, const uint16_t *s);

void frodo_add(uint16_t *out, const uint16_t *a, const uint16_t *b);
void frodo_sub(uint16_t *out, const uint16_t *a, const uint16_t *b);
void frodo_add_inplace(uint16_t *out, const uint16_t *a);
void frodo_sub_inplace(uint16_t *out, const uint16_t *a);
void frodo_key_encode(uint16_t *out, const uint16_t *in);
void frodo_key_decode(uint16_t *out, const uint16_t *in);

#endif
