# ✅ Scenario: Real Frame Allocator initializes and performs sanity checks

> Last run: 2026-01-09 10:55:29

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 392ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "Initializing Real Frame Allocator..." | ✅ | 5097ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "frame_alloc: base=" | ✅ | 912ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And the serial output should contain "frame_alloc: total=" | ✅ | 822ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |
| 5 | And the serial output should contain "Running frame_alloc sanity check..." | ✅ | 737ms | <a href="./05/after.png"><img src="./05/after.png" width="150" /></a> - [💾](./05/registers.txt) |
| 6 | And the serial output should contain "frame_alloc: sanity: single ok" | ✅ | 1154ms | <a href="./06/after.png"><img src="./06/after.png" width="150" /></a> - [💾](./06/registers.txt) |
| 7 | And the serial output should contain "System halted" | ✅ | 893ms | <a href="./07/after.png"><img src="./07/after.png" width="150" /></a> - [💾](./07/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[15935193846] [INFO] System booted
[15952404567] [INFO] boot: phys ranges=35 modules=0
[15959822736] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[15963571701] [INFO] Initializing Real Frame Allocator...
[15965693964] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[15970798998] [INFO] Allocating bitmap of 8073 words...
[15980840634] [INFO] Bitmap allocated at 0xffffff8040000000
[15982259502] [INFO] Zeroing bitmap...
[15991306155] [INFO] Bitmap zeroed.
[16648087434] [INFO] frame_alloc: total=516638 free=511797 used=4841
[16653364926] [INFO] Running frame_alloc sanity check...
[16658394522] [INFO] frame_alloc: sanity: single ok
[16665626835] [INFO] frame_alloc: sanity: contig(8) ok
[16666269444] [INFO] Testing paging subsystem...
[16678572405] [INFO] Switched to new address space
[16684072878] [INFO] Paging subsystem test passed
[16688658558] [INFO] Paging subsystem test passed
[16689210483] [INFO] Initializing Kernel Heap...
[16698947463] [INFO] kheap: grew by 64 pages (phys=0x4f000, virt=0xffffa00000000000)
[16707765261] [INFO] global_alloc: switched to kernel heap
[16708528518] [INFO] Running heap sanity check...
[16735558752] [INFO] kheap: sanity ok
[16739751930] [INFO] kheap: forcing growth...
[16745617020] [INFO] kheap: grew by 16 pages (phys=0x100000, virt=0xffffa00000040000)
[16790268627] [INFO] kheap: big allocation ok (len=307200)
[16810700115] [INFO] kheap: reserved=268435456 committed=327680
[16817691858] [INFO] System halted

```
</details>
