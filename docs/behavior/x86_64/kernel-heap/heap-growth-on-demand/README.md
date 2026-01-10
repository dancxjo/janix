# ✅ Scenario: Heap Growth on Demand

> Last run: 2026-01-09 18:28:27

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booting | ✅ | 244ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the log should contain "kheap: forcing growth..." | ✅ | 2266ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> - [💾](./02/registers.txt) |
| 3 | And the log should contain "kheap: grew by 16 pages" | ✅ | 140ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And the log should contain "kheap: big allocation ok" | ✅ | 139ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[7485111876] [INFO] System booted
[7493047782] [INFO] boot: phys ranges=39 modules=3
[7494265779] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[7495507470] [INFO] Initializing Real Frame Allocator...
[7496455857] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[7496839416] [INFO] Allocating bitmap of 8073 words...
[7501597389] [INFO] Bitmap allocated at 0xffffff8040000000
[7502303589] [INFO] Zeroing bitmap...
[7505642364] [INFO] Bitmap zeroed.
[7813001757] [INFO] frame_alloc: total=516638 free=509832 used=6806
[7813569687] [INFO] Running frame_alloc sanity check...
[7815482334] [INFO] frame_alloc: sanity: single ok
[7817649840] [INFO] frame_alloc: sanity: contig(8) ok
[7817944794] [INFO] Paging subsystem test passed (skipped local test)
[7818293769] [INFO] Initializing Kernel Heap...
[7824510309] [INFO] kheap: grew by 64 pages (phys=0x4a000, virt=0xffffa00000000000)
[7825266273] [INFO] global_alloc: switched to kernel heap
[7825578585] [INFO] Running heap sanity check...
[7837335627] [INFO] kheap: sanity ok
[7837559334] [INFO] kheap: forcing growth...
[7839450993] [INFO] kheap: grew by 16 pages (phys=0x8d000, virt=0xffffa00000040000)
[7853974524] [INFO] kheap: big allocation ok (len=307200)
[7854535656] [INFO] kheap: reserved=268435456 committed=327680
[7861648773] [INFO] Registering syscall handler...
[7862163837] [INFO] Initializing Task System...
[7865437107] [INFO] threads: supported
[7867979724] [INFO] Spawning sprout: /boot/modules/sprout
[7868346651] [INFO] Spawning module: /boot/modules/sprout
[7933282830] [INFO] Sprout spawned successfully
[7933763442] [INFO] Entering Scheduler Loop (Main Task)...

```
</details>
