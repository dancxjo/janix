# ✅ Scenario: Real Frame Allocator initializes and performs sanity checks

> Last run: 2026-01-09 10:15:03

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 328ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "Initializing Real Frame Allocator..." | ✅ | 4187ms | - [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "frame_alloc: base=" | ✅ | 23ms | - [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And the serial output should contain "frame_alloc: total=" | ✅ | 16ms | - [📜](./04/serial.log) [💾](./04/registers.txt) |
| 5 | And the serial output should contain "Running frame_alloc sanity check..." | ✅ | 20ms | - - [💾](./05/registers.txt) |
| 6 | And the serial output should contain "frame_alloc: sanity: single ok" | ✅ | 773ms | <a href="./06/after.png"><img src="./06/after.png" width="150" /></a> - [💾](./06/registers.txt) |
| 7 | And the serial output should contain "System halted" | ✅ | 2045ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[15447116190] [INFO] System booted
[15462605631] [INFO] boot: phys ranges=35 modules=0
[15465091587] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[15470505600] [INFO] Initializing Real Frame Allocator...
[15472387260] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[15473066433] [INFO] Allocating bitmap of 8073 words...
[15480905748] [INFO] Bitmap allocated at 0xffffff8040000000
[15487093314] [INFO] Zeroing bitmap...
[15493197258] [INFO] Bitmap zeroed.
[16413300948] [INFO] frame_alloc: total=516638 free=450333 used=66305
[16414339689] [INFO] Running frame_alloc sanity check...
[16551406641] [INFO] frame_alloc: sanity: single ok
[16555573782] [INFO] frame_alloc: sanity: contig(8) ok
[16556210616] [INFO] Testing paging subsystem...
[16575993753] [INFO] Switched to new address space
[16576621908] [INFO] Paging subsystem test passed
[16577207922] [INFO] Paging subsystem test passed
[16577778327] [INFO] Initializing Kernel Heap...
[16591974861] [INFO] kheap: grew by 64 pages (phys=0x1003f000, virt=0xffffa00000000000)
[16593431712] [INFO] global_alloc: switched to kernel heap
[16600404843] [INFO] Running heap sanity check...
[16631811405] [INFO] kheap: sanity ok
[16639799781] [INFO] kheap: forcing growth...
[16646155317] [INFO] kheap: grew by 16 pages (phys=0x10082000, virt=0xffffa00000040000)
[16677230262] [INFO] kheap: big allocation ok (len=307200)
[16691939187] [INFO] kheap: reserved=268435456 committed=327680
[16695465732] [INFO] System halted

```
</details>
