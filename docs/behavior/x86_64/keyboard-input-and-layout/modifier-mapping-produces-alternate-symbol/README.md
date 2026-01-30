# ✅ Scenario: Modifier mapping produces alternate symbol

> Last run: 2026-01-29 22:19:33

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the clock window is ticking | ✅ | 32321ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I press Alt+A | ✅ | 2507ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the serial log should contain 'CONTRACT: input key_event' | ✅ | 1341ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And I should see the appropriate symbol rendered | ✅ | 207ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[20043667384] [CONTRACT] [kernel] thing-os kernel starting...
[20064992703] [INFO] [kernel::memory] Memory map has 64 entries
[20073322049] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[20079315965] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[20085064440] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[20094305280] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[20123827766] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[20129481935] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[20134817990] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[20140115083] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[20146138722] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78285000 (Usable)
[20151965669] [INFO] [kernel::memory]   [9] 0x78285000 - 0x782e9000 (Reserved)
[20157862199] [INFO] [kernel::memory]   [10] 0x782e9000 - 0x782ea000 (Other)
[20163410337] [INFO] [kernel::memory]   [11] 0x782ea000 - 0x782eb000 (Reserved)
[20169312742] [INFO] [kernel::memory]   [12] 0x782eb000 - 0x782ec000 (Other)
[20175041370] [INFO] [kernel::memory]   [13] 0x782ec000 - 0x782ed000 (Reserved)
[20180784564] [INFO] [kernel::memory]   [14] 0x782ed000 - 0x782ee000 (Other)
[20186528563] [INFO] [kernel::memory]   [15] 0x782ee000 - 0x782ef000 (Reserved)
[20192518983] [INFO] [kernel::memory]   [16] 0x782ef000 - 0x782f0000 (Other)
[20198115321] [INFO] [kernel::memory]   [17] 0x782f0000 - 0x782f1000 (Reserved)
[20203943417] [INFO] [kernel::memory]   [18] 0x782f1000 - 0x782f2000 (Other)
[20209577250] [INFO] [kernel::memory]   [19] 0x782f2000 - 0x782f3000 (Reserved)
[20215514038] [INFO] [kernel::memory]   [20] 0x782f3000 - 0x782f4000 (Other)
[20221160177] [INFO] [kernel::memory]   [21] 0x782f4000 - 0x782f5000 (Reserved)
[20226861910] [INFO] [kernel::memory]   [22] 0x782f5000 - 0x782f6000 (Other)
[20232572533] [INFO] [kernel::memory]   [23] 0x782f6000 - 0x782f7000 (Reserved)
[20238427978] [INFO] [kernel::memory]   [24] 0x782f7000 - 0x782f8000 (Other)
[20243946012] [INFO] [kernel::memory]   [25] 0x782f8000 - 0x782f9000 (Reserved)
[20249855567] [INFO] [kernel::memory]   [26] 0x782f9000 - 0x782fa000 (Other)
[20255433025] [INFO] [kernel::memory]   [27] 0x782fa000 - 0x782fb000 (Reserved)
[20261394848] [INFO] [kernel::memory]   [28] 0x782fb000 - 0x782fc000 (Other)
[20267063230] [INFO] [kernel::memory]   [29] 0x782fc000 - 0x782fd000 (Reserved)
[20272760740] [INFO] [kernel::memory]   [30] 0x782fd000 - 0x782fe000 (Other)
[20278605677] [INFO] [kernel::memory]   [31] 0x782fe000 - 0x782ff000 (Reserved)
[20284409939] [INFO] [kernel::memory]   [32] 0x782ff000 - 0x78300000 (Other)
[20289906880] [INFO] [kernel::memory]   [33] 0x78300000 - 0x78301000 (Reserved)
[20295762183] [INFO] [kernel::memory]   [34] 0x78301000 - 0x78302000 (Other)
[20301293708] [INFO] [kernel::memory]   [35] 0x78302000 - 0x78303000 (Reserved)
[20307299039] [INFO] [kernel::memory]   [36] 0x78303000 - 0x78304000 (Other)
[20313021336] [INFO] [kernel::memory]   [37] 0x78304000 - 0x78305000 (Reserved)
[20318743883] [INFO] [kernel::memory]   [38] 0x78305000 - 0x78306000 (Other)
[20324387857] [INFO] [kernel::memory]   [39] 0x78306000 - 0x78307000 (Reserved)
[20330455979] [INFO] [kernel::memory]   [40] 0x78307000 - 0x78308000 (Other)
[20336026760] [INFO] [kernel::memory]   [41] 0x78308000 - 0x78309000 (Reserved)
[20341976559] [INFO] [kernel::memory]   [42] 0x78309000 - 0x7830a000 (Other)
[20347580003] [INFO] [kernel::memory]   [43] 0x7830a000 - 0x7830b000 (Reserved)
[20353494814] [INFO] [kernel::memory]   [44] 0x7830b000 - 0x7830c000 (Other)
[20359159466] [INFO] [kernel::memory]   [45] 0x7830c000 - 0x7830d000 (Reserved)
[20364830142] [INFO] [kernel::memory]   [46] 0x7830d000 - 0x7830e000 (Other)
[20370358554] [INFO] [kernel::memory]   [47] 0x7830e000 - 0x7830f000 (Reserved)
[20376291197] [INFO] [kernel::memory]   [48] 0x7830f000 - 0x78310000 (Other)
[20381833232] [INFO] [kernel::memory]   [49] 0x78310000 - 0x78311000 (Reserved)
[20387699821] [INFO] [kernel::memory]   [50] 0x78311000 - 0x78312000 (Other)
[20393241630] [INFO] [kernel::memory]   [51] 0x78312000 - 0x78313000 (Reserved)
[20399093130] [INFO] [kernel::memory]   [52] 0x78313000 - 0x78314000 (Other)
[20405054123] [INFO] [kernel::memory]   [53] 0x78314000 - 0x78315000 (Reserved)
[20410801321] [INFO] [kernel::memory]   [54] 0x78315000 - 0x78316000 (Other)
[20416418363] [INFO] [kernel::memory]   [55] 0x78316000 - 0x78317000 (Reserved)
[20422220982] [INFO] [kernel::memory]   [56] 0x78317000 - 0x78318000 (Other)
[20427721249] [INFO] [kernel::memory]   [57] 0x78318000 - 0x78319000 (Reserved)
[20433534166] [INFO] [kernel::memory]   [58] 0x78319000 - 0x7831a000 (Other)
[20439131740] [INFO] [kernel::memory]   [59] 0x7831a000 - 0x7831b000 (Reserved)
[20445051405] [INFO] [kernel::memory]   [60] 0x7831b000 - 0x7831c000 (Other)
[20450661147] [INFO] [kernel::memory]   [61] 0x7831c000 - 0x7831d000 (Reserved)
[20456492147] [INFO] [kernel::memory]   [62] 0x7831d000 - 0x7831e000 (Other)
[20462170295] [INFO] [kernel::memory]   [63] 0x7831e000 - 0x7831f000 (Reserved)
[20468494457] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[20761035169] [CONTRACT] [kernel::memory] Frame allocator initialized with 488093 free frames
[20776097286] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[20792374229] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[20799217283] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[20804515653] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[20817992902] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[20823246342] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[20833205088] [INFO] [bran::arch] IOAPIC: Registers initialized
[20839998858] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[20847744328] [INFO] [bran::arch] IOAPIC: All pins masked
[20854666518] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[20860569876] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[20866132710] [INFO] [bran::arch] IOAPIC: Init complete
[20871352021] [CONTRACT] [kernel] Initializing global allocator...
[21297113498] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[21305221807] [CONTRACT] [kernel] Initializing SIMD...
[21311194358] [CONTRACT] [kernel] Initializing tasking...
[21327117344] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[21335063462] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[21342278238] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[21354952686] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[21360537616] [INFO] [kernel::task::scheduler]   Initializing boot task...
[21368149705] [INFO] [kernel::task::scheduler]   Creating boot task...
[21379028067] [INFO] [kernel::task::scheduler]   Creating idle task...
[21390086932] [INFO] [kernel::task::scheduler]   Boot task initialized
[21395546462] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[21402070049] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[21414529277] [INFO] [kernel::root] Spawning Root service...
[21429095753] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[21448266276] [INFO] [kernel::root::service] ROOT: started once
[26048240338] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[26055044622] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[26134102134] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[26172656322] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[26226335941] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[26290863024] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[26305286278] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[26377372148] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[26424466277] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[26444938574] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[26454268884] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=656, idx=3) BAR5=0x810c4000
[26503606414] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[26517385331] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[26525594001] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[26536388925] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[26546644440] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[26553295359] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[26572151143] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[26594184483] [INFO] [kernel::task::loader] Segment: vaddr=210000 exec=false
[26604946520] [INFO] [kernel::task::loader] Segment: vaddr=215000 exec=false
[26622326762] [INFO] [kernel] Warning: Module registry page overflow, truncating list.
[26630150456] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[26636447065] [CONTRACT] [kernel] Spawning init process...
[26643376873] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[26675429665] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (61907300 ticks/sec), init_cnt=619073 for 100Hz
[26685632675] [CONTRACT] [kernel] Entering scheduler loop.
[26690752474] [INFO] [kernel::tests::time_test] TIME ANCHORING TEST: Starting...
[26697975891] [INFO] [kernel::tests::time_test] TIME ANCHORING TEST: FAIL - sys_time_now returned 13348742706 but should have failed before anchor
[26709047645] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: Starting...
[26720063046] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: 1000 samples monotonic - PASS
[26729196354] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: trace::now() monotonic - PASS
[26738300723] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: trace and syscall consistent - PASS
[26747036710] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: All tests PASS
[26754534346] [INFO] [kernel::tests::fw_tables] FW_TABLES SELFTEST: Starting...
[26761888201] [INFO] [kernel::tests::fw_tables] FW_TABLES SELFTEST: PASS
[26793081240] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0008580
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[26810741741] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072367741888 RFLAGS_BEFORE=134 CR3_BEFORE=50331648 fs_base=0 gs_base=18446744071563860640
[26844204679] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9b0 rip=0x2013ff rflags=0x202
[26853915731] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[26860835122] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[26867811045] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[26899999939] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[26911509850] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[26922199770] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[26929060507] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[26940261903] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[26950325687] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[26956919985] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[26968416766] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[26975399913] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[26982605063] [INFO] [sprout::devtree] SPROUT: build() called
[26988609874] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[27006244734] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[27013590412] [INFO] [sprout] SPROUT: About to create Supervisor...
[27080554827] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[27088843170] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[27095935429] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[27111083255] [INFO] [sprout::supervisor] SPROUT: Found 64 modules
[27167957394] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[27185602660] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[27198952616] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[27212849109] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[27224277398] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[27236420409] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[27250935875] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[27263195713] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[27280604469] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[27292591437] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[27304817997] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[27318622172] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[27331172460] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[27343855143] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/fontd'
[27355547381] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/blossom'
[27368028247] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/ingestd'
[27379839598] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/cambium'
[27392758695] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/scheduler_fairness'
[27405923015] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/boot/drawlist_demo'
[27418335334] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/boot/tick_printer'
[27431552472] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/boot/photosynthesis'
[27443887378] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/boot/ata_disk'
[27456785888] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/boot/iso_reader'
[27469075749] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Alternate.cur'
[27483683297] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Busy.cur'
[27498229580] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Diagonal1.ani'
[27511517786] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Diagonal2.ani'
[27525014642] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Handwriting.cur'
[27538781159] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Help.cur'
[27552539769] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Horizontal.ani'
[27566449877] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Link.ani'
[27580476317] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Move.cur'
[27593790610] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Normal.cur'
[27607454822] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/cursors/plain/Precision.cur'
[27621074874] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/cursors/plain/Text.cur'
[27634639252] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/cursors/plain/Unavailabe.cur'
[27648420423] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/cursors/plain/Vertical.ani'
[27661998281] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/cursors/plain/Working.ani'
[27675653294] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/wallpapers/clouds.bmp'
[27688540746] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/wallpapers/leather.bmp'
[27701765787] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/wallpapers/linen.bmp'
[27714716400] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[27729203146] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/fonts/Hack-Regular.ttf'
[27742398735] [INFO] [sprout::supervisor] SPROUT: Module[42] = '/assets/fonts/NotoSans-Regular.ttf'
[27756101796] [INFO] [sprout::supervisor] SPROUT: Module[43] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[27770271306] [INFO] [sprout::supervisor] SPROUT: Module[44] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[27783978244] [INFO] [sprout::supervisor] SPROUT: Module[45] = '/assets/fonts/NotoSerif-Regular.ttf'
[27797572707] [INFO] [sprout::supervisor] SPROUT: Module[46] = '/assets/pci/pci.ids'
[27809716492] [INFO] [sprout::supervisor] SPROUT: Module[47] = '/assets/cursors/future/alias.svg'
[27823353932] [INFO] [sprout::supervisor] SPROUT: Module[48] = '/assets/cursors/future/all-scroll.svg'
[27837264343] [INFO] [sprout::supervisor] SPROUT: Module[49] = '/assets/cursors/future/bottom_left_corner.svg'
[27851478989] [INFO] [sprout::supervisor] SPROUT: Module[50] = '/assets/cursors/future/bottom_right_corner.svg'
[27866252177] [INFO] [sprout::supervisor] SPROUT: Module[51] = '/assets/cursors/future/bottom_side.svg'
[27879555116] [INFO] [sprout::supervisor] SPROUT: Module[52] = '/assets/cursors/future/cell.svg'
[27892943741] [INFO] [sprout::supervisor] SPROUT: Module[53] = '/assets/cursors/future/center_ptr.svg'
[27906592365] [INFO] [sprout::supervisor] SPROUT: Module[54] = '/assets/cursors/future/col-resize.svg'
[27920391133] [INFO] [sprout::supervisor] SPROUT: Module[55] = '/assets/cursors/future/color-picker.svg'
[27934921205] [INFO] [sprout::supervisor] SPROUT: Module[56] = '/assets/cursors/future/context-menu.svg'
[27948893595] [INFO] [sprout::supervisor] SPROUT: Module[57] = '/assets/cursors/future/copy.svg'
[27963244839] [INFO] [sprout::supervisor] SPROUT: Module[58] = '/assets/cursors/future/crosshair.svg'
[27977382508] [INFO] [sprout::supervisor] SPROUT: Module[59] = '/assets/cursors/future/default.svg'
[27991061704] [INFO] [sprout::supervisor] SPROUT: Module[60] = '/assets/cursors/future/dnd-move.svg'
[28005363644] [INFO] [sprout::supervisor] SPROUT: Module[61] = '/assets/cursors/future/dnd-no-drop.svg'
[28019404715] [INFO] [sprout::supervisor] SPROUT: Module[62] = '/assets/cursors/future/down-arrow.svg'
[28034343863] [INFO] [sprout::supervisor] SPROUT: Module[63] = '/assets/cursors/future/draft.svg'
[28046937366] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[28075188934] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[28310104952] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[28319336002] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[28328292878] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[28337036011] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[28345165296] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/fontd'
[28352776276] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/blossom'
[28361012134] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[28368697966] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/photosynthesis'
[28376836845] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/drawlist_demo'
[28385431619] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[28394995223] [INFO] [kernel::task::loader] Loading module: /boot/clock
[28401744610] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28411479638] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[28428371277] [INFO] [kernel::task::loader] Segment: vaddr=20b000 exec=false
[28438602943] [INFO] [kernel::task::loader] Segment: vaddr=20f000 exec=false
[28489290858] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0047260
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28527787895] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368149584 RFLAGS_BEFORE=134 CR3_BEFORE=50778112 fs_base=0 gs_base=18446744071563860640
[28554561132] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[28564801797] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[28573077913] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[28579918590] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28589467235] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[28606617487] [INFO] [kernel::task::loader] Segment: vaddr=20c000 exec=false
[28615328070] [INFO] [kernel::task::loader] Segment: vaddr=20e000 exec=false
[28630312238] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[28637603292] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[28644975903] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[28651476187] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28660998298] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[28688252089] [INFO] [kernel::task::loader] Segment: vaddr=218000 exec=false
[28701553268] [INFO] [kernel::task::loader] Segment: vaddr=220000 exec=false
[28716932614] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[28724058898] [INFO] [sprout::supervisor] SPROUT: Seeding initial asset requests...
[28761760668] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0043108
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28775230949] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368176784 RFLAGS_BEFORE=134 CR3_BEFORE=50929664 fs_base=0 gs_base=18446744071563860640
[28794385992] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044d30
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28807647773] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368194208 RFLAGS_BEFORE=134 CR3_BEFORE=51077120 fs_base=0 gs_base=18446744071563860640
[28822779126] [INFO] [ingestd] INGESTD: Starting unified content provider service...
[28830792953] [INFO] [ingestd] INGESTD: Initializing Limine module content source...
[28900191359] [INFO] [ingestd] INGESTD: Created Limine ContentSource node
[28906803948] [INFO] [ingestd] INGESTD: Performing initial boot module scan...
[28989222273] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/fontd'
[28996616706] [INFO] [kernel::task::loader] Loading module: /boot/fontd
[29002711598] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[29011436232] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[29046987873] [INFO] [kernel::task::loader] Segment: vaddr=225000 exec=false
[29058647438] [INFO] [kernel::task::loader] Segment: vaddr=22c000 exec=false
[29073368294] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[29079660982] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/blossom'
[29086611867] [INFO] [kernel::task::loader] Loading module: /boot/blossom
[29092830011] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[29101322558] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[29123825847] [INFO] [kernel::task::loader] Segment: vaddr=214000 exec=false
[29132390593] [INFO] [kernel::task::loader] Segment: vaddr=217000 exec=false
[29147006021] [INFO] [sprout::supervisor] SPROUT: App launched (PID=8)
[29153229793] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[29160753673] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[29166812268] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[29175252575] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[29187838093] [INFO] [kernel::task::loader] Segment: vaddr=207000 exec=false
[29195424806] [INFO] [kernel::task::loader] Segment: vaddr=209000 exec=false
[29229000784] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0049af0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29241599168] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368243424 RFLAGS_BEFORE=130 CR3_BEFORE=51302400 fs_base=0 gs_base=18446744071563860640
[29269777115] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004baf8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29282530086] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368259808 RFLAGS_BEFORE=130 CR3_BEFORE=51572736 fs_base=0 gs_base=18446744071563860640
[29298026240] [INFO] [blossom] BLOSSOM: Starting SVG Cache Service
[29304721265] [INFO] [blossom] BLOSSOM: Init UI pipeline...
[29319121358] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004d358
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29331803619] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368276192 RFLAGS_BEFORE=130 CR3_BEFORE=51757056 fs_base=0 gs_base=18446744071563860640
[29347144592] [INFO] [cambium] cambium starting (v5.2: drain-to-eagain + smart-resync)...
[29365497783] [INFO] [sprout::supervisor] SPROUT: App launched (PID=9)
[29372629971] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/photosynthesis'
[29380577003] [INFO] [kernel::task::loader] Loading module: /boot/photosynthesis
[29387571342] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[29397004491] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[29434340819] [INFO] [kernel::task::loader] Segment: vaddr=226000 exec=false
[29447442389] [INFO] [kernel::task::loader] Segment: vaddr=22e000 exec=false
[29463497962] [INFO] [sprout::supervisor] SPROUT: App launched (PID=10)
[29470625080] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/drawlist_demo'
[29479394621] [INFO] [kernel::task::loader] Loading module: /boot/drawlist_demo
[29486344240] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[29495633442] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[29512541315] [INFO] [kernel::task::loader] Segment: vaddr=20b000 exec=false
[29521714978] [INFO] [kernel::task::loader] Segment: vaddr=20e000 exec=false
[29537305293] [INFO] [sprout::supervisor] SPROUT: App launched (PID=11)
[29544800558] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[29585440206] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[29595253724] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[29604943152] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29615610134] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=552 subj_lo=0
[29646321515] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0049eb8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29659048272] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368321584 RFLAGS_BEFORE=134 CR3_BEFORE=51884032 fs_base=0 gs_base=18446744071563860640
[29674574244] [INFO] [photosynthesis] Photosynthesis starting...
[29689415131] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004cf38
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29702377149] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368338976 RFLAGS_BEFORE=130 CR3_BEFORE=52162560 fs_base=0 gs_base=18446744071563860640
[29718226401] [INFO] [drawlist_demo] DrawList demo starting (with new commands)...
[29783298725] [INFO] [ingestd] INGESTD: Published new asset '/boot/sprout' (hash=61811fc99aafd0a2)
[29799265932] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[29808041386] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[29817376978] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29827495998] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=587 subj_lo=0
[29852302545] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f8fb0
[29861096460] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[29870580537] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[29880319090] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=589 subj_lo=0
[29909707178] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[29918736362] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[29927957344] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29938216399] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=594 subj_lo=0
[29957792117] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[29965139020] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1920x1080 stride=7680)
[30573756302] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f8fb0
[30582587990] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[30591723750] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[30601553164] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=595 subj_lo=0
[30632042790] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[30641577258] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[30650762886] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[30660503114] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=601 subj_lo=0
[30704975146] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[30713977917] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[30723411685] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[30733387795] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=605 subj_lo=0
[30772351761] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[30781052105] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[30790223180] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[30800002427] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=609 subj_lo=0
[30843532555] [INFO] [ingestd] INGESTD: Created File node '/boot/sprout' (90640 bytes, hash=61811fc99aafd0a2)
[30855332627] [INFO] [ingestd] INGESTD: Published asset '/boot/sprout' (raw, 90640 bytes, hash=61811fc99aafd0a2)
[30910003225] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[30917254133] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[30926740731] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[30938633127] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[30946366465] [INFO] [kernel::task::loader] Segment: vaddr=206000 exec=false
[30962926519] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=12)
[30988421180] [INFO] [blossom] BLOSSOM: UI pipeline ready
[31003153481] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0049eb8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x70006
[31016607206] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368402128 RFLAGS_BEFORE=130 CR3_BEFORE=60870656 fs_base=0 gs_base=18446744071563860640
[31033926626] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=6, drv_resp_w=7)
[31088369206] [INFO] [blossom] BLOSSOM: Service node created, req=9, resp=12
[31096894635] [INFO] [blossom] BLOSSOM: Service ready
[31138947751] [INFO] [kernel::syscall::handlers::device] DEVICE: task 12 claimed device 631 (handle 0)
[31169256946] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x7e9000 -> virt=0x1001f000
[31192547366] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[31201653093] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[31208789532] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[31218696382] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[31231203177] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[31239849685] [INFO] [kernel::task::loader] Segment: vaddr=207000 exec=false
[31256454903] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=13)
[31264464256] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[31273730868] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=13, r=14)
[31283062862] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=15, r=16)
[31292132842] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=17, r=18)
[31299590204] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[31306667649] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[31316271627] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[31328009460] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[31336144962] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[31353213741] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=14)
[31361022406] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[31367854094] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[31377543242] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[31389278491] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[31397627927] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[31414282488] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=15)
[31450455480] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0051e40
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x28b
[31464787082] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369438800 RFLAGS_BEFORE=134 CR3_BEFORE=61005824 fs_base=0 gs_base=18446744071563860640
[31482215500] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[31492509274] [INFO] [rtc_cmos] Starting... arg=28b
[31499768954] [INFO] [rtc_cmos] Serving device ID: ThingId([139, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[31512720472] [INFO] [rtc_cmos] RTC: 2026-01-29 22:22:19 = 1769725339 unix_secs
[31520729223] [INFO] [kernel::time] System clock anchored: unix_secs=1769725339, mono_ns=15760068404, offset=1769725323239931596ns
[31533320194] [INFO] [rtc_cmos] System clock anchored
[31560631712] [INFO] [rtc_cmos] RTC: Set sys.TimeState = 1 (Anchored)
[31592810527] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[31614260903] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00560a0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xd
[31627617536] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369471568 RFLAGS_BEFORE=134 CR3_BEFORE=61124608 fs_base=0 gs_base=18446744071563860640
[31644414778] [INFO] [ps2_kbd] ps2_kbd: online (handle=13)
[31659992563] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb005d530
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xf
[31673169523] [INFO] [task.user_enter] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369487952 RFLAGS_BEFORE=134 CR3_BEFORE=61235200 fs_base=0 gs_base=18446744071563860640
[31689806102] [INFO] [ps2_mouse] ps2_mouse: online (handle=15)
[31696635798] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[31714932339] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=19, r=20)
[31723045975] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[31730301593] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[31740396194] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[31752328692] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[31761084870] [INFO] [kernel::task::loader] Segment: vaddr=206000 exec=false
[31777811953] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=16)
[31807073726] [INFO] [ps2_kbd] ps2_kbd: created driver node 976
[31814910416] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[31824315730] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[31838466289] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00560a0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe001000110013
[31852501387] [INFO] [task.user_enter] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369526112 RFLAGS_BEFORE=134 CR3_BEFORE=61345792 fs_base=0 gs_base=18446744071563860640
[31869014374] [INFO] [bristle] bristle: online (kbd=14, mouse=16, evt=17, evt_echo=19)
[31901255066] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[31909213016] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[31918005401] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
[31935744016] [INFO] [bristle] bristle: registered in graph as svc.Input (id=996)
[31959772084] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[31971965360] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=988 backend=BootFB
[31980360459] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[31986892786] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[31996148723] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[32160594078] [INFO] [kernel::task::loader] Segment: vaddr=2c5000 exec=false
[32183325561] [INFO] [kernel::task::loader] Segment: vaddr=2d9000 exec=false
[32200401973] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=17)
[32208326609] [INFO] [kernel::task::loader] Loading module: /boot/echo
[32214917897] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[32224336310] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[32235807049] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[32243654791] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[32260087982] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=18)
[32267505316] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[32275482273] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[32314013681] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004f0b0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x3dc
[32327643457] [INFO] [task.user_enter] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369559568 RFLAGS_BEFORE=130 CR3_BEFORE=61468672 fs_base=0 gs_base=18446744071563860640
[32343722118] [INFO] [bloom::logging] bloom: logging initialized
[32379790685] [INFO] [bloom] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[32397108040] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004fb00
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x14
[32410447611] [INFO] [task.user_enter] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369575952 RFLAGS_BEFORE=130 CR3_BEFORE=62447616 fs_base=0 gs_base=18446744071563860640
[32427018744] [INFO] [echo] echo: online (handle=20)
[32433159553] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[32477621924] [INFO] [ingestd] INGESTD: Published new asset '/boot/bristle' (hash=d55e2ab85adcac5d)
[32566525015] [INFO] [bloom::compositor] bloom: compositor bytespace 895 (1920x1080 stride=7680 format=2)
[32605430641] [INFO] [ingestd] INGESTD: Created File node '/boot/bristle' (29104 bytes, hash=d55e2ab85adcac5d)
[32615182602] [INFO] [ingestd] INGESTD: Published asset '/boot/bristle' (raw, 29104 bytes, hash=d55e2ab85adcac5d)
[32634553235] [INFO] [bloom::compositor] bloom: display backend: BootFB
[32654541127] [INFO] [bloom::compositor] bloom: mapped size=8294400 (source=bytespace_info)
[32717702856] [INFO] [stem::ui] UiBuilder: created root 1032
[32738613068] [INFO] [bloom] bloom: spawned asset watcher (tid=19)
[32745508185] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[32763912245] [INFO] [photosynthesis] Found UI Root: 1032
T:3C90 [32783781354] [INFO] [bloom::painter_resources] [bloom] asset_watcher_entry: spawning sub-loaders
T:41D0 T:4040 T:2A00 [32874116169] [INFO] [ps2_mouse] ps2_mouse: init done
[32879795009] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[32888657106] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[32897388565] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[32947843789] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f64e0
[32956455299] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[32965623271] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[32975501102] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=741 subj_lo=0
[33033670738] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f64e0
[33042518443] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[33051479152] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[33061373357] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=643 pred=0 subj_lo=0
[33116941592] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f64e0
[33125621301] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[33134665461] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[33144290193] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=747 subj_lo=0
[33163615946] [INFO] [bloom::painter_resources] [bloom] wallpaper loader: loading leather.bmp
[33182104289] [INFO] [bloom::asset] [asset_bank] worker spawned tid=23 (priority=2)
[33195865161] [INFO] [bloom::painter_resources] [bloom] cursor loader: loading default cursor
[33207108901] [INFO] [bloom::painter_resources] [bloom] icon loader started
[33236601190] [INFO] [bloom] [bloom] Starting UI loop immediately (not waiting for fonts)
T:4C30 [33249198479] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[33257318416] [INFO] [bloom::asset] [asset_bank] load_wallpaper_immediate: leather.bmp
[33306366124] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef60
[33307998717] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[33309544034] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[33311245403] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x0 kind=0 pred=0 subj_lo=0
[33391954123] [INFO] [ingestd] INGESTD: Published new asset '/boot/rtc_cmos' (hash=32366e1092006840)
[33573657461] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7feba0
[33574919092] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[33576420983] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[33578229282] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=770 pred=0 subj_lo=0
[33631466884] [INFO] [ingestd] INGESTD: Created File node '/boot/rtc_cmos' (33368 bytes, hash=32366e1092006840)
[33633281244] [INFO] [ingestd] INGESTD: Published asset '/boot/rtc_cmos' (raw, 33368 bytes, hash=32366e1092006840)
[33973445110] [INFO] [ingestd] INGESTD: Published new asset '/boot/clock' (hash=771f0ce19bac6901)
[34127746945] [INFO] [ingestd] INGESTD: Created File node '/boot/clock' (66064 bytes, hash=771f0ce19bac6901)
[34129677152] [INFO] [ingestd] INGESTD: Published asset '/boot/clock' (raw, 66064 bytes, hash=771f0ce19bac6901)
[34155720151] [INFO] [bloom::asset] [asset_bank] mapping bytespace 188 (4718646 bytes) for 'leather.bmp'
[34186960870] [INFO] [bloom::asset] [asset_bank] decoding BMP for 'leather.bmp'...
[34868951318] [INFO] [bloom::asset] [asset_bank] BMP decoded: 1536x1024 for 'leather.bmp'
[35934962277] [INFO] [bloom::asset] [asset_bank] publish_wallpaper (pending): 1536x1024
[35937431356] [INFO] [bloom::asset] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[36002943227] [INFO] [bloom::present] bloom: driver REGISTER (kind=1 caps=0x3)
[36007757786] [INFO] [bloom::present] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=8
[36031391469] [INFO] [bloom::reclaimer] [reclaimer] +6291456 bytes (total: 6291456)
[36033544848] [INFO] [bloom::asset] [asset_bank] promoting wallpaper 'leather.bmp' to gen=1 (6291456b)
[36074181092] [INFO] [display_bootfb] display_bootfb: bound bytespace 895
[36211733456] [INFO] [ingestd] INGESTD: Published new asset '/boot/font_explorer' (hash=1cdbe11025543439)
[36401432994] [INFO] [ingestd] INGESTD: Created File node '/boot/font_explorer' (61968 bytes, hash=1cdbe11025543439)
[36406663951] [INFO] [ingestd] INGESTD: Published asset '/boot/font_explorer' (raw, 61968 bytes, hash=1cdbe11025543439)
[36654970987] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x10040870
[36661327085] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[36665937806] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36667486544] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=800 pred=0 subj_lo=0
[36714950900] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x10040870
[36720781147] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[36724664090] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36726166974] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=770 pred=0 subj_lo=0
[36745912958] [INFO] [ingestd] INGESTD: Published new asset '/boot/ps2_kbd' (hash=4da765f0b8256772)
[36775016610] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x10040870
[36781535077] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[36783581808] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36785273948] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=801 pred=0 subj_lo=0
[36832760661] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x10040870
[36839832621] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[36843480291] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36845083990] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=802 pred=0 subj_lo=0
[36988023274] [INFO] [ingestd] INGESTD: Created File node '/boot/ps2_kbd' (25008 bytes, hash=4da765f0b8256772)
[36997314718] [INFO] [ingestd] INGESTD: Published asset '/boot/ps2_kbd' (raw, 25008 bytes, hash=4da765f0b8256772)
[37141364159] [INFO] [bloom::present] bloom: driver BIND ACK
[37143839089] [INFO] [bloom] [CONTRACT] [bloom] First frame rendered
[37305195860] [INFO] [bloom::asset] [asset_bank] mapping bytespace 251 (3051 bytes)
[37331147882] [INFO] [bloom::asset] [asset_bank] mapped to 0x11c95000
[37334157399] [INFO] [bloom::asset] [asset_bank] detected SVG format
[37335627246] [INFO] [bloom::asset] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[37635176006] [INFO] [bloom::raster] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[37650613012] [INFO] [bloom::raster] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[37662068855] [INFO] [bloom::raster] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[37665422711] [INFO] [bloom::raster] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[37668154285] [INFO] [bloom::raster] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[37673817183] [INFO] [bloom::asset] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[37714313205] [INFO] [bloom::asset] [asset_bank] publish_cursor (pending)
[37719947876] [INFO] [bloom::asset] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[37722060438] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 194 (23272 bytes)
[37741017866] [INFO] [ingestd] INGESTD: Published new asset '/boot/echo' (hash=a11f50e00fa91378)
[37759833842] [INFO] [bloom::asset] [asset_bank] mapped at 0x11c96000
[37765257179] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[37800025445] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[37934708609] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[37942939699] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 197 (309408 bytes)
[37970037786] [INFO] [bloom::asset] [asset_bank] mapped at 0x11c9c000
[37993721970] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[38214381852] [INFO] [ingestd] INGESTD: Created File node '/boot/echo' (25008 bytes, hash=a11f50e00fa91378)
[38221939854] [INFO] [ingestd] INGESTD: Published asset '/boot/echo' (raw, 25008 bytes, hash=a11f50e00fa91378)
[38387893671] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[38636960966] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[38643220303] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 200 (569208 bytes)
[38672597827] [INFO] [bloom::asset] [asset_bank] mapped at 0x11dc3000
[38678273990] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[41016551579] [INFO] [ingestd] INGESTD: Published new asset '/boot/bloom' (hash=7904093ad49c5bd5)
[44517323089] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[44532533603] [INFO] [ingestd] INGESTD: Created File node '/boot/bloom' (895496 bytes, hash=7904093ad49c5bd5)
[44534754108] [INFO] [ingestd] INGESTD: Published asset '/boot/bloom' (raw, 895496 bytes, hash=7904093ad49c5bd5)
[44926728583] [INFO] [bloom::reclaimer] [reclaimer] +16384 bytes (total: 6307840)
[44928915301] [INFO] [bloom::asset] [asset_bank] promoting cursor to gen=2 (16384b)
[44931582426] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 6410240)
[44933764962] [INFO] [bloom::asset] [asset_bank] promoting font 'DSEG7Classic-Regular.ttf' to gen=2 (102400b) in slot 0
[44936975611] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 6512640)
[44939051388] [INFO] [bloom::asset] [asset_bank] promoting font 'Hack-Regular.ttf' to gen=2 (102400b) in slot 1
[44984763611] [INFO] [clock] CLOCK PUBLISH: thing=789 now_text='22:22:25' tick=22402916465
[45015638133] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[45017853196] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 203 (258156 bytes)
[45040561645] [INFO] [bloom::asset] [asset_bank] mapped at 0x11e55000
[45042706966] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[46799411507] [INFO] [ingestd] INGESTD: Published new asset '/boot/ps2_mouse' (hash=8d3523c823935b52)
[47077533552] [INFO] [clock] CLOCK PUBLISH: thing=789 now_text='22:22:26' tick=22800990171
[47652707316] [INFO] [drawlist_demo] Frame 4: color cycle, clip=true, scale=0.8
[47722450016] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[47855210209] [INFO] [ingestd] INGESTD: Created File node '/boot/ps2_mouse' (25008 bytes, hash=8d3523c823935b52)
[47857466012] [INFO] [ingestd] INGESTD: Published asset '/boot/ps2_mouse' (raw, 25008 bytes, hash=8d3523c823935b52)
[47868181474] [INFO] [clock] CLOCK PUBLISH: thing=789 now_text='22:22:27' tick=23869799136
[47918017442] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[47920353791] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 206 (656852 bytes)
[47937900095] [INFO] [bloom::asset] [asset_bank] mapped at 0x11e95000
[47940127920] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[51742255130] [INFO] [ingestd] INGESTD: Published new asset '/boot/root_batch_bench' (hash=323a9e6726f38ab3)
[52980715212] [INFO] [clock] CLOCK PUBLISH: thing=789 now_text='22:22:28' tick=25097529977
[55819615635] [INFO] [ingestd] INGESTD: Created File node '/boot/root_batch_bench' (29200 bytes, hash=323a9e6726f38ab3)
[55822937681] [INFO] [ingestd] INGESTD: Published asset '/boot/root_batch_bench' (raw, 29200 bytes, hash=323a9e6726f38ab3)
[55830884352] [INFO] [clock] CLOCK PUBLISH: thing=789 now_text='22:22:30' tick=26953638190
[59898126990] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[59915659349] [INFO] [bloom::cursor_rasterizer] [cursor_rasterizer] rasterizing cursor snapshot gen=2
[59979062794] [INFO] [clock] CLOCK PUBLISH: thing=789 now_text='22:22:32' tick=29220584024
[60000235386] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 6615040)
[60002885846] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=3 (102400b) in slot 2
[60006674284] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 6717440)
[60008740553] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol-Regular.ttf' to gen=3 (102400b) in slot 3
[60063449754] [INFO] [ingestd] INGESTD: Published new asset '/boot/root_watch_tester' (hash=5b29257b2e16f32e)
[60136634499] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol2-Regular.ttf' in slot 4
[60138985767] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 209 (616196 bytes)
[60161872073] [INFO] [bloom::asset] [asset_bank] mapped at 0x11f49000
[60165373196] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSerif-Regular.ttf'...
[61196354024] [INFO] [ingestd] INGESTD: Created File node '/boot/root_watch_tester' (41488 bytes, hash=5b29257b2e16f32e)
[61198825102] [INFO] [ingestd] INGESTD: Published asset '/boot/root_watch_tester' (raw, 41488 bytes, hash=5b29257b2e16f32e)
[62661159467] [INFO] [drawlist_demo] Frame 8: color cycle, clip=true, scale=1
[63913586152] [INFO] [clock] CLOCK PUBLISH: thing=789 now_text='22:22:34' tick=31104438396
[66262173774] [INFO] [ingestd] INGESTD: Published new asset '/boot/display_bootfb' (hash=e581a9426f17f21e)
[68171898167] [INFO] [clock] CLOCK PUBLISH: thing=789 now_text='22:22:36' tick=33144944571
[69167024327] [INFO] [ingestd] INGESTD: Created File node '/boot/display_bootfb' (29272 bytes, hash=e581a9426f17f21e)
[69175290548] [INFO] [ingestd] INGESTD: Published asset '/boot/display_bootfb' (raw, 29272 bytes, hash=e581a9426f17f21e)
[72310543124] [INFO] [echo] KeyDown LAlt +Alt
[73448132524] [INFO] [clock] CLOCK PUBLISH: thing=789 now_text='22:22:38' tick=35119068780
[76481173551] [INFO] [clock] CLOCK PUBLISH: thing=789 now_text='22:22:40' tick=36954740959
[76732653759] [INFO] [ingestd] INGESTD: Published new asset '/boot/fontd' (hash=a6d7f7d6cc48065e)
[77692912950] [INFO] [drawlist_demo] Frame 12: color cycle, clip=true, scale=0.8
[79129639704] [INFO] [echo] KeyDown A +Alt
[80394832970] [INFO] [clock] CLOCK PUBLISH: thing=789 now_text='22:22:42' tick=38973046896
[80659469493] [INFO] [echo] KeyUp A +Alt
[80924716643] [INFO] [ingestd] INGESTD: Created File node '/boot/fontd' (184848 bytes, hash=a6d7f7d6cc48065e)
[80932146786] [INFO] [ingestd] INGESTD: Published asset '/boot/fontd' (raw, 184848 bytes, hash=a6d7f7d6cc48065e)
[82635954270] [INFO] [echo] KeyUp LAlt
[85253101972] [INFO] [clock] CLOCK PUBLISH: thing=789 now_text='22:22:44' tick=41200111992
[89694997691] [INFO] [clock] CLOCK PUBLISH: thing=789 now_text='22:22:46' tick=43038818140
[89919327894] [INFO] [ingestd] INGESTD: Published new asset '/boot/blossom' (hash=e5637a1100e504c3)
[93514944147] [INFO] [drawlist_demo] Frame 16: color cycle, clip=true, scale=1
[94202387043] [INFO] [ingestd] INGESTD: Created File node '/boot/blossom' (98832 bytes, hash=e5637a1100e504c3)
[94209855371] [INFO] [ingestd] INGESTD: Published asset '/boot/blossom' (raw, 98832 bytes, hash=e5637a1100e504c3)

```
</details>
