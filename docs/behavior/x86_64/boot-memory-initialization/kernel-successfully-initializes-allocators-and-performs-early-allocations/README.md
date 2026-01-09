# ✅ Scenario: Kernel successfully initializes allocators and performs early allocations

> Last run: 2026-01-09 10:15:03

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 601ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "boot: phys ranges=" | ✅ | 3979ms | - [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "System halted" | ✅ | 2436ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[15987423342] [INFO] System booted
[16000223646] [INFO] boot: phys ranges=35 modules=0
[16006151370] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[16014865911] [INFO] Initializing Real Frame Allocator...
[16016691933] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[16017396021] [INFO] Allocating bitmap of 8073 words...
[16025133201] [INFO] Bitmap allocated at 0xffffff8040000000
[16031247210] [INFO] Zeroing bitmap...
[16039658316] [INFO] Bitmap zeroed.
[17846689773] [INFO] frame_alloc: total=516638 free=450333 used=66305
[17859241719] [INFO] Running frame_alloc sanity check...
[17915161506] [INFO] frame_alloc: sanity: single ok
[17923536246] [INFO] frame_alloc: sanity: contig(8) ok
[17927827830] [INFO] Testing paging subsystem...
[17937376974] [INFO] Switched to new address space
[17941487256] [INFO] Paging subsystem test passed
[17941990968] [INFO] Paging subsystem test passed
[17942451087] [INFO] Initializing Kernel Heap...
[17949904170] [INFO] kheap: grew by 64 pages (phys=0x1003f000, virt=0xffffa00000000000)
[17954269179] [INFO] global_alloc: switched to kernel heap
[17955372666] [INFO] Running heap sanity check...
[17976178671] [INFO] kheap: sanity ok
[17978961924] [INFO] kheap: forcing growth...
[17982497544] [INFO] kheap: grew by 16 pages (phys=0x10082000, virt=0xffffa00000040000)
[18009745743] [INFO] kheap: big allocation ok (len=307200)
[18023327619] [INFO] kheap: reserved=268435456 committed=327680
[18027732756] [INFO] System halted

```
</details>
