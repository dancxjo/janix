# ✅ Scenario: Cursor responds to mouse movement

> Last run: 2026-01-19 21:36:59

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given a cursor is visible on the screen | ✅ | 4780ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | When I move the mouse | ✅ | 612ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the serial log should contain 'CONTRACT: input pointer_move' | ✅ | 416ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And the cursor should move correspondingly on the screen | ✅ | 411ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[10970200461] [CONTRACT] [kernel] thing-os kernel starting...
[10981541967] [INFO] [kernel::memory] Memory map has 64 entries
[10983772899] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[10984417554] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[10984758279] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[10985124777] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[10985463654] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[10985793060] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[10986125469] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[10986454644] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[10986851436] [INFO] [kernel::memory]   [8] 0x1780000 - 0x7866a000 (Usable)
[10987197573] [INFO] [kernel::memory]   [9] 0x7866a000 - 0x786cc000 (Reserved)
[10987556910] [INFO] [kernel::memory]   [10] 0x786cc000 - 0x7884e000 (Other)
[10987913805] [INFO] [kernel::memory]   [11] 0x7884e000 - 0x7884f000 (Reserved)
[10988288487] [INFO] [kernel::memory]   [12] 0x7884f000 - 0x788e6000 (Other)
[10988649309] [INFO] [kernel::memory]   [13] 0x788e6000 - 0x788e7000 (Reserved)
[10989022308] [INFO] [kernel::memory]   [14] 0x788e7000 - 0x78988000 (Other)
[10989381315] [INFO] [kernel::memory]   [15] 0x78988000 - 0x78989000 (Reserved)
[10989755139] [INFO] [kernel::memory]   [16] 0x78989000 - 0x789c9000 (Other)
[10990127940] [INFO] [kernel::memory]   [17] 0x789c9000 - 0x789ca000 (Reserved)
[10990491831] [INFO] [kernel::memory]   [18] 0x789ca000 - 0x78a55000 (Other)
[10990843875] [INFO] [kernel::memory]   [19] 0x78a55000 - 0x78a56000 (Reserved)
[10991210307] [INFO] [kernel::memory]   [20] 0x78a56000 - 0x78aa2000 (Other)
[10991567169] [INFO] [kernel::memory]   [21] 0x78aa2000 - 0x78aa3000 (Reserved)
[10991932347] [INFO] [kernel::memory]   [22] 0x78aa3000 - 0x78aa9000 (Other)
[10992335508] [INFO] [kernel::memory]   [23] 0x78aa9000 - 0x78aaa000 (Reserved)
[10992701709] [INFO] [kernel::memory]   [24] 0x78aaa000 - 0x78f2b000 (Other)
[10993055469] [INFO] [kernel::memory]   [25] 0x78f2b000 - 0x78f2c000 (Reserved)
[10993437576] [INFO] [kernel::memory]   [26] 0x78f2c000 - 0x793ad000 (Other)
[10993814865] [INFO] [kernel::memory]   [27] 0x793ad000 - 0x793ae000 (Reserved)
[10997533569] [INFO] [kernel::memory]   [28] 0x793ae000 - 0x796af000 (Other)
[10998115590] [INFO] [kernel::memory]   [29] 0x796af000 - 0x796b0000 (Reserved)
[10998698766] [INFO] [kernel::memory]   [30] 0x796b0000 - 0x796b9000 (Other)
[10999268676] [INFO] [kernel::memory]   [31] 0x796b9000 - 0x796ba000 (Reserved)
[10999890165] [INFO] [kernel::memory]   [32] 0x796ba000 - 0x796be000 (Other)
[11000734800] [INFO] [kernel::memory]   [33] 0x796be000 - 0x796bf000 (Reserved)
[11001274185] [INFO] [kernel::memory]   [34] 0x796bf000 - 0x796c1000 (Other)
[11001821490] [INFO] [kernel::memory]   [35] 0x796c1000 - 0x796c2000 (Reserved)
[11002395987] [INFO] [kernel::memory]   [36] 0x796c2000 - 0x796c4000 (Other)
[11002944150] [INFO] [kernel::memory]   [37] 0x796c4000 - 0x796c5000 (Reserved)
[11003443968] [INFO] [kernel::memory]   [38] 0x796c5000 - 0x796c7000 (Other)
[11004042621] [INFO] [kernel::memory]   [39] 0x796c7000 - 0x796c8000 (Reserved)
[11004676221] [INFO] [kernel::memory]   [40] 0x796c8000 - 0x796ca000 (Other)
[11005326321] [INFO] [kernel::memory]   [41] 0x796ca000 - 0x796cb000 (Reserved)
[11006123469] [INFO] [kernel::memory]   [42] 0x796cb000 - 0x796cd000 (Other)
[11006920221] [INFO] [kernel::memory]   [43] 0x796cd000 - 0x796ce000 (Reserved)
[11007511251] [INFO] [kernel::memory]   [44] 0x796ce000 - 0x796d8000 (Other)
[11008082547] [INFO] [kernel::memory]   [45] 0x796d8000 - 0x796d9000 (Reserved)
[11008633383] [INFO] [kernel::memory]   [46] 0x796d9000 - 0x796dd000 (Other)
[11008992456] [INFO] [kernel::memory]   [47] 0x796dd000 - 0x796de000 (Reserved)
[11009356149] [INFO] [kernel::memory]   [48] 0x796de000 - 0x796e0000 (Other)
[11009735682] [INFO] [kernel::memory]   [49] 0x796e0000 - 0x796e1000 (Reserved)
[11010213588] [INFO] [kernel::memory]   [50] 0x796e1000 - 0x796e3000 (Other)
[11010569988] [INFO] [kernel::memory]   [51] 0x796e3000 - 0x796e4000 (Reserved)
[11010935199] [INFO] [kernel::memory]   [52] 0x796e4000 - 0x796e8000 (Other)
[11011286352] [INFO] [kernel::memory]   [53] 0x796e8000 - 0x79757000 (Other)
[11011638396] [INFO] [kernel::memory]   [54] 0x79757000 - 0x7990d000 (Other)
[11011990935] [INFO] [kernel::memory]   [55] 0x7990d000 - 0x7a16c000 (Reserved)
[11012354694] [INFO] [kernel::memory]   [56] 0x7a16c000 - 0x7bb6c000 (Usable)
[11012711589] [INFO] [kernel::memory]   [57] 0x7bb6c000 - 0x7bb8d000 (Reserved)
[11013103365] [INFO] [kernel::memory]   [58] 0x7bb8d000 - 0x7bb91000 (Other)
[11013458478] [INFO] [kernel::memory]   [59] 0x7bb91000 - 0x7bb92000 (Reserved)
[11013823029] [INFO] [kernel::memory]   [60] 0x7bb92000 - 0x7bb94000 (Other)
[11014174842] [INFO] [kernel::memory]   [61] 0x7bb94000 - 0x7bb95000 (Reserved)
[11014539756] [INFO] [kernel::memory]   [62] 0x7bb95000 - 0x7bb97000 (Other)
[11014891833] [INFO] [kernel::memory]   [63] 0x7bb97000 - 0x7bb98000 (Reserved)
[11015542527] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[11263246665] [CONTRACT] [kernel::memory] Frame allocator initialized with 495746 free frames
[11273217318] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[11279153556] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[11280365613] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[11281070295] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[11287874301] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[11288353164] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11291190933] [INFO] [bran::arch] IOAPIC: Registers initialized
[11292250893] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[11293827402] [INFO] [bran::arch] IOAPIC: All pins masked
[11295130968] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11295742293] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11296214721] [INFO] [bran::arch] IOAPIC: Init complete
[11296849509] [CONTRACT] [kernel] Initializing global allocator...
[11657355699] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[11658076254] [CONTRACT] [kernel] Initializing SIMD...
[11659476213] [CONTRACT] [kernel] Initializing tasking...
[11664057966] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[11665661799] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[11666258901] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[11672128875] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[11672550153] [INFO] [kernel::task::scheduler]   Initializing boot task...
[11673278760] [INFO] [kernel::task::scheduler]   Creating boot task...
[11677738281] [INFO] [kernel::task::scheduler]   Creating idle task...
[11682559713] [INFO] [kernel::task::scheduler]   Boot task initialized
[11682966570] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[11683813581] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[11689538652] [INFO] [kernel::root] Spawning Root service...
[11697057306] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[11706003144] [INFO] [kernel::root::service] ROOT: started once
[12579075333] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[12579928779] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[12621678630] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[12639359271] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[12667141872] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[12699926118] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[12701681388] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[12737690526] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[12761517186] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[12767793324] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[12770402007] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[12793961367] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[12796261467] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[12797252820] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[12800559420] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[12803108472] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[12803883741] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12811487700] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12823626255] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[12824514153] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[12826987008] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[12827869395] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[12836323269] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[12837053823] [CONTRACT] [kernel] Spawning init process...
[12838647525] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[12873447906] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62392800 ticks/sec), init_cnt=623928 for 100Hz
[12875178558] [CONTRACT] [kernel] Entering scheduler loop.
[12887070009] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[12891874809] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367676144 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563776488
[12902660100] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[12904158432] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[12904954491] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[12905671911] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[12926441517] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[12930324396] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[12933570408] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[12934357755] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[12937889712] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[12940497735] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[12941277492] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[12944346459] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[12945100971] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[12945820239] [INFO] [sprout::devtree] SPROUT: build() called
[12946459251] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[12950880492] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[12951630285] [INFO] [sprout] SPROUT: About to create Supervisor...
[12952281837] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[12953065521] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[12953807262] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[12958432575] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[12994307568] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[12999513087] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[13002759759] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[13006072695] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[13008307521] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[13011168093] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[13014591513] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[13017672426] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[13020824322] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[13023991629] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[13027173291] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[13030788474] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[13034324325] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[13037242053] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[13040585712] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[13043480076] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[13046323587] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[13049308734] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[13052186235] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[13055498016] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[13058517648] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[13061689641] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[13064678319] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[13067659638] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[13071070584] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[13074175818] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[13077258810] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[13080278409] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[13083464097] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[13086514551] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[13089596124] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[13092742014] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[13095751647] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[13098866022] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[13101987096] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[13105420086] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[13108727742] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[13111930425] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[13114967613] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[13118062716] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[13121089146] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[13124091453] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[13127169165] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[13129530216] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[13142144730] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[13213027542] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[13214325861] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[13216187919] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[13217269527] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[13218043674] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[13218871479] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[13221367764] [INFO] [kernel::task::loader] Loading module: /boot/clock
[13221902001] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13222983642] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13226748645] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[13227335253] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[13228650534] [INFO] [kernel::task::loader] Segment: vaddr=204ee8 exec=false
[13229214966] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13236250434] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[13239176940] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[13240207893] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[13240736355] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13242138591] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13246109250] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[13246681404] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[13247493270] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[13248034437] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13253792310] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[13254974337] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[13255952919] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[13256472009] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13257453858] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13267845855] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[13268547501] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[13272888255] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[13273475061] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[13280024142] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[13281054567] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[13282594017] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[13283137527] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13284195408] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13288066374] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[13288693407] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[13290101253] [INFO] [kernel::task::loader] Segment: vaddr=204c88 exec=false
[13290696210] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13297563444] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[13298408607] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[13317301965] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13318703805] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367986928 RFLAGS_BEFORE=134 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563776488
[13323285294] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[13324379541] [INFO] [clock] starting clock publisher
[13335484701] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13336783977] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368009968 RFLAGS_BEFORE=134 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563776488
[13342410840] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ab8
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[13343614581] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368026352 RFLAGS_BEFORE=134 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563776488
[13345679523] [ERROR] [INGESTD] Starting...
[13351054662] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ae8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13352454918] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368053488 RFLAGS_BEFORE=134 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563776488
[13355277045] [INFO] [cambium] cambium starting (v3: catch-up then stream)...
[13362312282] [INFO] [clock] Clock thing created: 356
[13363357293] [INFO] [clock] Waiting for UI Root (Compositor)...
[13369465824] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[13370731572] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[13371884031] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[13373221422] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[13387031394] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[13397889285] [ERROR] [INGESTD] Watch active. Loop start.
[13416777033] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[13418201544] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[13658146689] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[13658906019] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[13660179027] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13663444740] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[13664132130] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13665225486] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[13665860505] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13673621676] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[13680687999] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[13682003148] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368106128 RFLAGS_BEFORE=134 CR3_BEFORE=54927360 fs_base=0 gs_base=18446744071563776488
[13685670636] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[13704536340] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[13713787131] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[13723361817] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[13725370692] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[13726115568] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[13727311191] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13730678247] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[13731332175] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13732713852] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[13733317092] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[13740343650] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[13741811127] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[13743202242] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[13744395852] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[13746011862] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[13747117461] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[13747644999] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13748691726] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13752921567] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[13753626315] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[13754586285] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[13755163686] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[13762444014] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[13763931918] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[13764493314] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13765537698] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13768745100] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[13769348571] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13770653127] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[13771248051] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[13777428621] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[13781250120] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[13782381756] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[13783371888] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13784447325] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13787417193] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[13788010203] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13789013832] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[13789589220] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13796358114] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[13815081258] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044678
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[13816488378] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368129664 RFLAGS_BEFORE=130 CR3_BEFORE=55033856 fs_base=0 gs_base=18446744071563776488
[13820480190] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[13821439962] [INFO] [rtc_cmos] Starting... arg=db
[13823049768] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[13826374881] [INFO] [rtc_cmos] RTC: 2026-01-20 05:37:58 = 1768887478 unix_secs
[13828290531] [INFO] [kernel::time] System clock anchored: unix_secs=1768887478, mono_ns=6913809160, offset=1768887471086190840ns
[13829663628] [INFO] [rtc_cmos] System clock anchored
[13849582230] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[13851924141] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004b958
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[13853084685] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368164192 RFLAGS_BEFORE=134 CR3_BEFORE=55136256 fs_base=0 gs_base=18446744071563776488
[13856709867] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[13858661454] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[13864296831] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[13865709033] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[13871443344] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ee60
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[13872946956] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368181680 RFLAGS_BEFORE=130 CR3_BEFORE=55230464 fs_base=0 gs_base=18446744071563776488
[13876423011] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[13877310513] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[13881199497] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00af048
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[13882472571] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368207552 RFLAGS_BEFORE=134 CR3_BEFORE=55332864 fs_base=0 gs_base=18446744071563776488
[13886165007] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[13895888160] [INFO] [bristle] bristle: registered in graph as svc.Input (id=457)
[13905007413] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=431 backend=BootFB
[13906297878] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[13906837131] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13907944710] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13969883334] [INFO] [kernel::task::loader] Segment: vaddr=2615d0 exec=false
[13971685926] [INFO] [kernel::task::loader]   Overlap at 261000: merging perms to r=true w=false x=true
[13980220980] [INFO] [kernel::task::loader] Segment: vaddr=26d1a0 exec=false
[13981007040] [INFO] [kernel::task::loader]   Overlap at 26d000: merging perms to r=true w=true x=true
[13988341884] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[13989837675] [INFO] [kernel::task::loader] Loading module: /boot/echo
[13990392702] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13991698512] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13994749032] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[13995416292] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13996501629] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[13997134569] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[14039257980] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014c50
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1af
[14040761328] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368250080 RFLAGS_BEFORE=130 CR3_BEFORE=55439360 fs_base=0 gs_base=18446744071563776488
[14044140462] [INFO] [bloom::logging] bloom: logging initialized
[14049438579] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ba00
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[14050752771] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368267632 RFLAGS_BEFORE=130 CR3_BEFORE=55975936 fs_base=0 gs_base=18446744071563776488
[14054430654] [INFO] [echo] echo: online (handle=12)
[14055250407] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[14062092165] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[14064590232] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[14065408137] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[14071737603] [INFO] [bloom] bloom: [bloom] Bootstrapped via Bytespace ID=431
[14078416044] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[14079235038] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[14080234047] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
[14085024228] [INFO] [bloom] bloom: [bloom] starting (arg_req=1 arg_resp=4 bristle=10 font_svc=0)
[14096907594] [INFO] [bloom] bloom: [bloom] spawned wallpaper_loader thread (tid=15)
[14103188550] [INFO] [bloom] bloom: [bloom] spawned cursor_loader thread (tid=16)
[14109275301] [INFO] [bloom] bloom: [bloom] spawned font_loader thread (tid=17)
[14110337076] [INFO] [bloom] bloom: [bloom] discovering compositor target...
T:AEF0 [14116850121] [INFO] [bloom] bloom: [wallpaper_loader] thread started
T:AB20 [14119575921] [INFO] [bloom] bloom: [cursor_loader] thread started
T:A0B0 [14121572652] [INFO] [bloom] bloom: [font_loader] thread started
[14122278027] [INFO] [bloom] bloom: [font_loader] scanning boot modules for fonts...
[14130199974] [INFO] [bloom] bloom: [font_loader] found 42 boot modules
[14221736034] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/DSEG7Classic-Regular.ttf'
[14241585600] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=179 size=23272 name='/assets/fonts/DSEG7Classic-Regular.ttf'
[14273230818] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:35F0 [14280105873] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[14281463592] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[14284540281] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/Hack-Regular.ttf'
[14289533676] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[14290423818] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[14313930180] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[14316776463] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[14324864235] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=182 size=309408 name='/assets/fonts/Hack-Regular.ttf'
[14329249572] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[14331558087] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSans-Regular.ttf'
[14336541615] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[14337633816] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[14655823149] [INFO] [ps2_mouse] ps2_mouse: drained 0x28
[14964020148] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[14965319259] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[14969207022] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=185 size=569208 name='/assets/fonts/NotoSans-Regular.ttf'
[14973861936] [INFO] [ps2_mouse] ps2_mouse: drained 0x32
[14975490849] [INFO] [bloom] bloom: [wallpaper_loader] searching for wallpaper...
[14976463689] [INFO] [bloom] bloom: [wallpaper_loader] trying: /assets/wallpapers/clouds.bmp
[14982514899] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol-Regular.ttf'
[14987492487] [INFO] [ps2_mouse] ps2_mouse: drained 0xce
[14989146315] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[14996117730] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[14996925471] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[15313937232] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=188 size=258156 name='/assets/fonts/NotoSansSymbol-Regular.ttf'
[15476570571] [INFO] [bloom] bloom: [cursor_loader] searching for cursor...
[15477837672] [INFO] [bloom] bloom: [cursor_loader] trying: /assets/cursors/plain/Normal.cur
[18103342257] [INFO] [bloom::compositor] bloom: compositor bytespace 376 (1280x720 stride=5120 format=1)
[20075745624] [INFO] [bloom::compositor] bloom: display backend: BootFB
[20081917449] [INFO] [ps2_mouse] ps2_mouse: init done
[20082637641] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[20083599624] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[20084380602] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[20733459351] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[21401241378] [INFO] [bloom] bloom: [bloom] compositor target: 1280x720 @ 0x10463000 backend=BootFB
[21402948732] [INFO] [bloom] bloom: [bloom] presenter: driver (req=1 resp=4)
[21404579691] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[22048573239] [INFO] [stem::ui] UiBuilder: created root 539
[22049716095] [INFO] [bloom] bloom: [bloom] created UI root node: 539
[22060726281] [INFO] [display_bootfb] display_bootfb: bound bytespace 376
[22391558211] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd5a0
[22392415023] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[22393166532] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22394182965] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[22706116257] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[22709335176] [INFO] [bloom] bloom: [bloom] ui watch opened UNFILTERED (userspace filter pred=UI_TEXT id=511)

```
</details>
