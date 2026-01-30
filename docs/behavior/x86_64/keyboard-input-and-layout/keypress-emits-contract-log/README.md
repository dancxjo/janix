# ❌ Scenario: Keypress emits contract log

> Last run: 2026-01-29 22:19:33

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the clock window is ticking | ✅ | 44983ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | When I press a key | ❌ | 1018ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[19357629251] [CONTRACT] [kernel] thing-os kernel starting...
[19379467414] [INFO] [kernel::memory] Memory map has 64 entries
[19387720681] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[19422032594] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[19427910761] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[19433545294] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[19439034796] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[19444661911] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[19450175473] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[19455716714] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[19461649668] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78285000 (Usable)
[19467381075] [INFO] [kernel::memory]   [9] 0x78285000 - 0x782e9000 (Reserved)
[19473592677] [INFO] [kernel::memory]   [10] 0x782e9000 - 0x782ea000 (Other)
[19479298058] [INFO] [kernel::memory]   [11] 0x782ea000 - 0x782eb000 (Reserved)
[19485307351] [INFO] [kernel::memory]   [12] 0x782eb000 - 0x782ec000 (Other)
[19491279195] [INFO] [kernel::memory]   [13] 0x782ec000 - 0x782ed000 (Reserved)
[19497161673] [INFO] [kernel::memory]   [14] 0x782ed000 - 0x782ee000 (Other)
[19503114694] [INFO] [kernel::memory]   [15] 0x782ee000 - 0x782ef000 (Reserved)
[19509264000] [INFO] [kernel::memory]   [16] 0x782ef000 - 0x782f0000 (Other)
[19514999179] [INFO] [kernel::memory]   [17] 0x782f0000 - 0x782f1000 (Reserved)
[19521144714] [INFO] [kernel::memory]   [18] 0x782f1000 - 0x782f2000 (Other)
[19527235003] [INFO] [kernel::memory]   [19] 0x782f2000 - 0x782f3000 (Reserved)
[19533401962] [INFO] [kernel::memory]   [20] 0x782f3000 - 0x782f4000 (Other)
[19539192853] [INFO] [kernel::memory]   [21] 0x782f4000 - 0x782f5000 (Reserved)
[19545219604] [INFO] [kernel::memory]   [22] 0x782f5000 - 0x782f6000 (Other)
[19551008349] [INFO] [kernel::memory]   [23] 0x782f6000 - 0x782f7000 (Reserved)
[19556908106] [INFO] [kernel::memory]   [24] 0x782f7000 - 0x782f8000 (Other)
[19562635319] [INFO] [kernel::memory]   [25] 0x782f8000 - 0x782f9000 (Reserved)
[19568556336] [INFO] [kernel::memory]   [26] 0x782f9000 - 0x782fa000 (Other)
[19574393848] [INFO] [kernel::memory]   [27] 0x782fa000 - 0x782fb000 (Reserved)
[19580372040] [INFO] [kernel::memory]   [28] 0x782fb000 - 0x782fc000 (Other)
[19586245204] [INFO] [kernel::memory]   [29] 0x782fc000 - 0x782fd000 (Reserved)
[19592220863] [INFO] [kernel::memory]   [30] 0x782fd000 - 0x782fe000 (Other)
[19597900796] [INFO] [kernel::memory]   [31] 0x782fe000 - 0x782ff000 (Reserved)
[19603934720] [INFO] [kernel::memory]   [32] 0x782ff000 - 0x78300000 (Other)
[19609753734] [INFO] [kernel::memory]   [33] 0x78300000 - 0x78301000 (Reserved)
[19615647272] [INFO] [kernel::memory]   [34] 0x78301000 - 0x78302000 (Other)
[19621350149] [INFO] [kernel::memory]   [35] 0x78302000 - 0x78303000 (Reserved)
[19627291502] [INFO] [kernel::memory]   [36] 0x78303000 - 0x78304000 (Other)
[19632997232] [INFO] [kernel::memory]   [37] 0x78304000 - 0x78305000 (Reserved)
[19639148092] [INFO] [kernel::memory]   [38] 0x78305000 - 0x78306000 (Other)
[19645096886] [INFO] [kernel::memory]   [39] 0x78306000 - 0x78307000 (Reserved)
[19651099779] [INFO] [kernel::memory]   [40] 0x78307000 - 0x78308000 (Other)
[19656964350] [INFO] [kernel::memory]   [41] 0x78308000 - 0x78309000 (Reserved)
[19662802105] [INFO] [kernel::memory]   [42] 0x78309000 - 0x7830a000 (Other)
[19668757560] [INFO] [kernel::memory]   [43] 0x7830a000 - 0x7830b000 (Reserved)
[19674712528] [INFO] [kernel::memory]   [44] 0x7830b000 - 0x7830c000 (Other)
[19680454400] [INFO] [kernel::memory]   [45] 0x7830c000 - 0x7830d000 (Reserved)
[19686544682] [INFO] [kernel::memory]   [46] 0x7830d000 - 0x7830e000 (Other)
[19692300960] [INFO] [kernel::memory]   [47] 0x7830e000 - 0x7830f000 (Reserved)
[19698176754] [INFO] [kernel::memory]   [48] 0x7830f000 - 0x78310000 (Other)
[19703937301] [INFO] [kernel::memory]   [49] 0x78310000 - 0x78311000 (Reserved)
[19709996978] [INFO] [kernel::memory]   [50] 0x78311000 - 0x78312000 (Other)
[19715942876] [INFO] [kernel::memory]   [51] 0x78312000 - 0x78313000 (Reserved)
[19722084441] [INFO] [kernel::memory]   [52] 0x78313000 - 0x78314000 (Other)
[19727806650] [INFO] [kernel::memory]   [53] 0x78314000 - 0x78315000 (Reserved)
[19733797060] [INFO] [kernel::memory]   [54] 0x78315000 - 0x78316000 (Other)
[19739610408] [INFO] [kernel::memory]   [55] 0x78316000 - 0x78317000 (Reserved)
[19745518870] [INFO] [kernel::memory]   [56] 0x78317000 - 0x78318000 (Other)
[19751373917] [INFO] [kernel::memory]   [57] 0x78318000 - 0x78319000 (Reserved)
[19757339801] [INFO] [kernel::memory]   [58] 0x78319000 - 0x7831a000 (Other)
[19763198079] [INFO] [kernel::memory]   [59] 0x7831a000 - 0x7831b000 (Reserved)
[19769220405] [INFO] [kernel::memory]   [60] 0x7831b000 - 0x7831c000 (Other)
[19775174990] [INFO] [kernel::memory]   [61] 0x7831c000 - 0x7831d000 (Reserved)
[19781289818] [INFO] [kernel::memory]   [62] 0x7831d000 - 0x7831e000 (Other)
[19787109175] [INFO] [kernel::memory]   [63] 0x7831e000 - 0x7831f000 (Reserved)
[19793841020] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[20096449275] [CONTRACT] [kernel::memory] Frame allocator initialized with 488093 free frames
[20112305530] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[20129317777] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[20136606521] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[20142263207] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[20156241508] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[20161605878] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[20170923362] [INFO] [bran::arch] IOAPIC: Registers initialized
[20177230853] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[20184345723] [INFO] [bran::arch] IOAPIC: All pins masked
[20191002929] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[20196774139] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[20202448741] [INFO] [bran::arch] IOAPIC: Init complete
[20209829016] [CONTRACT] [kernel] Initializing global allocator...
[20650907812] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[20660341182] [CONTRACT] [kernel] Initializing SIMD...
[20667626725] [CONTRACT] [kernel] Initializing tasking...
[20685038372] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[20694423729] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[20702151031] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[20716894956] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[20722986492] [INFO] [kernel::task::scheduler]   Initializing boot task...
[20731125952] [INFO] [kernel::task::scheduler]   Creating boot task...
[20742707021] [INFO] [kernel::task::scheduler]   Creating idle task...
[20754984160] [INFO] [kernel::task::scheduler]   Boot task initialized
[20761072844] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[20767823787] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[20781145551] [INFO] [kernel::root] Spawning Root service...
[20796652885] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[20817282957] [INFO] [kernel::root::service] ROOT: started once
[25032342511] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[25039198732] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[25106858515] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[25145910833] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[25203822089] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[25269683418] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[25282656276] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[25357128321] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[25405488564] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[25426854675] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[25436460564] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=656, idx=3) BAR5=0x810c4000
[25486882309] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[25500749950] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[25508693963] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[25518979690] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[25548036866] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[25555051263] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[25602318322] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[25624352806] [INFO] [kernel::task::loader] Segment: vaddr=210000 exec=false
[25636161582] [INFO] [kernel::task::loader] Segment: vaddr=215000 exec=false
[25652496697] [INFO] [kernel] Warning: Module registry page overflow, truncating list.
[25660983012] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[25667466834] [CONTRACT] [kernel] Spawning init process...
[25674531557] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[25706196629] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (61929500 ticks/sec), init_cnt=619295 for 100Hz
[25716860603] [CONTRACT] [kernel] Entering scheduler loop.
[25721985573] [INFO] [kernel::tests::time_test] TIME ANCHORING TEST: Starting...
[25729473079] [INFO] [kernel::tests::time_test] TIME ANCHORING TEST: FAIL - sys_time_now returned 12864393813 but should have failed before anchor
[25740494223] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: Starting...
[25751936665] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: 1000 samples monotonic - PASS
[25761084992] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: trace::now() monotonic - PASS
[25770546561] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: trace and syscall consistent - PASS
[25779567808] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: All tests PASS
[25787168142] [INFO] [kernel::tests::fw_tables] FW_TABLES SELFTEST: Starting...
[25794632429] [INFO] [kernel::tests::fw_tables] FW_TABLES SELFTEST: PASS
[25825485020] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0008580
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[25843454782] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072367741888 RFLAGS_BEFORE=134 CR3_BEFORE=50331648 fs_base=0 gs_base=18446744071563860640
[25877618451] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9b0 rip=0x2013ff rflags=0x202
[25887322802] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[25894497921] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[25901420665] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[25934085625] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[25945547287] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[25957178802] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[25963917651] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[25975111271] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[25985720271] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[25992246426] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[26004304326] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[26010687887] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[26018102540] [INFO] [sprout::devtree] SPROUT: build() called
[26024363046] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[26042183818] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[26131081958] [INFO] [sprout] SPROUT: About to create Supervisor...
[26138338709] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[26146536971] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[26153796878] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[26170837735] [INFO] [sprout::supervisor] SPROUT: Found 64 modules
[26231761222] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[26248068771] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[26261913552] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[26275181275] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[26286859542] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[26299450343] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[26313245154] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[26326301882] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[26337592555] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[26350024611] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[26362746026] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[26377619325] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[26392313715] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[26406027835] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/fontd'
[26419218452] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/blossom'
[26432527666] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/ingestd'
[26445863210] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/cambium'
[26459148039] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/scheduler_fairness'
[26472811116] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/boot/drawlist_demo'
[26486438961] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/boot/tick_printer'
[26500046266] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/boot/photosynthesis'
[26513867915] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/boot/ata_disk'
[26528429505] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/boot/iso_reader'
[26541665455] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Alternate.cur'
[26563144520] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Busy.cur'
[26577928576] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Diagonal1.ani'
[26592667467] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Diagonal2.ani'
[26607940736] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Handwriting.cur'
[26623478388] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Help.cur'
[26637989250] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Horizontal.ani'
[26652831968] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Link.ani'
[26668243099] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Move.cur'
[26682474839] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Normal.cur'
[26697249174] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/cursors/plain/Precision.cur'
[26712237468] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/cursors/plain/Text.cur'
[26726521047] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/cursors/plain/Unavailabe.cur'
[26741666235] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/cursors/plain/Vertical.ani'
[26756578605] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/cursors/plain/Working.ani'
[26771206347] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/wallpapers/clouds.bmp'
[26787143958] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/wallpapers/leather.bmp'
[26801278274] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/wallpapers/linen.bmp'
[26815261990] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[26830448632] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/fonts/Hack-Regular.ttf'
[26845031840] [INFO] [sprout::supervisor] SPROUT: Module[42] = '/assets/fonts/NotoSans-Regular.ttf'
[26859666734] [INFO] [sprout::supervisor] SPROUT: Module[43] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[26874877601] [INFO] [sprout::supervisor] SPROUT: Module[44] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[26889524072] [INFO] [sprout::supervisor] SPROUT: Module[45] = '/assets/fonts/NotoSerif-Regular.ttf'
[26904197253] [INFO] [sprout::supervisor] SPROUT: Module[46] = '/assets/pci/pci.ids'
[26918073646] [INFO] [sprout::supervisor] SPROUT: Module[47] = '/assets/cursors/future/alias.svg'
[26932203462] [INFO] [sprout::supervisor] SPROUT: Module[48] = '/assets/cursors/future/all-scroll.svg'
[26947331691] [INFO] [sprout::supervisor] SPROUT: Module[49] = '/assets/cursors/future/bottom_left_corner.svg'
[26962614316] [INFO] [sprout::supervisor] SPROUT: Module[50] = '/assets/cursors/future/bottom_right_corner.svg'
[26977888789] [INFO] [sprout::supervisor] SPROUT: Module[51] = '/assets/cursors/future/bottom_side.svg'
[26992725576] [INFO] [sprout::supervisor] SPROUT: Module[52] = '/assets/cursors/future/cell.svg'
[27007693842] [INFO] [sprout::supervisor] SPROUT: Module[53] = '/assets/cursors/future/center_ptr.svg'
[27022385251] [INFO] [sprout::supervisor] SPROUT: Module[54] = '/assets/cursors/future/col-resize.svg'
[27037981957] [INFO] [sprout::supervisor] SPROUT: Module[55] = '/assets/cursors/future/color-picker.svg'
[27053421743] [INFO] [sprout::supervisor] SPROUT: Module[56] = '/assets/cursors/future/context-menu.svg'
[27068287554] [INFO] [sprout::supervisor] SPROUT: Module[57] = '/assets/cursors/future/copy.svg'
[27082805430] [INFO] [sprout::supervisor] SPROUT: Module[58] = '/assets/cursors/future/crosshair.svg'
[27097760257] [INFO] [sprout::supervisor] SPROUT: Module[59] = '/assets/cursors/future/default.svg'
[27112231615] [INFO] [sprout::supervisor] SPROUT: Module[60] = '/assets/cursors/future/dnd-move.svg'
[27127240719] [INFO] [sprout::supervisor] SPROUT: Module[61] = '/assets/cursors/future/dnd-no-drop.svg'
[27141755292] [INFO] [sprout::supervisor] SPROUT: Module[62] = '/assets/cursors/future/down-arrow.svg'
[27157361043] [INFO] [sprout::supervisor] SPROUT: Module[63] = '/assets/cursors/future/draft.svg'
[27170345078] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[27198549286] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[27434963025] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[27443470607] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[27452245823] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[27460667160] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[27487415731] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/fontd'
[27495316253] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/blossom'
[27503550415] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[27511938619] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/photosynthesis'
[27520197915] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/drawlist_demo'
[27528381479] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[27538369842] [INFO] [kernel::task::loader] Loading module: /boot/clock
[27544610179] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[27554244846] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[27570868053] [INFO] [kernel::task::loader] Segment: vaddr=20b000 exec=false
[27580877691] [INFO] [kernel::task::loader] Segment: vaddr=20f000 exec=false
[27597256981] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[27606738413] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[27614197216] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[27620673806] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[27629239406] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[27645961443] [INFO] [kernel::task::loader] Segment: vaddr=20c000 exec=false
[27655294160] [INFO] [kernel::task::loader] Segment: vaddr=20e000 exec=false
[27670595402] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[27678012650] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[27685654170] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[27692276151] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[27701576788] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[27728363062] [INFO] [kernel::task::loader] Segment: vaddr=218000 exec=false
[27741760153] [INFO] [kernel::task::loader] Segment: vaddr=220000 exec=false
[27756731277] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[27764076612] [INFO] [sprout::supervisor] SPROUT: Seeding initial asset requests...
[27810049269] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451b0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[27823279293] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368153616 RFLAGS_BEFORE=130 CR3_BEFORE=50778112 fs_base=0 gs_base=18446744071563860640
[27843767702] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00041a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[27857016486] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368171792 RFLAGS_BEFORE=130 CR3_BEFORE=50929664 fs_base=0 gs_base=18446744071563860640
[27876247855] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ab8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[27889353978] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368189232 RFLAGS_BEFORE=134 CR3_BEFORE=51077120 fs_base=0 gs_base=18446744071563860640
[27904197396] [INFO] [ingestd] INGESTD: Starting unified content provider service...
[27911974351] [INFO] [ingestd] INGESTD: Initializing Limine module content source...
[27979028277] [INFO] [ingestd] INGESTD: Created Limine ContentSource node
[27986393000] [INFO] [ingestd] INGESTD: Performing initial boot module scan...
[28070616878] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/fontd'
[28078503940] [INFO] [kernel::task::loader] Loading module: /boot/fontd
[28084681024] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28093285967] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[28128997863] [INFO] [kernel::task::loader] Segment: vaddr=225000 exec=false
[28140621555] [INFO] [kernel::task::loader] Segment: vaddr=22c000 exec=false
[28155156965] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[28162032276] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/blossom'
[28170053203] [INFO] [kernel::task::loader] Loading module: /boot/blossom
[28176279601] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28184821469] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[28207422141] [INFO] [kernel::task::loader] Segment: vaddr=214000 exec=false
[28217204704] [INFO] [kernel::task::loader] Segment: vaddr=217000 exec=false
[28232005888] [INFO] [sprout::supervisor] SPROUT: App launched (PID=8)
[28239042815] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[28245996110] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[28252109623] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28260772203] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[28273207997] [INFO] [kernel::task::loader] Segment: vaddr=207000 exec=false
[28280852376] [INFO] [kernel::task::loader] Segment: vaddr=209000 exec=false
[28295005466] [INFO] [sprout::supervisor] SPROUT: App launched (PID=9)
[28302187474] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/photosynthesis'
[28310339113] [INFO] [kernel::task::loader] Loading module: /boot/photosynthesis
[28317002942] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28325795167] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[28363256118] [INFO] [kernel::task::loader] Segment: vaddr=226000 exec=false
[28376850347] [INFO] [kernel::task::loader] Segment: vaddr=22e000 exec=false
[28392443013] [INFO] [sprout::supervisor] SPROUT: App launched (PID=10)
[28400149236] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/drawlist_demo'
[28408321241] [INFO] [kernel::task::loader] Loading module: /boot/drawlist_demo
[28415127289] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28424585077] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[28441769191] [INFO] [kernel::task::loader] Segment: vaddr=20b000 exec=false
[28451845433] [INFO] [kernel::task::loader] Segment: vaddr=20e000 exec=false
[28528184843] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0049010
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28541415107] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368243568 RFLAGS_BEFORE=130 CR3_BEFORE=51302400 fs_base=0 gs_base=18446744071563860640
[28571246995] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004a338
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28584426617] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368259952 RFLAGS_BEFORE=130 CR3_BEFORE=51572736 fs_base=0 gs_base=18446744071563860640
[28600136807] [INFO] [blossom] BLOSSOM: Starting SVG Cache Service
[28608115944] [INFO] [blossom] BLOSSOM: Init UI pipeline...
[28623965140] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004cf78
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28637387832] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368276336 RFLAGS_BEFORE=130 CR3_BEFORE=51757056 fs_base=0 gs_base=18446744071563860640
[28652907129] [INFO] [cambium] cambium starting (v5.2: drain-to-eagain + smart-resync)...
[28669654077] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004d710
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28683265690] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368292720 RFLAGS_BEFORE=130 CR3_BEFORE=51884032 fs_base=0 gs_base=18446744071563860640
[28699111478] [INFO] [photosynthesis] Photosynthesis starting...
[28715064238] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004e938
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28728847541] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368319856 RFLAGS_BEFORE=130 CR3_BEFORE=52162560 fs_base=0 gs_base=18446744071563860640
[28745516386] [INFO] [drawlist_demo] DrawList demo starting (with new commands)...
[28764442740] [INFO] [sprout::supervisor] SPROUT: App launched (PID=11)
[28772293814] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[28793739122] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[28803380779] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[28813793013] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28825108071] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=561 subj_lo=0
[28868276334] [INFO] [ingestd] INGESTD: Published new asset '/boot/sprout' (hash=61811fc99aafd0a2)
[28947300940] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[28955542522] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[28964635843] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28974575394] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=588 subj_lo=0
[29000224468] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f8fb0
[29009172607] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[29018544872] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[29028330943] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=589 subj_lo=0
[29058990782] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[29068067408] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[29077437369] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29088191102] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=594 subj_lo=0
[29108070566] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[29115924155] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1920x1080 stride=7680)
[29820951328] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f8fb0
[29829560881] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[29838814380] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[29848684057] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=595 subj_lo=0
[29885286595] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[29893856196] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[29903059821] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29912912020] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=601 subj_lo=0
[29957619295] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[29966250468] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[29975302245] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29985080641] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=606 subj_lo=0
[30022897005] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fbfb0
[30031498275] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[30040533127] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[30050342135] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=610 subj_lo=0
[30086926024] [INFO] [ingestd] INGESTD: Created File node '/boot/sprout' (90640 bytes, hash=61811fc99aafd0a2)
[30097303585] [INFO] [ingestd] INGESTD: Published asset '/boot/sprout' (raw, 90640 bytes, hash=61811fc99aafd0a2)
[30155166180] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[30162411574] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[30172040503] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[30184185351] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[30192456316] [INFO] [kernel::task::loader] Segment: vaddr=206000 exec=false
[30209180839] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=12)
[30237395488] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0049010
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x70006
[30251144155] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072368407712 RFLAGS_BEFORE=134 CR3_BEFORE=60870656 fs_base=0 gs_base=18446744071563860640
[30267918263] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=6, drv_resp_w=7)
[30299329791] [INFO] [blossom] BLOSSOM: UI pipeline ready
[30353735262] [INFO] [blossom] BLOSSOM: Service node created, req=9, resp=12
[30362793928] [INFO] [blossom] BLOSSOM: Service ready
[30389098748] [INFO] [kernel::syscall::handlers::device] DEVICE: task 12 claimed device 631 (handle 0)
[30418138775] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x7e9000 -> virt=0x1001f000
[30452319993] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[30461070841] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[30468117916] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[30478245967] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[30490693539] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[30499592283] [INFO] [kernel::task::loader] Segment: vaddr=207000 exec=false
[30517572520] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=13)
[30525667471] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[30535194431] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=13, r=14)
[30545059764] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=15, r=16)
[30554318977] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=17, r=18)
[30563054446] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[30570160968] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[30579862198] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[30591801397] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[30600080457] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[30617968939] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=14)
[30625830361] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[30633220038] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[30643243331] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[30655277426] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[30663518280] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[30680100970] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=15)
[30689855709] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=19, r=20)
[30698671117] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[30705610037] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[30715302122] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[30727246692] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[30735984017] [INFO] [kernel::task::loader] Segment: vaddr=206000 exec=false
[30752740956] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=16)
[30796295192] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0051170
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x28b
[30810134531] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369438800 RFLAGS_BEFORE=134 CR3_BEFORE=61005824 fs_base=0 gs_base=18446744071563860640
[30827137496] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[30837057694] [INFO] [rtc_cmos] Starting... arg=28b
[30844023339] [INFO] [rtc_cmos] Serving device ID: ThingId([139, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[30855839848] [INFO] [rtc_cmos] RTC: 2026-01-29 22:21:25 = 1769725285 unix_secs
[30864612971] [INFO] [kernel::time] System clock anchored: unix_secs=1769725285, mono_ns=15432028272, offset=1769725269567971728ns
[30876442382] [INFO] [rtc_cmos] System clock anchored
[30903326305] [INFO] [rtc_cmos] RTC: Set sys.TimeState = 1 (Anchored)
[30935070930] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[30951599888] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0056068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xd
[30964744838] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369471568 RFLAGS_BEFORE=134 CR3_BEFORE=61124608 fs_base=0 gs_base=18446744071563860640
[30981238120] [INFO] [ps2_kbd] ps2_kbd: online (handle=13)
[31002951843] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb005d530
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xf
[31016248174] [INFO] [task.user_enter] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369487952 RFLAGS_BEFORE=134 CR3_BEFORE=61235200 fs_base=0 gs_base=18446744071563860640
[31032736299] [INFO] [ps2_mouse] ps2_mouse: online (handle=15)
[31040604898] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[31055522744] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb005e178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe001000110013
[31069535531] [INFO] [task.user_enter] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369512848 RFLAGS_BEFORE=134 CR3_BEFORE=61345792 fs_base=0 gs_base=18446744071563860640
[31087906257] [INFO] [bristle] bristle: online (kbd=14, mouse=16, evt=17, evt_echo=19)
[31119120701] [INFO] [ps2_kbd] ps2_kbd: created driver node 985
[31127279131] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[31138674232] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[31157116170] [INFO] [bristle] bristle: registered in graph as svc.Input (id=993)
[31203839974] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[31211923745] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[31219625823] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
[31235201527] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=968 backend=BootFB
[31243620003] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[31250093001] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[31259508715] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[31422711918] [INFO] [kernel::task::loader] Segment: vaddr=2c5000 exec=false
[31445510071] [INFO] [kernel::task::loader] Segment: vaddr=2d9000 exec=false
[31462956075] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=17)
[31470879311] [INFO] [kernel::task::loader] Loading module: /boot/echo
[31477827265] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[31489032623] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[31501347478] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[31511140039] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[31558220532] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[31573874457] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004f220
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x3c8
[31589413390] [INFO] [task.user_enter] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369561088 RFLAGS_BEFORE=134 CR3_BEFORE=61468672 fs_base=0 gs_base=18446744071563860640
[31607842415] [INFO] [bloom::logging] bloom: logging initialized
[31648573240] [INFO] [bloom] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[31669018342] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0056068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x14
[31683706088] [INFO] [task.user_enter] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072369577664 RFLAGS_BEFORE=134 CR3_BEFORE=62447616 fs_base=0 gs_base=18446744071563860640
[31703371523] [INFO] [echo] echo: online (handle=20)
[31710526495] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[31729833267] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=18)
[31737911415] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[31747138220] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[31795647111] [INFO] [ingestd] INGESTD: Published new asset '/boot/bristle' (hash=d55e2ab85adcac5d)
[31891486841] [INFO] [bloom::compositor] bloom: compositor bytespace 895 (1920x1080 stride=7680 format=2)
[31937419332] [INFO] [ingestd] INGESTD: Created File node '/boot/bristle' (29104 bytes, hash=d55e2ab85adcac5d)
[31948046692] [INFO] [ingestd] INGESTD: Published asset '/boot/bristle' (raw, 29104 bytes, hash=d55e2ab85adcac5d)
[31966034001] [INFO] [bloom::compositor] bloom: display backend: BootFB
[31989733588] [INFO] [bloom::compositor] bloom: mapped size=8294400 (source=bytespace_info)
[32058717094] [INFO] [stem::ui] UiBuilder: created root 1032
[32080156234] [INFO] [bloom] bloom: spawned asset watcher (tid=19)
[32087338920] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
T:3C90 [32116437693] [INFO] [bloom::painter_resources] [bloom] asset_watcher_entry: spawning sub-loaders
[32178011738] [INFO] [ps2_mouse] ps2_mouse: init done
[32184919591] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[32193750173] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[32201350420] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
T:41D0 T:4040 T:2A00 [32277088656] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f64e0
[32285901071] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[32294983945] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[32304806898] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=739 subj_lo=0
[32351369221] [INFO] [photosynthesis] Found UI Root: 1032
[32369273377] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f64e0
[32377802446] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[32386915150] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[32396670930] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=644 pred=0 subj_lo=0
[32454825712] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7f64e0
[32463608996] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[32473389255] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[32483639311] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=745 subj_lo=0
[32523208253] [INFO] [bloom] [bloom] Starting UI loop immediately (not waiting for fonts)
[32536606464] [INFO] [bloom::painter_resources] [bloom] wallpaper loader: loading leather.bmp
[32548186473] [INFO] [bloom::asset] [asset_bank] worker spawned tid=23 (priority=2)
[32555506013] [INFO] [bloom::painter_resources] [bloom] cursor loader: loading default cursor
[32560135797] [INFO] [bloom::painter_resources] [bloom] icon loader started
T:4C30 [32594352322] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[32597139020] [INFO] [bloom::asset] [asset_bank] load_wallpaper_immediate: leather.bmp
[32613745040] [INFO] [ingestd] INGESTD: Published new asset '/boot/rtc_cmos' (hash=32366e1092006840)
[32792110129] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef60
[32794249778] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[32796304535] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[32798411943] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x0 kind=0 pred=0 subj_lo=0
[32906264578] [INFO] [ingestd] INGESTD: Created File node '/boot/rtc_cmos' (33368 bytes, hash=32366e1092006840)
[32925553227] [INFO] [ingestd] INGESTD: Published asset '/boot/rtc_cmos' (raw, 33368 bytes, hash=32366e1092006840)
[33230427549] [INFO] [bloom::present] bloom: driver REGISTER (kind=1 caps=0x3)
[33233744946] [INFO] [bloom::present] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=8
[33290625629] [INFO] [display_bootfb] display_bootfb: bound bytespace 895
[33313601872] [INFO] [ingestd] INGESTD: Published new asset '/boot/clock' (hash=771f0ce19bac6901)
[33424271905] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7feba0
[33425910082] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[33427975759] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[33429976218] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=784 pred=0 subj_lo=0
[33563408347] [INFO] [ingestd] INGESTD: Created File node '/boot/clock' (66064 bytes, hash=771f0ce19bac6901)
[33565897180] [INFO] [ingestd] INGESTD: Published asset '/boot/clock' (raw, 66064 bytes, hash=771f0ce19bac6901)
[33657906897] [INFO] [bloom::asset] [asset_bank] mapping bytespace 188 (4718646 bytes) for 'leather.bmp'
[33693310278] [INFO] [bloom::asset] [asset_bank] decoding BMP for 'leather.bmp'...
[34440029675] [INFO] [bloom::asset] [asset_bank] BMP decoded: 1536x1024 for 'leather.bmp'
[35270824150] [INFO] [bloom::asset] [asset_bank] publish_wallpaper (pending): 1536x1024
[35273786068] [INFO] [bloom::asset] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[35465913412] [INFO] [ingestd] INGESTD: Published new asset '/boot/font_explorer' (hash=1cdbe11025543439)
[35635949958] [INFO] [ingestd] INGESTD: Created File node '/boot/font_explorer' (61968 bytes, hash=1cdbe11025543439)
[35638252181] [INFO] [ingestd] INGESTD: Published asset '/boot/font_explorer' (raw, 61968 bytes, hash=1cdbe11025543439)
[35778650011] [INFO] [bloom::present] bloom: driver BIND ACK
[35803824437] [INFO] [bloom::reclaimer] [reclaimer] +6291456 bytes (total: 6291456)
[35808978748] [INFO] [bloom::asset] [asset_bank] promoting wallpaper 'leather.bmp' to gen=1 (6291456b)
[35915204187] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x10040870
[35918434518] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[35920199474] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35921867169] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=797 pred=0 subj_lo=0
[35957919224] [INFO] [ingestd] INGESTD: Published new asset '/boot/ps2_kbd' (hash=4da765f0b8256772)
[35972222756] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x10040870
[35978566579] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[35984619923] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35986733842] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=784 pred=0 subj_lo=0
[36030559280] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x10040870
[36037040300] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[36043704861] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36045327625] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=798 pred=0 subj_lo=0
[36090379371] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x10040870
[36097276136] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[36099472847] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36101087915] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=799 pred=0 subj_lo=0
[36218969601] [INFO] [ingestd] INGESTD: Created File node '/boot/ps2_kbd' (25008 bytes, hash=4da765f0b8256772)
[36226699318] [INFO] [ingestd] INGESTD: Published asset '/boot/ps2_kbd' (raw, 25008 bytes, hash=4da765f0b8256772)
[36505661450] [INFO] [bloom::asset] [asset_bank] mapping bytespace 251 (3051 bytes)
[36536485624] [INFO] [bloom::asset] [asset_bank] mapped to 0x11c95000
[36541599694] [INFO] [bloom::asset] [asset_bank] detected SVG format
[36546567645] [INFO] [bloom::asset] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[36640041328] [INFO] [bloom::raster] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[36658582393] [INFO] [bloom::raster] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[36665037706] [INFO] [bloom::raster] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[36685188725] [INFO] [ingestd] INGESTD: Published new asset '/boot/echo' (hash=a11f50e00fa91378)
[36705859849] [INFO] [bloom::raster] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[36708987471] [INFO] [bloom::raster] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[36723118666] [INFO] [bloom::asset] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[36788020477] [INFO] [bloom::asset] [asset_bank] publish_cursor (pending)
[36790228143] [INFO] [bloom::asset] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[36792406970] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 194 (23272 bytes)
[36816251298] [INFO] [bloom::asset] [asset_bank] mapped at 0x11c96000
[36818187474] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[36849388196] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[36977072577] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[36979770860] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 197 (309408 bytes)
[36993904512] [INFO] [ingestd] INGESTD: Created File node '/boot/echo' (25008 bytes, hash=a11f50e00fa91378)
[36995799313] [INFO] [ingestd] INGESTD: Published asset '/boot/echo' (raw, 25008 bytes, hash=a11f50e00fa91378)
[37008198188] [INFO] [bloom::asset] [asset_bank] mapped at 0x11c9c000
[37011466446] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[37343976679] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[37536488503] [INFO] [bloom] [CONTRACT] [bloom] First frame rendered
[37563519338] [INFO] [bloom::reclaimer] [reclaimer] +16384 bytes (total: 6307840)
[37565714865] [INFO] [bloom::asset] [asset_bank] promoting cursor to gen=2 (16384b)
[37568355043] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 6410240)
[37570380706] [INFO] [bloom::asset] [asset_bank] promoting font 'DSEG7Classic-Regular.ttf' to gen=2 (102400b) in slot 0
[37616446188] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[37618643630] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 200 (569208 bytes)
[37641861174] [INFO] [bloom::asset] [asset_bank] mapped at 0x11dc3000
[37644252333] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[39227521820] [INFO] [ingestd] INGESTD: Published new asset '/boot/bloom' (hash=7904093ad49c5bd5)
[41406741403] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[41448201235] [INFO] [ingestd] INGESTD: Created File node '/boot/bloom' (895496 bytes, hash=7904093ad49c5bd5)
[41450551949] [INFO] [ingestd] INGESTD: Published asset '/boot/bloom' (raw, 895496 bytes, hash=7904093ad49c5bd5)
[41633622901] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[41636031496] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 203 (258156 bytes)
[41662507206] [INFO] [bloom::asset] [asset_bank] mapped at 0x11e55000
[41664742391] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[44374802080] [INFO] [ingestd] INGESTD: Published new asset '/boot/ps2_mouse' (hash=8d3523c823935b52)
[45414560532] [INFO] [drawlist_demo] Frame 4: color cycle, clip=true, scale=0.8
[47451045286] [INFO] [ingestd] INGESTD: Created File node '/boot/ps2_mouse' (25008 bytes, hash=8d3523c823935b52)
[47453207249] [INFO] [ingestd] INGESTD: Published asset '/boot/ps2_mouse' (raw, 25008 bytes, hash=8d3523c823935b52)
[47890683014] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[47932265833] [INFO] [bloom::cursor_rasterizer] [cursor_rasterizer] rasterizing cursor snapshot gen=2
[47991811009] [INFO] [bloom::present] display: full-frame damage, using full-frame present
[48034211809] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 6512640)
[48036172705] [INFO] [bloom::asset] [asset_bank] promoting font 'Hack-Regular.ttf' to gen=3 (102400b) in slot 1
[48039162925] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 6615040)
[48042374148] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=3 (102400b) in slot 2
[48086623702] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:33' tick=23952411610
[48139475628] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[48141861935] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 206 (656852 bytes)
[48164282426] [INFO] [bloom::asset] [asset_bank] mapped at 0x11e9d000
[48166422431] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[49160061732] [INFO] [ingestd] INGESTD: Published new asset '/boot/root_batch_bench' (hash=323a9e6726f38ab3)
[50787552617] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:34' tick=24587802193
[51850342225] [INFO] [ingestd] INGESTD: Created File node '/boot/root_batch_bench' (29200 bytes, hash=323a9e6726f38ab3)
[51854085271] [INFO] [ingestd] INGESTD: Published asset '/boot/root_batch_bench' (raw, 29200 bytes, hash=323a9e6726f38ab3)
[52679397893] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:35' tick=25548058168
[54436430112] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:36' tick=26478561306
[55033146781] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[55123125285] [INFO] [ingestd] INGESTD: Published new asset '/boot/root_watch_tester' (hash=5b29257b2e16f32e)
[55174816408] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:37' tick=27527168152
[55264029239] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol2-Regular.ttf' in slot 4
[55268144414] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 209 (616196 bytes)
[55283590625] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 6717440)
[55286114262] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol-Regular.ttf' to gen=4 (102400b) in slot 3
[55299570675] [INFO] [bloom::asset] [asset_bank] mapped at 0x11f49000
[55301761716] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSerif-Regular.ttf'...
[55439256083] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 6819840)
[55441701362] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol2-Regular.ttf' to gen=4 (102400b) in slot 4
[55929010752] [INFO] [ingestd] INGESTD: Created File node '/boot/root_watch_tester' (41488 bytes, hash=5b29257b2e16f32e)
[55931775830] [INFO] [ingestd] INGESTD: Published asset '/boot/root_watch_tester' (raw, 41488 bytes, hash=5b29257b2e16f32e)
[57168068498] [INFO] [drawlist_demo] Frame 8: color cycle, clip=true, scale=1
[58694076885] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:38' tick=28590247865
[60510195564] [INFO] [ingestd] INGESTD: Published new asset '/boot/display_bootfb' (hash=e581a9426f17f21e)
[60523867959] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:39' tick=29475501012
[62452813446] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:40' tick=30511665076
[62948293617] [INFO] [ingestd] INGESTD: Created File node '/boot/display_bootfb' (29272 bytes, hash=e581a9426f17f21e)
[62950972839] [INFO] [ingestd] INGESTD: Published asset '/boot/display_bootfb' (raw, 29272 bytes, hash=e581a9426f17f21e)
[64589419157] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:41' tick=31486713687
[68262306450] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:42' tick=32638097049
[69420827714] [INFO] [ingestd] INGESTD: Published new asset '/boot/fontd' (hash=a6d7f7d6cc48065e)
[71292125781] [INFO] [drawlist_demo] Frame 12: color cycle, clip=true, scale=0.8
[71974097120] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:44' tick=34598065221
[74023950052] [INFO] [ingestd] INGESTD: Created File node '/boot/fontd' (184848 bytes, hash=a6d7f7d6cc48065e)
[74031837510] [INFO] [ingestd] INGESTD: Published asset '/boot/fontd' (raw, 184848 bytes, hash=a6d7f7d6cc48065e)
[76097604084] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:46' tick=36673806903
[79630321999] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:48' tick=38513720888
[81951157043] [INFO] [ingestd] INGESTD: Published new asset '/boot/blossom' (hash=e5637a1100e504c3)
[86029682135] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:50' tick=40860843356
[87897692819] [INFO] [drawlist_demo] Frame 16: color cycle, clip=true, scale=1
[87903962439] [INFO] [ingestd] INGESTD: Created File node '/boot/blossom' (98832 bytes, hash=e5637a1100e504c3)
[87905603292] [INFO] [ingestd] INGESTD: Published asset '/boot/blossom' (raw, 98832 bytes, hash=e5637a1100e504c3)
[88837926575] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:53' tick=43717721070
[91365767106] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:54' tick=44542476040
[91534207078] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[91705818250] [INFO] [ingestd] INGESTD: Published new asset '/boot/ingestd' (hash=e0533d4f12c4b151)
[91829286487] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSerif-Regular.ttf' in slot 5
[91877781870] [INFO] [ingestd] INGESTD: Created File node '/boot/ingestd' (135704 bytes, hash=e0533d4f12c4b151)
[91880100736] [INFO] [ingestd] INGESTD: Published asset '/boot/ingestd' (raw, 135704 bytes, hash=e0533d4f12c4b151)
[92105331166] [INFO] [ingestd] INGESTD: Published new asset '/boot/cambium' (hash=2d10baecde2fddb5)
[92464931896] [INFO] [ingestd] INGESTD: Created File node '/boot/cambium' (41488 bytes, hash=2d10baecde2fddb5)
[92467277800] [INFO] [ingestd] INGESTD: Published asset '/boot/cambium' (raw, 41488 bytes, hash=2d10baecde2fddb5)
[93064069522] [INFO] [ingestd] INGESTD: Published new asset '/boot/scheduler_fairness' (hash=64b72b7c8fe97783)
[93145640255] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 6922240)
[93148046649] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSerif-Regular.ttf' to gen=5 (102400b) in slot 5
[93271284494] [INFO] [ingestd] INGESTD: Created File node '/boot/scheduler_fairness' (29168 bytes, hash=64b72b7c8fe97783)
[93273612001] [INFO] [ingestd] INGESTD: Published asset '/boot/scheduler_fairness' (raw, 29168 bytes, hash=64b72b7c8fe97783)
[93553887632] [INFO] [ingestd] INGESTD: Published new asset '/boot/drawlist_demo' (hash=86785d91a2fcc035)
[93609126091] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:56' tick=46752471396
[93710046927] [INFO] [ingestd] INGESTD: Created File node '/boot/drawlist_demo' (61968 bytes, hash=86785d91a2fcc035)
[93712368600] [INFO] [ingestd] INGESTD: Published asset '/boot/drawlist_demo' (raw, 61968 bytes, hash=86785d91a2fcc035)
[94037186047] [INFO] [ingestd] INGESTD: Published new asset '/boot/tick_printer' (hash=899e49344cab13b9)
[94221022226] [INFO] [ingestd] INGESTD: Created File node '/boot/tick_printer' (25008 bytes, hash=899e49344cab13b9)
[94223364332] [INFO] [ingestd] INGESTD: Published asset '/boot/tick_printer' (raw, 25008 bytes, hash=899e49344cab13b9)
[94526315297] [INFO] [ingestd] INGESTD: Published new asset '/boot/photosynthesis' (hash=419b269c1691744c)
[95267871923] [INFO] [ingestd] INGESTD: Created File node '/boot/photosynthesis' (193040 bytes, hash=419b269c1691744c)
[95270142418] [INFO] [ingestd] INGESTD: Published asset '/boot/photosynthesis' (raw, 193040 bytes, hash=419b269c1691744c)
[95570154072] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:57' tick=47729443289
[95581193314] [INFO] [ingestd] INGESTD: Published new asset '/boot/ata_disk' (hash=4a89935b43d87275)
[95748020511] [INFO] [ingestd] INGESTD: Created File node '/boot/ata_disk' (37472 bytes, hash=4a89935b43d87275)
[95750383221] [INFO] [ingestd] INGESTD: Published asset '/boot/ata_disk' (raw, 37472 bytes, hash=4a89935b43d87275)
[96077203178] [INFO] [ingestd] INGESTD: Published new asset '/boot/iso_reader' (hash=fefec3dea03cb985)
[96259851018] [INFO] [ingestd] INGESTD: Created File node '/boot/iso_reader' (70248 bytes, hash=fefec3dea03cb985)
[96264105792] [INFO] [ingestd] INGESTD: Published asset '/boot/iso_reader' (raw, 70248 bytes, hash=fefec3dea03cb985)
[96891474605] [INFO] [ingestd] INGESTD: Published new asset '/assets/cursors/plain/Alternate.cur' (hash=b508a251ddc6775c)
[97090126361] [INFO] [ingestd] INGESTD: Created File node '/assets/cursors/plain/Alternate.cur' (4286 bytes, hash=b508a251ddc6775c)
[97098149454] [INFO] [ingestd] INGESTD: Published asset '/assets/cursors/plain/Alternate.cur' (raw, 4286 bytes, hash=b508a251ddc6775c)
[97374909620] [INFO] [ingestd] INGESTD: Published new asset '/assets/cursors/plain/Busy.cur' (hash=10640e8f8d360989)
[97454140912] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:58' tick=48659510207
[97562680274] [INFO] [ingestd] INGESTD: Created File node '/assets/cursors/plain/Busy.cur' (4286 bytes, hash=10640e8f8d360989)
[97566674335] [INFO] [ingestd] INGESTD: Published asset '/assets/cursors/plain/Busy.cur' (raw, 4286 bytes, hash=10640e8f8d360989)
[97861454798] [INFO] [ingestd] INGESTD: Published new asset '/assets/cursors/plain/Diagonal1.ani' (hash=0f05dcdb415c5e77)
[98058032686] [INFO] [drawlist_demo] Frame 20: color cycle, clip=true, scale=0.8
[98088178888] [INFO] [ingestd] INGESTD: Created File node '/assets/cursors/plain/Diagonal1.ani' (13030 bytes, hash=0f05dcdb415c5e77)
[98091622786] [INFO] [ingestd] INGESTD: Published asset '/assets/cursors/plain/Diagonal1.ani' (raw, 13030 bytes, hash=0f05dcdb415c5e77)
[98408339406] [INFO] [ingestd] INGESTD: Published new asset '/assets/cursors/plain/Diagonal2.ani' (hash=e7bb9bd63c66a69d)
[98973392404] [INFO] [ingestd] INGESTD: Created File node '/assets/cursors/plain/Diagonal2.ani' (13030 bytes, hash=e7bb9bd63c66a69d)
[98982596349] [INFO] [ingestd] INGESTD: Published asset '/assets/cursors/plain/Diagonal2.ani' (raw, 13030 bytes, hash=e7bb9bd63c66a69d)
[99293724789] [INFO] [ingestd] INGESTD: Published new asset '/assets/cursors/plain/Handwriting.cur' (hash=0838f13251e9673a)
[99491132440] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:21:59' tick=49686835409
[99528587786] [INFO] [ingestd] INGESTD: Created File node '/assets/cursors/plain/Handwriting.cur' (4286 bytes, hash=0838f13251e9673a)
[99532971197] [INFO] [ingestd] INGESTD: Published asset '/assets/cursors/plain/Handwriting.cur' (raw, 4286 bytes, hash=0838f13251e9673a)
[99836119572] [INFO] [ingestd] INGESTD: Published new asset '/assets/cursors/plain/Help.cur' (hash=7a616f96410a4e35)
[100016357594] [INFO] [ingestd] INGESTD: Created File node '/assets/cursors/plain/Help.cur' (4286 bytes, hash=7a616f96410a4e35)
[100029973067] [INFO] [ingestd] INGESTD: Published asset '/assets/cursors/plain/Help.cur' (raw, 4286 bytes, hash=7a616f96410a4e35)
[100396256741] [INFO] [ingestd] INGESTD: Published new asset '/assets/cursors/plain/Horizontal.ani' (hash=b4474ea3ce895f0a)
[100558000252] [INFO] [ingestd] INGESTD: Created File node '/assets/cursors/plain/Horizontal.ani' (13030 bytes, hash=b4474ea3ce895f0a)
[100563287876] [INFO] [ingestd] INGESTD: Published asset '/assets/cursors/plain/Horizontal.ani' (raw, 13030 bytes, hash=b4474ea3ce895f0a)
[101357412039] [INFO] [ingestd] INGESTD: Published new asset '/assets/cursors/plain/Link.ani' (hash=807c54a2f8605f6b)
[101541367766] [INFO] [ingestd] INGESTD: Created File node '/assets/cursors/plain/Link.ani' (38778 bytes, hash=807c54a2f8605f6b)
[101544904807] [INFO] [ingestd] INGESTD: Published asset '/assets/cursors/plain/Link.ani' (raw, 38778 bytes, hash=807c54a2f8605f6b)
[101603475136] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:22:00' tick=50723949099
[101921518769] [INFO] [ingestd] INGESTD: Published new asset '/assets/cursors/plain/Move.cur' (hash=484a53716866470c)
[102137406514] [INFO] [ingestd] INGESTD: Created File node '/assets/cursors/plain/Move.cur' (4286 bytes, hash=484a53716866470c)
[102150717034] [INFO] [ingestd] INGESTD: Published asset '/assets/cursors/plain/Move.cur' (raw, 4286 bytes, hash=484a53716866470c)
[102480990984] [INFO] [ingestd] INGESTD: Published new asset '/assets/cursors/plain/Normal.cur' (hash=285c33152a794fe7)
[102660377616] [INFO] [ingestd] INGESTD: Created File node '/assets/cursors/plain/Normal.cur' (4286 bytes, hash=285c33152a794fe7)
[102670561984] [INFO] [ingestd] INGESTD: Published asset '/assets/cursors/plain/Normal.cur' (raw, 4286 bytes, hash=285c33152a794fe7)
[103005303092] [INFO] [ingestd] INGESTD: Published new asset '/assets/cursors/plain/Precision.cur' (hash=34a694aac06e30e9)
[103272037302] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:22:01' tick=51527408002
[103449665051] [INFO] [ingestd] INGESTD: Created File node '/assets/cursors/plain/Precision.cur' (4286 bytes, hash=34a694aac06e30e9)
[103458502718] [INFO] [ingestd] INGESTD: Published asset '/assets/cursors/plain/Precision.cur' (raw, 4286 bytes, hash=34a694aac06e30e9)
[106028004514] [INFO] [ingestd] INGESTD: Published new asset '/assets/cursors/plain/Text.cur' (hash=b4fcd04b16fc7e07)
[106099138989] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:22:02' tick=52936434259
[106276770874] [INFO] [ingestd] INGESTD: Created File node '/assets/cursors/plain/Text.cur' (4286 bytes, hash=b4fcd04b16fc7e07)
[106281678774] [INFO] [ingestd] INGESTD: Published asset '/assets/cursors/plain/Text.cur' (raw, 4286 bytes, hash=b4fcd04b16fc7e07)
[106587652256] [INFO] [ingestd] INGESTD: Published new asset '/assets/cursors/plain/Unavailabe.cur' (hash=b4e6654d3e843c02)
[106828026644] [INFO] [ingestd] INGESTD: Created File node '/assets/cursors/plain/Unavailabe.cur' (4286 bytes, hash=b4e6654d3e843c02)
[106832547815] [INFO] [ingestd] INGESTD: Published asset '/assets/cursors/plain/Unavailabe.cur' (raw, 4286 bytes, hash=b4e6654d3e843c02)
[107167317739] [INFO] [clock] CLOCK PUBLISH: thing=806 now_text='22:22:03' tick=53492397857
[107205702619] [INFO] [ingestd] INGESTD: Published new asset '/assets/cursors/plain/Vertical.ani' (hash=e54ed565542834a7)
[107397651550] [INFO] [ingestd] INGESTD: Created File node '/assets/cursors/plain/Vertical.ani' (13028 bytes, hash=e54ed565542834a7)
[107400783157] [INFO] [ingestd] INGESTD: Published asset '/assets/cursors/plain/Vertical.ani' (raw, 13028 bytes, hash=e54ed565542834a7)
[107668059494] [INFO] [drawlist_demo] Frame 24: color cycle, clip=true, scale=1
[107777132563] [INFO] [ingestd] INGESTD: Published new asset '/assets/cursors/plain/Working.ani' (hash=0bde7caac55ecf17)
[108004224687] [INFO] [ingestd] INGESTD: Created File node '/assets/cursors/plain/Working.ani' (34470 bytes, hash=0bde7caac55ecf17)
[108007286445] [INFO] [ingestd] INGESTD: Published asset '/assets/cursors/plain/Working.ani' (raw, 34470 bytes, hash=0bde7caac55ecf17)
[108819839474] [INFO] [ingestd] INGESTD: Published new asset '/assets/wallpapers/clouds.bmp' (hash=6302b2cf74732177)
[109093909452] [INFO] [ingestd] INGESTD: Created File node '/assets/wallpapers/clouds.bmp' (786486 bytes, hash=6302b2cf74732177)
[109096879429] [INFO] [ingestd] INGESTD: Published asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)

```
</details>
