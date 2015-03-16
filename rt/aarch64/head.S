#include "aarch64.h"

.macro read_cpuid rd
	mrs \rd, mpidr_el1
	and \rd, \rd, #MPIDR_AFF0_MASK
.endm

.global _start
.section .text.head
_start:
	read_cpuid x10

	ldr x10, =_stack_end
	mov sp, x10

	ldr x10, =main
	br x10
