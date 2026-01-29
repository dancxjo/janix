# ✅ Scenario: Designer checks time while browsing fonts

> Last run: 2026-01-29 21:07:41

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 16666ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | And I wait for the system to reach ready state | ⏭️ | 1421ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[32934254015] [CONTRACT] [kernel] thing-os kernel starting...
[32957435771] [INFO] [kernel::memory] Memory map has 64 entries
[32966942544] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[32974515786] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[32981339894] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[32988387425] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[32995381121] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[33002290290] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[33008961556] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[33015476972] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[33022723620] [INFO] [kernel::memory]   [8] 0x1780000 - 0x7829c000 (Usable)
[33029718073] [INFO] [kernel::memory]   [9] 0x7829c000 - 0x78300000 (Reserved)
[33037221879] [INFO] [kernel::memory]   [10] 0x78300000 - 0x78301000 (Other)
[33044110762] [INFO] [kernel::memory]   [11] 0x78301000 - 0x78302000 (Reserved)
[33051415245] [INFO] [kernel::memory]   [12] 0x78302000 - 0x78303000 (Other)
[33058539483] [INFO] [kernel::memory]   [13] 0x78303000 - 0x78304000 (Reserved)
[33065967835] [INFO] [kernel::memory]   [14] 0x78304000 - 0x78305000 (Other)
[33072950455] [INFO] [kernel::memory]   [15] 0x78305000 - 0x78306000 (Reserved)
[33080181196] [INFO] [kernel::memory]   [16] 0x78306000 - 0x78307000 (Other)
[33087722292] [INFO] [kernel::memory]   [17] 0x78307000 - 0x78308000 (Reserved)
[33095557037] [INFO] [kernel::memory]   [18] 0x78308000 - 0x78309000 (Other)
[33102588285] [INFO] [kernel::memory]   [19] 0x78309000 - 0x7830a000 (Reserved)
[33109831238] [INFO] [kernel::memory]   [20] 0x7830a000 - 0x7830b000 (Other)
[33116925828] [INFO] [kernel::memory]   [21] 0x7830b000 - 0x7830c000 (Reserved)
[33124235394] [INFO] [kernel::memory]   [22] 0x7830c000 - 0x7830d000 (Other)
[33131167935] [INFO] [kernel::memory]   [23] 0x7830d000 - 0x7830e000 (Reserved)
[33138325288] [INFO] [kernel::memory]   [24] 0x7830e000 - 0x7830f000 (Other)
[33145347684] [INFO] [kernel::memory]   [25] 0x7830f000 - 0x78310000 (Reserved)
[33152551290] [INFO] [kernel::memory]   [26] 0x78310000 - 0x78311000 (Other)
[33160036515] [INFO] [kernel::memory]   [27] 0x78311000 - 0x78312000 (Reserved)
[33194850071] [INFO] [kernel::memory]   [28] 0x78312000 - 0x78313000 (Other)
[33201758017] [INFO] [kernel::memory]   [29] 0x78313000 - 0x78314000 (Reserved)
[33208930998] [INFO] [kernel::memory]   [30] 0x78314000 - 0x78315000 (Other)
[33215925023] [INFO] [kernel::memory]   [31] 0x78315000 - 0x78316000 (Reserved)
[33223244191] [INFO] [kernel::memory]   [32] 0x78316000 - 0x78317000 (Other)
[33230127810] [INFO] [kernel::memory]   [33] 0x78317000 - 0x78318000 (Reserved)
[33237172340] [INFO] [kernel::memory]   [34] 0x78318000 - 0x78319000 (Other)
[33244143582] [INFO] [kernel::memory]   [35] 0x78319000 - 0x7831a000 (Reserved)
[33251335754] [INFO] [kernel::memory]   [36] 0x7831a000 - 0x7831b000 (Other)
[33258331129] [INFO] [kernel::memory]   [37] 0x7831b000 - 0x7831c000 (Reserved)
[33265400390] [INFO] [kernel::memory]   [38] 0x7831c000 - 0x7831d000 (Other)
[33272815711] [INFO] [kernel::memory]   [39] 0x7831d000 - 0x7831e000 (Reserved)
[33280385173] [INFO] [kernel::memory]   [40] 0x7831e000 - 0x7831f000 (Other)
[33287809665] [INFO] [kernel::memory]   [41] 0x7831f000 - 0x78320000 (Reserved)
[33295110082] [INFO] [kernel::memory]   [42] 0x78320000 - 0x78321000 (Other)
[33301918786] [INFO] [kernel::memory]   [43] 0x78321000 - 0x78322000 (Reserved)
[33313154267] [INFO] [kernel::memory]   [44] 0x78322000 - 0x78323000 (Other)
[33320204089] [INFO] [kernel::memory]   [45] 0x78323000 - 0x78324000 (Reserved)
[33327841456] [INFO] [kernel::memory]   [46] 0x78324000 - 0x78325000 (Other)
[33334920955] [INFO] [kernel::memory]   [47] 0x78325000 - 0x78326000 (Reserved)
[33342445524] [INFO] [kernel::memory]   [48] 0x78326000 - 0x78327000 (Other)
[33349639270] [INFO] [kernel::memory]   [49] 0x78327000 - 0x78328000 (Reserved)
[33356697990] [INFO] [kernel::memory]   [50] 0x78328000 - 0x78329000 (Other)
[33363716944] [INFO] [kernel::memory]   [51] 0x78329000 - 0x7832a000 (Reserved)
[33371035558] [INFO] [kernel::memory]   [52] 0x7832a000 - 0x7832b000 (Other)
[33378130096] [INFO] [kernel::memory]   [53] 0x7832b000 - 0x7832c000 (Reserved)
[33385597933] [INFO] [kernel::memory]   [54] 0x7832c000 - 0x7832d000 (Other)
[33392626222] [INFO] [kernel::memory]   [55] 0x7832d000 - 0x7832e000 (Reserved)
[33399919844] [INFO] [kernel::memory]   [56] 0x7832e000 - 0x7832f000 (Other)
[33406940680] [INFO] [kernel::memory]   [57] 0x7832f000 - 0x78330000 (Reserved)
[33414498694] [INFO] [kernel::memory]   [58] 0x78330000 - 0x78331000 (Other)
[33421317885] [INFO] [kernel::memory]   [59] 0x78331000 - 0x78332000 (Reserved)
[33428561591] [INFO] [kernel::memory]   [60] 0x78332000 - 0x78333000 (Other)
[33435587426] [INFO] [kernel::memory]   [61] 0x78333000 - 0x78334000 (Reserved)
[33442881299] [INFO] [kernel::memory]   [62] 0x78334000 - 0x78335000 (Other)
[33449798872] [INFO] [kernel::memory]   [63] 0x78335000 - 0x78336000 (Reserved)
[33457328573] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[33760669601] [CONTRACT] [kernel::memory] Frame allocator initialized with 488116 free frames
[33777895930] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[33795786818] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[33803719536] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[33809977067] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[33824113724] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[33830266458] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[33840487201] [INFO] [bran::arch] IOAPIC: Registers initialized
[33848050864] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[33856423045] [INFO] [bran::arch] IOAPIC: All pins masked
[33863647506] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[33870155407] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[33876420497] [INFO] [bran::arch] IOAPIC: Init complete
[33882210578] [CONTRACT] [kernel] Initializing global allocator...
[35854245841] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[35863993000] [CONTRACT] [kernel] Initializing SIMD...
[35871067837] [CONTRACT] [kernel] Initializing tasking...
[35888252105] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[35897433716] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[35906001996] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[35920325720] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[35927090786] [INFO] [kernel::task::scheduler]   Initializing boot task...
[36061409297] [INFO] [kernel::task::scheduler]   Creating boot task...
[36074120207] [INFO] [kernel::task::scheduler]   Creating idle task...
[36086553143] [INFO] [kernel::task::scheduler]   Boot task initialized
[36093415653] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[36101259380] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[36115001810] [INFO] [kernel::root] Spawning Root service...
[36131002425] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[36151957043] [INFO] [kernel::root::service] ROOT: started once
[40421843176] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[40430388492] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[40523973598] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[40564782237] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[40619711576] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[40685121999] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[40702146919] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[40769939894] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller

```
</details>
