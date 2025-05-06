#ifndef __RANDOM_H__
#define __RANDOM_H__


// Generate random bytes and output the result to random_array
__attribute__((annotate("scylla_opaque")))
__attribute__((annotate("scylla_mutability(mut, _)")))
int randombytes(unsigned char* random_array, unsigned long long nbytes);


#endif
