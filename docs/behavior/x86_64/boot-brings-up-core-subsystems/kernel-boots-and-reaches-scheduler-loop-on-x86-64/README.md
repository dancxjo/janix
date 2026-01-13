# ✅ Scenario: Kernel boots and reaches scheduler loop on x86_64

> Last run: 2026-01-12 19:08:36

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 313ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I wait for the system to reach ready state | ✅ | 3436ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the boot log should contain all required signals | ✅ | 583ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And the system should show liveness | ✅ | 207ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[9513208650] [INFO] [kernel] thing-os kernel starting...
[9528413037] [INFO] [kernel::memory] Memory map has 64 entries
[9530347101] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[9531423528] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[9531933378] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[9532504740] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[9533012544] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[9533473224] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[9533983536] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[9534445734] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[9534975747] [INFO] [kernel::memory]   [8] 0x1780000 - 0x796dc000 (Usable)
[9535479294] [INFO] [kernel::memory]   [9] 0x796dc000 - 0x796f4000 (Reserved)
[9536011353] [INFO] [kernel::memory]   [10] 0x796f4000 - 0x79a01000 (Other)
[9536579514] [INFO] [kernel::memory]   [11] 0x79a01000 - 0x79b74000 (Other)
[9537148929] [INFO] [kernel::memory]   [12] 0x79b74000 - 0x7a16c000 (Reserved)
[9537657657] [INFO] [kernel::memory]   [13] 0x7a16c000 - 0x7bb6c000 (Usable)
[9538155165] [INFO] [kernel::memory]   [14] 0x7bb6c000 - 0x7bbd1000 (Reserved)
[9538706199] [INFO] [kernel::memory]   [15] 0x7bbd1000 - 0x7bbd5000 (Other)
[9539244495] [INFO] [kernel::memory]   [16] 0x7bbd5000 - 0x7bbd6000 (Reserved)
[9539812227] [INFO] [kernel::memory]   [17] 0x7bbd6000 - 0x7bbd9000 (Other)
[9540341316] [INFO] [kernel::memory]   [18] 0x7bbd9000 - 0x7bbda000 (Reserved)
[9540877566] [INFO] [kernel::memory]   [19] 0x7bbda000 - 0x7bbdd000 (Other)
[9541409229] [INFO] [kernel::memory]   [20] 0x7bbdd000 - 0x7bbde000 (Reserved)
[9541970031] [INFO] [kernel::memory]   [21] 0x7bbde000 - 0x7bbe1000 (Other)
[9542522418] [INFO] [kernel::memory]   [22] 0x7bbe1000 - 0x7bbe2000 (Reserved)
[9543139947] [INFO] [kernel::memory]   [23] 0x7bbe2000 - 0x7bbe5000 (Other)
[9543718107] [INFO] [kernel::memory]   [24] 0x7bbe5000 - 0x7bbe7000 (Reserved)
[9544309269] [INFO] [kernel::memory]   [25] 0x7bbe7000 - 0x7bbea000 (Other)
[9544902477] [INFO] [kernel::memory]   [26] 0x7bbea000 - 0x7bbeb000 (Reserved)
[9545530203] [INFO] [kernel::memory]   [27] 0x7bbeb000 - 0x7bbef000 (Other)
[9546152781] [INFO] [kernel::memory]   [28] 0x7bbef000 - 0x7bbf0000 (Reserved)
[9546770409] [INFO] [kernel::memory]   [29] 0x7bbf0000 - 0x7bbf3000 (Other)
[9547358535] [INFO] [kernel::memory]   [30] 0x7bbf3000 - 0x7bbf4000 (Reserved)
[9547965636] [INFO] [kernel::memory]   [31] 0x7bbf4000 - 0x7bbf7000 (Other)
[9548564355] [INFO] [kernel::memory]   [32] 0x7bbf7000 - 0x7bbf8000 (Reserved)
[9549155055] [INFO] [kernel::memory]   [33] 0x7bbf8000 - 0x7bbfb000 (Other)
[9549800205] [INFO] [kernel::memory]   [34] 0x7bbfb000 - 0x7bbfc000 (Reserved)
[9550413345] [INFO] [kernel::memory]   [35] 0x7bbfc000 - 0x7bbfe000 (Other)
[9550965435] [INFO] [kernel::memory]   [36] 0x7bbfe000 - 0x7bbff000 (Reserved)
[9551529867] [INFO] [kernel::memory]   [37] 0x7bbff000 - 0x7bc01000 (Other)
[9552085323] [INFO] [kernel::memory]   [38] 0x7bc01000 - 0x7bc02000 (Reserved)
[9553060044] [INFO] [kernel::memory]   [39] 0x7bc02000 - 0x7bc06000 (Other)
[9553639557] [INFO] [kernel::memory]   [40] 0x7bc06000 - 0x7bc07000 (Reserved)
[9554243490] [INFO] [kernel::memory]   [41] 0x7bc07000 - 0x7bc0a000 (Other)
[9554821056] [INFO] [kernel::memory]   [42] 0x7bc0a000 - 0x7bc0b000 (Reserved)
[9555414396] [INFO] [kernel::memory]   [43] 0x7bc0b000 - 0x7bc1b000 (Other)
[9556005195] [INFO] [kernel::memory]   [44] 0x7bc1b000 - 0x7bc29000 (Reserved)
[9556596786] [INFO] [kernel::memory]   [45] 0x7bc29000 - 0x7e219000 (Usable)
[9557168577] [INFO] [kernel::memory]   [46] 0x7e219000 - 0x7e93f000 (Reserved)
[9557758485] [INFO] [kernel::memory]   [47] 0x7e93f000 - 0x7ea00000 (Reserved)
[9558359250] [INFO] [kernel::memory]   [48] 0x7ea00000 - 0x7f4ed000 (Reserved)
[9558959817] [INFO] [kernel::memory]   [49] 0x7f4ed000 - 0x7f5ed000 (Reserved)
[9559560351] [INFO] [kernel::memory]   [50] 0x7f5ed000 - 0x7f6ed000 (Reserved)
[9560061819] [INFO] [kernel::memory]   [51] 0x7f6ed000 - 0x7f76d000 (Reserved)
[9560611632] [INFO] [kernel::memory]   [52] 0x7f76d000 - 0x7f77f000 (Acpi)
[9561238764] [INFO] [kernel::memory]   [53] 0x7f77f000 - 0x7f7ff000 (Other)
[9561784023] [INFO] [kernel::memory]   [54] 0x7f7ff000 - 0x7fec1000 (Reserved)
[9562392312] [INFO] [kernel::memory]   [55] 0x7fec1000 - 0x7fec5000 (Reserved)
[9563057262] [INFO] [kernel::memory]   [56] 0x7fec5000 - 0x7fec7000 (Other)
[9563652846] [INFO] [kernel::memory]   [57] 0x7fec7000 - 0x7fef4000 (Reserved)
[9564278031] [INFO] [kernel::memory]   [58] 0x7fef4000 - 0x7ff78000 (Reserved)
[9564908397] [INFO] [kernel::memory]   [59] 0x7ff78000 - 0x80000000 (Other)
[9565533021] [INFO] [kernel::memory]   [60] 0x80000000 - 0x80384000 (Framebuffer)
[9566279514] [INFO] [kernel::memory]   [61] 0xe0000000 - 0xf0000000 (Reserved)
[9566925324] [INFO] [kernel::memory]   [62] 0xffc00000 - 0x100000000 (Reserved)
[9567575589] [INFO] [kernel::memory]   [63] 0xfd00000000 - 0x10000000000 (Reserved)
[9568619907] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[9833303106] [INFO] [kernel::memory] Frame allocator initialized with 509668 free frames
[9840096981] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[9844905213] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[9846046617] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[9846769977] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[9853175937] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[9853678857] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[9856722777] [INFO] [bran::arch] IOAPIC: Registers initialized
[9857896125] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[9859456662] [INFO] [bran::arch] IOAPIC: All pins masked
[9860821872] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[9861451644] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[9861946710] [INFO] [bran::arch] IOAPIC: Init complete
[9862354491] [INFO] [kernel] Initializing global allocator...
[9867985710] [INFO] [kernel::memory::global_alloc] Global allocator initialized
[9868671186] [INFO] [kernel] Initializing SIMD...
[9870185391] [INFO] [kernel] Initializing tasking...
[9875194296] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[9875864889] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[9876453543] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[9879230493] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[9879656622] [INFO] [kernel::task::scheduler]   Initializing boot task...
[9880447863] [INFO] [kernel::task::scheduler]   Creating boot task...
[9885026151] [INFO] [kernel::task::scheduler]   Creating idle task...
[9889116336] [INFO] [kernel::task::scheduler]   Boot task initialized
[9889522632] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[9890324961] [INFO] [kernel::task::scheduler]   Scheduler initialized
[9896848797] [INFO] [kernel::root] Spawning Root service...
[9904363689] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[9910230759] [INFO] [kernel::root::service] ROOT: started once
[10258932024] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[10259788143] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[10306952073] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[10319063205] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[10336207266] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[10359040857] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[10361342805] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[10383759210] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[10395904926] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[10413743340] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[10415928600] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[10418659944] [INFO] [logging] BEGIN rootdump
[10419306876] [INFO] [kernel::root::handlers::debug] ROOT DUMP NODES count=150
(entry1:log.Entry { level: 3, line: 176, timestamp: 0x24dfcb786, message: 7, event: 6, file: 12, module: 6, tid: 0 })
(entry2:log.Entry { level: 3, line: 28, timestamp: 0x24e6045b5, message: 14, event: 13, file: 15, module: 13, tid: 0 })
(host3:dev.Host { hhdm_offset: 0xffff800000000000, arch: "x86_64", platform_profile: "unknown", source: 0, confidence: 2 })
(entry4:log.Entry { level: 3, line: 12, timestamp: 0x24ebd9385, message: 18, event: 17, file: 19, module: 17, tid: 2 })
(platform5:dev.bus.Platform { source: 0, confidence: 2, name: "platform0" })
(kernel6:proc.Kernel { version: 1 })
(root7:svc.Root {  })
(cpu8:dev.Cpu { source: 0, confidence: 2, id: 0 })
(range9:mem.Range { source: 0, confidence: 2, start: 0, end: 0xa0000, kind: 0 })
(rangeA:mem.Range { source: 0, confidence: 2, start: 0x100000, end: 0x800000, kind: 0 })
(rangeB:mem.Range { source: 0, confidence: 2, start: 0x800000, end: 0x808000, kind: 8 })
(rangeC:mem.Range { source: 0, confidence: 2, start: 0x808000, end: 0x80b000, kind: 0 })
(rangeD:mem.Range { source: 0, confidence: 2, start: 0x80b000, end: 0x80c000, kind: 8 })
(rangeE:mem.Range { source: 0, confidence: 2, start: 0x80c000, end: 0x811000, kind: 0 })
(rangeF:mem.Range { source: 0, confidence: 2, start: 0x811000, end: 0x900000, kind: 8 })
(range10:mem.Range { source: 0, confidence: 2, start: 0x900000, end: 0x1780000, kind: 1 })
(range11:mem.Range { source: 0, confidence: 2, start: 0x1780000, end: 0x796dc000, kind: 0 })
(range12:mem.Range { source: 0, confidence: 2, start: 0x796dc000, end: 0x796f4000, kind: 1 })
(range13:mem.Range { source: 0, confidence: 2, start: 0x796f4000, end: 0x79a01000, kind: 8 })
(range14:mem.Range { source: 0, confidence: 2, start: 0x79a01000, end: 0x79b74000, kind: 8 })
(range15:mem.Range { source: 0, confidence: 2, start: 0x79b74000, end: 0x7a16c000, kind: 1 })
(range16:mem.Range { source: 0, confidence: 2, start: 0x7a16c000, end: 0x7bb6c000, kind: 0 })
(range17:mem.Range { source: 0, confidence: 2, start: 0x7bb6c000, end: 0x7bbd1000, kind: 1 })
(range18:mem.Range { source: 0, confidence: 2, start: 0x7bbd1000, end: 0x7bbd5000, kind: 8 })
(range19:mem.Range { source: 0, confidence: 2, start: 0x7bbd5000, end: 0x7bbd6000, kind: 1 })
(range1A:mem.Range { source: 0, confidence: 2, start: 0x7bbd6000, end: 0x7bbd9000, kind: 8 })
(range1B:mem.Range { source: 0, confidence: 2, start: 0x7bbd9000, end: 0x7bbda000, kind: 1 })
(range1C:mem.Range { source: 0, confidence: 2, start: 0x7bbda000, end: 0x7bbdd000, kind: 8 })
(range1D:mem.Range { source: 0, confidence: 2, start: 0x7bbdd000, end: 0x7bbde000, kind: 1 })
(range1E:mem.Range { source: 0, confidence: 2, start: 0x7bbde000, end: 0x7bbe1000, kind: 8 })
(range1F:mem.Range { source: 0, confidence: 2, start: 0x7bbe1000, end: 0x7bbe2000, kind: 1 })
(range20:mem.Range { source: 0, confidence: 2, start: 0x7bbe2000, end: 0x7bbe5000, kind: 8 })
(range21:mem.Range { source: 0, confidence: 2, start: 0x7bbe5000, end: 0x7bbe7000, kind: 1 })
(range22:mem.Range { source: 0, confidence: 2, start: 0x7bbe7000, end: 0x7bbea000, kind: 8 })
(range23:mem.Range { source: 0, confidence: 2, start: 0x7bbea000, end: 0x7bbeb000, kind: 1 })
(range24:mem.Range { source: 0, confidence: 2, start: 0x7bbeb000, end: 0x7bbef000, kind: 8 })
(range25:mem.Range { source: 0, confidence: 2, start: 0x7bbef000, end: 0x7bbf0000, kind: 1 })
(range26:mem.Range { source: 0, confidence: 2, start: 0x7bbf0000, end: 0x7bbf3000, kind: 8 })
(range27:mem.Range { source: 0, confidence: 2, start: 0x7bbf3000, end: 0x7bbf4000, kind: 1 })
(range28:mem.Range { source: 0, confidence: 2, start: 0x7bbf4000, end: 0x7bbf7000, kind: 8 })
(range29:mem.Range { source: 0, confidence: 2, start: 0x7bbf7000, end: 0x7bbf8000, kind: 1 })
(range2A:mem.Range { source: 0, confidence: 2, start: 0x7bbf8000, end: 0x7bbfb000, kind: 8 })
(range2B:mem.Range { source: 0, confidence: 2, start: 0x7bbfb000, end: 0x7bbfc000, kind: 1 })
(range2C:mem.Range { source: 0, confidence: 2, start: 0x7bbfc000, end: 0x7bbfe000, kind: 8 })
(range2D:mem.Range { source: 0, confidence: 2, start: 0x7bbfe000, end: 0x7bbff000, kind: 1 })
(range2E:mem.Range { source: 0, confidence: 2, start: 0x7bbff000, end: 0x7bc01000, kind: 8 })
(range2F:mem.Range { source: 0, confidence: 2, start: 0x7bc01000, end: 0x7bc02000, kind: 1 })
(range30:mem.Range { source: 0, confidence: 2, start: 0x7bc02000, end: 0x7bc06000, kind: 8 })
(range31:mem.Range { source: 0, confidence: 2, start: 0x7bc06000, end: 0x7bc07000, kind: 1 })
(range32:mem.Range { source: 0, confidence: 2, start: 0x7bc07000, end: 0x7bc0a000, kind: 8 })
(range33:mem.Range { source: 0, confidence: 2, start: 0x7bc0a000, end: 0x7bc0b000, kind: 1 })
(range34:mem.Range { source: 0, confidence: 2, start: 0x7bc0b000, end: 0x7bc1b000, kind: 8 })
(range35:mem.Range { source: 0, confidence: 2, start: 0x7bc1b000, end: 0x7bc29000, kind: 1 })
(range36:mem.Range { source: 0, confidence: 2, start: 0x7bc29000, end: 0x7e219000, kind: 0 })
(range37:mem.Range { source: 0, confidence: 2, start: 0x7e219000, end: 0x7e93f000, kind: 1 })
(range38:mem.Range { source: 0, confidence: 2, start: 0x7e93f000, end: 0x7ea00000, kind: 1 })
(range39:mem.Range { source: 0, confidence: 2, start: 0x7ea00000, end: 0x7f4ed000, kind: 1 })
(range3A:mem.Range { source: 0, confidence: 2, start: 0x7f4ed000, end: 0x7f5ed000, kind: 1 })
(range3B:mem.Range { source: 0, confidence: 2, start: 0x7f5ed000, end: 0x7f6ed000, kind: 1 })
(range3C:mem.Range { source: 0, confidence: 2, start: 0x7f6ed000, end: 0x7f76d000, kind: 1 })
(range3D:mem.Range { source: 0, confidence: 2, start: 0x7f76d000, end: 0x7f77f000, kind: 7 })
(range3E:mem.Range { source: 0, confidence: 2, start: 0x7f77f000, end: 0x7f7ff000, kind: 8 })
(range3F:mem.Range { source: 0, confidence: 2, start: 0x7f7ff000, end: 0x7fec1000, kind: 1 })
(range40:mem.Range { source: 0, confidence: 2, start: 0x7fec1000, end: 0x7fec5000, kind: 1 })
(range41:mem.Range { source: 0, confidence: 2, start: 0x7fec5000, end: 0x7fec7000, kind: 8 })
(range42:mem.Range { source: 0, confidence: 2, start: 0x7fec7000, end: 0x7fef4000, kind: 1 })
(range43:mem.Range { source: 0, confidence: 2, start: 0x7fef4000, end: 0x7ff78000, kind: 1 })
(range44:mem.Range { source: 0, confidence: 2, start: 0x7ff78000, end: 0x80000000, kind: 8 })
(range45:mem.Range { source: 0, confidence: 2, start: 0x80000000, end: 0x80384000, kind: 6 })
(range46:mem.Range { source: 0, confidence: 2, start: 0xe0000000, end: 0xf0000000, kind: 1 })
(range47:mem.Range { source: 0, confidence: 2, start: 0xffc00000, end: 0x100000000, kind: 1 })
(range48:mem.Range { source: 0, confidence: 2, start: 0xfd00000000, end: 0x10000000000, kind: 1 })
(module49:boot.Module { source: 0, confidence: 2, name: "/boot/sprout", phys_base: 0x7bc0b000, size_bytes: 62316, index: 0 })
(bytespace4A:Bytespace {  })
(range4B:mem.Range { phys_base: 0x7bc0b000, size_bytes: 62316 })
(module4C:boot.Module { source: 0, confidence: 2, name: "/boot/threads", phys_base: 0x7bc07000, size_bytes: 11416, index: 1 })
(bytespace4D:Bytespace {  })
(range4E:mem.Range { phys_base: 0x7bc07000, size_bytes: 11416 })
(module4F:boot.Module { source: 0, confidence: 2, name: "/boot/rtc_cmos", phys_base: 0x7bc02000, size_bytes: 13812, index: 2 })
(bytespace50:Bytespace {  })
(range51:mem.Range { phys_base: 0x7bc02000, size_bytes: 13812 })
(module52:boot.Module { source: 0, confidence: 2, name: "/boot/clock", phys_base: 0x7bbff000, size_bytes: 6368, index: 3 })
(bytespace53:Bytespace {  })
(range54:mem.Range { phys_base: 0x7bbff000, size_bytes: 6368 })
(module55:boot.Module { source: 0, confidence: 2, name: "/boot/ps2_kbd", phys_base: 0x7bbfc000, size_bytes: 7312, index: 4 })
(bytespace56:Bytespace {  })
(range57:mem.Range { phys_base: 0x7bbfc000, size_bytes: 7312 })
(module58:boot.Module { source: 0, confidence: 2, name: "/boot/ps2_mouse", phys_base: 0x7bbf8000, size_bytes: 9088, index: 5 })
(bytespace59:Bytespace {  })
(range5A:mem.Range { phys_base: 0x7bbf8000, size_bytes: 9088 })
(module5B:boot.Module { source: 0, confidence: 2, name: "/boot/bristle", phys_base: 0x7bbf4000, size_bytes: 11664, index: 6 })
(bytespace5C:Bytespace {  })
(range5D:mem.Range { phys_base: 0x7bbf4000, size_bytes: 11664 })
(module5E:boot.Module { source: 0, confidence: 2, name: "/boot/echo", phys_base: 0x7bbf0000, size_bytes: 12104, index: 7 })
(bytespace5F:Bytespace {  })
(range60:mem.Range { phys_base: 0x7bbf0000, size_bytes: 12104 })
(module61:boot.Module { source: 0, confidence: 2, name: "/boot/virtio_gpu", phys_base: 0x7bbeb000, size_bytes: 16208, index: 8 })
(bytespace62:Bytespace {  })
(range63:mem.Range { phys_base: 0x7bbeb000, size_bytes: 16208 })
(module64:boot.Module { source: 0, confidence: 2, name: "/boot/inkwell", phys_base: 0x7bbe7000, size_bytes: 10232, index: 9 })
(bytespace65:Bytespace {  })
(range66:mem.Range { phys_base: 0x7bbe7000, size_bytes: 10232 })
(module67:boot.Module { source: 0, confidence: 2, name: "/boot/blossom", phys_base: 0x796f4000, size_bytes: 0x30c84c, index: 10 })
(bytespace68:Bytespace {  })
(range69:mem.Range { phys_base: 0x796f4000, size_bytes: 0x30c84c })
(module6A:boot.Module { source: 0, confidence: 2, name: "/boot/bloom", phys_base: 0x7bbe2000, size_bytes: 8780, index: 11 })
(bytespace6B:Bytespace {  })
(range6C:mem.Range { phys_base: 0x7bbe2000, size_bytes: 8780 })
(module6D:boot.Module { source: 0, confidence: 2, name: "/boot/display_bootfb", phys_base: 0x7bbde000, size_bytes: 11452, index: 12 })
(bytespace6E:Bytespace {  })
(range6F:mem.Range { phys_base: 0x7bbde000, size_bytes: 11452 })
(module70:boot.Module { source: 0, confidence: 2, name: "/boot/display_virtio_gpu", phys_base: 0x7bbda000, size_bytes: 9244, index: 13 })
(bytespace71:Bytespace {  })
(range72:mem.Range { phys_base: 0x7bbda000, size_bytes: 9244 })
(module73:boot.Module { source: 0, confidence: 2, name: "/boot/display_ramfb", phys_base: 0x7bbd6000, size_bytes: 10500, index: 14 })
(bytespace74:Bytespace {  })
(range75:mem.Range { phys_base: 0x7bbd6000, size_bytes: 10500 })
(module76:boot.Module { source: 0, confidence: 2, name: "/boot/stack_heap_torture", phys_base: 0x7bbd1000, size_bytes: 16228, index: 15 })
(bytespace77:Bytespace {  })
(range78:mem.Range { phys_base: 0x7bbd1000, size_bytes: 16228 })
(framebuffer79:dev.display.Framebuffer { source: 0, confidence: 2, phys_base: 0x80000000, size_bytes: 0x384000, virt_base: 0xffff800080000000, width: 1280, height: 720, stride: 5120, ... })
(boot7A:fw.Boot { source: 0, confidence: 2 })
(acpi7B:fw.table.Acpi { source: 0, confidence: 2, phys_base: 0x7f77e014 })
(bytespace7C:Bytespace {  })
(range7D:mem.Range { phys_base: 0x265000, size_bytes: 4096, page_count: 1 })
(scheduler7E:svc.Scheduler {  })
(entry7F:log.Entry { level: 3, line: 334, timestamp: 0x26383a892, message: 81, event: 13, file: 15, module: 13, tid: 0 })
(entry80:log.Entry { level: 3, line: 108, timestamp: 0x2638d2c62, message: 83, event: 82, file: 84, module: 82, tid: 0 })
(pci81:dev.bus.Pci { source: 4, confidence: 2, name: "pci0", segment: 0 })
(function82:dev.pci.Function { source: 4, confidence: 2, name: "Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller", bus: 0, device: 0, function: 0, vendor_id: 32902, device_id: 10688, ... })
(entry83:log.Entry { level: 3, line: 261, timestamp: 0x26671e1bc, message: 109, event: 82, file: 84, module: 82, tid: 0 })
(function84:dev.pci.Function { source: 4, confidence: 2, bus: 0, device: 1, function: 0, vendor_id: 4660, device_id: 4369, class_code: 3, ... })
(entry85:log.Entry { level: 3, line: 261, timestamp: 0x26722f4e8, message: 111, event: 82, file: 84, module: 82, tid: 0 })
(function86:dev.pci.Function { source: 4, confidence: 2, name: "Intel Corporation 82574L Gigabit Network Connection", bus: 0, device: 2, function: 0, vendor_id: 32902, device_id: 4307, ... })
(entry87:log.Entry { level: 3, line: 261, timestamp: 0x268237ac7, message: 117, event: 82, file: 84, module: 82, tid: 0 })
(function88:dev.pci.Function { source: 4, confidence: 2, name: "Intel Corporation 82801IB (ICH9) LPC Interface Controller", bus: 0, device: 31, function: 0, vendor_id: 32902, device_id: 10520, ... })
(entry89:log.Entry { level: 3, line: 261, timestamp: 0x26989c206, message: 125, event: 82, file: 84, module: 82, tid: 0 })
(entry8A:log.Entry { level: 3, line: 369, timestamp: 0x2699e8030, message: 126, event: 82, file: 84, module: 82, tid: 0 })
(lpc8B:dev.bridge.Lpc { source: 4, confidence: 2, name: "lpc0" })
(legacyio8C:dev.bus.LegacyIo { source: 4, confidence: 2, name: "isa0" })
(cmos8D:dev.rtc.Cmos { source: 4, confidence: 2, name: "rtc0" })
(range8E:cap.ioport.Range { port_start: 112, port_end: 113 })
(ps2controller8F:dev.input.Ps2Controller { source: 4, confidence: 2, name: "i8042" })
(range90:cap.ioport.Range { port_start: 96, port_end: 100 })
(entry91:log.Entry { level: 3, line: 499, timestamp: 0x26af4933b, message: 140, event: 82, file: 84, module: 82, tid: 0 })
(function92:dev.pci.Function { source: 4, confidence: 2, name: "Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode]", bus: 0, device: 31, function: 2, vendor_id: 32902, device_id: 10530, ... })
(entry93:log.Entry { level: 3, line: 261, timestamp: 0x26bb419a0, message: 144, event: 82, file: 84, module: 82, tid: 0 })
(function94:dev.pci.Function { source: 4, confidence: 2, name: "Intel Corporation 82801I (ICH9 Family) SMBus Controller", bus: 0, device: 31, function: 3, vendor_id: 32902, device_id: 10544, ... })
(entry95:log.Entry { level: 3, line: 261, timestamp: 0x26cc2c8db, message: 149, event: 82, file: 84, module: 82, tid: 0 })
(entry96:log.Entry { level: 3, line: 337, timestamp: 0x26cdfdf41, message: 150, event: 13, file: 15, module: 13, tid: 0 })
[10511508777] [INFO] [kernel::root::handlers::debug] ROOT DUMP EDGES
(host3:dev.Host)-[:HAS_BUS]->(platform5:dev.bus.Platform)
(host3:dev.Host)-[:HAS_CPU]->(cpu8:dev.Cpu)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range9:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(rangeA:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(rangeB:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(rangeC:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(rangeD:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(rangeE:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(rangeF:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range10:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range11:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range12:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range13:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range14:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range15:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range16:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range17:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range18:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range19:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range1A:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range1B:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range1C:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range1D:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range1E:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range1F:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range20:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range21:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range22:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range23:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range24:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range25:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range26:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range27:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range28:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range29:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range2A:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range2B:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range2C:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range2D:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range2E:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range2F:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range30:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range31:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range32:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range33:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range34:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range35:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range36:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range37:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range38:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range39:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range3A:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range3B:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range3C:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range3D:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range3E:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range3F:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range40:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range41:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range42:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range43:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range44:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range45:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range46:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range47:mem.Range)
(host3:dev.Host)-[:HAS_MEMORY_RANGE]->(range48:mem.Range)
(host3:dev.Host)-[:HAS_MODULE]->(module49:boot.Module)
(host3:dev.Host)-[:HAS_MODULE]->(module4C:boot.Module)
(host3:dev.Host)-[:HAS_MODULE]->(module4F:boot.Module)
(host3:dev.Host)-[:HAS_MODULE]->(module52:boot.Module)
(host3:dev.Host)-[:HAS_MODULE]->(module55:boot.Module)
(host3:dev.Host)-[:HAS_MODULE]->(module58:boot.Module)
(host3:dev.Host)-[:HAS_MODULE]->(module5B:boot.Module)
(host3:dev.Host)-[:HAS_MODULE]->(module5E:boot.Module)
(host3:dev.Host)-[:HAS_MODULE]->(module61:boot.Module)
(host3:dev.Host)-[:HAS_MODULE]->(module64:boot.Module)
(host3:dev.Host)-[:HAS_MODULE]->(module67:boot.Module)
(host3:dev.Host)-[:HAS_MODULE]->(module6A:boot.Module)
(host3:dev.Host)-[:HAS_MODULE]->(module6D:boot.Module)
(host3:dev.Host)-[:HAS_MODULE]->(module70:boot.Module)
(host3:dev.Host)-[:HAS_MODULE]->(module73:boot.Module)
(host3:dev.Host)-[:HAS_MODULE]->(module76:boot.Module)
(host3:dev.Host)-[:HAS_DEVICE]->(framebuffer79:dev.display.Framebuffer)
(host3:dev.Host)-[:HAS_FIRMWARE]->(boot7A:fw.Boot)
(host3:dev.Host)-[:HAS_BUS]->(pci81:dev.bus.Pci)
(kernel6:proc.Kernel)-[:RUNS_ON]->(host3:dev.Host)
(kernel6:proc.Kernel)-[:PROVIDES]->(root7:svc.Root)
(kernel6:proc.Kernel)-[:PROVIDES]->(scheduler7E:svc.Scheduler)
(module49:boot.Module)-[:BACKED_BY]->(bytespace4A:Bytespace)
(bytespace4A:Bytespace)-[:BACKED_BY]->(range4B:mem.Range)
(module4C:boot.Module)-[:BACKED_BY]->(bytespace4D:Bytespace)
(bytespace4D:Bytespace)-[:BACKED_BY]->(range4E:mem.Range)
(module4F:boot.Module)-[:BACKED_BY]->(bytespace50:Bytespace)
(bytespace50:Bytespace)-[:BACKED_BY]->(range51:mem.Range)
(module52:boot.Module)-[:BACKED_BY]->(bytespace53:Bytespace)
(bytespace53:Bytespace)-[:BACKED_BY]->(range54:mem.Range)
(module55:boot.Module)-[:BACKED_BY]->(bytespace56:Bytespace)
(bytespace56:Bytespace)-[:BACKED_BY]->(range57:mem.Range)
(module58:boot.Module)-[:BACKED_BY]->(bytespace59:Bytespace)
(bytespace59:Bytespace)-[:BACKED_BY]->(range5A:mem.Range)
(module5B:boot.Module)-[:BACKED_BY]->(bytespace5C:Bytespace)
(bytespace5C:Bytespace)-[:BACKED_BY]->(range5D:mem.Range)
(module5E:boot.Module)-[:BACKED_BY]->(bytespace5F:Bytespace)
(bytespace5F:Bytespace)-[:BACKED_BY]->(range60:mem.Range)
(module61:boot.Module)-[:BACKED_BY]->(bytespace62:Bytespace)
(bytespace62:Bytespace)-[:BACKED_BY]->(range63:mem.Range)
(module64:boot.Module)-[:BACKED_BY]->(bytespace65:Bytespace)
(bytespace65:Bytespace)-[:BACKED_BY]->(range66:mem.Range)
(module67:boot.Module)-[:BACKED_BY]->(bytespace68:Bytespace)
(bytespace68:Bytespace)-[:BACKED_BY]->(range69:mem.Range)
(module6A:boot.Module)-[:BACKED_BY]->(bytespace6B:Bytespace)
(bytespace6B:Bytespace)-[:BACKED_BY]->(range6C:mem.Range)
(module6D:boot.Module)-[:BACKED_BY]->(bytespace6E:Bytespace)
(bytespace6E:Bytespace)-[:BACKED_BY]->(range6F:mem.Range)
(module70:boot.Module)-[:BACKED_BY]->(bytespace71:Bytespace)
(bytespace71:Bytespace)-[:BACKED_BY]->(range72:mem.Range)
(module73:boot.Module)-[:BACKED_BY]->(bytespace74:Bytespace)
(bytespace74:Bytespace)-[:BACKED_BY]->(range75:mem.Range)
(module76:boot.Module)-[:BACKED_BY]->(bytespace77:Bytespace)
(bytespace77:Bytespace)-[:BACKED_BY]->(range78:mem.Range)
(framebuffer79:dev.display.Framebuffer)-[:BACKED_BY]->(range45:mem.Range)
(boot7A:fw.Boot)-[:PROVIDES_TABLE]->(acpi7B:fw.table.Acpi)
(acpi7B:fw.table.Acpi)-[:BACKED_BY]->(bytespace7C:Bytespace)
(bytespace7C:Bytespace)-[:BACKED_BY]->(range7D:mem.Range)
(pci81:dev.bus.Pci)-[:HAS_DEVICE]->(function82:dev.pci.Function)
(pci81:dev.bus.Pci)-[:HAS_DEVICE]->(function84:dev.pci.Function)
(pci81:dev.bus.Pci)-[:HAS_DEVICE]->(function86:dev.pci.Function)
(pci81:dev.bus.Pci)-[:HAS_DEVICE]->(function88:dev.pci.Function)
(pci81:dev.bus.Pci)-[:HAS_DEVICE]->(function92:dev.pci.Function)
(pci81:dev.bus.Pci)-[:HAS_DEVICE]->(function94:dev.pci.Function)
(function88:dev.pci.Function)-[:IMPLEMENTS]->(lpc8B:dev.bridge.Lpc)
(lpc8B:dev.bridge.Lpc)-[:HAS_BUS]->(legacyio8C:dev.bus.LegacyIo)
(legacyio8C:dev.bus.LegacyIo)-[:HAS_DEVICE]->(cmos8D:dev.rtc.Cmos)
(legacyio8C:dev.bus.LegacyIo)-[:HAS_DEVICE]->(ps2controller8F:dev.input.Ps2Controller)
(cmos8D:dev.rtc.Cmos)-[:USES_IOPORTS]->(range8E:cap.ioport.Range)
(ps2controller8F:dev.input.Ps2Controller)-[:USES_IOPORTS]->(range90:cap.ioport.Range)
[10562591886] [INFO] [logging] END rootdump
[10565099556] [INFO] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[10568373156] [INFO] [kernel] Found sprout module, loading...
[10585604964] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[10586321196] [INFO] [kernel] Spawning sprout...
[10588303836] [INFO] [kernel::tests::root_test] ROOT SELFTEST: Starting...
[10590103392] [DEBUG] [sched.switch] Context switch from_tid=0 to_tid=2 from_user=0 to_user=0 cr3_before=2514944 cr3_after=2075897856
[10597964388] [DEBUG] [sched.switch] Context switch from_tid=2 to_tid=3 from_user=0 to_user=1 cr3_before=2075897856 cr3_after=2514944
[10599102558] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004c930
[10600245810] [INFO] [kernel::task::scheduler::spawn] Entering user mode: PC=0x200000 SP=0x800000
[10603283625] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562068589 RSP_BEFORE=18446744072367686528 RFLAGS_BEFORE=130 CR3_BEFORE=2514944 fs_base=0 gs_base=18446744071563563000
[10613452311] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x2003ff rflags=0x206
[10614587643] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[10615165572] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[10615819929] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[10620977994] [DEBUG] [sched.switch] Context switch from_tid=3 to_tid=0 from_user=1 to_user=0 cr3_before=2514944 cr3_after=2075897856
[10622420028] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=9e
[10623452730] [INFO] [kernel::tests::root_test] ROOT SELFTEST: Created bytespace id=158
[10633543569] [DEBUG] [sched.switch] Context switch from_tid=2 to_tid=3 from_user=0 to_user=1 cr3_before=2075897856 cr3_after=2514944
[10634544624] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=1
[10635754470] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[10636662333] [DEBUG] [sched.switch] Context switch from_tid=3 to_tid=0 from_user=1 to_user=0 cr3_before=2514944 cr3_after=2075897856
[10637561187] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=31
[10638225939] [INFO] [kernel::tests::root_test] ROOT SELFTEST: GetKind id=158 -> symbol_id=31
[10644422019] [DEBUG] [sched.switch] Context switch from_tid=2 to_tid=3 from_user=0 to_user=1 cr3_before=2075897856 cr3_after=2514944
[10645410732] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=ffff800000000000
[10646099310] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[10647051195] [DEBUG] [sched.switch] Context switch from_tid=3 to_tid=0 from_user=1 to_user=0 cr3_before=2514944 cr3_after=2075897856
[10647953976] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=af
[10648507254] [INFO] [kernel::tests::root_test] ROOT SELFTEST: Subscribed stream_id=175
[10649116929] [INFO] [kernel::tests::root_test] ROOT SELFTEST: Testing GetKind Loop...
[10650122307] [INFO] [kernel::tests::root_test] ROOT SELFTEST: Loop 0, sending GetKind...
[10655120916] [DEBUG] [sched.switch] Context switch from_tid=2 to_tid=3 from_user=0 to_user=1 cr3_before=2075897856 cr3_after=2514944
[10656130320] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=1
[10656920868] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[10657766955] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[10659172491] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=31
[10660114707] [INFO] [kernel::tests::root_test] ROOT SELFTEST: Loop 0 Success, kind=31
[10661008908] [INFO] [kernel::tests::root_test] ROOT SELFTEST: Loop 1, sending GetKind...
[10666050747] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=1
[10667097375] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[10667888715] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=31
[10668332598] [INFO] [kernel::tests::root_test] ROOT SELFTEST: Loop 1 Success, kind=31
[10668905511] [INFO] [kernel::tests::root_test] ROOT SELFTEST: Loop 2, sending GetKind...
[10671779250] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=7f77e014
[10672473405] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[10673116047] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[10674169605] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=31
[10674616194] [INFO] [kernel::tests::root_test] ROOT SELFTEST: Loop 2 Success, kind=31
[10675196103] [INFO] [kernel::tests::root_test] ROOT SELFTEST: Loop 3, sending GetKind...
[10679029944] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=0
[10679710668] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[10680476367] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[10681145673] [INFO] [sprout::devtree] SPROUT: build() called
[10681700568] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[10682843820] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=31
[10683450162] [INFO] [kernel::tests::root_test] ROOT SELFTEST: Loop 3 Success, kind=31
[10684244835] [INFO] [kernel::tests::root_test] ROOT SELFTEST: Loop 4, sending GetKind...
[10689453456] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=1
[10690274001] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[10690969179] [INFO] [sprout] SPROUT: About to create Supervisor...
[10691718213] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[10692469128] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[10693038741] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[10694086062] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=31
[10694533278] [INFO] [kernel::tests::root_test] ROOT SELFTEST: Loop 4 Success, kind=31
[10695225090] [INFO] [kernel::tests::root_test] ROOT SELFTEST: PASS
[10695773616] [INFO] [kernel] System initialized. Entering scheduler loop.
[10701244059] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=10
[10702049028] [INFO] [sprout::supervisor] SPROUT: Found 16 modules
[10729882746] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[10740994440] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[10743452148] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=10
[10752422175] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[10771008600] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[10772923491] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/threads'
[10773833532] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/inkwell'
[10774491783] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/stack_heap_torture'
[10775421030] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[10783781052] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[10784403069] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/threads'
[10790522952] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[10791123057] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/inkwell'
[10797683556] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[10798249968] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/stack_heap_torture'
[10804668831] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[10805291970] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[10814630739] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb005e870
[10815379542] [INFO] [kernel::task::scheduler::spawn] Entering user mode: PC=0x200000 SP=0x800000
[10816030962] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562068589 RSP_BEFORE=18446744072367760064 RFLAGS_BEFORE=134 CR3_BEFORE=2940928 fs_base=0 gs_base=18446744071563563000
[10819943937] [INFO] [clock] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fff30 rip=0x20002e rflags=0x206
[10820872194] [INFO] [clock] [clock] starting
[10822499721] [INFO] [clock] CLOCK: unix=5 mono_ns=5410925751
[10823358216] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0062f00
[10824099891] [INFO] [kernel::task::scheduler::spawn] Entering user mode: PC=0x200000 SP=0x800000
[10824806190] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562068589 RSP_BEFORE=18446744072367778128 RFLAGS_BEFORE=134 CR3_BEFORE=3035136 fs_base=0 gs_base=18446744071563563000
[10828557597] [INFO] [threads_demo] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fff10 rip=0x20002e rflags=0x202
[10829446155] [INFO] [threads_demo] THREADS: starting
[10836854127] [INFO] [threads_demo] Spawned thread B with ID 8
[10837568412] [INFO] [threads_demo] A: tick 0
[10838093607] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0067090
[10838823171] [INFO] [kernel::task::scheduler::spawn] Entering user mode: PC=0x200000 SP=0x800000
[10839442350] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562068589 RSP_BEFORE=18446744072367794912 RFLAGS_BEFORE=130 CR3_BEFORE=3133440 fs_base=0 gs_base=18446744071563563000
[10841984208] [INFO] [inkwell] INKWELL: Starting bytespace mapping demo
[10842981039] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb006b270
[10843666053] [INFO] [kernel::task::scheduler::spawn] Entering user mode: PC=0x200000 SP=0x800000
[10844382285] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562068589 RSP_BEFORE=18446744072367811776 RFLAGS_BEFORE=134 CR3_BEFORE=3231744 fs_base=0 gs_base=18446744071563563000
[10847228337] [INFO] [stack_heap_torture] [torture] Starting stack_heap_torture
[10848879987] [INFO] [user.print] stack_heap_torture::tests::stack: [torture] Running stack_recursion target_depth=256
[10849729341] [INFO] [user.print] stack_heap_torture::tests::stack: [torture] recursion depth=0
[10850498175] [INFO] [user.print] stack_heap_torture::tests::stack: [torture] recursion depth=64
[10851122073] [INFO] [user.print] stack_heap_torture::tests::stack: [torture] recursion depth=128
[10851726930] [INFO] [user.print] stack_heap_torture::tests::stack: [torture] recursion depth=192
[10852330302] [INFO] [user.print] stack_heap_torture::tests::stack: [torture] recursion depth=256
[10853054256] [INFO] [user.print] stack_heap_torture::tests::stack: PASS: stack_recursion
[10853679276] [INFO] [user.print] stack_heap_torture::tests::stack: [torture] Running stack_context_stress
[10855281096] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=1
T[10866397047] [INFO] [threads_demo] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x10200f70 rip=0x2003d5 rflags=0x202
[10867211520] [INFO] [threads_demo] B: tick 0
[10868338701] [INFO] [user.print] stack_heap_torture::tests::stack: [torture] context_stress iteration=0
[10933092588] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=f2
[10935106182] [INFO] [inkwell] INKWELL: Created bytespace id=242
[10936345992] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=500
[10942286817] [INFO] [inkwell] INKWELL: Physical base = 0x340000
[10965695763] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=2d0
[10976251572] [INFO] [inkwell] INKWELL: Mapped at user VA = 0x700000
[10977249789] [INFO] [inkwell] INKWELL: Writing test pattern via mapped memory...
[10978390599] [INFO] [inkwell] INKWELL: Pattern written
[10979126004] [INFO] [inkwell] INKWELL: Reading back via syscall...
[10981112901] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=1400
[10987118340] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[10988331057] [INFO] [inkwell] INKWELL: FAIL - bytespace_read error EIO
[10989115203] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=1
[11181086730] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=10a
[11183662017] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=126
[11185474047] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=0
[11187063525] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=0
[11188494042] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=0
[11189866776] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=0
[11191370685] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=0
[11472362451] [INFO] [sprout::pipelines] SPROUT: Spawned blossom (PID=9)
[11481083064] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=10)
[11489009499] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=11)
[11493433512] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0085e10
[11494178223] [INFO] [kernel::task::scheduler::spawn] Entering user mode: PC=0x200000 SP=0x800000
[11494902012] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562068589 RSP_BEFORE=18446744072367921248 RFLAGS_BEFORE=134 CR3_BEFORE=8142848 fs_base=0 gs_base=18446744071563563000
[11499469014] [INFO] [blossom::runtime] blossom: starting (disp_req_r=1, disp_resp_w=2, drv_req_w=4, drv_resp_r=7)
[11500630218] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0089ee0
[11501340807] [INFO] [kernel::task::scheduler::spawn] Entering user mode: PC=0x200000 SP=0x800000
[11502052881] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562068589 RSP_BEFORE=18446744072367937840 RFLAGS_BEFORE=134 CR3_BEFORE=27717632 fs_base=0 gs_base=18446744071563563000
[11505766701] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=5, drv_resp_w=6)
[11506896621] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb008e060
[11507627736] [INFO] [kernel::task::scheduler::spawn] Entering user mode: PC=0x200000 SP=0x800000
[11508326016] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562068589 RSP_BEFORE=18446744072367954608 RFLAGS_BEFORE=130 CR3_BEFORE=27815936 fs_base=0 gs_base=18446744071563563000
[11511756102] [INFO] [bloom] bloom: starting ui stub (disp_req_w=0, disp_resp_r=3)
[11531674902] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=10f
[11542462899] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=126
[11543668290] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=1
[11544555561] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=11c
[11546991687] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=14
[11547823518] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=500
[11550514932] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[11551341714] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=2d0
[11552202618] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=11d
[11553987489] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[11554661712] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=1400
[11556766815] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[11557539114] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=20
[11560180797] [INFO] [kernel::syscall::handlers::device] DEVICE: task 10 claimed device 121 (handle 0)
[11562788490] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0xffff800080000000
[11571472440] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[11574294534] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[11575452306] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=1
[11576418381] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[11584477080] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=12)
[11585516085] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[11586876345] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=8, r=9)
[11587912545] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=10, r=11)
[11589141498] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=12, r=13)
[11596005399] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=13)
[11603387268] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=14)
[11611403925] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=15)
[11619033063] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=16)
[11619796353] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[11620655178] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[11630314344] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[11631717372] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00973a0
[11632433901] [INFO] [kernel::task::scheduler::spawn] Entering user mode: PC=0x200000 SP=0x800000
[11633071296] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562068589 RSP_BEFORE=18446744072367992304 RFLAGS_BEFORE=134 CR3_BEFORE=28180480 fs_base=0 gs_base=18446744071563563000
[11636933055] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffee0 rip=0x200037 rflags=0x202
[11637713835] [INFO] [rtc_cmos] Starting... arg=8d
[11638724328] [INFO] [rtc_cmos] Serving device ID: ThingId(141)
[11641871769] [INFO] [rtc_cmos] RTC: 2026-01-13 03:08:39 = 1768273719 unix_secs
[11643022776] [INFO] [kernel::time] System clock anchored: unix_secs=1768273719, mono_ns=5821254681, offset=1768273713178745319ns
[11644189095] [INFO] [rtc_cmos] System clock anchored
[11644882722] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb009f880
[11645612418] [INFO] [kernel::task::scheduler::spawn] Entering user mode: PC=0x200000 SP=0x800000
[11646231102] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562068589 RSP_BEFORE=18446744072368026320 RFLAGS_BEFORE=130 CR3_BEFORE=28282880 fs_base=0 gs_base=18446744071563563000
[11649768966] [INFO] [ps2_kbd] ps2_kbd: online (handle=8)
[11657097375] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[11658348009] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[11659070148] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[11661719784] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00a3950
[11662481721] [INFO] [kernel::task::scheduler::spawn] Entering user mode: PC=0x200000 SP=0x800000
[11663139147] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562068589 RSP_BEFORE=18446744072368042912 RFLAGS_BEFORE=134 CR3_BEFORE=28377088 fs_base=0 gs_base=18446744071563563000
[11666691399] [INFO] [ps2_mouse] ps2_mouse: online (handle=10)
[11667485478] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[11668241970] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00a7a20
[11668935795] [INFO] [kernel::task::scheduler::spawn] Entering user mode: PC=0x200000 SP=0x800000
[11669705751] [INFO] [task.user_enter] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562068589 RSP_BEFORE=18446744072368059504 RFLAGS_BEFORE=130 CR3_BEFORE=28475392 fs_base=0 gs_base=18446744071563563000
[11673166791] [INFO] [bristle] bristle: online (kbd=9, mouse=11, evt=12)
[11673974334] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00abaf0
[11674685055] [INFO] [kernel::task::scheduler::spawn] Entering user mode: PC=0x200000 SP=0x800000
[11675294664] [INFO] [task.user_enter] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562068589 RSP_BEFORE=18446744072368076096 RFLAGS_BEFORE=130 CR3_BEFORE=28573696 fs_base=0 gs_base=18446744071563563000
[11678792631] [INFO] [echo] echo: online (handle=13)
[11679427254] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[11691593364] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[11692802517] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=135
[11705566158] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=146
[11706505701] [INFO] [bristle] bristle: registered in graph as svc.Input (id=326)
[11710750986] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[11714188860] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[11715258885] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=14d
[11718520374] [INFO] [user.print] stack_heap_torture::tests::stack: [torture] context_stress iteration=25
[11719300395] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[11723636199] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[11724729159] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=0
[11727301674] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[11728432452] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=0
[11729054007] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[11732266425] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[11735116404] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[11738004465] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[11740719969] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[11743439664] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[11746132332] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[11748933207] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=-1 value=0
[11751677223] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=126
[11754621318] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=500
[11757433941] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=2d0
[11760364209] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=1400
[11763177690] [TRACE] [kernel::syscall::handlers] ROOT_CALL_DEBUG: status=0 value=1
[12039357990] [INFO] [blossom::runtime] blossom: asset worker started tid=17
[12045943800] [INFO] [blossom::runtime] blossom: asset worker started tid=18
[12047823645] [INFO] [blossom::runtime] blossom: wallpaper job queued
[12048639702] [INFO] [blossom::runtime] blossom: cursor job queued
[12049445793] [INFO] [blossom::runtime] blossom: render loop started
[12259531020] [INFO] [ps2_mouse] ps2_mouse: sending enable command
T[12268010370] [INFO] [blossom::runtime] blossom: asset='clouds.bmp' len=3145782 header=[42 4D 36 00 30 00 00 00 00 00 36 00 00 00 28 00]
[12293016648] [INFO] [blossom::runtime] blossom: wallpaper decoded ok (256x256)
[12294239364] [INFO] [blossom::runtime] blossom: asset='Normal.cur' len=4286 header=[00 00 02 00 01 00 20 20 00 00 00 00 00 00 A8 10]
T[12300643047] [INFO] [blossom::runtime] blossom: wallpaper ready gen=1
[12302526093] [INFO] [blossom::runtime] blossom: cursor ready gen=2
[12530753499] [INFO] [user.print] stack_heap_torture::tests::stack: [torture] context_stress iteration=50
[12950697969] [INFO] [clock] CLOCK: unix=1768273719 mono_ns=6475174150
[12951760074] [INFO] [threads_demo] B: tick 1
[12952386480] [INFO] [threads_demo] A: tick 1
[12970902054] [INFO] [user.print] stack_heap_torture::tests::stack: [torture] context_stress iteration=75
[13453538703] [INFO] [user.print] stack_heap_torture::tests::stack: PASS: stack_context_stress
[13454820027] [INFO] [stack_heap_torture::tests::heap] [torture] Running heap_churn iterations=500
[13455668556] [INFO] [stack_heap_torture::tests::heap] [torture] heap_churn iteration=0
[13456412112] [INFO] [stack_heap_torture::tests::heap] [torture] heap_churn iteration=100
[13457043336] [INFO] [stack_heap_torture::tests::heap] [torture] heap_churn iteration=200
[13457657631] [INFO] [stack_heap_torture::tests::heap] [torture] heap_churn iteration=300
[13458271002] [INFO] [stack_heap_torture::tests::heap] [torture] heap_churn iteration=400
[13458919089] [INFO] [stack_heap_torture::tests::heap] PASS: heap_churn
[13459552029] [INFO] [stack_heap_torture::tests::heap] [torture] Running heap_realloc
[13479451194] [INFO] [stack_heap_torture::tests::heap] PASS: heap_realloc
[13481060934] [INFO] [stack_heap_torture::tests::heap] [torture] Running heap_fuzz seed=123
[13482377205] [INFO] [stack_heap_torture::tests::heap] [torture] heap_fuzz iteration=0 live_allocs=0
[13485036345] [INFO] [stack_heap_torture::tests::heap] [torture] heap_fuzz iteration=250 live_allocs=6
[13486617078] [INFO] [stack_heap_torture::tests::heap] [torture] heap_fuzz iteration=500 live_allocs=22
[13488132570] [INFO] [stack_heap_torture::tests::heap] [torture] heap_fuzz iteration=750 live_allocs=28
[13489778544] [INFO] [stack_heap_torture::tests::heap] PASS: heap_fuzz seed=123
[13490774517] [INFO] [stack_heap_torture] [torture] All tests completed successfully
[13673417472] [INFO] [ps2_mouse] ps2_mouse: init done
[13674290454] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[13675395327] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[13675982991] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[13907063643] [INFO] [sched.activity] Scheduler Activity Rollup yields=3718 pops=3719 pushes=3718 idle_picks=0 runq_len=16
[14979680727] [INFO] [clock] CLOCK: unix=1768273720 mono_ns=7489687458
[14980609677] [INFO] [threads_demo] B: tick 2
[14981225028] [INFO] [threads_demo] A: tick 2
[17113605036] [INFO] [clock] CLOCK: unix=1768273721 mono_ns=8556651510
[17114536626] [INFO] [threads_demo] B: tick 3
[17115119967] [INFO] [threads_demo] A: tick 3
[17928695763] [INFO] [sched.activity] Scheduler Activity Rollup yields=4348 pops=4348 pushes=4348 idle_picks=0 runq_len=16

```
</details>
