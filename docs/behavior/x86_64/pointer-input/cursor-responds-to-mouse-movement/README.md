# ✅ Scenario: Cursor responds to mouse movement

> Last run: 2026-01-29 22:19:33

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given a cursor is visible on the screen | ✅ | 14945ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I move the mouse | ✅ | 1345ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the serial log should contain 'CONTRACT: input pointer_move' | ✅ | 1341ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And the cursor should move correspondingly on the screen | ✅ | 205ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[18819248585] [CONTRACT] [kernel] thing-os kernel starting...
[18840873076] [INFO] [kernel::memory] Memory map has 64 entries
[18849163625] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[18855311775] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[18860734974] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[18866207976] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[18871709380] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[18877045724] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[18883981313] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[18889815303] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[18895792226] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78285000 (Usable)
[18901526183] [INFO] [kernel::memory]   [9] 0x78285000 - 0x782e9000 (Reserved)
[18907427626] [INFO] [kernel::memory]   [10] 0x782e9000 - 0x782ea000 (Other)
[18912996895] [INFO] [kernel::memory]   [11] 0x782ea000 - 0x782eb000 (Reserved)
[18918954271] [INFO] [kernel::memory]   [12] 0x782eb000 - 0x782ec000 (Other)
[18924648848] [INFO] [kernel::memory]   [13] 0x782ec000 - 0x782ed000 (Reserved)
[18930449269] [INFO] [kernel::memory]   [14] 0x782ed000 - 0x782ee000 (Other)
[18936298473] [INFO] [kernel::memory]   [15] 0x782ee000 - 0x782ef000 (Reserved)
[18942598435] [INFO] [kernel::memory]   [16] 0x782ef000 - 0x782f0000 (Other)
[18948739734] [INFO] [kernel::memory]   [17] 0x782f0000 - 0x782f1000 (Reserved)
[18954722975] [INFO] [kernel::memory]   [18] 0x782f1000 - 0x782f2000 (Other)
[18960358589] [INFO] [kernel::memory]   [19] 0x782f2000 - 0x782f3000 (Reserved)
[18966214272] [INFO] [kernel::memory]   [20] 0x782f3000 - 0x782f4000 (Other)
[18971981672] [INFO] [kernel::memory]   [21] 0x782f4000 - 0x782f5000 (Reserved)
[18977736522] [INFO] [kernel::memory]   [22] 0x782f5000 - 0x782f6000 (Other)
[18983444202] [INFO] [kernel::memory]   [23] 0x782f6000 - 0x782f7000 (Reserved)
[18989283885] [INFO] [kernel::memory]   [24] 0x782f7000 - 0x782f8000 (Other)
[18994914299] [INFO] [kernel::memory]   [25] 0x782f8000 - 0x782f9000 (Reserved)
[19000841075] [INFO] [kernel::memory]   [26] 0x782f9000 - 0x782fa000 (Other)
[19006473545] [INFO] [kernel::memory]   [27] 0x782fa000 - 0x782fb000 (Reserved)
[19012567370] [INFO] [kernel::memory]   [28] 0x782fb000 - 0x782fc000 (Other)
[19018310357] [INFO] [kernel::memory]   [29] 0x782fc000 - 0x782fd000 (Reserved)
[19024056520] [INFO] [kernel::memory]   [30] 0x782fd000 - 0x782fe000 (Other)
[19029775190] [INFO] [kernel::memory]   [31] 0x782fe000 - 0x782ff000 (Reserved)
[19035598275] [INFO] [kernel::memory]   [32] 0x782ff000 - 0x78300000 (Other)
[19041218846] [INFO] [kernel::memory]   [33] 0x78300000 - 0x78301000 (Reserved)
[19047008166] [INFO] [kernel::memory]   [34] 0x78301000 - 0x78302000 (Other)
[19052598031] [INFO] [kernel::memory]   [35] 0x78302000 - 0x78303000 (Reserved)
[19058453101] [INFO] [kernel::memory]   [36] 0x78303000 - 0x78304000 (Other)
[19064133994] [INFO] [kernel::memory]   [37] 0x78304000 - 0x78305000 (Reserved)
[19069826308] [INFO] [kernel::memory]   [38] 0x78305000 - 0x78306000 (Other)
[19075660054] [INFO] [kernel::memory]   [39] 0x78306000 - 0x78307000 (Reserved)
[19081605398] [INFO] [kernel::memory]   [40] 0x78307000 - 0x78308000 (Other)
[19087271769] [INFO] [kernel::memory]   [41] 0x78308000 - 0x78309000 (Reserved)
[19093072949] [INFO] [kernel::memory]   [42] 0x78309000 - 0x7830a000 (Other)
[19098732813] [INFO] [kernel::memory]   [43] 0x7830a000 - 0x7830b000 (Reserved)
[19104561018] [INFO] [kernel::memory]   [44] 0x7830b000 - 0x7830c000 (Other)
[19110269872] [INFO] [kernel::memory]   [45] 0x7830c000 - 0x7830d000 (Reserved)
[19115972419] [INFO] [kernel::memory]   [46] 0x7830d000 - 0x7830e000 (Other)
[19121702565] [INFO] [kernel::memory]   [47] 0x7830e000 - 0x7830f000 (Reserved)
[19127506003] [INFO] [kernel::memory]   [48] 0x7830f000 - 0x78310000 (Other)
[19133117425] [INFO] [kernel::memory]   [49] 0x78310000 - 0x78311000 (Reserved)
[19139037779] [INFO] [kernel::memory]   [50] 0x78311000 - 0x78312000 (Other)
[19144712322] [INFO] [kernel::memory]   [51] 0x78312000 - 0x78313000 (Reserved)
[19150718209] [INFO] [kernel::memory]   [52] 0x78313000 - 0x78314000 (Other)
[19156465319] [INFO] [kernel::memory]   [53] 0x78314000 - 0x78315000 (Reserved)
[19162189939] [INFO] [kernel::memory]   [54] 0x78315000 - 0x78316000 (Other)
[19168005512] [INFO] [kernel::memory]   [55] 0x78316000 - 0x78317000 (Reserved)
[19173887880] [INFO] [kernel::memory]   [56] 0x78317000 - 0x78318000 (Other)
[19179461443] [INFO] [kernel::memory]   [57] 0x78318000 - 0x78319000 (Reserved)
[19185449850] [INFO] [kernel::memory]   [58] 0x78319000 - 0x7831a000 (Other)
[19190999027] [INFO] [kernel::memory]   [59] 0x7831a000 - 0x7831b000 (Reserved)
[19196858179] [INFO] [kernel::memory]   [60] 0x7831b000 - 0x7831c000 (Other)
[19202613286] [INFO] [kernel::memory]   [61] 0x7831c000 - 0x7831d000 (Reserved)
[19208367481] [INFO] [kernel::memory]   [62] 0x7831d000 - 0x7831e000 (Other)
[19214010026] [INFO] [kernel::memory]   [63] 0x7831e000 - 0x7831f000 (Reserved)
[19220208051] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[19509950330] [CONTRACT] [kernel::memory] Frame allocator initialized with 488093 free frames
[19525479229] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[19542113546] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[19548882463] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[19554171412] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[19567445282] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[19572574362] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[19581589329] [INFO] [bran::arch] IOAPIC: Registers initialized
[19587931927] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[19595075426] [INFO] [bran::arch] IOAPIC: All pins masked
[19601324109] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[19606928838] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[19612112436] [INFO] [bran::arch] IOAPIC: Init complete
[19617122299] [CONTRACT] [kernel] Initializing global allocator...
[20045306647] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[20053420013] [CONTRACT] [kernel] Initializing SIMD...
[20059626825] [CONTRACT] [kernel] Initializing tasking...
[20075570179] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[20083611766] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[20090771629] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[20103780331] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[20109281093] [INFO] [kernel::task::scheduler]   Initializing boot task...
[20117023227] [INFO] [kernel::task::scheduler]   Creating boot task...
[20127873471] [INFO] [kernel::task::scheduler]   Creating idle task...
[20139252423] [INFO] [kernel::task::scheduler]   Boot task initialized
[20144901376] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[20151762494] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[20164346704] [INFO] [kernel::root] Spawning Root service...
[20179223181] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[20198585517] [INFO] [kernel::root::service] ROOT: started once
[24404791529] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[24412563083] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[24490389688] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[24530411376] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[24585741136] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[24657281742] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[24671927673] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[24746477948] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[24794484577] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[24817215223] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[24827284829] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=656, idx=3) BAR5=0x810c4000
[24880367237] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[24895063084] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[24903428403] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[24913957266] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[24924304132] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[24931025805] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24951512995] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[24973556365] [INFO] [kernel::task::loader] Segment: vaddr=210000 exec=false
[24985868302] [INFO] [kernel::task::loader] Segment: vaddr=215000 exec=false
[25002646391] [INFO] [kernel] Warning: Module registry page overflow, truncating list.
[25011501254] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[25018483072] [CONTRACT] [kernel] Spawning init process...
[25025771778] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[25057598975] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (61960400 ticks/sec), init_cnt=619604 for 100Hz
[25068654171] [CONTRACT] [kernel] Entering scheduler loop.
[25073932010] [INFO] [kernel::tests::time_test] TIME ANCHORING TEST: Starting...
[25081914488] [INFO] [kernel::tests::time_test] TIME ANCHORING TEST: FAIL - sys_time_now returned 12540694168 but should have failed before anchor
[25093182756] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: Starting...
[25105090709] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: 1000 samples monotonic - PASS
[25114622814] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: trace::now() monotonic - PASS
[25124260260] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: trace and syscall consistent - PASS
[25133293125] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: All tests PASS
[25141311065] [INFO] [kernel::tests::fw_tables] FW_TABLES SELFTEST: Starting...
[25148823421] [INFO] [kernel::tests::fw_tables] FW_TABLES SELFTEST: PASS
[25180176119] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0008580
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[25198444010] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072367741888 RFLAGS_BEFORE=134 CR3_BEFORE=50331648 fs_base=0 gs_base=18446744071563860640
[25234363565] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9b0 rip=0x2013ff rflags=0x202
[25244712952] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[25251514004] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[25258502906] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[25293288768] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[25304841409] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[25316375823] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[25322947104] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[25334710869] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[25345037456] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[25352011392] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[25364663217] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[25371461584] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[25379359874] [INFO] [sprout::devtree] SPROUT: build() called
[25385796121] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[25415906175] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[25425055431] [INFO] [sprout] SPROUT: About to create Supervisor...
[25432291735] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[25439985623] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[25447811107] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[25466289518] [INFO] [sprout::supervisor] SPROUT: Found 64 modules
[25532604544] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[25549090914] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[25562934483] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[25576915577] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[25589215260] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[25602091962] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[25617565802] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[25631825951] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[25645190441] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[25657914888] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[25671371740] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[25687120627] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[25700710583] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[25714401648] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/fontd'
[25727296263] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/blossom'
[25740598502] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/ingestd'
[25755188448] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/cambium'
[25769535151] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/scheduler_fairness'
[25783310294] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/boot/drawlist_demo'
[25796705225] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/boot/tick_printer'
[25810910363] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/boot/photosynthesis'
[25824077650] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/boot/ata_disk'
[25838036277] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/boot/iso_reader'
[25851136962] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Alternate.cur'
[25916330826] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Busy.cur'
[25932138481] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Diagonal1.ani'
[25948274112] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Diagonal2.ani'
[25963735293] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Handwriting.cur'
[25978538342] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Help.cur'
[25993115225] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Horizontal.ani'
[26008124194] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Link.ani'
[26023388690] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Move.cur'
[26037640349] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Normal.cur'
[26052357856] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/cursors/plain/Precision.cur'
[26067617761] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/cursors/plain/Text.cur'
[26081970895] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/cursors/plain/Unavailabe.cur'
[26097222857] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/cursors/plain/Vertical.ani'
[26112783000] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/cursors/plain/Working.ani'
[26127140150] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/wallpapers/clouds.bmp'
[26142275873] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/wallpapers/leather.bmp'
[26157166279] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/wallpapers/linen.bmp'
[26170892624] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[26185794148] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/fonts/Hack-Regular.ttf'
[26200178022] [INFO] [sprout::supervisor] SPROUT: Module[42] = '/assets/fonts/NotoSans-Regular.ttf'
[26214313755] [INFO] [sprout::supervisor] SPROUT: Module[43] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[26229218287] [INFO] [sprout::supervisor] SPROUT: Module[44] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[26243753517] [INFO] [sprout::supervisor] SPROUT: Module[45] = '/assets/fonts/NotoSerif-Regular.ttf'
[26257883755] [INFO] [sprout::supervisor] SPROUT: Module[46] = '/assets/pci/pci.ids'
[26271605390] [INFO] [sprout::supervisor] SPROUT: Module[47] = '/assets/cursors/future/alias.svg'
[26286235212] [INFO] [sprout::supervisor] SPROUT: Module[48] = '/assets/cursors/future/all-scroll.svg'
[26301431166] [INFO] [sprout::supervisor] SPROUT: Module[49] = '/assets/cursors/future/bottom_left_corner.svg'
[26316795415] [INFO] [sprout::supervisor] SPROUT: Module[50] = '/assets/cursors/future/bottom_right_corner.svg'
[26331913950] [INFO] [sprout::supervisor] SPROUT: Module[51] = '/assets/cursors/future/bottom_side.svg'
[26348320643] [INFO] [sprout::supervisor] SPROUT: Module[52] = '/assets/cursors/future/cell.svg'
[26362391920] [INFO] [sprout::supervisor] SPROUT: Module[53] = '/assets/cursors/future/center_ptr.svg'
[26376956073] [INFO] [sprout::supervisor] SPROUT: Module[54] = '/assets/cursors/future/col-resize.svg'
[26391412273] [INFO] [sprout::supervisor] SPROUT: Module[55] = '/assets/cursors/future/color-picker.svg'
[26406799242] [INFO] [sprout::supervisor] SPROUT: Module[56] = '/assets/cursors/future/context-menu.svg'
[26429427949] [INFO] [sprout::supervisor] SPROUT: Module[57] = '/assets/cursors/future/copy.svg'
[26446226247] [INFO] [sprout::supervisor] SPROUT: Module[58] = '/assets/cursors/future/crosshair.svg'
[26466882648] [INFO] [sprout::supervisor] SPROUT: Module[59] = '/assets/cursors/future/default.svg'
[26486608625] [INFO] [sprout::supervisor] SPROUT: Module[60] = '/assets/cursors/future/dnd-move.svg'
[26506227288] [INFO] [sprout::supervisor] SPROUT: Module[61] = '/assets/cursors/future/dnd-no-drop.svg'
[26525315859] [INFO] [sprout::supervisor] SPROUT: Module[62] = '/assets/cursors/future/down-arrow.svg'
[26545507459] [INFO] [sprout::supervisor] SPROUT: Module[63] = '/assets/cursors/future/draft.svg'
[26560628074] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[26597038297] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[26878812375] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[26887770186] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[26897633470] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[26907349163] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[26915727672] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/fontd'
[26923829855] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/blossom'
[26932924148] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[26941388779] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/photosynthesis'
[26950551447] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/drawlist_demo'
[26959623643] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[26969758028] [INFO] [kernel::task::loader] Loading module: /boot/clock
[26976545963] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[26986148265] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[27003401735] [INFO] [kernel::task::loader] Segment: vaddr=20b000 exec=false
[27013464870] [INFO] [kernel::task::loader] Segment: vaddr=20f000 exec=false
[27030719679] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[27040603718] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[27049116794] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[27055696971] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[27064803485] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[27081480086] [INFO] [kernel::task::loader] Segment: vaddr=20c000 exec=false
[27090468275] [INFO] [kernel::task::loader] Segment: vaddr=20e000 exec=false
[27148156042] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0047260
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[27187691189] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368149648 RFLAGS_BEFORE=134 CR3_BEFORE=50778112 fs_base=0 gs_base=18446744071563860640
[27211537325] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00041a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[27225265519] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368167856 RFLAGS_BEFORE=130 CR3_BEFORE=50929664 fs_base=0 gs_base=18446744071563860640
[27245617774] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[27253937146] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[27261681993] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[27268390985] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[27278066378] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[27305159265] [INFO] [kernel::task::loader] Segment: vaddr=218000 exec=false
[27318488702] [INFO] [kernel::task::loader] Segment: vaddr=220000 exec=false
[27333961802] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[27341520356] [INFO] [sprout::supervisor] SPROUT: Seeding initial asset requests...
[27371200693] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0040b78
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[27384776802] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368197456 RFLAGS_BEFORE=134 CR3_BEFORE=51077120 fs_base=0 gs_base=18446744071563860640
[27401823099] [INFO] [ingestd] INGESTD: Starting unified content provider service...
[27410950609] [INFO] [ingestd] INGESTD: Initializing Limine module content source...
[27476404582] [INFO] [ingestd] INGESTD: Created Limine ContentSource node
[27483245999] [INFO] [ingestd] INGESTD: Performing initial boot module scan...
[27567777951] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/fontd'
[27575163857] [INFO] [kernel::task::loader] Loading module: /boot/fontd
[27581158437] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[27589613197] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[27625445590] [INFO] [kernel::task::loader] Segment: vaddr=225000 exec=false
[27637517987] [INFO] [kernel::task::loader] Segment: vaddr=22c000 exec=false
[27651939019] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[27658931060] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/blossom'
[27666284545] [INFO] [kernel::task::loader] Loading module: /boot/blossom
[27672352334] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[27681045909] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[27703801516] [INFO] [kernel::task::loader] Segment: vaddr=214000 exec=false
[27712715668] [INFO] [kernel::task::loader] Segment: vaddr=217000 exec=false
[27741053344] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0049010
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[27753798079] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368243632 RFLAGS_BEFORE=130 CR3_BEFORE=51302400 fs_base=0 gs_base=18446744071563860640
[27782569958] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004a338
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[27795644022] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368260016 RFLAGS_BEFORE=130 CR3_BEFORE=51572736 fs_base=0 gs_base=18446744071563860640
[27810921414] [INFO] [blossom] BLOSSOM: Starting SVG Cache Service
[27818135500] [INFO] [blossom] BLOSSOM: Init UI pipeline...
[27832751873] [INFO] [sprout::supervisor] SPROUT: App launched (PID=8)
[27840229431] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[27847834716] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[27854252647] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[27863848404] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[27877646876] [INFO] [kernel::task::loader] Segment: vaddr=207000 exec=false
[27886157916] [INFO] [kernel::task::loader] Segment: vaddr=209000 exec=false
[27901735341] [INFO] [sprout::supervisor] SPROUT: App launched (PID=9)
[27909772335] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/photosynthesis'
[27917989044] [INFO] [kernel::task::loader] Loading module: /boot/photosynthesis
[27924896509] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[27934474919] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[27971498507] [INFO] [kernel::task::loader] Segment: vaddr=226000 exec=false
[27984791749] [INFO] [kernel::task::loader] Segment: vaddr=22e000 exec=false
[28000585002] [INFO] [sprout::supervisor] SPROUT: App launched (PID=10)
[28007726383] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/drawlist_demo'
[28015704743] [INFO] [kernel::task::loader] Loading module: /boot/drawlist_demo
[28022734084] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28031997239] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[28048995286] [INFO] [kernel::task::loader] Segment: vaddr=20b000 exec=false
[28058341511] [INFO] [kernel::task::loader] Segment: vaddr=20e000 exec=false
[28074681205] [INFO] [sprout::supervisor] SPROUT: App launched (PID=11)
[28081871633] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[28128763863] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[28138531885] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[28148427002] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28159639288] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=547 subj_lo=0
[28188751892] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0049af0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28203143953] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368300416 RFLAGS_BEFORE=130 CR3_BEFORE=51757056 fs_base=0 gs_base=18446744071563860640
[28219191644] [INFO] [cambium] cambium starting (v5.2: drain-to-eagain + smart-resync)...
[28237500004] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004baf8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28250867116] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368317872 RFLAGS_BEFORE=130 CR3_BEFORE=51884032 fs_base=0 gs_base=18446744071563860640
[28266478578] [INFO] [photosynthesis] Photosynthesis starting...
[28282904736] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004e310
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28296280026] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368335344 RFLAGS_BEFORE=134 CR3_BEFORE=52162560 fs_base=0 gs_base=18446744071563860640
[28311819607] [INFO] [drawlist_demo] DrawList demo starting (with new commands)...
[28374262639] [INFO] [ingestd] INGESTD: Published new asset '/boot/sprout' (hash=61811fc99aafd0a2)
[28390089707] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[28398403095] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[28407612624] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28417678915] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=587 subj_lo=0
[28442500797] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f8fb0
[28451127745] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[28460273779] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[28470137514] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=589 subj_lo=0
[28499728218] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[28509070214] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[28518533745] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28528748675] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=594 subj_lo=0
[28548251665] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[28555724338] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1920x1080 stride=7680)
[29160046557] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f8fb0
[29169110828] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[29178314376] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[29188659558] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=595 subj_lo=0
[29220688173] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[29229401029] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[29238532313] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29248426010] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=601 subj_lo=0
[29295106992] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[29304196819] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[29313566107] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29323376914] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=606 subj_lo=0
[29358859895] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[29367703569] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[29376821351] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29386414204] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=610 subj_lo=0
[29422727658] [INFO] [ingestd] INGESTD: Created File node '/boot/sprout' (90640 bytes, hash=61811fc99aafd0a2)
[29433011824] [INFO] [ingestd] INGESTD: Published asset '/boot/sprout' (raw, 90640 bytes, hash=61811fc99aafd0a2)
[29495024979] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[29502296240] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[29511738090] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[29523991492] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[29532228495] [INFO] [kernel::task::loader] Segment: vaddr=206000 exec=false
[29548778222] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=12)
[29576684001] [INFO] [blossom] BLOSSOM: UI pipeline ready
[29591636077] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0049af0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x70006
[29606097524] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368402160 RFLAGS_BEFORE=134 CR3_BEFORE=60870656 fs_base=0 gs_base=18446744071563860640
[29623945110] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=6, drv_resp_w=7)
[29683211737] [INFO] [blossom] BLOSSOM: Service node created, req=9, resp=12
[29692045125] [INFO] [blossom] BLOSSOM: Service ready
[29736261480] [INFO] [kernel::syscall::handlers::device] DEVICE: task 12 claimed device 631 (handle 0)
[29765327436] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x7e9000 -> virt=0x1001f000
[29790764145] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[29799817019] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[29807205389] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[29817382141] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[29830733223] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[29839733449] [INFO] [kernel::task::loader] Segment: vaddr=207000 exec=false
[29857122505] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=13)
[29865097102] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[29876328228] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=13, r=14)
[29884951989] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=15, r=16)
[29895671346] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=17, r=18)
[29903235933] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[29910074496] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[29919873417] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[29931265154] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[29939844816] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[29957429349] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=14)
[29966360306] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[29973509093] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[29983407302] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[29995161516] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[30003466752] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[30020101578] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=15)
[30057644696] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0056068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x28b
[30071850098] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369434704 RFLAGS_BEFORE=134 CR3_BEFORE=61005824 fs_base=0 gs_base=18446744071563860640
[30090094834] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[30101263701] [INFO] [rtc_cmos] Starting... arg=28b
[30108989474] [INFO] [rtc_cmos] Serving device ID: ThingId([139, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[30123081337] [INFO] [rtc_cmos] RTC: 2026-01-29 22:24:00 = 1769725440 unix_secs
[30131412587] [INFO] [kernel::time] System clock anchored: unix_secs=1769725440, mono_ns=15065429714, offset=1769725424934570286ns
[30144549199] [INFO] [rtc_cmos] System clock anchored
[30171824661] [INFO] [rtc_cmos] RTC: Set sys.TimeState = 1 (Anchored)
[30204703554] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[30226297570] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb005ea80
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xd
[30240159537] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369467472 RFLAGS_BEFORE=134 CR3_BEFORE=61124608 fs_base=0 gs_base=18446744071563860640
[30258408924] [INFO] [ps2_kbd] ps2_kbd: online (handle=13)
[30274450483] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb005f380
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xf
[30288407893] [INFO] [task.user_enter] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369483856 RFLAGS_BEFORE=134 CR3_BEFORE=61235200 fs_base=0 gs_base=18446744071563860640
[30305879174] [INFO] [ps2_mouse] ps2_mouse: online (handle=15)
[30312878187] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[30330313248] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=19, r=20)
[30338710375] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[30345843387] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[30356238927] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[30368056373] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[30377050821] [INFO] [kernel::task::loader] Segment: vaddr=206000 exec=false
[30394114116] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=16)
[30420394413] [INFO] [ps2_kbd] ps2_kbd: created driver node 976
[30428120636] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[30437217965] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[30451515920] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb005ea80
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe001000110013
[30465846449] [INFO] [task.user_enter] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369526128 RFLAGS_BEFORE=130 CR3_BEFORE=61345792 fs_base=0 gs_base=18446744071563860640
[30482584120] [INFO] [bristle] bristle: online (kbd=14, mouse=16, evt=17, evt_echo=19)
[30517314632] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[30525287957] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[30534298665] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
[30548052908] [INFO] [bristle] bristle: registered in graph as svc.Input (id=996)
[30572503187] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[30585221508] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=988 backend=BootFB
[30593542149] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[30600147868] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[30609582430] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[30774011599] [INFO] [kernel::task::loader] Segment: vaddr=2c5000 exec=false
[30796759491] [INFO] [kernel::task::loader] Segment: vaddr=2d9000 exec=false
[30814173768] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=17)
[30822075633] [INFO] [kernel::task::loader] Loading module: /boot/echo
[30828579764] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[30837919338] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[30849286190] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[30857180392] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[30874211496] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=18)
[30881543616] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[30890078494] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[30929012672] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00560a0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x3dc
[30942640380] [INFO] [task.user_enter] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369557968 RFLAGS_BEFORE=130 CR3_BEFORE=61468672 fs_base=0 gs_base=18446744071563860640
[30959673246] [INFO] [bloom::logging] bloom: logging initialized
[30996468762] [INFO] [bloom] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[31015812689] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb005f380
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x14
[31029195667] [INFO] [task.user_enter] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369574352 RFLAGS_BEFORE=130 CR3_BEFORE=62447616 fs_base=0 gs_base=18446744071563860640
[31073424314] [INFO] [echo] echo: online (handle=20)
[31080973170] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[31114600383] [INFO] [ps2_mouse] ps2_mouse: drained 0x28
[31135320254] [INFO] [ps2_mouse] ps2_mouse: drained 0x32
[31145401056] [INFO] [ingestd] INGESTD: Published new asset '/boot/bristle' (hash=d55e2ab85adcac5d)
[31167617182] [INFO] [ps2_mouse] ps2_mouse: drained 0xce
[31240597452] [INFO] [bloom::compositor] bloom: compositor bytespace 895 (1920x1080 stride=7680 format=2)
[31277959988] [INFO] [ingestd] INGESTD: Created File node '/boot/bristle' (29104 bytes, hash=d55e2ab85adcac5d)
[31289044941] [INFO] [ingestd] INGESTD: Published asset '/boot/bristle' (raw, 29104 bytes, hash=d55e2ab85adcac5d)
[31310885941] [INFO] [bloom::compositor] bloom: display backend: BootFB
[31332441848] [INFO] [bloom::compositor] bloom: mapped size=8294400 (source=bytespace_info)
[31398309460] [INFO] [stem::ui] UiBuilder: created root 1035
[31418589690] [INFO] [bloom] bloom: spawned asset watcher (tid=19)
[31430033636] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[31458124392] [INFO] [ps2_mouse] ps2_mouse: init done
[31467729465] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[31477208700] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[31490802550] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
T:3C90 [31513218304] [INFO] [bloom::painter_resources] [bloom] asset_watcher_entry: spawning sub-loaders
T:41D0 T:4040 T:2A00 [31613657258] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f64e0
[31626981362] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[31641392380] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[31651516592] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=741 subj_lo=0
[31690520974] [INFO] [photosynthesis] Found UI Root: 1035
[31723024486] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f64e0
[31736268941] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[31754343587] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[31767317522] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=643 pred=0 subj_lo=0
[31820902159] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f64e0
[31834713263] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[31846844286] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[31859340370] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=747 subj_lo=0
[31895288047] [INFO] [bloom] [bloom] Starting UI loop immediately (not waiting for fonts)
[31939781197] [INFO] [bloom::painter_resources] [bloom] wallpaper loader: loading leather.bmp
[31952973014] [INFO] [bloom::asset] [asset_bank] worker spawned tid=23 (priority=2)
[31961882274] [INFO] [bloom::painter_resources] [bloom] cursor loader: loading default cursor
[31971531556] [INFO] [bloom::painter_resources] [bloom] icon loader started
T:4C30 [32006823759] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[32012161127] [INFO] [bloom::asset] [asset_bank] load_wallpaper_immediate: leather.bmp
[32031518884] [INFO] [ingestd] INGESTD: Published new asset '/boot/rtc_cmos' (hash=32366e1092006840)
[32142136867] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef60
[32145478035] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[32146872097] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[32148555390] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x0 kind=0 pred=0 subj_lo=0
[32257849716] [INFO] [ingestd] INGESTD: Created File node '/boot/rtc_cmos' (33368 bytes, hash=32366e1092006840)
[32259091682] [INFO] [ingestd] INGESTD: Published asset '/boot/rtc_cmos' (raw, 33368 bytes, hash=32366e1092006840)
[32503628174] [INFO] [bloom::present] bloom: driver REGISTER (kind=1 caps=0x3)
[32506326207] [INFO] [bloom::present] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=8
[32556548154] [INFO] [display_bootfb] display_bootfb: bound bytespace 895
[32570049361] [INFO] [ingestd] INGESTD: Published new asset '/boot/clock' (hash=771f0ce19bac6901)
[32736831576] [INFO] [ingestd] INGESTD: Created File node '/boot/clock' (66064 bytes, hash=771f0ce19bac6901)
[32738614291] [INFO] [ingestd] INGESTD: Published asset '/boot/clock' (raw, 66064 bytes, hash=771f0ce19bac6901)
[32769984318] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7feba0
[32771332593] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[32772798139] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[32774213660] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=788 pred=0 subj_lo=0
[32865853937] [INFO] [bloom::asset] [asset_bank] mapping bytespace 188 (4718646 bytes) for 'leather.bmp'
[32900353775] [INFO] [bloom::asset] [asset_bank] decoding BMP for 'leather.bmp'...
[33554898218] [INFO] [bloom::asset] [asset_bank] BMP decoded: 1536x1024 for 'leather.bmp'
[34304605927] [INFO] [bloom::asset] [asset_bank] publish_wallpaper (pending): 1536x1024
[34307032181] [INFO] [bloom::asset] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[34455057037] [INFO] [bloom::present] bloom: driver BIND ACK
[34475787801] [INFO] [bloom::reclaimer] [reclaimer] +6291456 bytes (total: 6291456)
[34482639385] [INFO] [bloom::asset] [asset_bank] promoting wallpaper 'leather.bmp' to gen=1 (6291456b)
[34500494202] [INFO] [ingestd] INGESTD: Published new asset '/boot/font_explorer' (hash=1cdbe11025543439)
[34681545808] [INFO] [ingestd] INGESTD: Created File node '/boot/font_explorer' (61968 bytes, hash=1cdbe11025543439)
[34688137438] [INFO] [ingestd] INGESTD: Published asset '/boot/font_explorer' (raw, 61968 bytes, hash=1cdbe11025543439)
[34934562550] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x10040870
[34940865774] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[34943677836] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34945091976] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=803 pred=0 subj_lo=0
[34989036670] [INFO] [ingestd] INGESTD: Published new asset '/boot/ps2_kbd' (hash=4da765f0b8256772)
[35000310430] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x10040870
[35005036513] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[35006516728] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35007980954] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=788 pred=0 subj_lo=0
[35046833843] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x10040870
[35053579774] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[35060054796] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35062990550] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=804 pred=0 subj_lo=0
[35108548567] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x10040870
[35117395509] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[35119077984] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35120521651] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=805 pred=0 subj_lo=0
[35230114900] [INFO] [ingestd] INGESTD: Created File node '/boot/ps2_kbd' (25008 bytes, hash=4da765f0b8256772)
[35237715213] [INFO] [ingestd] INGESTD: Published asset '/boot/ps2_kbd' (raw, 25008 bytes, hash=4da765f0b8256772)
[35540953433] [INFO] [bloom::asset] [asset_bank] mapping bytespace 251 (3051 bytes)
[35565117325] [INFO] [bloom::asset] [asset_bank] mapped to 0x11c95000
[35573217956] [INFO] [bloom::asset] [asset_bank] detected SVG format
[35574968938] [INFO] [bloom::asset] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[35661753158] [INFO] [bloom::raster] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[35679961424] [INFO] [bloom::raster] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[35695254854] [INFO] [bloom::raster] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[35704079504] [INFO] [bloom::raster] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[35734600042] [INFO] [ingestd] INGESTD: Published new asset '/boot/echo' (hash=a11f50e00fa91378)
[35750172880] [INFO] [bloom::raster] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[35766986568] [INFO] [bloom::asset] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[35792412890] [INFO] [bloom::asset] [asset_bank] publish_cursor (pending)
[35798552235] [INFO] [bloom::asset] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[35806705356] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 194 (23272 bytes)
[35832117821] [INFO] [bloom::asset] [asset_bank] mapped at 0x11c96000
[35836326810] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[35868892362] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[36028969867] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[36036475213] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 197 (309408 bytes)
[36050016836] [INFO] [ingestd] INGESTD: Created File node '/boot/echo' (25008 bytes, hash=a11f50e00fa91378)
[36057011193] [INFO] [ingestd] INGESTD: Published asset '/boot/echo' (raw, 25008 bytes, hash=a11f50e00fa91378)
[36076107943] [INFO] [bloom::asset] [asset_bank] mapped at 0x11c9c000
[36080505967] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[36411179648] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[36642722003] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[36649550085] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 200 (569208 bytes)
[36673542557] [INFO] [bloom::asset] [asset_bank] mapped at 0x11dc3000
[36676545064] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[38310011213] [INFO] [ingestd] INGESTD: Published new asset '/boot/bloom' (hash=7904093ad49c5bd5)
[43233208471] [INFO] [ingestd] INGESTD: Created File node '/boot/bloom' (895496 bytes, hash=7904093ad49c5bd5)
[43235964819] [INFO] [ingestd] INGESTD: Published asset '/boot/bloom' (raw, 895496 bytes, hash=7904093ad49c5bd5)
[47863677099] [INFO] [drawlist_demo] Frame 4: color cycle, clip=true, scale=0.8

```
</details>
