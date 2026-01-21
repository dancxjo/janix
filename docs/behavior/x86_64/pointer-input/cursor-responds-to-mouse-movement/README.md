# ✅ Scenario: Cursor responds to mouse movement

> Last run: 2026-01-20 17:49:56

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given a cursor is visible on the screen | ✅ | 4436ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | When I move the mouse | ✅ | 581ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the serial log should contain 'CONTRACT: input pointer_move' | ✅ | 435ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And the cursor should move correspondingly on the screen | ✅ | 380ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[10126849227] [CONTRACT] [kernel] thing-os kernel starting...
[10136289570] [INFO] [kernel::memory] Memory map has 64 entries
[10139116383] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[10139870862] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[10140201951] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[10140559011] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[10140874491] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[10141185054] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[10141500072] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[10141812153] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[10142172117] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78659000 (Usable)
[10142505120] [INFO] [kernel::memory]   [9] 0x78659000 - 0x786bb000 (Reserved)
[10142868813] [INFO] [kernel::memory]   [10] 0x786bb000 - 0x7883d000 (Other)
[10143197328] [INFO] [kernel::memory]   [11] 0x7883d000 - 0x7883e000 (Reserved)
[10143575211] [INFO] [kernel::memory]   [12] 0x7883e000 - 0x788d5000 (Other)
[10143962796] [INFO] [kernel::memory]   [13] 0x788d5000 - 0x788d6000 (Reserved)
[10144307712] [INFO] [kernel::memory]   [14] 0x788d6000 - 0x78977000 (Other)
[10144713084] [INFO] [kernel::memory]   [15] 0x78977000 - 0x78978000 (Reserved)
[10145109546] [INFO] [kernel::memory]   [16] 0x78978000 - 0x789b8000 (Other)
[10145447235] [INFO] [kernel::memory]   [17] 0x789b8000 - 0x789b9000 (Reserved)
[10145789478] [INFO] [kernel::memory]   [18] 0x789b9000 - 0x78a44000 (Other)
[10146149442] [INFO] [kernel::memory]   [19] 0x78a44000 - 0x78a45000 (Reserved)
[10146494655] [INFO] [kernel::memory]   [20] 0x78a45000 - 0x78a91000 (Other)
[10146823995] [INFO] [kernel::memory]   [21] 0x78a91000 - 0x78a92000 (Reserved)
[10147201416] [INFO] [kernel::memory]   [22] 0x78a92000 - 0x78a98000 (Other)
[10147532175] [INFO] [kernel::memory]   [23] 0x78a98000 - 0x78a99000 (Reserved)
[10147874616] [INFO] [kernel::memory]   [24] 0x78a99000 - 0x78f1a000 (Other)
[10148203032] [INFO] [kernel::memory]   [25] 0x78f1a000 - 0x78f1b000 (Reserved)
[10148544153] [INFO] [kernel::memory]   [26] 0x78f1b000 - 0x7939c000 (Other)
[10148874483] [INFO] [kernel::memory]   [27] 0x7939c000 - 0x7939d000 (Reserved)
[10149216825] [INFO] [kernel::memory]   [28] 0x7939d000 - 0x7969e000 (Other)
[10149566526] [INFO] [kernel::memory]   [29] 0x7969e000 - 0x7969f000 (Reserved)
[10149909594] [INFO] [kernel::memory]   [30] 0x7969f000 - 0x796a8000 (Other)
[10150239957] [INFO] [kernel::memory]   [31] 0x796a8000 - 0x796a9000 (Reserved)
[10150581342] [INFO] [kernel::memory]   [32] 0x796a9000 - 0x796ad000 (Other)
[10150910583] [INFO] [kernel::memory]   [33] 0x796ad000 - 0x796ae000 (Reserved)
[10151252628] [INFO] [kernel::memory]   [34] 0x796ae000 - 0x796b0000 (Other)
[10151581242] [INFO] [kernel::memory]   [35] 0x796b0000 - 0x796b1000 (Reserved)
[10151920350] [INFO] [kernel::memory]   [36] 0x796b1000 - 0x796b3000 (Other)
[10152247842] [INFO] [kernel::memory]   [37] 0x796b3000 - 0x796b4000 (Reserved)
[10152587742] [INFO] [kernel::memory]   [38] 0x796b4000 - 0x796b6000 (Other)
[10152958068] [INFO] [kernel::memory]   [39] 0x796b6000 - 0x796b7000 (Reserved)
[10153299585] [INFO] [kernel::memory]   [40] 0x796b7000 - 0x796b9000 (Other)
[10153627473] [INFO] [kernel::memory]   [41] 0x796b9000 - 0x796ba000 (Reserved)
[10153967538] [INFO] [kernel::memory]   [42] 0x796ba000 - 0x796bc000 (Other)
[10154313939] [INFO] [kernel::memory]   [43] 0x796bc000 - 0x796bd000 (Reserved)
[10154660274] [INFO] [kernel::memory]   [44] 0x796bd000 - 0x796c7000 (Other)
[10154989977] [INFO] [kernel::memory]   [45] 0x796c7000 - 0x796c8000 (Reserved)
[10155330900] [INFO] [kernel::memory]   [46] 0x796c8000 - 0x796cc000 (Other)
[10155660834] [INFO] [kernel::memory]   [47] 0x796cc000 - 0x796cd000 (Reserved)
[10156028520] [INFO] [kernel::memory]   [48] 0x796cd000 - 0x796cf000 (Other)
[10156848801] [INFO] [kernel::memory]   [49] 0x796cf000 - 0x796d0000 (Reserved)
[10157233845] [INFO] [kernel::memory]   [50] 0x796d0000 - 0x796d2000 (Other)
[10157565165] [INFO] [kernel::memory]   [51] 0x796d2000 - 0x796d3000 (Reserved)
[10157907804] [INFO] [kernel::memory]   [52] 0x796d3000 - 0x796d7000 (Other)
[10158237903] [INFO] [kernel::memory]   [53] 0x796d7000 - 0x79750000 (Other)
[10158570180] [INFO] [kernel::memory]   [54] 0x79750000 - 0x79907000 (Other)
[10158904008] [INFO] [kernel::memory]   [55] 0x79907000 - 0x7a16c000 (Reserved)
[10159263048] [INFO] [kernel::memory]   [56] 0x7a16c000 - 0x7bb6c000 (Usable)
[10159615686] [INFO] [kernel::memory]   [57] 0x7bb6c000 - 0x7bb8d000 (Reserved)
[10159957335] [INFO] [kernel::memory]   [58] 0x7bb8d000 - 0x7bb91000 (Other)
[10160287236] [INFO] [kernel::memory]   [59] 0x7bb91000 - 0x7bb92000 (Reserved)
[10160628753] [INFO] [kernel::memory]   [60] 0x7bb92000 - 0x7bb94000 (Other)
[10160957334] [INFO] [kernel::memory]   [61] 0x7bb94000 - 0x7bb95000 (Reserved)
[10161299445] [INFO] [kernel::memory]   [62] 0x7bb95000 - 0x7bb97000 (Other)
[10161630336] [INFO] [kernel::memory]   [63] 0x7bb97000 - 0x7bb98000 (Reserved)
[10162190346] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[10389105837] [CONTRACT] [kernel::memory] Frame allocator initialized with 495729 free frames
[10395727419] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[10400239542] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[10401266568] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[10401891654] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[10408482282] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[10408922370] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[10411627380] [INFO] [bran::arch] IOAPIC: Registers initialized
[10412607381] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[10413996714] [INFO] [bran::arch] IOAPIC: All pins masked
[10415313282] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[10415887680] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[10416369051] [INFO] [bran::arch] IOAPIC: Init complete
[10416925266] [CONTRACT] [kernel] Initializing global allocator...
[10730040024] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[10730636598] [CONTRACT] [kernel] Initializing SIMD...
[10731922245] [CONTRACT] [kernel] Initializing tasking...
[10736566995] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[10738049850] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[10738588377] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[10744122477] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[10744511712] [INFO] [kernel::task::scheduler]   Initializing boot task...
[10745219034] [INFO] [kernel::task::scheduler]   Creating boot task...
[10749522795] [INFO] [kernel::task::scheduler]   Creating idle task...
[10754152101] [INFO] [kernel::task::scheduler]   Boot task initialized
[10754524704] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[10755487809] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[10761065403] [INFO] [kernel::root] Spawning Root service...
[10768493736] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[10777860324] [INFO] [kernel::root::service] ROOT: started once
[11660891550] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[11661781428] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[11702403009] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[11719179120] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[11744674425] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[11775043335] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[11776686273] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[11812645218] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[11834950743] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[11841109170] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[11843472696] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[11866158282] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[11868557085] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[11869600809] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[11872797717] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[11875201338] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[11875955817] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[11883220008] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[11894706021] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[11895558972] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[11898005724] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[11898805611] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[11906196819] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[11906770788] [CONTRACT] [kernel] Spawning init process...
[11908441578] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[11942899287] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62151700 ticks/sec), init_cnt=621517 for 100Hz
[11944343334] [CONTRACT] [kernel] Entering scheduler loop.
[11952799980] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[11957218647] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072367676144 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563780584
[11967105414] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[11968473264] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[11969189925] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[11969900514] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[11988652830] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[11992056978] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[11995045590] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[11995783206] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[11998977936] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[12001288365] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[12001990077] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[12005073267] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[12005810388] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[12006475602] [INFO] [sprout::devtree] SPROUT: build() called
[12007074387] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[12011328615] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[12012061743] [INFO] [sprout] SPROUT: About to create Supervisor...
[12012656271] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[12013350690] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[12013991286] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[12018268779] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[12051578022] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[12055849971] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[12058994541] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[12061866003] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[12064105977] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[12066719643] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[12069943677] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[12072938889] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[12076049898] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[12079041645] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[12082060089] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[12085422822] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[12088820205] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[12091655334] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[12094534023] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[12097385883] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[12100203126] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[12102995058] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[12105819594] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[12109142694] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[12112464177] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[12115645608] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[12118542249] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[12121538847] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[12124664673] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[12127653054] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[12130872897] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[12133993575] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[12137164380] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[12140284167] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[12143266806] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[12146264757] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[12149248947] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[12152253366] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[12155148159] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[12158188647] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[12161361696] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[12164471286] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[12167401917] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[12170309019] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[12173341818] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[12176261229] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[12179236410] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[12181389627] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[12193136043] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[12255724140] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[12256578708] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[12258379122] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[12259393641] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[12260143830] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[12260960250] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[12263259723] [INFO] [kernel::task::loader] Loading module: /boot/clock
[12263768814] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12264796005] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12268360401] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[12268918926] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[12270192132] [INFO] [kernel::task::loader] Segment: vaddr=204ee8 exec=false
[12270851340] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12326209500] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12340925949] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072367986928 RFLAGS_BEFORE=134 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563780584
[12345599013] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[12346683228] [INFO] [clock] starting clock publisher
[12351611250] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[12354523368] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[12355476738] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[12356032062] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12357008070] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12360760566] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[12361347537] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[12362146797] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[12362709447] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12368389242] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[12369426960] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[12370356075] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[12370835532] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12371725179] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12381701079] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[12382266996] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[12386367246] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[12387391335] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[12392773041] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[12393366447] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[12394188774] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[12394648629] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12395535900] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12398878206] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[12399651297] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[12400867578] [INFO] [kernel::task::loader] Segment: vaddr=204c88 exec=false
[12401444517] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12407307297] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[12407966274] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[12421334475] [INFO] [clock] Clock thing created: 327
[12421908642] [INFO] [clock] Waiting for UI Root (Compositor)...
[12424605270] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044678
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12425819439] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368012096 RFLAGS_BEFORE=130 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563780584
[12429969684] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045680
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[12430995225] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368034864 RFLAGS_BEFORE=134 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563780584
[12432708123] [ERROR] [INGESTD] Starting...
[12436075410] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045718
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12437092239] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368051248 RFLAGS_BEFORE=134 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563780584
[12439668252] [INFO] [cambium] cambium starting (v3: catch-up then stream)...
[12444616602] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[12447728832] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[12448827996] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[12449771961] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[12450975669] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[12462831381] [ERROR] [INGESTD] Watch active. Loop start.
[12472508763] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[12473612052] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[12675845391] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[12676537632] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[12677528457] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12680395926] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[12680966529] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12681917160] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[12682488984] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[12688402122] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[12694004598] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00141a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[12694995654] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368100656 RFLAGS_BEFORE=134 CR3_BEFORE=54927360 fs_base=0 gs_base=18446744071563780584
[12698531868] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[12714082062] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[12721679652] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[12729826626] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[12730958460] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[12731464581] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[12732462897] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12735393429] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[12735964989] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12737195097] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[12737747451] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[12743532021] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[12744569706] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[12745771203] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[12746784072] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[12748104501] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[12749035167] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[12749509773] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12750412389] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12752690973] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[12753258606] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[12754135779] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[12754827789] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[12760353045] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[12761574243] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[12762216984] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12763165965] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12765969282] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[12766608426] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12767807811] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[12768723099] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[12774375372] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[12775938219] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[12776888454] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[12777379725] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12778317255] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12781430310] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[12781946991] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12782829906] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[12783331836] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[12788769543] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[12802406892] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014318
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[12803505957] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368124336 RFLAGS_BEFORE=130 CR3_BEFORE=55033856 fs_base=0 gs_base=18446744071563780584
[12807149124] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[12808002306] [INFO] [rtc_cmos] Starting... arg=db
[12809428599] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[12812445393] [INFO] [rtc_cmos] RTC: 2026-01-21 01:50:58 = 1768960258 unix_secs
[12813543996] [INFO] [kernel::time] System clock anchored: unix_secs=1768960258, mono_ns=6406549842, offset=1768960251593450158ns
[12814694079] [INFO] [rtc_cmos] System clock anchored
[12829137156] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[12831166623] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045dc0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[12832141707] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368158960 RFLAGS_BEFORE=134 CR3_BEFORE=55136256 fs_base=0 gs_base=18446744071563780584
[12835183317] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[12836992872] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[12838020789] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[12838735371] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[12843517731] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ba00
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[12844685832] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368176448 RFLAGS_BEFORE=130 CR3_BEFORE=55230464 fs_base=0 gs_base=18446744071563780584
[12847855812] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[12848606100] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[12851956425] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0091ae8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[12853020279] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368212480 RFLAGS_BEFORE=134 CR3_BEFORE=55332864 fs_base=0 gs_base=18446744071563780584
[12856186398] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[12863339511] [INFO] [bristle] bristle: registered in graph as svc.Input (id=457)
[12871036728] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=431 backend=BootFB
[12872058408] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[12872533047] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12873503709] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12933670926] [INFO] [kernel::task::loader] Segment: vaddr=26b720 exec=false
[12934317693] [INFO] [kernel::task::loader]   Overlap at 26b000: merging perms to r=true w=false x=true
[12941378670] [INFO] [kernel::task::loader] Segment: vaddr=277610 exec=false
[12941967819] [INFO] [kernel::task::loader]   Overlap at 277000: merging perms to r=true w=true x=true
[12948201057] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[12949468587] [INFO] [kernel::task::loader] Loading module: /boot/echo
[12949970946] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12950891547] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12953698230] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[12954264477] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12955236525] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[12955816368] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[12961819992] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[12962765706] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[12963465735] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[12973163709] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014c40
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1af
[12974244459] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368250160 RFLAGS_BEFORE=134 CR3_BEFORE=55439360 fs_base=0 gs_base=18446744071563780584
[12976927128] [INFO] [bloom::logging] bloom: logging initialized
[12998279514] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004b958
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[12999353763] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368267712 RFLAGS_BEFORE=134 CR3_BEFORE=56016896 fs_base=0 gs_base=18446744071563780584
[13002422796] [INFO] [echo] echo: online (handle=12)
[13003110549] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[13009041012] [INFO] [bloom] bloom: [bloom] Bootstrapped via Bytespace ID=431
[13011958839] [INFO] [bloom] bloom: [bloom] starting (arg_req=1 arg_resp=4 bristle=10 font_svc=0)
[13020842241] [INFO] [bloom] bloom: [bloom] spawned wallpaper_loader thread (tid=15)
[13027334760] [INFO] [bloom] bloom: [bloom] spawned cursor_loader thread (tid=16)
[13032761478] [INFO] [bloom] bloom: [bloom] spawned font_loader thread (tid=17)
[13033409334] [INFO] [bloom] bloom: [bloom] discovering compositor target...
T:DB40 [13037955975] [INFO] [bloom] bloom: [wallpaper_loader] thread started
[13041387282] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[13042147503] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[13043070513] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
T:D770 [13045958409] [INFO] [bloom] bloom: [cursor_loader] thread started
T:CD00 [13047811953] [INFO] [bloom] bloom: [font_loader] thread started
[13048539537] [INFO] [bloom] bloom: [font_loader] scanning boot modules for fonts...
[13053795645] [INFO] [bloom] bloom: [font_loader] found 42 boot modules
[13133983104] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/DSEG7Classic-Regular.ttf'
[13140247296] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=179 size=23272 name='/assets/fonts/DSEG7Classic-Regular.ttf'
[13147108986] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:CD50 [13151281803] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[13152318465] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[13154997504] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/Hack-Regular.ttf'
[13159413795] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[13160220777] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[13180738065] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[13183063278] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[13190556192] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=182 size=309408 name='/assets/fonts/Hack-Regular.ttf'
[13215247980] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[13217520393] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSans-Regular.ttf'
[13223548965] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[13224330174] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[13555650141] [INFO] [ps2_mouse] ps2_mouse: drained 0x28
[13808848515] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[13810160430] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[13813742316] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=185 size=569208 name='/assets/fonts/NotoSans-Regular.ttf'
[13818289089] [INFO] [ps2_mouse] ps2_mouse: drained 0x32
[13819836162] [INFO] [bloom] bloom: [wallpaper_loader] searching for wallpaper...
[13820734059] [INFO] [bloom] bloom: [wallpaper_loader] trying: /assets/wallpapers/clouds.bmp
[13824590439] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[13826757945] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol-Regular.ttf'
[13832425860] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[13833189183] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[13884349743] [INFO] [ps2_mouse] ps2_mouse: drained 0xce
[14204252898] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=188 size=258156 name='/assets/fonts/NotoSansSymbol-Regular.ttf'
[14504501220] [INFO] [bloom] bloom: [cursor_loader] searching for cursor...
[14505496005] [INFO] [bloom] bloom: [cursor_loader] trying: /assets/cursors/plain/Normal.cur
[15155803872] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[15808928982] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=191 size=656852 name='/assets/fonts/NotoSansSymbol2-Regular.ttf'
[16141694811] [INFO] [bloom::compositor] bloom: compositor bytespace 376 (1280x720 stride=5120 format=1)
[17028057222] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[17029259148] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[17128577400] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (258156 bytes)
[17131385964] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSerif-Regular.ttf'
[17133769554] [INFO] [bloom::compositor] bloom: display backend: BootFB
[17138419848] [INFO] [bloom::asset] [asset_bank] mapped at 0x10463000
[17139215973] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[17448832896] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[17453407125] [INFO] [ps2_mouse] ps2_mouse: init done
[17454010728] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[17455070853] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[17455767879] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[17774309124] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=194 size=616196 name='/assets/fonts/NotoSerif-Regular.ttf'
[18112802829] [INFO] [bloom] bloom: [bloom] compositor target: 1280x720 @ 0x104a3000 backend=BootFB
[18114082503] [INFO] [bloom] bloom: [bloom] presenter: driver (req=1 resp=4)
[18115637859] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[18757016058] [INFO] [stem::ui] UiBuilder: created root 548
[18758070771] [INFO] [bloom] bloom: [bloom] created UI root node: 548
[18767175537] [INFO] [display_bootfb] display_bootfb: bound bytespace 376
[19105807941] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd5b0
[19106627001] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[19107284724] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19108140315] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[19417559271] [INFO] [clock] Found UI Root: 548 (attempt 4)
[19458189696] [INFO] [bloom] bloom: [font_loader] immediate scan complete, entering watch loop
[19493995158] [INFO] [bloom] bloom: [bloom] ui watch opened UNFILTERED (userspace filter pred=UI_TEXT id=520)

```
</details>
