/* Licensed under the Apache License, Version 2.0 or the MIT License. */
/* SPDX-License-Identifier: Apache-2.0 OR MIT                         */
/* Copyright Tock Contributors 2024.                                  */

MEMORY
{
  /* Divide flash region into 2 sections(rom, prog) */
  /* [Originally] flash : ORIGIN = 0x00000000, LENGTH = 512K*/
  rom (rx)  : ORIGIN = 0x00000410, LENGTH = 256K

  /* For TockOS Applications */
  prog (rx) : ORIGIN = 0x00040410, LENGTH = 256K

  /* SRAM*/
  ram (rwx) : ORIGIN = 0x20000000, LENGTH = 64K
}

ENTRY(_start) /* to get to main */

SECTIONS {
    .vector_table ORIGIN(rom) : {
    KEEP(*(.vectors))

    /* 초기 Stack Pointer */
    LONG(ORIGIN(ram) + LENGTH(ram))

    /* Reset Handler (Thumb 모드 적용) */
    LONG(_start)
  }
}

INCLUDE tock_kernel_layout.ld
