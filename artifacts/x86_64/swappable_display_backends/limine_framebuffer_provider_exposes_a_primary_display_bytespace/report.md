# Scenario: Limine framebuffer provider exposes a primary display bytespace

**Architecture**: `x86_64`  
**Feature**: `Swappable display backends`

[Back to Index](../../../../index.md)

## Steps Summary

| Index | Step | Status | Artifacts |
| :---: | --- | :---: | --- |
| 0 | I boot the system with display provider "limine_fb" | ✅ | �� 📝  |
| 1 | the serial log should contain "DISPLAY: selected provider limine_fb" | ✅ | �� 📝  |
| 2 | the graph should contain a Thing named "device.display.primary" | ✅ | �� 📝  |
| 3 | the Thing "device.display.primary" should link to a Thing of kind "kind.Bytespace" | ⏭️ | �� 📝  |

## Execution Details

### 1. I boot the system with display provider "limine_fb" ✅

![Screenshot](steps/000_i_boot_the_system_with_display_provider__limine_fb_/screen.png)

```

```

---

### 2. the serial log should contain "DISPLAY: selected provider limine_fb" ✅

![Screenshot](steps/001_the_serial_log_should_contain__display__selected_provider_limine_fb_/screen.png)

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

### 3. the graph should contain a Thing named "device.display.primary" ✅

![Screenshot](steps/002_the_graph_should_contain_a_thing_named__device_display_primary_/screen.png)

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

### 4. the Thing "device.display.primary" should link to a Thing of kind "kind.Bytespace" ❌

![Screenshot](steps/003_the_thing__device_display_primary__should_link_to_a_thing_of_kind__kind_bytespace_/screen.png)

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

