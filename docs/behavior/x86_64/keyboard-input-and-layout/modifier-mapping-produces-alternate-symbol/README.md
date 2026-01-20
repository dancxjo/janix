# ✅ Scenario: Modifier mapping produces alternate symbol

> Last run: 2026-01-19 21:32:34

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the clock window is ticking | ✅ | 9793ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I press Alt+A | ✅ | 703ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the serial log should contain 'CONTRACT: input key_event' | ✅ | 429ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And I should see the appropriate symbol rendered | ✅ | 407ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11042597874] [CONTRACT] [kernel] thing-os kernel starting...
[11052419037] [INFO] [kernel::memory] Memory map has 64 entries
[11054684553] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[11056779492] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[11057135826] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[11057499684] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[11057830410] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[11058158661] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[11058487968] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[11058814305] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[11059174995] [INFO] [kernel::memory]   [8] 0x1780000 - 0x7866a000 (Usable)
[11059519416] [INFO] [kernel::memory]   [9] 0x7866a000 - 0x786cc000 (Reserved)
[11059891260] [INFO] [kernel::memory]   [10] 0x786cc000 - 0x7884e000 (Other)
[11060238849] [INFO] [kernel::memory]   [11] 0x7884e000 - 0x7884f000 (Reserved)
[11060598384] [INFO] [kernel::memory]   [12] 0x7884f000 - 0x788e6000 (Other)
[11060943432] [INFO] [kernel::memory]   [13] 0x788e6000 - 0x788e7000 (Reserved)
[11061331644] [INFO] [kernel::memory]   [14] 0x788e7000 - 0x78988000 (Other)
[11061731967] [INFO] [kernel::memory]   [15] 0x78988000 - 0x78989000 (Reserved)
[11062091436] [INFO] [kernel::memory]   [16] 0x78989000 - 0x789c9000 (Other)
[11062438959] [INFO] [kernel::memory]   [17] 0x789c9000 - 0x789ca000 (Reserved)
[11062797504] [INFO] [kernel::memory]   [18] 0x789ca000 - 0x78a55000 (Other)
[11063161428] [INFO] [kernel::memory]   [19] 0x78a55000 - 0x78a56000 (Reserved)
[11063520402] [INFO] [kernel::memory]   [20] 0x78a56000 - 0x78aa2000 (Other)
[11063867067] [INFO] [kernel::memory]   [21] 0x78aa2000 - 0x78aa3000 (Reserved)
[11064225480] [INFO] [kernel::memory]   [22] 0x78aa3000 - 0x78aa9000 (Other)
[11064572013] [INFO] [kernel::memory]   [23] 0x78aa9000 - 0x78aaa000 (Reserved)
[11064930261] [INFO] [kernel::memory]   [24] 0x78aaa000 - 0x78f2b000 (Other)
[11065274847] [INFO] [kernel::memory]   [25] 0x78f2b000 - 0x78f2c000 (Reserved)
[11065632171] [INFO] [kernel::memory]   [26] 0x78f2c000 - 0x793ad000 (Other)
[11065977846] [INFO] [kernel::memory]   [27] 0x793ad000 - 0x793ae000 (Reserved)
[11066347314] [INFO] [kernel::memory]   [28] 0x793ae000 - 0x796af000 (Other)
[11066693187] [INFO] [kernel::memory]   [29] 0x796af000 - 0x796b0000 (Reserved)
[11067050478] [INFO] [kernel::memory]   [30] 0x796b0000 - 0x796b9000 (Other)
[11067395394] [INFO] [kernel::memory]   [31] 0x796b9000 - 0x796ba000 (Reserved)
[11067752751] [INFO] [kernel::memory]   [32] 0x796ba000 - 0x796be000 (Other)
[11068098591] [INFO] [kernel::memory]   [33] 0x796be000 - 0x796bf000 (Reserved)
[11068454694] [INFO] [kernel::memory]   [34] 0x796bf000 - 0x796c1000 (Other)
[11068863267] [INFO] [kernel::memory]   [35] 0x796c1000 - 0x796c2000 (Reserved)
[11069246859] [INFO] [kernel::memory]   [36] 0x796c2000 - 0x796c4000 (Other)
[11069609826] [INFO] [kernel::memory]   [37] 0x796c4000 - 0x796c5000 (Reserved)
[11070087534] [INFO] [kernel::memory]   [38] 0x796c5000 - 0x796c7000 (Other)
[11070467760] [INFO] [kernel::memory]   [39] 0x796c7000 - 0x796c8000 (Reserved)
[11070834291] [INFO] [kernel::memory]   [40] 0x796c8000 - 0x796ca000 (Other)
[11071236990] [INFO] [kernel::memory]   [41] 0x796ca000 - 0x796cb000 (Reserved)
[11071608537] [INFO] [kernel::memory]   [42] 0x796cb000 - 0x796cd000 (Other)
[11071998465] [INFO] [kernel::memory]   [43] 0x796cd000 - 0x796ce000 (Reserved)
[11072358429] [INFO] [kernel::memory]   [44] 0x796ce000 - 0x796d8000 (Other)
[11072705787] [INFO] [kernel::memory]   [45] 0x796d8000 - 0x796d9000 (Reserved)
[11073083868] [INFO] [kernel::memory]   [46] 0x796d9000 - 0x796dd000 (Other)
[11073429213] [INFO] [kernel::memory]   [47] 0x796dd000 - 0x796de000 (Reserved)
[11073788682] [INFO] [kernel::memory]   [48] 0x796de000 - 0x796e0000 (Other)
[11074134291] [INFO] [kernel::memory]   [49] 0x796e0000 - 0x796e1000 (Reserved)
[11074494585] [INFO] [kernel::memory]   [50] 0x796e1000 - 0x796e3000 (Other)
[11074841448] [INFO] [kernel::memory]   [51] 0x796e3000 - 0x796e4000 (Reserved)
[11075201808] [INFO] [kernel::memory]   [52] 0x796e4000 - 0x796e8000 (Other)
[11075549067] [INFO] [kernel::memory]   [53] 0x796e8000 - 0x79757000 (Other)
[11075895501] [INFO] [kernel::memory]   [54] 0x79757000 - 0x7990d000 (Other)
[11076438186] [INFO] [kernel::memory]   [55] 0x7990d000 - 0x7a16c000 (Reserved)
[11076906456] [INFO] [kernel::memory]   [56] 0x7a16c000 - 0x7bb6c000 (Usable)
[11077266552] [INFO] [kernel::memory]   [57] 0x7bb6c000 - 0x7bb8d000 (Reserved)
[11077630047] [INFO] [kernel::memory]   [58] 0x7bb8d000 - 0x7bb91000 (Other)
[11077982190] [INFO] [kernel::memory]   [59] 0x7bb91000 - 0x7bb92000 (Reserved)
[11078347863] [INFO] [kernel::memory]   [60] 0x7bb92000 - 0x7bb94000 (Other)
[11078700270] [INFO] [kernel::memory]   [61] 0x7bb94000 - 0x7bb95000 (Reserved)
[11079065316] [INFO] [kernel::memory]   [62] 0x7bb95000 - 0x7bb97000 (Other)
[11079441186] [INFO] [kernel::memory]   [63] 0x7bb97000 - 0x7bb98000 (Reserved)
[11080069539] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[11316376368] [CONTRACT] [kernel::memory] Frame allocator initialized with 495746 free frames
[11323350258] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[11327996394] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[11329089024] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[11329793574] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[11336337573] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[11336831814] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11339588502] [INFO] [bran::arch] IOAPIC: Registers initialized
[11340916356] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[11342425908] [INFO] [bran::arch] IOAPIC: All pins masked
[11343773826] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11344395282] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11344870251] [INFO] [bran::arch] IOAPIC: Init complete
[11345469003] [CONTRACT] [kernel] Initializing global allocator...
[11700797856] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[11701511745] [CONTRACT] [kernel] Initializing SIMD...
[11702900484] [CONTRACT] [kernel] Initializing tasking...
[11707560216] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[11709220281] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[11709811014] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[11715528528] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[11715949080] [INFO] [kernel::task::scheduler]   Initializing boot task...
[11716745799] [INFO] [kernel::task::scheduler]   Creating boot task...
[11721455724] [INFO] [kernel::task::scheduler]   Creating idle task...
[11726374440] [INFO] [kernel::task::scheduler]   Boot task initialized
[11726816442] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[11727671175] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[11733413076] [INFO] [kernel::root] Spawning Root service...
[11741054160] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[11750046132] [INFO] [kernel::root::service] ROOT: started once
[12651026949] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[12651899898] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[12695859627] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[12713803971] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[12742996068] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[12779490042] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[12781438098] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[12819552669] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[12843397611] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[12850191189] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[12852888972] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[12876472620] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[12879453345] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[12880659627] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[12884033349] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[12886931046] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[12887770995] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12895753992] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12907731012] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[12908836941] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[12911494332] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[12912452157] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[12921035094] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[12921884283] [CONTRACT] [kernel] Spawning init process...
[12923495277] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[12958509729] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62477300 ticks/sec), init_cnt=624773 for 100Hz
[12960699774] [CONTRACT] [kernel] Entering scheduler loop.
[12972573636] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[12977868387] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367676144 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563776488
[12989030736] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[12990717531] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[12996798375] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[12997646442] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[13013465091] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[13017633750] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[13021004601] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[13021904478] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[13026843621] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[13029215463] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[13030087719] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[13033193118] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[13034003202] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[13034719896] [INFO] [sprout::devtree] SPROUT: build() called
[13035348480] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[13039897200] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[13040705667] [INFO] [sprout] SPROUT: About to create Supervisor...
[13041496116] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[13042439553] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[13043167236] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[13048424202] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[13090696674] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[13099910571] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[13105598253] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[13110266136] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[13112731137] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[13115743047] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[13119320115] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[13122617442] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[13126131579] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[13129533813] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[13132987494] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[13136705670] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[13140287094] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[13143206274] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[13146133572] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[13150249959] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[13153900980] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[13157295294] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[13160313045] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[13163667429] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[13166693232] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[13169923668] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[13172905680] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[13177272240] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[13181517393] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[13184693247] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[13187899065] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[13191502896] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[13194779664] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[13197811572] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[13200831006] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[13203922842] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[13207005471] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[13210281975] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[13214090868] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[13217461224] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[13220894544] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[13224619518] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[13227769797] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[13231069104] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[13234338744] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[13237677321] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[13241085363] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[13243515351] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[13257479334] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[13336443219] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[13337414739] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[13339526673] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[13340638377] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[13341410841] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[13342280985] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[13345167330] [INFO] [kernel::task::loader] Loading module: /boot/clock
[13345818486] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13346908806] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13350461553] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[13351064034] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[13352408289] [INFO] [kernel::task::loader] Segment: vaddr=204ee8 exec=false
[13352995062] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13360172430] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[13363141506] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[13364066199] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[13364596806] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13366034715] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13369872252] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[13370449818] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[13371251949] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[13371788694] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13377338997] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[13378570623] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[13379524059] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[13380026616] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13381017309] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13391160486] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[13391833455] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[13396016502] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[13396616145] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[13402478694] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[13403240730] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[13404655737] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[13405227759] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13406252772] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13409753115] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[13410367872] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[13411866699] [INFO] [kernel::task::loader] Segment: vaddr=204c88 exec=false
[13412543694] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13418714496] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[13419744954] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[13436658048] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13437971811] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367986928 RFLAGS_BEFORE=134 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563776488
[13441858584] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[13442891022] [INFO] [clock] starting clock publisher
[13452779043] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13453968231] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368009968 RFLAGS_BEFORE=134 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563776488
[13459238562] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ab8
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[13460979675] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368026352 RFLAGS_BEFORE=134 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563776488
[13463912187] [ERROR] [INGESTD] Starting...
[13467864630] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ae8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13469518227] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368053488 RFLAGS_BEFORE=134 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563776488
[13473189147] [INFO] [cambium] cambium starting (v3: catch-up then stream)...
[13479784428] [INFO] [clock] Clock thing created: 356
[13480766046] [INFO] [clock] Waiting for UI Root (Compositor)...
[13501173642] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[13503089193] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[13504616664] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[13506425295] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[13514594313] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[13524622287] [ERROR] [INGESTD] Watch active. Loop start.
[13532891493] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[13534227399] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[13837796610] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[13838617749] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[13839909765] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13843168647] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[13843861713] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13844980116] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[13845605103] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13852583943] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[13859669307] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[13861136685] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368106128 RFLAGS_BEFORE=134 CR3_BEFORE=54927360 fs_base=0 gs_base=18446744071563776488
[13865154468] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[13884819267] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[13900244457] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[13911371529] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[13912967112] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[13913711691] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[13915483527] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13918893615] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[13919892261] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13921730493] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[13923080622] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[13929908817] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[13931322570] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[13932841065] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[13934188785] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[13935887757] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[13937026884] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[13937569470] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13938732291] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13941341172] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[13942053510] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[13943209071] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[13943831715] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[13950959121] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[13952769600] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[13953429600] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13955067060] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13958080719] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[13958749893] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13960036926] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[13960960134] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[13968665700] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[13971625701] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[13973220690] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[13974091428] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13976187390] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13979689548] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[13980710865] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13982275395] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[13983230217] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[14040455451] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044678
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[14058425898] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368129664 RFLAGS_BEFORE=130 CR3_BEFORE=55033856 fs_base=0 gs_base=18446744071563776488
[14065020519] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[14067007119] [INFO] [rtc_cmos] Starting... arg=db
[14069007744] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[14072952168] [INFO] [rtc_cmos] RTC: 2026-01-20 05:33:14 = 1768887194 unix_secs
[14074403970] [INFO] [kernel::time] System clock anchored: unix_secs=1768887194, mono_ns=7036940064, offset=1768887186963059936ns
[14075792709] [INFO] [rtc_cmos] System clock anchored
[14095184301] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[14097589308] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ae8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[14098811298] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368164304 RFLAGS_BEFORE=130 CR3_BEFORE=55136256 fs_base=0 gs_base=18446744071563776488
[14102927322] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[14104944612] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[14106105552] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[14107101261] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[14112764193] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004b958
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[14114469270] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368181712 RFLAGS_BEFORE=130 CR3_BEFORE=55230464 fs_base=0 gs_base=18446744071563776488
[14119838535] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[14120815071] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[14125394085] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0095fb0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[14126795628] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368207584 RFLAGS_BEFORE=130 CR3_BEFORE=55332864 fs_base=0 gs_base=18446744071563776488
[14130656298] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[14135686620] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[14140112019] [INFO] [bristle] bristle: registered in graph as svc.Input (id=454)
[14154754449] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=456 backend=BootFB
[14156185593] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[14156767746] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14157869616] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14216466594] [INFO] [kernel::task::loader] Segment: vaddr=2615d0 exec=false
[14217523122] [INFO] [kernel::task::loader]   Overlap at 261000: merging perms to r=true w=false x=true
[14224900074] [INFO] [kernel::task::loader] Segment: vaddr=26d1a0 exec=false
[14226206940] [INFO] [kernel::task::loader]   Overlap at 26d000: merging perms to r=true w=true x=true
[14234154330] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[14235849606] [INFO] [kernel::task::loader] Loading module: /boot/echo
[14236443738] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14237541549] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14240546958] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[14241187686] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14242586655] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[14243329551] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[14250229125] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[14252071086] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[14253070656] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[14266960026] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014c40
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1c8
[14268514953] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368249728 RFLAGS_BEFORE=130 CR3_BEFORE=55439360 fs_base=0 gs_base=18446744071563776488
[14272179834] [INFO] [bloom::logging] bloom: logging initialized
[14277060732] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004b958
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[14278800096] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368267184 RFLAGS_BEFORE=130 CR3_BEFORE=55975936 fs_base=0 gs_base=18446744071563776488
[14285057754] [INFO] [echo] echo: online (handle=12)
[14286252585] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[14294867169] [INFO] [bloom] bloom: [bloom] Bootstrapped via Bytespace ID=456
[14298265146] [INFO] [bloom] bloom: [bloom] starting (arg_req=1 arg_resp=4 bristle=10 font_svc=0)
[14308690836] [INFO] [bloom] bloom: [bloom] spawned wallpaper_loader thread (tid=15)
[14317362411] [INFO] [bloom] bloom: [bloom] spawned cursor_loader thread (tid=16)
[14323304127] [INFO] [bloom] bloom: [bloom] spawned font_loader thread (tid=17)
[14324094477] [INFO] [bloom] bloom: [bloom] discovering compositor target...
T:AEF0 [14329661808] [INFO] [bloom] bloom: [wallpaper_loader] thread started
[14333670021] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[14334773376] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[14336377077] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
T:AB20 [14341134522] [INFO] [bloom] bloom: [cursor_loader] thread started
T:A0B0 [14344317273] [INFO] [bloom] bloom: [font_loader] thread started
[14345123859] [INFO] [bloom] bloom: [font_loader] scanning boot modules for fonts...
[14351624232] [INFO] [bloom] bloom: [font_loader] found 42 boot modules
[14444457390] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/DSEG7Classic-Regular.ttf'
[14452159128] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=179 size=23272 name='/assets/fonts/DSEG7Classic-Regular.ttf'
[14480804151] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:35F0 [14487197340] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[14488562583] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[14491840176] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/Hack-Regular.ttf'
[14496373848] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[14497256367] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[14521675080] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[14524594953] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[14532634545] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=182 size=309408 name='/assets/fonts/Hack-Regular.ttf'
[14537344008] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[14540877219] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSans-Regular.ttf'
[14545463427] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[14546279847] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[15141387987] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[15142653405] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[15146101740] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=185 size=569208 name='/assets/fonts/NotoSans-Regular.ttf'
[15151034382] [INFO] [bloom] bloom: [wallpaper_loader] searching for wallpaper...
[15152256471] [INFO] [bloom] bloom: [wallpaper_loader] trying: /assets/wallpapers/clouds.bmp
[15157015203] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[15160765983] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol-Regular.ttf'
[15167859069] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[15168691296] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[15533968065] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=188 size=258156 name='/assets/fonts/NotoSansSymbol-Regular.ttf'
[15541464675] [INFO] [bloom] bloom: [cursor_loader] searching for cursor...
[15542497608] [INFO] [bloom] bloom: [cursor_loader] trying: /assets/cursors/plain/Normal.cur
[15865194378] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[16515919035] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=191 size=656852 name='/assets/fonts/NotoSansSymbol2-Regular.ttf'
[17437894452] [INFO] [bloom::compositor] bloom: compositor bytespace 376 (1280x720 stride=5120 format=1)
[18097162083] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSerif-Regular.ttf'
[18183926475] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[18185315313] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[18186273303] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (258156 bytes)
[18190916007] [INFO] [bloom::compositor] bloom: display backend: BootFB
[18196239138] [INFO] [ps2_mouse] ps2_mouse: init done
[18196935735] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[18197960352] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18198731100] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[18203059545] [INFO] [bloom::asset] [asset_bank] mapped at 0x10463000
[18203912661] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[18460122813] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=194 size=616196 name='/assets/fonts/NotoSerif-Regular.ttf'
[18788950356] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[19455906756] [INFO] [bloom] bloom: [bloom] compositor target: 1280x720 @ 0x104a3000 backend=BootFB
[19457305494] [INFO] [bloom] bloom: [bloom] presenter: driver (req=1 resp=4)
[19458950676] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[19791840486] [INFO] [stem::ui] UiBuilder: created root 545
[19792883781] [INFO] [bloom] bloom: [bloom] created UI root node: 545
[19804542978] [INFO] [display_bootfb] display_bootfb: bound bytespace 376
[20137700583] [INFO] [bloom] bloom: [font_loader] immediate scan complete, entering watch loop
[20140857363] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd5a0
[20141691141] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[20142377013] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20143607154] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[20156511804] [INFO] [clock] Found UI Root: 545 (attempt 4)
[20468114007] [INFO] [bloom] bloom: [bloom] ui watch opened UNFILTERED (userspace filter pred=UI_TEXT id=514)
[22131197352] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[22132593450] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[22133490258] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 191 (656852 bytes)
[22152701010] [INFO] [bloom::asset] [asset_bank] mapped at 0x10bab000
[22153599303] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[24450805995] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c1b40
[24452078079] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[24453207834] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24454327821] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[24780396267] [INFO] [bloom] bloom: [font_loader] watch opened (id=574)
[26101076892] [INFO] [bloom] bloom: [cursor_loader] SUCCESS: found candidate '/assets/cursors/plain/Normal.cur', enqueuing load
[27314297472] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[27316216884] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol2-Regular.ttf' in slot 4
[27353988684] [INFO] [bloom] bloom: [cursor_loader] thread done, sleeping forever
[27359152392] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 194 (616196 bytes)
[27364053849] [INFO] [bloom] bloom: [wallpaper_loader] SUCCESS: found candidate '/assets/wallpapers/clouds.bmp', enqueuing load
[27365424042] [INFO] [bloom] bloom: [wallpaper_loader] thread done, sleeping forever
[27372070143] [INFO] [bloom::asset] [asset_bank] mapped at 0x10c4c000
[27372900984] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSerif-Regular.ttf'...
[29002996869] [INFO] [cambium] Found 1 bindings
[30021677928] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fedd0
[30022642782] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[30023500287] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[30024400758] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x4 kind=0 pred=0 subj_lo=356
[30348419376] [INFO] [clock] Binding created: 590 (source=356 target=569)
[30349410234] [INFO] [clock] CLOCK: Entering main loop, publishing to thing_id=356
[30352195797] [INFO] [clock] unix=1768887202 utc=2026-01-20 05:33:22 mono_ns=15175253115
[30381953481] [INFO] [cambium] Opened watch 600 for source 356 (binding 590, start_seq=0)
[30406707078] [INFO] [cambium] CATCH-UP: Draining 1 watches for historical events...
[30747115509] [INFO] [cambium] cambium: drain complete payloads=0 overflows=0 last_seq=none
[30748707033] [INFO] [cambium] Entering event loop with 1 bindings (v3)
[31401794490] [INFO] [echo] KeyDown LAlt +Alt
[31732971666] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:33:22' tick=15175253115
[34370765682] [INFO] [echo] KeyDown A +Alt
[35026137696] [INFO] [clock] unix=1768887204 utc=2026-01-20 05:33:24 mono_ns=17512863885
[36342262110] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:33:24' tick=17512863885
[38157010980] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[38158400082] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSerif-Regular.ttf' in slot 5
[38159356851] [INFO] [bloom::asset] [asset_bank] load_cursor_immediate: /assets/cursors/plain/Normal.cur
[38276521041] [INFO] [bloom::asset] [asset_bank] mapping bytespace 152 (4286 bytes)
[38282280168] [INFO] [bloom::asset] [asset_bank] mapped to 0x10ce3000
[38283117576] [INFO] [bloom::asset] [asset_bank] checking CUR header: len=4286
[38283900534] [INFO] [bloom::asset] [asset_bank] valid CUR header detected
[38284944918] [INFO] [bloom::asset] [asset_bank] CUR: hotspot=(0, 0), img_size=4264, offset=22
[38285764374] [INFO] [bloom::asset] [asset_bank] decoding embedded DIB at offset 22...
[38287545714] [INFO] [bloom::asset] [asset_bank] SUCCESS: cursor DIB decoded 32x32
[38295591180] [INFO] [bloom::asset] [asset_bank] publish_cursor (pending)
[38296497426] [INFO] [bloom::asset] [asset_bank] cursor: Static frame 32x32 hotspot (0, 0)
[38297623650] [INFO] [bloom::asset] [asset_bank] load_wallpaper_immediate: /assets/wallpapers/clouds.bmp
[38422926234] [INFO] [bloom::asset] [asset_bank] mapping bytespace 170 (3145782 bytes)
[38432954703] [INFO] [bloom::asset] [asset_bank] mapped to 0x10ce5000
[38433783960] [INFO] [bloom::asset] [asset_bank] decoding BMP...
[38846116749] [INFO] [bloom::asset] [asset_bank] BMP decoded: 1024x1024
[39259328427] [INFO] [bloom::asset] [asset_bank] bytespace unmapped
[39260307570] [INFO] [bloom::asset] [asset_bank] publish_wallpaper (pending): 1024x1024
[39579129051] [INFO] [cambium] [cambium] write: binding_src=356 target=569 pred=ui.Text(514) val=604 seq=1141
[39584884746] [INFO] [cambium] Updated target 569 with value 604 (seq=1141)
[39598337064] [INFO] [cambium] Updated target 569 with value 15175253115 (seq=1143)
[39610598841] [INFO] [cambium] Updated target 569 with value 619 (seq=1153)
[39622829631] [INFO] [cambium] Updated target 569 with value 17512863885 (seq=1155)

```
</details>
