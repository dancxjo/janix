# ✅ Scenario: Lilac Boot Screen

> Last run: 2026-01-09 13:03:50

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 1281ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I wait for the system to boot | ✅ | 3661ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the screen should be filled with "Lilac" | ✅ | 2062ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[14450242587] [INFO] System booted
[14461564326] [INFO] boot: phys ranges=35 modules=0
[14464032363] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[14473459605] [INFO] Initializing Real Frame Allocator...
[14476653213] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[14477278431] [INFO] Allocating bitmap of 8073 words...
[14484554634] [INFO] Bitmap allocated at 0xffffff8040000000
[14488916343] [INFO] Zeroing bitmap...
[14495951745] [INFO] Bitmap zeroed.
[15436306875] [INFO] frame_alloc: total=516638 free=511759 used=4879
[15437227212] [INFO] Running frame_alloc sanity check...
[15445152492] [INFO] frame_alloc: sanity: single ok
[15453289203] [INFO] frame_alloc: sanity: contig(8) ok
[15453816510] [INFO] Testing paging subsystem...
[15473018286] [INFO] Switched to new address space
[15473553744] [INFO] Paging subsystem test passed
[15474013368] [INFO] Paging subsystem test passed
[15477832821] [INFO] Initializing Kernel Heap...
[15491575539] [INFO] kheap: grew by 64 pages (phys=0x4f000, virt=0xffffa00000000000)
[15492885771] [INFO] global_alloc: switched to kernel heap
[15493404696] [INFO] Running heap sanity check...
[15536010600] [INFO] kheap: sanity ok
[15537855234] [INFO] kheap: forcing growth...
[15544986897] [INFO] kheap: grew by 16 pages (phys=0x100000, virt=0xffffa00000040000)
[15608013300] [INFO] kheap: big allocation ok (len=307200)
[15611570865] [INFO] kheap: reserved=268435456 committed=327680
[15632227974] [INFO] Initializing Task System...
[15643417317] [INFO] threads: supported
[15649551918] [INFO] Spawning Thread A...
[15664698258] [INFO] Spawning Thread B...
[15678233538] [INFO] Entering Scheduler Loop (Main Task)...
[15691092351] [INFO] Thread A (arg=1) ticks=15690938109
[16191770694] [INFO] Thread B (arg=2) ticks=16191586290
[16636085466] [INFO] Thread A (arg=1) ticks=16636051641
[16897796751] [INFO] Thread B (arg=2) ticks=16897762299
[17145849501] [INFO] Thread A (arg=1) ticks=17145819471
[17394406260] [INFO] Thread B (arg=2) ticks=17394375339
[17649339081] [INFO] Thread A (arg=1) ticks=17649311823
[17907470361] [INFO] Thread B (arg=2) ticks=17907437724
[18162177297] [INFO] Thread A (arg=1) ticks=18162131361
[18503662551] [INFO] Thread B (arg=2) ticks=18503634237
[19030134156] [INFO] Thread A (arg=1) ticks=19030100925
[19728587637] [INFO] Thread B (arg=2) ticks=19728557046
[20106252309] [INFO] Thread A (arg=1) ticks=20106222939
[20369321610] [INFO] Thread B (arg=2) ticks=20369287026
[20634770805] [INFO] Thread A (arg=1) ticks=20634735891
[20899188486] [INFO] Thread B (arg=2) ticks=20899155486
[21169802742] [INFO] Thread A (arg=1) ticks=21169770732
[21426422094] [INFO] Thread B (arg=2) ticks=21426393021
[21681316206] [INFO] Thread A (arg=1) ticks=21681285549
[21925969197] [INFO] Thread B (arg=2) ticks=21925939662
[22168941432] [INFO] Thread A (arg=1) ticks=22168911105
[22426017141] [INFO] Thread B (arg=2) ticks=22425984933
[22690688790] [INFO] Thread A (arg=1) ticks=22690665294
[22961155767] [INFO] Thread B (arg=2) ticks=22961126166
[23228854275] [INFO] Thread A (arg=1) ticks=23228821605
[23489316939] [INFO] Thread B (arg=2) ticks=23489285985
[23730912084] [INFO] Thread A (arg=1) ticks=23730879348
[23966822781] [INFO] Thread B (arg=2) ticks=23966791629
[24210150558] [INFO] Thread A (arg=1) ticks=24210116733
[24463920855] [INFO] Thread B (arg=2) ticks=24463895214
[24717010065] [INFO] Thread A (arg=1) ticks=24716983566
[24963431691] [INFO] Thread B (arg=2) ticks=24963400077
[25209014127] [INFO] Thread A (arg=1) ticks=25208985846
[25448155524] [INFO] Thread B (arg=2) ticks=25448124339
[25698175503] [INFO] Thread A (arg=1) ticks=25698145605
[25948814232] [INFO] Thread B (arg=2) ticks=25948788624

```
</details>
