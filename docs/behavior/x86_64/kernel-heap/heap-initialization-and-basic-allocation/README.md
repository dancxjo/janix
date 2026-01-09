# ✅ Scenario: Heap Initialization and Basic Allocation

> Last run: 2026-01-09 10:51:25

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booting | ⏭️ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[16752832998] [INFO] System booted
[16768727151] [INFO] boot: phys ranges=35 modules=0
[16774305174] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[16780479738] [INFO] Initializing Real Frame Allocator...
[16784950809] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[16787834514] [INFO] Allocating bitmap of 8073 words...
[16797326931] [INFO] Bitmap allocated at 0xffffff8040000000
[16802180934] [INFO] Zeroing bitmap...
[16809475122] [INFO] Bitmap zeroed.
[17724601950] [INFO] frame_alloc: total=516638 free=511797 used=4841
[17736006750] [INFO] Running frame_alloc sanity check...
[17741661267] [INFO] frame_alloc: sanity: single ok
[17749581333] [INFO] frame_alloc: sanity: contig(8) ok
[17751686568] [INFO] Testing paging subsystem...
[17763340386] [INFO] Switched to new address space
[17766000351] [INFO] Paging subsystem test passed
[17767016949] [INFO] Paging subsystem test passed
[17767568478] [INFO] Initializing Kernel Heap...
[17776733436] [INFO] kheap: grew by 64 pages (phys=0x4f000, virt=0xffffa00000000000)
[17781653274] [INFO] global_alloc: switched to kernel heap
[17782525530] [INFO] Running heap sanity check...
[17809520553] [INFO] kheap: sanity ok
[17810507682] [INFO] kheap: forcing growth...
[17815345317] [INFO] kheap: grew by 16 pages (phys=0x100000, virt=0xffffa00000040000)
[17850713595] [INFO] kheap: big allocation ok (len=307200)
[17864929203] [INFO] kheap: reserved=268435456 committed=327680
[17869370145] [INFO] System halted

```
</details>
