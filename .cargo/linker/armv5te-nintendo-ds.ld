OUTPUT_FORMAT("elf32-littlearm")
OUTPUT_ARCH(arm)
ENTRY(_start)

MEMORY {
	EWRAM (rwx) : ORIGIN = 0x02000000, LENGTH = 4M - 512k
	DTCM  (rw)  : ORIGIN = 0x02FF0000, LENGTH = 16K
	ITCM  (rx)  : ORIGIN = 0x01000000, LENGTH = 32K
}

PHDRS {
    main PT_LOAD FLAGS(7);
    dtcm PT_LOAD FLAGS(7);
    itcm PT_LOAD FLAGS(7);
}

PROVIDE(__dtcm_data_size = 0);
PROVIDE(__dtcm_reserved_size = 0x40);
PROVIDE(__svc_stack_size = 0x100);
PROVIDE(__irq_stack_size = 0x100);

SECTIONS {
    __dtcm_top   = ORIGIN(dtcm) + LENGTH(dtcm);
    __sp_svc = __dtcm_top - __dtcm_reserved_size;
    __sp_irq = __sp_svc   - __svc_stack_size;
    __sp_usr = __sp_irq   - __irq_stack_size - __dtcm_data_size;

    .text : {
        *(.text .text.*)
        . = ALIGN(4);
    } >EWRAM :main

    .rodata : {
        *(.rodata .rodata.*)
        . = ALIGN(4);
    } >EWRAM :main

    .data ALIGN(4) : {
        __data_start = ABSOLUTE(.);
        *(.data .data.*)
        CONSTRUCTORS
        . = ALIGN(4);
    } >EWRAM :main

    .preinit_array ALIGN(4) : {
        PROVIDE_HIDDEN(__preinit_array_start = .);
        KEEP(*(.preinit_array))
        PROVIDE_HIDDEN(__preinit_array_end = .);
        PROVIDE_HIDDEN(__preinit_array_size = __preinit_array_end - __preinit_array_start);
    } >EWRAM :main

    .init_array ALIGN(4) : {
        PROVIDE_HIDDEN(__init_array_start = .);
        KEEP(*(SORT_BY_INIT_PRIORITY(.init_array.*)))
        KEEP(*(.init_array*))
        PROVIDE_HIDDEN(__init_array_end = .);
        PROVIDE_HIDDEN(__init_array_size = __init_array_end - __init_array_start);
    } >EWRAM :main

    .fini : { KEEP(*(.fini)) } >EWRAM :main
    .fini_array ALIGN(4) : {
        PROVIDE_HIDDEN(__fini_array_start = .);
        KEEP(*(SORT_BY_INIT_PRIORITY(.fini_array.*)))
        KEEP(*(.fini_array*))
        PROVIDE_HIDDEN(__fini_array_end = .);
        PROVIDE_HIDDEN(__fini_array_size = __fini_array_end - __fini_array_start);
    } >EWRAM :main

    .bss (NOLOAD) : {
        PROVIDE_HIDDEN(__bss_start = ABSOLUTE(.));
        *(.dynbss)
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(4);
        PROVIDE_HIDDEN(__bss_end = ABSOLUTE(.));
        PROVIDE_HIDDEN(__bss_size = __bss_end - __bss_start);
    } >EWRAM :NONE

    .noinit (NOLOAD) : {
        __noinit_start = ABSOLUTE(.);
        *(.noinit .noinit.*)
        . = ALIGN(4);
        __noinit_end = ABSOLUTE(.);
    } >EWRAM :NONE

    .got : { *(.got.plt) *(.got) } >EWRAM :main

    .ARM.attributes 0 : { KEEP(*(.ARM.attributes)) }
    .ARM.extab : { *(.ARM.extab*) } >EWRAM :main
    .ARM.exidx : {
        __exidx_start = .;
        *(.ARM.exidx*)
        __exidx_end = .;
    } >EWRAM :main
}

SECTIONS {
    .dtcm : {
        __dtcm_start = ORIGIN(dtcm);
        *(.dtcm .dtcm.*)
        . = ALIGN(4);
        __dtcm_end = ABSOLUTE(.);
    } >DTCM AT >EWRAM :dtcm

    .sbss __dtcm_end (NOLOAD) : {
        PROVIDE_HIDDEN(__sbss_start = ABSOLUTE(.));
        *(.dynsbss)
        *(.sbss .sbss.*)
        *(scommon)
        . = ALIGN(4);
        PROVIDE_HIDDEN(__sbss_end = ABSOLUTE(.));
        PROVIDE_HIDDEN(__sbss_size = __sbss_end - __sbss_start);
    } >DTCM :NONE
}

SECTIONS {
    .itcm : {
        KEEP(*(.vectors .vectors.*))
        *(.itcm .itcm.*)
        . = ALIGN(4);
    } >ITCM AT >EWRAM :itcm
}