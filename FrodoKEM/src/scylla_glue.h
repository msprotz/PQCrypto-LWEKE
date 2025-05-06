#include <inttypes.h>

__attribute__((annotate("scylla_opaque")))
__attribute__((annotate("scylla_mutability: (_) -> _")))
static inline const uint8_t *scylla_u8_of_u32(const uint32_t *src) {
  return (const uint8_t *)src;
}

__attribute__((annotate("scylla_opaque")))
__attribute__((annotate("scylla_mutability: (_) -> _")))
static inline const uint8_t *scylla_u8_of_u16(const uint16_t *src) {
  return (const uint8_t *)src;
}

__attribute__((annotate("scylla_opaque")))
__attribute__((annotate("scylla_mutability: (_) -> _")))
static inline const uint16_t *scylla_u16_of_u8(const uint8_t *src) {
  return (const uint16_t *)src;
}

__attribute__((annotate("scylla_opaque")))
__attribute__((annotate("scylla_mutability: (_) -> _")))
static inline const uint32_t *scylla_u32_of_u16(const uint16_t *src) {
  return (const uint32_t *)src;
}

// SCYLLA: this breaks strict aliasing, which the original code presumably did anyhow
__attribute__((annotate("scylla_opaque")))
__attribute__((annotate("scylla_mutability: (_) -> _")))
static inline const uint16_t *scylla_u16_of_u32(const uint32_t *src) {
  return (const uint16_t *)src;
}

// MUTABLE VARIANTS

__attribute__((annotate("scylla_opaque")))
__attribute__((annotate("scylla_mutability: (mut) -> mut")))
static inline uint8_t *scylla_u8_of_u32_mut(uint32_t *src) {
  return (uint8_t *)src;
}

__attribute__((annotate("scylla_opaque")))
__attribute__((annotate("scylla_mutability: (mut) -> mut")))
static inline uint8_t *scylla_u8_of_i16_mut(int16_t *src) {
  return (uint8_t *)src;
}

__attribute__((annotate("scylla_opaque")))
__attribute__((annotate("scylla_mutability: (mut) -> mut")))
static inline uint8_t *scylla_u8_of_u16_mut(uint16_t *src) {
  return (uint8_t *)src;
}

__attribute__((annotate("scylla_opaque")))
__attribute__((annotate("scylla_mutability: (mut) -> mut")))
static inline uint16_t *scylla_u16_of_u8_mut(uint8_t *src) {
  return (uint16_t *)src;
}

__attribute__((annotate("scylla_opaque")))
__attribute__((annotate("scylla_mutability: (mut) -> mut")))
static inline uint32_t *scylla_u32_of_u16_mut(uint16_t *src) {
  return (uint32_t *)src;
}

// SCYLLA: this breaks strict aliasing, which the original code presumably did anyhow
__attribute__((annotate("scylla_opaque")))
__attribute__((annotate("scylla_mutability: (mut) -> mut")))
static inline uint16_t *scylla_u16_of_u32_mut(uint32_t *src) {
  return (uint16_t *)src;
}
