# ✅ Scenario: System halts

> Last run: 2026-01-09 09:13:04

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I turn on the machine | ✅ | 338ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | Then I should see that the machine has halted | ✅ | 5710ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[13072076754] [INFO] System booted
[13086351201] [INFO] boot: phys ranges=35 modules=0
[13092664695] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[13096760490] [INFO] Initializing Real Frame Allocator...
[13098678879] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[13595136060] [INFO] frame_alloc: total=516638 free=511667 used=4971
[13597676433] [INFO] Running frame_alloc sanity check...
[13601281815] [INFO] frame_alloc: sanity: single ok
[13607189772] [INFO] frame_alloc: sanity: contig(8) ok
[13609795122] [INFO] Testing paging subsystem...
[13618731357] [INFO] Switched to new address space
[13621994793] [INFO] Paging subsystem test passed
[13622467551] [INFO] Paging subsystem test passed
[13622902821] [INFO] Initializing Kernel Heap...
[13630133550] [INFO] kheap: grew by 64 pages (phys=0x3f000, virt=0xffffa00000000000)
[13635525453] [INFO] global_alloc: switched to kernel heap
[13636582773] [INFO] Running heap sanity check...
[13655852529] [INFO] kheap: sanity ok
[13658620734] [INFO] kheap: forcing growth...
[13661526780] [INFO] kheap: grew by 16 pages (phys=0x82000, virt=0xffffa00000040000)
[13686692976] [INFO] kheap: big allocation ok (len=307200)
[13699595877] [INFO] kheap: reserved=268435456 committed=327680
[13701231258] [INFO] System halted

```
</details>
