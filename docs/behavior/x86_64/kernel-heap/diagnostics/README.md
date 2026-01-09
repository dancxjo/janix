# ✅ Scenario: Diagnostics

> Last run: 2026-01-09 13:03:50

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booting | ✅ | 625ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the log should contain "kheap: reserved=" | ✅ | 4819ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> - [💾](./02/registers.txt) |
| 3 | And the log should contain "committed=" | ✅ | 222ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[14435529570] [INFO] System booted
[14448460521] [INFO] boot: phys ranges=35 modules=0
[14452950930] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[14458428600] [INFO] Initializing Real Frame Allocator...
[14460780807] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[14463371010] [INFO] Allocating bitmap of 8073 words...
[14470174719] [INFO] Bitmap allocated at 0xffffff8040000000
[14472231609] [INFO] Zeroing bitmap...
[14478438381] [INFO] Bitmap zeroed.
[14959128591] [INFO] frame_alloc: total=516638 free=511759 used=4879
[14963042325] [INFO] Running frame_alloc sanity check...
[14966752911] [INFO] frame_alloc: sanity: single ok
[14973242130] [INFO] frame_alloc: sanity: contig(8) ok
[14975746467] [INFO] Testing paging subsystem...
[14984905980] [INFO] Switched to new address space
[14988176775] [INFO] Paging subsystem test passed
[14988635046] [INFO] Paging subsystem test passed
[14989059360] [INFO] Initializing Kernel Heap...
[14996207226] [INFO] kheap: grew by 64 pages (phys=0x4f000, virt=0xffffa00000000000)
[15001305198] [INFO] global_alloc: switched to kernel heap
[15002358756] [INFO] Running heap sanity check...
[15022563534] [INFO] kheap: sanity ok
[15025622106] [INFO] kheap: forcing growth...
[15029338995] [INFO] kheap: grew by 16 pages (phys=0x100000, virt=0xffffa00000040000)
[15059045958] [INFO] kheap: big allocation ok (len=307200)
[15063054996] [INFO] kheap: reserved=268435456 committed=327680
[15074929551] [INFO] Initializing Task System...
[15081536481] [INFO] threads: supported
[15084618153] [INFO] Spawning Thread A...
[15093214554] [INFO] Spawning Thread B...
[15101696379] [INFO] Entering Scheduler Loop (Main Task)...
[15107860416] [INFO] Thread A (arg=1) ticks=15107701554
[15357981573] [INFO] Thread B (arg=2) ticks=15357788523
[15796217217] [INFO] Thread A (arg=1) ticks=15796185042
[16297605456] [INFO] Thread B (arg=2) ticks=16297572819
[16975496406] [INFO] Thread A (arg=1) ticks=16975462944
[17567262867] [INFO] Thread B (arg=2) ticks=17567232375
[18284646600] [INFO] Thread A (arg=1) ticks=18284613897
[19139200509] [INFO] Thread B (arg=2) ticks=19139166123
[19605319800] [INFO] Thread A (arg=1) ticks=19605288648
[19870263006] [INFO] Thread B (arg=2) ticks=19870229049
[20780444685] [INFO] Thread A (arg=1) ticks=20780409639
[21110437062] [INFO] Thread B (arg=2) ticks=21110405184

```
</details>
