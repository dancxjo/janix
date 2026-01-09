# ✅ Scenario: Diagnostics

> Last run: 2026-01-09 09:13:04

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booting | ⏭️ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12817832346] [INFO] System booted
[12842708571] [INFO] boot: phys ranges=35 modules=0
[12848849706] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[12862831608] [INFO] Initializing Real Frame Allocator...
[12864626478] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[14578396833] [INFO] frame_alloc: total=516638 free=511667 used=4971
[14586634425] [INFO] Running frame_alloc sanity check...
[14597551782] [INFO] frame_alloc: sanity: single ok
[14605210917] [INFO] frame_alloc: sanity: contig(8) ok
[14608728387] [INFO] Testing paging subsystem...
[14630202345] [INFO] Switched to new address space
[14634242403] [INFO] Paging subsystem test passed
[14634719814] [INFO] Paging subsystem test passed
[14635155744] [INFO] Initializing Kernel Heap...
[14652748902] [INFO] kheap: grew by 64 pages (phys=0x3f000, virt=0xffffa00000000000)
[14657762691] [INFO] global_alloc: switched to kernel heap
[14658214692] [INFO] Running heap sanity check...
[14701350147] [INFO] kheap: sanity ok
[14706598302] [INFO] kheap: forcing growth...
[14711746500] [INFO] kheap: grew by 16 pages (phys=0x82000, virt=0xffffa00000040000)
[14769414462] [INFO] kheap: big allocation ok (len=307200)
[14788061541] [INFO] kheap: reserved=268435456 committed=327680
[14794253859] [INFO] System halted

```
</details>
