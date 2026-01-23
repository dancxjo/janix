# ✅ Scenario: Ingestd detects PNG content

> Last run: 2026-01-23 19:28:15

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10132ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "SPROUT: Launching app '/boot/ingestd'" | ✅ | 1246ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "SPROUT: Launching app '/boot/png_creator'" | ✅ | 1101ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And I wait for 5 seconds | ⏭️ | 220ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[14538888479] [INFO] [kernel] thing-os kernel starting...
[14598194293] [INFO] [kernel::memory] Memory map has 64 entries
[14599819942] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[14600719024] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[14601244501] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[14601704720] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[14602027652] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[14602339275] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[14602654675] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[14602967339] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[14603391098] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78b3f000 (Usable)
[14603729579] [INFO] [kernel::memory]   [9] 0x78b3f000 - 0x78b9e000 (Reserved)
[14604060965] [INFO] [kernel::memory]   [10] 0x78b9e000 - 0x78c29000 (Other)
[14604384955] [INFO] [kernel::memory]   [11] 0x78c29000 - 0x78c2a000 (Reserved)
[14604719513] [INFO] [kernel::memory]   [12] 0x78c2a000 - 0x78c30000 (Other)
[14605042905] [INFO] [kernel::memory]   [13] 0x78c30000 - 0x78c31000 (Reserved)
[14605378134] [INFO] [kernel::memory]   [14] 0x78c31000 - 0x78cc8000 (Other)
[14605771092] [INFO] [kernel::memory]   [15] 0x78cc8000 - 0x78cc9000 (Reserved)
[14606124220] [INFO] [kernel::memory]   [16] 0x78cc9000 - 0x78d09000 (Other)
[14606447813] [INFO] [kernel::memory]   [17] 0x78d09000 - 0x78d0a000 (Reserved)
[14606781911] [INFO] [kernel::memory]   [18] 0x78d0a000 - 0x78d56000 (Other)
[14616480687] [INFO] [kernel::memory]   [19] 0x78d56000 - 0x78d57000 (Reserved)
[14616837969] [INFO] [kernel::memory]   [20] 0x78d57000 - 0x78df8000 (Other)
[14617328595] [INFO] [kernel::memory]   [21] 0x78df8000 - 0x78df9000 (Reserved)
[14617706345] [INFO] [kernel::memory]   [22] 0x78df9000 - 0x78dfb000 (Other)
[14618037441] [INFO] [kernel::memory]   [23] 0x78dfb000 - 0x78dfc000 (Reserved)
[14618383996] [INFO] [kernel::memory]   [24] 0x78dfc000 - 0x78dfe000 (Other)
[14618715385] [INFO] [kernel::memory]   [25] 0x78dfe000 - 0x78dff000 (Reserved)
[14619055540] [INFO] [kernel::memory]   [26] 0x78dff000 - 0x78e09000 (Other)
[14619472644] [INFO] [kernel::memory]   [27] 0x78e09000 - 0x78e0a000 (Reserved)
[14619834556] [INFO] [kernel::memory]   [28] 0x78e0a000 - 0x78e0e000 (Other)
[14620178309] [INFO] [kernel::memory]   [29] 0x78e0e000 - 0x78e0f000 (Reserved)
[14620521268] [INFO] [kernel::memory]   [30] 0x78e0f000 - 0x78e11000 (Other)
[14620854175] [INFO] [kernel::memory]   [31] 0x78e11000 - 0x78e12000 (Reserved)
[14621196427] [INFO] [kernel::memory]   [32] 0x78e12000 - 0x78e14000 (Other)
[14621531234] [INFO] [kernel::memory]   [33] 0x78e14000 - 0x78e15000 (Reserved)
[14621944270] [INFO] [kernel::memory]   [34] 0x78e15000 - 0x78e17000 (Other)
[14622290858] [INFO] [kernel::memory]   [35] 0x78e17000 - 0x78e18000 (Reserved)
[14622634118] [INFO] [kernel::memory]   [36] 0x78e18000 - 0x78e19000 (Other)
[14622972252] [INFO] [kernel::memory]   [37] 0x78e19000 - 0x78e1a000 (Reserved)
[14623311998] [INFO] [kernel::memory]   [38] 0x78e1a000 - 0x78e1b000 (Other)
[14623642836] [INFO] [kernel::memory]   [39] 0x78e1b000 - 0x78e1c000 (Reserved)
[14624047456] [INFO] [kernel::memory]   [40] 0x78e1c000 - 0x78e20000 (Other)
[14624395958] [INFO] [kernel::memory]   [41] 0x78e20000 - 0x78e21000 (Reserved)
[14624740753] [INFO] [kernel::memory]   [42] 0x78e21000 - 0x78e23000 (Other)
[14625069643] [INFO] [kernel::memory]   [43] 0x78e23000 - 0x78e24000 (Reserved)
[14634826276] [INFO] [kernel::memory]   [44] 0x78e24000 - 0x78e28000 (Other)
[14635175884] [INFO] [kernel::memory]   [45] 0x78e28000 - 0x78e29000 (Reserved)
[14635601240] [INFO] [kernel::memory]   [46] 0x78e29000 - 0x78e2d000 (Other)
[14635958889] [INFO] [kernel::memory]   [47] 0x78e2d000 - 0x78e2e000 (Reserved)
[14636308043] [INFO] [kernel::memory]   [48] 0x78e2e000 - 0x78e37000 (Other)
[14636644034] [INFO] [kernel::memory]   [49] 0x78e37000 - 0x78e38000 (Reserved)
[14636993875] [INFO] [kernel::memory]   [50] 0x78e38000 - 0x78e3a000 (Other)
[14637335795] [INFO] [kernel::memory]   [51] 0x78e3a000 - 0x78e3b000 (Reserved)
[14637747031] [INFO] [kernel::memory]   [52] 0x78e3b000 - 0x78e3d000 (Other)
[14638096440] [INFO] [kernel::memory]   [53] 0x78e3d000 - 0x78e3e000 (Reserved)
[14638439884] [INFO] [kernel::memory]   [54] 0x78e3e000 - 0x78e40000 (Other)
[14638769633] [INFO] [kernel::memory]   [55] 0x78e40000 - 0x78e41000 (Reserved)
[14639113958] [INFO] [kernel::memory]   [56] 0x78e41000 - 0x78fc4000 (Other)
[14639444550] [INFO] [kernel::memory]   [57] 0x78fc4000 - 0x78fc5000 (Reserved)
[14639789305] [INFO] [kernel::memory]   [58] 0x78fc5000 - 0x79446000 (Other)
[14640187914] [INFO] [kernel::memory]   [59] 0x79446000 - 0x79447000 (Reserved)
[14640559460] [INFO] [kernel::memory]   [60] 0x79447000 - 0x79748000 (Other)
[14640900178] [INFO] [kernel::memory]   [61] 0x79748000 - 0x79749000 (Reserved)
[14641254713] [INFO] [kernel::memory]   [62] 0x79749000 - 0x7979c000 (Other)
[14641591691] [INFO] [kernel::memory]   [63] 0x7979c000 - 0x79951000 (Other)
[14642394702] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[15040626320] [INFO] [kernel::memory] Frame allocator initialized with 490327 free frames
[15052361812] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[15059067132] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[15060485218] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[15061408521] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[15070535886] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[15071629529] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[15077259780] [INFO] [bran::arch] IOAPIC: Registers initialized
[15078715412] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[15080711798] [INFO] [bran::arch] IOAPIC: All pins masked
[15082521305] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[15083346819] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[15084087148] [INFO] [bran::arch] IOAPIC: Init complete
[15084624523] [INFO] [kernel] Initializing global allocator...
[17662895287] [INFO] [kernel::memory::arena] ArenaHeap: Adding arena 'global.pinned' base=ffffffffb0000000 size=33554432 flags=ArenaFlags(PINNED)
[17677976239] [INFO] [kernel::memory::global_alloc] Arena allocator initialized (pinned pre-expanded)
[17679002741] [INFO] [kernel] Initializing SIMD...
[17681961062] [INFO] [kernel] Initializing tasking...
[17691059242] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[17691906902] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[17692648252] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[17698911408] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[17699469059] [INFO] [kernel::task::scheduler]   Initializing boot task...
[17700816170] [INFO] [kernel::task::scheduler]   Creating boot task...
[17713061808] [INFO] [kernel::task::scheduler]   Creating idle task...
[17722644682] [INFO] [kernel::task::scheduler]   Boot task initialized
[17723174896] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[17724652218] [INFO] [kernel::task::scheduler]   Scheduler initialized
[17736405555] [INFO] [kernel::root] Spawning Root service...
[17947909139] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[17960535284] [INFO] [kernel::root::service] ROOT: started once
[19359440027] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[19360470332] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[19412870004] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[19435850748] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[19495092116] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[19551576361] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[19553618182] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[19594367244] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[19621351616] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[19631159992] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[19634353125] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=194, idx=3) BAR5=0x810c4000
[19659317169] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[19662542274] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[19665806351] [INFO] [logging] BEGIN rootdump
[19666615795] [INFO] [kernel::root::handlers::debug] ROOT DUMP NODES count=200
(entry:1:log.Entry { level: 3, line: 200, timestamp: 0x4215555fb, message: 11, event: 10, file: 12, module: 10, tid: 0 })
(entry:2:log.Entry { level: 3, line: 28, timestamp: 0x42dd1458a, message: 14, event: 13, file: 15, module: 13, tid: 0 })
(host:3:dev.Host { hhdm_offset: 0xffff800000000000, arch: "x86_64", platform_profile: "unknown", source: 0, confidence: 2 })
(entry:4:log.Entry { level: 3, line: 13, timestamp: 0x42e912d17, message: 18, event: 17, file: 19, module: 17, tid: 2 })
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
(range:11:mem.Range { source: 0, confidence: 2, start: 0x1780000, end: 0x78b3f000, kind: 0 })
(range:12:mem.Range { source: 0, confidence: 2, start: 0x78b3f000, end: 0x78b9e000, kind: 1 })
(range:13:mem.Range { source: 0, confidence: 2, start: 0x78b9e000, end: 0x78c29000, kind: 8 })
(range:14:mem.Range { source: 0, confidence: 2, start: 0x78c29000, end: 0x78c2a000, kind: 1 })
(range:15:mem.Range { source: 0, confidence: 2, start: 0x78c2a000, end: 0x78c30000, kind: 8 })
(range:16:mem.Range { source: 0, confidence: 2, start: 0x78c30000, end: 0x78c31000, kind: 1 })
(range:17:mem.Range { source: 0, confidence: 2, start: 0x78c31000, end: 0x78cc8000, kind: 8 })
(range:18:mem.Range { source: 0, confidence: 2, start: 0x78cc8000, end: 0x78cc9000, kind: 1 })
(range:19:mem.Range { source: 0, confidence: 2, start: 0x78cc9000, end: 0x78d09000, kind: 8 })
(range:1A:mem.Range { source: 0, confidence: 2, start: 0x78d09000, end: 0x78d0a000, kind: 1 })
(range:1B:mem.Range { source: 0, confidence: 2, start: 0x78d0a000, end: 0x78d56000, kind: 8 })
(range:1C:mem.Range { source: 0, confidence: 2, start: 0x78d56000, end: 0x78d57000, kind: 1 })
(range:1D:mem.Range { source: 0, confidence: 2, start: 0x78d57000, end: 0x78df8000, kind: 8 })
(range:1E:mem.Range { source: 0, confidence: 2, start: 0x78df8000, end: 0x78df9000, kind: 1 })
(range:1F:mem.Range { source: 0, confidence: 2, start: 0x78df9000, end: 0x78dfb000, kind: 8 })
(range:20:mem.Range { source: 0, confidence: 2, start: 0x78dfb000, end: 0x78dfc000, kind: 1 })
(range:21:mem.Range { source: 0, confidence: 2, start: 0x78dfc000, end: 0x78dfe000, kind: 8 })
(range:22:mem.Range { source: 0, confidence: 2, start: 0x78dfe000, end: 0x78dff000, kind: 1 })
(range:23:mem.Range { source: 0, confidence: 2, start: 0x78dff000, end: 0x78e09000, kind: 8 })
(range:24:mem.Range { source: 0, confidence: 2, start: 0x78e09000, end: 0x78e0a000, kind: 1 })
(range:25:mem.Range { source: 0, confidence: 2, start: 0x78e0a000, end: 0x78e0e000, kind: 8 })
(range:26:mem.Range { source: 0, confidence: 2, start: 0x78e0e000, end: 0x78e0f000, kind: 1 })
(range:27:mem.Range { source: 0, confidence: 2, start: 0x78e0f000, end: 0x78e11000, kind: 8 })
(range:28:mem.Range { source: 0, confidence: 2, start: 0x78e11000, end: 0x78e12000, kind: 1 })
(range:29:mem.Range { source: 0, confidence: 2, start: 0x78e12000, end: 0x78e14000, kind: 8 })
(range:2A:mem.Range { source: 0, confidence: 2, start: 0x78e14000, end: 0x78e15000, kind: 1 })
(range:2B:mem.Range { source: 0, confidence: 2, start: 0x78e15000, end: 0x78e17000, kind: 8 })
(range:2C:mem.Range { source: 0, confidence: 2, start: 0x78e17000, end: 0x78e18000, kind: 1 })
(range:2D:mem.Range { source: 0, confidence: 2, start: 0x78e18000, end: 0x78e19000, kind: 8 })
(range:2E:mem.Range { source: 0, confidence: 2, start: 0x78e19000, end: 0x78e1a000, kind: 1 })
(range:2F:mem.Range { source: 0, confidence: 2, start: 0x78e1a000, end: 0x78e1b000, kind: 8 })
(range:30:mem.Range { source: 0, confidence: 2, start: 0x78e1b000, end: 0x78e1c000, kind: 1 })
(range:31:mem.Range { source: 0, confidence: 2, start: 0x78e1c000, end: 0x78e20000, kind: 8 })
(range:32:mem.Range { source: 0, confidence: 2, start: 0x78e20000, end: 0x78e21000, kind: 1 })
(range:33:mem.Range { source: 0, confidence: 2, start: 0x78e21000, end: 0x78e23000, kind: 8 })
(range:34:mem.Range { source: 0, confidence: 2, start: 0x78e23000, end: 0x78e24000, kind: 1 })
(range:35:mem.Range { source: 0, confidence: 2, start: 0x78e24000, end: 0x78e28000, kind: 8 })
(range:36:mem.Range { source: 0, confidence: 2, start: 0x78e28000, end: 0x78e29000, kind: 1 })
(range:37:mem.Range { source: 0, confidence: 2, start: 0x78e29000, end: 0x78e2d000, kind: 8 })
(range:38:mem.Range { source: 0, confidence: 2, start: 0x78e2d000, end: 0x78e2e000, kind: 1 })
(range:39:mem.Range { source: 0, confidence: 2, start: 0x78e2e000, end: 0x78e37000, kind: 8 })
(range:3A:mem.Range { source: 0, confidence: 2, start: 0x78e37000, end: 0x78e38000, kind: 1 })
(range:3B:mem.Range { source: 0, confidence: 2, start: 0x78e38000, end: 0x78e3a000, kind: 8 })
(range:3C:mem.Range { source: 0, confidence: 2, start: 0x78e3a000, end: 0x78e3b000, kind: 1 })
(range:3D:mem.Range { source: 0, confidence: 2, start: 0x78e3b000, end: 0x78e3d000, kind: 8 })
(range:3E:mem.Range { source: 0, confidence: 2, start: 0x78e3d000, end: 0x78e3e000, kind: 1 })
(range:3F:mem.Range { source: 0, confidence: 2, start: 0x78e3e000, end: 0x78e40000, kind: 8 })
(range:40:mem.Range { source: 0, confidence: 2, start: 0x78e40000, end: 0x78e41000, kind: 1 })
(range:41:mem.Range { source: 0, confidence: 2, start: 0x78e41000, end: 0x78fc4000, kind: 8 })
(range:42:mem.Range { source: 0, confidence: 2, start: 0x78fc4000, end: 0x78fc5000, kind: 1 })
(range:43:mem.Range { source: 0, confidence: 2, start: 0x78fc5000, end: 0x79446000, kind: 8 })
(range:44:mem.Range { source: 0, confidence: 2, start: 0x79446000, end: 0x79447000, kind: 1 })
(range:45:mem.Range { source: 0, confidence: 2, start: 0x79447000, end: 0x79748000, kind: 8 })
(range:46:mem.Range { source: 0, confidence: 2, start: 0x79748000, end: 0x79749000, kind: 1 })
(range:47:mem.Range { source: 0, confidence: 2, start: 0x79749000, end: 0x7979c000, kind: 8 })
(range:48:mem.Range { source: 0, confidence: 2, start: 0x7979c000, end: 0x79951000, kind: 8 })
(module:49:boot.Module { source: 0, confidence: 2, name: "/boot/sprout", phys_base: 0x7bbd8000, size_bytes: 0x113c8, index: 0, bytespace: 74 })
(bytespace:4A:Bytespace {  })
(range:4B:mem.Range { phys_base: 0x7bbd8000, size_bytes: 0x113c8 })
(module:4C:boot.Module { source: 0, confidence: 2, name: "/boot/bristle", phys_base: 0x7e24f000, size_bytes: 16640, index: 1, bytespace: 77 })
(bytespace:4D:Bytespace {  })
(range:4E:mem.Range { phys_base: 0x7e24f000, size_bytes: 16640 })
(module:4F:boot.Module { source: 0, confidence: 2, name: "/boot/rtc_cmos", phys_base: 0x7bbd2000, size_bytes: 19904, index: 2, bytespace: 80 })
(bytespace:50:Bytespace {  })
(range:51:mem.Range { phys_base: 0x7bbd2000, size_bytes: 19904 })
(module:52:boot.Module { source: 0, confidence: 2, name: "/boot/clock", phys_base: 0x7bbcb000, size_bytes: 21048, index: 3, bytespace: 83 })
(bytespace:53:Bytespace {  })
(range:54:mem.Range { phys_base: 0x7bbcb000, size_bytes: 21048 })
(module:55:boot.Module { source: 0, confidence: 2, name: "/boot/ps2_kbd", phys_base: 0x7bbc6000, size_bytes: 12616, index: 4, bytespace: 86 })
(bytespace:56:Bytespace {  })
(range:57:mem.Range { phys_base: 0x7bbc6000, size_bytes: 12616 })
(module:58:boot.Module { source: 0, confidence: 2, name: "/boot/echo", phys_base: 0x7bbc0000, size_bytes: 16808, index: 5, bytespace: 89 })
(bytespace:59:Bytespace {  })
(range:5A:mem.Range { phys_base: 0x7bbc0000, size_bytes: 16808 })
(module:5B:boot.Module { source: 0, confidence: 2, name: "/boot/bloom", phys_base: 0x79749000, size_bytes: 0x52958, index: 6, bytespace: 92 })
(bytespace:5C:Bytespace {  })
(range:5D:mem.Range { phys_base: 0x79749000, size_bytes: 0x52958 })
(module:5E:boot.Module { source: 0, confidence: 2, name: "/boot/ps2_mouse", phys_base: 0x7bbb9000, size_bytes: 17624, index: 7, bytespace: 95 })
(bytespace:5F:Bytespace {  })
(range:60:mem.Range { phys_base: 0x7bbb9000, size_bytes: 17624 })
(module:61:boot.Module { source: 0, confidence: 2, name: "/boot/root_batch_bench", phys_base: 0x7bbb3000, size_bytes: 16592, index: 8, bytespace: 98 })
(bytespace:62:Bytespace {  })
(range:63:mem.Range { phys_base: 0x7bbb3000, size_bytes: 16592 })
(module:64:boot.Module { source: 0, confidence: 2, name: "/boot/root_watch_tester", phys_base: 0x7bbab000, size_bytes: 25560, index: 9, bytespace: 101 })
(bytespace:65:Bytespace {  })
(range:66:mem.Range { phys_base: 0x7bbab000, size_bytes: 25560 })
(module:67:boot.Module { source: 0, confidence: 2, name: "/boot/ingestd", phys_base: 0x7bba3000, size_bytes: 28488, index: 10, bytespace: 104 })
(bytespace:68:Bytespace {  })
(range:69:mem.Range { phys_base: 0x7bba3000, size_bytes: 28488 })
(module:6A:boot.Module { source: 0, confidence: 2, name: "/boot/png_creator", phys_base: 0x7bb9e000, size_bytes: 13120, index: 11, bytespace: 107 })
(bytespace:6B:Bytespace {  })
(range:6C:mem.Range { phys_base: 0x7bb9e000, size_bytes: 13120 })
(module:6D:boot.Module { source: 0, confidence: 2, name: "/boot/bindd", phys_base: 0x7bb99000, size_bytes: 14392, index: 12, bytespace: 110 })
(bytespace:6E:Bytespace {  })
(range:6F:mem.Range { phys_base: 0x7bb99000, size_bytes: 14392 })
(module:70:boot.Module { source: 0, confidence: 2, name: "/boot/scheduler_fairness", phys_base: 0x7bb94000, size_bytes: 13368, index: 13, bytespace: 113 })
(bytespace:71:Bytespace {  })
(range:72:mem.Range { phys_base: 0x7bb94000, size_bytes: 13368 })
(module:73:boot.Module { source: 0, confidence: 2, name: "/boot/hogger", phys_base: 0x7bb90000, size_bytes: 10000, index: 14, bytespace: 116 })
(bytespace:74:Bytespace {  })
(range:75:mem.Range { phys_base: 0x7bb90000, size_bytes: 10000 })
(module:76:boot.Module { source: 0, confidence: 2, name: "/boot/tick_printer", phys_base: 0x7bb8c000, size_bytes: 10712, index: 15, bytespace: 119 })
(bytespace:77:Bytespace {  })
(range:78:mem.Range { phys_base: 0x7bb8c000, size_bytes: 10712 })
(module:79:boot.Module { source: 0, confidence: 2, name: "/assets/wallpapers/clouds.bmp", phys_base: 0x79447000, size_bytes: 0x300036, index: 16, bytespace: 122 })
(bytespace:7A:Bytespace {  })
(range:7B:mem.Range { phys_base: 0x79447000, size_bytes: 0x300036 })
(module:7C:boot.Module { source: 0, confidence: 2, name: "/assets/wallpapers/leather.bmp", phys_base: 0x78fc5000, size_bytes: 0x480036, index: 17, bytespace: 125 })
(bytespace:7D:Bytespace {  })
(range:7E:mem.Range { phys_base: 0x78fc5000, size_bytes: 0x480036 })
(module:7F:boot.Module { source: 0, confidence: 2, name: "/assets/pci/pci.ids", phys_base: 0x78e41000, size_bytes: 0x182396, index: 18, bytespace: 128 })
(bytespace:80:Bytespace {  })
(range:81:mem.Range { phys_base: 0x78e41000, size_bytes: 0x182396 })
(module:82:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Move.cur", phys_base: 0x78e3e000, size_bytes: 4286, index: 19, bytespace: 131 })
(bytespace:83:Bytespace {  })
(range:84:mem.Range { phys_base: 0x78e3e000, size_bytes: 4286 })
(module:85:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Normal.cur", phys_base: 0x78e3b000, size_bytes: 4286, index: 20, bytespace: 134 })
(bytespace:86:Bytespace {  })
(range:87:mem.Range { phys_base: 0x78e3b000, size_bytes: 4286 })
(module:88:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Unavailabe.cur", phys_base: 0x78e38000, size_bytes: 4286, index: 21, bytespace: 137 })
(bytespace:89:Bytespace {  })
(range:8A:mem.Range { phys_base: 0x78e38000, size_bytes: 4286 })
(module:8B:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Working.ani", phys_base: 0x78e2e000, size_bytes: 34470, index: 22, bytespace: 140 })
(bytespace:8C:Bytespace {  })
(range:8D:mem.Range { phys_base: 0x78e2e000, size_bytes: 34470 })
(module:8E:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Horizontal.ani", phys_base: 0x78e29000, size_bytes: 13030, index: 23, bytespace: 143 })
(bytespace:8F:Bytespace {  })
(range:90:mem.Range { phys_base: 0x78e29000, size_bytes: 13030 })
(module:91:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Diagonal1.ani", phys_base: 0x78e24000, size_bytes: 13030, index: 24, bytespace: 146 })
(bytespace:92:Bytespace {  })
(range:93:mem.Range { phys_base: 0x78e24000, size_bytes: 13030 })
(module:94:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Help.cur", phys_base: 0x78e21000, size_bytes: 4286, index: 25, bytespace: 149 })
(bytespace:95:Bytespace {  })
(range:96:mem.Range { phys_base: 0x78e21000, size_bytes: 4286 })
(module:97:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Vertical.ani", phys_base: 0x78e1c000, size_bytes: 13028, index: 26, bytespace: 152 })
(bytespace:98:Bytespace {  })
(range:99:mem.Range { phys_base: 0x78e1c000, size_bytes: 13028 })
(module:9A:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/readme.txt", phys_base: 0x78e1a000, size_bytes: 308, index: 27, bytespace: 155 })
(bytespace:9B:Bytespace {  })
(range:9C:mem.Range { phys_base: 0x78e1a000, size_bytes: 308 })
(module:9D:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/plain.crs", phys_base: 0x78e18000, size_bytes: 457, index: 28, bytespace: 158 })
(bytespace:9E:Bytespace {  })
(range:9F:mem.Range { phys_base: 0x78e18000, size_bytes: 457 })
(module:A0:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Busy.cur", phys_base: 0x78e15000, size_bytes: 4286, index: 29, bytespace: 161 })
(bytespace:A1:Bytespace {  })
(range:A2:mem.Range { phys_base: 0x78e15000, size_bytes: 4286 })
(module:A3:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Alternate.cur", phys_base: 0x78e12000, size_bytes: 4286, index: 30, bytespace: 164 })
(bytespace:A4:Bytespace {  })
(range:A5:mem.Range { phys_base: 0x78e12000, size_bytes: 4286 })
(module:A6:boot.Module { source: 0, confidence: 2, name: "/assets/cursors/plain/Text.cur", phys_base: 0x78e0f000, size_bytes: 4286, index: 31, bytespace: 167 })
(bytespace:A7:Bytespace {  })
(range:A8:mem.Range { phys_base: 0x78e0f000, size_bytes: 4286 })
(framebuffer:A9:dev.display.Framebuffer { source: 0, confidence: 2, phys_base: 0x80000000, size_bytes: 0x384000, virt_base: 0xffff800080000000, width: 1280, height: 720, stride: 5120, ... })
(boot:AA:fw.Boot { source: 0, confidence: 2 })
(acpi:AB:fw.table.Acpi { source: 0, confidence: 2, phys_base: 0x7f77e014 })
(bytespace:AC:Bytespace {  })
(range:AD:mem.Range { phys_base: 0x2ffc000, size_bytes: 4096, page_count: 1 })
(scheduler:AE:svc.Scheduler {  })
(entry:AF:log.Entry { level: 3, line: 335, timestamp: 0x481f26bbe, message: 98, event: 13, file: 15, module: 13, tid: 0 })
(entry:B0:log.Entry { level: 3, line: 108, timestamp: 0x481ff8bb8, message: 100, event: 99, file: 101, module: 99, tid: 0 })
(pci:B1:dev.bus.Pci { source: 4, confidence: 2, name: "pci0", segment: 0 })
(function:B2:dev.pci.Function { source: 4, confidence: 2, name: "Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller", bus: 0, device: 0, function: 0, vendor_id: 32902, device_id: 10688, ... })
(entry:B3:log.Entry { level: 3, line: 261, timestamp: 0x4853b59db, message: 126, event: 99, file: 101, module: 99, tid: 0 })
(function:B4:dev.pci.Function { source: 4, confidence: 2, bus: 0, device: 1, function: 0, vendor_id: 4660, device_id: 4369, class_code: 3, ... })
(entry:B5:log.Entry { level: 3, line: 261, timestamp: 0x48691bec8, message: 128, event: 99, file: 101, module: 99, tid: 0 })
(function:B6:dev.pci.Function { source: 4, confidence: 2, name: "Intel Corporation 82574L Gigabit Network Connection", bus: 0, device: 2, function: 0, vendor_id: 32902, device_id: 4307, ... })
(entry:B7:log.Entry { level: 3, line: 261, timestamp: 0x48a0f9453, message: 134, event: 99, file: 101, module: 99, tid: 0 })
(function:B8:dev.pci.Function { source: 4, confidence: 2, name: "Intel Corporation 82801IB (ICH9) LPC Interface Controller", bus: 0, device: 31, function: 0, vendor_id: 32902, device_id: 10520, ... })
(entry:B9:log.Entry { level: 3, line: 261, timestamp: 0x48d71cec0, message: 142, event: 99, file: 101, module: 99, tid: 0 })
(entry:BA:log.Entry { level: 3, line: 364, timestamp: 0x48d8406fe, message: 143, event: 99, file: 101, module: 99, tid: 0 })
(lpc:BB:dev.bridge.Lpc { source: 4, confidence: 2, name: "lpc0" })
(legacyio:BC:dev.bus.LegacyIo { source: 4, confidence: 2, name: "isa0" })
(cmos:BD:dev.rtc.Cmos { source: 4, confidence: 2, name: "rtc0" })
(range:BE:cap.ioport.Range { port_start: 112, port_end: 113 })
(ps2controller:BF:dev.input.Ps2Controller { source: 4, confidence: 2, name: "i8042" })
(range:C0:cap.ioport.Range { port_start: 96, port_end: 100 })
(entry:C1:log.Entry { level: 3, line: 576, timestamp: 0x48ff4a51a, message: 157, event: 99, file: 101, module: 99, tid: 0 })
(function:C2:dev.pci.Function { source: 4, confidence: 2, name: "Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode]", bus: 0, device: 31, function: 2, vendor_id: 32902, device_id: 10530, ... })
(entry:C3:log.Entry { level: 3, line: 261, timestamp: 0x49198c39e, message: 161, event: 99, file: 101, module: 99, tid: 0 })
(entry:C4:log.Entry { level: 3, line: 375, timestamp: 0x492250b0f, message: 163, event: 99, file: 101, module: 99, tid: 0 })
(entry:C5:log.Entry { level: 3, line: 490, timestamp: 0x49257e4bf, message: 164, event: 99, file: 101, module: 99, tid: 0 })
(function:C6:dev.pci.Function { source: 4, confidence: 2, name: "Intel Corporation 82801I (ICH9 Family) SMBus Controller", bus: 0, device: 31, function: 3, vendor_id: 32902, device_id: 10544, ... })
(entry:C7:log.Entry { level: 3, line: 261, timestamp: 0x493d95cc4, message: 168, event: 99, file: 101, module: 99, tid: 0 })
(entry:C8:log.Entry { level: 3, line: 338, timestamp: 0x4940429d4, message: 169, event: 13, file: 15, module: 13, tid: 0 })
[19804910499] [INFO] [kernel::root::handlers::debug] ROOT DUMP EDGES
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
[19902807223] [INFO] [logging] END rootdump
[19912045881] [INFO] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[19916764785] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[19919754923] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[19920716877] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19930670145] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[20072844724] [INFO] [kernel::task::loader] Segment: vaddr=20d0d0 exec=false
[20074279501] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[20077424458] [INFO] [kernel::task::loader] Segment: vaddr=210168 exec=false
[20078035972] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[20088648511] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[20089813843] [INFO] [kernel] Spawning init process...
[20092078838] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[20348923584] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (78878500 ticks/sec), init_cnt=788785 for 100Hz
[20350960330] [INFO] [kernel] Entering scheduler loop.
[20352564156] [DEBUG] [sched.switch] Context switch from_tid=0 to_tid=2 from_user=0 to_user=0 cr3_before=50319360 cr3_after=2025365504
[20364636960] [DEBUG] [sched.switch] Context switch from_tid=2 to_tid=3 from_user=0 to_user=1 cr3_before=2025365504 cr3_after=50319360
[20366125856] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0110e40
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[20372543040] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562145933 RSP_BEFORE=18446744072368490608 RFLAGS_BEFORE=130 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563768496
[20394446389] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff980 rip=0x2004cf rflags=0x202
[20396389178] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[20397261647] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[20397989843] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[20402274883] [DEBUG] [sched.switch] Context switch from_tid=3 to_tid=2 from_user=1 to_user=0 cr3_before=50319360 cr3_after=2025365504
[20432658397] [DEBUG] [sched.switch] Context switch from_tid=0 to_tid=3 from_user=0 to_user=1 cr3_before=2025365504 cr3_after=50319360
[20436290167] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[20437784859] [DEBUG] [sched.switch] Context switch from_tid=3 to_tid=2 from_user=1 to_user=0 cr3_before=50319360 cr3_after=2025365504
[20442988948] [DEBUG] [sched.switch] Context switch from_tid=0 to_tid=3 from_user=0 to_user=1 cr3_before=2025365504 cr3_after=50319360
[20444442703] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[20445776040] [DEBUG] [sched.switch] Context switch from_tid=3 to_tid=2 from_user=1 to_user=0 cr3_before=50319360 cr3_after=2025365504
[20450082299] [DEBUG] [sched.switch] Context switch from_tid=0 to_tid=3 from_user=0 to_user=1 cr3_before=2025365504 cr3_after=50319360
[20451604878] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[20452438074] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[20456695514] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[20461489585] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[20462456960] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[20471671914] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[20472748276] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[20473495550] [INFO] [sprout::devtree] SPROUT: build() called
[20474188604] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[20488675651] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[20489652058] [INFO] [sprout] SPROUT: About to create Supervisor...
[20490274168] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[20491066069] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[20498133447] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[20517437541] [INFO] [sprout::supervisor] SPROUT: Found 32 modules
[20596208416] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[20602948551] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[20618502030] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[20622926420] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[20636179605] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[20661215065] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/ps2_kbd'
[20665779206] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/echo'
[20670424733] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/bloom'
[20680879662] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/ps2_mouse'
[20685245046] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/root_batch_bench'
[20690640642] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_watch_tester'
[20694548805] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/ingestd'
[20710899899] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/png_creator'
[20715295102] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/bindd'
[20719755647] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/scheduler_fairness'
[20723985475] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/hogger'
[20727924165] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/tick_printer'
[20731821205] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/assets/wallpapers/clouds.bmp'
[20736335860] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/wallpapers/leather.bmp'
[20740180092] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/pci/pci.ids'
[20757418033] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Move.cur'
[20762468465] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Normal.cur'
[20774837507] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Unavailabe.cur'
[20778958616] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Working.ani'
[20803921330] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[20808757736] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Diagonal1.ani'
[20820755047] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Help.cur'
[20825373945] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Vertical.ani'
[20835334978] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/readme.txt'
[20839484639] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/plain.crs'
[20856384116] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Busy.cur'
[20860758397] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Alternate.cur'
[20873576987] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Text.cur'
[20884990489] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[20914578802] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[21042393733] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[21043573779] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[21056919934] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/echo'
[21058998615] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[21059904191] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/png_creator'
[21060727821] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/bindd'
[21061924750] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[21081171664] [INFO] [kernel::task::loader] Loading module: /boot/clock
[21082056778] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21083214423] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[21087548557] [INFO] [kernel::task::loader] Segment: vaddr=203280 exec=false
[21088128084] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[21089191216] [INFO] [kernel::task::loader] Segment: vaddr=203fd8 exec=false
[21089786866] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[21212093757] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[21216191421] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/echo'
[21217544911] [INFO] [kernel::task::loader] Loading module: /boot/echo
[21227360144] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21228687986] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[21232554984] [INFO] [kernel::task::loader] Segment: vaddr=202240 exec=false
[21233248782] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[21234462086] [INFO] [kernel::task::loader] Segment: vaddr=202fa0 exec=false
[21235079875] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[21254711470] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[21255955989] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[21257641606] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[21263099331] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21264249943] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[21269028235] [INFO] [kernel::task::loader] Segment: vaddr=203cd8 exec=false
[21269680991] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[21284317144] [INFO] [kernel::task::loader] Segment: vaddr=205d30 exec=false
[21285711336] [INFO] [kernel::task::loader]   Overlap at 205000: merging perms to r=true w=true x=true
[21293030969] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[21301368394] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/png_creator'
[21302530365] [INFO] [kernel::task::loader] Loading module: /boot/png_creator
[21303219837] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21304321617] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[21314845299] [INFO] [kernel::task::loader] Segment: vaddr=201870 exec=false
[21315630223] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[21323083046] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[21324065711] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[21341606372] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[21343311991] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/bindd'
[21344680300] [INFO] [kernel::task::loader] Loading module: /boot/bindd
[21345284956] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21346381662] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[21362162535] [INFO] [kernel::task::loader] Segment: vaddr=202028 exec=false
[21363337107] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[21364305015] [INFO] [kernel::task::loader] Segment: vaddr=2025d8 exec=false
[21365080581] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[21382083608] [INFO] [sprout::supervisor] SPROUT: App launched (PID=8)
[21383369593] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[21418106244] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb01293f0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21419681451] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562145933 RSP_BEFORE=18446744072368590368 RFLAGS_BEFORE=130 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563768496
[21431831779] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x20002f rflags=0x202
[21433267530] [INFO] [clock] starting clock publisher
[21465445769] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb012f560
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21467100700] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562145933 RSP_BEFORE=18446744072368615312 RFLAGS_BEFORE=134 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563768496
[21476070985] [INFO] [echo] echo: online (handle=0)
[21477066682] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[21496340980] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0133cd0
USER_TRAMPOLINE: PC=0x200780 SP=0x800000 ARG0=0x0
[21497983083] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2099072 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562145933 RSP_BEFORE=18446744072368633600 RFLAGS_BEFORE=134 CR3_BEFORE=50950144 fs_base=0 gs_base=18446744071563768496
[21500249291] [ERROR] [INGESTD] Starting...
[21501243991] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff990
[21502574032] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[21503718098] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21531493227] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb013ae50
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21533471608] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562145933 RSP_BEFORE=18446744072368662656 RFLAGS_BEFORE=130 CR3_BEFORE=51060736 fs_base=0 gs_base=18446744071563768496
[21543815322] [ERROR] [PNG_CREATOR] Starting...
[21548861354] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb013f600
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21550087062] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562145933 RSP_BEFORE=18446744072368681008 RFLAGS_BEFORE=134 CR3_BEFORE=51159040 fs_base=0 gs_base=18446744071563768496
[21562698312] [INFO] [bindd] bindd starting...
[21578103554] [INFO] [clock] Clock thing created: 339
[21579147674] [INFO] [clock] Waiting for UI Root (Compositor)...
[21589016511] [ERROR] [INGESTD] Watch active. Loop start.
[21666016318] [ERROR] [PNG_CREATOR] Created bytespace node 354
[21681702933] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[21825248152] [ERROR] [PNG_CREATOR] Wrote PNG header
[21835181796] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[21836991529] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[23165130261] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[23166758814] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[23167356054] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[23168476953] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[23181793168] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[23182514991] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[23183988693] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[23184516464] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[23204979796] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[23206323679] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[23207838908] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[23217949650] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[23219588179] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[23221037321] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[23221908082] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23222999904] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[23225953999] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[23226655046] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[23227679840] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[23228275456] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[23358156642] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0154b40
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xbd
[23403116006] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562145933 RSP_BEFORE=18446744072368768368 RFLAGS_BEFORE=130 CR3_BEFORE=55214080 fs_base=0 gs_base=18446744071563768496
[23413334281] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[23414966445] [INFO] [rtc_cmos] Starting... arg=bd
[23417017159] [INFO] [rtc_cmos] Serving device ID: ThingId([189, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[23430837005] [INFO] [rtc_cmos] RTC: 2026-01-23 19:28:24 = 1769196504 unix_secs
[23432399831] [INFO] [kernel::time] System clock anchored: unix_secs=1769196504, mono_ns=11715871102, offset=1769196492284128898ns
[23433657971] [INFO] [rtc_cmos] System clock anchored
[23467311939] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[23469762849] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb015d6a0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[23471001690] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562145933 RSP_BEFORE=18446744072368804048 RFLAGS_BEFORE=130 CR3_BEFORE=55316480 fs_base=0 gs_base=18446744071563768496
[24113139489] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[24116211519] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[24129478540] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[24130406776] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[24139919950] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[24141999183] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[24142573981] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24143800318] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[24156882926] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[24157522176] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[24158805757] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[24159414559] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[24172456806] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[24184501145] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[24186084748] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[24186762021] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24187793363] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[24203301274] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[24203912246] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[24205048329] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[24205662089] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[24213632900] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[24242237589] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0167c70
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[24243516662] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562145933 RSP_BEFORE=18446744072368846496 RFLAGS_BEFORE=134 CR3_BEFORE=55410688 fs_base=0 gs_base=18446744071563768496
[24247493901] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[24254562786] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[24258918992] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb016e5d0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[24260301774] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562145933 RSP_BEFORE=18446744072368873472 RFLAGS_BEFORE=134 CR3_BEFORE=55513088 fs_base=0 gs_base=18446744071563768496
[24265278195] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[24284156007] [INFO] [bristle] bristle: registered in graph as svc.Input (id=435)
[24307889653] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=426 backend=BootFB
[24309439006] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[24310033033] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24311159762] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[24432581897] [INFO] [kernel::task::loader] Segment: vaddr=242250 exec=false
[24433581113] [INFO] [kernel::task::loader]   Overlap at 242000: merging perms to r=true w=false x=true
[24455879882] [INFO] [kernel::task::loader] Segment: vaddr=2516d0 exec=false
[24456734487] [INFO] [kernel::task::loader]   Overlap at 251000: merging perms to r=true w=true x=true
[24475587383] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[24477004973] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[24481010708] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[24500311097] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0176d50
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1aa
[24502054950] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562145933 RSP_BEFORE=18446744072368908160 RFLAGS_BEFORE=130 CR3_BEFORE=55619584 fs_base=0 gs_base=18446744071563768496
[24517199322] [INFO] [bloom::logging] bloom: logging initialized
[24533667609] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[24534935963] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[24535952626] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
[24541269201] [INFO] [bloom] bloom: [bloom] Bootstrapped via Bytespace ID=426
[24553211634] [INFO] [bloom] bloom: [bloom] starting (arg_req=1 arg_resp=4 bristle=10 font_svc=0)
[24590727197] [INFO] [bloom] bloom: [bloom] spawned wallpaper_loader thread (tid=14)
[24598533071] [INFO] [bloom] bloom: [bloom] spawned cursor_loader thread (tid=15)
[24606166172] [INFO] [bloom] bloom: [bloom] spawned font_loader thread (tid=16)
[24626440671] [INFO] [bloom] bloom: [bloom] discovering compositor target...
T:CD90 [24639581105] [INFO] [bloom] bloom: [wallpaper_loader] thread started
T:C9C0 [24642330138] [INFO] [bloom] bloom: [cursor_loader] thread started
T:C550 [24645342444] [INFO] [bloom] bloom: [font_loader] thread started
[24646026891] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c1d98
[24646700956] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[24647336541] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[24668509583] [INFO] [bloom] bloom: [font_loader] watch opened (id=466)
[24682956276] [INFO] [bloom] bloom: [font_loader] watch next error: EAGAIN
[24786037431] [INFO] [bloom::compositor] bloom: compositor bytespace 367 (1280x720 stride=5120 format=1)
[24802497408] [INFO] [bloom::compositor] bloom: display backend: BootFB
[24807333320] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[24826745839] [INFO] [bloom] bloom: [bloom] compositor target: 1280x720 @ 0x10002000 backend=BootFB
[24837479599] [INFO] [bloom] bloom: [bloom] presenter: driver (req=1 resp=4)
[24841646304] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[24860995048] [INFO] [stem::ui] UiBuilder: created root 475
[24896499362] [INFO] [bloom] bloom: [bloom] entering transactional frame loop (acquire -> build -> present)
[24897862043] [INFO] [bloom] bloom: [bloom] reclaimer: budget=33554432 bytes
[25225411477] [INFO] [bloom] bloom: [wallpaper_loader] searching for wallpaper...
[25226783662] [INFO] [bloom] bloom: [wallpaper_loader] trying: /assets/wallpapers/clouds.bmp
[25269166296] [INFO] [ps2_mouse] ps2_mouse: init done
[25270133037] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[25271112376] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[25271842907] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[25325860864] [INFO] [bloom] bloom: [wallpaper_loader] SUCCESS: found candidate '/assets/wallpapers/clouds.bmp', enqueuing load
[25334981892] [INFO] [bloom::asset] [asset_bank] worker spawned tid=17 (priority=2)
[25335939649] [INFO] [bloom] bloom: [wallpaper_loader] thread done, sleeping forever
T:3920 [25354784884] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[25356207595] [INFO] [bloom::asset] [asset_bank] load_wallpaper_immediate: /assets/wallpapers/clouds.bmp
[25373111202] [INFO] [clock] Found UI Root: 475 (attempt 1)
[25505831182] [INFO] [bloom::asset] [asset_bank] mapping bytespace 122 (3145782 bytes)
[25523678845] [INFO] [bloom::asset] [asset_bank] mapped to 0x10386000
[25524751013] [INFO] [bloom::asset] [asset_bank] decoding BMP...
[26766092367] [INFO] [bloom::asset] [asset_bank] BMP decoded: 1024x1024
[27811074676] [INFO] [bloom] bloom: [cursor_loader] searching for cursor...
[27812374144] [INFO] [bloom] bloom: [cursor_loader] trying: /assets/cursors/plain/Normal.cur
[27959031446] [INFO] [bloom::asset] [asset_bank] bytespace unmapped
[27974610798] [INFO] [bloom::asset] [asset_bank] publish_wallpaper (pending): 1024x1024
[28008289320] [INFO] [clock] Binding created: 509 (Clock->Text)
[28023247918] [INFO] [clock] unix=1769196506 utc=2026-01-23 19:28:26 mono_ns=14010618091
[28116419633] [INFO] [bloom::reclaimer] [reclaimer] +4194304 bytes (total: 4194304)
[28132571781] [INFO] [bloom::asset] [asset_bank] promoting wallpaper to gen=1 (4194304b)
[28134697651] [INFO] [bloom] bloom: [bloom] frame 12: wallpaper now visible (gen=1)
[28350582526] [INFO] [bloom] bloom: [font_loader] watch next error: EBADF
P[28414036433] [ERROR] [kernel::trap] user_page_fault va=0xff2d7ed0ff2e7fd0 rip=0x0000000000216d3c err=0x0006 present=0 user=1 write=1 instr_fetch=0

```
</details>
