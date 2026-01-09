# ✅ Scenario: Diagnostics

> Last run: 2026-01-09 10:15:03

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booting | ⏭️ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[13768255314] [INFO] System booted
[13781325525] [INFO] boot: phys ranges=35 modules=0
[13787109633] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[13790637663] [INFO] Initializing Real Frame Allocator...
[13792463355] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[13795218360] [INFO] Allocating bitmap of 8073 words...
[13802463477] [INFO] Bitmap allocated at 0xffffff8040000000
[13808003781] [INFO] Zeroing bitmap...
[13814229990] [INFO] Bitmap zeroed.
[14667369684] [INFO] frame_alloc: total=516638 free=450333 used=66305
[14671860687] [INFO] Running frame_alloc sanity check...
[14783576346] [INFO] frame_alloc: sanity: single ok
[14788132920] [INFO] frame_alloc: sanity: contig(8) ok
[14793371703] [INFO] Testing paging subsystem...
[14810332548] [INFO] Switched to new address space
[14816257632] [INFO] Paging subsystem test passed
[14818616241] [INFO] Paging subsystem test passed
[14819031876] [INFO] Initializing Kernel Heap...
[14835289689] [INFO] kheap: grew by 64 pages (phys=0x1003f000, virt=0xffffa00000000000)
[14840387232] [INFO] global_alloc: switched to kernel heap
[14840876094] [INFO] Running heap sanity check...
[14881106097] [INFO] kheap: sanity ok
[14885409858] [INFO] kheap: forcing growth...
[14892218418] [INFO] kheap: grew by 16 pages (phys=0x10082000, virt=0xffffa00000040000)
[14946828237] [INFO] kheap: big allocation ok (len=307200)
[14972117160] [INFO] kheap: reserved=268435456 committed=327680
[14977677198] [INFO] System halted

```
</details>
