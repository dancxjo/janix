# ✅ Scenario: Boot produces logs, graphics, and a live desktop

> Last run: 2026-01-29 22:19:33

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 13002ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | Then I should see log messages on the terminal | ✅ | 1519ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And each log message should include a monotonically increasing timestamp | ✅ | 1337ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And I should see the system clock tick for several seconds in the serial console | ✅ | 1336ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |
| 5 | And I should see the wallpaper on the screen within 60 seconds | ✅ | 3095ms | <a href="./05/after.png"><img src="./05/after.png" width="150" /></a> [📜](./05/serial.log) [💾](./05/registers.txt) |
| 6 | And I should see a cursor centered on the screen | ✅ | 2940ms | <a href="./06/after.png"><img src="./06/after.png" width="150" /></a> [📜](./06/serial.log) [💾](./06/registers.txt) |
| 7 | And I should see the text "thing-os" in the top-left corner of the screen | ✅ | 1542ms | <a href="./07/after.png"><img src="./07/after.png" width="150" /></a> [📜](./07/serial.log) [💾](./07/registers.txt) |
| 8 | And I should see frame count information in the top-left corner of the screen | ✅ | 1366ms | <a href="./08/after.png"><img src="./08/after.png" width="150" /></a> [📜](./08/serial.log) [💾](./08/registers.txt) |
| 9 | And I should see a clock window displaying a ticking clock | ✅ | 9554ms | <a href="./09/after.png"><img src="./09/after.png" width="150" /></a> [📜](./09/serial.log) [💾](./09/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[24933031113] [CONTRACT] [kernel] thing-os kernel starting...
[24964106175] [INFO] [kernel::memory] Memory map has 64 entries
[24972814192] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[24978857990] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[24984291174] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[24989696199] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[24995225248] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[25000582395] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[25006039639] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[25011284190] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[25017178786] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78285000 (Usable)
[25022871826] [INFO] [kernel::memory]   [9] 0x78285000 - 0x782e9000 (Reserved)
[25028490187] [INFO] [kernel::memory]   [10] 0x782e9000 - 0x782ea000 (Other)
[25034236431] [INFO] [kernel::memory]   [11] 0x782ea000 - 0x782eb000 (Reserved)
[25040008789] [INFO] [kernel::memory]   [12] 0x782eb000 - 0x782ec000 (Other)
[25045629994] [INFO] [kernel::memory]   [13] 0x782ec000 - 0x782ed000 (Reserved)
[25051410875] [INFO] [kernel::memory]   [14] 0x782ed000 - 0x782ee000 (Other)
[25057002185] [INFO] [kernel::memory]   [15] 0x782ee000 - 0x782ef000 (Reserved)
[25062910019] [INFO] [kernel::memory]   [16] 0x782ef000 - 0x782f0000 (Other)
[25068647573] [INFO] [kernel::memory]   [17] 0x782f0000 - 0x782f1000 (Reserved)
[25074359270] [INFO] [kernel::memory]   [18] 0x782f1000 - 0x782f2000 (Other)
[25080261771] [INFO] [kernel::memory]   [19] 0x782f2000 - 0x782f3000 (Reserved)
[25209637025] [INFO] [kernel::memory]   [20] 0x782f3000 - 0x782f4000 (Other)
[25215832791] [INFO] [kernel::memory]   [21] 0x782f4000 - 0x782f5000 (Reserved)
[25222140944] [INFO] [kernel::memory]   [22] 0x782f5000 - 0x782f6000 (Other)
[25228040698] [INFO] [kernel::memory]   [23] 0x782f6000 - 0x782f7000 (Reserved)
[25234614603] [INFO] [kernel::memory]   [24] 0x782f7000 - 0x782f8000 (Other)
[25241249206] [INFO] [kernel::memory]   [25] 0x782f8000 - 0x782f9000 (Reserved)
[25247654178] [INFO] [kernel::memory]   [26] 0x782f9000 - 0x782fa000 (Other)
[25253809117] [INFO] [kernel::memory]   [27] 0x782fa000 - 0x782fb000 (Reserved)
[25260470684] [INFO] [kernel::memory]   [28] 0x782fb000 - 0x782fc000 (Other)
[25267745637] [INFO] [kernel::memory]   [29] 0x782fc000 - 0x782fd000 (Reserved)
[25274936552] [INFO] [kernel::memory]   [30] 0x782fd000 - 0x782fe000 (Other)
[25281824849] [INFO] [kernel::memory]   [31] 0x782fe000 - 0x782ff000 (Reserved)
[25289020587] [INFO] [kernel::memory]   [32] 0x782ff000 - 0x78300000 (Other)
[25309136624] [INFO] [kernel::memory]   [33] 0x78300000 - 0x78301000 (Reserved)
[25316015647] [INFO] [kernel::memory]   [34] 0x78301000 - 0x78302000 (Other)
[25331406671] [INFO] [kernel::memory]   [35] 0x78302000 - 0x78303000 (Reserved)
[25347924677] [INFO] [kernel::memory]   [36] 0x78303000 - 0x78304000 (Other)
[25354620828] [INFO] [kernel::memory]   [37] 0x78304000 - 0x78305000 (Reserved)
[25361193439] [INFO] [kernel::memory]   [38] 0x78305000 - 0x78306000 (Other)
[25368233895] [INFO] [kernel::memory]   [39] 0x78306000 - 0x78307000 (Reserved)
[25374873802] [INFO] [kernel::memory]   [40] 0x78307000 - 0x78308000 (Other)
[25381305140] [INFO] [kernel::memory]   [41] 0x78308000 - 0x78309000 (Reserved)
[25397093582] [INFO] [kernel::memory]   [42] 0x78309000 - 0x7830a000 (Other)
[25405833115] [INFO] [kernel::memory]   [43] 0x7830a000 - 0x7830b000 (Reserved)
[25420297465] [INFO] [kernel::memory]   [44] 0x7830b000 - 0x7830c000 (Other)
[25428110814] [INFO] [kernel::memory]   [45] 0x7830c000 - 0x7830d000 (Reserved)
[25437503847] [INFO] [kernel::memory]   [46] 0x7830d000 - 0x7830e000 (Other)
[25443471777] [INFO] [kernel::memory]   [47] 0x7830e000 - 0x7830f000 (Reserved)
[25454959175] [INFO] [kernel::memory]   [48] 0x7830f000 - 0x78310000 (Other)
[25461307163] [INFO] [kernel::memory]   [49] 0x78310000 - 0x78311000 (Reserved)
[25468398613] [INFO] [kernel::memory]   [50] 0x78311000 - 0x78312000 (Other)
[25474934059] [INFO] [kernel::memory]   [51] 0x78312000 - 0x78313000 (Reserved)
[25481805484] [INFO] [kernel::memory]   [52] 0x78313000 - 0x78314000 (Other)
[25488018942] [INFO] [kernel::memory]   [53] 0x78314000 - 0x78315000 (Reserved)
[25494968673] [INFO] [kernel::memory]   [54] 0x78315000 - 0x78316000 (Other)
[25501452115] [INFO] [kernel::memory]   [55] 0x78316000 - 0x78317000 (Reserved)
[25507876138] [INFO] [kernel::memory]   [56] 0x78317000 - 0x78318000 (Other)
[25514588350] [INFO] [kernel::memory]   [57] 0x78318000 - 0x78319000 (Reserved)
[25520858611] [INFO] [kernel::memory]   [58] 0x78319000 - 0x7831a000 (Other)
[25526651926] [INFO] [kernel::memory]   [59] 0x7831a000 - 0x7831b000 (Reserved)
[25532746416] [INFO] [kernel::memory]   [60] 0x7831b000 - 0x7831c000 (Other)
[25538465287] [INFO] [kernel::memory]   [61] 0x7831c000 - 0x7831d000 (Reserved)
[25544245938] [INFO] [kernel::memory]   [62] 0x7831d000 - 0x7831e000 (Other)
[25550191979] [INFO] [kernel::memory]   [63] 0x7831e000 - 0x7831f000 (Reserved)
[25556513385] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[25856263475] [CONTRACT] [kernel::memory] Frame allocator initialized with 488093 free frames
[25872192344] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[25889723840] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[25896773683] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[25902226162] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[25921342402] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[25926959299] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[25937969085] [INFO] [bran::arch] IOAPIC: Registers initialized
[25945040749] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[25952938093] [INFO] [bran::arch] IOAPIC: All pins masked
[25960412384] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[25966351226] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[25971582212] [INFO] [bran::arch] IOAPIC: Init complete
[25976929988] [CONTRACT] [kernel] Initializing global allocator...
[27580373419] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[27588929701] [CONTRACT] [kernel] Initializing SIMD...
[27595818105] [CONTRACT] [kernel] Initializing tasking...
[27617383232] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[27632779893] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[27639857766] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[27655679687] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[27661420401] [INFO] [kernel::task::scheduler]   Initializing boot task...
[27821373069] [INFO] [kernel::task::scheduler]   Creating boot task...
[27832700128] [INFO] [kernel::task::scheduler]   Creating idle task...
[27844364898] [INFO] [kernel::task::scheduler]   Boot task initialized
[27850021185] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[27856782216] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[27869599773] [INFO] [kernel::root] Spawning Root service...
[27884633910] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[27907062369] [INFO] [kernel::root::service] ROOT: started once
[32303664173] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[32312947933] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[32398420982] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[32449609098] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[32526859816] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[32618678182] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[32641348871] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[32712161259] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[32760291424] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[32789699040] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[32803669007] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=656, idx=3) BAR5=0x810c4000
[32854427632] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[32875672369] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[32886964715] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[32901659751] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[32970758454] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[32978157579] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[33173319598] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[33199129882] [INFO] [kernel::task::loader] Segment: vaddr=210000 exec=false
[33210552303] [INFO] [kernel::task::loader] Segment: vaddr=215000 exec=false
[33230820216] [INFO] [kernel] Warning: Module registry page overflow, truncating list.
[33243097665] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[33251195493] [CONTRACT] [kernel] Spawning init process...
[33258389340] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[33294016445] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (61875300 ticks/sec), init_cnt=618753 for 100Hz
[33310600467] [CONTRACT] [kernel] Entering scheduler loop.
[33315923999] [INFO] [kernel::tests::time_test] TIME ANCHORING TEST: Starting...
[33323268482] [INFO] [kernel::tests::time_test] TIME ANCHORING TEST: FAIL - sys_time_now returned 16661394473 but should have failed before anchor
[33340842646] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: Starting...
[33356915748] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: 1000 samples monotonic - PASS
[33369705976] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: trace::now() monotonic - PASS
[33379800612] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: trace and syscall consistent - PASS
[33389144439] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: All tests PASS
[33401407299] [INFO] [kernel::tests::fw_tables] FW_TABLES SELFTEST: Starting...
[33413170906] [INFO] [kernel::tests::fw_tables] FW_TABLES SELFTEST: PASS
[33448301365] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0008580
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[33477197071] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072367741888 RFLAGS_BEFORE=134 CR3_BEFORE=50331648 fs_base=0 gs_base=18446744071563860640
[33515807939] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9b0 rip=0x2013ff rflags=0x202
[33532354627] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[33543063419] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[33552596419] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[33584557084] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[33599075575] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[33615473048] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[33625135796] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[33639853104] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[33650075952] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[33660787764] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[33675362978] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[33959322129] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[33968371471] [INFO] [sprout::devtree] SPROUT: build() called
[33975437913] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[33997043142] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[34004535465] [INFO] [sprout] SPROUT: About to create Supervisor...
[34011208778] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[34022496596] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[34033309358] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[34054712222] [INFO] [sprout::supervisor] SPROUT: Found 64 modules
[34112883593] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[34137859779] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[34154091592] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[34177780945] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[34191963189] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[34205569394] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[34223756036] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[34241443131] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[34255415963] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[34270512139] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[34286421606] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[34304776162] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[34322216644] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[34338612520] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/fontd'
[34355903819] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/blossom'
[34369473603] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/ingestd'
[34384572571] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/cambium'
[34399102596] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/scheduler_fairness'
[34417584063] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/boot/drawlist_demo'
[34435876845] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/boot/tick_printer'
[34453581906] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/boot/photosynthesis'
[34470277643] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/boot/ata_disk'
[34488180072] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/boot/iso_reader'
[34505468555] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Alternate.cur'
[34526541971] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Busy.cur'
[34543534585] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Diagonal1.ani'
[34565552805] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Diagonal2.ani'
[34580991712] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Handwriting.cur'
[34599907022] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Help.cur'
[34619814316] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Horizontal.ani'
[34637750782] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Link.ani'
[34658736808] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Move.cur'
[34678664490] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Normal.cur'
[34696076411] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/cursors/plain/Precision.cur'
[34712625967] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/cursors/plain/Text.cur'
[34732576509] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/cursors/plain/Unavailabe.cur'
[34752592146] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/cursors/plain/Vertical.ani'
[34769131282] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/cursors/plain/Working.ani'
[34789703806] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/wallpapers/clouds.bmp'
[34809794420] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/wallpapers/leather.bmp'
[34825444255] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/wallpapers/linen.bmp'
[34845379322] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[34865568538] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/fonts/Hack-Regular.ttf'
[34885308348] [INFO] [sprout::supervisor] SPROUT: Module[42] = '/assets/fonts/NotoSans-Regular.ttf'
[34900750909] [INFO] [sprout::supervisor] SPROUT: Module[43] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[34917630630] [INFO] [sprout::supervisor] SPROUT: Module[44] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[34938980601] [INFO] [sprout::supervisor] SPROUT: Module[45] = '/assets/fonts/NotoSerif-Regular.ttf'
[34957279471] [INFO] [sprout::supervisor] SPROUT: Module[46] = '/assets/pci/pci.ids'
[34974766175] [INFO] [sprout::supervisor] SPROUT: Module[47] = '/assets/cursors/future/alias.svg'
[34989958211] [INFO] [sprout::supervisor] SPROUT: Module[48] = '/assets/cursors/future/all-scroll.svg'
[35009637417] [INFO] [sprout::supervisor] SPROUT: Module[49] = '/assets/cursors/future/bottom_left_corner.svg'
[35031534300] [INFO] [sprout::supervisor] SPROUT: Module[50] = '/assets/cursors/future/bottom_right_corner.svg'
[35049621699] [INFO] [sprout::supervisor] SPROUT: Module[51] = '/assets/cursors/future/bottom_side.svg'
[35071514708] [INFO] [sprout::supervisor] SPROUT: Module[52] = '/assets/cursors/future/cell.svg'
[35086726547] [INFO] [sprout::supervisor] SPROUT: Module[53] = '/assets/cursors/future/center_ptr.svg'
[35106801866] [INFO] [sprout::supervisor] SPROUT: Module[54] = '/assets/cursors/future/col-resize.svg'
[35122523929] [INFO] [sprout::supervisor] SPROUT: Module[55] = '/assets/cursors/future/color-picker.svg'
[35143259550] [INFO] [sprout::supervisor] SPROUT: Module[56] = '/assets/cursors/future/context-menu.svg'
[35159527563] [INFO] [sprout::supervisor] SPROUT: Module[57] = '/assets/cursors/future/copy.svg'
[35179069951] [INFO] [sprout::supervisor] SPROUT: Module[58] = '/assets/cursors/future/crosshair.svg'
[35194977494] [INFO] [sprout::supervisor] SPROUT: Module[59] = '/assets/cursors/future/default.svg'
[35210977925] [INFO] [sprout::supervisor] SPROUT: Module[60] = '/assets/cursors/future/dnd-move.svg'
[35231155714] [INFO] [sprout::supervisor] SPROUT: Module[61] = '/assets/cursors/future/dnd-no-drop.svg'
[35251423111] [INFO] [sprout::supervisor] SPROUT: Module[62] = '/assets/cursors/future/down-arrow.svg'
[35267313151] [INFO] [sprout::supervisor] SPROUT: Module[63] = '/assets/cursors/future/draft.svg'
[35285292174] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[35318393994] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[35551565103] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[35561016184] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[35571894599] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[35585071258] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[35597520546] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/fontd'
[35605247210] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/blossom'
[35616852075] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[35629294941] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/photosynthesis'
[35641764734] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/drawlist_demo'
[35654567742] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[35665627567] [INFO] [kernel::task::loader] Loading module: /boot/clock
[35675851312] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[35689285007] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[35710570015] [INFO] [kernel::task::loader] Segment: vaddr=20b000 exec=false
[35721324592] [INFO] [kernel::task::loader] Segment: vaddr=20f000 exec=false
[35742056953] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[35755334049] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[35766555951] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[35777213224] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[35789895524] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[35810545421] [INFO] [kernel::task::loader] Segment: vaddr=20c000 exec=false
[35824309855] [INFO] [kernel::task::loader] Segment: vaddr=20e000 exec=false
[35872662814] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[35880021896] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[35887608409] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[35894154187] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[35903392704] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[35937188479] [INFO] [kernel::task::loader] Segment: vaddr=218000 exec=false
[35950367566] [INFO] [kernel::task::loader] Segment: vaddr=220000 exec=false
[36016606111] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0047260
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[36383643497] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368149632 RFLAGS_BEFORE=130 CR3_BEFORE=50778112 fs_base=0 gs_base=18446744071563860640
[36407692209] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00041a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[36421237012] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368167904 RFLAGS_BEFORE=130 CR3_BEFORE=50929664 fs_base=0 gs_base=18446744071563860640
[36440139337] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00475b0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[36453415193] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368185296 RFLAGS_BEFORE=130 CR3_BEFORE=51077120 fs_base=0 gs_base=18446744071563860640
[36468128178] [INFO] [ingestd] INGESTD: Starting unified content provider service...
[36475752066] [INFO] [ingestd] INGESTD: Initializing Limine module content source...
[36494016468] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[36500955545] [INFO] [sprout::supervisor] SPROUT: Seeding initial asset requests...
[36565666337] [INFO] [ingestd] INGESTD: Created Limine ContentSource node
[36572405018] [INFO] [ingestd] INGESTD: Performing initial boot module scan...
[36656722608] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/fontd'
[36663965692] [INFO] [kernel::task::loader] Loading module: /boot/fontd
[36669970967] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[36678398015] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[36714704165] [INFO] [kernel::task::loader] Segment: vaddr=225000 exec=false
[36726387353] [INFO] [kernel::task::loader] Segment: vaddr=22c000 exec=false
[36740994878] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[36747359729] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/blossom'
[36754444889] [INFO] [kernel::task::loader] Loading module: /boot/blossom
[36760889706] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[36769532679] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[36792030593] [INFO] [kernel::task::loader] Segment: vaddr=214000 exec=false
[36800707536] [INFO] [kernel::task::loader] Segment: vaddr=217000 exec=false
[36815118160] [INFO] [sprout::supervisor] SPROUT: App launched (PID=8)
[36821303264] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[36828800296] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[36835039524] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[36843389086] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[36855868958] [INFO] [kernel::task::loader] Segment: vaddr=207000 exec=false
[36863353803] [INFO] [kernel::task::loader] Segment: vaddr=209000 exec=false
[36877481912] [INFO] [sprout::supervisor] SPROUT: App launched (PID=9)
[36883604146] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/photosynthesis'
[36891128087] [INFO] [kernel::task::loader] Loading module: /boot/photosynthesis
[36897748960] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[36906430263] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[36943295443] [INFO] [kernel::task::loader] Segment: vaddr=226000 exec=false
[36956144587] [INFO] [kernel::task::loader] Segment: vaddr=22e000 exec=false
[36971067819] [INFO] [sprout::supervisor] SPROUT: App launched (PID=10)
[36977744117] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/drawlist_demo'
[36986579587] [INFO] [kernel::task::loader] Loading module: /boot/drawlist_demo
[36993446116] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[37002587147] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[37019099453] [INFO] [kernel::task::loader] Segment: vaddr=20b000 exec=false
[37028859990] [INFO] [kernel::task::loader] Segment: vaddr=20e000 exec=false
[37043154316] [INFO] [sprout::supervisor] SPROUT: App launched (PID=11)
[37050060777] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[37094978473] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0049010
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[37107993501] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368243648 RFLAGS_BEFORE=134 CR3_BEFORE=51302400 fs_base=0 gs_base=18446744071563860640
[37135579333] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004a338
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[37148699102] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368260032 RFLAGS_BEFORE=134 CR3_BEFORE=51572736 fs_base=0 gs_base=18446744071563860640
[37163949851] [INFO] [blossom] BLOSSOM: Starting SVG Cache Service
[37171482114] [INFO] [blossom] BLOSSOM: Init UI pipeline...
[37186220528] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004cf78
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[37199067657] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368276416 RFLAGS_BEFORE=134 CR3_BEFORE=51757056 fs_base=0 gs_base=18446744071563860640
[37214749913] [INFO] [cambium] cambium starting (v5.2: drain-to-eagain + smart-resync)...
[37231152738] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004e590
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[37244314857] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368292800 RFLAGS_BEFORE=134 CR3_BEFORE=51884032 fs_base=0 gs_base=18446744071563860640
[37260156838] [INFO] [photosynthesis] Photosynthesis starting...
[37275566599] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0051170
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[37296190076] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368319936 RFLAGS_BEFORE=134 CR3_BEFORE=52162560 fs_base=0 gs_base=18446744071563860640
[37314105352] [INFO] [drawlist_demo] DrawList demo starting (with new commands)...
[37344247419] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[37358142708] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[37373286267] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[37388876435] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=564 subj_lo=0
[37431298346] [INFO] [ingestd] INGESTD: Published new asset '/boot/sprout' (hash=61811fc99aafd0a2)
[37744699746] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[37758120872] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[37767788711] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[37781423926] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=588 subj_lo=0
[37813140832] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f8fb0
[37822524267] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[37836158935] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[37852154032] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=589 subj_lo=0
[37889300316] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[37903566253] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[37916673822] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[37928643435] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=594 subj_lo=0
[37954648110] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[37967913639] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1920x1080 stride=7680)
[39293124215] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f8fb0
[39306686536] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[39321201015] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[39334024177] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=596 subj_lo=0
[39367607679] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[39381333375] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[39392366114] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[39407876063] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=601 subj_lo=0
[39451060359] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[39464561854] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[39474865991] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[39490513439] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=606 subj_lo=0
[39535305867] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[39549257886] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[39564335216] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[39576070028] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=610 subj_lo=0
[39621541523] [INFO] [ingestd] INGESTD: Created File node '/boot/sprout' (90640 bytes, hash=61811fc99aafd0a2)
[39673053237] [INFO] [ingestd] INGESTD: Published asset '/boot/sprout' (raw, 90640 bytes, hash=61811fc99aafd0a2)
[39736037734] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[39744714598] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[39754586878] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[39770409054] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[39782006671] [INFO] [kernel::task::loader] Segment: vaddr=206000 exec=false
[39802624926] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=12)
[39835618487] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0049010
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x70006
[39853504701] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368407776 RFLAGS_BEFORE=130 CR3_BEFORE=60870656 fs_base=0 gs_base=18446744071563860640
[39873663782] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=6, drv_resp_w=7)
[39907698599] [INFO] [blossom] BLOSSOM: UI pipeline ready
[39965522434] [INFO] [blossom] BLOSSOM: Service node created, req=9, resp=12
[39978274061] [INFO] [blossom] BLOSSOM: Service ready
[40011191036] [INFO] [kernel::syscall::handlers::device] DEVICE: task 12 claimed device 631 (handle 0)
[40079681280] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x7e9000 -> virt=0x1001f000
[40113954260] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[40139944551] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[40150868313] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[40164780911] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[40181275189] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[40194048879] [INFO] [kernel::task::loader] Segment: vaddr=207000 exec=false
[40213672629] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=13)
[40228364269] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[40243112228] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=13, r=14)
[40255696285] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=15, r=16)
[40267358162] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=17, r=18)
[40278287699] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[40286081203] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[40301390132] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[40316998993] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[40326916771] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[40344148440] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=14)
[40356269229] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[40367301310] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[40381619966] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[40396244866] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[40409992232] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[40430444435] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=15)
[40442457798] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=19, r=20)
[40458436127] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[40467253000] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[40484510632] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[40497557115] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[40507338582] [INFO] [kernel::task::loader] Segment: vaddr=206000 exec=false
[40527735514] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=16)
[40571563865] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0049010
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x28b
[40590370665] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369442896 RFLAGS_BEFORE=134 CR3_BEFORE=61005824 fs_base=0 gs_base=18446744071563860640
[40608129468] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[40622471703] [INFO] [rtc_cmos] Starting... arg=28b
[40633671121] [INFO] [rtc_cmos] Serving device ID: ThingId([139, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[40652044144] [INFO] [rtc_cmos] RTC: 2026-01-29 22:19:57 = 1769725197 unix_secs
[40663583643] [INFO] [kernel::time] System clock anchored: unix_secs=1769725197, mono_ns=20331501748, offset=1769725176668498252ns
[40675699686] [INFO] [rtc_cmos] System clock anchored
[40704180204] [INFO] [rtc_cmos] RTC: Set sys.TimeState = 1 (Anchored)
[40736719026] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[40762574237] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb005d668
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xd
[40782840363] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369475664 RFLAGS_BEFORE=134 CR3_BEFORE=61124608 fs_base=0 gs_base=18446744071563860640
[40802188711] [INFO] [ps2_kbd] ps2_kbd: online (handle=13)
[40821483824] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb005e178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xf
[40841980385] [INFO] [task.user_enter] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369492048 RFLAGS_BEFORE=134 CR3_BEFORE=61235200 fs_base=0 gs_base=18446744071563860640
[40862615352] [INFO] [ps2_mouse] ps2_mouse: online (handle=15)
[40873189780] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[40890658221] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb005f380
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe001000110013
[40908260479] [INFO] [task.user_enter] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369516944 RFLAGS_BEFORE=134 CR3_BEFORE=61345792 fs_base=0 gs_base=18446744071563860640
[40932408470] [INFO] [bristle] bristle: online (kbd=14, mouse=16, evt=17, evt_echo=19)
[40964728575] [INFO] [ps2_kbd] ps2_kbd: created driver node 985
[40976869838] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[40986853242] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[41007047257] [INFO] [bristle] bristle: registered in graph as svc.Input (id=993)
[41041886225] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[41052846931] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[41062406940] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
[41091281649] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[41108153363] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=968 backend=BootFB
[41121108277] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[41131216055] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[41143807093] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[41309362279] [INFO] [kernel::task::loader] Segment: vaddr=2c5000 exec=false
[41334104134] [INFO] [kernel::task::loader] Segment: vaddr=2d9000 exec=false
[41355966401] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=17)
[41367637561] [INFO] [kernel::task::loader] Loading module: /boot/echo
[41377770943] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[41393480211] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[41405895696] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[41417656857] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[41437492673] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=18)
[41447876412] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[41459290481] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[41496911147] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb005d668
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x3c8
[41517742594] [INFO] [task.user_enter] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369564464 RFLAGS_BEFORE=134 CR3_BEFORE=61468672 fs_base=0 gs_base=18446744071563860640
[41538471525] [INFO] [bloom::logging] bloom: logging initialized
[41577236960] [INFO] [bloom] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[41599829144] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb005f380
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x14
[41617466162] [INFO] [task.user_enter] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369581728 RFLAGS_BEFORE=134 CR3_BEFORE=62447616 fs_base=0 gs_base=18446744071563860640
[41635027845] [INFO] [echo] echo: online (handle=20)
[41644229861] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[41678244135] [INFO] [ingestd] INGESTD: Published new asset '/boot/bristle' (hash=d55e2ab85adcac5d)
[41773487403] [INFO] [bloom::compositor] bloom: compositor bytespace 896 (1920x1080 stride=7680 format=2)
[41804287309] [INFO] [ingestd] INGESTD: Created File node '/boot/bristle' (29104 bytes, hash=d55e2ab85adcac5d)
[41821776544] [INFO] [ingestd] INGESTD: Published asset '/boot/bristle' (raw, 29104 bytes, hash=d55e2ab85adcac5d)
[41853696618] [INFO] [bloom::compositor] bloom: display backend: BootFB
[41883651042] [INFO] [bloom::compositor] bloom: mapped size=8294400 (source=bytespace_info)
[41957792303] [INFO] [stem::ui] UiBuilder: created root 1032
[41982161661] [INFO] [bloom] bloom: spawned asset watcher (tid=19)
[41989958423] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
T:3C90 [42017375023] [INFO] [bloom::painter_resources] [bloom] asset_watcher_entry: spawning sub-loaders
T:41D0 T:4040 T:2A00 [42101961626] [INFO] [ps2_mouse] ps2_mouse: init done
[42112250819] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[42122736895] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[42134552453] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[42177820151] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f64e0
[42191283666] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[42201351920] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[42215974088] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=737 subj_lo=0
[42252668653] [INFO] [photosynthesis] Found UI Root: 1032
[42293358565] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f64e0
[42304445460] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[42313918491] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[42329811994] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=644 pred=0 subj_lo=0
[42381305688] [INFO] [bloom::painter_resources] [bloom] wallpaper loader: loading leather.bmp
[42405161583] [INFO] [bloom::asset] [asset_bank] worker spawned tid=23 (priority=2)
[42422900564] [INFO] [bloom::painter_resources] [bloom] cursor loader: loading default cursor
[42439298200] [INFO] [bloom::painter_resources] [bloom] icon loader started
[42468873978] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f64e0
[42479530521] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[42488796701] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[42504479325] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=743 subj_lo=0
T:4C30 [42537497893] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[42550407549] [INFO] [bloom::asset] [asset_bank] load_wallpaper_immediate: leather.bmp
[42610672071] [INFO] [bloom] [bloom] Starting UI loop immediately (not waiting for fonts)
[42636524769] [INFO] [ingestd] INGESTD: Published new asset '/boot/rtc_cmos' (hash=32366e1092006840)
[42761331997] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef60
[42763003978] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[42764534063] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[42766111840] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x0 kind=0 pred=0 subj_lo=0
[42982689726] [INFO] [ingestd] INGESTD: Created File node '/boot/rtc_cmos' (33368 bytes, hash=32366e1092006840)
[42984249039] [INFO] [ingestd] INGESTD: Published asset '/boot/rtc_cmos' (raw, 33368 bytes, hash=32366e1092006840)
[43322921749] [INFO] [bloom::present] bloom: driver REGISTER (kind=1 caps=0x3)
[43326105565] [INFO] [bloom::present] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=8
[43348149775] [INFO] [ingestd] INGESTD: Published new asset '/boot/clock' (hash=771f0ce19bac6901)
[43406340569] [INFO] [display_bootfb] display_bootfb: bound bytespace 896
[43479592543] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7feba0
[43485721359] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[43491155486] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[43492709693] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=782 pred=0 subj_lo=0
[43626559463] [INFO] [ingestd] INGESTD: Created File node '/boot/clock' (66064 bytes, hash=771f0ce19bac6901)
[43634413139] [INFO] [ingestd] INGESTD: Published asset '/boot/clock' (raw, 66064 bytes, hash=771f0ce19bac6901)
[43699435386] [INFO] [bloom::asset] [asset_bank] mapping bytespace 188 (4718646 bytes) for 'leather.bmp'
[43735980100] [INFO] [bloom::asset] [asset_bank] decoding BMP for 'leather.bmp'...
[44695564009] [INFO] [bloom::asset] [asset_bank] BMP decoded: 1536x1024 for 'leather.bmp'
[45804903080] [INFO] [bloom::asset] [asset_bank] publish_wallpaper (pending): 1536x1024
[45809452160] [INFO] [bloom::asset] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[45986313863] [INFO] [ingestd] INGESTD: Published new asset '/boot/font_explorer' (hash=1cdbe11025543439)
[46180367967] [INFO] [ingestd] INGESTD: Created File node '/boot/font_explorer' (61968 bytes, hash=1cdbe11025543439)
[46205506591] [INFO] [ingestd] INGESTD: Published asset '/boot/font_explorer' (raw, 61968 bytes, hash=1cdbe11025543439)
[46363360692] [INFO] [bloom::present] bloom: driver BIND ACK
[46387932744] [INFO] [bloom::reclaimer] [reclaimer] +6291456 bytes (total: 6291456)
[46394425416] [INFO] [bloom::asset] [asset_bank] promoting wallpaper 'leather.bmp' to gen=1 (6291456b)
[46519295693] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x10040870
[46521162548] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[46522566472] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[46527661825] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=795 pred=0 subj_lo=0
[46545215658] [INFO] [ingestd] INGESTD: Published new asset '/boot/ps2_kbd' (hash=4da765f0b8256772)
[46578389781] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x10040870
[46584340001] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[46586221389] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[46587713439] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=782 pred=0 subj_lo=0
[46630650329] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x10040870
[46637423854] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[46641416855] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[46643017986] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=796 pred=0 subj_lo=0
[46689584450] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x10040870
[46696173120] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[46702849867] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[46705742241] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=797 pred=0 subj_lo=0
[46809879701] [INFO] [ingestd] INGESTD: Created File node '/boot/ps2_kbd' (25008 bytes, hash=4da765f0b8256772)
[46817707654] [INFO] [ingestd] INGESTD: Published asset '/boot/ps2_kbd' (raw, 25008 bytes, hash=4da765f0b8256772)
[47048792626] [INFO] [bloom::asset] [asset_bank] mapping bytespace 251 (3051 bytes)
[47073586844] [INFO] [bloom::asset] [asset_bank] mapped to 0x11c95000
[47076401150] [INFO] [bloom::asset] [asset_bank] detected SVG format
[47078229292] [INFO] [bloom::asset] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[47278804811] [INFO] [bloom::raster] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[47295856431] [INFO] [bloom::raster] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[47304237794] [INFO] [bloom::raster] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[47310811768] [INFO] [bloom::raster] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[47314185340] [INFO] [bloom::raster] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[47326153736] [INFO] [bloom::asset] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[47344581085] [INFO] [ingestd] INGESTD: Published new asset '/boot/echo' (hash=a11f50e00fa91378)
[47371337700] [INFO] [bloom::asset] [asset_bank] publish_cursor (pending)
[47377120363] [INFO] [bloom::asset] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[47383753626] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 194 (23272 bytes)
[47423055605] [INFO] [bloom::asset] [asset_bank] mapped at 0x11c96000
[47426189873] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[47458109212] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[47603335517] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[47610477479] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 197 (309408 bytes)
[47630947390] [INFO] [bloom::asset] [asset_bank] mapped at 0x11c9c000
[47636482120] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[47718472963] [INFO] [ingestd] INGESTD: Created File node '/boot/echo' (25008 bytes, hash=a11f50e00fa91378)
[47724486441] [INFO] [ingestd] INGESTD: Published asset '/boot/echo' (raw, 25008 bytes, hash=a11f50e00fa91378)
[47954282082] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[48160593176] [INFO] [bloom] [CONTRACT] [bloom] First frame rendered
[48196629222] [INFO] [bloom::reclaimer] [reclaimer] +16384 bytes (total: 6307840)
[48202805819] [INFO] [bloom::asset] [asset_bank] promoting cursor to gen=2 (16384b)
[48205828955] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 6410240)
[48208230354] [INFO] [bloom::asset] [asset_bank] promoting font 'DSEG7Classic-Regular.ttf' to gen=2 (102400b) in slot 0
[48275287709] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[48283867595] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 200 (569208 bytes)
[48311669375] [INFO] [bloom::asset] [asset_bank] mapped at 0x11dc3000
[48316292678] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[50011484110] [INFO] [ingestd] INGESTD: Published new asset '/boot/bloom' (hash=7904093ad49c5bd5)
[53402270606] [INFO] [ingestd] INGESTD: Created File node '/boot/bloom' (895496 bytes, hash=7904093ad49c5bd5)
[53404579752] [INFO] [ingestd] INGESTD: Published asset '/boot/bloom' (raw, 895496 bytes, hash=7904093ad49c5bd5)
[54066746379] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[54260048327] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[54267471663] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 203 (258156 bytes)
[54290993346] [INFO] [bloom::asset] [asset_bank] mapped at 0x11e55000
[54296672121] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[56750918324] [INFO] [ingestd] INGESTD: Published new asset '/boot/ps2_mouse' (hash=8d3523c823935b52)
[58219171857] [INFO] [drawlist_demo] Frame 4: color cycle, clip=true, scale=0.8
[60986530692] [INFO] [ingestd] INGESTD: Created File node '/boot/ps2_mouse' (25008 bytes, hash=8d3523c823935b52)
[60988789502] [INFO] [ingestd] INGESTD: Published asset '/boot/ps2_mouse' (raw, 25008 bytes, hash=8d3523c823935b52)
[63040720162] [INFO] [clock] CLOCK PUBLISH: thing=803 now_text='22:20:06' tick=30267047794
[67157007650] [INFO] [ingestd] INGESTD: Published new asset '/boot/root_batch_bench' (hash=323a9e6726f38ab3)
[67884414037] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[67898230041] [INFO] [clock] CLOCK PUBLISH: thing=803 now_text='22:20:09' tick=32634404900
[67913190563] [INFO] [bloom::cursor_rasterizer] [cursor_rasterizer] rasterizing cursor snapshot gen=2
[67989198234] [INFO] [bloom::present] display: full-frame damage, using full-frame present
[68014216872] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 6512640)
[68019665367] [INFO] [bloom::asset] [asset_bank] promoting font 'Hack-Regular.ttf' to gen=3 (102400b) in slot 1
[68023218421] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 6615040)
[68025605777] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=3 (102400b) in slot 2
[68146534069] [INFO] [ingestd] INGESTD: Created File node '/boot/root_batch_bench' (29200 bytes, hash=323a9e6726f38ab3)
[68157055131] [INFO] [ingestd] INGESTD: Published asset '/boot/root_batch_bench' (raw, 29200 bytes, hash=323a9e6726f38ab3)
[68195199802] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[68204284513] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 206 (656852 bytes)
[68231880978] [INFO] [bloom::asset] [asset_bank] mapped at 0x11e9d000
[68237575404] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[70637831963] [INFO] [clock] CLOCK PUBLISH: thing=803 now_text='22:20:11' tick=34487355718
[71395656152] [INFO] [drawlist_demo] Frame 8: color cycle, clip=true, scale=1
[72712855859] [INFO] [ingestd] INGESTD: Published new asset '/boot/root_watch_tester' (hash=5b29257b2e16f32e)
[73058907819] [INFO] [clock] CLOCK PUBLISH: thing=803 now_text='22:20:12' tick=35443999748
[75584864396] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[75619401035] [INFO] [ingestd] INGESTD: Created File node '/boot/root_watch_tester' (41488 bytes, hash=5b29257b2e16f32e)
[75628788146] [INFO] [ingestd] INGESTD: Published asset '/boot/root_watch_tester' (raw, 41488 bytes, hash=5b29257b2e16f32e)
[76190327924] [INFO] [clock] CLOCK PUBLISH: thing=803 now_text='22:20:14' tick=37805189072
[76908526194] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol2-Regular.ttf' in slot 4
[76910997233] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 209 (616196 bytes)
[76980935919] [INFO] [bloom::asset] [asset_bank] mapped at 0x11f51000
[76986972055] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSerif-Regular.ttf'...
[78381749208] [INFO] [ingestd] INGESTD: Published new asset '/boot/display_bootfb' (hash=e581a9426f17f21e)
[79024383421] [INFO] [clock] CLOCK PUBLISH: thing=803 now_text='22:20:15' tick=38608792192
[82731251298] [INFO] [ingestd] INGESTD: Created File node '/boot/display_bootfb' (29272 bytes, hash=e581a9426f17f21e)
[82736385470] [INFO] [ingestd] INGESTD: Published asset '/boot/display_bootfb' (raw, 29272 bytes, hash=e581a9426f17f21e)
[83899595397] [INFO] [clock] CLOCK PUBLISH: thing=803 now_text='22:20:17' tick=40662738889
[87311077637] [INFO] [drawlist_demo] Frame 12: color cycle, clip=true, scale=0.8
[87995516350] [INFO] [clock] CLOCK PUBLISH: thing=803 now_text='22:20:19' tick=42534311926
[90768315040] [INFO] [ingestd] INGESTD: Published new asset '/boot/fontd' (hash=a6d7f7d6cc48065e)
[91911697241] [INFO] [clock] CLOCK PUBLISH: thing=803 now_text='22:20:21' tick=44576311408
[94684592625] [INFO] [ingestd] INGESTD: Created File node '/boot/fontd' (184848 bytes, hash=a6d7f7d6cc48065e)
[94687404657] [INFO] [ingestd] INGESTD: Published asset '/boot/fontd' (raw, 184848 bytes, hash=a6d7f7d6cc48065e)
[95620433444] [INFO] [clock] CLOCK PUBLISH: thing=803 now_text='22:20:23' tick=46535492530
[100016037933] [INFO] [clock] CLOCK PUBLISH: thing=803 now_text='22:20:25' tick=48643597905
[103105763891] [INFO] [ingestd] INGESTD: Published new asset '/boot/blossom' (hash=e5637a1100e504c3)
[103337668455] [INFO] [drawlist_demo] Frame 16: color cycle, clip=true, scale=1

```
</details>
