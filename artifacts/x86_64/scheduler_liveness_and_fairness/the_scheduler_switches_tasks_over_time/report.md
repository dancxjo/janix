# Scenario: The scheduler switches tasks over time

**Architecture**: `x86_64`
**Feature**: `Scheduler liveness and fairness`

[Back to Index](../../../../index.md)

## Steps Summary

| Index | Step | Status | Artifacts |
| :---: | --- | :---: | --- |
| 0 | the system has reached "kernel ready" | ✅ | �� 📝  |
| 1 | I wait for 200 milliseconds | ✅ | �� 📝  |
| 2 | the serial log should contain "TICK:" | ✅ | �� 📝  |
| 3 | the serial log should contain "switch" | ✅ | �� 📝  |

## Execution Details

### 1. the system has reached "kernel ready" ✅

![Screenshot](steps/000_the_system_has_reached__kernel_ready_/screen.png)

```
SYM:: mapping 202000->1788000 len 1000
PROC: ELF loaded, entry
SYM:: spawn: name=/boot/modules/sprout start
SYM:: spawn: allocating stack...
SYM:: spawn: stack allocated
SYM:: spawn: graph store start
SYM:: spawn: graph store done
SYM:: spawn: address space handled
SYM:: spawn: finished id=1 name=/boot/modules/sprout stack_top=0xffff800001799000 stack_size=0x10000
SYM:: mapping 7ffe0000->179a000 len 20000
SYM:: configure_ctx: k_stack=0xffff800001799000 u_stack=0x80000000 entry=0x201480
SYM:: configure_ctx: finalized sp=0xffff800001798f60
SPROUT: I am alive
PROC: spawned
MOD: /boot/modules/bloom
PROC: spawn_kernel_module
SYM:: module phys=7948d000 offset=ffff800000000000
SYM:: new P4 phys=17bc000 virt=ffff8000017bc000
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
SYM:: configure_ctx: k_stack=0xffff8000017e4000 u_stack=0x80000000 entry=0x205270
SYM:: configure_ctx: finalized sp=0xffff8000017e3f60
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
TICK: switch IDLE -> 1 sp=0xffff800001798f60
TICK: switch 1 -> 2 sp=0xffff8000017e3f60
```

---

### 2. I wait for 200 milliseconds ✅

![Screenshot](steps/001_i_wait_for_200_milliseconds/screen.png)

```
SYM:: mapping 202000->1788000 len 1000
PROC: ELF loaded, entry
SYM:: spawn: name=/boot/modules/sprout start
SYM:: spawn: allocating stack...
SYM:: spawn: stack allocated
SYM:: spawn: graph store start
SYM:: spawn: graph store done
SYM:: spawn: address space handled
SYM:: spawn: finished id=1 name=/boot/modules/sprout stack_top=0xffff800001799000 stack_size=0x10000
SYM:: mapping 7ffe0000->179a000 len 20000
SYM:: configure_ctx: k_stack=0xffff800001799000 u_stack=0x80000000 entry=0x201480
SYM:: configure_ctx: finalized sp=0xffff800001798f60
SPROUT: I am alive
PROC: spawned
MOD: /boot/modules/bloom
PROC: spawn_kernel_module
SYM:: module phys=7948d000 offset=ffff800000000000
SYM:: new P4 phys=17bc000 virt=ffff8000017bc000
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
SYM:: configure_ctx: k_stack=0xffff8000017e4000 u_stack=0x80000000 entry=0x205270
SYM:: configure_ctx: finalized sp=0xffff8000017e3f60
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
TICK: switch IDLE -> 1 sp=0xffff800001798f60
TICK: switch 1 -> 2 sp=0xffff8000017e3f60
```

---

### 3. the serial log should contain "TICK:" ✅

![Screenshot](steps/002_the_serial_log_should_contain__tick__/screen.png)

```
SYM:: mapping 202000->1788000 len 1000
PROC: ELF loaded, entry
SYM:: spawn: name=/boot/modules/sprout start
SYM:: spawn: allocating stack...
SYM:: spawn: stack allocated
SYM:: spawn: graph store start
SYM:: spawn: graph store done
SYM:: spawn: address space handled
SYM:: spawn: finished id=1 name=/boot/modules/sprout stack_top=0xffff800001799000 stack_size=0x10000
SYM:: mapping 7ffe0000->179a000 len 20000
SYM:: configure_ctx: k_stack=0xffff800001799000 u_stack=0x80000000 entry=0x201480
SYM:: configure_ctx: finalized sp=0xffff800001798f60
SPROUT: I am alive
PROC: spawned
MOD: /boot/modules/bloom
PROC: spawn_kernel_module
SYM:: module phys=7948d000 offset=ffff800000000000
SYM:: new P4 phys=17bc000 virt=ffff8000017bc000
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
SYM:: configure_ctx: k_stack=0xffff8000017e4000 u_stack=0x80000000 entry=0x205270
SYM:: configure_ctx: finalized sp=0xffff8000017e3f60
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
TICK: switch IDLE -> 1 sp=0xffff800001798f60
TICK: switch 1 -> 2 sp=0xffff8000017e3f60
```

---

### 4. the serial log should contain "switch" ✅

![Screenshot](steps/003_the_serial_log_should_contain__switch_/screen.png)

```
SYM:: mapping 202000->1788000 len 1000
PROC: ELF loaded, entry
SYM:: spawn: name=/boot/modules/sprout start
SYM:: spawn: allocating stack...
SYM:: spawn: stack allocated
SYM:: spawn: graph store start
SYM:: spawn: graph store done
SYM:: spawn: address space handled
SYM:: spawn: finished id=1 name=/boot/modules/sprout stack_top=0xffff800001799000 stack_size=0x10000
SYM:: mapping 7ffe0000->179a000 len 20000
SYM:: configure_ctx: k_stack=0xffff800001799000 u_stack=0x80000000 entry=0x201480
SYM:: configure_ctx: finalized sp=0xffff800001798f60
SPROUT: I am alive
PROC: spawned
MOD: /boot/modules/bloom
PROC: spawn_kernel_module
SYM:: module phys=7948d000 offset=ffff800000000000
SYM:: new P4 phys=17bc000 virt=ffff8000017bc000
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
SYM:: configure_ctx: k_stack=0xffff8000017e4000 u_stack=0x80000000 entry=0x205270
SYM:: configure_ctx: finalized sp=0xffff8000017e3f60
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
TICK: switch IDLE -> 1 sp=0xffff800001798f60
TICK: switch 1 -> 2 sp=0xffff8000017e3f60
```

---
