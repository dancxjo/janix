# ❌ Scenario: Keypress emits contract log

> Last run: 2026-01-20 18:14:20

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the clock window is ticking | ✅ | 9126ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I press a key | ❌ | 226ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[10314811650] [CONTRACT] [kernel] thing-os kernel starting...
[10324253280] [INFO] [kernel::memory] Memory map has 64 entries
[10326407190] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[10326999078] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[10327322841] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[10327662972] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[10327980069] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[10328290533] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[10328607102] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[10328918820] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[10329260502] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78656000 (Usable)
[10329679107] [INFO] [kernel::memory]   [9] 0x78656000 - 0x786b8000 (Reserved)
[10330023165] [INFO] [kernel::memory]   [10] 0x786b8000 - 0x7883a000 (Other)
[10330356036] [INFO] [kernel::memory]   [11] 0x7883a000 - 0x7883b000 (Reserved)
[10330701381] [INFO] [kernel::memory]   [12] 0x7883b000 - 0x788d2000 (Other)
[10331033361] [INFO] [kernel::memory]   [13] 0x788d2000 - 0x788d3000 (Reserved)
[10331377452] [INFO] [kernel::memory]   [14] 0x788d3000 - 0x78974000 (Other)
[10331788368] [INFO] [kernel::memory]   [15] 0x78974000 - 0x78975000 (Reserved)
[10332136419] [INFO] [kernel::memory]   [16] 0x78975000 - 0x789b5000 (Other)
[10332470577] [INFO] [kernel::memory]   [17] 0x789b5000 - 0x789b6000 (Reserved)
[10332863244] [INFO] [kernel::memory]   [18] 0x789b6000 - 0x78a41000 (Other)
[10333221789] [INFO] [kernel::memory]   [19] 0x78a41000 - 0x78a42000 (Reserved)
[10333573074] [INFO] [kernel::memory]   [20] 0x78a42000 - 0x78a8e000 (Other)
[10333910004] [INFO] [kernel::memory]   [21] 0x78a8e000 - 0x78a8f000 (Reserved)
[10334257956] [INFO] [kernel::memory]   [22] 0x78a8f000 - 0x78a95000 (Other)
[10334593896] [INFO] [kernel::memory]   [23] 0x78a95000 - 0x78a96000 (Reserved)
[10334968182] [INFO] [kernel::memory]   [24] 0x78a96000 - 0x78f17000 (Other)
[10335303264] [INFO] [kernel::memory]   [25] 0x78f17000 - 0x78f18000 (Reserved)
[10335650457] [INFO] [kernel::memory]   [26] 0x78f18000 - 0x79399000 (Other)
[10335986166] [INFO] [kernel::memory]   [27] 0x79399000 - 0x7939a000 (Reserved)
[10336356987] [INFO] [kernel::memory]   [28] 0x7939a000 - 0x7969b000 (Other)
[10336720251] [INFO] [kernel::memory]   [29] 0x7969b000 - 0x7969c000 (Reserved)
[10337087343] [INFO] [kernel::memory]   [30] 0x7969c000 - 0x796a5000 (Other)
[10337425032] [INFO] [kernel::memory]   [31] 0x796a5000 - 0x796a6000 (Reserved)
[10337770938] [INFO] [kernel::memory]   [32] 0x796a6000 - 0x796aa000 (Other)
[10338107175] [INFO] [kernel::memory]   [33] 0x796aa000 - 0x796ab000 (Reserved)
[10338453873] [INFO] [kernel::memory]   [34] 0x796ab000 - 0x796ad000 (Other)
[10338788592] [INFO] [kernel::memory]   [35] 0x796ad000 - 0x796ae000 (Reserved)
[10339134630] [INFO] [kernel::memory]   [36] 0x796ae000 - 0x796b0000 (Other)
[10339491096] [INFO] [kernel::memory]   [37] 0x796b0000 - 0x796b1000 (Reserved)
[10339840104] [INFO] [kernel::memory]   [38] 0x796b1000 - 0x796b3000 (Other)
[10340177661] [INFO] [kernel::memory]   [39] 0x796b3000 - 0x796b4000 (Reserved)
[10340569041] [INFO] [kernel::memory]   [40] 0x796b4000 - 0x796b6000 (Other)
[10340913330] [INFO] [kernel::memory]   [41] 0x796b6000 - 0x796b7000 (Reserved)
[10341267255] [INFO] [kernel::memory]   [42] 0x796b7000 - 0x796b9000 (Other)
[10341609366] [INFO] [kernel::memory]   [43] 0x796b9000 - 0x796ba000 (Reserved)
[10341963819] [INFO] [kernel::memory]   [44] 0x796ba000 - 0x796c4000 (Other)
[10342306227] [INFO] [kernel::memory]   [45] 0x796c4000 - 0x796c5000 (Reserved)
[10342681239] [INFO] [kernel::memory]   [46] 0x796c5000 - 0x796c9000 (Other)
[10343019654] [INFO] [kernel::memory]   [47] 0x796c9000 - 0x796ca000 (Reserved)
[10343373249] [INFO] [kernel::memory]   [48] 0x796ca000 - 0x796cc000 (Other)
[10343713578] [INFO] [kernel::memory]   [49] 0x796cc000 - 0x796cd000 (Reserved)
[10344100635] [INFO] [kernel::memory]   [50] 0x796cd000 - 0x796cf000 (Other)
[10344438852] [INFO] [kernel::memory]   [51] 0x796cf000 - 0x796d0000 (Reserved)
[10344785385] [INFO] [kernel::memory]   [52] 0x796d0000 - 0x796d4000 (Other)
[10345121358] [INFO] [kernel::memory]   [53] 0x796d4000 - 0x79750000 (Other)
[10345455516] [INFO] [kernel::memory]   [54] 0x79750000 - 0x79907000 (Other)
[10345795680] [INFO] [kernel::memory]   [55] 0x79907000 - 0x7a16c000 (Reserved)
[10346173629] [INFO] [kernel::memory]   [56] 0x7a16c000 - 0x7bb6c000 (Usable)
[10346514189] [INFO] [kernel::memory]   [57] 0x7bb6c000 - 0x7bb8d000 (Reserved)
[10346861811] [INFO] [kernel::memory]   [58] 0x7bb8d000 - 0x7bb91000 (Other)
[10347196992] [INFO] [kernel::memory]   [59] 0x7bb91000 - 0x7bb92000 (Reserved)
[10347581904] [INFO] [kernel::memory]   [60] 0x7bb92000 - 0x7bb94000 (Other)
[10347920847] [INFO] [kernel::memory]   [61] 0x7bb94000 - 0x7bb95000 (Reserved)
[10348273023] [INFO] [kernel::memory]   [62] 0x7bb95000 - 0x7bb97000 (Other)
[10348611372] [INFO] [kernel::memory]   [63] 0x7bb97000 - 0x7bb98000 (Reserved)
[10349164617] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[10583808642] [CONTRACT] [kernel::memory] Frame allocator initialized with 495726 free frames
[10590449232] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[10594889646] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[10595924658] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[10596604656] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[10602992400] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[10603447800] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[10606232967] [INFO] [bran::arch] IOAPIC: Registers initialized
[10607255934] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[10608628338] [INFO] [bran::arch] IOAPIC: All pins masked
[10609908441] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[10610497095] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[10610948205] [INFO] [bran::arch] IOAPIC: Init complete
[10611523065] [CONTRACT] [kernel] Initializing global allocator...
[10945183062] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[10946217777] [CONTRACT] [kernel] Initializing SIMD...
[10947539988] [CONTRACT] [kernel] Initializing tasking...
[10952267139] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[10953796359] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[10954353300] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[10960087446] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[10960495887] [INFO] [kernel::task::scheduler]   Initializing boot task...
[10961200899] [INFO] [kernel::task::scheduler]   Creating boot task...
[10965567591] [INFO] [kernel::task::scheduler]   Creating idle task...
[10970478750] [INFO] [kernel::task::scheduler]   Boot task initialized
[10970868810] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[10971700080] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[10977700437] [INFO] [kernel::root] Spawning Root service...
[10985008485] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[10993310163] [INFO] [kernel::root::service] ROOT: started once
[11844153783] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[11845068675] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[11885189481] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[11902037070] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[11927808981] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[11958440406] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[11960264151] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[11998409082] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[12021256797] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[12029617743] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[12032295462] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[12055987119] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[12058476969] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[12059441955] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[12062670609] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[12064999023] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[12065761455] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12072839229] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12083994285] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[12084824961] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[12087163176] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[12087970224] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[12095464821] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[12096079413] [CONTRACT] [kernel] Spawning init process...
[12097541643] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[12132115809] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62182000 ticks/sec), init_cnt=621820 for 100Hz
[12133587279] [CONTRACT] [kernel] Entering scheduler loop.
[12142173681] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[12146672868] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072367676144 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563780584
[12156696321] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[12158012625] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[12158739153] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[12159413442] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[12178956900] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[12182964849] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[12185918778] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[12186644679] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[12189858318] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[12192016584] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[12192736149] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[12195786768] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[12196513065] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[12197227416] [INFO] [sprout::devtree] SPROUT: build() called
[12198099144] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[12202419075] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[12203154150] [INFO] [sprout] SPROUT: About to create Supervisor...
[12203808936] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[12204553350] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[12205179294] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[12209552751] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[12247296798] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[12251778726] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[12254885478] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[12257837889] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[12260017770] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[12262850886] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[12266595297] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[12269642880] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[12272526519] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[12275631852] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[12278641584] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[12281962737] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[12285357711] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[12288184623] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[12291052521] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[12293905107] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[12297074955] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[12300026079] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[12302846952] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[12305985747] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[12309186219] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[12312338280] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[12315187665] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[12318075660] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[12321163305] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[12324073509] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[12327039384] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[12330278334] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[12333613281] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[12338653965] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[12341860212] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[12345281685] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[12348314682] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[12351396849] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[12354462648] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[12357597252] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[12360744759] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[12364386540] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[12367359642] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[12370262784] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[12373208760] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[12376213344] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[12379409823] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[12381604917] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[12393505476] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[12455245011] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[12456195312] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[12458118882] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[12459158514] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[12506745306] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[12507929907] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[12510407811] [INFO] [kernel::task::loader] Loading module: /boot/clock
[12510996366] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12512062761] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12515744472] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[12516325767] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[12517654842] [INFO] [kernel::task::loader] Segment: vaddr=204ee8 exec=false
[12518220594] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12524821782] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[12527779968] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[12528691923] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[12529196460] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12530096073] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12533996904] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[12534558399] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[12535358880] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[12536395542] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12542186877] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[12543329238] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[12544246935] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[12544724907] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12545649963] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12555531747] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[12556240488] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[12560384793] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[12561026544] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[12566752374] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[12567451941] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[12568388844] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[12568883052] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12569855562] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12573326733] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[12574510179] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[12575742696] [INFO] [kernel::task::loader] Segment: vaddr=204c88 exec=false
[12576302178] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12582156642] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[12582894357] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[12597871605] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0040128
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12599054655] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072367988688 RFLAGS_BEFORE=130 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563780584
[12602774514] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[12603647694] [INFO] [clock] starting clock publisher
[12611151069] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12612240531] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368012304 RFLAGS_BEFORE=130 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563780584
[12616824726] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0042c18
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[12617898546] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368028688 RFLAGS_BEFORE=130 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563780584
[12620187591] [ERROR] [INGESTD] Starting...
[12623904711] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044898
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12625097859] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368055824 RFLAGS_BEFORE=130 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563780584
[12628023738] [INFO] [cambium] cambium starting (v3: catch-up then stream)...
[12634223646] [INFO] [clock] Clock thing created: 356
[12634879059] [INFO] [clock] Waiting for UI Root (Compositor)...
[12639755601] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[12640916013] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[12641956569] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[12643210866] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[12650120703] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[12661159368] [ERROR] [INGESTD] Watch active. Loop start.
[12668899287] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[12670065771] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[12877321809] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[12878036127] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[12879091335] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12881876634] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[12882539637] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12883542738] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[12884121756] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[12890749311] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[12896977533] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[12898096299] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368106112 RFLAGS_BEFORE=130 CR3_BEFORE=54927360 fs_base=0 gs_base=18446744071563780584
[12901428144] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[12917009028] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[12925567776] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[12934009440] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[12935239416] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[12935973237] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[12937102464] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12940181991] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[12940747875] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12942076653] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[12942620427] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[12948217821] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[12949257156] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[12950538216] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[12952140729] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[12953496534] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[12954458517] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[12955009485] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12955961205] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12958218141] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[12958790691] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[12959674761] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[12960210285] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[12965812431] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[12967072668] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[12967595553] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12968537868] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12971376462] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[12971928189] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12973123548] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[12973666035] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[12979251846] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[12980766909] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[12981770241] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[12982256892] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12983158419] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12985963584] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[12986576592] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12987581838] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[12988612065] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[12994455936] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[13009003227] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0040128
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[13010112786] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368129760 RFLAGS_BEFORE=130 CR3_BEFORE=55033856 fs_base=0 gs_base=18446744071563780584
[13013600523] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[13014555807] [INFO] [rtc_cmos] Starting... arg=db
[13016041962] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[13019541480] [INFO] [rtc_cmos] RTC: 2026-01-21 02:14:46 = 1768961686 unix_secs
[13020733770] [INFO] [kernel::time] System clock anchored: unix_secs=1768961686, mono_ns=6510124087, offset=1768961679489875913ns
[13021962459] [INFO] [rtc_cmos] System clock anchored
[13037465364] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[13039595679] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045660
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[13040818791] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368164288 RFLAGS_BEFORE=134 CR3_BEFORE=55136256 fs_base=0 gs_base=18446744071563780584
[13044524559] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[13046379291] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[13047562209] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[13048379850] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[13053528015] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004b958
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[13054773402] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368181696 RFLAGS_BEFORE=134 CR3_BEFORE=55230464 fs_base=0 gs_base=18446744071563780584
[13059366705] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[13060244538] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[13063825533] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00965c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[13064952384] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368207568 RFLAGS_BEFORE=130 CR3_BEFORE=55332864 fs_base=0 gs_base=18446744071563780584
[13068312180] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[13074986991] [INFO] [bristle] bristle: registered in graph as svc.Input (id=457)
[13083683349] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=431 backend=BootFB
[13084789872] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[13085295861] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13086365787] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13148163633] [INFO] [kernel::task::loader] Segment: vaddr=26ea30 exec=false
[13149206400] [INFO] [kernel::task::loader]   Overlap at 26e000: merging perms to r=true w=false x=true
[13156450362] [INFO] [kernel::task::loader] Segment: vaddr=27a9b0 exec=false
[13157115312] [INFO] [kernel::task::loader]   Overlap at 27a000: merging perms to r=true w=true x=true
[13164335415] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[13165779924] [INFO] [kernel::task::loader] Loading module: /boot/echo
[13166311191] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13167325479] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13170275250] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[13170880107] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13171875354] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[13172699430] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13178350317] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[13179338502] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[13180091199] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[13191767853] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014c50
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1af
[13192998720] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368250160 RFLAGS_BEFORE=134 CR3_BEFORE=55439360 fs_base=0 gs_base=18446744071563780584
[13195908528] [INFO] [bloom::logging] bloom: logging initialized
[13219141188] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045950
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[13220332884] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368267632 RFLAGS_BEFORE=130 CR3_BEFORE=56033280 fs_base=0 gs_base=18446744071563780584
[13223591271] [INFO] [echo] echo: online (handle=12)
[13224350997] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[13230726927] [INFO] [bloom] bloom: [bloom] Bootstrapped via Bytespace ID=431
[13233732963] [INFO] [bloom] bloom: [bloom] starting (arg_req=1 arg_resp=4 bristle=10 font_svc=0)
[13242989826] [INFO] [bloom] bloom: [bloom] spawned wallpaper_loader thread (tid=15)
[13249749381] [INFO] [bloom] bloom: [bloom] spawned cursor_loader thread (tid=16)
[13255496562] [INFO] [bloom] bloom: [bloom] spawned font_loader thread (tid=17)
[13256203257] [INFO] [bloom] bloom: [bloom] discovering compositor target...
T:E870 [13261367163] [INFO] [bloom] bloom: [wallpaper_loader] thread started
[13267439856] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[13268373261] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[13269517602] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
T:E4A0 [13272617985] [INFO] [bloom] bloom: [cursor_loader] thread started
T:DA30 [13274651709] [INFO] [bloom] bloom: [font_loader] thread started
[13275377907] [INFO] [bloom] bloom: [font_loader] scanning boot modules for fonts...
[13281887982] [INFO] [bloom] bloom: [font_loader] found 42 boot modules
[13367913537] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/DSEG7Classic-Regular.ttf'
[13374818754] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=179 size=23272 name='/assets/fonts/DSEG7Classic-Regular.ttf'
[13383490329] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:FF10 [13388311794] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[13389502566] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[13392290175] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/Hack-Regular.ttf'
[13396465170] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[13397279214] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[13419274110] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[13421730696] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[13428211566] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=182 size=309408 name='/assets/fonts/Hack-Regular.ttf'
[13432461108] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[13434571128] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSans-Regular.ttf'
[13438811133] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[13439590758] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[13966718469] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[13968641808] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[13971772254] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=185 size=569208 name='/assets/fonts/NotoSans-Regular.ttf'
[13976493168] [INFO] [bloom] bloom: [wallpaper_loader] searching for wallpaper...
[13977364698] [INFO] [bloom] bloom: [wallpaper_loader] trying: /assets/wallpapers/clouds.bmp
[13981306515] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[13983686508] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol-Regular.ttf'
[13989650763] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[13990407882] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[14368594317] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=188 size=258156 name='/assets/fonts/NotoSansSymbol-Regular.ttf'
[14375065056] [INFO] [bloom] bloom: [cursor_loader] searching for cursor...
[14376057003] [INFO] [bloom] bloom: [cursor_loader] trying: /assets/cursors/plain/Normal.cur
[14690694282] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[15345125565] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=191 size=656852 name='/assets/fonts/NotoSansSymbol2-Regular.ttf'
[15969446460] [INFO] [bloom::compositor] bloom: compositor bytespace 376 (1280x720 stride=5120 format=1)
[16133355975] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[16134655911] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[16135554402] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (258156 bytes)
[16139573538] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSerif-Regular.ttf'
[16148011374] [INFO] [bloom::asset] [asset_bank] mapped at 0x10463000
[16148783046] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[16622799369] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=194 size=616196 name='/assets/fonts/NotoSerif-Regular.ttf'
[16626639018] [INFO] [bloom::compositor] bloom: display backend: BootFB
[16631864007] [INFO] [ps2_mouse] ps2_mouse: init done
[16632519321] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[16633380423] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[16634096886] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[16953612753] [INFO] [bloom] bloom: [font_loader] immediate scan complete, entering watch loop
[16956987993] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[17285651097] [INFO] [bloom] bloom: [bloom] compositor target: 1280x720 @ 0x104a3000 backend=BootFB
[17286878532] [INFO] [bloom] bloom: [bloom] presenter: driver (req=1 resp=4)
[17288334063] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[17369874225] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[17371211748] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[17372054601] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 191 (656852 bytes)
[17376695160] [INFO] [stem::ui] UiBuilder: created root 546
[17377425879] [INFO] [bloom] bloom: [bloom] created UI root node: 546
[17386428675] [INFO] [display_bootfb] display_bootfb: bound bytespace 376
[17391406230] [INFO] [bloom::asset] [asset_bank] mapped at 0x10bab000
[17392064877] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[17641316055] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd580
[17642257182] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[17642939490] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[17643819864] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[17968025130] [INFO] [bloom] bloom: [bloom] ui watch opened UNFILTERED (userspace filter pred=UI_TEXT id=521)
[19284207327] [INFO] [clock] Found UI Root: 546 (attempt 4)
[20024453625] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[20025618294] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol2-Regular.ttf' in slot 4
[20026537080] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 194 (616196 bytes)
[20038953495] [INFO] [bloom::asset] [asset_bank] mapped at 0x10c4c000
[20039773974] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSerif-Regular.ttf'...
[20980980867] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c1b40
[20982174279] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[20982882294] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20983615224] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[21311496756] [INFO] [bloom] bloom: [font_loader] watch opened (id=575)
[23606672298] [INFO] [bloom] bloom: [cursor_loader] SUCCESS: found candidate '/assets/cursors/plain/Normal.cur', enqueuing load
[23608634148] [INFO] [bloom] bloom: [cursor_loader] thread done, sleeping forever
[24607423797] [INFO] [bloom] bloom: [wallpaper_loader] SUCCESS: found candidate '/assets/wallpapers/clouds.bmp', enqueuing load
[24609120393] [INFO] [bloom] bloom: [wallpaper_loader] thread done, sleeping forever
[28302095481] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[28303447227] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSerif-Regular.ttf' in slot 5
[28304435511] [INFO] [bloom::asset] [asset_bank] load_cursor_immediate: /assets/cursors/plain/Normal.cur
[28325034507] [INFO] [clock] Binding created: 590 (source=356 target=580)
[28325865777] [INFO] [clock] CLOCK: Entering main loop, publishing to thing_id=356
[28328370873] [INFO] [clock] unix=1768961693 utc=2026-01-21 02:14:53 mono_ns=14163462109
[28377373035] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='02:14:53' tick=14163462109
[28484353623] [INFO] [bloom::asset] [asset_bank] mapping bytespace 152 (4286 bytes)
[28490752686] [INFO] [bloom::asset] [asset_bank] mapped to 0x10ce3000
[28491537987] [INFO] [bloom::asset] [asset_bank] checking CUR header: len=4286
[28492313553] [INFO] [bloom::asset] [asset_bank] valid CUR header detected
[28493212044] [INFO] [bloom::asset] [asset_bank] CUR: hotspot=(0, 0), img_size=4264, offset=22
[28494122184] [INFO] [bloom::asset] [asset_bank] decoding embedded DIB at offset 22...
[28495871151] [INFO] [bloom::asset] [asset_bank] SUCCESS: cursor DIB decoded 32x32
[28503006840] [INFO] [bloom::asset] [asset_bank] publish_cursor (pending)
[28503817683] [INFO] [bloom::asset] [asset_bank] cursor: Static frame 32x32 hotspot (0, 0)
[28504744224] [INFO] [bloom::asset] [asset_bank] load_wallpaper_immediate: /assets/wallpapers/clouds.bmp
[28623588477] [INFO] [bloom::asset] [asset_bank] mapping bytespace 170 (3145782 bytes)
[28636312848] [INFO] [bloom::asset] [asset_bank] mapped to 0x10ce5000
[28637087952] [INFO] [bloom::asset] [asset_bank] decoding BMP...
[29010468894] [INFO] [bloom::asset] [asset_bank] BMP decoded: 1024x1024
[29371021251] [INFO] [bloom::asset] [asset_bank] bytespace unmapped
[29371980825] [INFO] [bloom::asset] [asset_bank] publish_wallpaper (pending): 1024x1024
[31115994954] [INFO] [cambium] Found 1 bindings
[31127659794] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fedd0
[31128428661] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[31129107801] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[31129848024] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x4 kind=0 pred=0 subj_lo=356
[31136410338] [INFO] [cambium] Opened watch 635 for source 356 (binding 590, start_seq=0)
[31171127559] [INFO] [cambium] CATCH-UP: Draining 1 watches for historical events...
[31177946019] [INFO] [cambium] cambium: drain complete payloads=0 overflows=0 last_seq=none
[31178778015] [INFO] [cambium] Entering event loop with 1 bindings (v3)
[31904800785] [INFO] [bloom] bloom: [font_loader] drain complete: payloads=1038 overflows

```
</details>
