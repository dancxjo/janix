# ✅ Scenario: DrawList demo renders shapes and updates

> Last run: 2026-01-26 20:41:12

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 13719ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | And I wait for the system to reach ready state | ⏭️ | 610ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> - [💾](./02/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[28941937242] [CONTRACT] [kernel] thing-os kernel starting...
[28954255980] [INFO] [kernel::memory] Memory map has 64 entries
[28957051377] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[28957912240] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[28958253103] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[28958601148] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[28958960602] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[28959350666] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[28959671335] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[28959993850] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[28960357294] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78396000 (Usable)
[28960698631] [INFO] [kernel::memory]   [9] 0x78396000 - 0x783f9000 (Reserved)
[28961039047] [INFO] [kernel::memory]   [10] 0x783f9000 - 0x783fa000 (Other)
[28961457937] [INFO] [kernel::memory]   [11] 0x783fa000 - 0x783fb000 (Reserved)
[28961862426] [INFO] [kernel::memory]   [12] 0x783fb000 - 0x783fc000 (Other)
[28962206745] [INFO] [kernel::memory]   [13] 0x783fc000 - 0x783fd000 (Reserved)
[28962799771] [INFO] [kernel::memory]   [14] 0x783fd000 - 0x783fe000 (Other)
[28963149727] [INFO] [kernel::memory]   [15] 0x783fe000 - 0x783ff000 (Reserved)
[28963495201] [INFO] [kernel::memory]   [16] 0x783ff000 - 0x78400000 (Other)
[28963889890] [INFO] [kernel::memory]   [17] 0x78400000 - 0x78401000 (Reserved)
[28964237569] [INFO] [kernel::memory]   [18] 0x78401000 - 0x78402000 (Other)
[28964622841] [INFO] [kernel::memory]   [19] 0x78402000 - 0x78403000 (Reserved)
[28964981134] [INFO] [kernel::memory]   [20] 0x78403000 - 0x78404000 (Other)
[28965336258] [INFO] [kernel::memory]   [21] 0x78404000 - 0x78405000 (Reserved)
[28965691591] [INFO] [kernel::memory]   [22] 0x78405000 - 0x78406000 (Other)
[28966124477] [INFO] [kernel::memory]   [23] 0x78406000 - 0x78407000 (Reserved)
[28966473559] [INFO] [kernel::memory]   [24] 0x78407000 - 0x78408000 (Other)
[28966805798] [INFO] [kernel::memory]   [25] 0x78408000 - 0x78409000 (Reserved)
[28967149476] [INFO] [kernel::memory]   [26] 0x78409000 - 0x7840a000 (Other)
[28967501612] [INFO] [kernel::memory]   [27] 0x7840a000 - 0x7840b000 (Reserved)
[28967875525] [INFO] [kernel::memory]   [28] 0x7840b000 - 0x7840c000 (Other)
[28968371812] [INFO] [kernel::memory]   [29] 0x7840c000 - 0x7840d000 (Reserved)
[28968743936] [INFO] [kernel::memory]   [30] 0x7840d000 - 0x7840e000 (Other)
[28969087848] [INFO] [kernel::memory]   [31] 0x7840e000 - 0x7840f000 (Reserved)
[28969444170] [INFO] [kernel::memory]   [32] 0x7840f000 - 0x78410000 (Other)
[28969803801] [INFO] [kernel::memory]   [33] 0x78410000 - 0x78411000 (Reserved)
[28970159610] [INFO] [kernel::memory]   [34] 0x78411000 - 0x78412000 (Other)
[28970572217] [INFO] [kernel::memory]   [35] 0x78412000 - 0x78413000 (Reserved)
[28970935036] [INFO] [kernel::memory]   [36] 0x78413000 - 0x78414000 (Other)
[28971282136] [INFO] [kernel::memory]   [37] 0x78414000 - 0x78415000 (Reserved)
[28971640939] [INFO] [kernel::memory]   [38] 0x78415000 - 0x78416000 (Other)
[28972212377] [INFO] [kernel::memory]   [39] 0x78416000 - 0x78417000 (Reserved)
[28972639394] [INFO] [kernel::memory]   [40] 0x78417000 - 0x78418000 (Other)
[28973036151] [INFO] [kernel::memory]   [41] 0x78418000 - 0x78419000 (Reserved)
[28973385868] [INFO] [kernel::memory]   [42] 0x78419000 - 0x7841a000 (Other)
[28973718421] [INFO] [kernel::memory]   [43] 0x7841a000 - 0x7841b000 (Reserved)
[28974063493] [INFO] [kernel::memory]   [44] 0x7841b000 - 0x7841c000 (Other)
[28974479490] [INFO] [kernel::memory]   [45] 0x7841c000 - 0x7841d000 (Reserved)
[28974832909] [INFO] [kernel::memory]   [46] 0x7841d000 - 0x7841e000 (Other)
[28975243955] [INFO] [kernel::memory]   [47] 0x7841e000 - 0x7841f000 (Reserved)
[28975629817] [INFO] [kernel::memory]   [48] 0x7841f000 - 0x78420000 (Other)
[28975973201] [INFO] [kernel::memory]   [49] 0x78420000 - 0x78421000 (Reserved)
[28976326680] [INFO] [kernel::memory]   [50] 0x78421000 - 0x78422000 (Other)
[28976667911] [INFO] [kernel::memory]   [51] 0x78422000 - 0x78423000 (Reserved)
[28977036417] [INFO] [kernel::memory]   [52] 0x78423000 - 0x78424000 (Other)
[28977453946] [INFO] [kernel::memory]   [53] 0x78424000 - 0x78425000 (Reserved)
[28977811155] [INFO] [kernel::memory]   [54] 0x78425000 - 0x78426000 (Other)
[28978150105] [INFO] [kernel::memory]   [55] 0x78426000 - 0x78427000 (Reserved)
[28978539521] [INFO] [kernel::memory]   [56] 0x78427000 - 0x78428000 (Other)
[28978925611] [INFO] [kernel::memory]   [57] 0x78428000 - 0x78429000 (Reserved)
[28979294977] [INFO] [kernel::memory]   [58] 0x78429000 - 0x7842a000 (Other)
[28979639264] [INFO] [kernel::memory]   [59] 0x7842a000 - 0x7842b000 (Reserved)
[28980077404] [INFO] [kernel::memory]   [60] 0x7842b000 - 0x7842c000 (Other)
[28980427017] [INFO] [kernel::memory]   [61] 0x7842c000 - 0x7842d000 (Reserved)
[28980782435] [INFO] [kernel::memory]   [62] 0x7842d000 - 0x7842e000 (Other)
[28981293839] [INFO] [kernel::memory]   [63] 0x7842e000 - 0x7842f000 (Reserved)
[28981927154] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[29271165102] [CONTRACT] [kernel::memory] Frame allocator initialized with 488366 free frames
[29279994586] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[29286116842] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[29287428219] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[29288360012] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[29296855272] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[29297539208] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[29301297866] [INFO] [bran::arch] IOAPIC: Registers initialized
[29302614636] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[29305038454] [INFO] [bran::arch] IOAPIC: All pins masked
[29306779889] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[29307539952] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[29308208537] [INFO] [bran::arch] IOAPIC: Init complete
[29308939535] [CONTRACT] [kernel] Initializing global allocator...
[29869571325] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[29870527774] [CONTRACT] [kernel] Initializing SIMD...
[29872328145] [CONTRACT] [kernel] Initializing tasking...
[29878304678] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[29880253116] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[29880953005] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[29888459990] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[29889022166] [INFO] [kernel::task::scheduler]   Initializing boot task...
[29890142316] [INFO] [kernel::task::scheduler]   Creating boot task...
[29896091279] [INFO] [kernel::task::scheduler]   Creating idle task...
[29902475943] [INFO] [kernel::task::scheduler]   Boot task initialized
[29903030961] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[29903876558] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[29915523545] [INFO] [kernel::root] Spawning Root service...
[29925555313] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[29938383950] [INFO] [kernel::root::service] ROOT: started once

```
</details>
