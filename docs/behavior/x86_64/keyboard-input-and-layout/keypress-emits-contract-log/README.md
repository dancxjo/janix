# ❌ Scenario: Keypress emits contract log

> Last run: 2026-01-19 21:32:34

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the clock window is ticking | ✅ | 10107ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I press a key | ❌ | 1013ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11528713845] [CONTRACT] [kernel] thing-os kernel starting...
[11538864480] [INFO] [kernel::memory] Memory map has 64 entries
[11541443562] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[11542629087] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[11543043303] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[11543429205] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[11543764188] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[11544094848] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[11544431745] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[11544796560] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[11545168272] [INFO] [kernel::memory]   [8] 0x1780000 - 0x7866a000 (Usable)
[11545516323] [INFO] [kernel::memory]   [9] 0x7866a000 - 0x786cc000 (Reserved)
[11545874505] [INFO] [kernel::memory]   [10] 0x786cc000 - 0x7884e000 (Other)
[11546225097] [INFO] [kernel::memory]   [11] 0x7884e000 - 0x7884f000 (Reserved)
[11546589879] [INFO] [kernel::memory]   [12] 0x7884f000 - 0x788e6000 (Other)
[11546940174] [INFO] [kernel::memory]   [13] 0x788e6000 - 0x788e7000 (Reserved)
[11547304362] [INFO] [kernel::memory]   [14] 0x788e7000 - 0x78988000 (Other)
[11547660762] [INFO] [kernel::memory]   [15] 0x78988000 - 0x78989000 (Reserved)
[11548057884] [INFO] [kernel::memory]   [16] 0x78989000 - 0x789c9000 (Other)
[11548407519] [INFO] [kernel::memory]   [17] 0x789c9000 - 0x789ca000 (Reserved)
[11548768968] [INFO] [kernel::memory]   [18] 0x789ca000 - 0x78a55000 (Other)
[11549116887] [INFO] [kernel::memory]   [19] 0x78a55000 - 0x78a56000 (Reserved)
[11549477148] [INFO] [kernel::memory]   [20] 0x78a56000 - 0x78aa2000 (Other)
[11549824968] [INFO] [kernel::memory]   [21] 0x78aa2000 - 0x78aa3000 (Reserved)
[11550187209] [INFO] [kernel::memory]   [22] 0x78aa3000 - 0x78aa9000 (Other)
[11550536085] [INFO] [kernel::memory]   [23] 0x78aa9000 - 0x78aaa000 (Reserved)
[11550903375] [INFO] [kernel::memory]   [24] 0x78aaa000 - 0x78f2b000 (Other)
[11551288122] [INFO] [kernel::memory]   [25] 0x78f2b000 - 0x78f2c000 (Reserved)
[11551651617] [INFO] [kernel::memory]   [26] 0x78f2c000 - 0x793ad000 (Other)
[11552002176] [INFO] [kernel::memory]   [27] 0x793ad000 - 0x793ae000 (Reserved)
[11552364648] [INFO] [kernel::memory]   [28] 0x793ae000 - 0x796af000 (Other)
[11552713491] [INFO] [kernel::memory]   [29] 0x796af000 - 0x796b0000 (Reserved)
[11553081837] [INFO] [kernel::memory]   [30] 0x796b0000 - 0x796b9000 (Other)
[11553440712] [INFO] [kernel::memory]   [31] 0x796b9000 - 0x796ba000 (Reserved)
[11553810378] [INFO] [kernel::memory]   [32] 0x796ba000 - 0x796be000 (Other)
[11554169748] [INFO] [kernel::memory]   [33] 0x796be000 - 0x796bf000 (Reserved)
[11554541262] [INFO] [kernel::memory]   [34] 0x796bf000 - 0x796c1000 (Other)
[11554941816] [INFO] [kernel::memory]   [35] 0x796c1000 - 0x796c2000 (Reserved)
[11555306301] [INFO] [kernel::memory]   [36] 0x796c2000 - 0x796c4000 (Other)
[11555656992] [INFO] [kernel::memory]   [37] 0x796c4000 - 0x796c5000 (Reserved)
[11556020487] [INFO] [kernel::memory]   [38] 0x796c5000 - 0x796c7000 (Other)
[11556371574] [INFO] [kernel::memory]   [39] 0x796c7000 - 0x796c8000 (Reserved)
[11556784536] [INFO] [kernel::memory]   [40] 0x796c8000 - 0x796ca000 (Other)
[11557135755] [INFO] [kernel::memory]   [41] 0x796ca000 - 0x796cb000 (Reserved)
[11557500537] [INFO] [kernel::memory]   [42] 0x796cb000 - 0x796cd000 (Other)
[11557869543] [INFO] [kernel::memory]   [43] 0x796cd000 - 0x796ce000 (Reserved)
[11558234490] [INFO] [kernel::memory]   [44] 0x796ce000 - 0x796d8000 (Other)
[11558585313] [INFO] [kernel::memory]   [45] 0x796d8000 - 0x796d9000 (Reserved)
[11558948643] [INFO] [kernel::memory]   [46] 0x796d9000 - 0x796dd000 (Other)
[11559297321] [INFO] [kernel::memory]   [47] 0x796dd000 - 0x796de000 (Reserved)
[11559658902] [INFO] [kernel::memory]   [48] 0x796de000 - 0x796e0000 (Other)
[11560006227] [INFO] [kernel::memory]   [49] 0x796e0000 - 0x796e1000 (Reserved)
[11560367049] [INFO] [kernel::memory]   [50] 0x796e1000 - 0x796e3000 (Other)
[11560715892] [INFO] [kernel::memory]   [51] 0x796e3000 - 0x796e4000 (Reserved)
[11561075460] [INFO] [kernel::memory]   [52] 0x796e4000 - 0x796e8000 (Other)
[11561436150] [INFO] [kernel::memory]   [53] 0x796e8000 - 0x79757000 (Other)
[11561838090] [INFO] [kernel::memory]   [54] 0x79757000 - 0x7990d000 (Other)
[11562188220] [INFO] [kernel::memory]   [55] 0x7990d000 - 0x7a16c000 (Reserved)
[11562548448] [INFO] [kernel::memory]   [56] 0x7a16c000 - 0x7bb6c000 (Usable)
[11562900096] [INFO] [kernel::memory]   [57] 0x7bb6c000 - 0x7bb8d000 (Reserved)
[11563260984] [INFO] [kernel::memory]   [58] 0x7bb8d000 - 0x7bb91000 (Other)
[11563611081] [INFO] [kernel::memory]   [59] 0x7bb91000 - 0x7bb92000 (Reserved)
[11563974642] [INFO] [kernel::memory]   [60] 0x7bb92000 - 0x7bb94000 (Other)
[11564324607] [INFO] [kernel::memory]   [61] 0x7bb94000 - 0x7bb95000 (Reserved)
[11564706087] [INFO] [kernel::memory]   [62] 0x7bb95000 - 0x7bb97000 (Other)
[11565058329] [INFO] [kernel::memory]   [63] 0x7bb97000 - 0x7bb98000 (Reserved)
[11565867093] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[11811133884] [CONTRACT] [kernel::memory] Frame allocator initialized with 495746 free frames
[11819022303] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[11824388169] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[11825841225] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[11826784068] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[11833667703] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[11834386740] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11837733138] [INFO] [bran::arch] IOAPIC: Registers initialized
[11839102605] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[11840980668] [INFO] [bran::arch] IOAPIC: All pins masked
[11842589847] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11843493684] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11844304527] [INFO] [bran::arch] IOAPIC: Init complete
[11845165101] [CONTRACT] [kernel] Initializing global allocator...
[12205504179] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[12206555163] [CONTRACT] [kernel] Initializing SIMD...
[12208190313] [CONTRACT] [kernel] Initializing tasking...
[12213614358] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[12215539182] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[12216437277] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[12222361569] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[12223056384] [INFO] [kernel::task::scheduler]   Initializing boot task...
[12224122977] [INFO] [kernel::task::scheduler]   Creating boot task...
[12228701067] [INFO] [kernel::task::scheduler]   Creating idle task...
[12233767854] [INFO] [kernel::task::scheduler]   Boot task initialized
[12234427821] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[12235569324] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[12241526649] [INFO] [kernel::root] Spawning Root service...
[12249424407] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[12258748656] [INFO] [kernel::root::service] ROOT: started once
[13157042052] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[13157928069] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[13202605284] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[13221722976] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[13250717997] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[13287383637] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[13289809467] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[13330511304] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[13358833323] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[13366584000] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[13369168527] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[13395560574] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[13398491700] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[13399559151] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[13403127474] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[13406451234] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[13407443082] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13415502309] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13428044190] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[13429164243] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[13431706266] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[13432934295] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[13441456479] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[13442229735] [CONTRACT] [kernel] Spawning init process...
[13443812580] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[13478627547] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62200600 ticks/sec), init_cnt=622006 for 100Hz
[13480434858] [CONTRACT] [kernel] Entering scheduler loop.
[13491568893] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[13496495727] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367676144 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563776488
[13507109319] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[13508586300] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[13509370545] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[13510161621] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[13532302146] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[13536320589] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[13539875976] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[13540671507] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[13545971934] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[13548390009] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[13549178940] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[13552267641] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[13553023308] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[13553735283] [INFO] [sprout::devtree] SPROUT: build() called
[13554385911] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[13558724586] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[13559543184] [INFO] [sprout] SPROUT: About to create Supervisor...
[13560287796] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[13561109727] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[13561757121] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[13567098666] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[13603898484] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[13608860628] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[13612327905] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[13615412349] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[13617668295] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[13620555333] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[13624064850] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[13627177047] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[13630192092] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[13633293894] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[13636333755] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[13639722030] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[13643527194] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[13646500725] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[13649455611] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[13652884872] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[13655741979] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[13658601627] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[13661575587] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[13664879613] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[13667904624] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[13671214062] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[13674351042] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[13678159077] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[13681353840] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[13684350504] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[13687383204] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[13690394058] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[13693551795] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[13696541826] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[13699700817] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[13702983393] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[13706036124] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[13709365857] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[13712418720] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[13715570154] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[13718785344] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[13721998983] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[13725128637] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[13728162162] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[13731375339] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[13734401967] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[13737503505] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[13739797203] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[13752389805] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[13824183549] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[13825124181] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[13826965251] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[13828024419] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[13828786488] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[13829603799] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[13832112591] [INFO] [kernel::task::loader] Loading module: /boot/clock
[13832648874] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13833720846] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13837811691] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[13838405394] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[13839842247] [INFO] [kernel::task::loader] Segment: vaddr=204ee8 exec=false
[13840613754] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13847842140] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[13850708454] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[13851687927] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[13852254438] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13853709309] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13857649905] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[13858212489] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[13859000793] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[13859593077] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13865643099] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[13866905481] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[13867878717] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[13868392890] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13869363222] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13879862832] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[13880527485] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[13884732807] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[13885331163] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[13891604661] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[13892384319] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[13893854337] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[13894371348] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13895772132] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13899755661] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[13900369593] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[13901673687] [INFO] [kernel::task::loader] Segment: vaddr=204c88 exec=false
[13902321114] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13909008069] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[13909846104] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[13926858429] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13928161335] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367986928 RFLAGS_BEFORE=134 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563776488
[13932185718] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[13933239210] [INFO] [clock] starting clock publisher
[13946207154] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13947667965] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368009968 RFLAGS_BEFORE=134 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563776488
[13953199986] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ab8
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[13954335549] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368026352 RFLAGS_BEFORE=134 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563776488
[13956223941] [ERROR] [INGESTD] Starting...
[13959767646] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ae8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13960849683] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368053488 RFLAGS_BEFORE=134 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563776488
[13963435761] [INFO] [cambium] cambium starting (v3: catch-up then stream)...
[13969365927] [INFO] [clock] Clock thing created: 356
[13970064834] [INFO] [clock] Waiting for UI Root (Compositor)...
[13976728425] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[13978148745] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[13979227152] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[13980539793] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[13992916146] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[14012398158] [ERROR] [INGESTD] Watch active. Loop start.
[14020325187] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[14021669607] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[14255946507] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[14256932052] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[14258501796] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14261750910] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[14262698472] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14264091798] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[14264988870] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[14273453007] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[14280836460] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[14282350830] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368106128 RFLAGS_BEFORE=134 CR3_BEFORE=54927360 fs_base=0 gs_base=18446744071563776488
[14285881962] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[14304825315] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[14314006740] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[14322945615] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[14324221395] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[14324799258] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[14325921720] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14329987848] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[14330969565] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14332563003] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[14333224554] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[14340127791] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[14341496268] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[14342943648] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[14344162074] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[14345682879] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[14346748119] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[14347264536] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14348372148] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14350960866] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[14351569650] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[14352530841] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[14353101774] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[14360309964] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[14361869478] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[14362460145] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14363573532] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14366662530] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[14367319956] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14368704669] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[14369326554] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[14375779407] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[14377600512] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[14378713767] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[14379245364] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14380273479] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14384032047] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[14384997726] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14386237767] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[14386823385] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[14393171364] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[14409511908] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044678
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[14411617770] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368129664 RFLAGS_BEFORE=130 CR3_BEFORE=55033856 fs_base=0 gs_base=18446744071563776488
[14417223480] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[14418284034] [INFO] [rtc_cmos] Starting... arg=db
[14419891794] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[14423943270] [INFO] [rtc_cmos] RTC: 2026-01-20 05:33:01 = 1768887181 unix_secs
[14425279440] [INFO] [kernel::time] System clock anchored: unix_secs=1768887181, mono_ns=7212381429, offset=1768887173787618571ns
[14426517996] [INFO] [rtc_cmos] System clock anchored
[14446407591] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[14448903150] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ae8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[14450035941] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368164352 RFLAGS_BEFORE=134 CR3_BEFORE=55136256 fs_base=0 gs_base=18446744071563776488
[14453465961] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[14455534665] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[14456736690] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[14457571920] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[14463042132] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004b958
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[14464728696] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368181760 RFLAGS_BEFORE=134 CR3_BEFORE=55230464 fs_base=0 gs_base=18446744071563776488
[14472417333] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[14474436999] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[14479257837] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0095fb0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[14480570577] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368207632 RFLAGS_BEFORE=130 CR3_BEFORE=55332864 fs_base=0 gs_base=18446744071563776488
[14484146391] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[14491272873] [INFO] [bristle] bristle: registered in graph as svc.Input (id=457)
[14501299296] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=431 backend=BootFB
[14502568707] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[14503485975] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14505057666] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14567916924] [INFO] [kernel::task::loader] Segment: vaddr=2615d0 exec=false
[14568840495] [INFO] [kernel::task::loader]   Overlap at 261000: merging perms to r=true w=false x=true
[14577027366] [INFO] [kernel::task::loader] Segment: vaddr=26d1a0 exec=false
[14577738813] [INFO] [kernel::task::loader]   Overlap at 26d000: merging perms to r=true w=true x=true
[14584718445] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[14586315711] [INFO] [kernel::task::loader] Loading module: /boot/echo
[14586858264] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14587909083] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14590809123] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[14591438664] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14592547431] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[14593246338] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[14599937385] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[14601158451] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[14602035162] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[14613673800] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014c50
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1af
[14614981326] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368249616 RFLAGS_BEFORE=130 CR3_BEFORE=55439360 fs_base=0 gs_base=18446744071563776488
[14618032176] [INFO] [bloom::logging] bloom: logging initialized
[14621417052] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004b958
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[14622563802] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368267088 RFLAGS_BEFORE=134 CR3_BEFORE=55975936 fs_base=0 gs_base=18446744071563776488
[14626581354] [INFO] [echo] echo: online (handle=12)
[14627457867] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[14634730737] [INFO] [bloom] bloom: [bloom] Bootstrapped via Bytespace ID=431
[14638211082] [INFO] [bloom] bloom: [bloom] starting (arg_req=1 arg_resp=4 bristle=10 font_svc=0)
[14648031552] [INFO] [bloom] bloom: [bloom] spawned wallpaper_loader thread (tid=15)
[14654460876] [INFO] [bloom] bloom: [bloom] spawned cursor_loader thread (tid=16)
[14661824925] [INFO] [bloom] bloom: [bloom] spawned font_loader thread (tid=17)
[14662612998] [INFO] [bloom] bloom: [bloom] discovering compositor target...
T:AEF0 [14668398591] [INFO] [bloom] bloom: [wallpaper_loader] thread started
T:AB20 [14670210225] [INFO] [bloom] bloom: [cursor_loader] thread started
[14677466397] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[14678367033] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[14679370266] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
T:A0B0 [14682892422] [INFO] [bloom] bloom: [font_loader] thread started
[14683868694] [INFO] [bloom] bloom: [font_loader] scanning boot modules for fonts...
[14689914261] [INFO] [bloom] bloom: [font_loader] found 42 boot modules
[14779547277] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/DSEG7Classic-Regular.ttf'
[14786826120] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=179 size=23272 name='/assets/fonts/DSEG7Classic-Regular.ttf'
[14814678054] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:35F0 [14820236079] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[14821752693] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[14825834760] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/Hack-Regular.ttf'
[14830621080] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[14831747436] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[14854924425] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[14858982369] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[14866257417] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=182 size=309408 name='/assets/fonts/Hack-Regular.ttf'
[14871096405] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[14873898039] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSans-Regular.ttf'
[14879335515] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[14880457977] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[15448114176] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=185 size=569208 name='/assets/fonts/NotoSans-Regular.ttf'
[15745436619] [INFO] [bloom] bloom: [wallpaper_loader] searching for wallpaper...
[15746509053] [INFO] [bloom] bloom: [wallpaper_loader] trying: /assets/wallpapers/clouds.bmp
[15779762526] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[15781223370] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[16071447645] [INFO] [bloom] bloom: [cursor_loader] searching for cursor...
[16072890603] [INFO] [bloom] bloom: [cursor_loader] trying: /assets/cursors/plain/Normal.cur
[16085159442] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[16088882799] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol-Regular.ttf'
[16096556718] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[16097682183] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[16737945576] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=188 size=258156 name='/assets/fonts/NotoSansSymbol-Regular.ttf'
[17054291859] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[17710203852] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=191 size=656852 name='/assets/fonts/NotoSansSymbol2-Regular.ttf'
[18042398253] [INFO] [bloom::compositor] bloom: compositor bytespace 376 (1280x720 stride=5120 format=1)
[18694198314] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSerif-Regular.ttf'
[18874106757] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[18875527803] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[18876476355] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (258156 bytes)
[18882270363] [INFO] [bloom::compositor] bloom: display backend: BootFB
[18886252440] [INFO] [ps2_mouse] ps2_mouse: init done
[18886971774] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[18888663387] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18889414203] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[18893590650] [INFO] [bloom::asset] [asset_bank] mapped at 0x10463000
[18894408489] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[19071991059] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=194 size=616196 name='/assets/fonts/NotoSerif-Regular.ttf'
[19076421111] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[19381751433] [INFO] [bloom] bloom: [font_loader] immediate scan complete, entering watch loop
[19390873920] [INFO] [bloom] bloom: [bloom] compositor target: 1280x720 @ 0x104a3000 backend=BootFB
[19392058686] [INFO] [bloom] bloom: [bloom] presenter: driver (req=1 resp=4)
[19393513788] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[19710556173] [INFO] [stem::ui] UiBuilder: created root 546
[19711422588] [INFO] [bloom] bloom: [bloom] created UI root node: 546
[19721239593] [INFO] [display_bootfb] display_bootfb: bound bytespace 376
[20038471629] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd5a0
[20039374377] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[20040069654] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20040983721] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[20372297946] [INFO] [bloom] bloom: [bloom] ui watch opened UNFILTERED (userspace filter pred=UI_TEXT id=517)
[20380532865] [INFO] [clock] Found UI Root: 546 (attempt 4)
[22246425795] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[22248590463] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[22249947060] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 191 (656852 bytes)
[22270233447] [INFO] [bloom::asset] [asset_bank] mapped at 0x10bab000
[22271043927] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[23708206896] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c1b40
[23709216828] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[23709950484] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23710731099] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[24036379917] [INFO] [bloom] bloom: [font_loader] watch opened (id=572)
[25346594625] [INFO] [bloom] bloom: [cursor_loader] SUCCESS: found candidate '/assets/cursors/plain/Normal.cur', enqueuing load
[28624044537] [INFO] [bloom] bloom: [cursor_loader] thread done, sleeping forever
[28629561279] [INFO] [bloom] bloom: [wallpaper_loader] SUCCESS: found candidate '/assets/wallpapers/clouds.bmp', enqueuing load
[28630964274] [INFO] [bloom] bloom: [wallpaper_loader] thread done, sleeping forever
[28751381175] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[28753017348] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol2-Regular.ttf' in slot 4
[28753957056] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 194 (616196 bytes)
[28764133431] [INFO] [bloom::asset] [asset_bank] mapped at 0x10c4c000
[28764882894] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSerif-Regular.ttf'...
[31580384682] [INFO] [clock] Binding created: 590 (source=356 target=575)
[31581430782] [INFO] [clock] CLOCK: Entering main loop, publishing to thing_id=356
[31584250632] [INFO] [clock] unix=1768887189 utc=2026-01-20 05:33:09 mono_ns=15791285119
[32923016643] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:33:09' tick=15791285119
[33579638829] [INFO] [cambium] Found 1 bindings
[34563018996] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fedd0
[34564290882] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[34565616822] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34567005099] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x4 kind=0 pred=0 subj_lo=356
[34889829513] [INFO] [cambium] Opened watch 609 for source 356 (binding 590, start_seq=0)
[34917051609] [INFO] [cambium] CATCH-UP: Draining 1 watches for historical events...
[35255040381] [INFO] [cambium] cambium: drain complete payloads=0 overflows=0 last_seq=none
[35256535281] [INFO] [cambium] Entering event loop with 1 bindings (v3)
[36235022571] [INFO] [clock] unix=1768887191 utc=2026-01-20 05:33:11 mono_ns=18117293452
[37553040294] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:33:11' tick=18117293452

```
</details>
