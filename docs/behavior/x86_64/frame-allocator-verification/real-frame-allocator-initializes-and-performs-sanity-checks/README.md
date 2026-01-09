# ✅ Scenario: Real Frame Allocator initializes and performs sanity checks

> Last run: 2026-01-09 10:51:25

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 311ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "Initializing Real Frame Allocator..." | ✅ | 4018ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "frame_alloc: base=" | ✅ | 773ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And the serial output should contain "frame_alloc: total=" | ✅ | 769ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |
| 5 | And the serial output should contain "Running frame_alloc sanity check..." | ✅ | 660ms | <a href="./05/after.png"><img src="./05/after.png" width="150" /></a> - [💾](./05/registers.txt) |
| 6 | And the serial output should contain "frame_alloc: sanity: single ok" | ✅ | 995ms | <a href="./06/after.png"><img src="./06/after.png" width="150" /></a> - [💾](./06/registers.txt) |
| 7 | And the serial output should contain "System halted" | ✅ | 768ms | <a href="./07/after.png"><img src="./07/after.png" width="150" /></a> - [💾](./07/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12613737477] [INFO] System booted
[12626766405] [INFO] boot: phys ranges=35 modules=0
[12628790493] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[12634395741] [INFO] Initializing Real Frame Allocator...
[12636239748] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[12639816750] [INFO] Allocating bitmap of 8073 words...
[12646047711] [INFO] Bitmap allocated at 0xffffff8040000000
[12651175482] [INFO] Zeroing bitmap...
[12656565240] [INFO] Bitmap zeroed.
[13294951329] [INFO] frame_alloc: total=516638 free=511797 used=4841
[13295986341] [INFO] Running frame_alloc sanity check...
[13300795761] [INFO] frame_alloc: sanity: single ok
[13309757439] [INFO] frame_alloc: sanity: contig(8) ok
[13310390148] [INFO] Testing paging subsystem...
[13331206251] [INFO] Switched to new address space
[13334423520] [INFO] Paging subsystem test passed
[13334936274] [INFO] Paging subsystem test passed
[13335382236] [INFO] Initializing Kernel Heap...
[13349131257] [INFO] kheap: grew by 64 pages (phys=0x4f000, virt=0xffffa00000000000)
[13358433429] [INFO] global_alloc: switched to kernel heap
[13358959713] [INFO] Running heap sanity check...
[13399151535] [INFO] kheap: sanity ok
[13403797770] [INFO] kheap: forcing growth...
[13411049223] [INFO] kheap: grew by 16 pages (phys=0x100000, virt=0xffffa00000040000)
[13465873014] [INFO] kheap: big allocation ok (len=307200)
[13488550086] [INFO] kheap: reserved=268435456 committed=327680
[13492937370] [INFO] System halted

```
</details>
