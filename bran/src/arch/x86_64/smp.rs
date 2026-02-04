use core::arch::global_asm;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

// Trampoline symbols
unsafe extern "C" {
    pub static trampoline_start: u8;
    pub static trampoline_end: u8;
}

// Data area in the trampoline (relative to start)
// 0x00: flag (atomic, AP sets to 1 when done)
// 0x08: cr3
// 0x10: stack_top
// 0x18: entry_point
// 0x20: cpu_index
// 0x28: hhdm (optional)

// We define the offsets manually in asm to match these
const TRAMPOLINE_DATA_OFFSET: usize = 0x100; // Place data at offset 0x100
const TRAMPOLINE_SIZE: usize = 4096;

global_asm!(
    r#"
    .section .text
    .intel_syntax noprefix
    .code16
    .global trampoline_start
    .global trampoline_end

trampoline_start:
    cli
    cld
    
    // Set DS = CS = 0x800 (assuming started at 0x8000)
    mov ax, cs
    mov ds, ax
    mov es, ax
    mov ss, ax
    xor sp, sp

    // Fixup GDT base in GDTR
    // GDT linear address = 0x8000 + gdt_offset
    
    .equ gdt_offset, gdt - trampoline_start
    mov eax, 0x8000 + gdt_offset
    
    // Store EAX into GDTR base (at offset 2)
    // Offset = gdtr - trampoline_start
    .equ gdtr_offset, gdtr - trampoline_start
    
    mov bx, gdtr_offset
    mov [bx + 2], eax
    
    // Load GDT
    lgdt [bx]

    // CR0.PE = 1
    mov eax, cr0
    or eax, 1
    mov cr0, eax

    // Jump to protected mode
    // ljmp 0x08, 0x8000 + protected_mode_offset
    
    .equ protected_mode_offset, protected_mode - trampoline_start
    .byte 0x66, 0xea
    .long 0x8000 + protected_mode_offset
    .word 0x08

    .align 16
gdt:
    .quad 0x0000000000000000 // Null
    .quad 0x00cf9a000000ffff // Code 32
    .quad 0x00cf92000000ffff // Data 32
    .quad 0x00af9a000000ffff // Code 64 (Long) - 0x18
    .quad 0x00af92000000ffff // Data 64 - 0x20
gdt_end:

gdtr:
    .word gdt_end - gdt - 1
    .long 0 // filled at runtime

    .code32
protected_mode:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov fs, ax
    mov gs, ax

    // Enable PAE (CR4.PAE = bit 5)
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    // Load CR3 (from data block at 0x8000 + 0x108)
    mov eax, [0x8108]
    mov cr3, eax

    // EFER.LME = 1 (MSR 0xC0000080 bit 8)
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    // CR0.PG = 1 (Bit 31)
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    // Jump to Long Mode
    // ljmp 0x18, 0x8000 + long_mode_offset
    
    .equ long_mode_offset, long_mode - trampoline_start
    .byte 0xea
    .long 0x8000 + long_mode_offset
    .word 0x18

    .code64
long_mode:
    mov ax, 0x20
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov fs, ax
    mov gs, ax

    // Get Stack Top (0x8110)
    mov rsp, [0x8110]
    
    // Get Entry Point (0x8118)
    mov rbx, [0x8118]
    
    // Get CPU Index (0x8120)
    mov rdi, [0x8120]

    // Signal we are done (flag at 0x8100)
    mov rax, 1
    mov [0x8100], rax
    
    // Call entry point
    call rbx

halt_loop:
    hlt
    jmp halt_loop

trampoline_end:
    "#
);

#[repr(C)]
pub struct TrampolineData {
    pub flag: AtomicU64,    // 0x00
    pub cr3: u64,           // 0x08
    pub stack_top: u64,     // 0x10
    pub entry_point: u64,   // 0x18
    pub cpu_index: u64,     // 0x20
}

