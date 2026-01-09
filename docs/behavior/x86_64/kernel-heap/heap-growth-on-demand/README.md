# ✅ Scenario: Heap Growth on Demand

> Last run: 2026-01-09 10:55:29

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booting | ⏭️ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[17498094999] [INFO] System booted
[17511802407] [INFO] boot: phys ranges=35 modules=0
[17513822931] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[17516172135] [INFO] Initializing Real Frame Allocator...
[17517902259] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[17518534704] [INFO] Allocating bitmap of 8073 words...
[17524870803] [INFO] Bitmap allocated at 0xffffff8040000000
[17527786584] [INFO] Zeroing bitmap...
[17535974346] [INFO] Bitmap zeroed.
[18036286125] [INFO] frame_alloc: total=516638 free=511797 used=4841
[18037230255] [INFO] Running frame_alloc sanity check...
[18041120559] [INFO] frame_alloc: sanity: single ok
[18048400656] [INFO] frame_alloc: sanity: contig(8) ok
[18048920241] [INFO] Testing paging subsystem...
[18057793545] [INFO] Switched to new address space
[18062118855] [INFO] Paging subsystem test passed
[18066333021] [INFO] Paging subsystem test passed
[18068711100] [INFO] Initializing Kernel Heap...
[18075788907] [INFO] kheap: grew by 64 pages (phys=0x4f000, virt=0xffffa00000000000)
[18081791871] [INFO] global_alloc: switched to kernel heap
[18082349604] [INFO] Running heap sanity check...
[18102138714] [INFO] kheap: sanity ok
[18105431586] [INFO] kheap: forcing growth...
[18111533286] [INFO] kheap: grew by 16 pages (phys=0x100000, virt=0xffffa00000040000)
[18140662650] [INFO] kheap: big allocation ok (len=307200)
[18154715007] [INFO] kheap: reserved=268435456 committed=327680
[18160072788] [INFO] System halted

```
</details>
