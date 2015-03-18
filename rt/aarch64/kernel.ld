ENTRY(_start)

MEMORY
{
  RAM (xrw)       : ORIGIN = 0x80080000, LENGTH = 16M
}

SECTIONS
{
  .text :
  {
    . = ALIGN(8);
    *(.text.head)
    *(.text)
    *(.text*)
    *(.rodata)
    *(.rodata*)
    *(.glue_7)
    *(.glue_7t)
    *(.eh_frame)
  } >RAM


   .ARM.extab   : { *(.ARM.extab* .gnu.linkonce.armextab.*) } >RAM
    .ARM : {
    __exidx_start = .;
      *(.ARM.exidx*)
      __exidx_end = .;
    } >RAM

  .data :
  {
    . = ALIGN(8);
    _data_start = .;
    *(.data)
    *(.data*)

    . = ALIGN(8);
    _data_end = .;
  } >RAM

  . = ALIGN(8);
  .bss :
  {
    . = ALIGN(8);
    _bss_start = .;
    _bss_clear_start = .;

    . = ALIGN(64K);
    _pt_start = .;
    . += 64K * 16;
    _pt_end = .;

    . = ALIGN(8);
    *(.bss)
    *(.bss*)
    *(COMMON)

    . = ALIGN(8);
    _bss_clear_end = .;

    . = ALIGN(8);
    _boot_stack_start = .;
    . += 1024;
    _boot_stack_end = .;

    . = ALIGN(8);
    _stack_start = .;
    . += 1024;
    _stack_end = .;
    _bss_end = .;
  } >RAM


  /DISCARD/ :
  {
    libc.a ( * )
    libm.a ( * )
    libgcc.a ( * )
    *(.note)
    *(.note.*)
  }

  .ARM.attributes 0 : { *(.ARM.attributes) }
}
