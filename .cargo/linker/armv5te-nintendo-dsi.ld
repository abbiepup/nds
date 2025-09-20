OUTPUT_FORMAT("elf32-littlearm")
OUTPUT_ARCH(arm)
ENTRY(_start)

MEMORY {
	EWRAM (rwx) : ORIGIN = 0x02000000, LENGTH = 4M - 512k
	DTCM  (rw)  : ORIGIN = 0x02FF0000, LENGTH = 16K
	ITCM  (rx)  : ORIGIN = 0x01000000, LENGTH = 32K
}

SECTIONS {
    .itcm : {
        PROVIDE (__itcm_start = ORIGIN(DTCM));
        *(.itcm .itcm.*)
        . = ALIGN(4);
    } > ITCM AT> EWRAM

    .text : {
        *(.text .text.*)
        . = ALIGN(4);
    } > EWRAM

    .rodata : {
        *(.rodata .rodata.*)
        . = ALIGN(4);
    } > EWRAM

    .dtcm : {
        PROVIDE (__dtcm_start = ORIGIN(DTCM));
        __dtcm_top = ORIGIN(DTCM) + LENGTH(DTCM);
        *(.dtcm .dtcm.*)
        . = ALIGN(4);
    } > DTCM AT> EWRAM

    .data : {
        *(.data .data.*)
        . = ALIGN(4);
    } > EWRAM

    .bss (NOLOAD) : {
        PROVIDE (__bss_start = .);
        *(.dynbss)
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(4);
        PROVIDE (__bss_end = .);
        PROVIDE (__bss_size = __bss_end - __bss_start);
    } > EWRAM

    .ARM.extab : { *(.ARM.extab*) } > EWRAM
    .ARM.exidx : {
        PROVIDE_HIDDEN (__exidx_start = .);
        *(.ARM.exidx*) 
        PROVIDE_HIDDEN (__exidx_end = .);
    } > EWRAM

    .stack (NOLOAD) : {
        PROVIDE (__stack_size = 0x100);
        __sp_irq = __dtcm_top;
        __sp_svc = __sp_irq - __stack_size;
        __sp_usr = __sp_svc - __stack_size;
    } > DTCM

    .ARM.attributes 0 : { KEEP (*(.ARM.attributes)) }
}