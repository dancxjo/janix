# ✅ Scenario: Lilac Boot Screen

> Last run: 2026-01-09 11:57:58

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 597ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I wait for the system to boot | ✅ | 6851ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the screen should be filled with "Lilac" | ✅ | 5034ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[21872564901] [INFO] System booted
[21900847122] [INFO] boot: phys ranges=35 modules=0
[21904341888] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[21907237506] [INFO] Initializing Real Frame Allocator...
[21909464148] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[21910284495] [INFO] Allocating bitmap of 8073 words...
[21923168652] [INFO] Bitmap allocated at 0xffffff8040000000
[21931157655] [INFO] Zeroing bitmap...
[21956176044] [INFO] Bitmap zeroed.
[23258001393] [INFO] frame_alloc: total=516638 free=511760 used=4878
[23265982410] [INFO] Running frame_alloc sanity check...
[23270594589] [INFO] frame_alloc: sanity: single ok
[23286407298] [INFO] frame_alloc: sanity: contig(8) ok
[23287102608] [INFO] Testing paging subsystem...
[23312529306] [INFO] Switched to new address space
[23316012621] [INFO] Paging subsystem test passed
[23316594378] [INFO] Paging subsystem test passed
[23317135677] [INFO] Initializing Kernel Heap...
[23340839148] [INFO] kheap: grew by 64 pages (phys=0x4f000, virt=0xffffa00000000000)
[23353057365] [INFO] global_alloc: switched to kernel heap
[23353740531] [INFO] Running heap sanity check...
[23409389025] [INFO] kheap: sanity ok
[23413575075] [INFO] kheap: forcing growth...
[23422561899] [INFO] kheap: grew by 16 pages (phys=0x100000, virt=0xffffa00000040000)
[23466684813] [INFO] kheap: big allocation ok (len=307200)
[23481051033] [INFO] Initializing Task System...
[23491009047] [INFO] Spawning Thread A...
[23504326527] [INFO] Spawning Thread B...
[23517964569] [INFO] Entering Scheduler Loop (Main Task)...
[23527946079] [INFO] Thread A (arg=0) ticks=23527748343
[23915517384] [INFO] Thread B (arg=0) ticks=23915279619
[24339085023] [INFO] Thread A (arg=0) ticks=24339046149
[24707593251] [INFO] Thread B (arg=0) ticks=24707563122
[25160674143] [INFO] Thread A (arg=0) ticks=25160632893
[25631418285] [INFO] Thread B (arg=0) ticks=25631383206
[26440459353] [INFO] Thread A (arg=0) ticks=26440421337
[26962804275] [INFO] Thread B (arg=0) ticks=26962770516
[27649953243] [INFO] Thread A (arg=0) ticks=27649918824
[27972153528] [INFO] Thread B (arg=0) ticks=27972124224
[28375908858] [INFO] Thread A (arg=0) ticks=28375867509
[28753897722] [INFO] Thread B (arg=0) ticks=28753865415
[29161443366] [INFO] Thread A (arg=0) ticks=29161404822
[29486242434] [INFO] Thread B (arg=0) ticks=29486205309
[29908815354] [INFO] Thread A (arg=0) ticks=29908787172
[30361584528] [INFO] Thread B (arg=0) ticks=30361552650
[30931308309] [INFO] Thread A (arg=0) ticks=30931272669
[31613400027] [INFO] Thread B (arg=0) ticks=31613365344
[32127262332] [INFO] Thread A (arg=0) ticks=32127226989
[32422109874] [INFO] Thread B (arg=0) ticks=32422071462
[32881147893] [INFO] Thread A (arg=0) ticks=32881111131
[33190501047] [INFO] Thread B (arg=0) ticks=33190464153
[33603881355] [INFO] Thread A (arg=0) ticks=33603843933
[33955859784] [INFO] Thread B (arg=0) ticks=33955824078
[34417875822] [INFO] Thread A (arg=0) ticks=34417644459
[34738632456] [INFO] Thread B (arg=0) ticks=34738598631
[35137745049] [INFO] Thread A (arg=0) ticks=35137709739
[35734209423] [INFO] Thread B (arg=0) ticks=35734173156
[36478859109] [INFO] Thread A (arg=0) ticks=36478823766

```
</details>
