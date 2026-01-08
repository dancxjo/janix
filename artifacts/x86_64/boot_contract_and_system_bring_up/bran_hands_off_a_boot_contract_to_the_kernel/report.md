# Scenario: Bran hands off a boot contract to the Kernel

**Architecture**: `x86_64`  
**Feature**: `Boot contract and system bring-up`

[Back to Index](../../../../index.md)

## Steps Summary

| Index | Step | Status | Artifacts |
| :---: | --- | :---: | --- |
| 0 | I boot the system | ✅ | �� 📝  |
| 1 | the serial output contains "BOOT:" | ✅ | �� 📝  |
| 2 | the serial output contains "TICK: switch" | ✅ | �� 📝  |
| 3 | the serial output contains "SPROUT: I am alive" | ✅ | �� 📝  |
| 4 | it does not panic | ⏭️ | �� 📝  |

## Execution Details

### 1. I boot the system ✅

![Screenshot](steps/000_i_boot_the_system/screen.png)

```

```

---

### 2. the serial output contains "BOOT:" ✅

![Screenshot](steps/001_the_serial_output_contains__boot__/screen.png)

```
JPROC: Applied 7 boot grants to /boot/modules/inputd (path: /boot/modules/inputd)
SCHED: configure_ctx_locked: k_stack=0xffff8000023d9000 u_stack=0x80000000 entry=0x206630
SCHED: configure_ctx_locked: finalized sp=0xffff8000023d8f60
PROC: spawned
PROC: spawn_kernel_module
[19654.034 ms] PROC: module phys=15ea6000 offset=ffff800000000000
[19654.327 ms] MMU: new P4 phys=23d9000 virt=ffff8000023d9000
[19654.559 ms] MMU: copy kernel pml4[256]
[19655.549 ms] MMU: copy kernel pml4[511]
JJJ[19660.485 ms] USER: TRACE: bloom_run: ENTER
[19661.859 ms] USER: BLOOM: alive
[19663.225 ms] USER: BLOOM: builtin font ready (lazy unifont)
JJ[19674.109 ms] USER: INPUTD: Starting...
J[19677.075 ms] JPROC: ELF loaded, entry
JPROC: Applied 1 boot grants to /boot/modules/thingcheck (path: /boot/modules/thingcheck)
SCHED: configure_ctx_locked: k_stack=0xffff80000254d000 u_stack=0x80000000 entry=0x210270
SCHED: configure_ctx_locked: finalized sp=0xffff80000254cf60
PROC: spawned
BOOT: Module hello_window not found
USER: INPUTD: Created bytespace.input.pointer0
JJ[19702.404 ms] USER: INPUTD: Ready. EventStream initialized.
[19685.384 ms] USER: SPROUT: failed to spawn hello_window: error 5
PROC: spawn_kernel_module
[19719.521 ms] PROC: module phys=14d06000 offset=ffff800000000000
[19720.115 ms] MMU: new P4 phys=254e000 virt=ffff80000254e000
[19720.387 ms] MMU: copy kernel pml4[256]
[19722.266 ms] MMU: copy kernel pml4[511]
JJJJPROC: ELF loaded, entry
JJJPROC: Applied 7 boot grants to /boot/modules/rtc_cmos (path: /boot/modules/rtc_cmos)
SCHED: configure_ctx_locked: k_stack=0xffff8000025b0000 u_stack=0x80000000 entry=0x20d200
SCHED: configure_ctx_locked: finalized sp=0xffff8000025aff60
PROC: spawned
JJ[19850.354 ms] THREAD: Allocated stack slot: group=1 bottom=0x8eff0000 top=0x8f000000 active=1
J[19853.355 ms] THREAD: Mapped user stack: bottom=0x8eff0000 top=0x8f000000 phys=0x26c4000
[19854.604 ms] THREAD: Thread 6 context: entry=0xa59910 user_sp=0x8f000000 arg0=0x90000178 kernel_sp=0xffff8000026c4000
[19904.468 ms] USER: BLOOM: present thread spawned
JJJJ[20032.505 ms] USER: RTC-CMOS: Starting...
[20033.847 ms] USER: RTC-CMOS: Hardware found. Publishing device...
JJ[20166.727 ms] USER: RTC-CMOS: Active. Time: 2026-1-8 4:44:45
PROC: spawn_kernel_module
[20231.188 ms] PROC: module phys=1727b000 offset=ffff800000000000
[20231.865 ms] MMU: new P4 phys=3166000 virt=ffff800003166000
[20232.409 ms] MMU: copy kernel pml4[256]
[20234.265 ms] MMU: copy kernel pml4[511]
JJJJPROC: ELF loaded, entry
JPROC: Applied 6 boot grants to /boot/modules/timed (path: /boot/modules/timed)
SCHED: configure_ctx_locked: k_stack=0xffff8000031c5000 u_stack=0x80000000 entry=0x20f4c0
SCHED: configure_ctx_locked: finalized sp=0xffff8000031c4f60
PROC: spawned
JJ[20354.762 ms] USER: TIMED: Starting...
```

---

### 3. the serial output contains "TICK: switch" ✅

![Screenshot](steps/002_the_serial_output_contains__tick__switch_/screen.png)

```
SCHED: configure_ctx_locked: finalized sp=0xffff80000254cf60
PROC: spawned
BOOT: Module hello_window not found
USER: INPUTD: Created bytespace.input.pointer0
JJ[19702.404 ms] USER: INPUTD: Ready. EventStream initialized.
[19685.384 ms] USER: SPROUT: failed to spawn hello_window: error 5
PROC: spawn_kernel_module
[19719.521 ms] PROC: module phys=14d06000 offset=ffff800000000000
[19720.115 ms] MMU: new P4 phys=254e000 virt=ffff80000254e000
[19720.387 ms] MMU: copy kernel pml4[256]
[19722.266 ms] MMU: copy kernel pml4[511]
JJJJPROC: ELF loaded, entry
JJJPROC: Applied 7 boot grants to /boot/modules/rtc_cmos (path: /boot/modules/rtc_cmos)
SCHED: configure_ctx_locked: k_stack=0xffff8000025b0000 u_stack=0x80000000 entry=0x20d200
SCHED: configure_ctx_locked: finalized sp=0xffff8000025aff60
PROC: spawned
JJ[19850.354 ms] THREAD: Allocated stack slot: group=1 bottom=0x8eff0000 top=0x8f000000 active=1
J[19853.355 ms] THREAD: Mapped user stack: bottom=0x8eff0000 top=0x8f000000 phys=0x26c4000
[19854.604 ms] THREAD: Thread 6 context: entry=0xa59910 user_sp=0x8f000000 arg0=0x90000178 kernel_sp=0xffff8000026c4000
[19904.468 ms] USER: BLOOM: present thread spawned
JJJJ[20032.505 ms] USER: RTC-CMOS: Starting...
[20033.847 ms] USER: RTC-CMOS: Hardware found. Publishing device...
JJ[20166.727 ms] USER: RTC-CMOS: Active. Time: 2026-1-8 4:44:45
PROC: spawn_kernel_module
[20231.188 ms] PROC: module phys=1727b000 offset=ffff800000000000
[20231.865 ms] MMU: new P4 phys=3166000 virt=ffff800003166000
[20232.409 ms] MMU: copy kernel pml4[256]
[20234.265 ms] MMU: copy kernel pml4[511]
JJJJPROC: ELF loaded, entry
JPROC: Applied 6 boot grants to /boot/modules/timed (path: /boot/modules/timed)
SCHED: configure_ctx_locked: k_stack=0xffff8000031c5000 u_stack=0x80000000 entry=0x20f4c0
SCHED: configure_ctx_locked: finalized sp=0xffff8000031c4f60
PROC: spawned
JJ[20354.762 ms] USER: TIMED: Starting...
[20494.457 ms] USER: TIMED: Found RTC base: 1767847485s at mono 20038472006ns
[20560.579 ms] USER: TIMED: Published system.time
PROC: spawn_kernel_module
[20634.267 ms] PROC: module phys=17644000 offset=ffff800000000000
[20634.692 ms] MMU: new P4 phys=32c8000 virt=ffff8000032c8000
[20634.919 ms] MMU: copy kernel pml4[256]
[20635.813 ms] MMU: copy kernel pml4[511]
JJJJPROC: ELF loaded, entry
JPROC: Applied 6 boot grants to /boot/modules/clock (path: /boot/modules/clock)
SCHED: configure_ctx_locked: k_stack=0xffff800003322000 u_stack=0x80000000 entry=0x20b830
SCHED: configure_ctx_locked: finalized sp=0xffff800003321f60
PROC: spawned
[20640.634 ms] USER: SPROUT: boot sequence complete.
JJ[20771.384 ms] USER: CLOCK: Starting simple time ticker...
[20911.773 ms] USER: CLOCK: 04:44:45
[21005.344 ms] USER: BLOOM: first paint complete
```

---

### 4. the serial output contains "SPROUT: I am alive" ✅

![Screenshot](steps/003_the_serial_output_contains__sprout__i_am_alive_/screen.png)

```
PROC: spawned
BOOT: Module hello_window not found
USER: INPUTD: Created bytespace.input.pointer0
JJ[19702.404 ms] USER: INPUTD: Ready. EventStream initialized.
[19685.384 ms] USER: SPROUT: failed to spawn hello_window: error 5
PROC: spawn_kernel_module
[19719.521 ms] PROC: module phys=14d06000 offset=ffff800000000000
[19720.115 ms] MMU: new P4 phys=254e000 virt=ffff80000254e000
[19720.387 ms] MMU: copy kernel pml4[256]
[19722.266 ms] MMU: copy kernel pml4[511]
JJJJPROC: ELF loaded, entry
JJJPROC: Applied 7 boot grants to /boot/modules/rtc_cmos (path: /boot/modules/rtc_cmos)
SCHED: configure_ctx_locked: k_stack=0xffff8000025b0000 u_stack=0x80000000 entry=0x20d200
SCHED: configure_ctx_locked: finalized sp=0xffff8000025aff60
PROC: spawned
JJ[19850.354 ms] THREAD: Allocated stack slot: group=1 bottom=0x8eff0000 top=0x8f000000 active=1
J[19853.355 ms] THREAD: Mapped user stack: bottom=0x8eff0000 top=0x8f000000 phys=0x26c4000
[19854.604 ms] THREAD: Thread 6 context: entry=0xa59910 user_sp=0x8f000000 arg0=0x90000178 kernel_sp=0xffff8000026c4000
[19904.468 ms] USER: BLOOM: present thread spawned
JJJJ[20032.505 ms] USER: RTC-CMOS: Starting...
[20033.847 ms] USER: RTC-CMOS: Hardware found. Publishing device...
JJ[20166.727 ms] USER: RTC-CMOS: Active. Time: 2026-1-8 4:44:45
PROC: spawn_kernel_module
[20231.188 ms] PROC: module phys=1727b000 offset=ffff800000000000
[20231.865 ms] MMU: new P4 phys=3166000 virt=ffff800003166000
[20232.409 ms] MMU: copy kernel pml4[256]
[20234.265 ms] MMU: copy kernel pml4[511]
JJJJPROC: ELF loaded, entry
JPROC: Applied 6 boot grants to /boot/modules/timed (path: /boot/modules/timed)
SCHED: configure_ctx_locked: k_stack=0xffff8000031c5000 u_stack=0x80000000 entry=0x20f4c0
SCHED: configure_ctx_locked: finalized sp=0xffff8000031c4f60
PROC: spawned
JJ[20354.762 ms] USER: TIMED: Starting...
[20494.457 ms] USER: TIMED: Found RTC base: 1767847485s at mono 20038472006ns
[20560.579 ms] USER: TIMED: Published system.time
PROC: spawn_kernel_module
[20634.267 ms] PROC: module phys=17644000 offset=ffff800000000000
[20634.692 ms] MMU: new P4 phys=32c8000 virt=ffff8000032c8000
[20634.919 ms] MMU: copy kernel pml4[256]
[20635.813 ms] MMU: copy kernel pml4[511]
JJJJPROC: ELF loaded, entry
JPROC: Applied 6 boot grants to /boot/modules/clock (path: /boot/modules/clock)
SCHED: configure_ctx_locked: k_stack=0xffff800003322000 u_stack=0x80000000 entry=0x20b830
SCHED: configure_ctx_locked: finalized sp=0xffff800003321f60
PROC: spawned
[20640.634 ms] USER: SPROUT: boot sequence complete.
JJ[20771.384 ms] USER: CLOCK: Starting simple time ticker...
[20911.773 ms] USER: CLOCK: 04:44:45
[21005.344 ms] USER: BLOOM: first paint complete
JJJJ[21944.642 ms] USER: CLOCK: 04:44:46
```

---

### 5. it does not panic ❌

![Screenshot](steps/004_it_does_not_panic/screen.png)

```
JJ[19702.404 ms] USER: INPUTD: Ready. EventStream initialized.
[19685.384 ms] USER: SPROUT: failed to spawn hello_window: error 5
PROC: spawn_kernel_module
[19719.521 ms] PROC: module phys=14d06000 offset=ffff800000000000
[19720.115 ms] MMU: new P4 phys=254e000 virt=ffff80000254e000
[19720.387 ms] MMU: copy kernel pml4[256]
[19722.266 ms] MMU: copy kernel pml4[511]
JJJJPROC: ELF loaded, entry
JJJPROC: Applied 7 boot grants to /boot/modules/rtc_cmos (path: /boot/modules/rtc_cmos)
SCHED: configure_ctx_locked: k_stack=0xffff8000025b0000 u_stack=0x80000000 entry=0x20d200
SCHED: configure_ctx_locked: finalized sp=0xffff8000025aff60
PROC: spawned
JJ[19850.354 ms] THREAD: Allocated stack slot: group=1 bottom=0x8eff0000 top=0x8f000000 active=1
J[19853.355 ms] THREAD: Mapped user stack: bottom=0x8eff0000 top=0x8f000000 phys=0x26c4000
[19854.604 ms] THREAD: Thread 6 context: entry=0xa59910 user_sp=0x8f000000 arg0=0x90000178 kernel_sp=0xffff8000026c4000
[19904.468 ms] USER: BLOOM: present thread spawned
JJJJ[20032.505 ms] USER: RTC-CMOS: Starting...
[20033.847 ms] USER: RTC-CMOS: Hardware found. Publishing device...
JJ[20166.727 ms] USER: RTC-CMOS: Active. Time: 2026-1-8 4:44:45
PROC: spawn_kernel_module
[20231.188 ms] PROC: module phys=1727b000 offset=ffff800000000000
[20231.865 ms] MMU: new P4 phys=3166000 virt=ffff800003166000
[20232.409 ms] MMU: copy kernel pml4[256]
[20234.265 ms] MMU: copy kernel pml4[511]
JJJJPROC: ELF loaded, entry
JPROC: Applied 6 boot grants to /boot/modules/timed (path: /boot/modules/timed)
SCHED: configure_ctx_locked: k_stack=0xffff8000031c5000 u_stack=0x80000000 entry=0x20f4c0
SCHED: configure_ctx_locked: finalized sp=0xffff8000031c4f60
PROC: spawned
JJ[20354.762 ms] USER: TIMED: Starting...
[20494.457 ms] USER: TIMED: Found RTC base: 1767847485s at mono 20038472006ns
[20560.579 ms] USER: TIMED: Published system.time
PROC: spawn_kernel_module
[20634.267 ms] PROC: module phys=17644000 offset=ffff800000000000
[20634.692 ms] MMU: new P4 phys=32c8000 virt=ffff8000032c8000
[20634.919 ms] MMU: copy kernel pml4[256]
[20635.813 ms] MMU: copy kernel pml4[511]
JJJJPROC: ELF loaded, entry
JPROC: Applied 6 boot grants to /boot/modules/clock (path: /boot/modules/clock)
SCHED: configure_ctx_locked: k_stack=0xffff800003322000 u_stack=0x80000000 entry=0x20b830
SCHED: configure_ctx_locked: finalized sp=0xffff800003321f60
PROC: spawned
[20640.634 ms] USER: SPROUT: boot sequence complete.
JJ[20771.384 ms] USER: CLOCK: Starting simple time ticker...
[20911.773 ms] USER: CLOCK: 04:44:45
[21005.344 ms] USER: BLOOM: first paint complete
JJJJ[21944.642 ms] USER: CLOCK: 04:44:46
[22979.656 ms] USER: CLOCK: 04:44:47
[23695.543 ms] USER: BLOOM: cursor set loaded
[23756.943 ms] USER: BLOOM: loading wallpaper sync
```

---

