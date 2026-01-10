# ✅ Scenario: Heap Initialization and Basic Allocation

> Last run: 2026-01-09 18:28:27

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booting | ✅ | 245ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the log should contain "Initializing Kernel Heap..." | ✅ | 2269ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the log should contain "kheap: grew by 64 pages" | ✅ | 145ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And the log should contain "global_alloc: switched to kernel heap" | ✅ | 141ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |
| 5 | And the log should contain "kheap: sanity ok" | ✅ | 146ms | <a href="./05/after.png"><img src="./05/after.png" width="150" /></a> [📜](./05/serial.log) [💾](./05/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[7550081220] [INFO] System booted
[7557934230] [INFO] boot: phys ranges=39 modules=3
[7559176812] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[7560423321] [INFO] Initializing Real Frame Allocator...
[7561371576] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[7561739757] [INFO] Allocating bitmap of 8073 words...
[7566457899] [INFO] Bitmap allocated at 0xffffff8040000000
[7567150404] [INFO] Zeroing bitmap...
[7570561779] [INFO] Bitmap zeroed.
[7872999816] [INFO] frame_alloc: total=516638 free=509832 used=6806
[7873552401] [INFO] Running frame_alloc sanity check...
[7875515043] [INFO] frame_alloc: sanity: single ok
[7877602557] [INFO] frame_alloc: sanity: contig(8) ok
[7877893155] [INFO] Paging subsystem test passed (skipped local test)
[7878260478] [INFO] Initializing Kernel Heap...
[7884606345] [INFO] kheap: grew by 64 pages (phys=0x4a000, virt=0xffffa00000000000)
[7885401084] [INFO] global_alloc: switched to kernel heap
[7885714551] [INFO] Running heap sanity check...
[7900888050] [INFO] kheap: sanity ok
[7901339622] [INFO] kheap: forcing growth...
[7903522836] [INFO] kheap: grew by 16 pages (phys=0x8d000, virt=0xffffa00000040000)
[7920730884] [INFO] kheap: big allocation ok (len=307200)
[7921378872] [INFO] kheap: reserved=268435456 committed=327680
[7928621514] [INFO] Registering syscall handler...
[7929142848] [INFO] Initializing Task System...
[7932655137] [INFO] threads: supported
[7935252501] [INFO] Spawning sprout: /boot/modules/sprout
[7935621243] [INFO] Spawning module: /boot/modules/sprout
[8004435912] [INFO] Sprout spawned successfully
[8005299918] [INFO] Entering Scheduler Loop (Main Task)...

```
</details>
