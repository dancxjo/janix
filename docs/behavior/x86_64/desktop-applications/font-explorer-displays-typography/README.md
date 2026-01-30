# ✅ Scenario: Font Explorer displays typography

> Last run: 2026-01-29 22:19:33

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 10948ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | And I wait for the system to reach ready state | ⏭️ | 206ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[20888387522] [CONTRACT] [kernel] thing-os kernel starting...
[20916776119] [INFO] [kernel::memory] Memory map has 64 entries
[20929331318] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[20937306977] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[20944650019] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[20952008350] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[20960655358] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[20969174678] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[20976819563] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[20984222832] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[20992949599] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78285000 (Usable)
[21001759082] [INFO] [kernel::memory]   [9] 0x78285000 - 0x782e9000 (Reserved)
[21009805369] [INFO] [kernel::memory]   [10] 0x782e9000 - 0x782ea000 (Other)
[21018532670] [INFO] [kernel::memory]   [11] 0x782ea000 - 0x782eb000 (Reserved)
[21027319746] [INFO] [kernel::memory]   [12] 0x782eb000 - 0x782ec000 (Other)
[21037042194] [INFO] [kernel::memory]   [13] 0x782ec000 - 0x782ed000 (Reserved)
[21046161505] [INFO] [kernel::memory]   [14] 0x782ed000 - 0x782ee000 (Other)
[21054011179] [INFO] [kernel::memory]   [15] 0x782ee000 - 0x782ef000 (Reserved)
[21062305670] [INFO] [kernel::memory]   [16] 0x782ef000 - 0x782f0000 (Other)
[21070945564] [INFO] [kernel::memory]   [17] 0x782f0000 - 0x782f1000 (Reserved)
[21113222534] [INFO] [kernel::memory]   [18] 0x782f1000 - 0x782f2000 (Other)
[21122496209] [INFO] [kernel::memory]   [19] 0x782f2000 - 0x782f3000 (Reserved)
[21132250714] [INFO] [kernel::memory]   [20] 0x782f3000 - 0x782f4000 (Other)
[21139839836] [INFO] [kernel::memory]   [21] 0x782f4000 - 0x782f5000 (Reserved)
[21149255021] [INFO] [kernel::memory]   [22] 0x782f5000 - 0x782f6000 (Other)
[21157712443] [INFO] [kernel::memory]   [23] 0x782f6000 - 0x782f7000 (Reserved)
[21166610357] [INFO] [kernel::memory]   [24] 0x782f7000 - 0x782f8000 (Other)
[21174263973] [INFO] [kernel::memory]   [25] 0x782f8000 - 0x782f9000 (Reserved)
[21183929571] [INFO] [kernel::memory]   [26] 0x782f9000 - 0x782fa000 (Other)
[21192678441] [INFO] [kernel::memory]   [27] 0x782fa000 - 0x782fb000 (Reserved)
[21201327224] [INFO] [kernel::memory]   [28] 0x782fb000 - 0x782fc000 (Other)
[21209025549] [INFO] [kernel::memory]   [29] 0x782fc000 - 0x782fd000 (Reserved)
[21217750580] [INFO] [kernel::memory]   [30] 0x782fd000 - 0x782fe000 (Other)
[21224858930] [INFO] [kernel::memory]   [31] 0x782fe000 - 0x782ff000 (Reserved)
[21230745561] [INFO] [kernel::memory]   [32] 0x782ff000 - 0x78300000 (Other)
[21237101443] [INFO] [kernel::memory]   [33] 0x78300000 - 0x78301000 (Reserved)
[21243125751] [INFO] [kernel::memory]   [34] 0x78301000 - 0x78302000 (Other)
[21248856920] [INFO] [kernel::memory]   [35] 0x78302000 - 0x78303000 (Reserved)
[21255467005] [INFO] [kernel::memory]   [36] 0x78303000 - 0x78304000 (Other)
[21261400482] [INFO] [kernel::memory]   [37] 0x78304000 - 0x78305000 (Reserved)
[21270334538] [INFO] [kernel::memory]   [38] 0x78305000 - 0x78306000 (Other)
[21279014642] [INFO] [kernel::memory]   [39] 0x78306000 - 0x78307000 (Reserved)
[21287938317] [INFO] [kernel::memory]   [40] 0x78307000 - 0x78308000 (Other)
[21296343507] [INFO] [kernel::memory]   [41] 0x78308000 - 0x78309000 (Reserved)
[21304635738] [INFO] [kernel::memory]   [42] 0x78309000 - 0x7830a000 (Other)
[21314023602] [INFO] [kernel::memory]   [43] 0x7830a000 - 0x7830b000 (Reserved)
[21323588634] [INFO] [kernel::memory]   [44] 0x7830b000 - 0x7830c000 (Other)
[21334976838] [INFO] [kernel::memory]   [45] 0x7830c000 - 0x7830d000 (Reserved)
[21341653566] [INFO] [kernel::memory]   [46] 0x7830d000 - 0x7830e000 (Other)
[21347890620] [INFO] [kernel::memory]   [47] 0x7830e000 - 0x7830f000 (Reserved)
[21354570452] [INFO] [kernel::memory]   [48] 0x7830f000 - 0x78310000 (Other)
[21361568570] [INFO] [kernel::memory]   [49] 0x78310000 - 0x78311000 (Reserved)
[21368202429] [INFO] [kernel::memory]   [50] 0x78311000 - 0x78312000 (Other)
[21374307088] [INFO] [kernel::memory]   [51] 0x78312000 - 0x78313000 (Reserved)
[21380052276] [INFO] [kernel::memory]   [52] 0x78313000 - 0x78314000 (Other)
[21385875018] [INFO] [kernel::memory]   [53] 0x78314000 - 0x78315000 (Reserved)
[21391731877] [INFO] [kernel::memory]   [54] 0x78315000 - 0x78316000 (Other)
[21403213283] [INFO] [kernel::memory]   [55] 0x78316000 - 0x78317000 (Reserved)
[21412522051] [INFO] [kernel::memory]   [56] 0x78317000 - 0x78318000 (Other)
[21421432055] [INFO] [kernel::memory]   [57] 0x78318000 - 0x78319000 (Reserved)
[21431377071] [INFO] [kernel::memory]   [58] 0x78319000 - 0x7831a000 (Other)
[21439513912] [INFO] [kernel::memory]   [59] 0x7831a000 - 0x7831b000 (Reserved)
[21447563051] [INFO] [kernel::memory]   [60] 0x7831b000 - 0x7831c000 (Other)
[21455908665] [INFO] [kernel::memory]   [61] 0x7831c000 - 0x7831d000 (Reserved)
[21466750475] [INFO] [kernel::memory]   [62] 0x7831d000 - 0x7831e000 (Other)
[21474386116] [INFO] [kernel::memory]   [63] 0x7831e000 - 0x7831f000 (Reserved)
[21483098524] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[21895406701] [CONTRACT] [kernel::memory] Frame allocator initialized with 488093 free frames
[21916662452] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[21939147919] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[21948021720] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[21956504561] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[21975883790] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[21985069981] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[21997262858] [INFO] [bran::arch] IOAPIC: Registers initialized
[22005882421] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[22017892924] [INFO] [bran::arch] IOAPIC: All pins masked
[22026065556] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[22033138192] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[22041247477] [INFO] [bran::arch] IOAPIC: Init complete
[22048851911] [CONTRACT] [kernel] Initializing global allocator...
[22833347466] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[22845700693] [CONTRACT] [kernel] Initializing SIMD...
[22856217490] [CONTRACT] [kernel] Initializing tasking...
[22882399918] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[22895521000] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[22906911512] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[22926153441] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[22935632229] [INFO] [kernel::task::scheduler]   Initializing boot task...
[22974428105] [INFO] [kernel::task::scheduler]   Creating boot task...
[22993803958] [INFO] [kernel::task::scheduler]   Creating idle task...
[23013323408] [INFO] [kernel::task::scheduler]   Boot task initialized
[23023423980] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[23035344975] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[23057725795] [INFO] [kernel::root] Spawning Root service...
[23083377652] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[23111969436] [INFO] [kernel::root::service] ROOT: started once
[28627307302] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[28637219531] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[28731552083] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[28787022296] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[28865248062] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[28960018463] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[28977128540] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[29075336560] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller

```
</details>
