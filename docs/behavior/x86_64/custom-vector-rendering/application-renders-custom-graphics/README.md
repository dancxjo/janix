# ✅ Scenario: Application renders custom graphics

> Last run: 2026-01-29 22:19:33

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 10682ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | And I wait for 5 seconds | ⏭️ | 206ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[21167906521] [CONTRACT] [kernel] thing-os kernel starting...
[21188990086] [INFO] [kernel::memory] Memory map has 64 entries
[21197102260] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[21203070891] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[21208619391] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[21214142124] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[21219424482] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[21224935832] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[21230260308] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[21235657480] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[21241389758] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78285000 (Usable)
[21246968390] [INFO] [kernel::memory]   [9] 0x78285000 - 0x782e9000 (Reserved)
[21279713688] [INFO] [kernel::memory]   [10] 0x782e9000 - 0x782ea000 (Other)
[21285293736] [INFO] [kernel::memory]   [11] 0x782ea000 - 0x782eb000 (Reserved)
[21291322500] [INFO] [kernel::memory]   [12] 0x782eb000 - 0x782ec000 (Other)
[21297004689] [INFO] [kernel::memory]   [13] 0x782ec000 - 0x782ed000 (Reserved)
[21302807043] [INFO] [kernel::memory]   [14] 0x782ed000 - 0x782ee000 (Other)
[21308420936] [INFO] [kernel::memory]   [15] 0x782ee000 - 0x782ef000 (Reserved)
[21314380990] [INFO] [kernel::memory]   [16] 0x782ef000 - 0x782f0000 (Other)
[21320099968] [INFO] [kernel::memory]   [17] 0x782f0000 - 0x782f1000 (Reserved)
[21326005975] [INFO] [kernel::memory]   [18] 0x782f1000 - 0x782f2000 (Other)
[21331566674] [INFO] [kernel::memory]   [19] 0x782f2000 - 0x782f3000 (Reserved)
[21337358043] [INFO] [kernel::memory]   [20] 0x782f3000 - 0x782f4000 (Other)
[21343021421] [INFO] [kernel::memory]   [21] 0x782f4000 - 0x782f5000 (Reserved)
[21348952906] [INFO] [kernel::memory]   [22] 0x782f5000 - 0x782f6000 (Other)
[21354647955] [INFO] [kernel::memory]   [23] 0x782f6000 - 0x782f7000 (Reserved)
[21360431523] [INFO] [kernel::memory]   [24] 0x782f7000 - 0x782f8000 (Other)
[21365987403] [INFO] [kernel::memory]   [25] 0x782f8000 - 0x782f9000 (Reserved)
[21371938074] [INFO] [kernel::memory]   [26] 0x782f9000 - 0x782fa000 (Other)
[21377563003] [INFO] [kernel::memory]   [27] 0x782fa000 - 0x782fb000 (Reserved)
[21383372337] [INFO] [kernel::memory]   [28] 0x782fb000 - 0x782fc000 (Other)
[21389165683] [INFO] [kernel::memory]   [29] 0x782fc000 - 0x782fd000 (Reserved)
[21394951156] [INFO] [kernel::memory]   [30] 0x782fd000 - 0x782fe000 (Other)
[21400610719] [INFO] [kernel::memory]   [31] 0x782fe000 - 0x782ff000 (Reserved)
[21406474142] [INFO] [kernel::memory]   [32] 0x782ff000 - 0x78300000 (Other)
[21412239534] [INFO] [kernel::memory]   [33] 0x78300000 - 0x78301000 (Reserved)
[21418134456] [INFO] [kernel::memory]   [34] 0x78301000 - 0x78302000 (Other)
[21423725098] [INFO] [kernel::memory]   [35] 0x78302000 - 0x78303000 (Reserved)
[21429523297] [INFO] [kernel::memory]   [36] 0x78303000 - 0x78304000 (Other)
[21435169053] [INFO] [kernel::memory]   [37] 0x78304000 - 0x78305000 (Reserved)
[21440907799] [INFO] [kernel::memory]   [38] 0x78305000 - 0x78306000 (Other)
[21446563910] [INFO] [kernel::memory]   [39] 0x78306000 - 0x78307000 (Reserved)
[21452375569] [INFO] [kernel::memory]   [40] 0x78307000 - 0x78308000 (Other)
[21457884910] [INFO] [kernel::memory]   [41] 0x78308000 - 0x78309000 (Reserved)
[21463703224] [INFO] [kernel::memory]   [42] 0x78309000 - 0x7830a000 (Other)
[21469274073] [INFO] [kernel::memory]   [43] 0x7830a000 - 0x7830b000 (Reserved)
[21475257130] [INFO] [kernel::memory]   [44] 0x7830b000 - 0x7830c000 (Other)
[21480914895] [INFO] [kernel::memory]   [45] 0x7830c000 - 0x7830d000 (Reserved)
[21486729249] [INFO] [kernel::memory]   [46] 0x7830d000 - 0x7830e000 (Other)
[21492541533] [INFO] [kernel::memory]   [47] 0x7830e000 - 0x7830f000 (Reserved)
[21498432395] [INFO] [kernel::memory]   [48] 0x7830f000 - 0x78310000 (Other)
[21503993864] [INFO] [kernel::memory]   [49] 0x78310000 - 0x78311000 (Reserved)
[21509762136] [INFO] [kernel::memory]   [50] 0x78311000 - 0x78312000 (Other)
[21515256888] [INFO] [kernel::memory]   [51] 0x78312000 - 0x78313000 (Reserved)
[21521044006] [INFO] [kernel::memory]   [52] 0x78313000 - 0x78314000 (Other)
[21526653806] [INFO] [kernel::memory]   [53] 0x78314000 - 0x78315000 (Reserved)
[21532420111] [INFO] [kernel::memory]   [54] 0x78315000 - 0x78316000 (Other)
[21538289647] [INFO] [kernel::memory]   [55] 0x78316000 - 0x78317000 (Reserved)
[21544015509] [INFO] [kernel::memory]   [56] 0x78317000 - 0x78318000 (Other)
[21549648982] [INFO] [kernel::memory]   [57] 0x78318000 - 0x78319000 (Reserved)
[21555525059] [INFO] [kernel::memory]   [58] 0x78319000 - 0x7831a000 (Other)
[21561101016] [INFO] [kernel::memory]   [59] 0x7831a000 - 0x7831b000 (Reserved)
[21566963877] [INFO] [kernel::memory]   [60] 0x7831b000 - 0x7831c000 (Other)
[21572556733] [INFO] [kernel::memory]   [61] 0x7831c000 - 0x7831d000 (Reserved)
[21578324665] [INFO] [kernel::memory]   [62] 0x7831d000 - 0x7831e000 (Other)
[21583926539] [INFO] [kernel::memory]   [63] 0x7831e000 - 0x7831f000 (Reserved)
[21590272147] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[21878382384] [CONTRACT] [kernel::memory] Frame allocator initialized with 488093 free frames
[21893324378] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[21909790097] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[21916502280] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[21922113280] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[21935197172] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[21940336593] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[21949396274] [INFO] [bran::arch] IOAPIC: Registers initialized
[21955566226] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[21962814747] [INFO] [bran::arch] IOAPIC: All pins masked
[21969281276] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[21974812082] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[21980491353] [INFO] [bran::arch] IOAPIC: Init complete
[21985527931] [CONTRACT] [kernel] Initializing global allocator...
[22615034746] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[22623315604] [CONTRACT] [kernel] Initializing SIMD...
[22629250212] [CONTRACT] [kernel] Initializing tasking...
[22646136845] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[22654255446] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[22661391722] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[22674504550] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[22680046813] [INFO] [kernel::task::scheduler]   Initializing boot task...
[22704529018] [INFO] [kernel::task::scheduler]   Creating boot task...
[22719248105] [INFO] [kernel::task::scheduler]   Creating idle task...
[22734670424] [INFO] [kernel::task::scheduler]   Boot task initialized
[22740784353] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[22751014697] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[22767390232] [INFO] [kernel::root] Spawning Root service...
[22784182193] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[22808904051] [INFO] [kernel::root::service] ROOT: started once
[26941227474] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[26951152311] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[27019013981] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[27065726893] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[27122021804] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[27194422818] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[27214380283] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[27288573105] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[27337773567] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[27361503510] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[27374860920] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=656, idx=3) BAR5=0x810c4000

```
</details>
