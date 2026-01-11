ENTRY(_start)
SECTIONS {
    . = 0x200000;
    .text : {
        *(.text.entry)
        *(.text*)
    }
    .rodata : { *(.rodata*) }
    .data : { *(.data*) }
    .bss : { 
        *(.bss*)
        *(COMMON)
    }
}
