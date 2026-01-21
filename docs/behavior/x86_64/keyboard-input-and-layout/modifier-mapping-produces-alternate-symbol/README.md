# ✅ Scenario: Modifier mapping produces alternate symbol

> Last run: 2026-01-20 19:32:43

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the clock window is ticking | ✅ | 10587ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I press Alt+A | ✅ | 706ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the serial log should contain 'CONTRACT: input key_event' | ✅ | 397ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And I should see the appropriate symbol rendered | ✅ | 395ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[10930317189] [CONTRACT] [kernel] thing-os kernel starting...
[10939990083] [INFO] [kernel::memory] Memory map has 64 entries
[10942593750] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[10943208276] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[10943570649] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[10943927049] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[10944254706] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[10947542595] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[10947878865] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[10948202859] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[10948598265] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78656000 (Usable)
[10948936614] [INFO] [kernel::memory]   [9] 0x78656000 - 0x786b8000 (Reserved)
[10949284698] [INFO] [kernel::memory]   [10] 0x786b8000 - 0x7883a000 (Other)
[10949625159] [INFO] [kernel::memory]   [11] 0x7883a000 - 0x7883b000 (Reserved)
[10949977434] [INFO] [kernel::memory]   [12] 0x7883b000 - 0x788d2000 (Other)
[10950340104] [INFO] [kernel::memory]   [13] 0x788d2000 - 0x788d3000 (Reserved)
[10950692346] [INFO] [kernel::memory]   [14] 0x788d3000 - 0x78974000 (Other)
[10951031850] [INFO] [kernel::memory]   [15] 0x78974000 - 0x78975000 (Reserved)
[10951384686] [INFO] [kernel::memory]   [16] 0x78975000 - 0x789b5000 (Other)
[10951727721] [INFO] [kernel::memory]   [17] 0x789b5000 - 0x789b6000 (Reserved)
[10952083230] [INFO] [kernel::memory]   [18] 0x789b6000 - 0x78a41000 (Other)
[10952423922] [INFO] [kernel::memory]   [19] 0x78a41000 - 0x78a42000 (Reserved)
[10952778474] [INFO] [kernel::memory]   [20] 0x78a42000 - 0x78a8e000 (Other)
[10953120519] [INFO] [kernel::memory]   [21] 0x78a8e000 - 0x78a8f000 (Reserved)
[10953496653] [INFO] [kernel::memory]   [22] 0x78a8f000 - 0x78a95000 (Other)
[10953838434] [INFO] [kernel::memory]   [23] 0x78a95000 - 0x78a96000 (Reserved)
[10954190775] [INFO] [kernel::memory]   [24] 0x78a96000 - 0x78f17000 (Other)
[10954533480] [INFO] [kernel::memory]   [25] 0x78f17000 - 0x78f18000 (Reserved)
[10954887801] [INFO] [kernel::memory]   [26] 0x78f18000 - 0x79399000 (Other)
[10955227371] [INFO] [kernel::memory]   [27] 0x79399000 - 0x7939a000 (Reserved)
[10955583375] [INFO] [kernel::memory]   [28] 0x7939a000 - 0x7969b000 (Other)
[10955926443] [INFO] [kernel::memory]   [29] 0x7969b000 - 0x7969c000 (Reserved)
[10956370194] [INFO] [kernel::memory]   [30] 0x7969c000 - 0x796a5000 (Other)
[10956764247] [INFO] [kernel::memory]   [31] 0x796a5000 - 0x796a6000 (Reserved)
[10957122462] [INFO] [kernel::memory]   [32] 0x796a6000 - 0x796aa000 (Other)
[10957467939] [INFO] [kernel::memory]   [33] 0x796aa000 - 0x796ab000 (Reserved)
[10957820907] [INFO] [kernel::memory]   [34] 0x796ab000 - 0x796ad000 (Other)
[10958160345] [INFO] [kernel::memory]   [35] 0x796ad000 - 0x796ae000 (Reserved)
[10958514171] [INFO] [kernel::memory]   [36] 0x796ae000 - 0x796b0000 (Other)
[10958853576] [INFO] [kernel::memory]   [37] 0x796b0000 - 0x796b1000 (Reserved)
[10959206709] [INFO] [kernel::memory]   [38] 0x796b1000 - 0x796b3000 (Other)
[10959550899] [INFO] [kernel::memory]   [39] 0x796b3000 - 0x796b4000 (Reserved)
[10959903504] [INFO] [kernel::memory]   [40] 0x796b4000 - 0x796b6000 (Other)
[10960272279] [INFO] [kernel::memory]   [41] 0x796b6000 - 0x796b7000 (Reserved)
[10960626501] [INFO] [kernel::memory]   [42] 0x796b7000 - 0x796b9000 (Other)
[10960970460] [INFO] [kernel::memory]   [43] 0x796b9000 - 0x796ba000 (Reserved)
[10961323494] [INFO] [kernel::memory]   [44] 0x796ba000 - 0x796c4000 (Other)
[10961665440] [INFO] [kernel::memory]   [45] 0x796c4000 - 0x796c5000 (Reserved)
[10962019959] [INFO] [kernel::memory]   [46] 0x796c5000 - 0x796c9000 (Other)
[10962362961] [INFO] [kernel::memory]   [47] 0x796c9000 - 0x796ca000 (Reserved)
[10962718206] [INFO] [kernel::memory]   [48] 0x796ca000 - 0x796cc000 (Other)
[10963059393] [INFO] [kernel::memory]   [49] 0x796cc000 - 0x796cd000 (Reserved)
[10963439124] [INFO] [kernel::memory]   [50] 0x796cd000 - 0x796cf000 (Other)
[10963785228] [INFO] [kernel::memory]   [51] 0x796cf000 - 0x796d0000 (Reserved)
[10964139120] [INFO] [kernel::memory]   [52] 0x796d0000 - 0x796d4000 (Other)
[10964483277] [INFO] [kernel::memory]   [53] 0x796d4000 - 0x796d5000 (Reserved)
[10964839215] [INFO] [kernel::memory]   [54] 0x796d5000 - 0x79750000 (Other)
[10965184362] [INFO] [kernel::memory]   [55] 0x79750000 - 0x79907000 (Other)
[10965529707] [INFO] [kernel::memory]   [56] 0x79907000 - 0x7a16c000 (Reserved)
[10965886140] [INFO] [kernel::memory]   [57] 0x7a16c000 - 0x7bb6c000 (Usable)
[10966234158] [INFO] [kernel::memory]   [58] 0x7bb6c000 - 0x7bb8c000 (Reserved)
[10966707411] [INFO] [kernel::memory]   [59] 0x7bb8c000 - 0x7bb90000 (Other)
[10967701800] [INFO] [kernel::memory]   [60] 0x7bb90000 - 0x7bb91000 (Reserved)
[10968297483] [INFO] [kernel::memory]   [61] 0x7bb91000 - 0x7bb93000 (Other)
[10968886764] [INFO] [kernel::memory]   [62] 0x7bb93000 - 0x7bb94000 (Reserved)
[10969689456] [INFO] [kernel::memory]   [63] 0x7bb94000 - 0x7bb96000 (Other)
[10971023547] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[11208905136] [CONTRACT] [kernel::memory] Frame allocator initialized with 495726 free frames
[11215832496] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[11221051380] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[11222397252] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[11223283962] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[11229784236] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[11230500699] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11233874454] [INFO] [bran::arch] IOAPIC: Registers initialized
[11235148815] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[11236981998] [INFO] [bran::arch] IOAPIC: All pins masked
[11238471420] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11239327770] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11240097561] [INFO] [bran::arch] IOAPIC: Init complete
[11240901771] [CONTRACT] [kernel] Initializing global allocator...
[11571235038] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[11572318164] [CONTRACT] [kernel] Initializing SIMD...
[11575491774] [CONTRACT] [kernel] Initializing tasking...
[11580265950] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[11581825398] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[11582392140] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[11587798860] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[11588210304] [INFO] [kernel::task::scheduler]   Initializing boot task...
[11588919969] [INFO] [kernel::task::scheduler]   Creating boot task...
[11593226997] [INFO] [kernel::task::scheduler]   Creating idle task...
[11597827329] [INFO] [kernel::task::scheduler]   Boot task initialized
[11598210789] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[11599085718] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[11605070730] [INFO] [kernel::root] Spawning Root service...
[11612338353] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[11620920729] [INFO] [kernel::root::service] ROOT: started once
[12469563777] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[12470476656] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[12510614754] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[12528208341] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[12554726250] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[12586350084] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[12588117069] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[12625484124] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[12650329659] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[12657043575] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[12659505672] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[12682707774] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[12685148124] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[12686150169] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[12689292825] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[12691627542] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[12692355885] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12699653571] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12711337485] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[12712189974] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[12714507465] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[12715295637] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[12722914248] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[12723540489] [CONTRACT] [kernel] Spawning init process...
[12725042319] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[12759673311] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62179300 ticks/sec), init_cnt=621793 for 100Hz
[12761237973] [CONTRACT] [kernel] Entering scheduler loop.
[12769861599] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[12774404016] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072367676144 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563780584
[12784392159] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[12786002097] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[12786716811] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[12787465185] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[12806628285] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[12810062463] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[12813132387] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[12813926598] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[12817085952] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[12819299394] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[12820020543] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[12822910716] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[12823612329] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[12824316714] [INFO] [sprout::devtree] SPROUT: build() called
[12824930877] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[12830290473] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[12831039045] [INFO] [sprout] SPROUT: About to create Supervisor...
[12831668124] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[12832399866] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[12833015844] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[12837409497] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[12872529945] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[12877132653] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[12880144629] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[12883063083] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[12885156603] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[12887858808] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[12891484254] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[12894634203] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[12897604797] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[12900586611] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[12903529353] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[12906866940] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[12910250595] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[12913117140] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[12916004376] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[12918977544] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[12921863724] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[12924958002] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[12927756864] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[12930885165] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[12933855198] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[12937122825] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[12940002537] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[12942963693] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[12946223301] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[12949195380] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[12952143105] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[12955056312] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[12958313478] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[12961290078] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[12964244799] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[12967237041] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[12970248357] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[12973259211] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[12976214823] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[12979253199] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[12982301904] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[12985322724] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[12988238703] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[12991437228] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[12994423431] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[12997479297] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[13000520313] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[13002645744] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[13014408396] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[13078146345] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[13079077242] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[13080918807] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[13081939200] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[13082675364] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[13083469344] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[13085904348] [INFO] [kernel::task::loader] Loading module: /boot/clock
[13086428850] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13087596456] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13091362812] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[13091942985] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[13093233582] [INFO] [kernel::task::loader] Segment: vaddr=204ee8 exec=false
[13093799763] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13151847027] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13167452001] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072367986928 RFLAGS_BEFORE=134 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563780584
[13172587659] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[13173861459] [INFO] [clock] starting clock publisher
[13179398925] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[13182734994] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[13183835742] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[13184382321] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13185438057] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13189801713] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[13190434587] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[13191294732] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[13191875631] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13197807018] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[13198736958] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[13199691186] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[13200269940] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13201268949] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13211529903] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[13212234090] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[13216552932] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[13217751987] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[13223840817] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[13224621795] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[13225592688] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[13226116530] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13227612948] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13232127480] [INFO] [kernel::task::loader] Segment: vaddr=204360 exec=false
[13233246873] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[13234759395] [INFO] [kernel::task::loader] Segment: vaddr=205168 exec=false
[13235332176] [INFO] [kernel::task::loader]   Overlap at 205000: merging perms to r=true w=true x=true
[13242373848] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[13243600359] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[13262090358] [INFO] [clock] Clock thing created: 327
[13263165894] [INFO] [clock] Waiting for UI Root (Compositor)...
[13267037355] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044678
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13270148331] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368012096 RFLAGS_BEFORE=130 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563780584
[13279970154] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045680
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[13281608340] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368034864 RFLAGS_BEFORE=134 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563780584
[13284852636] [ERROR] [INGESTD] Starting...
[13290663375] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045718
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13291846029] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368051248 RFLAGS_BEFORE=134 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563780584
[13294527741] [INFO] [cambium] cambium starting (v4: no-op suppression)...
[13300325214] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[13303673823] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[13304925315] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[13306503111] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[13308589008] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[13326169659] [ERROR] [INGESTD] Watch active. Loop start.
[13336002801] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[13337238090] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[13561401975] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[13562168334] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[13563229482] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13566484107] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[13567378605] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13569079491] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[13569945312] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13578068196] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[13586839332] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00141a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[13588134945] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368100848 RFLAGS_BEFORE=134 CR3_BEFORE=54931456 fs_base=0 gs_base=18446744071563780584
[13593317397] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[13616816928] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[13627723065] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[13637385597] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[13639078068] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[13639934154] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[13641546303] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13645000017] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[13645921806] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13647673974] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[13648594872] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[13656005154] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[13657825566] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[13659707259] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[13661179389] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[13662959772] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[13664440251] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[13665085830] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13666148034] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13668579408] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[13669484796] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[13670808063] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[13671648045] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[13679344503] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[13681640313] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[13682311896] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13683388455] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13686588597] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[13687319316] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13688608593] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[13689180285] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[13695317229] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[13697094972] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[13698515655] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[13699320954] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13700890170] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13704678306] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[13705559868] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13707047277] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[13708175151] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13715517255] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[13730937198] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014318
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[13732280595] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368124512 RFLAGS_BEFORE=134 CR3_BEFORE=55037952 fs_base=0 gs_base=18446744071563780584
[13736857893] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[13737836178] [INFO] [rtc_cmos] Starting... arg=db
[13739548086] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[13744695954] [INFO] [rtc_cmos] RTC: 2026-01-21 03:33:30 = 1768966410 unix_secs
[13746016218] [INFO] [kernel::time] System clock anchored: unix_secs=1768966410, mono_ns=6872751319, offset=1768966403127248681ns
[13747285233] [INFO] [rtc_cmos] System clock anchored
[13767073881] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[13770026589] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ee60
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[13771182183] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368159088 RFLAGS_BEFORE=130 CR3_BEFORE=55140352 fs_base=0 gs_base=18446744071563780584
[13774869075] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[13778812014] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[13780293582] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[13781192073] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[13788343998] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00963a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[13790516652] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368176592 RFLAGS_BEFORE=130 CR3_BEFORE=55234560 fs_base=0 gs_base=18446744071563780584
[13794561594] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[13795508034] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[13800081636] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb009c588
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[13801404672] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368212624 RFLAGS_BEFORE=134 CR3_BEFORE=55336960 fs_base=0 gs_base=18446744071563780584
[13805117964] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[13811508183] [INFO] [bristle] bristle: registered in graph as svc.Input (id=457)
[13824062802] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=431 backend=BootFB
[13825663599] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[13826238030] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13827480150] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13892313204] [INFO] [kernel::task::loader] Segment: vaddr=26e020 exec=false
[13893304161] [INFO] [kernel::task::loader]   Overlap at 26e000: merging perms to r=true w=false x=true
[13901677020] [INFO] [kernel::task::loader] Segment: vaddr=278d58 exec=false
[13903106745] [INFO] [kernel::task::loader]   Overlap at 278000: merging perms to r=true w=true x=true
[13911024996] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[13912518147] [INFO] [kernel::task::loader] Loading module: /boot/echo
[13913062251] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13914087528] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13916977338] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[13917581271] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13918979349] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[13919601201] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13925815167] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[13927156683] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[13928152392] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[13940506701] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014c50
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1af
[13941764133] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368250464 RFLAGS_BEFORE=134 CR3_BEFORE=55443456 fs_base=0 gs_base=18446744071563780584
[13944753669] [INFO] [bloom::logging] bloom: logging initialized
[13968316593] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0091ae8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[13969552509] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368267920 RFLAGS_BEFORE=134 CR3_BEFORE=56029184 fs_base=0 gs_base=18446744071563780584
[13973908113] [INFO] [echo] echo: online (handle=12)
[13974696516] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
T:E580 T:E410 T:D530 [14011241607] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[14012294505] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[14013274704] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
[14113290510] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:25D0 [14118575559] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[14119845003] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[14124943602] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[14125865589] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[14148913185] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[14151466329] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[14160297690] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[14165019693] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[14165740512] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[15138626976] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[15140395446] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[15426809079] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[15436170981] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[15437005056] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[17721028380] [INFO] [bloom::compositor] bloom: compositor bytespace 376 (1280x720 stride=5120 format=1)
[18044115936] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[18045152862] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[18046031982] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (258156 bytes)
[18053457873] [INFO] [bloom::asset] [asset_bank] mapped at 0x10463000
[18054193740] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[18367894611] [INFO] [bloom::compositor] bloom: display backend: BootFB
[18694231050] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[18698505771] [INFO] [ps2_mouse] ps2_mouse: init done
[18699121914] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[18699952821] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18700675587] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[19357258305] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[19680189837] [INFO] [stem::ui] UiBuilder: created root 516
[19689259557] [INFO] [display_bootfb] display_bootfb: bound bytespace 376
[19796598657] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[19798122201] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[19799021583] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 191 (656852 bytes)
[19803412596] [INFO] [clock] Found UI Root: 516 (attempt 4)
[19809331839] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c1b90
[19810222113] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[19810978770] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19811898216] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[19817040210] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd610
[19817795811] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[19818498909] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19819248273] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=488 subj_lo=0
[19826339808] [INFO] [bloom::asset] [asset_bank] mapped at 0x10bab000
[19827100128] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[26646763902] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[26647784295] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol2-Regular.ttf' in slot 4
[26648615763] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 194 (616196 bytes)
[26658462039] [INFO] [bloom::asset] [asset_bank] mapped at 0x10c4c000
[26659153323] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSerif-Regular.ttf'...
[33176102817] [INFO] [clock] Binding created: 553 (source=327 target=542)
[33177312795] [INFO] [clock] CLOCK: Entering main loop, publishing to thing_id=327
[33180189471] [INFO] [clock] unix=1768966419 utc=2026-01-21 03:33:39 mono_ns=16589234409
[33538133838] [INFO] [cambium] Found 1 bindings
[34526709459] [INFO] [echo] KeyDown LAlt +Alt
[34529246928] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fedd0
[34530000780] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[34530721995] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34531460931] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x4 kind=0 pred=0 subj_lo=327
[34847517540] [INFO] [clock] CLOCK PUBLISH: thing=327 now_text='03:33:39' tick=16589234409
[34855872480] [INFO] [cambium] Opened watch 571 for source 327 (binding 553, start_seq=0)
[34880334951] [INFO] [cambium] CATCH-UP: Draining 1 watches for historical events...
[36850240779] [INFO] [echo] KeyDown A +Alt
[37178692419] [INFO] [echo] KeyUp A +Alt
[38162585703] [INFO] [clock] unix=1768966422 utc=2026-01-21 03:33:42 mono_ns=19081109520
[39479566962] [INFO] [clock] CLOCK PUBLISH: thing=327 now_text='03:33:42' tick=19081109520
[40126437054] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 102400)
[40127511303] [INFO] [bloom::asset] [asset_bank] promoting font 'DSEG7Classic-Regular.ttf' to gen=1 (102400b) in slot 0
[40128585915] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 204800)
[40129233738] [INFO] [bloom::asset] [asset_bank] promoting font 'Hack-Regular.ttf' to gen=1 (102400b) in slot 1
[40130042799] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 307200)
[40130667720] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=1 (102400b) in slot 2
[40131461172] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 409600)
[40132075566] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol-Regular.ttf' to gen=1 (102400b) in slot 3
[40132866180] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 512000)
[40133511792] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol2-Regular.ttf' to gen=1 (102400b) in slot 4

```
</details>
