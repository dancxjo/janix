# ✅ Scenario: Lilac Boot Screen

> Last run: 2026-01-10 14:40:51

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 560ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I wait for the system to boot | ✅ | 6327ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the screen should be filled with "Lilac" | ✅ | 2287ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[19655485443] [INFO] thing-os kernel v0.1.0 starting...
[19679027280] [INFO] Intent-Mechanism paging split active
[19679464596] [INFO] System booted
[19683856632] [INFO] Memory map has 35 entries
[19689184713] [INFO]   [0] 0x0 - 0xa0000 (Usable)
[19697264829] [INFO]   [1] 0x100000 - 0x800000 (Usable)
[19701726429] [INFO]   [2] 0x800000 - 0x808000 (Other)
[19706135163] [INFO]   [3] 0x808000 - 0x80b000 (Usable)
[19716100074] [INFO]   [4] 0x80b000 - 0x80c000 (Other)
[19716483831] [INFO]   [5] 0x80c000 - 0x811000 (Usable)
[19716869898] [INFO]   [6] 0x811000 - 0x900000 (Other)
[19717250949] [INFO]   [7] 0x900000 - 0x1780000 (Reserved)
[19717704963] [INFO]   [8] 0x1780000 - 0x79fa6000 (Usable)
[19718107695] [INFO]   [9] 0x79fa6000 - 0x7a16c000 (Reserved)
[19718831748] [INFO] HHDM Offset: 0xffff800000000000
[20689063890] [INFO] Frame allocator initialized with 511944 free frames
[20691418506] [INFO] Initializing global allocator...
[20724328845] [INFO] Initializing tasking...
[20728271355] [INFO]   Acquiring scheduler lock...
[20732629962] [INFO]   Lock acquired, checking if initialized...
[20738133867] [INFO]   Allocating scheduler...
[20742279195] [INFO]   Leaking scheduler...
[20742716676] [INFO]   Initializing boot task...
[20743129770] [INFO]   Creating boot task...
[20746123794] [INFO]   Pushing boot task to list...
[20752908066] [INFO]   Boot task created successfully
[20754358746] [INFO]   Storing scheduler pointer...
[20754906414] [INFO]   Scheduler initialized
[20755584168] [INFO] Checking threads_supported...
[20756116821] [INFO] Spawning Thread A...
[20759535093] [INFO] Spawning Thread B...
[20763100644] [INFO] System initialized. Entering scheduler loop.
[20769339261] [INFO] Thread A (arg=1) ticks=20769099120
[21653667948] [INFO] Thread B (arg=2) ticks=21653474865
[22762775805] [INFO] Thread A (arg=1) ticks=22762747194
[23543216202] [INFO] Thread B (arg=2) ticks=23543190396
[24171292629] [INFO] Thread A (arg=1) ticks=24171263226
[24732516270] [INFO] Thread B (arg=2) ticks=24732478683
[25390591941] [INFO] Thread A (arg=1) ticks=25390562967
[25913447241] [INFO] Thread B (arg=2) ticks=25913418498
[26435973498] [INFO] Thread A (arg=1) ticks=26435954853
[26956462665] [INFO] Thread B (arg=2) ticks=26956439697

```
</details>
