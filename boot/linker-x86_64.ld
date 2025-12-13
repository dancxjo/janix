/* Tell the linker that we want an x86_64 ELF64 output file */
OUTPUT_FORMAT(elf64-x86-64)

/* We want the symbol kmain to be our entry point */
ENTRY(kmain)

/* Program headers. You can add FLAGS(...) if you want strict RWX,
   but leaving them default is fine during bring-up. */
PHDRS
{
    text    PT_LOAD;
    rodata  PT_LOAD;
    data    PT_LOAD;
}

SECTIONS
{
    /* Place in the topmost 2GiB of the address space (Limine-friendly) */
    . = 0xffffffff80000000;

    .text : {
        *(.text .text.*)
    } :text

    /* Move to the next memory page for .rodata */
    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .rodata : {
        *(.rodata .rodata.*)
    } :rodata

    /* Move to the next memory page for .data */
    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .data : {
        *(.data .data.*)
    } :data

    /* ---- Limine requests: keep them even under LTO + --gc-sections ----
       Put them in a dedicated output section (still part of :data mapping).
       The wildcard forms matter: .requests.* is common. */
    .limine_requests : ALIGN(8) {
        KEEP(*(.requests_start_marker))
        KEEP(*(.requests_start_marker.*))

        KEEP(*(.requests))
        KEEP(*(.requests.*))

        KEEP(*(.requests_end_marker))
        KEEP(*(.requests_end_marker.*))
    } :data

    /* NOTE: .bss needs to be last in :data to avoid writing zeros to the binary */
    .bss : {
        *(.bss .bss.*)
        *(COMMON)
    } :data

    _end = .;

    /DISCARD/ : {
        *(.eh_frame*)
        *(.note .note.*)
    }
}
