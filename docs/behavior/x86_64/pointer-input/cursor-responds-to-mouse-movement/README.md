# ✅ Scenario: Cursor responds to mouse movement

> Last run: 2026-01-20 18:14:20

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given a cursor is visible on the screen | ✅ | 4325ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | When I move the mouse | ✅ | 619ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the serial log should contain 'CONTRACT: input pointer_move' | ✅ | 374ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And the cursor should move correspondingly on the screen | ✅ | 372ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[9878872047] [CONTRACT] [kernel] thing-os kernel starting...
[9888070137] [INFO] [kernel::memory] Memory map has 64 entries
[9890040732] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[9890663178] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[9890973411] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[9891301563] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[9891608958] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[9891908400] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[9892254669] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[9892555761] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[9892891470] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78656000 (Usable)
[9893209293] [INFO] [kernel::memory]   [9] 0x78656000 - 0x786b8000 (Reserved)
[9893553483] [INFO] [kernel::memory]   [10] 0x786b8000 - 0x7883a000 (Other)
[9893907540] [INFO] [kernel::memory]   [11] 0x7883a000 - 0x7883b000 (Reserved)
[9894238596] [INFO] [kernel::memory]   [12] 0x7883b000 - 0x788d2000 (Other)
[9894558201] [INFO] [kernel::memory]   [13] 0x788d2000 - 0x788d3000 (Reserved)
[9894910146] [INFO] [kernel::memory]   [14] 0x788d3000 - 0x78974000 (Other)
[9895255557] [INFO] [kernel::memory]   [15] 0x78974000 - 0x78975000 (Reserved)
[9895621560] [INFO] [kernel::memory]   [16] 0x78975000 - 0x789b5000 (Other)
[9895947930] [INFO] [kernel::memory]   [17] 0x789b5000 - 0x789b6000 (Reserved)
[9896293143] [INFO] [kernel::memory]   [18] 0x789b6000 - 0x78a41000 (Other)
[9896620206] [INFO] [kernel::memory]   [19] 0x78a41000 - 0x78a42000 (Reserved)
[9896957268] [INFO] [kernel::memory]   [20] 0x78a42000 - 0x78a8e000 (Other)
[9897281163] [INFO] [kernel::memory]   [21] 0x78a8e000 - 0x78a8f000 (Reserved)
[9897640995] [INFO] [kernel::memory]   [22] 0x78a8f000 - 0x78a95000 (Other)
[9897969411] [INFO] [kernel::memory]   [23] 0x78a95000 - 0x78a96000 (Reserved)
[9898304889] [INFO] [kernel::memory]   [24] 0x78a96000 - 0x78f17000 (Other)
[9898628190] [INFO] [kernel::memory]   [25] 0x78f17000 - 0x78f18000 (Reserved)
[9898989012] [INFO] [kernel::memory]   [26] 0x78f18000 - 0x79399000 (Other)
[9899316801] [INFO] [kernel::memory]   [27] 0x79399000 - 0x7939a000 (Reserved)
[9899654952] [INFO] [kernel::memory]   [28] 0x7939a000 - 0x7969b000 (Other)
[9899981850] [INFO] [kernel::memory]   [29] 0x7969b000 - 0x7969c000 (Reserved)
[9900346401] [INFO] [kernel::memory]   [30] 0x7969c000 - 0x796a5000 (Other)
[9900674553] [INFO] [kernel::memory]   [31] 0x796a5000 - 0x796a6000 (Reserved)
[9901013826] [INFO] [kernel::memory]   [32] 0x796a6000 - 0x796aa000 (Other)
[9901340361] [INFO] [kernel::memory]   [33] 0x796aa000 - 0x796ab000 (Reserved)
[9901700127] [INFO] [kernel::memory]   [34] 0x796ab000 - 0x796ad000 (Other)
[9902072895] [INFO] [kernel::memory]   [35] 0x796ad000 - 0x796ae000 (Reserved)
[9902422794] [INFO] [kernel::memory]   [36] 0x796ae000 - 0x796b0000 (Other)
[9902751342] [INFO] [kernel::memory]   [37] 0x796b0000 - 0x796b1000 (Reserved)
[9903089064] [INFO] [kernel::memory]   [38] 0x796b1000 - 0x796b3000 (Other)
[9903416358] [INFO] [kernel::memory]   [39] 0x796b3000 - 0x796b4000 (Reserved)
[9903755631] [INFO] [kernel::memory]   [40] 0x796b4000 - 0x796b6000 (Other)
[9904083156] [INFO] [kernel::memory]   [41] 0x796b6000 - 0x796b7000 (Reserved)
[9904422594] [INFO] [kernel::memory]   [42] 0x796b7000 - 0x796b9000 (Other)
[9904750581] [INFO] [kernel::memory]   [43] 0x796b9000 - 0x796ba000 (Reserved)
[9905088369] [INFO] [kernel::memory]   [44] 0x796ba000 - 0x796c4000 (Other)
[9905434473] [INFO] [kernel::memory]   [45] 0x796c4000 - 0x796c5000 (Reserved)
[9905774043] [INFO] [kernel::memory]   [46] 0x796c5000 - 0x796c9000 (Other)
[9906100941] [INFO] [kernel::memory]   [47] 0x796c9000 - 0x796ca000 (Reserved)
[9906438663] [INFO] [kernel::memory]   [48] 0x796ca000 - 0x796cc000 (Other)
[9906765066] [INFO] [kernel::memory]   [49] 0x796cc000 - 0x796cd000 (Reserved)
[9907101270] [INFO] [kernel::memory]   [50] 0x796cd000 - 0x796cf000 (Other)
[9907426287] [INFO] [kernel::memory]   [51] 0x796cf000 - 0x796d0000 (Reserved)
[9907760841] [INFO] [kernel::memory]   [52] 0x796d0000 - 0x796d4000 (Other)
[9908084736] [INFO] [kernel::memory]   [53] 0x796d4000 - 0x79750000 (Other)
[9908407212] [INFO] [kernel::memory]   [54] 0x79750000 - 0x79907000 (Other)
[9908893599] [INFO] [kernel::memory]   [55] 0x79907000 - 0x7a16c000 (Reserved)
[9909465984] [INFO] [kernel::memory]   [56] 0x7a16c000 - 0x7bb6c000 (Usable)
[9909797931] [INFO] [kernel::memory]   [57] 0x7bb6c000 - 0x7bb8d000 (Reserved)
[9910135752] [INFO] [kernel::memory]   [58] 0x7bb8d000 - 0x7bb91000 (Other)
[9910460274] [INFO] [kernel::memory]   [59] 0x7bb91000 - 0x7bb92000 (Reserved)
[9910796775] [INFO] [kernel::memory]   [60] 0x7bb92000 - 0x7bb94000 (Other)
[9911123343] [INFO] [kernel::memory]   [61] 0x7bb94000 - 0x7bb95000 (Reserved)
[9911465619] [INFO] [kernel::memory]   [62] 0x7bb95000 - 0x7bb97000 (Other)
[9911795520] [INFO] [kernel::memory]   [63] 0x7bb97000 - 0x7bb98000 (Reserved)
[9912358500] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[10138629534] [CONTRACT] [kernel::memory] Frame allocator initialized with 495726 free frames
[10145055855] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[10149434823] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[10150449573] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[10151094591] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[10157146824] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[10157584734] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[10160298225] [INFO] [bran::arch] IOAPIC: Registers initialized
[10161321060] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[10162723758] [INFO] [bran::arch] IOAPIC: All pins masked
[10163952678] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[10164532356] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[10164972510] [INFO] [bran::arch] IOAPIC: Init complete
[10165528692] [CONTRACT] [kernel] Initializing global allocator...
[10480440663] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[10481073174] [CONTRACT] [kernel] Initializing SIMD...
[10482580680] [CONTRACT] [kernel] Initializing tasking...
[10486813986] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[10488250179] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[10489032675] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[10494466389] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[10494854832] [INFO] [kernel::task::scheduler]   Initializing boot task...
[10495589907] [INFO] [kernel::task::scheduler]   Creating boot task...
[10499743122] [INFO] [kernel::task::scheduler]   Creating idle task...
[10504300191] [INFO] [kernel::task::scheduler]   Boot task initialized
[10504674708] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[10505524062] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[10510892205] [INFO] [kernel::root] Spawning Root service...
[10518389475] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[10526858496] [INFO] [kernel::root::service] ROOT: started once
[11333578905] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[11334350940] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[11371789638] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[11387676399] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[11412218004] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[11441422740] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[11442946383] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[11480763855] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[11506260513] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[11512163685] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[11516041218] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[11537344236] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[11539590480] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[11540507715] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[11543531604] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[11545785174] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[11546498502] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[11553396987] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[11564170398] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[11564948967] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[11567118189] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[11567849139] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[11574903846] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[11575502961] [CONTRACT] [kernel] Spawning init process...
[11576900907] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[11611348122] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62181700 ticks/sec), init_cnt=621817 for 100Hz
[11612834211] [CONTRACT] [kernel] Entering scheduler loop.
[11620674450] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[11625030450] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072367676144 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563780584
[11634581805] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[11635785843] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[11636469042] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[11637120231] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[11655288678] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[11658507564] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[11661369357] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[11662048134] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[11665037670] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[11667067797] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[11667804060] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[11670668856] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[11671382382] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[11672070630] [INFO] [sprout::devtree] SPROUT: build() called
[11672649813] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[11676784812] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[11677878069] [INFO] [sprout] SPROUT: About to create Supervisor...
[11678500746] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[11679209421] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[11679806589] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[11684054316] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[11716526514] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[11720614356] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[11723536737] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[11726261415] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[11728307844] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[11730946986] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[11734177950] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[11737063173] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[11739810687] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[11742859623] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[11745791211] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[11749069695] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[11752415202] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[11755241553] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[11757967584] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[11760822810] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[11763545079] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[11766259560] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[11768882268] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[11771933085] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[11774797056] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[11778098541] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[11780865855] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[11783664651] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[11786643924] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[11789464731] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[11792254287] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[11795122218] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[11798122413] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[11800955661] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[11803773762] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[11806710498] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[11809730031] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[11812653336] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[11815442199] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[11818330161] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[11821270923] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[11824200498] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[11827016982] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[11829789774] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[11832644736] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[11835451749] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[11838293577] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[11840333703] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[11851522287] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[11910438078] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[11911290897] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[11913159456] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[11914139160] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[11914884135] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[11915671185] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[11917925613] [INFO] [kernel::task::loader] Loading module: /boot/clock
[11918461236] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[11919418665] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[11922685302] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[11923219968] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[11924437404] [INFO] [kernel::task::loader] Segment: vaddr=204ee8 exec=false
[11924977119] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[11930838282] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[11933413668] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[11934239757] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[11934749838] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[11935958595] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[11939547048] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[11940084816] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[11941073760] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[11941598229] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12001475343] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12015757512] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072367986928 RFLAGS_BEFORE=134 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563780584
[12020596632] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[12021635769] [INFO] [clock] starting clock publisher
[12026075985] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12027127926] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368009968 RFLAGS_BEFORE=134 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563780584
[12031763403] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[12032699547] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[12033649287] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[12034172832] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12035156991] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12044904927] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[12045548988] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[12049237563] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[12049840671] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[12055100409] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[12055847001] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[12056765424] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[12057253263] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12058165317] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12061379715] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[12061950615] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[12063262365] [INFO] [kernel::task::loader] Segment: vaddr=204c88 exec=false
[12063833232] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12069199758] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[12070172829] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[12080780085] [INFO] [clock] Clock thing created: 336
[12081365538] [INFO] [clock] Waiting for UI Root (Compositor)...
[12085382925] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00043c8
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[12086387643] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368037456 RFLAGS_BEFORE=134 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563780584
[12088052691] [ERROR] [INGESTD] Starting...
[12091355661] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb003fc58
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12092438028] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368055088 RFLAGS_BEFORE=134 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563780584
[12094860129] [INFO] [cambium] cambium starting (v3: catch-up then stream)...
[12100060599] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[12101804418] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[12102963807] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[12103936581] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[12105174510] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[12118567956] [ERROR] [INGESTD] Watch active. Loop start.
[12125735952] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[12126835446] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[12327271803] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[12327937776] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[12328932033] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12331624503] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[12332189199] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12334123527] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[12335326608] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[12342303270] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[12347997618] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00043c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[12349049856] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368100640 RFLAGS_BEFORE=130 CR3_BEFORE=54927360 fs_base=0 gs_base=18446744071563780584
[12352189278] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[12367329777] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[12375341385] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[12383227758] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[12384305901] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[12384855021] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[12385851423] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12388790997] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[12389368002] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12390565242] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[12391100634] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[12396501414] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[12397488180] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[12399089010] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[12400087689] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[12401351886] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[12402273246] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[12402754089] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12403648191] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12405766461] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[12406292679] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[12407126160] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[12407640927] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[12413188788] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[12414365568] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[12414868158] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12415795029] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12418552872] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[12419107404] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12420284382] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[12420813966] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[12425967774] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[12427720371] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[12428710206] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[12429187056] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12430050864] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12433224210] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[12433750131] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12434671656] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[12435217245] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[12440836122] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[12453931347] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0043308
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[12454983948] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368124240 RFLAGS_BEFORE=134 CR3_BEFORE=55033856 fs_base=0 gs_base=18446744071563780584
[12458340312] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[12459155577] [INFO] [rtc_cmos] Starting... arg=db
[12460611933] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[12463916025] [INFO] [rtc_cmos] RTC: 2026-01-21 02:15:12 = 1768961712 unix_secs
[12465060993] [INFO] [kernel::time] System clock anchored: unix_secs=1768961712, mono_ns=6232287154, offset=1768961705767712846ns
[12466182366] [INFO] [rtc_cmos] System clock anchored
[12480238287] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[12482240958] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ba00
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[12483344643] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368158848 RFLAGS_BEFORE=130 CR3_BEFORE=55136256 fs_base=0 gs_base=18446744071563780584
[12486453474] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[12488258178] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[12489357870] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[12490124163] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[12494898966] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb008efc0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[12496127622] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368176256 RFLAGS_BEFORE=130 CR3_BEFORE=55230464 fs_base=0 gs_base=18446744071563780584
[12500124087] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[12500919321] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[12504184011] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0096460
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[12505253970] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368212288 RFLAGS_BEFORE=130 CR3_BEFORE=55332864 fs_base=0 gs_base=18446744071563780584
[12508412202] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[12514484532] [INFO] [bristle] bristle: registered in graph as svc.Input (id=457)
[12522124923] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=431 backend=BootFB
[12523150728] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[12523632198] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12524575107] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12582391074] [INFO] [kernel::task::loader] Segment: vaddr=26ea30 exec=false
[12583011540] [INFO] [kernel::task::loader]   Overlap at 26e000: merging perms to r=true w=false x=true
[12590047668] [INFO] [kernel::task::loader] Segment: vaddr=27a9b0 exec=false
[12590649588] [INFO] [kernel::task::loader]   Overlap at 27a000: merging perms to r=true w=true x=true
[12597133329] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[12598323540] [INFO] [kernel::task::loader] Loading module: /boot/echo
[12598869855] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12599813952] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12602511768] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[12603110157] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12604113324] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[12604663995] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[12610009830] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[12610918749] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[12611659863] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[12621268407] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014c50
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1af
[12622309095] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368250064 RFLAGS_BEFORE=130 CR3_BEFORE=55439360 fs_base=0 gs_base=18446744071563780584
[12625510887] [INFO] [bloom::logging] bloom: logging initialized
[12646198785] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ee60
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[12647198124] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368267600 RFLAGS_BEFORE=134 CR3_BEFORE=56033280 fs_base=0 gs_base=18446744071563780584
[12650301015] [INFO] [echo] echo: online (handle=12)
[12650991408] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[12656904645] [INFO] [bloom] bloom: [bloom] Bootstrapped via Bytespace ID=431
[12659793366] [INFO] [bloom] bloom: [bloom] starting (arg_req=1 arg_resp=4 bristle=10 font_svc=0)
[12669042771] [INFO] [bloom] bloom: [bloom] spawned wallpaper_loader thread (tid=15)
[12674392731] [INFO] [bloom] bloom: [bloom] spawned cursor_loader thread (tid=16)
[12679760082] [INFO] [bloom] bloom: [bloom] spawned font_loader thread (tid=17)
[12680470638] [INFO] [bloom] bloom: [bloom] discovering compositor target...
T:E870 [12685157496] [INFO] [bloom] bloom: [wallpaper_loader] thread started
T:E4A0 [12686833467] [INFO] [bloom] bloom: [cursor_loader] thread started
T:DA30 [12688575702] [INFO] [bloom] bloom: [font_loader] thread started
[12689271639] [INFO] [bloom] bloom: [font_loader] scanning boot modules for fonts...
[12694711161] [INFO] [bloom] bloom: [font_loader] found 42 boot modules
[12730737954] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[12731608197] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[12732578661] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
[12785224122] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/DSEG7Classic-Regular.ttf'
[12792185703] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=179 size=23272 name='/assets/fonts/DSEG7Classic-Regular.ttf'
[12798909288] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:FF10 [12803185989] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[12804375507] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[12806945745] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/Hack-Regular.ttf'
[12811240398] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[12812026920] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[12833321523] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[12835776690] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[12863932884] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=182 size=309408 name='/assets/fonts/Hack-Regular.ttf'
[12868691187] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[12870998778] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSans-Regular.ttf'
[12875623728] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[12876726225] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[13125604569] [INFO] [ps2_mouse] ps2_mouse: drained 0x28
[13397356764] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[13398583275] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[13402670061] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=185 size=569208 name='/assets/fonts/NotoSans-Regular.ttf'
[13407415296] [INFO] [ps2_mouse] ps2_mouse: drained 0x32
[13408935969] [INFO] [bloom] bloom: [wallpaper_loader] searching for wallpaper...
[13409801724] [INFO] [bloom] bloom: [wallpaper_loader] trying: /assets/wallpapers/clouds.bmp
[13413682821] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[13416261837] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol-Regular.ttf'
[13420930248] [INFO] [ps2_mouse] ps2_mouse: drained 0xce
[13423663770] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[13424368848] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[13775711235] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=188 size=258156 name='/assets/fonts/NotoSansSymbol-Regular.ttf'
[13781777724] [INFO] [bloom] bloom: [cursor_loader] searching for cursor...
[13782913914] [INFO] [bloom] bloom: [cursor_loader] trying: /assets/cursors/plain/Normal.cur
[14105528976] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[14758504134] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=191 size=656852 name='/assets/fonts/NotoSansSymbol2-Regular.ttf'
[15708883146] [INFO] [bloom::compositor] bloom: compositor bytespace 376 (1280x720 stride=5120 format=1)
[15877325739] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[15878503080] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[16061735976] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (258156 bytes)
[16064991228] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSerif-Regular.ttf'
[16073582679] [INFO] [bloom::asset] [asset_bank] mapped at 0x10463000
[16074522387] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[16402514931] [INFO] [bloom::compositor] bloom: display backend: BootFB
[16411062162] [INFO] [ps2_mouse] ps2_mouse: init done
[16412158587] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[16413474858] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[16414551714] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[16724504181] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=194 size=616196 name='/assets/fonts/NotoSerif-Regular.ttf'
[16728206517] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[17052854643] [INFO] [bloom] bloom: [font_loader] immediate scan complete, entering watch loop
[17062583901] [INFO] [bloom] bloom: [bloom] compositor target: 1280x720 @ 0x104a3000 backend=BootFB
[17063526975] [INFO] [bloom] bloom: [bloom] presenter: driver (req=1 resp=4)
[17064863475] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[17392469292] [INFO] [stem::ui] UiBuilder: created root 549
[17393265087] [INFO] [bloom] bloom: [bloom] created UI root node: 549
[17402487135] [INFO] [display_bootfb] display_bootfb: bound bytespace 376
[17720112597] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd580
[17720920371] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[17721592779] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[17722907367] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[18036563424] [INFO] [bloom] bloom: [bloom] ui watch opened UNFILTERED (userspace filter pred=UI_TEXT id=522)
[18436380897] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[18437482932] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[18438270246] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 191 (656852 bytes)
[18446901198] [INFO] [clock] Found UI Root: 549 (attempt 4)
[18451024185] [INFO] [bloom::asset] [asset_bank] mapped at 0x10bab000
[18451699200] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...

```
</details>
