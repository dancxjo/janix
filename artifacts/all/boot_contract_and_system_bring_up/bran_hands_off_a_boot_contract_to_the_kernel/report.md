# Scenario: Bran hands off a boot contract to the Kernel

**Architecture**: `all`  
**Feature**: `Boot contract and system bring-up`

[Back to Index](../../../../index.md)

## Steps Summary

| Index | Step | Status | Artifacts |
| :---: | --- | :---: | --- |
| 0 | I boot ThingOS on "x86_64" | ✅ | 📸 📝  |
| 1 | the serial log should contain "bran:" | ⏭️ | 📸 📝  |

## Execution Details

### 1. I boot ThingOS on "x86_64" ✅

![Screenshot](steps/000_i_boot_thingos_on__x86_64_/screen.png)

```

```

---

### 2. the serial log should contain "bran:" 

![Screenshot](steps/001_the_serial_log_should_contain__bran__/screen.png)

```
SYM:: spawn: graph store done
SYM:: spawn: address space handled
SYM:: spawn: finished id=1 name=/boot/modules/sprout stack_top=0xffff800001799000 stack_size=0x10000
SYM:: mapping 7ffe0000->179a000 len 20000
SYM:: configure_ctx: k_stack=0xffff800001799000 u_stack=0x80000000 entry=0x201410
SYM:: configure_ctx: finalized sp=0xffff800001798f18
PROC: spawned
MOD: /boot/modules/bloom
PROC: spawn_kernel_module
SYM:: copy kernel pml4[256]
SYM:: copy kernel pml4[511]
SYM:: mapping 200000->17bd000 len 4000
SYM:: mapping 204000->17c4000 len e000
SYM:: mapping 212000->17d2000 len 1000
SYM:: mapping 213000->17d3000 len 1000
PROC: ELF loaded, entry
SYM:: spawn: name=/boot/modules/bloom start
SYM:: spawn: allocating stack...
SYM:: spawn: stack allocated
SYM:: spawn: graph store start
SYM:: spawn: graph store done
SYM:: spawn: address space handled
SYM:: spawn: finished id=2 name=/boot/modules/bloom stack_top=0xffff8000017e4000 stack_size=0x10000
SYM:: mapping 7ffe0000->17e4000 len 20000
SYM:: configure_ctx: k_stack=0xffff8000017e4000 u_stack=0x80000000 entry=0x205760
SYM:: configure_ctx: finalized sp=0xffff8000017e3f18
PROC: spawned
MOD: /boot/modules/graph_smoke
MOD: /boot/modules/log_smoke
MOD: /boot/modules/cap_fail
MOD: /boot/modules/clock
MOD: /boot/modules/inputd
MOD: /boot/modules/echo
MOD: /boot/modules/inspector
MOD: /boot/modules/logview
SYM:: entering loop
SCHED: calling irq_enable...
SCHED: first tick!
GENERAL PROTECTION FAULT

KERNEL PANIC:
panicked at crates/kernel/src/machine/x86_64/idt.rs:43:5:
GPF
Halting.
GENERAL PROTECTION FAULT

KERNEL PANIC:
panicked at crates/kernel/src/machine/x86_64/idt.rs:43:5:
GPF
Halting.
```

---

