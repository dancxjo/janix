# ✅ Scenario: System halts

> Last run: 2026-01-09 10:51:25

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I turn on the machine | ✅ | 414ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | Then I should see that the machine has halted | ✅ | 9103ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26721593910] [INFO] System booted
[26736616236] [INFO] boot: phys ranges=35 modules=0
[26739675798] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[26751074064] [INFO] Initializing Real Frame Allocator...
[26758903611] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[26759916843] [INFO] Allocating bitmap of 8073 words...
[26768275380] [INFO] Bitmap allocated at 0xffffff8040000000
[26769705930] [INFO] Zeroing bitmap...
[26777754168] [INFO] Bitmap zeroed.
[27429079062] [INFO] frame_alloc: total=516638 free=511797 used=4841
[27437432055] [INFO] Running frame_alloc sanity check...
[27442740567] [INFO] frame_alloc: sanity: single ok
[27450348354] [INFO] frame_alloc: sanity: contig(8) ok
[27451248099] [INFO] Testing paging subsystem...
[27466525218] [INFO] Switched to new address space
[27472451424] [INFO] Paging subsystem test passed
[27477412281] [INFO] Paging subsystem test passed
[27478017765] [INFO] Initializing Kernel Heap...
[27488058345] [INFO] kheap: grew by 64 pages (phys=0x4f000, virt=0xffffa00000000000)
[27495706359] [INFO] global_alloc: switched to kernel heap
[27496927392] [INFO] Running heap sanity check...
[27524564595] [INFO] kheap: sanity ok
[27527446287] [INFO] kheap: forcing growth...
[27535435950] [INFO] kheap: grew by 16 pages (phys=0x100000, virt=0xffffa00000040000)
[27586512921] [INFO] kheap: big allocation ok (len=307200)
[27619583310] [INFO] kheap: reserved=268435456 committed=327680
[27623961750] [INFO] System halted

```
</details>
