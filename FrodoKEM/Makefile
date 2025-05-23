####  Makefile for compilation on Unix-like operative systems  ####

CC=gcc
ifeq "$(CC)" "gcc"
    COMPILER=gcc
else ifeq "$(CC)" "clang"
    COMPILER=clang
endif

ARCHITECTURE=_AMD64_
ifeq "$(ARCH)" "x64"
    ARCHITECTURE=_AMD64_
else ifeq "$(ARCH)" "x86"
    ARCHITECTURE=_X86_
    USE_OPT_LEVEL=_FAST_GENERIC_
else ifeq "$(ARCH)" "ARM"
    ARCHITECTURE=_ARM_
    USE_OPT_LEVEL=_FAST_GENERIC_
else ifeq "$(ARCH)" "PPC"
    ARCHITECTURE=_PPC_
    USE_OPT_LEVEL=_REFERENCE_
else ifeq "$(ARCH)" "s390x"
    ARCHITECTURE=_S390X_
    USE_OPT_LEVEL=_REFERENCE_
endif

ifeq "$(ARCHITECTURE)" "_AMD64_"
    USE_OPT_LEVEL=_FAST_
endif

ifeq "$(OPT_LEVEL)" "REFERENCE"
    USE_OPT_LEVEL=_REFERENCE_
else ifeq "$(OPT_LEVEL)" "FAST_GENERIC"
    USE_OPT_LEVEL=_FAST_GENERIC_
else ifeq "$(OPT_LEVEL)" "FAST"
    ifeq "$(ARCHITECTURE)" "_AMD64_"
        USE_OPT_LEVEL=_FAST_
    endif
endif

USE_GENERATION_A=_AES128_FOR_A_
ifeq "$(GENERATION_A)" "AES128"
    USE_GENERATION_A=_AES128_FOR_A_
else ifeq "$(GENERATION_A)" "SHAKE128"
    USE_GENERATION_A=_SHAKE128_FOR_A_
endif

ifeq "$(ARCH)" "ARM"
    ARM_SETTING=#-lrt
endif

USING_OPENSSL=_USE_OPENSSL_
ifeq "$(USE_OPENSSL)" "FALSE"
    USING_OPENSSL=NO_OPENSSL
endif

OPENSSL_INCLUDE_DIR?=/usr/include
OPENSSL_LIB_DIR?=/usr/lib

AR=ar rcs
RANLIB=ranlib
LN=ln -s

VALGRIND_CFLAGS=
ifeq "$(DO_VALGRIND_CHECK)" "TRUE"
VALGRIND_CFLAGS= -g -O0 -DDO_VALGRIND_CHECK
endif

ifeq "$(EXTRA_CFLAGS)" ""
CFLAGS= -O3 
else
CFLAGS= $(EXTRA_CFLAGS)
endif
CFLAGS+= $(VALGRIND_CFLAGS)
CFLAGS+= -std=gnu11 -Wall -Wextra -DNIX -D $(ARCHITECTURE) -D $(USE_OPT_LEVEL) -D $(USE_GENERATION_A) -D $(USING_OPENSSL)
ifeq "$(CC)" "gcc"
ifneq "$(ARCHITECTURE)" "_PPC_"
ifneq "$(ARCHITECTURE)" "_S390X_"
CFLAGS+= -march=native
endif
endif
endif

ifeq "$(USE_OPENSSL)" "FALSE"
LDFLAGS=-lm
else
CFLAGS+= -I$(OPENSSL_INCLUDE_DIR)
LDFLAGS=-lm -L$(OPENSSL_LIB_DIR) -lssl -lcrypto
endif

ifeq "$(ARCHITECTURE)" "_AMD64_"
ifeq "$(USE_OPT_LEVEL)" "_FAST_"
CFLAGS += -mavx2 -maes -msse2
endif
endif

.PHONY: all check clean prettyprint

all: lib640 lib976 lib1344 tests KATS

objs/%.o: src/%.c
	@mkdir -p $(@D)
	$(CC) -c  $(CFLAGS) $< -o $@

objs/frodo640.o: src/frodo640.c
	@mkdir -p $(@D)
	$(CC) -c  $(CFLAGS) $< -o $@

objs/frodo976.o: src/frodo976.c
	@mkdir -p $(@D)
	$(CC) -c  $(CFLAGS) $< -o $@

objs/frodo1344.o: src/frodo1344.c
	@mkdir -p $(@D)
	$(CC) -c  $(CFLAGS) $< -o $@

# RAND
objs/random/random.o: ../common/random/random.c ../common/random/random.h
	@mkdir -p $(@D)
	$(CC) -c $(CFLAGS) ../common/random/random.c -o objs/random/random.o
RAND_OBJS := objs/random/random.o

# KEM_FRODO
KEM_FRODO640_OBJS := $(addprefix objs/, frodo640.o util.o)
KEM_FRODO640_HEADERS := $(addprefix src/, api_frodo640.h config.h frodo_macrify.h)
$(KEM_FRODO640_OBJS): $(KEM_FRODO640_HEADERS) $(addprefix src/, kem.c noise.c util.c)

KEM_FRODO976_OBJS := $(addprefix objs/, frodo976.o util.o)
KEM_FRODO976_HEADERS := $(addprefix src/, api_frodo976.h config.h frodo_macrify.h)
$(KEM_FRODO976_OBJS): $(KEM_FRODO976_HEADERS) $(addprefix src/, kem.c noise.c util.c)

KEM_FRODO1344_OBJS := $(addprefix objs/, frodo1344.o util.o)
KEM_FRODO1344_HEADERS := $(addprefix src/, api_frodo1344.h config.h frodo_macrify.h)
$(KEM_FRODO1344_OBJS): $(KEM_FRODO1344_HEADERS) $(addprefix src/, kem.c noise.c util.c)

# AES
ifeq "$(USE_OPENSSL)" "FALSE" 
ifeq "$(USE_OPT_LEVEL)" "_FAST_"
# AES_NI
objs/aes/aes_ni.o: ../common/aes/aes_ni.c ../common/aes/aes.h
	@mkdir -p $(@D)
	$(CC) -c $(CFLAGS) ../common/aes/aes_ni.c -o objs/aes/aes_ni.o
AES_OBJS := objs/aes/aes_ni.o
else
objs/aes/aes_c.o: ../common/aes/aes_c.c ../common/aes/aes.h
	@mkdir -p $(@D)
	$(CC) -c $(CFLAGS) ../common/aes/aes_c.c -o objs/aes/aes_c.o
AES_OBJS := objs/aes/aes_c.o
endif
else
objs/aes/aes_openssl.o: ../common/aes/aes_openssl.c ../common/aes/aes_openssl.h
	@mkdir -p $(@D)
	$(CC) -c $(CFLAGS) ../common/aes/aes_openssl.c -o objs/aes/aes_openssl.o
AES_OBJS := objs/aes/aes_openssl.o
endif

# SHAKE
objs/sha3/fips202.o: ../common/sha3/fips202.c ../common/sha3/fips202.h
	@mkdir -p $(@D)
	$(CC) -c $(CFLAGS) ../common/sha3/fips202.c -o objs/sha3/fips202.o
SHAKE_OBJS := objs/sha3/fips202.o

ifeq "$(USE_OPT_LEVEL)" "_FAST_"
ifeq "$(GENERATION_A)" "SHAKE128"
# SHAKEx4
objs/sha3/fips202x4.o: ../common/sha3/fips202x4.c ../common/sha3/fips202x4.h
	@mkdir -p $(@D)
	$(CC) -c $(CFLAGS) ../common/sha3/fips202x4.c -o objs/sha3/fips202x4.o

objs/sha3/keccak4x/KeccakP-1600-times4-SIMD256.o: ../common/sha3/keccak4x/KeccakP-1600-times4-SIMD256.c ../common/sha3/keccak4x/KeccakP-1600-times4-SnP.h
	@mkdir -p $(@D)
	$(CC) -c $(CFLAGS) ../common/sha3/keccak4x/KeccakP-1600-times4-SIMD256.c -o objs/sha3/keccak4x/KeccakP-1600-times4-SIMD256.o

SHAKEx4_OBJS := objs/sha3/fips202x4.o objs/sha3/keccak4x/KeccakP-1600-times4-SIMD256.o
endif
endif

lib640: $(KEM_FRODO640_OBJS) $(RAND_OBJS) $(AES_OBJS) $(SHAKE_OBJS) $(SHAKEx4_OBJS)
	rm -rf frodo640
	mkdir frodo640
	$(AR) frodo640/libfrodo.a $^
	$(RANLIB) frodo640/libfrodo.a

lib976: $(KEM_FRODO976_OBJS) $(RAND_OBJS) $(AES_OBJS) $(SHAKE_OBJS) $(SHAKEx4_OBJS)
	rm -rf frodo976
	mkdir frodo976
	$(AR) frodo976/libfrodo.a $^
	$(RANLIB) frodo976/libfrodo.a

lib1344: $(KEM_FRODO1344_OBJS) $(RAND_OBJS) $(AES_OBJS) $(SHAKE_OBJS) $(SHAKEx4_OBJS)
	rm -rf frodo1344
	mkdir frodo1344
	$(AR) frodo1344/libfrodo.a $^
	$(RANLIB) frodo1344/libfrodo.a

tests: lib640 lib976 lib1344 tests/ds_benchmark.h
	$(CC) $(CFLAGS) -L./frodo640 tests/test_KEM640.c -lfrodo $(LDFLAGS) -o frodo640/test_KEM $(ARM_SETTING)
	$(CC) $(CFLAGS) -L./frodo976 tests/test_KEM976.c -lfrodo $(LDFLAGS) -o frodo976/test_KEM $(ARM_SETTING)
	$(CC) $(CFLAGS) -L./frodo1344 tests/test_KEM1344.c -lfrodo $(LDFLAGS) -o frodo1344/test_KEM $(ARM_SETTING)

lib640_for_KATs: $(KEM_FRODO640_OBJS) $(AES_OBJS) $(SHAKE_OBJS) $(SHAKEx4_OBJS)
	$(AR) frodo640/libfrodo_for_testing.a $^
	$(RANLIB) frodo640/libfrodo_for_testing.a

lib976_for_KATs: $(KEM_FRODO976_OBJS) $(AES_OBJS) $(SHAKE_OBJS) $(SHAKEx4_OBJS)
	$(AR) frodo976/libfrodo_for_testing.a $^
	$(RANLIB) frodo976/libfrodo_for_testing.a

lib1344_for_KATs: $(KEM_FRODO1344_OBJS) $(AES_OBJS) $(SHAKE_OBJS) $(SHAKEx4_OBJS)
	$(AR) frodo1344/libfrodo_for_testing.a $^
	$(RANLIB) frodo1344/libfrodo_for_testing.a

KATS: lib640_for_KATs lib976_for_KATs lib1344_for_KATs
ifeq "$(GENERATION_A)" "SHAKE128"
	$(CC) $(CFLAGS) -L./frodo640 tests/PQCtestKAT_kem640_shake.c tests/rng.c -lfrodo_for_testing $(LDFLAGS) -o frodo640/PQCtestKAT_kem_shake $(ARM_SETTING)
	$(CC) $(CFLAGS) -L./frodo976 tests/PQCtestKAT_kem976_shake.c tests/rng.c -lfrodo_for_testing $(LDFLAGS) -o frodo976/PQCtestKAT_kem_shake $(ARM_SETTING)
	$(CC) $(CFLAGS) -L./frodo1344 tests/PQCtestKAT_kem1344_shake.c tests/rng.c -lfrodo_for_testing $(LDFLAGS) -o frodo1344/PQCtestKAT_kem_shake $(ARM_SETTING)
else
	$(CC) $(CFLAGS) -L./frodo640 tests/PQCtestKAT_kem640.c tests/rng.c -lfrodo_for_testing $(LDFLAGS) -o frodo640/PQCtestKAT_kem $(ARM_SETTING)
	$(CC) $(CFLAGS) -L./frodo976 tests/PQCtestKAT_kem976.c tests/rng.c -lfrodo_for_testing $(LDFLAGS) -o frodo976/PQCtestKAT_kem $(ARM_SETTING)
	$(CC) $(CFLAGS) -L./frodo1344 tests/PQCtestKAT_kem1344.c tests/rng.c -lfrodo_for_testing $(LDFLAGS) -o frodo1344/PQCtestKAT_kem $(ARM_SETTING)
endif

check: tests

test640:
ifeq "$(DO_VALGRIND_CHECK)" "TRUE"
	valgrind --tool=memcheck --error-exitcode=1 --max-stackframe=20480000 frodo640/test_KEM
else
	frodo640/test_KEM
endif

test976:
ifeq "$(DO_VALGRIND_CHECK)" "TRUE"
	valgrind --tool=memcheck --error-exitcode=1 --max-stackframe=20480000 frodo976/test_KEM
else
	frodo976/test_KEM
endif

test1344:
ifeq "$(DO_VALGRIND_CHECK)" "TRUE"
	valgrind --tool=memcheck --error-exitcode=1 --max-stackframe=20480000 frodo1344/test_KEM
else
	frodo1344/test_KEM
endif

clean:
	rm -rf objs *.req frodo640 frodo976 frodo1344
	find . -name .DS_Store -type f -delete

prettyprint:
	astyle --style=java --indent=tab --pad-header --pad-oper --align-pointer=name --align-reference=name --suffix=none src/*.h src/*/*.h src/*/*.c

tests-rs: frodo640/test_KEM-rs
	RUST_BACKTRACE=1 $<

frodo640/test_KEM-rs: tests/test_KEM640.c frodo-rs/target/release/libfrodo_rs.a objs/aes/aes_c.o objs/random/random.o objs/sha3/fips202.o
	mkdir -p $(@D)
	$(CC) $(CFLAGS) $^ -lm -o $@

frodo-rs/target/release/libfrodo_rs.a: $(wildcard frodo-rs/src/*)
	cd frodo-rs && cargo build --release

SCYLLA_HOME ?= ../../scylla

FRODO_RS_INPUTS=src/util.c src/frodo640.c ../common/sha3/fips202.c 

comma:= ,
empty:=
space:= $(empty) $(empty)

# Because of the various #includes, add a bunch of stuff as dependencies
frodo-rs/src/frodo640.rs: $(FRODO_RS_INPUTS) $(wildcard src/*.c) $(wildcard src/*.h)
	$(SCYLLA_HOME)/scylla --ccopts -DSCYLLA,-I../common/sha3, --ccopts $(subst $(space),$(comma),$(CFLAGS)) $(FRODO_RS_INPUTS) --errors_as_warnings --ignore_lib_errors --output frodo-rs/src/
