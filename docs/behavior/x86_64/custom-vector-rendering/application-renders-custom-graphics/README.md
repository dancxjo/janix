# ✅ Scenario: Application renders custom graphics

> Last run: 2026-01-28 21:25:48

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 8864ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | And I wait for 5 seconds | ⏭️ | 719ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> - [💾](./02/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[18320797682] [CONTRACT] [kernel] thing-os kernel starting...
[18334997536] [INFO] [kernel::memory] Memory map has 64 entries
[18337714945] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[18338598993] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[18338930916] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[18339296937] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[18339631353] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[18339957858] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[18340284220] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[18340607011] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[18341042075] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78316000 (Usable)
[18341389288] [INFO] [kernel::memory]   [9] 0x78316000 - 0x78379000 (Reserved)
[18341742676] [INFO] [kernel::memory]   [10] 0x78379000 - 0x7837a000 (Other)
[18342101995] [INFO] [kernel::memory]   [11] 0x7837a000 - 0x7837b000 (Reserved)
[18342456052] [INFO] [kernel::memory]   [12] 0x7837b000 - 0x7837c000 (Other)
[18342800250] [INFO] [kernel::memory]   [13] 0x7837c000 - 0x7837d000 (Reserved)
[18343226651] [INFO] [kernel::memory]   [14] 0x7837d000 - 0x7837e000 (Other)
[18343688035] [INFO] [kernel::memory]   [15] 0x7837e000 - 0x7837f000 (Reserved)
[18344045707] [INFO] [kernel::memory]   [16] 0x7837f000 - 0x78380000 (Other)
[18344413352] [INFO] [kernel::memory]   [17] 0x78380000 - 0x78381000 (Reserved)
[18344767287] [INFO] [kernel::memory]   [18] 0x78381000 - 0x78382000 (Other)
[18345111643] [INFO] [kernel::memory]   [19] 0x78382000 - 0x78383000 (Reserved)
[18345545164] [INFO] [kernel::memory]   [20] 0x78383000 - 0x78384000 (Other)
[18345892697] [INFO] [kernel::memory]   [21] 0x78384000 - 0x78385000 (Reserved)
[18346249532] [INFO] [kernel::memory]   [22] 0x78385000 - 0x78386000 (Other)
[18346620424] [INFO] [kernel::memory]   [23] 0x78386000 - 0x78387000 (Reserved)
[18346975240] [INFO] [kernel::memory]   [24] 0x78387000 - 0x78388000 (Other)
[18347319902] [INFO] [kernel::memory]   [25] 0x78388000 - 0x78389000 (Reserved)
[18347740542] [INFO] [kernel::memory]   [26] 0x78389000 - 0x7838a000 (Other)
[18348086727] [INFO] [kernel::memory]   [27] 0x7838a000 - 0x7838b000 (Reserved)
[18348441874] [INFO] [kernel::memory]   [28] 0x7838b000 - 0x7838c000 (Other)
[18348783542] [INFO] [kernel::memory]   [29] 0x7838c000 - 0x7838d000 (Reserved)
[18349156354] [INFO] [kernel::memory]   [30] 0x7838d000 - 0x7838e000 (Other)
[18349498854] [INFO] [kernel::memory]   [31] 0x7838e000 - 0x7838f000 (Reserved)
[18349854815] [INFO] [kernel::memory]   [32] 0x7838f000 - 0x78390000 (Other)
[18350265833] [INFO] [kernel::memory]   [33] 0x78390000 - 0x78391000 (Reserved)
[18350627054] [INFO] [kernel::memory]   [34] 0x78391000 - 0x78392000 (Other)
[18350972459] [INFO] [kernel::memory]   [35] 0x78392000 - 0x78393000 (Reserved)
[18351343917] [INFO] [kernel::memory]   [36] 0x78393000 - 0x78394000 (Other)
[18351688770] [INFO] [kernel::memory]   [37] 0x78394000 - 0x78395000 (Reserved)
[18352047049] [INFO] [kernel::memory]   [38] 0x78395000 - 0x78396000 (Other)
[18352465157] [INFO] [kernel::memory]   [39] 0x78396000 - 0x78397000 (Reserved)
[18352926090] [INFO] [kernel::memory]   [40] 0x78397000 - 0x78398000 (Other)
[18353276217] [INFO] [kernel::memory]   [41] 0x78398000 - 0x78399000 (Reserved)
[18353649770] [INFO] [kernel::memory]   [42] 0x78399000 - 0x7839a000 (Other)
[18353996037] [INFO] [kernel::memory]   [43] 0x7839a000 - 0x7839b000 (Reserved)
[18354345290] [INFO] [kernel::memory]   [44] 0x7839b000 - 0x7839c000 (Other)
[18354766896] [INFO] [kernel::memory]   [45] 0x7839c000 - 0x7839d000 (Reserved)
[18355129354] [INFO] [kernel::memory]   [46] 0x7839d000 - 0x7839e000 (Other)
[18355478029] [INFO] [kernel::memory]   [47] 0x7839e000 - 0x7839f000 (Reserved)
[18355847737] [INFO] [kernel::memory]   [48] 0x7839f000 - 0x783a0000 (Other)
[18356195414] [INFO] [kernel::memory]   [49] 0x783a0000 - 0x783a1000 (Reserved)
[18356551681] [INFO] [kernel::memory]   [50] 0x783a1000 - 0x783a2000 (Other)
[18356971038] [INFO] [kernel::memory]   [51] 0x783a2000 - 0x783a3000 (Reserved)
[18357326546] [INFO] [kernel::memory]   [52] 0x783a3000 - 0x783a4000 (Other)
[18357670024] [INFO] [kernel::memory]   [53] 0x783a4000 - 0x783a5000 (Reserved)
[18358055706] [INFO] [kernel::memory]   [54] 0x783a5000 - 0x783a6000 (Other)
[18358389778] [INFO] [kernel::memory]   [55] 0x783a6000 - 0x783a7000 (Reserved)
[18358739722] [INFO] [kernel::memory]   [56] 0x783a7000 - 0x783a8000 (Other)
[18359071112] [INFO] [kernel::memory]   [57] 0x783a8000 - 0x783a9000 (Reserved)
[18359487888] [INFO] [kernel::memory]   [58] 0x783a9000 - 0x783aa000 (Other)
[18359834708] [INFO] [kernel::memory]   [59] 0x783aa000 - 0x783ab000 (Reserved)
[18360187891] [INFO] [kernel::memory]   [60] 0x783ab000 - 0x783ac000 (Other)
[18360542516] [INFO] [kernel::memory]   [61] 0x783ac000 - 0x783ad000 (Reserved)
[18360897431] [INFO] [kernel::memory]   [62] 0x783ad000 - 0x783ae000 (Other)
[18361239641] [INFO] [kernel::memory]   [63] 0x783ae000 - 0x783af000 (Reserved)
[18361910383] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[18646369390] [CONTRACT] [kernel::memory] Frame allocator initialized with 488238 free frames
[18655000923] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[18661212440] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[18662561909] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[18663474330] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[18671782207] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[18672769987] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[18676592767] [INFO] [bran::arch] IOAPIC: Registers initialized
[18677921141] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[18679896299] [INFO] [bran::arch] IOAPIC: All pins masked
[18681541827] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[18682222446] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[18682742053] [INFO] [bran::arch] IOAPIC: Init complete
[18683386627] [CONTRACT] [kernel] Initializing global allocator...
[19080999167] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[19081918135] [CONTRACT] [kernel] Initializing SIMD...
[19083708967] [CONTRACT] [kernel] Initializing tasking...
[19089796176] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[19092076708] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[19092767950] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[19100556264] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[19101140193] [INFO] [kernel::task::scheduler]   Initializing boot task...
[19102385791] [INFO] [kernel::task::scheduler]   Creating boot task...
[19108281758] [INFO] [kernel::task::scheduler]   Creating idle task...
[19114819368] [INFO] [kernel::task::scheduler]   Boot task initialized
[19115356197] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[19116451522] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[19123798098] [INFO] [kernel::root] Spawning Root service...
[19134601067] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[19147366082] [INFO] [kernel::root::service] ROOT: started once

```
</details>
