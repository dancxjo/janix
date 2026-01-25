# ✅ Scenario: Ingestd detects PNG content

> Last run: 2026-01-25 19:33:17

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 14459ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "SPROUT: Launching app '/boot/ingestd'" | ✅ | 730ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "SPROUT: Launching app '/boot/png_creator'" | ✅ | 648ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> - [💾](./03/registers.txt) |
| 4 | And I wait for 5 seconds | ⏭️ | 207ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[27157012515] [INFO] [kernel] thing-os kernel starting...
[27355941878] [INFO] [kernel::memory] Memory map has 64 entries
[27360450612] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[27363625653] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[27364040485] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[27364454519] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[27364764058] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[27365068112] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[27365376476] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[27365679548] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[27366019321] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78a97000 (Usable)
[27366411435] [INFO] [kernel::memory]   [9] 0x78a97000 - 0x78af6000 (Reserved)
[27366762451] [INFO] [kernel::memory]   [10] 0x78af6000 - 0x78b81000 (Other)
[27367087424] [INFO] [kernel::memory]   [11] 0x78b81000 - 0x78b82000 (Reserved)
[27367421851] [INFO] [kernel::memory]   [12] 0x78b82000 - 0x78b88000 (Other)
[27367745712] [INFO] [kernel::memory]   [13] 0x78b88000 - 0x78b89000 (Reserved)
[27368081742] [INFO] [kernel::memory]   [14] 0x78b89000 - 0x78c20000 (Other)
[27368467640] [INFO] [kernel::memory]   [15] 0x78c20000 - 0x78c21000 (Reserved)
[27368821456] [INFO] [kernel::memory]   [16] 0x78c21000 - 0x78c61000 (Other)
[27369144300] [INFO] [kernel::memory]   [17] 0x78c61000 - 0x78c62000 (Reserved)
[27369476425] [INFO] [kernel::memory]   [18] 0x78c62000 - 0x78cae000 (Other)
[27369796527] [INFO] [kernel::memory]   [19] 0x78cae000 - 0x78caf000 (Reserved)
[27370128986] [INFO] [kernel::memory]   [20] 0x78caf000 - 0x78d50000 (Other)
[27370449635] [INFO] [kernel::memory]   [21] 0x78d50000 - 0x78d51000 (Reserved)
[27370844319] [INFO] [kernel::memory]   [22] 0x78d51000 - 0x78d53000 (Other)
[27371191994] [INFO] [kernel::memory]   [23] 0x78d53000 - 0x78d54000 (Reserved)
[27371527316] [INFO] [kernel::memory]   [24] 0x78d54000 - 0x78d56000 (Other)
[27371851469] [INFO] [kernel::memory]   [25] 0x78d56000 - 0x78d57000 (Reserved)
[27373563722] [INFO] [kernel::memory]   [26] 0x78d57000 - 0x78d61000 (Other)
[27377267444] [INFO] [kernel::memory]   [27] 0x78d61000 - 0x78d62000 (Reserved)
[27381299261] [INFO] [kernel::memory]   [28] 0x78d62000 - 0x78d66000 (Other)
[27384248780] [INFO] [kernel::memory]   [29] 0x78d66000 - 0x78d67000 (Reserved)
[27384668318] [INFO] [kernel::memory]   [30] 0x78d67000 - 0x78d69000 (Other)
[27385012983] [INFO] [kernel::memory]   [31] 0x78d69000 - 0x78d6a000 (Reserved)
[27385349630] [INFO] [kernel::memory]   [32] 0x78d6a000 - 0x78d6c000 (Other)
[27385674392] [INFO] [kernel::memory]   [33] 0x78d6c000 - 0x78d6d000 (Reserved)
[27386009730] [INFO] [kernel::memory]   [34] 0x78d6d000 - 0x78d6f000 (Other)
[27386333411] [INFO] [kernel::memory]   [35] 0x78d6f000 - 0x78d70000 (Reserved)
[27386668680] [INFO] [kernel::memory]   [36] 0x78d70000 - 0x78d71000 (Other)
[27387054850] [INFO] [kernel::memory]   [37] 0x78d71000 - 0x78d72000 (Reserved)
[27387405593] [INFO] [kernel::memory]   [38] 0x78d72000 - 0x78d73000 (Other)
[27387726758] [INFO] [kernel::memory]   [39] 0x78d73000 - 0x78d74000 (Reserved)
[27388060487] [INFO] [kernel::memory]   [40] 0x78d74000 - 0x78d78000 (Other)
[27388383024] [INFO] [kernel::memory]   [41] 0x78d78000 - 0x78d79000 (Reserved)
[27388716008] [INFO] [kernel::memory]   [42] 0x78d79000 - 0x78d7b000 (Other)
[27389037211] [INFO] [kernel::memory]   [43] 0x78d7b000 - 0x78d7c000 (Reserved)
[27389452808] [INFO] [kernel::memory]   [44] 0x78d7c000 - 0x78d80000 (Other)
[27389781694] [INFO] [kernel::memory]   [45] 0x78d80000 - 0x78d81000 (Reserved)
[27390116489] [INFO] [kernel::memory]   [46] 0x78d81000 - 0x78d85000 (Other)
[27390440649] [INFO] [kernel::memory]   [47] 0x78d85000 - 0x78d86000 (Reserved)
[27394596886] [INFO] [kernel::memory]   [48] 0x78d86000 - 0x78d8f000 (Other)
[27398342915] [INFO] [kernel::memory]   [49] 0x78d8f000 - 0x78d90000 (Reserved)
[27399370517] [INFO] [kernel::memory]   [50] 0x78d90000 - 0x78d92000 (Other)
[27399699493] [INFO] [kernel::memory]   [51] 0x78d92000 - 0x78d93000 (Reserved)
[27400142202] [INFO] [kernel::memory]   [52] 0x78d93000 - 0x78d95000 (Other)
[27400468542] [INFO] [kernel::memory]   [53] 0x78d95000 - 0x78d96000 (Reserved)
[27400877501] [INFO] [kernel::memory]   [54] 0x78d96000 - 0x78d98000 (Other)
[27401218580] [INFO] [kernel::memory]   [55] 0x78d98000 - 0x78d99000 (Reserved)
[27401552392] [INFO] [kernel::memory]   [56] 0x78d99000 - 0x78f1c000 (Other)
[27401874534] [INFO] [kernel::memory]   [57] 0x78f1c000 - 0x7939d000 (Other)
[27402196685] [INFO] [kernel::memory]   [58] 0x7939d000 - 0x7969e000 (Other)
[27402518074] [INFO] [kernel::memory]   [59] 0x7969e000 - 0x796f2000 (Other)
[27402839946] [INFO] [kernel::memory]   [60] 0x796f2000 - 0x798b6000 (Other)
[27403251300] [INFO] [kernel::memory]   [61] 0x798b6000 - 0x7a16c000 (Reserved)
[27403591523] [INFO] [kernel::memory]   [62] 0x7a16c000 - 0x7bb6c000 (Usable)
[27403918994] [INFO] [kernel::memory]   [63] 0x7bb6c000 - 0x7bb8f000 (Reserved)
[27404498896] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[27749152780] [INFO] [kernel::memory] Frame allocator initialized with 496815 free frames
[27763676586] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[27778153790] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[27780577556] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[27781357263] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[27790359802] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[27795828032] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[27803105314] [INFO] [bran::arch] IOAPIC: Registers initialized
[27804901887] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[27809058679] [INFO] [bran::arch] IOAPIC: All pins masked
[27810722490] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[27811407076] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[27811890150] [INFO] [bran::arch] IOAPIC: Init complete
[27812428613] [INFO] [kernel] Initializing global allocator...
[28331779848] [INFO] [kernel::memory::arena] ArenaHeap: Adding arena 'global.pinned' base=ffffffffb0000000 size=33554432 flags=ArenaFlags(PINNED)
[28343656062] [INFO] [kernel::memory::global_alloc] Arena allocator initialized (pinned pre-expanded)
[28344331710] [INFO] [kernel] Initializing SIMD...
[28346438523] [INFO] [kernel] Initializing tasking...
[28357119224] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[28361698503] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[28366596470] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[28405035308] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[28408855779] [INFO] [kernel::task::scheduler]   Initializing boot task...
[28413435338] [INFO] [kernel::task::scheduler]   Creating boot task...
[28422693727] [INFO] [kernel::task::scheduler]   Creating idle task...
[28446788505] [INFO] [kernel::task::scheduler]   Boot task initialized
[28450809364] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[28455397662] [INFO] [kernel::task::scheduler]   Scheduler initialized
[28464069053] [INFO] [kernel::root] Spawning Root service...
[28487646693] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[28509513801] [INFO] [kernel::root::service] ROOT: started once
[29676993191] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[29681913736] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[29746531736] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[29775865747] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[29832326930] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[29905566675] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[29908239217] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[29954106924] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[29985352771] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[29994502142] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[30007631039] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=194, idx=3) BAR5=0x810c4000
[30037907399] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[30049843426] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[30054496239] [INFO] [logging] BEGIN rootdump
[30055289853] [INFO] [kernel::root::handlers::debug] ROOT DUMP NODES count=200
(entry:1:log.Entry { level: 3, line: 201, timestamp: 0x6a13ec745, message: 11, event: 10, file: 12, module: 10, tid: 0 })
(entry:2:log.Entry { level: 3, line: 28, timestamp: 0x6a287a6f1, message: 14, event: 13, file: 15, module: 13, tid: 0 })
(host:3:dev.Host { hhdm_offset: 0xffff800000000000, arch: "x86_64", platform_profile: "unknown", source: 0, confidence: 2 })
(entry:4:log.Entry { level: 3, line: 13, timestamp: 0x6a38217c9, message: 18, event: 17, file: 19, module: 17, tid: 2 })
(platform:5:dev.bus.Platform { source: 0, confidence: 2, name: "platform0" })
(kernel:6:proc.Kernel { version: 1 })
(root:7:svc.Root {  })
(cpu:8:dev.Cpu { source: 0, confidence: 2, id: 0 })
(range:9:mem.Range { source: 0, confidence: 2, start: 0, end: 0xa0000, kind: 0 })
(range:A:mem.Range { source: 0, confidence: 2, start: 0x100000, end: 0x800000, kind: 0 })
(range:B:mem.Range { source: 0, confidence: 2, start: 0x800000, end: 0x808000, kind: 8 })
(range:C:mem.Range { source: 0, confidence: 2, start: 0x808000, end: 0x80b000, kind: 0 })
(range:D:mem.Range { source: 0, confidence: 2, start: 0x80b000, end: 0x80c000, kind: 8 })
(range:E:mem.Range { source: 0, confidence: 2, start: 0x80c000, end: 0x811000, kind: 0 })
(range:F:mem.Range { source: 0, confidence: 2, start: 0x811000, end: 0x900000, kind: 8 })
(range:10:mem.Range { source: 0, confidence: 2, start: 0x900000, end: 0x1780000, kind: 1 })
(range:11:mem.Range { source: 0, confidence: 2, start: 0x1780000, end: 0x78a97000, kind: 0 })
(range:12:mem.Range { source: 0, confidence: 2, start: 0x78a97000, end: 0x78af6000, kind: 1 })
(range:13:mem.Range { source: 0, confidence: 2, start: 0x78af6000, end: 0x78b81000, kind: 8 })
(range:14:mem.Range { source: 0, confidence: 2, start: 0x78b81000, end: 0x78b82000, kind: 1 })
(range:15:mem.Range { source: 0, confidence: 2, start: 0x78b82000, end: 0x78b88000, kind: 8 })
(range:16:mem.Range { source: 0, confidence: 2, start: 0x78b88000, end: 0x78b89000, kind: 1 })
(range:17:mem.Range { source: 0, confidence: 2, start: 0x78b89000, end: 0x78c20000, kind: 8 })
(range:18:mem.Range { source: 0, confidence: 2, start: 0x78c20000, end: 0x78c21000, kind: 1 })
(range:19:mem.Range { source: 0, confidence: 2, start: 0x78c21000, end: 0x78c61000, kind: 8 })
(range:1A:mem.Range { source: 0, confidence: 2, start: 0x78c61000, end: 0x78c62000, kind: 1 })
(range:1B:mem.Range { source: 0, confidence: 2, start: 0x78c62000, end: 0x78cae000, kind: 8 })
(range:1C:mem.Range { source: 0, confidence: 2, start: 0x78cae000, end: 0x78caf000, kind: 1 })
(range:1D:mem.Range { source: 0, confidence: 2, start: 0x78caf000, end: 0x78d50000, kind: 8 })
(range:1E:mem.Range { source: 0, confidence: 2, start: 0x78d50000, end: 0x78d51000, kind: 1 })
(range:1F:mem.Range { source: 0, confidence: 2, start: 0x78d51000, end: 0x78d53000, kind: 8 })
(range:20:mem.Range { source: 0, confidence: 2, start: 0x78d53000, end: 0x78d54000, kind: 1 })
(range:21:mem.Range { source: 0, confidence: 2, start: 0x78d54000, end: 0x78d56000, kind: 8 })
(range:22:mem.Range { source: 0, confidence: 2, start: 0x78d56000, end: 0x78d57000, kind: 1 })
(range:23:mem.Range { source: 0, confidence: 2, start: 0x78d57000, end: 0x78d61000, kind: 8 })
(range:24:mem.Range { source: 0, confidence: 2, start: 0x78d61000, end: 0x78d62000, kind: 1 })
(range:25:mem.Range { source: 0, confidence: 2, start: 0x78d62000, end: 0x78d66000, kind: 8 })
(range:26:mem.Range { source: 0, confidence: 2, start: 0x78d66000, end: 0x78d67000, kind: 1 })
(range:27:mem.Range { source: 0, confidence: 2, start: 0x78d67000, end: 0x78d69000, kind: 8 })
(range:28:mem.Range { source: 0, confidence: 2, start: 0x78d69000, end: 0x78d6a000, kind: 1 })
(range:29:mem.Range { source: 0, confidence: 2, start: 0x78d6a000, end: 0x78d6c000, kind: 8 })
(range:2A:mem.Range { source: 0, confidence: 2, start: 0x78d6c000, end: 0x78d6d000, kind: 1 })
(range:2B:mem.Range { source: 0, confidence: 2, start: 0x78d6d000, end: 0x78d6f000, kind: 8 })
(range:2C:mem.Range { source: 0, confidence: 2, start: 0x78d6f000, end: 0x78d70000, kind: 1 })
(range:2D:mem.Range { source: 0, confidence: 2, start: 0x78d70000, end: 0x78d71000, kind: 8 })
(range:2E:mem.Range { source: 0, confidence: 2, start: 0x78d71000, end: 0x78d72000, kind: 1 })
(range:2F:mem.Range { source: 0, confidence: 2, start: 0x78d72000, end: 0x78d73000, kind: 8 })
(range:30:mem.Range { source: 0, confidence: 2, start: 0x78d73000, end: 0x78d74000, kind: 1 })
(range:31:mem.Range { source: 0, confidence: 2, start: 0x78d74000, end: 0x78d78000, kind: 8 })
(range:32:mem.Range { source: 0, confidence: 2, start: 0x78d78000, end: 0x78d79000, kind: 1 })
(range:33:mem.Range { source: 0, confidence: 2, start: 0x78d79000, end: 0x78d7b000, kind: 8 })
(range:34:mem.Range { source: 0, confidence: 2, start: 0x78d7b000, end: 0x78d7c000, kind: 1 })
(range:35:mem.Range { source: 0, confidence: 2, start: 0x78d7c000, end: 0x78d80000, kind: 8 })
(range:36:mem.Range { source: 0, confidence: 2, start: 0x78d80000, end: 0x78d81000, kind: 1 })
(range:37:mem.Range { source: 0, confidence: 2, start: 0x78d81000, end: 0x78d85000, kind: 8 })
(range:38:mem.Range { source: 0, confidence: 2, start: 0x78d85000, end: 0x78d86000, kind: 1 })
(range:39:mem.Range { source: 0, confidence: 2, start: 0x78d86000, end: 0x78d8f000, kind: 8 })
(range:3A:mem.Range { source: 0, confidence: 2, start: 0x78d8f000, end: 0x78d90000, kind: 1 })
(range:3B:mem.Range { source: 0, confidence: 2, start: 0x78d90000, end: 0x78d92000, kind: 8 })
(range:3C:mem.Range { source: 0, confidence: 2, start: 0x78d92000, end: 0x78d93000, kind: 1 })
(range:3D:mem.Range { source: 0, confidence: 2, start: 0x78d93000, end: 0x78d95000, kind: 8 })
(range:3E:mem.Range { source: 0, confidence: 2, start: 0x78d95000, end: 0x78d96000, kind: 1 })
(range:3F:mem.Range { source: 0, confidence: 2, start: 0x78d96000, end: 0x78d98000, kind: 8 })
(range:40:mem.Range { source: 0, confidence: 2, start: 0x78d98000, end: 0x78d99000, kind: 1 })
(range:41:mem.Range { source: 0, confidence: 2, start: 0x78d99000, end: 0x78f1c000, kind: 8 })
(range:42:mem.Range { source: 0, confidence: 2, start: 0x78f1c000, end: 0x7939d000, kind: 8 })
(range:43:mem.Range { source: 0, confidence: 2, start: 0x7939d000, end: 0x7969e000, kind: 8 })
(range:44:mem.Range { source: 0, confidence: 2, start: 0x7969e000, end: 0x796f2000, kind: 8 })
(range:45:mem.Range { source: 0, confidence: 2, start: 0x796f2000, end: 0x798b6000, kind: 8 })
(range:46:mem.Range { source: 0, confidence: 2, start: 0x798b6000, end: 0x7a16c000, kind: 1 })
(range:47:mem.Range { source: 0, confidence: 2, start: 0x7a16c000, end: 0x7bb6c000, kind: 0 })
(range:48:mem.Range { source: 0, confidence: 2, start: 0x7bb6c000, end: 0x7bb8f000, kind: 1 })
(module:49:boot.Module { source: 0, confidence: 2, name: "/boot/sprout", phys_base: 0x7bbd8000, size_bytes: 0x11ab0, index: 0, bytespace: 74 })
(bytespace:4A:Bytespace {  })
(range:4B:mem.Range { phys_base: 0x7bbd8000, size_bytes: 0x11ab0 })
(module:4C:boot.Module { source: 0, confidence: 2, name: "/boot/bristle", phys_base: 0x7e24f000, size_bytes: 16776, index: 1, bytespace: 77 })
(bytespace:4D:Bytespace {  })
(range:4E:mem.Range { phys_base: 0x7e24f000, size_bytes: 16776 })
(module:4F:boot.Module { source: 0, confidence: 2, name: "/boot/rtc_cmos", phys_base: 0x7bbd2000, size_bytes: 20088, index: 2, bytespace: 80 })
(bytespace:50:Bytespace {  })
(range:51:mem.Range { phys_base: 0x7bbd2000, size_bytes: 20088 })
(module:52:boot.Module { source: 0, confidence: 2, name: "/boot/clock", phys_base: 0x7bbcb000, size_bytes: 20960, index: 3, bytespace: 83 })
(bytespace:53:Bytespace {  })
(range:54:mem.Range { phys_base: 0x7bbcb000, size_bytes: 20960 })
(module:55:boot.Module { source: 0, confidence: 2, name: "/boot/ps2_kbd", phys_base: 0x7bbc6000, size_bytes: 12760, index: 4, bytespace: 86 })
(bytespace:56:Bytespace {  })
(range:57:mem.Range { phys_base: 0x7bbc6000, size_bytes: 12760 })
(module:58:boot.Module { source: 0, confidence: 2, name: "/boot/echo", phys_base: 0x7bbc0000, size_bytes: 16824, index: 5, bytespace: 89 })
(bytespace:59:Bytespace {  })
(range:5A:mem.Range { phys_base: 0x7bbc0000, size_bytes: 16824 })
(module:5B:boot.Module { source: 0, confidence: 2, name: "/boot/bloom", phys_base: 0x7969e000, size_bytes: 0x53200, index: 6, bytespace: 92 })
(bytespace:5C:Bytespace {  })
(range:5D:mem.Range { phys_base: 0x7969e000, size_bytes: 0x53200 })
(module:5E:boot.Module { source: 0, confidence: 2, name: "/boot/ps2_mouse", phys_base: 0x7bbb9000, size_bytes: 17768, index: 7, bytespace: 95 })
(bytespace:5F:Bytespace {  })
(range:60:mem.Range { phys_base: 0x7bbb9000, size_bytes: 17768 })
(module:61:boot.Module { source: 0, confidence: 2, name: "/boot/root_batch_bench", phys_base: 0x7bbb3000, size_bytes: 16608, index: 8, bytespace: 98 })
(bytespace:62:Bytespace {  })
(range:63:mem.Range { phys_base: 0x7bbb3000, size_bytes: 16608 })
(module:64:boot.Module { source: 0, confidence: 2, name: "/boot/root_watch_tester", phys_base: 0x7bbab000, size_bytes: 25576, index: 9, bytespace: 101 })
(bytespace:65:Bytespace {  })
(range:66:mem.Range { phys_base: 0x7bbab000, size_bytes: 25576 })
(module:67:boot.Module { source: 0, confidence: 2, name: "/boot/ingestd", phys_base: 0x7bba4000, size_bytes: 22696, index: 10, bytespace: 104 })
(bytespace:68:Bytespace {  })
(range:69:mem.Range { phys_base: 0x7bba4000, size_bytes: 22696 })
(module:6A:boot.Module { source: 0, confidence: 2, name: "/boot/bindd", phys_base: 0x7bb9f000, size_bytes: 14440, index: 11, bytespace: 107 })
(bytespace:6B:Bytespace {  })
(range:6C:mem.Range { phys_base: 0x7bb9f000, size_bytes: 14440 })
(module:6D:boot.Module { source: 0, confidence: 2, name: "/boot/png_creator", phys_base: 0x7bb9c000, size_bytes: 5256, index: 12, bytespace: 110 })
(bytespace:6E:Bytespace {  })
(range:6F:mem.Range { phys_base: 0x7bb9c000, size_bytes: 5256 })
(module:70:boot.Module { source: 0, confidence: 2, name: "/boot/scheduler_fairness", phys_base: 0x7bb97000, size_bytes: 13400, index: 13, bytespace: 113 })
(bytespace:71:Bytespace {  })
(range:72:mem.Range { phys_base: 0x7bb97000, size_bytes: 13400 })
(module:73:boot.Module { source: 0, confidence: 2, name: "/boot/hogger", phys_base: 0x7bb93000, size_bytes: 10016, index: 14, bytespace: 116 })
(bytespace:74:Bytespace {  })
(range:75:mem.Range { phys_base: 0x7bb93000, size_bytes: 10016 })
(module:76:boot.Module { source: 0, confidence: 2, name: "/boot/tick_printer", phys_base: 0x7bb8f000, size_bytes: 10728, index: 15, bytespace: 119 })
(bytespace:77:Bytespace {  })
(range:78:mem.Range { phys_base: 0x7bb8f000, size_bytes: 10728 })
(module:79:boot.Module { source: 0, confidence: 2, name: "/assets/wallpapers/clouds.bmp", phys_base: 0x7939d000, size_bytes: 0x300036, index: 16, bytespace: 122 })
(bytespace:7A:Bytespace {  })
(range:7B:mem.Range { phys_base: 0x7939d000, size_bytes: 0x300036 })
(module:7C:boot.Module { source: 0, confidence: 2, name: "/assets/wallpapers/leather.bmp", phys_base: 0x78f1c000, size_bytes: 0x480036, index: 17, bytespace: 125 })
(bytespace:7D:Bytespace {  })
(range:7E:mem.Range { phys_base: 0x78f1c000, size_bytes: 0x480036 })
(module:7F:boot.Module { source: 0, confidence: 2, name: "/assets/pci/pci.ids", phys_base: 0x78d99000, size_bytes: 0x182422, index: 18, bytespace: 128 })
(bytespace:80:Bytespace {  })
(range:81:mem.Range { phys_base: 0x78d99000, size_bytes: 0x182422 })
(module:82:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Move.cur", phys_base: 0x78d96000, size_bytes: 4286, index: 19, bytespace: 131 })
(bytespace:83:Bytespace {  })
(range:84:mem.Range { phys_base: 0x78d96000, size_bytes: 4286 })
(module:85:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Normal.cur", phys_base: 0x78d93000, size_bytes: 4286, index: 20, bytespace: 134 })
(bytespace:86:Bytespace {  })
(range:87:mem.Range { phys_base: 0x78d93000, size_bytes: 4286 })
(module:88:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Unavailabe.cur", phys_base: 0x78d90000, size_bytes: 4286, index: 21, bytespace: 137 })
(bytespace:89:Bytespace {  })
(range:8A:mem.Range { phys_base: 0x78d90000, size_bytes: 4286 })
(module:8B:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Working.ani", phys_base: 0x78d86000, size_bytes: 34470, index: 22, bytespace: 140 })
(bytespace:8C:Bytespace {  })
(range:8D:mem.Range { phys_base: 0x78d86000, size_bytes: 34470 })
(module:8E:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Horizontal.ani", phys_base: 0x78d81000, size_bytes: 13030, index: 23, bytespace: 143 })
(bytespace:8F:Bytespace {  })
(range:90:mem.Range { phys_base: 0x78d81000, size_bytes: 13030 })
(module:91:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Diagonal1.ani", phys_base: 0x78d7c000, size_bytes: 13030, index: 24, bytespace: 146 })
(bytespace:92:Bytespace {  })
(range:93:mem.Range { phys_base: 0x78d7c000, size_bytes: 13030 })
(module:94:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Help.cur", phys_base: 0x78d79000, size_bytes: 4286, index: 25, bytespace: 149 })
(bytespace:95:Bytespace {  })
(range:96:mem.Range { phys_base: 0x78d79000, size_bytes: 4286 })
(module:97:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Vertical.ani", phys_base: 0x78d74000, size_bytes: 13028, index: 26, bytespace: 152 })
(bytespace:98:Bytespace {  })
(range:99:mem.Range { phys_base: 0x78d74000, size_bytes: 13028 })
(module:9A:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/readme.txt", phys_base: 0x78d72000, size_bytes: 308, index: 27, bytespace: 155 })
(bytespace:9B:Bytespace {  })
(range:9C:mem.Range { phys_base: 0x78d72000, size_bytes: 308 })
(module:9D:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/plain.crs", phys_base: 0x78d70000, size_bytes: 457, index: 28, bytespace: 158 })
(bytespace:9E:Bytespace {  })
(range:9F:mem.Range { phys_base: 0x78d70000, size_bytes: 457 })
(module:A0:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Busy.cur", phys_base: 0x78d6d000, size_bytes: 4286, index: 29, bytespace: 161 })
(bytespace:A1:Bytespace {  })
(range:A2:mem.Range { phys_base: 0x78d6d000, size_bytes: 4286 })
(module:A3:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Alternate.cur", phys_base: 0x78d6a000, size_bytes: 4286, index: 30, bytespace: 164 })
(bytespace:A4:Bytespace {  })
(range:A5:mem.Range { phys_base: 0x78d6a000, size_bytes: 4286 })
(module:A6:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Text.cur", phys_base: 0x78d67000, size_bytes: 4286, index: 31, bytespace: 167 })
(bytespace:A7:Bytespace {  })
(range:A8:mem.Range { phys_base: 0x78d67000, size_bytes: 4286 })
(framebuffer:A9:dev.display.Framebuffer { source: 0, confidence: 2, phys_base: 0x80000000, size_bytes: 0x384000, virt_base: 0xffff800080000000, width: 1280, height: 720, stride: 5120, ... })
(boot:AA:fw.Boot { source: 0, confidence: 2 })
(acpi:AB:fw.table.Acpi { source: 0, confidence: 2, phys_base: 0x7f77e014 })
(bytespace:AC:Bytespace {  })
(range:AD:mem.Range { phys_base: 0x2ffc000, size_bytes: 4096, page_count: 1 })
(scheduler:AE:svc.Scheduler {  })
(entry:AF:log.Entry { level: 3, line: 335, timestamp: 0x6e923fd4d, message: 98, event: 13, file: 15, module: 13, tid: 0 })
(entry:B0:log.Entry { level: 3, line: 108, timestamp: 0x6e965b75c, message: 100, event: 99, file: 101, module: 99, tid: 0 })
(pci:B1:dev.bus.Pci { source: 4, confidence: 2, name: "pci0", segment: 0 })
(function:B2:dev.pci.Function { source: 4, confidence: 2, name: "Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller", bus: 0, device: 0, function: 0, vendor_id: 32902, device_id: 10688, ... })
(entry:B3:log.Entry { level: 3, line: 261, timestamp: 0x6ed34732e, message: 126, event: 99, file: 101, module: 99, tid: 0 })
(function:B4:dev.pci.Function { source: 4, confidence: 2, bus: 0, device: 1, function: 0, vendor_id: 4660, device_id: 4369, class_code: 3, ... })
(entry:B5:log.Entry { level: 3, line: 261, timestamp: 0x6ef1c23a2, message: 128, event: 99, file: 101, module: 99, tid: 0 })
(function:B6:dev.pci.Function { source: 4, confidence: 2, name: "Intel Corporation 82574L Gigabit Network Connection", bus: 0, device: 2, function: 0, vendor_id: 32902, device_id: 4307, ... })
(entry:B7:log.Entry { level: 3, line: 261, timestamp: 0x6f270c980, message: 134, event: 99, file: 101, module: 99, tid: 0 })
(function:B8:dev.pci.Function { source: 4, confidence: 2, name: "Intel Corporation 82801IB (ICH9) LPC Interface Controller", bus: 0, device: 31, function: 0, vendor_id: 32902, device_id: 10520, ... })
(entry:B9:log.Entry { level: 3, line: 261, timestamp: 0x6f6a13a62, message: 142, event: 99, file: 101, module: 99, tid: 0 })
(entry:BA:log.Entry { level: 3, line: 364, timestamp: 0x6f6b2be9f, message: 143, event: 99, file: 101, module: 99, tid: 0 })
(lpc:BB:dev.bridge.Lpc { source: 4, confidence: 2, name: "lpc0" })
(legacyio:BC:dev.bus.LegacyIo { source: 4, confidence: 2, name: "isa0" })
(cmos:BD:dev.rtc.Cmos { source: 4, confidence: 2, name: "rtc0" })
(range:BE:cap.ioport.Range { port_start: 112, port_end: 113 })
(ps2controller:BF:dev.input.Ps2Controller { source: 4, confidence: 2, name: "i8042" })
(range:C0:cap.ioport.Range { port_start: 96, port_end: 100 })
(entry:C1:log.Entry { level: 3, line: 576, timestamp: 0x6f9b7694c, message: 157, event: 99, file: 101, module: 99, tid: 0 })
(function:C2:dev.pci.Function { source: 4, confidence: 2, name: "Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode]", bus: 0, device: 31, function: 2, vendor_id: 32902, device_id: 10530, ... })
(entry:C3:log.Entry { level: 3, line: 261, timestamp: 0x6fb5d932f, message: 161, event: 99, file: 101, module: 99, tid: 0 })
(entry:C4:log.Entry { level: 3, line: 375, timestamp: 0x6fc1f822e, message: 163, event: 99, file: 101, module: 99, tid: 0 })
(entry:C5:log.Entry { level: 3, line: 490, timestamp: 0x6fcf1f5b7, message: 164, event: 99, file: 101, module: 99, tid: 0 })
(function:C6:dev.pci.Function { source: 4, confidence: 2, name: "Intel Corporation 82801I (ICH9 Family) SMBus Controller", bus: 0, device: 31, function: 3, vendor_id: 32902, device_id: 10544, ... })
(entry:C7:log.Entry { level: 3, line: 261, timestamp: 0x6fef9686a, message: 168, event: 99, file: 101, module: 99, tid: 0 })
(entry:C8:log.Entry { level: 3, line: 338, timestamp: 0x6ff3367c9, message: 169, event: 13, file: 15, module: 13, tid: 0 })
[30327722159] [INFO] [kernel::root::handlers::debug] ROOT DUMP EDGES
(host:3:dev.Host)-[:HAS_BUS]->(platform:5:dev.bus.Platform)
(host:3:dev.Host)-[:HAS_CPU]->(cpu:8:dev.Cpu)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:9:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:A:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:B:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:C:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:D:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:E:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:F:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:10:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:11:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:12:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:13:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:14:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:15:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:16:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:17:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:18:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:19:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:1A:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:1B:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:1C:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:1D:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:1E:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:1F:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:20:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:21:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:22:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:23:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:24:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:25:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:26:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:27:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:28:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:29:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:2A:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:2B:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:2C:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:2D:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:2E:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:2F:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:30:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:31:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:32:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:33:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:34:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:35:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:36:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:37:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:38:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:39:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:3A:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:3B:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:3C:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:3D:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:3E:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:3F:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:40:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:41:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:42:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:43:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:44:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:45:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:46:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:47:mem.Range)
(host:3:dev.Host)-[:HAS_MEMORY_RANGE]->(range:48:mem.Range)
(host:3:dev.Host)-[:HAS_MODULE]->(module:49:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:4C:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:4F:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:52:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:55:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:58:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:5B:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:5E:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:61:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:64:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:67:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:6A:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:6D:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:70:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:73:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:76:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:79:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:7C:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:7F:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:82:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:85:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:88:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:8B:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:8E:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:91:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:94:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:97:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:9A:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:9D:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:A0:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:A3:boot.Module)
(host:3:dev.Host)-[:HAS_MODULE]->(module:A6:boot.Module)
(host:3:dev.Host)-[:HAS_DEVICE]->(framebuffer:A9:dev.display.Framebuffer)
(host:3:dev.Host)-[:HAS_FIRMWARE]->(boot:AA:fw.Boot)
(host:3:dev.Host)-[:HAS_BUS]->(pci:B1:dev.bus.Pci)
(kernel:6:proc.Kernel)-[:RUNS_ON]->(host:3:dev.Host)
(kernel:6:proc.Kernel)-[:PROVIDES]->(root:7:svc.Root)
(kernel:6:proc.Kernel)-[:PROVIDES]->(scheduler:AE:svc.Scheduler)
(module:49:boot.Module)-[:BACKED_BY]->(bytespace:4A:Bytespace)
(bytespace:4A:Bytespace)-[:BACKED_BY]->(range:4B:mem.Range)
(module:4C:boot.Module)-[:BACKED_BY]->(bytespace:4D:Bytespace)
(bytespace:4D:Bytespace)-[:BACKED_BY]->(range:4E:mem.Range)
(module:4F:boot.Module)-[:BACKED_BY]->(bytespace:50:Bytespace)
(bytespace:50:Bytespace)-[:BACKED_BY]->(range:51:mem.Range)
(module:52:boot.Module)-[:BACKED_BY]->(bytespace:53:Bytespace)
(bytespace:53:Bytespace)-[:BACKED_BY]->(range:54:mem.Range)
(module:55:boot.Module)-[:BACKED_BY]->(bytespace:56:Bytespace)
(bytespace:56:Bytespace)-[:BACKED_BY]->(range:57:mem.Range)
(module:58:boot.Module)-[:BACKED_BY]->(bytespace:59:Bytespace)
(bytespace:59:Bytespace)-[:BACKED_BY]->(range:5A:mem.Range)
(module:5B:boot.Module)-[:BACKED_BY]->(bytespace:5C:Bytespace)
(bytespace:5C:Bytespace)-[:BACKED_BY]->(range:5D:mem.Range)
(module:5E:boot.Module)-[:BACKED_BY]->(bytespace:5F:Bytespace)
(bytespace:5F:Bytespace)-[:BACKED_BY]->(range:60:mem.Range)
(module:61:boot.Module)-[:BACKED_BY]->(bytespace:62:Bytespace)
(bytespace:62:Bytespace)-[:BACKED_BY]->(range:63:mem.Range)
(module:64:boot.Module)-[:BACKED_BY]->(bytespace:65:Bytespace)
(bytespace:65:Bytespace)-[:BACKED_BY]->(range:66:mem.Range)
(module:67:boot.Module)-[:BACKED_BY]->(bytespace:68:Bytespace)
(bytespace:68:Bytespace)-[:BACKED_BY]->(range:69:mem.Range)
(module:6A:boot.Module)-[:BACKED_BY]->(bytespace:6B:Bytespace)
(bytespace:6B:Bytespace)-[:BACKED_BY]->(range:6C:mem.Range)
(module:6D:boot.Module)-[:BACKED_BY]->(bytespace:6E:Bytespace)
(bytespace:6E:Bytespace)-[:BACKED_BY]->(range:6F:mem.Range)
(module:70:boot.Module)-[:BACKED_BY]->(bytespace:71:Bytespace)
(bytespace:71:Bytespace)-[:BACKED_BY]->(range:72:mem.Range)
(module:73:boot.Module)-[:BACKED_BY]->(bytespace:74:Bytespace)
(bytespace:74:Bytespace)-[:BACKED_BY]->(range:75:mem.Range)
(module:76:boot.Module)-[:BACKED_BY]->(bytespace:77:Bytespace)
(bytespace:77:Bytespace)-[:BACKED_BY]->(range:78:mem.Range)
(module:79:boot.Module)-[:BACKED_BY]->(bytespace:7A:Bytespace)
(bytespace:7A:Bytespace)-[:BACKED_BY]->(range:7B:mem.Range)
(module:7C:boot.Module)-[:BACKED_BY]->(bytespace:7D:Bytespace)
(bytespace:7D:Bytespace)-[:BACKED_BY]->(range:7E:mem.Range)
(module:7F:boot.Module)-[:BACKED_BY]->(bytespace:80:Bytespace)
(bytespace:80:Bytespace)-[:BACKED_BY]->(range:81:mem.Range)
(module:82:boot.Module)-[:BACKED_BY]->(bytespace:83:Bytespace)
(bytespace:83:Bytespace)-[:BACKED_BY]->(range:84:mem.Range)
(module:85:boot.Module)-[:BACKED_BY]->(bytespace:86:Bytespace)
(bytespace:86:Bytespace)-[:BACKED_BY]->(range:87:mem.Range)
(module:88:boot.Module)-[:BACKED_BY]->(bytespace:89:Bytespace)
(bytespace:89:Bytespace)-[:BACKED_BY]->(range:8A:mem.Range)
(module:8B:boot.Module)-[:BACKED_BY]->(bytespace:8C:Bytespace)
(bytespace:8C:Bytespace)-[:BACKED_BY]->(range:8D:mem.Range)
(module:8E:boot.Module)-[:BACKED_BY]->(bytespace:8F:Bytespace)
(bytespace:8F:Bytespace)-[:BACKED_BY]->(range:90:mem.Range)
(module:91:boot.Module)-[:BACKED_BY]->(bytespace:92:Bytespace)
(bytespace:92:Bytespace)-[:BACKED_BY]->(range:93:mem.Range)
(module:94:boot.Module)-[:BACKED_BY]->(bytespace:95:Bytespace)
(bytespace:95:Bytespace)-[:BACKED_BY]->(range:96:mem.Range)
(module:97:boot.Module)-[:BACKED_BY]->(bytespace:98:Bytespace)
(bytespace:98:Bytespace)-[:BACKED_BY]->(range:99:mem.Range)
(module:9A:boot.Module)-[:BACKED_BY]->(bytespace:9B:Bytespace)
(bytespace:9B:Bytespace)-[:BACKED_BY]->(range:9C:mem.Range)
(module:9D:boot.Module)-[:BACKED_BY]->(bytespace:9E:Bytespace)
(bytespace:9E:Bytespace)-[:BACKED_BY]->(range:9F:mem.Range)
(module:A0:boot.Module)-[:BACKED_BY]->(bytespace:A1:Bytespace)
(bytespace:A1:Bytespace)-[:BACKED_BY]->(range:A2:mem.Range)
(module:A3:boot.Module)-[:BACKED_BY]->(bytespace:A4:Bytespace)
(bytespace:A4:Bytespace)-[:BACKED_BY]->(range:A5:mem.Range)
(module:A6:boot.Module)-[:BACKED_BY]->(bytespace:A7:Bytespace)
(bytespace:A7:Bytespace)-[:BACKED_BY]->(range:A8:mem.Range)
(boot:AA:fw.Boot)-[:PROVIDES_TABLE]->(acpi:AB:fw.table.Acpi)
(acpi:AB:fw.table.Acpi)-[:BACKED_BY]->(bytespace:AC:Bytespace)
(bytespace:AC:Bytespace)-[:BACKED_BY]->(range:AD:mem.Range)
(pci:B1:dev.bus.Pci)-[:HAS_DEVICE]->(function:B2:dev.pci.Function)
(pci:B1:dev.bus.Pci)-[:HAS_DEVICE]->(function:B4:dev.pci.Function)
(pci:B1:dev.bus.Pci)-[:HAS_DEVICE]->(function:B6:dev.pci.Function)
(pci:B1:dev.bus.Pci)-[:HAS_DEVICE]->(function:B8:dev.pci.Function)
(pci:B1:dev.bus.Pci)-[:HAS_DEVICE]->(function:C2:dev.pci.Function)
(pci:B1:dev.bus.Pci)-[:HAS_DEVICE]->(function:C6:dev.pci.Function)
(function:B8:dev.pci.Function)-[:IMPLEMENTS]->(lpc:BB:dev.bridge.Lpc)
(lpc:BB:dev.bridge.Lpc)-[:HAS_BUS]->(legacyio:BC:dev.bus.LegacyIo)
(legacyio:BC:dev.bus.LegacyIo)-[:HAS_DEVICE]->(cmos:BD:dev.rtc.Cmos)
(legacyio:BC:dev.bus.LegacyIo)-[:HAS_DEVICE]->(ps2controller:BF:dev.input.Ps2Controller)
(cmos:BD:dev.rtc.Cmos)-[:USES_IOPORTS]->(range:BE:cap.ioport.Range)
(ps2controller:BF:dev.input.Ps2Controller)-[:USES_IOPORTS]->(range:C0:cap.ioport.Range)
[30460534270] [INFO] [logging] END rootdump
[30463642620] [INFO] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[30468561101] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[30471738624] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[30473974372] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[30485234934] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[30506857478] [INFO] [kernel::task::loader] Segment: vaddr=20d2a0 exec=false
[30508068437] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[30511601118] [INFO] [kernel::task::loader] Segment: vaddr=210850 exec=false
[30512254528] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[30529575168] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[30530402692] [INFO] [kernel] Spawning init process...
[30532786278] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[30558261711] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (61496400 ticks/sec), init_cnt=614964 for 100Hz
[30560362584] [INFO] [kernel] Entering scheduler loop.
[30562097488] [DEBUG] [sched.switch] Context switch from_tid=0 to_tid=2 from_user=0 to_user=0 cr3_before=50319360 cr3_after=2024677376
[30573006714] [DEBUG] [sched.switch] Context switch from_tid=2 to_tid=3 from_user=0 to_user=1 cr3_before=2024677376 cr3_after=50319360
[30574476295] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb01113f0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[30581037793] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562165469 RSP_BEFORE=18446744072368492064 RFLAGS_BEFORE=130 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563827328
[30602502947] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x2002ef rflags=0x206
[30604280135] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[30605271331] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[30606360211] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[30610022949] [DEBUG] [sched.switch] Context switch from_tid=3 to_tid=2 from_user=1 to_user=0 cr3_before=50319360 cr3_after=2024677376
[30621025044] [DEBUG] [sched.switch] Context switch from_tid=0 to_tid=3 from_user=0 to_user=1 cr3_before=2024677376 cr3_after=50319360
[30665301047] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[30667084296] [DEBUG] [sched.switch] Context switch from_tid=3 to_tid=2 from_user=1 to_user=0 cr3_before=50319360 cr3_after=2024677376
[30672558279] [DEBUG] [sched.switch] Context switch from_tid=0 to_tid=3 from_user=0 to_user=1 cr3_before=2024677376 cr3_after=50319360
[30674145379] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[30675936081] [DEBUG] [sched.switch] Context switch from_tid=3 to_tid=2 from_user=1 to_user=0 cr3_before=50319360 cr3_after=2024677376
[30679336736] [DEBUG] [sched.switch] Context switch from_tid=0 to_tid=3 from_user=0 to_user=1 cr3_before=2024677376 cr3_after=50319360
[30680862323] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[30681772028] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[30686098181] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[30688996166] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[30689957365] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[30694468885] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[30695318020] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[30696028803] [INFO] [sprout::devtree] SPROUT: build() called
[30696940952] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[30701450767] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[30702230717] [INFO] [sprout] SPROUT: About to create Supervisor...
[30703313651] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[30704345100] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[30705001455] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[30710044898] [INFO] [sprout::supervisor] SPROUT: Found 32 modules
[30781772867] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[30787958919] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[30793202847] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[30797061737] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[30799595030] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[30804581058] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/ps2_kbd'
[30809932546] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/echo'
[30814852892] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/bloom'
[30819631223] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/ps2_mouse'
[30825619605] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/root_batch_bench'
[30892588666] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_watch_tester'
[30896683473] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/ingestd'
[30900509285] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/bindd'
[30905335039] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/png_creator'
[30909063354] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/scheduler_fairness'
[30913411115] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/hogger'
[30917256044] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/tick_printer'
[30922581134] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/assets/wallpapers/clouds.bmp'
[30927321855] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/wallpapers/leather.bmp'
[30931671809] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/pci/pci.ids'
[30936838822] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Move.cur'
[30944043057] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Normal.cur'
[30948432486] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Unavailabe.cur'
[30952400556] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Working.ani'
[30956421124] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[30960716615] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Diagonal1.ani'
[30964351918] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Help.cur'
[30968814755] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Vertical.ani'
[30972618250] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/readme.txt'
[30977512727] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/plain.crs'
[30982246810] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Busy.cur'
[30986342917] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Alternate.cur'
[30991988811] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Text.cur'
[30996236222] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[31012610355] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[31119804422] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[31120841917] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[31123176116] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/echo'
[31136409521] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[31137259104] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/png_creator'
[31138133754] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/bindd'
[31139634160] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[31144053241] [INFO] [kernel::task::loader] Loading module: /boot/clock
[31147582821] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[31148893168] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[31163916673] [INFO] [kernel::task::loader] Segment: vaddr=2031d0 exec=false
[31164960367] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[31166732723] [INFO] [kernel::task::loader] Segment: vaddr=203f80 exec=false
[31167371038] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[31203997788] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[31217182168] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/echo'
[31222021219] [INFO] [kernel::task::loader] Loading module: /boot/echo
[31223563575] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[31224521209] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[31237969835] [INFO] [kernel::task::loader] Segment: vaddr=202250 exec=false
[31238630594] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[31239773728] [INFO] [kernel::task::loader] Segment: vaddr=202fb0 exec=false
[31240377935] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[31269183167] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[31270655050] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[31272649092] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[31282824800] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[31283928792] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[31288454769] [INFO] [kernel::task::loader] Segment: vaddr=2028e8 exec=false
[31289245229] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[31301102289] [INFO] [kernel::task::loader] Segment: vaddr=2046e3 exec=false
[31301763207] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[31327237557] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[31330816545] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/png_creator'
[31332697907] [INFO] [kernel::task::loader] Loading module: /boot/png_creator
[31333314819] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[31334209536] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[31336531915] [INFO] [kernel::task::loader] Segment: vaddr=2001f6 exec=false
[31337079901] [INFO] [kernel::task::loader]   Overlap at 200000: merging perms to r=true w=false x=true
[31347376107] [INFO] [kernel::task::loader] Segment: vaddr=2002d7 exec=false
[31347988468] [INFO] [kernel::task::loader]   Overlap at 200000: merging perms to r=true w=true x=true
[31367606495] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[31368914303] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/bindd'
[31370262402] [INFO] [kernel::task::loader] Loading module: /boot/bindd
[31370841715] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[31371793581] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[31388326574] [INFO] [kernel::task::loader] Segment: vaddr=202058 exec=false
[31389045164] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[31389938789] [INFO] [kernel::task::loader] Segment: vaddr=202608 exec=false
[31390518959] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[31411781311] [INFO] [sprout::supervisor] SPROUT: App launched (PID=8)
[31412625401] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[31444804646] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb01299a0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[31446159473] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562165469 RSP_BEFORE=18446744072368591824 RFLAGS_BEFORE=130 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563827328
[31452178114] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x2000cf rflags=0x202
[31453582253] [INFO] [clock] starting clock publisher
[31467375042] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb012f710
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[31468631965] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562165469 RSP_BEFORE=18446744072368615744 RFLAGS_BEFORE=130 CR3_BEFORE=50847744 fs_base=0 gs_base=18446744071563827328
[31475606768] [INFO] [echo] echo: online (handle=0)
[31476852598] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[31484340065] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0133e80
USER_TRAMPOLINE: PC=0x2004e0 SP=0x800000 ARG0=0x0
[31486128672] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2098400 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562165469 RSP_BEFORE=18446744072368634032 RFLAGS_BEFORE=130 CR3_BEFORE=50946048 fs_base=0 gs_base=18446744071563827328
[31488640931] [ERROR] [INGESTD] Starting...
[31490145008] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fdf90
[31492634142] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[31494844112] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[31509900088] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb013b200
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[31511365000] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562165469 RSP_BEFORE=18446744072368663600 RFLAGS_BEFORE=134 CR3_BEFORE=51052544 fs_base=0 gs_base=18446744071563827328
[31513517340] [ERROR] [PNG_CREATOR] Starting...
P[31515818661] [ERROR] [bran] KERNEL PANIC Location: bran/src/arch/x86_64/idt.rs:288:5 Message: PAGE FAULT at 0x10 RIP=0xffffffff80000418 CS=0x8 ERR=0x0

```
</details>
