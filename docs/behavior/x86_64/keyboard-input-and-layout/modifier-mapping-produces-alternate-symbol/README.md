# ✅ Scenario: Modifier mapping produces alternate symbol

> Last run: 2026-01-20 18:14:20

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the clock window is ticking | ✅ | 11578ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I press Alt+A | ✅ | 633ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the serial log should contain 'CONTRACT: input key_event' | ✅ | 379ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And I should see the appropriate symbol rendered | ✅ | 392ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[10157954895] [CONTRACT] [kernel] thing-os kernel starting...
[10167051048] [INFO] [kernel::memory] Memory map has 64 entries
[10169086092] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[10169669169] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[10169989929] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[10170331677] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[10170692631] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[10171415001] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[10171730184] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[10172037678] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[10172461563] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78656000 (Usable)
[10172790111] [INFO] [kernel::memory]   [9] 0x78656000 - 0x786b8000 (Reserved)
[10173128526] [INFO] [kernel::memory]   [10] 0x786b8000 - 0x7883a000 (Other)
[10173464565] [INFO] [kernel::memory]   [11] 0x7883a000 - 0x7883b000 (Reserved)
[10173808293] [INFO] [kernel::memory]   [12] 0x7883b000 - 0x788d2000 (Other)
[10174197924] [INFO] [kernel::memory]   [13] 0x788d2000 - 0x788d3000 (Reserved)
[10174547295] [INFO] [kernel::memory]   [14] 0x788d3000 - 0x78974000 (Other)
[10174880991] [INFO] [kernel::memory]   [15] 0x78974000 - 0x78975000 (Reserved)
[10175227194] [INFO] [kernel::memory]   [16] 0x78975000 - 0x789b5000 (Other)
[10175562276] [INFO] [kernel::memory]   [17] 0x789b5000 - 0x789b6000 (Reserved)
[10175907753] [INFO] [kernel::memory]   [18] 0x789b6000 - 0x78a41000 (Other)
[10176266958] [INFO] [kernel::memory]   [19] 0x78a41000 - 0x78a42000 (Reserved)
[10176613425] [INFO] [kernel::memory]   [20] 0x78a42000 - 0x78a8e000 (Other)
[10176946461] [INFO] [kernel::memory]   [21] 0x78a8e000 - 0x78a8f000 (Reserved)
[10177332792] [INFO] [kernel::memory]   [22] 0x78a8f000 - 0x78a95000 (Other)
[10177668369] [INFO] [kernel::memory]   [23] 0x78a95000 - 0x78a96000 (Reserved)
[10178012955] [INFO] [kernel::memory]   [24] 0x78a96000 - 0x78f17000 (Other)
[10178350446] [INFO] [kernel::memory]   [25] 0x78f17000 - 0x78f18000 (Reserved)
[10178696748] [INFO] [kernel::memory]   [26] 0x78f18000 - 0x79399000 (Other)
[10179030279] [INFO] [kernel::memory]   [27] 0x79399000 - 0x7939a000 (Reserved)
[10179375954] [INFO] [kernel::memory]   [28] 0x7939a000 - 0x7969b000 (Other)
[10179711069] [INFO] [kernel::memory]   [29] 0x7969b000 - 0x7969c000 (Reserved)
[10180059681] [INFO] [kernel::memory]   [30] 0x7969c000 - 0x796a5000 (Other)
[10180389153] [INFO] [kernel::memory]   [31] 0x796a5000 - 0x796a6000 (Reserved)
[10180768290] [INFO] [kernel::memory]   [32] 0x796a6000 - 0x796aa000 (Other)
[10181102184] [INFO] [kernel::memory]   [33] 0x796aa000 - 0x796ab000 (Reserved)
[10181446407] [INFO] [kernel::memory]   [34] 0x796ab000 - 0x796ad000 (Other)
[10181778948] [INFO] [kernel::memory]   [35] 0x796ad000 - 0x796ae000 (Reserved)
[10182125844] [INFO] [kernel::memory]   [36] 0x796ae000 - 0x796b0000 (Other)
[10182463170] [INFO] [kernel::memory]   [37] 0x796b0000 - 0x796b1000 (Reserved)
[10182812409] [INFO] [kernel::memory]   [38] 0x796b1000 - 0x796b3000 (Other)
[10183149042] [INFO] [kernel::memory]   [39] 0x796b3000 - 0x796b4000 (Reserved)
[10183500657] [INFO] [kernel::memory]   [40] 0x796b4000 - 0x796b6000 (Other)
[10183869894] [INFO] [kernel::memory]   [41] 0x796b6000 - 0x796b7000 (Reserved)
[10184215305] [INFO] [kernel::memory]   [42] 0x796b7000 - 0x796b9000 (Other)
[10184549265] [INFO] [kernel::memory]   [43] 0x796b9000 - 0x796ba000 (Reserved)
[10184998725] [INFO] [kernel::memory]   [44] 0x796ba000 - 0x796c4000 (Other)
[10185339021] [INFO] [kernel::memory]   [45] 0x796c4000 - 0x796c5000 (Reserved)
[10185681000] [INFO] [kernel::memory]   [46] 0x796c5000 - 0x796c9000 (Other)
[10186010835] [INFO] [kernel::memory]   [47] 0x796c9000 - 0x796ca000 (Reserved)
[10186348887] [INFO] [kernel::memory]   [48] 0x796ca000 - 0x796cc000 (Other)
[10186677369] [INFO] [kernel::memory]   [49] 0x796cc000 - 0x796cd000 (Reserved)
[10187022252] [INFO] [kernel::memory]   [50] 0x796cd000 - 0x796cf000 (Other)
[10187389212] [INFO] [kernel::memory]   [51] 0x796cf000 - 0x796d0000 (Reserved)
[10187733501] [INFO] [kernel::memory]   [52] 0x796d0000 - 0x796d4000 (Other)
[10188062940] [INFO] [kernel::memory]   [53] 0x796d4000 - 0x79750000 (Other)
[10188393006] [INFO] [kernel::memory]   [54] 0x79750000 - 0x79907000 (Other)
[10188721488] [INFO] [kernel::memory]   [55] 0x79907000 - 0x7a16c000 (Reserved)
[10189063764] [INFO] [kernel::memory]   [56] 0x7a16c000 - 0x7bb6c000 (Usable)
[10189398285] [INFO] [kernel::memory]   [57] 0x7bb6c000 - 0x7bb8d000 (Reserved)
[10189739241] [INFO] [kernel::memory]   [58] 0x7bb8d000 - 0x7bb91000 (Other)
[10190072178] [INFO] [kernel::memory]   [59] 0x7bb91000 - 0x7bb92000 (Reserved)
[10190470587] [INFO] [kernel::memory]   [60] 0x7bb92000 - 0x7bb94000 (Other)
[10190838801] [INFO] [kernel::memory]   [61] 0x7bb94000 - 0x7bb95000 (Reserved)
[10191321822] [INFO] [kernel::memory]   [62] 0x7bb95000 - 0x7bb97000 (Other)
[10193696568] [INFO] [kernel::memory]   [63] 0x7bb97000 - 0x7bb98000 (Reserved)
[10194285321] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[10428565653] [CONTRACT] [kernel::memory] Frame allocator initialized with 495726 free frames
[10435356096] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[10439722194] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[10440826341] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[10441487628] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[10447826730] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[10448393967] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[10451090001] [INFO] [bran::arch] IOAPIC: Registers initialized
[10452101484] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[10453484580] [INFO] [bran::arch] IOAPIC: All pins masked
[10454747226] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[10455326970] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[10455783789] [INFO] [bran::arch] IOAPIC: Init complete
[10456348650] [CONTRACT] [kernel] Initializing global allocator...
[10775801223] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[10776475776] [CONTRACT] [kernel] Initializing SIMD...
[10777902300] [CONTRACT] [kernel] Initializing tasking...
[10782262128] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[10783750032] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[10784290506] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[10789701417] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[10790130318] [INFO] [kernel::task::scheduler]   Initializing boot task...
[10790823615] [INFO] [kernel::task::scheduler]   Creating boot task...
[10795223670] [INFO] [kernel::task::scheduler]   Creating idle task...
[10800162450] [INFO] [kernel::task::scheduler]   Boot task initialized
[10800544557] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[10801364277] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[10806728559] [INFO] [kernel::root] Spawning Root service...
[10813686939] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[10822106163] [INFO] [kernel::root::service] ROOT: started once
[11631697638] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[11632500924] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[11672114421] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[11688811398] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[11713262385] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[11743366635] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[11745103491] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[11779595355] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[11800423866] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[11806290474] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[11808601728] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[11829293685] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[11831651964] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[11832556857] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[11835637341] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[11837925099] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[11838634467] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[11845593804] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[11856491691] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[11857367478] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[11859523401] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[11860292433] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[11867254905] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[11867829171] [CONTRACT] [kernel] Spawning init process...
[11869290279] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[11903758779] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62118300 ticks/sec), init_cnt=621183 for 100Hz
[11905099635] [CONTRACT] [kernel] Entering scheduler loop.
[11912839356] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[11917301286] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072367676144 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563780584
[11927046318] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[11928289131] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[11928992889] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[11929695921] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[11947816716] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[11951109951] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[11953910760] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[11954612472] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[11957655798] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[11959743279] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[11960454924] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[11963281209] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[11963979159] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[11964647541] [INFO] [sprout::devtree] SPROUT: build() called
[11965242168] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[11969871804] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[11970581139] [INFO] [sprout] SPROUT: About to create Supervisor...
[11971199130] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[11971917441] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[11972568927] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[11976822330] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[12009468372] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[12013500939] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[12016563240] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[12019415463] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[12021443181] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[12024056418] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[12027219798] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[12030058227] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[12033045981] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[12036182466] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[12039052641] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[12042320763] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[12045602415] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[12048332604] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[12051037614] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[12053832483] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[12056525052] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[12059263524] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[12061917912] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[12064917414] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[12067995258] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[12071145867] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[12073885164] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[12076667625] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[12079582119] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[12082417314] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[12085246008] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[12088063746] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[12090988536] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[12093777135] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[12096681597] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[12099576951] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[12102667929] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[12105514707] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[12108338814] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[12111264297] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[12114269508] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[12117168525] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[12119990091] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[12123022428] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[12125885409] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[12128731791] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[12131621898] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[12133876854] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[12144883806] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[12204612090] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[12205470618] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[12207249285] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[12208271790] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[12209022177] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[12209843877] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[12212216082] [INFO] [kernel::task::loader] Loading module: /boot/clock
[12212766423] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12213752958] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12217326066] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[12217875747] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[12219146511] [INFO] [kernel::task::loader] Segment: vaddr=204ee8 exec=false
[12219695169] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12226135449] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[12228719745] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[12229625397] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[12230122410] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12231421752] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12235207710] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[12235754916] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[12236562426] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[12237105276] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12297982092] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12312808332] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072367986928 RFLAGS_BEFORE=134 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563780584
[12317502120] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[12318675666] [INFO] [clock] starting clock publisher
[12323249598] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12324298437] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368009968 RFLAGS_BEFORE=134 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563780584
[12328961238] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[12330156762] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[12331159632] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[12331740762] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12332736669] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12342786027] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[12343349931] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[12347390979] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[12347988015] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[12353538054] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[12354271842] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[12355226400] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[12355739913] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12356687145] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12360199104] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[12360776340] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[12362182734] [INFO] [kernel::task::loader] Segment: vaddr=204c88 exec=false
[12362772675] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12368423529] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[12369128310] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[12379706031] [INFO] [clock] Clock thing created: 336
[12380328972] [INFO] [clock] Waiting for UI Root (Compositor)...
[12384377445] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00043c8
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[12385371537] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368037456 RFLAGS_BEFORE=134 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563780584
[12387053250] [ERROR] [INGESTD] Starting...
[12390305004] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb003fc58
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12391394829] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368055088 RFLAGS_BEFORE=134 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563780584
[12393914511] [INFO] [cambium] cambium starting (v3: catch-up then stream)...
[12400701192] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[12402466659] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[12403582851] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[12404558694] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[12405759927] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[12417564555] [ERROR] [INGESTD] Watch active. Loop start.
[12424606458] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[12425707338] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[12625122576] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[12625802871] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[12626858904] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12629649681] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[12630210615] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12631162698] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[12631710531] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[12637737090] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[12643256637] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00043c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[12644320227] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368100640 RFLAGS_BEFORE=130 CR3_BEFORE=54927360 fs_base=0 gs_base=18446744071563780584
[12647437374] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[12663162171] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[12671184273] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[12678787803] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[12679901223] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[12680430147] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[12681453081] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12684490005] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[12685047573] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12686340117] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[12686885937] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[12692937213] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[12694006776] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[12695260248] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[12696332715] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[12697662351] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[12698604171] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[12699102240] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12700006572] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12702518730] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[12703062867] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[12703976274] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[12704527935] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[12710306895] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[12711508194] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[12712024314] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12712975803] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12715896435] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[12716437206] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12717641706] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[12718174161] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[12724177158] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[12725912892] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[12726939687] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[12727436304] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12728328591] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12731603775] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[12732115011] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12733017231] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[12733568793] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[12739259709] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[12752637150] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014318
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[12753791820] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368124288 RFLAGS_BEFORE=130 CR3_BEFORE=55033856 fs_base=0 gs_base=18446744071563780584
[12757708854] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[12758541312] [INFO] [rtc_cmos] Starting... arg=db
[12760013013] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[12763010403] [INFO] [rtc_cmos] RTC: 2026-01-21 02:14:58 = 1768961698 unix_secs
[12764117223] [INFO] [kernel::time] System clock anchored: unix_secs=1768961698, mono_ns=6381837907, offset=1768961691618162093ns
[12765268758] [INFO] [rtc_cmos] System clock anchored
[12779582277] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[12781544754] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[12782484396] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368158976 RFLAGS_BEFORE=134 CR3_BEFORE=55136256 fs_base=0 gs_base=18446744071563780584
[12785500464] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[12787326453] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[12789598008] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[12790393011] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[12795005850] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004b958
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[12796217412] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368176384 RFLAGS_BEFORE=134 CR3_BEFORE=55230464 fs_base=0 gs_base=18446744071563780584
[12799240608] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[12800056302] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[12803499753] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb008efc0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[12804582450] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368212416 RFLAGS_BEFORE=134 CR3_BEFORE=55332864 fs_base=0 gs_base=18446744071563780584
[12808008510] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[12814264782] [INFO] [bristle] bristle: registered in graph as svc.Input (id=457)
[12822342192] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=431 backend=BootFB
[12823428090] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[12823921143] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12824837619] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12889879431] [INFO] [kernel::task::loader] Segment: vaddr=26ea30 exec=false
[12890475246] [INFO] [kernel::task::loader]   Overlap at 26e000: merging perms to r=true w=false x=true
[12898393860] [INFO] [kernel::task::loader] Segment: vaddr=27a9b0 exec=false
[12898958622] [INFO] [kernel::task::loader]   Overlap at 27a000: merging perms to r=true w=true x=true
[12905366430] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[12906616041] [INFO] [kernel::task::loader] Loading module: /boot/echo
[12907147011] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12908088831] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12910774866] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[12911522679] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12912518124] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[12913081467] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[12919592697] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[12920511054] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[12921267249] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[12931268823] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1af
[12932335350] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368250112 RFLAGS_BEFORE=134 CR3_BEFORE=55439360 fs_base=0 gs_base=18446744071563780584
[12935018547] [INFO] [bloom::logging] bloom: logging initialized
[12956196924] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb008efc0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[12957294834] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368267584 RFLAGS_BEFORE=130 CR3_BEFORE=56033280 fs_base=0 gs_base=18446744071563780584
[12960352548] [INFO] [echo] echo: online (handle=12)
[12961117158] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[12966817776] [INFO] [bloom] bloom: [bloom] Bootstrapped via Bytespace ID=431
[12969626241] [INFO] [bloom] bloom: [bloom] starting (arg_req=1 arg_resp=4 bristle=10 font_svc=0)
[12978165552] [INFO] [bloom] bloom: [bloom] spawned wallpaper_loader thread (tid=15)
[12983434926] [INFO] [bloom] bloom: [bloom] spawned cursor_loader thread (tid=16)
[12989377698] [INFO] [bloom] bloom: [bloom] spawned font_loader thread (tid=17)
[12990155904] [INFO] [bloom] bloom: [bloom] discovering compositor target...
T:E870 [12994764024] [INFO] [bloom] bloom: [wallpaper_loader] thread started
T:E4A0 [12996397095] [INFO] [bloom] bloom: [cursor_loader] thread started
[12999735969] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[13000542324] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[13001445501] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
T:DA30 [13004424543] [INFO] [bloom] bloom: [font_loader] thread started
[13005144537] [INFO] [bloom] bloom: [font_loader] scanning boot modules for fonts...
[13010464269] [INFO] [bloom] bloom: [font_loader] found 42 boot modules
[13092294039] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/DSEG7Classic-Regular.ttf'
[13098625980] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=179 size=23272 name='/assets/fonts/DSEG7Classic-Regular.ttf'
[13105472391] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:FF10 [13109661114] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[13110771399] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[13113407703] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/Hack-Regular.ttf'
[13118021367] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[13118861283] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[13139533770] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[13141929966] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[13148651439] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=182 size=309408 name='/assets/fonts/Hack-Regular.ttf'
[13152649785] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[13154751687] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSans-Regular.ttf'
[13158803955] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[13159552857] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[13659151539] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[13660291623] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[13663392171] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=185 size=569208 name='/assets/fonts/NotoSans-Regular.ttf'
[13667911950] [INFO] [bloom] bloom: [wallpaper_loader] searching for wallpaper...
[13668816018] [INFO] [bloom] bloom: [wallpaper_loader] trying: /assets/wallpapers/clouds.bmp
[13673164956] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[13675380411] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol-Regular.ttf'
[13681198047] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[13681993017] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[14132536638] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=188 size=258156 name='/assets/fonts/NotoSansSymbol-Regular.ttf'
[14138544684] [INFO] [bloom] bloom: [cursor_loader] searching for cursor...
[14139393411] [INFO] [bloom] bloom: [cursor_loader] trying: /assets/cursors/plain/Normal.cur
[14472415980] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[15113084877] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=191 size=656852 name='/assets/fonts/NotoSansSymbol2-Regular.ttf'
[16062430098] [INFO] [bloom::compositor] bloom: compositor bytespace 376 (1280x720 stride=5120 format=1)
[17002501923] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[17003807205] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[17057652381] [INFO] [ps2_mouse] ps2_mouse: init done
[17058416067] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[17059326372] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[17060037621] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[17063431869] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (258156 bytes)
[17066117442] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSerif-Regular.ttf'
[17068388007] [INFO] [bloom::compositor] bloom: display backend: BootFB
[17073254913] [INFO] [bloom::asset] [asset_bank] mapped at 0x10463000
[17073981210] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[17374648005] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[17699202939] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=194 size=616196 name='/assets/fonts/NotoSerif-Regular.ttf'
[17708157456] [INFO] [bloom] bloom: [bloom] compositor target: 1280x720 @ 0x104a3000 backend=BootFB
[17709246951] [INFO] [bloom] bloom: [bloom] presenter: driver (req=1 resp=4)
[17710660011] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[18043492170] [INFO] [bloom] bloom: [font_loader] immediate scan complete, entering watch loop
[18046656144] [INFO] [stem::ui] UiBuilder: created root 545
[18047468472] [INFO] [bloom] bloom: [bloom] created UI root node: 545
[18057265314] [INFO] [display_bootfb] display_bootfb: bound bytespace 376
[18387895185] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd580
[18388807536] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[18389455524] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18390340419] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[18715702632] [INFO] [bloom] bloom: [bloom] ui watch opened UNFILTERED (userspace filter pred=UI_TEXT id=515)
[19049314944] [INFO] [clock] Found UI Root: 545 (attempt 4)
[20066787873] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[20067740319] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[20068624191] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 191 (656852 bytes)
[20078742387] [INFO] [bloom::asset] [asset_bank] mapped at 0x10bab000
[20079392091] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[22314628875] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c1b40
[22315508589] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[22316268249] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22316988309] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[22652406579] [INFO] [bloom] bloom: [font_loader] watch opened (id=572)
[24282616479] [INFO] [bloom] bloom: [cursor_loader] SUCCESS: found candidate '/assets/cursors/plain/Normal.cur', enqueuing load
[24284017296] [INFO] [bloom] bloom: [cursor_loader] thread done, sleeping forever
[25272099963] [INFO] [bloom] bloom: [wallpaper_loader] SUCCESS: found candidate '/assets/wallpapers/clouds.bmp', enqueuing load
[25273475205] [INFO] [bloom] bloom: [wallpaper_loader] thread done, sleeping forever
[26409681243] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[26410781100] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol2-Regular.ttf' in slot 4
[26411734668] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 194 (616196 bytes)
[26420173725] [INFO] [bloom::asset] [asset_bank] mapped at 0x10c4c000
[26420953053] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSerif-Regular.ttf'...
[27886036794] [INFO] [cambium] Found 1 bindings
[28543497147] [INFO] [clock] Binding created: 590 (source=336 target=575)
[28544408409] [INFO] [clock] CLOCK: Entering main loop, publishing to thing_id=336
[28547054349] [INFO] [clock] unix=1768961705 utc=2026-01-21 02:15:05 mono_ns=14272723030
[28900464417] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fedd0
[28901581071] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[28902397062] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28903228992] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x4 kind=0 pred=0 subj_lo=336
[29228780823] [INFO] [cambium] Opened watch 605 for source 336 (binding 590, start_seq=0)
[29250253824] [INFO] [cambium] CATCH-UP: Draining 1 watches for historical events...
[29585030508] [INFO] [echo] KeyDown LAlt +Alt
[29915703939] [INFO] [clock] CLOCK PUBLISH: thing=336 now_text='02:15:05' tick=14272723030
[31876437384] [INFO] [echo] KeyDown A +Alt
[31880046165] [INFO] [cambium] cambium: drain complete payloads=0 overflows=7 last_seq=none
[31880943765] [INFO] [cambium] Entering event loop with 1 bindings (v3)
[32204695347] [INFO] [echo] KeyUp A +Alt
[33188255364] [INFO] [clock] unix=1768961708 utc=2026-01-21 02:15:08 mono_ns=16593945901
[33398793021] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[33400292079] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSerif-Regular.ttf' in slot 5
[33401345373] [INFO] [bloom::asset] [asset_bank] load_cursor_immediate: /assets/cursors/plain/Normal.cur
[33427602615] [INFO] [clock] CLOCK PUBLISH: thing=336 now_text='02:15:08' tick=16593945901
[33522469101] [INFO] [bloom::asset] [asset_bank] mapping bytespace 152 (4286 bytes)
[33526980069] [INFO] [bloom::asset] [asset_bank] mapped to 0x10ce3000
[33527665248] [INFO] [bloom::asset] [asset_bank] checking CUR header: len=4286
[33528394614] [INFO] [bloom::asset] [asset_bank] valid CUR header detected
[33529243968] [INFO] [bloom::asset] [asset_bank] CUR: hotspot=(0, 0), img_size=4264, offset=22
[33530005377] [INFO] [bloom::asset] [asset_bank] decoding embedded DIB at offset 22...
[33531943269] [INFO] [bloom::asset] [asset_bank] SUCCESS: cursor DIB decoded 32x32
[33538931712] [INFO] [bloom::asset] [asset_bank] publish_cursor (pending)
[33539683254] [INFO] [bloom::asset] [asset_bank] cursor: Static frame 32x32 hotspot (0, 0)
[33540569337] [INFO] [bloom::asset] [asset_bank] load_wallpaper_immediate: /assets/wallpapers/clouds.bmp
[33657329871] [INFO] [bloom::asset] [asset_bank] mapping bytespace 170 (3145782 bytes)
[33666179613] [INFO] [bloom::asset] [asset_bank] mapped to 0x10ce5000
[33666879510] [INFO] [bloom::asset] [asset_bank] decoding BMP...
[34017605094] [INFO] [bloom::asset] [asset_bank] BMP decoded: 1024x1024
[34409605560] [INFO] [bloom::asset] [asset_bank] bytespace unmapped
[34410502401] [INFO] [bloom::asset] [asset_bank] publish_wallpaper (pending): 1024x1024
[36047399817] [INFO] [cambium] [cambium] write: binding_src=336 target=575 pred=ui.Text(515) val=599 seq=1141
[36052737963] [INFO] [cambium] Updated target 575 with value 599 (seq=1141)
[36068673564] [INFO] [cambium] Updated target 575 with value 620 (seq=1153)
[36819620145] [INFO] [bloom] bloom: [font_loader] drain complete: payloads=1042 overflows=27
[36826444974] [INFO] [bloom] bloom: [bloom] ui watch drained: 1046 batches
[36862392105] [INFO] [bloom] bloom: [bloom] entering transactional frame loop (acquire -> build -> present)
[36863541759] [INFO] [bloom] bloom: [bloom] reclaimer: budget=33554432 bytes
[36864597957] [INFO] [bloom::reclaimer] [reclaimer] +4194304 bytes (total: 4194304)
[36865315377] [INFO] [bloom::asset] [asset_bank] promoting wallpaper to gen=1 (4194304b)
[36866250927] [INFO] [bloom::reclaimer] [reclaimer] +4096 bytes (total: 4198400)
[36867002007] [INFO] [bloom::asset] [asset_bank] promoting cursor to gen=1 (4096b)
[36868023258] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4300800)
[36868776219] [INFO] [bloom::asset] [asset_bank] promoting font 'DSEG7Classic-Regular.ttf' to gen=1 (102400b) in slot 0
[36869736090] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4403200)
[36870442158] [INFO] [bloom::asset] [asset_bank] promoting font 'Hack-Regular.ttf' to gen=1 (102400b) in slot 1
[36871202742] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4505600)
[36871833108] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=1 (102400b) in slot 2
[36872602833] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4608000)
[36873235641] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol-Regular.ttf' to gen=1 (102400b) in slot 3
[36874073115] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4710400)
[36874713843] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol2-Regular.ttf' to gen=1 (102400b) in slot 4
[36875521419] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4812800)
[36876173697] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSerif-Regular.ttf' to gen=1 (102400b) in slot 5
[36877769742] [INFO] [bloom] bloom: [bloom] frame 1: wallpaper now visible (gen=1)
[36878775285] [INFO] [bloom] bloom: [bloom] frame 1: cursor now visible (gen=1)
[36883103730] [INFO] [bloom] bloom: [bloom] frame 1: fonts available (mode=legacy)
[36928475958] [INFO] [bloom::ui] bloom: [bloom][ui] Initializing cached UI symbols (one-time)

```
</details>
