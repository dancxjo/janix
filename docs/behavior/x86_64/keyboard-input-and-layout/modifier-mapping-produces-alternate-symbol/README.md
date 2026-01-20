# ✅ Scenario: Modifier mapping produces alternate symbol

> Last run: 2026-01-19 20:26:42

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the clock window is ticking | ⏭️ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[18871982547] [INFO] [kernel] thing-os kernel starting...
[18894901971] [INFO] [kernel::memory] Memory map has 64 entries
[18901954962] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[18906990498] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[18907551135] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[18909684453] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[18910244199] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[18910774938] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[18911334849] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[18911864499] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[18912460875] [INFO] [kernel::memory]   [8] 0x1780000 - 0x7866b000 (Usable)
[18913089888] [INFO] [kernel::memory]   [9] 0x7866b000 - 0x786cd000 (Reserved)
[18913678806] [INFO] [kernel::memory]   [10] 0x786cd000 - 0x7884f000 (Other)
[18914272971] [INFO] [kernel::memory]   [11] 0x7884f000 - 0x78850000 (Reserved)
[18918313425] [INFO] [kernel::memory]   [12] 0x78850000 - 0x788e7000 (Other)
[18920337381] [INFO] [kernel::memory]   [13] 0x788e7000 - 0x788e8000 (Reserved)
[18920952699] [INFO] [kernel::memory]   [14] 0x788e8000 - 0x78989000 (Other)
[18921522576] [INFO] [kernel::memory]   [15] 0x78989000 - 0x7898a000 (Reserved)
[18922112286] [INFO] [kernel::memory]   [16] 0x7898a000 - 0x789ca000 (Other)
[18922679754] [INFO] [kernel::memory]   [17] 0x789ca000 - 0x789cb000 (Reserved)
[18923267352] [INFO] [kernel::memory]   [18] 0x789cb000 - 0x78a56000 (Other)
[18923835480] [INFO] [kernel::memory]   [19] 0x78a56000 - 0x78a57000 (Reserved)
[18927731328] [INFO] [kernel::memory]   [20] 0x78a57000 - 0x78aa3000 (Other)
[18930517287] [INFO] [kernel::memory]   [21] 0x78aa3000 - 0x78aa4000 (Reserved)
[18931132506] [INFO] [kernel::memory]   [22] 0x78aa4000 - 0x78aaa000 (Other)
[18931698621] [INFO] [kernel::memory]   [23] 0x78aaa000 - 0x78aab000 (Reserved)
[18932284668] [INFO] [kernel::memory]   [24] 0x78aab000 - 0x78f2c000 (Other)
[18932851905] [INFO] [kernel::memory]   [25] 0x78f2c000 - 0x78f2d000 (Reserved)
[18933436797] [INFO] [kernel::memory]   [26] 0x78f2d000 - 0x793ae000 (Other)
[18934019247] [INFO] [kernel::memory]   [27] 0x793ae000 - 0x793af000 (Reserved)
[18934605921] [INFO] [kernel::memory]   [28] 0x793af000 - 0x796b0000 (Other)
[18935171706] [INFO] [kernel::memory]   [29] 0x796b0000 - 0x796b1000 (Reserved)
[18935822499] [INFO] [kernel::memory]   [30] 0x796b1000 - 0x796ba000 (Other)
[18936386205] [INFO] [kernel::memory]   [31] 0x796ba000 - 0x796bb000 (Reserved)
[18936971955] [INFO] [kernel::memory]   [32] 0x796bb000 - 0x796bf000 (Other)
[18944103387] [INFO] [kernel::memory]   [33] 0x796bf000 - 0x796c0000 (Reserved)
[18944891196] [INFO] [kernel::memory]   [34] 0x796c0000 - 0x796c2000 (Other)
[18945453285] [INFO] [kernel::memory]   [35] 0x796c2000 - 0x796c3000 (Reserved)
[18946026957] [INFO] [kernel::memory]   [36] 0x796c3000 - 0x796c5000 (Other)
[18946581588] [INFO] [kernel::memory]   [37] 0x796c5000 - 0x796c6000 (Reserved)
[18947179482] [INFO] [kernel::memory]   [38] 0x796c6000 - 0x796c8000 (Other)
[18947735532] [INFO] [kernel::memory]   [39] 0x796c8000 - 0x796c9000 (Reserved)
[18948312306] [INFO] [kernel::memory]   [40] 0x796c9000 - 0x796cb000 (Other)
[18948868950] [INFO] [kernel::memory]   [41] 0x796cb000 - 0x796cc000 (Reserved)
[18949445559] [INFO] [kernel::memory]   [42] 0x796cc000 - 0x796ce000 (Other)
[18954549042] [INFO] [kernel::memory]   [43] 0x796ce000 - 0x796cf000 (Reserved)
[18957586296] [INFO] [kernel::memory]   [44] 0x796cf000 - 0x796d9000 (Other)
[18958217256] [INFO] [kernel::memory]   [45] 0x796d9000 - 0x796da000 (Reserved)
[18958796835] [INFO] [kernel::memory]   [46] 0x796da000 - 0x796de000 (Other)
[18959357604] [INFO] [kernel::memory]   [47] 0x796de000 - 0x796df000 (Reserved)
[18963043374] [INFO] [kernel::memory]   [48] 0x796df000 - 0x796e1000 (Other)
[18963630147] [INFO] [kernel::memory]   [49] 0x796e1000 - 0x796e2000 (Reserved)
[18964208142] [INFO] [kernel::memory]   [50] 0x796e2000 - 0x796e4000 (Other)
[18964768647] [INFO] [kernel::memory]   [51] 0x796e4000 - 0x796e5000 (Reserved)
[18965345850] [INFO] [kernel::memory]   [52] 0x796e5000 - 0x796e9000 (Other)
[18965902560] [INFO] [kernel::memory]   [53] 0x796e9000 - 0x79758000 (Other)
[18966457422] [INFO] [kernel::memory]   [54] 0x79758000 - 0x7990e000 (Other)
[18967027959] [INFO] [kernel::memory]   [55] 0x7990e000 - 0x7a16c000 (Reserved)
[18967600476] [INFO] [kernel::memory]   [56] 0x7a16c000 - 0x7bb6c000 (Usable)
[18968160882] [INFO] [kernel::memory]   [57] 0x7bb6c000 - 0x7bb8d000 (Reserved)
[18968732442] [INFO] [kernel::memory]   [58] 0x7bb8d000 - 0x7bb91000 (Other)
[18969287040] [INFO] [kernel::memory]   [59] 0x7bb91000 - 0x7bb92000 (Reserved)
[18969859557] [INFO] [kernel::memory]   [60] 0x7bb92000 - 0x7bb94000 (Other)
[18970429764] [INFO] [kernel::memory]   [61] 0x7bb94000 - 0x7bb95000 (Reserved)
[18971002017] [INFO] [kernel::memory]   [62] 0x7bb95000 - 0x7bb97000 (Other)
[18971552688] [INFO] [kernel::memory]   [63] 0x7bb97000 - 0x7bb98000 (Reserved)
[18972543546] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[19392539166] [INFO] [kernel::memory] Frame allocator initialized with 495747 free frames
[19411220136] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[19423562037] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[19425651300] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[19431127683] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[19444136712] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[19449435654] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[19459011099] [INFO] [bran::arch] IOAPIC: Registers initialized
[19461405381] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[19463878071] [INFO] [bran::arch] IOAPIC: All pins masked
[19466107815] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[19467129033] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[19469060193] [INFO] [bran::arch] IOAPIC: Init complete
[19473738042] [INFO] [kernel] Initializing global allocator...
[20157650865] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[20161715706] [INFO] [kernel] Initializing SIMD...
[20164762398] [INFO] [kernel] Initializing tasking...
[20176518087] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[20177722521] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[20178661668] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[20189079735] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[20195053890] [INFO] [kernel::task::scheduler]   Initializing boot task...
[20198680095] [INFO] [kernel::task::scheduler]   Creating boot task...
[20208380181] [INFO] [kernel::task::scheduler]   Creating idle task...
[20217337074] [INFO] [kernel::task::scheduler]   Boot task initialized
[20223427323] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[20230785531] [INFO] [kernel::task::scheduler]   Scheduler initialized
[20245176930] [INFO] [kernel::root] Spawning Root service...
[20264276538] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[20283995985] [INFO] [kernel::root::service] ROOT: started once
[21835032219] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[21840168603] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[21921845847] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[21961759545] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[22013720586] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[22080611685] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[22084812288] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[22157179308] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[22216569474] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[22229675622] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[22238511537] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[22280430414] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[22288172709] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[22294441191] [INFO] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[22300368618] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[22305072240] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[22312796682] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22334954103] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[22360792443] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[22365262128] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[22370285850] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[22374576708] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[22388636589] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[22393573290] [INFO] [kernel] Spawning init process...
[22396777425] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[22432970274] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62500900 ticks/sec), init_cnt=625009 for 100Hz
[22442409693] [INFO] [kernel] Entering scheduler loop.
[22444806087] [DEBUG] [sched.switch] Context switch from_tid=0 to_tid=2 from_user=0 to_user=0 cr3_before=50319360 cr3_after=2020306944
[22475188362] [DEBUG] [sched.switch] Context switch from_tid=2 to_tid=3 from_user=0 to_user=1 cr3_before=2020306944 cr3_after=50319360
[22482280689] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0008580
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[22490515377] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367676096 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563776472
[22526748816] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[22538279445] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[22540180476] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[22541490906] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[22546459155] [DEBUG] [sched.switch] Context switch from_tid=3 to_tid=2 from_user=1 to_user=0 cr3_before=50319360 cr3_after=2020306944
[22579073187] [DEBUG] [sched.switch] Context switch from_tid=0 to_tid=3 from_user=0 to_user=1 cr3_before=2020306944 cr3_after=50319360
[22587105519] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[22589248968] [DEBUG] [sched.switch] Context switch from_tid=3 to_tid=2 from_user=1 to_user=0 cr3_before=50319360 cr3_after=2020306944
[22597894803] [DEBUG] [sched.switch] Context switch from_tid=0 to_tid=3 from_user=0 to_user=1 cr3_before=2020306944 cr3_after=50319360
[22611087015] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[22617408297] [DEBUG] [sched.switch] Context switch from_tid=3 to_tid=2 from_user=1 to_user=0 cr3_before=50319360 cr3_after=2020306944
[22625318826] [DEBUG] [sched.switch] Context switch from_tid=0 to_tid=3 from_user=0 to_user=1 cr3_before=2020306944 cr3_after=50319360
[22638248853] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[22640333100] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[22649162382] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[22658837586] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[22662895629] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[22670958255] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[22677527664] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[22680876702] [INFO] [sprout::devtree] SPROUT: build() called
[22681947882] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[22690608699] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[22698603246] [INFO] [sprout] SPROUT: About to create Supervisor...
[22701526056] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[22703581593] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[22705871133] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[22716578907] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[22832923872] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[23006738535] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[23012583429] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[23029138077] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[23046316524] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[23051851845] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[23075721504] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[23094633408] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[23100702108] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[23116479639] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[23128944795] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[23147785419] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[23174756682] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[23193678915] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[23213188977] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[23233006005] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[23239138230] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[23267288022] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[23274422490] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[23293513947] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[23306036985] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[23321159565] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[23354920941] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[23361857871] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[23380503102] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[23394924564] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[23406774402] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[23413573029] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[23427173748] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[23438977287] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[23445309954] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[23452205766] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[23466392268] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[23474223597] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[23480917878] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[23495342211] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[23505582837] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[23514420072] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[23521641627] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[23535081207] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[23549147061] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[23559895062] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[23575853466] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[23598400287] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[23649438318] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[23908016319] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[23909773833] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[23913264672] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[23931083088] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[23945879232] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[23947639848] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[23952497976] [INFO] [kernel::task::loader] Loading module: /boot/clock
[23953641294] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23955890145] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[23976301305] [INFO] [kernel::task::loader] Segment: vaddr=203dc0 exec=false
[23977838181] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[23982788148] [INFO] [kernel::task::loader] Segment: vaddr=204e30 exec=false
[23984499528] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[23997465987] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[24006908673] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[24008683776] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[24009706578] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24020227440] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[24029998839] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[24036471921] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[24042273486] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[24043317771] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[24055112037] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[24062021049] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[24065466711] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[24066491691] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24068356092] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[24087264663] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[24094046823] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[24107006748] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[24108204714] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[24119875032] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[24126470709] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[24133994511] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[24139440633] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24141385059] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[24150224175] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[24156944427] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[24161484336] [INFO] [kernel::task::loader] Segment: vaddr=204c88 exec=false
[24164857101] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[24176695026] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[24183522726] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[24216014229] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0042c18
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24226745202] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367998592 RFLAGS_BEFORE=130 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563776472
[24234072852] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[24235801590] [INFO] [clock] starting clock publisher
[24252162792] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24264783477] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368016976 RFLAGS_BEFORE=134 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563776472
[24280046505] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044678
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[24290569974] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368034544 RFLAGS_BEFORE=134 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563776472
[24294402198] [ERROR] [INGESTD] Starting...
[24304811520] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004a130
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24307379184] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368062240 RFLAGS_BEFORE=130 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563776472
[24312613446] [INFO] [cambium] cambium starting (v3: catch-up then stream)...
[24329405364] [INFO] [clock] Clock thing created: 367
[24330656493] [INFO] [clock] Waiting for UI Root (Compositor)...
[24354003003] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[24356327226] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[24358029795] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[24370321569] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[24396919899] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[24437007408] [ERROR] [INGESTD] Watch active. Loop start.
[24475294899] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[24477635523] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1920x1080 stride=7680)
[25218088764] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[25225282038] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[25228530327] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[25233911340] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[25240565163] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[25244367126] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[25245415470] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[25257607485] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[25277504868] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[25282244196] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368113664 RFLAGS_BEFORE=134 CR3_BEFORE=59535360 fs_base=0 gs_base=18446744071563776472
[25288596168] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[25324490664] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[25359963618] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x7e9000 -> virt=0x10000000
[25377722304] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[25384716555] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[25386576435] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[25388441727] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[25393917285] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[25400460129] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[25407531501] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[25408527573] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[25420494231] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[25428133467] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[25431577611] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[25435555068] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[25437925359] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[25439683500] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[25440541137] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[25442397024] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[25451368668] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[25457749977] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[25460994966] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[25461909792] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[25477026465] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[25479871395] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[25482714708] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[25484438430] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[25489695594] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[25496359053] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[25502140455] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[25503142632] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[25514823510] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[25523421297] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[25526336022] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[25527208575] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[25529316582] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[25539380493] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[25540867737] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[25542551133] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[25543484538] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[25554864555] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[25587803505] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044678
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[25591009224] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368136000 RFLAGS_BEFORE=130 CR3_BEFORE=59650048 fs_base=0 gs_base=18446744071563776472
[25598767755] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[25607212950] [INFO] [rtc_cmos] Starting... arg=db
[25609876545] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[25617432324] [INFO] [rtc_cmos] RTC: 2026-01-20 04:26:49 = 1768883209 unix_secs
[25623190989] [INFO] [kernel::time] System clock anchored: unix_secs=1768883209, mono_ns=12811171378, offset=1768883196188828622ns
[25625269296] [INFO] [rtc_cmos] System clock anchored
[25654810863] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[25665478839] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00944d0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[25668842826] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368170528 RFLAGS_BEFORE=130 CR3_BEFORE=59752448 fs_base=0 gs_base=18446744071563776472
[25675221891] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[25682600163] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[25688446245] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[25690254018] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[25707111903] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00a1ad8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[25719638802] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368188016 RFLAGS_BEFORE=130 CR3_BEFORE=59846656 fs_base=0 gs_base=18446744071563776472
[25725972756] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[25728596223] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[25740348447] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00b5db8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[25747568253] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368213888 RFLAGS_BEFORE=130 CR3_BEFORE=59949056 fs_base=0 gs_base=18446744071563776472
[25753491588] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[25767936348] [INFO] [bristle] bristle: registered in graph as svc.Input (id=468)
[25788846237] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=442 backend=BootFB
[25802269449] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[25807046859] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[25809027816] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[25908385899] [INFO] [kernel::task::loader] Segment: vaddr=261450 exec=false
[25915697049] [INFO] [kernel::task::loader]   Overlap at 261000: merging perms to r=true w=false x=true
[25935208926] [INFO] [kernel::task::loader] Segment: vaddr=26cfb8 exec=false
[25939392072] [INFO] [kernel::task::loader]   Overlap at 26c000: merging perms to r=true w=true x=true
[25952502279] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[25960597608] [INFO] [kernel::task::loader] Loading module: /boot/echo
[25966708713] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[25969297398] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[25974787707] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[25975842420] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[25982590755] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[25988975496] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[26003152065] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[26010774603] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[26014015071] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[26036131770] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014b08
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1ba
[26049020580] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368258464 RFLAGS_BEFORE=134 CR3_BEFORE=60055552 fs_base=0 gs_base=18446744071563776472
[26058163263] [INFO] [bloom::logging] bloom: logging initialized
[26077643328] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[26080248381] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[26082076449] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
[26087101986] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00944d0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[26096772174] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368275952 RFLAGS_BEFORE=134 CR3_BEFORE=60592128 fs_base=0 gs_base=18446744071563776472
[26104164801] [INFO] [echo] echo: online (handle=12)
[26109357351] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[26125663806] [INFO] [bloom] bloom: [bloom] Bootstrapped via Bytespace ID=442
[26138461932] [INFO] [bloom] bloom: [bloom] starting (arg_req=1 arg_resp=4 bristle=10 font_svc=0)
[26158392216] [INFO] [bloom] bloom: [bloom] spawned wallpaper_loader thread (tid=15)
[26180556765] [INFO] [bloom] bloom: [bloom] spawned cursor_loader thread (tid=16)
[26202347787] [INFO] [bloom] bloom: [bloom] spawned font_loader thread (tid=17)
[26214956196] [INFO] [bloom] bloom: [bloom] discovering compositor target...
T:B040 [26234631885] [INFO] [bloom] bloom: [wallpaper_loader] thread started
T:AC70 [26250829176] [INFO] [bloom] bloom: [cursor_loader] thread started
T:A200 [26269297626] [INFO] [bloom] bloom: [font_loader] thread started
[26271079527] [INFO] [bloom] bloom: [font_loader] scanning boot modules for fonts...
[26290548966] [INFO] [bloom] bloom: [font_loader] found 42 boot modules
[26657583150] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/DSEG7Classic-Regular.ttf'
[26696774412] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=179 size=23272 name='/assets/fonts/DSEG7Classic-Regular.ttf'
[26764262877] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:3620 [26784730500] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[26789162829] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[26795009340] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/Hack-Regular.ttf'
[26810055987] [INFO] [ps2_mouse] ps2_mouse: init done
[26811608109] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[26813093373] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[26817129735] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[26827209651] [INFO] [bloom::asset] [asset_bank] mapped at 0x107eb000
[26833088502] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[26873580855] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[26884208901] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[26900751999] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=182 size=309408 name='/assets/fonts/Hack-Regular.ttf'
[26918908764] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[26923490154] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSans-Regular.ttf'
[26927950896] [INFO] [bloom] bloom: [wallpaper_loader] searching for wallpaper...
[26935174266] [INFO] [bloom] bloom: [wallpaper_loader] trying: /assets/wallpapers/clouds.bmp
[26949059973] [INFO] [bloom::asset] [asset_bank] mapped at 0x107f1000
[26955354162] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[27509823462] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=185 size=569208 name='/assets/fonts/NotoSans-Regular.ttf'
[27525963564] [INFO] [bloom] bloom: [cursor_loader] searching for cursor...
[27532940094] [INFO] [bloom] bloom: [cursor_loader] trying: /assets/cursors/plain/Normal.cur
[27855359895] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol-Regular.ttf'
[27872360010] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[27876989382] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[27878422374] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[27893635374] [INFO] [bloom::asset] [asset_bank] mapped at 0x1083d000
[27900043050] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[28212445635] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=188 size=258156 name='/assets/fonts/NotoSansSymbol-Regular.ttf'
[28672838535] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[29496112503] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=191 size=656852 name='/assets/fonts/NotoSansSymbol2-Regular.ttf'
[29970449190] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSerif-Regular.ttf'
[30344080767] [INFO] [bloom::compositor] bloom: compositor bytespace 387 (1920x1080 stride=7680 format=1)
[30702115158] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=194 size=616196 name='/assets/fonts/NotoSerif-Regular.ttf'
[31031672595] [INFO] [bloom] bloom: [font_loader] immediate scan complete, entering watch loop
[31365005265] [INFO] [bloom::compositor] bloom: display backend: BootFB
[31703680272] [INFO] [bloom::compositor] bloom: mapped size=8294400 (source=bytespace_info)
[32056043250] [INFO] [bloom] bloom: [bloom] compositor target: 1920x1080 @ 0x108c8000 backend=BootFB
[32058095685] [INFO] [bloom] bloom: [bloom] presenter: driver (req=1 resp=4)
[32060355591] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[32452222869] [INFO] [stem::ui] UiBuilder: created root 552
[32458665525] [INFO] [bloom] bloom: [bloom] created UI root node: 552
[32492978364] [INFO] [display_bootfb] display_bootfb: bound bytespace 387
[32564910246] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[32571377850] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[32573577333] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (258156 bytes)
[32584130106] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd5b0
[32592292755] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[32593723668] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[32595272919] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[32609736852] [INFO] [bloom::asset] [asset_bank] mapped at 0x1189a000
[32617319955] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[32850634443] [INFO] [bloom] bloom: [bloom] ui watch opened UNFILTERED (userspace filter pred=UI_TEXT id=530)
[33614517255] [INFO] [clock] Found UI Root: 552 (attempt 5)
[36004687950] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c1b40
[36006913899] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[36008120478] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36009351378] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[36312950520] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[36321078651] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[36326672382] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 191 (656852 bytes)
[36335037354] [INFO] [bloom] bloom: [font_loader] watch opened (id=576)
[36358886289] [INFO] [bloom::asset] [asset_bank] mapped at 0x118da000
[36363948753] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[37956480771] [INFO] [bloom] bloom: [cursor_loader] SUCCESS: found candidate '/assets/cursors/plain/Normal.cur', enqueuing load
[38637461874] [INFO] [bloom] bloom: [cursor_loader] thread done, sleeping forever
[39643225347] [INFO] [bloom] bloom: [wallpaper_loader] SUCCESS: found candidate '/assets/wallpapers/clouds.bmp', enqueuing load
[43583761089] [INFO] [bloom] bloom: [wallpaper_loader] thread done, sleeping forever
[45635270043] [INFO] [cambium] Found 1 bindings
[45982365594] [INFO] [clock] Binding created: 596 (source=367 target=585)
[45997197312] [INFO] [clock] CLOCK: Entering main loop, publishing to thing_id=367
[46001970861] [INFO] [clock] unix=1768883219 utc=2026-01-20 04:26:59 mono_ns=22999684224
[46786260543] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fedd0
[46793927301] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[46795346004] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[46796839683] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x4 kind=0 pred=0 subj_lo=367
[47117792040] [INFO] [cambium] Opened watch 611 for source 367 (binding 596, start_seq=0)
[47168610984] [INFO] [cambium] CATCH-UP: Draining 1 watches for historical events...
[47510306811] [INFO] [clock] CLOCK PUBLISH: thing=367 now_text='04:26:59' tick=22999684224

```
</details>
