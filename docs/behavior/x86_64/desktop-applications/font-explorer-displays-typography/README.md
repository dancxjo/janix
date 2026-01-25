# ✅ Scenario: Font Explorer displays typography

> Last run: 2026-01-25 21:16:03

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 9715ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | And I wait for the system to reach ready state | ⏭️ | 601ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[20118067204] [CONTRACT] [kernel] thing-os kernel starting...
[20132329536] [INFO] [kernel::memory] Memory map has 64 entries
[20135109651] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[20135958956] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[20136305469] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[20136694771] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[20137101523] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[20137430670] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[20137761007] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[20138219643] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[20138648596] [INFO] [kernel::memory]   [8] 0x1780000 - 0x7852e000 (Usable)
[20139108779] [INFO] [kernel::memory]   [9] 0x7852e000 - 0x7858f000 (Reserved)
[20139478644] [INFO] [kernel::memory]   [10] 0x7858f000 - 0x78590000 (Other)
[20139829466] [INFO] [kernel::memory]   [11] 0x78590000 - 0x78591000 (Reserved)
[20140188034] [INFO] [kernel::memory]   [12] 0x78591000 - 0x78592000 (Other)
[20140533395] [INFO] [kernel::memory]   [13] 0x78592000 - 0x78593000 (Reserved)
[20140889136] [INFO] [kernel::memory]   [14] 0x78593000 - 0x78594000 (Other)
[20141242130] [INFO] [kernel::memory]   [15] 0x78594000 - 0x78595000 (Reserved)
[20141681349] [INFO] [kernel::memory]   [16] 0x78595000 - 0x78596000 (Other)
[20142028076] [INFO] [kernel::memory]   [17] 0x78596000 - 0x78597000 (Reserved)
[20142389052] [INFO] [kernel::memory]   [18] 0x78597000 - 0x78598000 (Other)
[20142735902] [INFO] [kernel::memory]   [19] 0x78598000 - 0x78599000 (Reserved)
[20143093069] [INFO] [kernel::memory]   [20] 0x78599000 - 0x7859a000 (Other)
[20143449300] [INFO] [kernel::memory]   [21] 0x7859a000 - 0x7859b000 (Reserved)
[20143884264] [INFO] [kernel::memory]   [22] 0x7859b000 - 0x7859c000 (Other)
[20144234296] [INFO] [kernel::memory]   [23] 0x7859c000 - 0x7859d000 (Reserved)
[20144594756] [INFO] [kernel::memory]   [24] 0x7859d000 - 0x7859e000 (Other)
[20144943122] [INFO] [kernel::memory]   [25] 0x7859e000 - 0x7859f000 (Reserved)
[20145299177] [INFO] [kernel::memory]   [26] 0x7859f000 - 0x785a0000 (Other)
[20145889045] [INFO] [kernel::memory]   [27] 0x785a0000 - 0x785a1000 (Reserved)
[20146324005] [INFO] [kernel::memory]   [28] 0x785a1000 - 0x785a2000 (Other)
[20146681166] [INFO] [kernel::memory]   [29] 0x785a2000 - 0x785a3000 (Reserved)
[20147041871] [INFO] [kernel::memory]   [30] 0x785a3000 - 0x785a4000 (Other)
[20147532418] [INFO] [kernel::memory]   [31] 0x785a4000 - 0x785a5000 (Reserved)
[20147897737] [INFO] [kernel::memory]   [32] 0x785a5000 - 0x785a6000 (Other)
[20148315068] [INFO] [kernel::memory]   [33] 0x785a6000 - 0x785a7000 (Reserved)
[20148689831] [INFO] [kernel::memory]   [34] 0x785a7000 - 0x785a8000 (Other)
[20149041050] [INFO] [kernel::memory]   [35] 0x785a8000 - 0x785a9000 (Reserved)
[20149403850] [INFO] [kernel::memory]   [36] 0x785a9000 - 0x785aa000 (Other)
[20149752255] [INFO] [kernel::memory]   [37] 0x785aa000 - 0x785ab000 (Reserved)
[20150115634] [INFO] [kernel::memory]   [38] 0x785ab000 - 0x785ac000 (Other)
[20150480115] [INFO] [kernel::memory]   [39] 0x785ac000 - 0x785ad000 (Reserved)
[20150902679] [INFO] [kernel::memory]   [40] 0x785ad000 - 0x785ae000 (Other)
[20151260466] [INFO] [kernel::memory]   [41] 0x785ae000 - 0x785af000 (Reserved)
[20151625891] [INFO] [kernel::memory]   [42] 0x785af000 - 0x785b0000 (Other)
[20151974499] [INFO] [kernel::memory]   [43] 0x785b0000 - 0x785b1000 (Reserved)
[20152338395] [INFO] [kernel::memory]   [44] 0x785b1000 - 0x785b2000 (Other)
[20152698606] [INFO] [kernel::memory]   [45] 0x785b2000 - 0x785b3000 (Reserved)
[20153118099] [INFO] [kernel::memory]   [46] 0x785b3000 - 0x785b4000 (Other)
[20153456953] [INFO] [kernel::memory]   [47] 0x785b4000 - 0x785b5000 (Reserved)
[20153802260] [INFO] [kernel::memory]   [48] 0x785b5000 - 0x785b6000 (Other)
[20154135873] [INFO] [kernel::memory]   [49] 0x785b6000 - 0x785b7000 (Reserved)
[20154480186] [INFO] [kernel::memory]   [50] 0x785b7000 - 0x785b8000 (Other)
[20154827952] [INFO] [kernel::memory]   [51] 0x785b8000 - 0x785b9000 (Reserved)
[20155246433] [INFO] [kernel::memory]   [52] 0x785b9000 - 0x785ba000 (Other)
[20155581758] [INFO] [kernel::memory]   [53] 0x785ba000 - 0x785bb000 (Reserved)
[20155922992] [INFO] [kernel::memory]   [54] 0x785bb000 - 0x785bc000 (Other)
[20212094853] [INFO] [kernel::memory]   [55] 0x785bc000 - 0x785bd000 (Reserved)
[20212572381] [INFO] [kernel::memory]   [56] 0x785bd000 - 0x785be000 (Other)
[20213019513] [INFO] [kernel::memory]   [57] 0x785be000 - 0x785bf000 (Reserved)
[20213388152] [INFO] [kernel::memory]   [58] 0x785bf000 - 0x785c0000 (Other)
[20213754297] [INFO] [kernel::memory]   [59] 0x785c0000 - 0x785c1000 (Reserved)
[20214140010] [INFO] [kernel::memory]   [60] 0x785c1000 - 0x785c2000 (Other)
[20214488555] [INFO] [kernel::memory]   [61] 0x785c2000 - 0x785c3000 (Reserved)
[20215006629] [INFO] [kernel::memory]   [62] 0x785c3000 - 0x785c4000 (Other)
[20215372740] [INFO] [kernel::memory]   [63] 0x785c4000 - 0x785c5000 (Reserved)
[20216070466] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[20547499784] [CONTRACT] [kernel::memory] Frame allocator initialized with 488774 free frames
[20556225863] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[20563544621] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[20565841454] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[20566649189] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[20583738618] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[20584352028] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[20591462787] [INFO] [bran::arch] IOAPIC: Registers initialized
[20595207364] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[20599278722] [INFO] [bran::arch] IOAPIC: All pins masked
[20603174318] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[20604970485] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[20605488908] [INFO] [bran::arch] IOAPIC: Init complete
[20606241922] [CONTRACT] [kernel] Initializing global allocator...
[21076078096] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[21076939413] [CONTRACT] [kernel] Initializing SIMD...
[21078710002] [CONTRACT] [kernel] Initializing tasking...
[21084880326] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[21087075148] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[21087765044] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[21095277775] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[21095890188] [INFO] [kernel::task::scheduler]   Initializing boot task...
[21097076713] [INFO] [kernel::task::scheduler]   Creating boot task...
[21102823208] [INFO] [kernel::task::scheduler]   Creating idle task...
[21109156877] [INFO] [kernel::task::scheduler]   Boot task initialized
[21109738248] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[21110638455] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[21117803352] [INFO] [kernel::root] Spawning Root service...
[21133281516] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[21145324912] [INFO] [kernel::root::service] ROOT: started once
[22810521418] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[22811641440] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[22872518763] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[22903935674] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[22943348207] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[22989329722] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[22991613276] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0

```
</details>
