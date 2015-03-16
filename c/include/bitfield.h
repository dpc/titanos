#ifndef __BITFIELD_H__
#define __BITFIELD_H__

#if !defined(__ASSEMBLY__)

#include <stdint.h>

#define DEF_BITFIELD32(NAME, M, N) \
enum { NAME ## _SHIFT = (N) }; \
enum { NAME ## _BITS = ( (1UL << (1UL + M - N)) - 1UL) }; \
static inline uint32_t NAME(uint32_t x) { \
	return (x >> NAME ## _SHIFT) & (NAME ## _BITS); \
} \
enum { NAME ## _MASK = ( (NAME ## _BITS) << (NAME ## _SHIFT)) }

#define DEF_BITFIELD64(NAME, M, N) \
enum { NAME ## _SHIFT = (N) }; \
enum { NAME ## _BITS = ( (1ULL << (1ULL + M - N)) - 1ULL) }; \
static inline uint64_t NAME(uint64_t x) { \
	return (x >> NAME ## _SHIFT) & (NAME ## _BITS); \
} \
enum { NAME ## _MASK = ( (uint64_t)(NAME ## _BITS) << (NAME ## _SHIFT)) }

#else /* __ASSEMBLY__ */

#define DEF_BITFIELD32(NAME, M, N) \
.equ NAME ## _SHIFT, N ; \
.equ NAME ## _BITS,  ((1 << (M + 1 - N)) - 1); \
.equ NAME ## _MASK, (((1 << (M + 1 - N)) - 1) << N)

#define DEF_BITFIELD64(NAME, M, N) \
.equ NAME ## _SHIFT, N ; \
.equ NAME ## _BITS,  ((1 << (M + 1 - N)) - M) ; \
.equ NAME ## _MASK, (((1 << (M + 1 - N)) - 1) << N)

#endif

#endif
