# Scenario: Bloom cannot access display bytespace without capability

**Architecture**: `x86_64`  
**Feature**: `Swappable display backends`

[Back to Index](../../../../index.md)

## Steps Summary

| Index | Step | Status | Artifacts |
| :---: | --- | :---: | --- |
| 0 | I boot the system with display provider "limine_fb" | ✅ | �� 📝  |
| 1 | sprout is online | ✅ | �� 📝  |
| 2 | the process "bloom" has no capability "cap.framebuffer" | ⏭️ | �� 📝  |

## Execution Details

### 1. I boot the system with display provider "limine_fb" ✅

![Screenshot](steps/000_i_boot_the_system_with_display_provider__limine_fb_/screen.png)

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
```

---

### 2. sprout is online ✅

![Screenshot](steps/001_sprout_is_online/screen.png)

```
SYM:: mapping 201000->1787000 len 1000
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
SYM:: configure_ctx: k_stack=0xffff800001799000 u_stack=0x80000000 entry=0x201680
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
SYM:: configure_ctx: k_stack=0xffff8000017e4000 u_stack=0x80000000 entry=0x205760
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
```

---

### 3. the process "bloom" has no capability "cap.framebuffer" ❌

![Screenshot](steps/002_the_process__bloom__has_no_capability__cap_framebuffer_/screen.png)

```
SYM:: mapping 201000->1787000 len 1000
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
SYM:: configure_ctx: k_stack=0xffff800001799000 u_stack=0x80000000 entry=0x201680
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
SYM:: configure_ctx: k_stack=0xffff8000017e4000 u_stack=0x80000000 entry=0x205760
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
```

---

