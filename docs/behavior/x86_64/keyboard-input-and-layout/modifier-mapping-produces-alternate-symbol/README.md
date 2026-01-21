# ✅ Scenario: Modifier mapping produces alternate symbol

> Last run: 2026-01-19 21:36:59

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the clock window is ticking | ✅ | 10891ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I press Alt+A | ✅ | 662ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the serial log should contain 'CONTRACT: input key_event' | ✅ | 404ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And I should see the appropriate symbol rendered | ✅ | 413ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11156856810] [CONTRACT] [kernel] thing-os kernel starting...
[11166523104] [INFO] [kernel::memory] Memory map has 64 entries
[11169938670] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[11170920981] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[11171421789] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[11171941209] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[11172285300] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[11172721923] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[11173516068] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[11173850292] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[11174248701] [INFO] [kernel::memory]   [8] 0x1780000 - 0x7866a000 (Usable)
[11174598897] [INFO] [kernel::memory]   [9] 0x7866a000 - 0x786cc000 (Reserved)
[11174960874] [INFO] [kernel::memory]   [10] 0x786cc000 - 0x7884e000 (Other)
[11175313281] [INFO] [kernel::memory]   [11] 0x7884e000 - 0x7884f000 (Reserved)
[11175679482] [INFO] [kernel::memory]   [12] 0x7884f000 - 0x788e6000 (Other)
[11176062183] [INFO] [kernel::memory]   [13] 0x788e6000 - 0x788e7000 (Reserved)
[11176429077] [INFO] [kernel::memory]   [14] 0x788e7000 - 0x78988000 (Other)
[11176782045] [INFO] [kernel::memory]   [15] 0x78988000 - 0x78989000 (Reserved)
[11177151150] [INFO] [kernel::memory]   [16] 0x78989000 - 0x789c9000 (Other)
[11177506428] [INFO] [kernel::memory]   [17] 0x789c9000 - 0x789ca000 (Reserved)
[11177873949] [INFO] [kernel::memory]   [18] 0x789ca000 - 0x78a55000 (Other)
[11178228699] [INFO] [kernel::memory]   [19] 0x78a55000 - 0x78a56000 (Reserved)
[11178596055] [INFO] [kernel::memory]   [20] 0x78a56000 - 0x78aa2000 (Other)
[11178950739] [INFO] [kernel::memory]   [21] 0x78aa2000 - 0x78aa3000 (Reserved)
[11179356573] [INFO] [kernel::memory]   [22] 0x78aa3000 - 0x78aa9000 (Other)
[11179711818] [INFO] [kernel::memory]   [23] 0x78aa9000 - 0x78aaa000 (Reserved)
[11180077095] [INFO] [kernel::memory]   [24] 0x78aaa000 - 0x78f2b000 (Other)
[11180429634] [INFO] [kernel::memory]   [25] 0x78f2b000 - 0x78f2c000 (Reserved)
[11180795208] [INFO] [kernel::memory]   [26] 0x78f2c000 - 0x793ad000 (Other)
[11181146823] [INFO] [kernel::memory]   [27] 0x793ad000 - 0x793ae000 (Reserved)
[11181511605] [INFO] [kernel::memory]   [28] 0x793ae000 - 0x796af000 (Other)
[11181866190] [INFO] [kernel::memory]   [29] 0x796af000 - 0x796b0000 (Reserved)
[11182231995] [INFO] [kernel::memory]   [30] 0x796b0000 - 0x796b9000 (Other)
[11182611726] [INFO] [kernel::memory]   [31] 0x796b9000 - 0x796ba000 (Reserved)
[11182974297] [INFO] [kernel::memory]   [32] 0x796ba000 - 0x796be000 (Other)
[11183323734] [INFO] [kernel::memory]   [33] 0x796be000 - 0x796bf000 (Reserved)
[11183685315] [INFO] [kernel::memory]   [34] 0x796bf000 - 0x796c1000 (Other)
[11184034125] [INFO] [kernel::memory]   [35] 0x796c1000 - 0x796c2000 (Reserved)
[11184396927] [INFO] [kernel::memory]   [36] 0x796c2000 - 0x796c4000 (Other)
[11184743955] [INFO] [kernel::memory]   [37] 0x796c4000 - 0x796c5000 (Reserved)
[11185106229] [INFO] [kernel::memory]   [38] 0x796c5000 - 0x796c7000 (Other)
[11185453950] [INFO] [kernel::memory]   [39] 0x796c7000 - 0x796c8000 (Reserved)
[11185858035] [INFO] [kernel::memory]   [40] 0x796c8000 - 0x796ca000 (Other)
[11186293602] [INFO] [kernel::memory]   [41] 0x796ca000 - 0x796cb000 (Reserved)
[11186662047] [INFO] [kernel::memory]   [42] 0x796cb000 - 0x796cd000 (Other)
[11187058179] [INFO] [kernel::memory]   [43] 0x796cd000 - 0x796ce000 (Reserved)
[11187435897] [INFO] [kernel::memory]   [44] 0x796ce000 - 0x796d8000 (Other)
[11187786687] [INFO] [kernel::memory]   [45] 0x796d8000 - 0x796d9000 (Reserved)
[11188150908] [INFO] [kernel::memory]   [46] 0x796d9000 - 0x796dd000 (Other)
[11188500114] [INFO] [kernel::memory]   [47] 0x796dd000 - 0x796de000 (Reserved)
[11188863114] [INFO] [kernel::memory]   [48] 0x796de000 - 0x796e0000 (Other)
[11189233869] [INFO] [kernel::memory]   [49] 0x796e0000 - 0x796e1000 (Reserved)
[11189594229] [INFO] [kernel::memory]   [50] 0x796e1000 - 0x796e3000 (Other)
[11189945316] [INFO] [kernel::memory]   [51] 0x796e3000 - 0x796e4000 (Reserved)
[11190306765] [INFO] [kernel::memory]   [52] 0x796e4000 - 0x796e8000 (Other)
[11190655575] [INFO] [kernel::memory]   [53] 0x796e8000 - 0x79757000 (Other)
[11191002834] [INFO] [kernel::memory]   [54] 0x79757000 - 0x7990d000 (Other)
[11191351908] [INFO] [kernel::memory]   [55] 0x7990d000 - 0x7a16c000 (Reserved)
[11191715106] [INFO] [kernel::memory]   [56] 0x7a16c000 - 0x7bb6c000 (Usable)
[11192068206] [INFO] [kernel::memory]   [57] 0x7bb6c000 - 0x7bb8d000 (Reserved)
[11192465559] [INFO] [kernel::memory]   [58] 0x7bb8d000 - 0x7bb91000 (Other)
[11192816382] [INFO] [kernel::memory]   [59] 0x7bb91000 - 0x7bb92000 (Reserved)
[11193178458] [INFO] [kernel::memory]   [60] 0x7bb92000 - 0x7bb94000 (Other)
[11193528423] [INFO] [kernel::memory]   [61] 0x7bb94000 - 0x7bb95000 (Reserved)
[11193893964] [INFO] [kernel::memory]   [62] 0x7bb95000 - 0x7bb97000 (Other)
[11194251750] [INFO] [kernel::memory]   [63] 0x7bb97000 - 0x7bb98000 (Reserved)
[11194871457] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[11441298891] [CONTRACT] [kernel::memory] Frame allocator initialized with 495746 free frames
[11448395508] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[11453182983] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[11454271125] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[11454961419] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[11461694937] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[11462170863] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11465173995] [INFO] [bran::arch] IOAPIC: Registers initialized
[11466264150] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[11467818087] [INFO] [bran::arch] IOAPIC: All pins masked
[11469205671] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11469910551] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11470393935] [INFO] [bran::arch] IOAPIC: Init complete
[11471083305] [CONTRACT] [kernel] Initializing global allocator...
[11839189461] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[11839899258] [CONTRACT] [kernel] Initializing SIMD...
[11841340533] [CONTRACT] [kernel] Initializing tasking...
[11846107944] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[11847678513] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[11848277859] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[11853975045] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[11854408929] [INFO] [kernel::task::scheduler]   Initializing boot task...
[11855160372] [INFO] [kernel::task::scheduler]   Creating boot task...
[11859565806] [INFO] [kernel::task::scheduler]   Creating idle task...
[11864297379] [INFO] [kernel::task::scheduler]   Boot task initialized
[11864728821] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[11865576624] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[11871287439] [INFO] [kernel::root] Spawning Root service...
[11878907106] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[11887813245] [INFO] [kernel::root::service] ROOT: started once
[12782230065] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[12783076053] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[12824726376] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[12841887399] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[12868853976] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[12901158666] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[12903147642] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[12942352401] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[12966552324] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[12974605875] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[12977508258] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[13001420817] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[13004528889] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[13005559413] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[13009096287] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[13011590658] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[13012364508] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13019879829] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13032896877] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[13033871301] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[13036660560] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[13037568060] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[13046168058] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[13046899041] [CONTRACT] [kernel] Spawning init process...
[13048477596] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[13083185775] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62183900 ticks/sec), init_cnt=621839 for 100Hz
[13084861911] [CONTRACT] [kernel] Entering scheduler loop.
[13094810784] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[13099599876] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367676144 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563776488
[13110229143] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[13111647549] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[13112438922] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[13113410178] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[13134466059] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[13138257495] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[13141687845] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[13142529048] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[13146307680] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[13149504456] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[13150591971] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[13153809471] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[13154582067] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[13155310971] [INFO] [sprout::devtree] SPROUT: build() called
[13156025256] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[13160449995] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[13161207246] [INFO] [sprout] SPROUT: About to create Supervisor...
[13161868929] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[13162766892] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[13163436297] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[13168176417] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[13208937852] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[13214035527] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[13217535936] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[13220601900] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[13223065614] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[13226199426] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[13229928195] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[13233178068] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[13236263040] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[13239424242] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[13242562641] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[13246063281] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[13250305695] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[13253704926] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[13257020007] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[13260152202] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[13263240804] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[13266264924] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[13269114243] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[13272604092] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[13275807930] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[13279378431] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[13283045259] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[13286250945] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[13289572560] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[13292755641] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[13295941791] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[13299243705] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[13302522090] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[13305744573] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[13309812120] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[13313505249] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[13316612892] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[13319763336] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[13322889195] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[13326196752] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[13329582915] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[13332941292] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[13336249311] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[13339562874] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[13342793178] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[13346219799] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[13349452017] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[13351761159] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[13364454642] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[13436876772] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[13437835686] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[13439747343] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[13440787074] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[13441613097] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[13442639826] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[13445460336] [INFO] [kernel::task::loader] Loading module: /boot/clock
[13446035757] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13447164918] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13451202336] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[13451814849] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[13453220715] [INFO] [kernel::task::loader] Segment: vaddr=204ee8 exec=false
[13453820820] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13461485532] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[13464352209] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[13465353693] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[13465885389] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13467263205] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13471901553] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[13472524923] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[13473334380] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[13473873897] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13480499241] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[13481801784] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[13482786570] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[13483286289] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13484282823] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13496278785] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[13497022836] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[13501778367] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[13502379330] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[13508898645] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[13509913824] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[13511412090] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[13511939463] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13512961770] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13516948632] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[13517546658] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[13519205139] [INFO] [kernel::task::loader] Segment: vaddr=204c88 exec=false
[13519795278] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13526882358] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[13527919812] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[13544880030] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13546181187] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367986928 RFLAGS_BEFORE=134 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563776488
[13550207088] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[13551412941] [INFO] [clock] starting clock publisher
[13561657098] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13562869650] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368009968 RFLAGS_BEFORE=134 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563776488
[13567954323] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ab8
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[13569214923] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368026352 RFLAGS_BEFORE=134 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563776488
[13571269965] [ERROR] [INGESTD] Starting...
[13574978703] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ae8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13576144428] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368053488 RFLAGS_BEFORE=134 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563776488
[13579198050] [INFO] [cambium] cambium starting (v3: catch-up then stream)...
[13585832205] [INFO] [clock] Clock thing created: 356
[13586552529] [INFO] [clock] Waiting for UI Root (Compositor)...
[13591531668] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[13592734221] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[13593837807] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[13595168631] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[13618142439] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[13627084449] [ERROR] [INGESTD] Watch active. Loop start.
[13635226968] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[13636507566] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[13869326889] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[13870246434] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[13871882475] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13875131193] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[13875793371] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13876874682] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[13877524815] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13885295094] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[13892564763] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[13893825495] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368106128 RFLAGS_BEFORE=134 CR3_BEFORE=54927360 fs_base=0 gs_base=18446744071563776488
[13897464768] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[13915784190] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[13924825761] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[13937270853] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[13938656424] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[13939227192] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[13940670117] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13944101787] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[13944883722] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13946401524] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[13947021462] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[13954970238] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[13956352344] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[13957763985] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[13959208593] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[13960697355] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[13961739165] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[13962262644] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13963374018] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13965807867] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[13966817733] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[13967758728] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[13968433644] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[13975684338] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[13977090765] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[13977652524] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13978705257] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13983596187] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[13984865334] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13986232359] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[13986841209] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[13993548789] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[13995404544] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[13996525125] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[13997053587] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13998101205] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14002381041] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[14003036619] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14004085128] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[14004696123] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[14012587017] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[14028927759] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044678
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[14030262873] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368129664 RFLAGS_BEFORE=130 CR3_BEFORE=55033856 fs_base=0 gs_base=18446744071563776488
[14034721272] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[14036015697] [INFO] [rtc_cmos] Starting... arg=db
[14038335234] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[14041990908] [INFO] [rtc_cmos] RTC: 2026-01-20 05:37:42 = 1768887462 unix_secs
[14043246261] [INFO] [kernel::time] System clock anchored: unix_secs=1768887462, mono_ns=7021372561, offset=1768887454978627439ns
[14044521711] [INFO] [rtc_cmos] System clock anchored
[14062052862] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[14065311216] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004b958
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[14066722131] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368164240 RFLAGS_BEFORE=134 CR3_BEFORE=55136256 fs_base=0 gs_base=18446744071563776488
[14073890391] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[14075959986] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[14077173462] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[14078061558] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[14083816593] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00149f0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[14085214011] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368181712 RFLAGS_BEFORE=130 CR3_BEFORE=55230464 fs_base=0 gs_base=18446744071563776488
[14088623637] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[14089462200] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[14094426357] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0095fb0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[14095871229] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368207584 RFLAGS_BEFORE=130 CR3_BEFORE=55332864 fs_base=0 gs_base=18446744071563776488
[14100285507] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[14107775484] [INFO] [bristle] bristle: registered in graph as svc.Input (id=457)
[14116536588] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=431 backend=BootFB
[14117899950] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[14118753363] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14120351454] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14188704552] [INFO] [kernel::task::loader] Segment: vaddr=2615d0 exec=false
[14189739663] [INFO] [kernel::task::loader]   Overlap at 261000: merging perms to r=true w=false x=true
[14198545251] [INFO] [kernel::task::loader] Segment: vaddr=26d1a0 exec=false
[14199265377] [INFO] [kernel::task::loader]   Overlap at 26d000: merging perms to r=true w=true x=true
[14208129507] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[14210884512] [INFO] [kernel::task::loader] Loading module: /boot/echo
[14211809007] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14212955889] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14216243085] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[14216906418] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14217999081] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[14218621164] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[14225809125] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[14226909312] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[14227735830] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[14241534351] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014b08
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1af
[14242965924] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368249840 RFLAGS_BEFORE=134 CR3_BEFORE=55439360 fs_base=0 gs_base=18446744071563776488
[14246119866] [INFO] [bloom::logging] bloom: logging initialized
[14249869755] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045e88
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[14251544703] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368267376 RFLAGS_BEFORE=130 CR3_BEFORE=55975936 fs_base=0 gs_base=18446744071563776488
[14256252714] [INFO] [echo] echo: online (handle=12)
[14257284756] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[14266668273] [INFO] [bloom] bloom: [bloom] Bootstrapped via Bytespace ID=431
[14270442054] [INFO] [bloom] bloom: [bloom] starting (arg_req=1 arg_resp=4 bristle=10 font_svc=0)
[14281358421] [INFO] [bloom] bloom: [bloom] spawned wallpaper_loader thread (tid=15)
[14288793816] [INFO] [bloom] bloom: [bloom] spawned cursor_loader thread (tid=16)
[14296827237] [INFO] [bloom] bloom: [bloom] spawned font_loader thread (tid=17)
[14297976627] [INFO] [bloom] bloom: [bloom] discovering compositor target...
T:AEF0 [14304062883] [INFO] [bloom] bloom: [wallpaper_loader] thread started
T:AB20 [14306008761] [INFO] [bloom] bloom: [cursor_loader] thread started
T:A0B0 [14307753867] [INFO] [bloom] bloom: [font_loader] thread started
[14308566690] [INFO] [bloom] bloom: [font_loader] scanning boot modules for fonts...
[14318418147] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[14319501009] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[14320588524] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
[14326185225] [INFO] [bloom] bloom: [font_loader] found 42 boot modules
[14415556287] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/DSEG7Classic-Regular.ttf'
[14422798533] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=179 size=23272 name='/assets/fonts/DSEG7Classic-Regular.ttf'
[14458193904] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:35F0 [14465065230] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[14466315072] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[14469895473] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/Hack-Regular.ttf'
[14474523756] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[14475405582] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[14501428887] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[14504422053] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[14513792370] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=182 size=309408 name='/assets/fonts/Hack-Regular.ttf'
[14519102730] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[14521814010] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSans-Regular.ttf'
[14527174530] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[14528000652] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[15150133515] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=185 size=569208 name='/assets/fonts/NotoSans-Regular.ttf'
[15382535289] [INFO] [bloom] bloom: [wallpaper_loader] searching for wallpaper...
[15383675472] [INFO] [bloom] bloom: [wallpaper_loader] trying: /assets/wallpapers/clouds.bmp
[15414371973] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[15415741770] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[15708098076] [INFO] [bloom] bloom: [cursor_loader] searching for cursor...
[15709324719] [INFO] [bloom] bloom: [cursor_loader] trying: /assets/cursors/plain/Normal.cur
[15721206204] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[15724757037] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol-Regular.ttf'
[15732020931] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[15732865698] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[16363334229] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=188 size=258156 name='/assets/fonts/NotoSansSymbol-Regular.ttf'
[16700752431] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[17346088980] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=191 size=656852 name='/assets/fonts/NotoSansSymbol2-Regular.ttf'
[17349950013] [INFO] [bloom::compositor] bloom: compositor bytespace 376 (1280x720 stride=5120 format=1)
[17673826632] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSerif-Regular.ttf'
[17909547480] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[17911036770] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[17912101284] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (258156 bytes)
[17921412333] [INFO] [bloom::asset] [asset_bank] mapped at 0x10463000
[17922330129] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[18003307179] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=194 size=616196 name='/assets/fonts/NotoSerif-Regular.ttf'
[18329265174] [INFO] [bloom::compositor] bloom: display backend: BootFB
[18334143003] [INFO] [ps2_mouse] ps2_mouse: init done
[18334872864] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[18335877780] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18336723999] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[18658224519] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[18984890496] [INFO] [bloom] bloom: [font_loader] immediate scan complete, entering watch loop
[18994004172] [INFO] [bloom] bloom: [bloom] compositor target: 1280x720 @ 0x104a3000 backend=BootFB
[18995223258] [INFO] [bloom] bloom: [bloom] presenter: driver (req=1 resp=4)
[18996877020] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[19328433300] [INFO] [stem::ui] UiBuilder: created root 546
[19329281103] [INFO] [bloom] bloom: [bloom] created UI root node: 546
[19339181070] [INFO] [display_bootfb] display_bootfb: bound bytespace 376
[19673528094] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd5a0
[19674469056] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[19675207365] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19676122290] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[19685715885] [INFO] [clock] Found UI Root: 546 (attempt 4)
[19867664388] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[19869122130] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[19870017123] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 191 (656852 bytes)
[19875441432] [INFO] [bloom] bloom: [bloom] ui watch opened UNFILTERED (userspace filter pred=UI_TEXT id=519)
[19909686291] [INFO] [bloom::asset] [asset_bank] mapped at 0x10bab000
[19910529771] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[23325493356] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c1b40
[23326382277] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[23327211138] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23328015678] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[23638092126] [INFO] [bloom] bloom: [font_loader] watch opened (id=572)
[23962877235] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[23975092383] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol2-Regular.ttf' in slot 4
[23976223755] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 194 (616196 bytes)
[23987104152] [INFO] [bloom::asset] [asset_bank] mapped at 0x10c4c000
[23987991489] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSerif-Regular.ttf'...
[24619930137] [INFO] [bloom] bloom: [cursor_loader] SUCCESS: found candidate '/assets/cursors/plain/Normal.cur', enqueuing load
[28227904980] [INFO] [bloom] bloom: [wallpaper_loader] SUCCESS: found candidate '/assets/wallpapers/clouds.bmp', enqueuing load
[32812673355] [INFO] [bloom] bloom: [cursor_loader] thread done, sleeping forever
[32818699881] [INFO] [bloom] bloom: [wallpaper_loader] thread done, sleeping forever
[34127939208] [INFO] [clock] Binding created: 588 (source=356 target=575)
[34128921717] [INFO] [clock] CLOCK: Entering main loop, publishing to thing_id=356
[34131663786] [INFO] [clock] unix=1768887472 utc=2026-01-20 05:37:52 mono_ns=17065020093
[34165868451] [INFO] [cambium] Found 1 bindings
[35162759214] [INFO] [echo] KeyDown LAlt +Alt
[35173718976] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fedd0
[35176998285] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[35178419166] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35179864137] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x4 kind=0 pred=0 subj_lo=356
[35505073758] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:37:52' tick=17065020093
[35508355938] [INFO] [cambium] Opened watch 607 for source 356 (binding 588, start_seq=0)
[35542914198] [INFO] [cambium] CATCH-UP: Draining 1 watches for historical events...
[37504074069] [INFO] [cambium] cambium: drain complete payloads=0 overflows=5 last_seq=none
[37505274279] [INFO] [cambium] Entering event loop with 1 bindings (v3)
[37829813406] [INFO] [echo] KeyDown A +Alt
[38190284748] [INFO] [echo] KeyUp A +Alt
[38849031177] [INFO] [clock] unix=1768887474 utc=2026-01-20 05:37:54 mono_ns=19424307870
[40191241101] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:37:54' tick=19424307870
[41056373853] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[41057855685] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSerif-Regular.ttf' in slot 5
[41059029429] [INFO] [bloom::asset] [asset_bank] load_cursor_immediate: /assets/cursors/plain/Normal.cur
[41186202387] [INFO] [bloom::asset] [asset_bank] mapping bytespace 152 (4286 bytes)
[41191165257] [INFO] [bloom::asset] [asset_bank] mapped to 0x10ce3000
[41191918119] [INFO] [bloom::asset] [asset_bank] checking CUR header: len=4286
[41192738796] [INFO] [bloom::asset] [asset_bank] valid CUR header detected
[41193673422] [INFO] [bloom::asset] [asset_bank] CUR: hotspot=(0, 0), img_size=4264, offset=22
[41194479810] [INFO] [bloom::asset] [asset_bank] decoding embedded DIB at offset 22...
[41196554421] [INFO] [bloom::asset] [asset_bank] SUCCESS: cursor DIB decoded 32x32
[41206669845] [INFO] [bloom::asset] [asset_bank] publish_cursor (pending)
[41207515338] [INFO] [bloom::asset] [asset_bank] cursor: Static frame 32x32 hotspot (0, 0)
[41208417789] [INFO] [bloom::asset] [asset_bank] load_wallpaper_immediate: /assets/wallpapers/clouds.bmp
[41348371977] [INFO] [bloom::asset] [asset_bank] mapping bytespace 170 (3145782 bytes)
[41358996855] [INFO] [bloom::asset] [asset_bank] mapped to 0x10ce5000
[41359795785] [INFO] [bloom::asset] [asset_bank] decoding BMP...
[41811159390] [INFO] [bloom::asset] [asset_bank] BMP decoded: 1024x1024
[42298411683] [INFO] [bloom::asset] [asset_bank] bytespace unmapped
[42299462238] [INFO] [bloom::asset] [asset_bank] publish_wallpaper (pending): 1024x1024

```
</details>
