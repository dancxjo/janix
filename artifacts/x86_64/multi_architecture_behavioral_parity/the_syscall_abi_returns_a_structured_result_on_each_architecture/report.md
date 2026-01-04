# Scenario: The syscall ABI returns a structured result on each architecture

**Architecture**: `x86_64`  
**Feature**: `Multi-architecture behavioral parity`

[Back to Index](../../../../index.md)

## Steps Summary

| Index | Step | Status | Artifacts |
| :---: | --- | :---: | --- |
| 0 | I boot ThingOS on "x86_64" | ✅ | 📸 📝  |
| 1 | a user task calls syscall "sys_log" | ⏭️ | 📸 📝  |

## Execution Details

### 1. I boot ThingOS on "x86_64" ✅

![Screenshot](steps/000_i_boot_thingos_on__x86_64_/screen.png)

```

```

---

### 2. a user task calls syscall "sys_log" 

![Screenshot](steps/001_a_user_task_calls_syscall__sys_log_/screen.png)

```
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
SYM:: configure_ctx: finalized sp=0xffff8000017e3f10
PROC: spawned
MOD: /boot/modules/graph_smoke
MOD: /boot/modules/log_smoke
MOD: /boot/modules/cap_fail
MOD: /boot/modules/clock
MOD: /boot/modules/inputd
MOD: /boot/modules/echo
MOD: /boot/modules/inspector
MOD: /boot/modules/logview
Booted.
SYM:: entering loop
SCHED: calling irq_enable...
SCHED: first tick!
TICK: switch IDLE -> 1 sp=0xffff800001798f10
GENERAL PROTECTION FAULT: 0x0000000000000000

KERNEL PANIC:
panicked at crates/kernel/src/machine/x86_64/idt.rs:45:5:
GPF
Halting.
TICK: switch 1 -> 2 sp=0xffff8000017e3f10
GENERAL PROTECTION FAULT: 0x0000000000000000

KERNEL PANIC:
panicked at crates/kernel/src/machine/x86_64/idt.rs:45:5:
GPF
Halting.
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
```

---

