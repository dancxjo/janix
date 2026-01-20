# ✅ Scenario: Boot produces logs, graphics, and a live desktop

> Last run: 2026-01-19 21:17:21

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 3630ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | Then I should see log messages on the terminal | ✅ | 388ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And each log message should include a monotonically increasing timestamp | ✅ | 409ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And I should see the system clock tick for several seconds in the serial console | ✅ | 1824ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> - [💾](./04/registers.txt) |
| 5 | And I should see the wallpaper on the screen within 15 seconds | ✅ | 15201ms | <a href="./05/after.png"><img src="./05/after.png" width="150" /></a> - [💾](./05/registers.txt) |
| 6 | And I should see a cursor centered on the screen | ✅ | 1011ms | <a href="./06/after.png"><img src="./06/after.png" width="150" /></a> - [💾](./06/registers.txt) |
| 7 | And I should see the text "thing-os" in the top-left corner of the screen | ✅ | 446ms | <a href="./07/after.png"><img src="./07/after.png" width="150" /></a> - [💾](./07/registers.txt) |
| 8 | And I should see frame count information in the top-left corner of the screen | ✅ | 467ms | <a href="./08/after.png"><img src="./08/after.png" width="150" /></a> - [💾](./08/registers.txt) |
| 9 | And I should see a clock window displaying a ticking clock | ✅ | 831ms | <a href="./09/after.png"><img src="./09/after.png" width="150" /></a> - [💾](./09/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[10235774307] [CONTRACT] [kernel] thing-os kernel starting...
[10478882007] [CONTRACT] [kernel::memory] Frame allocator initialized with 495746 free frames
[10498673922] [CONTRACT] [kernel] Initializing global allocator...
[10860640329] [CONTRACT] [kernel] Initializing SIMD...
[10862032698] [CONTRACT] [kernel] Initializing tasking...
[10881193290] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[11965730667] [CONTRACT] [kernel] KERNEL: root census complete: host=t1 kernel=t3 root=t4
[11997166269] [CONTRACT] [kernel] Spawning init process...
[12033207318] [CONTRACT] [kernel] Entering scheduler loop.
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xd1
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdf
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc

```
</details>
