#include "aarch64.h"

.macro read_cpuid rd
	mrs \rd, mpidr_el1
	and \rd, \rd, #MPIDR_AFF0_MASK
.endm

.global _start
.section .text.head
_start:
	msr daifset, #(DAIF_I_MASK | DAIF_F_MASK | DAIF_A_MASK)
	msr spsel, 1

	/* TODO: check the mode we're booted in */

	ldr x11, =_boot_stack_end
	read_cpuid x10
	lsl x10, x10, #10
	sub x11, x11, x10
	mov sp, x11

	ldr x10, =main
	br x10
