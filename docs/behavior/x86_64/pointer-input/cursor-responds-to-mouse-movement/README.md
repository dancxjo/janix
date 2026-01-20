# ✅ Scenario: Cursor responds to mouse movement

> Last run: 2026-01-19 21:17:21

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given a cursor is visible on the screen | ✅ | 4570ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | When I move the mouse | ✅ | 599ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the serial log should contain 'CONTRACT: input pointer_move' | ✅ | 407ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And the cursor should move correspondingly on the screen | ✅ | 385ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[10460921130] [CONTRACT] [kernel] thing-os kernel starting...
[10469955705] [INFO] [kernel::memory] Memory map has 64 entries
[10472059620] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[10472635437] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[10472956263] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[10473294216] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[10473620421] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[10473946395] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[10474265043] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[10474578477] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[10474928145] [INFO] [kernel::memory]   [8] 0x1780000 - 0x7866a000 (Usable)
[10475260191] [INFO] [kernel::memory]   [9] 0x7866a000 - 0x786cc000 (Reserved)
[10475603820] [INFO] [kernel::memory]   [10] 0x786cc000 - 0x7884e000 (Other)
[10476033084] [INFO] [kernel::memory]   [11] 0x7884e000 - 0x7884f000 (Reserved)
[10476385524] [INFO] [kernel::memory]   [12] 0x7884f000 - 0x788e6000 (Other)
[10476716415] [INFO] [kernel::memory]   [13] 0x788e6000 - 0x788e7000 (Reserved)
[10477076016] [INFO] [kernel::memory]   [14] 0x788e7000 - 0x78988000 (Other)
[10477402683] [INFO] [kernel::memory]   [15] 0x78988000 - 0x78989000 (Reserved)
[10477742055] [INFO] [kernel::memory]   [16] 0x78989000 - 0x789c9000 (Other)
[10478075388] [INFO] [kernel::memory]   [17] 0x789c9000 - 0x789ca000 (Reserved)
[10478424726] [INFO] [kernel::memory]   [18] 0x789ca000 - 0x78a55000 (Other)
[10478769576] [INFO] [kernel::memory]   [19] 0x78a55000 - 0x78a56000 (Reserved)
[10479141585] [INFO] [kernel::memory]   [20] 0x78a56000 - 0x78aa2000 (Other)
[10479485247] [INFO] [kernel::memory]   [21] 0x78aa2000 - 0x78aa3000 (Reserved)
[10479845904] [INFO] [kernel::memory]   [22] 0x78aa3000 - 0x78aa9000 (Other)
[10480184088] [INFO] [kernel::memory]   [23] 0x78aa9000 - 0x78aaa000 (Reserved)
[10480550553] [INFO] [kernel::memory]   [24] 0x78aaa000 - 0x78f2b000 (Other)
[10480894248] [INFO] [kernel::memory]   [25] 0x78f2b000 - 0x78f2c000 (Reserved)
[10481251242] [INFO] [kernel::memory]   [26] 0x78f2c000 - 0x793ad000 (Other)
[10481596587] [INFO] [kernel::memory]   [27] 0x793ad000 - 0x793ae000 (Reserved)
[10481954241] [INFO] [kernel::memory]   [28] 0x793ae000 - 0x796af000 (Other)
[10482298662] [INFO] [kernel::memory]   [29] 0x796af000 - 0x796b0000 (Reserved)
[10482655656] [INFO] [kernel::memory]   [30] 0x796b0000 - 0x796b9000 (Other)
[10483001826] [INFO] [kernel::memory]   [31] 0x796b9000 - 0x796ba000 (Reserved)
[10483360107] [INFO] [kernel::memory]   [32] 0x796ba000 - 0x796be000 (Other)
[10483716309] [INFO] [kernel::memory]   [33] 0x796be000 - 0x796bf000 (Reserved)
[10484072346] [INFO] [kernel::memory]   [34] 0x796bf000 - 0x796c1000 (Other)
[10484417427] [INFO] [kernel::memory]   [35] 0x796c1000 - 0x796c2000 (Reserved)
[10484768217] [INFO] [kernel::memory]   [36] 0x796c2000 - 0x796c4000 (Other)
[10485105840] [INFO] [kernel::memory]   [37] 0x796c4000 - 0x796c5000 (Reserved)
[10485460788] [INFO] [kernel::memory]   [38] 0x796c5000 - 0x796c7000 (Other)
[10485807156] [INFO] [kernel::memory]   [39] 0x796c7000 - 0x796c8000 (Reserved)
[10486161180] [INFO] [kernel::memory]   [40] 0x796c8000 - 0x796ca000 (Other)
[10486503423] [INFO] [kernel::memory]   [41] 0x796ca000 - 0x796cb000 (Reserved)
[10486873485] [INFO] [kernel::memory]   [42] 0x796cb000 - 0x796cd000 (Other)
[10487213847] [INFO] [kernel::memory]   [43] 0x796cd000 - 0x796ce000 (Reserved)
[10487569983] [INFO] [kernel::memory]   [44] 0x796ce000 - 0x796d8000 (Other)
[10487914338] [INFO] [kernel::memory]   [45] 0x796d8000 - 0x796d9000 (Reserved)
[10488260838] [INFO] [kernel::memory]   [46] 0x796d9000 - 0x796dd000 (Other)
[10488594468] [INFO] [kernel::memory]   [47] 0x796dd000 - 0x796de000 (Reserved)
[10488944169] [INFO] [kernel::memory]   [48] 0x796de000 - 0x796e0000 (Other)
[10489285785] [INFO] [kernel::memory]   [49] 0x796e0000 - 0x796e1000 (Reserved)
[10489642416] [INFO] [kernel::memory]   [50] 0x796e1000 - 0x796e3000 (Other)
[10489984131] [INFO] [kernel::memory]   [51] 0x796e3000 - 0x796e4000 (Reserved)
[10490348088] [INFO] [kernel::memory]   [52] 0x796e4000 - 0x796e8000 (Other)
[10490682279] [INFO] [kernel::memory]   [53] 0x796e8000 - 0x79757000 (Other)
[10491017460] [INFO] [kernel::memory]   [54] 0x79757000 - 0x7990d000 (Other)
[10491349869] [INFO] [kernel::memory]   [55] 0x7990d000 - 0x7a16c000 (Reserved)
[10491696237] [INFO] [kernel::memory]   [56] 0x7a16c000 - 0x7bb6c000 (Usable)
[10492034553] [INFO] [kernel::memory]   [57] 0x7bb6c000 - 0x7bb8d000 (Reserved)
[10492387092] [INFO] [kernel::memory]   [58] 0x7bb8d000 - 0x7bb91000 (Other)
[10492726926] [INFO] [kernel::memory]   [59] 0x7bb91000 - 0x7bb92000 (Reserved)
[10493106723] [INFO] [kernel::memory]   [60] 0x7bb92000 - 0x7bb94000 (Other)
[10497227829] [INFO] [kernel::memory]   [61] 0x7bb94000 - 0x7bb95000 (Reserved)
[10497581919] [INFO] [kernel::memory]   [62] 0x7bb95000 - 0x7bb97000 (Other)
[10497915252] [INFO] [kernel::memory]   [63] 0x7bb97000 - 0x7bb98000 (Reserved)
[10498511661] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[10723211631] [CONTRACT] [kernel::memory] Frame allocator initialized with 495746 free frames
[10730039628] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[10734729918] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[10735816179] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[10736483934] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[10742824521] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[10743272397] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[10745927610] [INFO] [bran::arch] IOAPIC: Registers initialized
[10746943218] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[10748374395] [INFO] [bran::arch] IOAPIC: All pins masked
[10749657006] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[10750242195] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[10750880679] [INFO] [bran::arch] IOAPIC: Init complete
[10751460324] [CONTRACT] [kernel] Initializing global allocator...
[11094610527] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[11095289238] [CONTRACT] [kernel] Initializing SIMD...
[11096635242] [CONTRACT] [kernel] Initializing tasking...
[11101299231] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[11102847987] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[11103400572] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[11109014103] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[11109456897] [INFO] [kernel::task::scheduler]   Initializing boot task...
[11110183656] [INFO] [kernel::task::scheduler]   Creating boot task...
[11114495733] [INFO] [kernel::task::scheduler]   Creating idle task...
[11119181931] [INFO] [kernel::task::scheduler]   Boot task initialized
[11119613703] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[11120430288] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[11126070285] [INFO] [kernel::root] Spawning Root service...
[11133567192] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[11142368391] [INFO] [kernel::root::service] ROOT: started once
[11974962813] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[11975780058] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[12014435730] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[12030893919] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[12056174757] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[12087008967] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[12088721766] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[12125793009] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[12147775629] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[12155115159] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[12157655367] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[12180779061] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[12183687978] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[12184700253] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[12187929666] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[12190582008] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[12191830398] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12200638098] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12211897005] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[12212915979] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[12215247033] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[12216101139] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[12223303719] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[12224001438] [CONTRACT] [kernel] Spawning init process...
[12225505182] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[12260212239] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62347100 ticks/sec), init_cnt=623471 for 100Hz
[12261814983] [CONTRACT] [kernel] Entering scheduler loop.
[12270965289] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[12275511501] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367676144 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563776488
[12285608082] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[12287124432] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[12287889570] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[12288607188] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[12308605518] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[12312379497] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[12315847236] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[12317131728] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[12323486340] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[12327341367] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[12328226262] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[12331587642] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[12332360733] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[12333316083] [INFO] [sprout::devtree] SPROUT: build() called
[12334261896] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[12342504537] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[12344114508] [INFO] [sprout] SPROUT: About to create Supervisor...
[12345769260] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[12346940298] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[12347698341] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[12354996885] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[12404733990] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[12413904426] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[12419775555] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[12425802609] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[12431387067] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[12437232753] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[12443938518] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[12450242277] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[12456319326] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[12462877449] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[12466956942] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[12470918790] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[12474875622] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[12478534167] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[12482417541] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[12486883761] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[12490911279] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[12494538309] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[12498073797] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[12502112040] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[12505821273] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[12510897762] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[12517600260] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[12522421428] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[12528534777] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[12532746600] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[12537373530] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[12542255946] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[12546312669] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[12550127931] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[12553867722] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[12558667704] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[12562417197] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[12567008355] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[12571343136] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[12575445696] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[12579848127] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[12585640650] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[12591322227] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[12644986530] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[12651241812] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[12655863594] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[12659582727] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[12662309913] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[12677768400] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[12760210716] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[12761697696] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[12765190020] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[12767425638] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[12768528366] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[12769706070] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[12773776389] [INFO] [kernel::task::loader] Loading module: /boot/clock
[12774612345] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12776292837] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12782568216] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[12784522707] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[12786785055] [INFO] [kernel::task::loader] Segment: vaddr=204ee8 exec=false
[12787711233] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12799415871] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[12804472659] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[12806033856] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[12806799357] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12809046030] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12815188914] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[12816694242] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[12817883034] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[12818955270] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12829808409] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[12832229025] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[12833199390] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[12833705643] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12834839325] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12846426747] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[12847092621] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[12850992561] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[12851647776] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[12857318793] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[12858067827] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[12859496067] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[12859995885] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12860970903] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12864531669] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[12865393794] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[12867368811] [INFO] [kernel::task::loader] Segment: vaddr=204c88 exec=false
[12868346370] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12876335868] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[12877129122] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[12894383964] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12895747161] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367986928 RFLAGS_BEFORE=134 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563776488
[12899849292] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[12900869520] [INFO] [clock] starting clock publisher
[12910386918] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12911525286] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368009968 RFLAGS_BEFORE=134 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563776488
[12916234749] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ab8
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[12917770206] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368026352 RFLAGS_BEFORE=134 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563776488
[12920092284] [ERROR] [INGESTD] Starting...
[12923695422] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ae8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12924899130] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368053488 RFLAGS_BEFORE=134 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563776488
[12927513027] [INFO] [cambium] cambium starting (v3: catch-up then stream)...
[12933336174] [INFO] [clock] Clock thing created: 356
[12934011816] [INFO] [clock] Waiting for UI Root (Compositor)...
[12938743224] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[12939897960] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[12941042862] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[12942335307] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[12949742586] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[12960108249] [ERROR] [INGESTD] Watch active. Loop start.
[12967545492] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[12968798832] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[13182292464] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[13182997344] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[13184020245] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13186800297] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[13187391624] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13188588699] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[13189171116] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13195219554] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[13201529814] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[13202698740] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368106128 RFLAGS_BEFORE=134 CR3_BEFORE=54927360 fs_base=0 gs_base=18446744071563776488
[13206118464] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[13222617210] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[13231061877] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[13239717150] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[13240911981] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[13241429289] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[13242555579] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13245422850] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[13246154427] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13247628405] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[13248237387] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[13263944265] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044898
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[13265128536] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368129632 RFLAGS_BEFORE=134 CR3_BEFORE=55033856 fs_base=0 gs_base=18446744071563776488
[13268639934] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[13269495162] [INFO] [rtc_cmos] Starting... arg=db
[13270984683] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[13274052429] [INFO] [rtc_cmos] RTC: 2026-01-20 05:19:13 = 1768886353 unix_secs
[13275373221] [INFO] [kernel::time] System clock anchored: unix_secs=1768886353, mono_ns=6637391673, offset=1768886346362608327ns
[13276546239] [INFO] [rtc_cmos] System clock anchored
[13283901543] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[13285077036] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[13286386113] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[13287446040] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[13288730763] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[13289765379] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[13290290442] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13291299120] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13293561402] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[13294113888] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[13294964463] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[13295506125] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[13301329206] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[13302677091] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[13303239939] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13304144601] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13306896207] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[13307459352] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13308710877] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[13309251978] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[13315282893] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[13316863065] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[13317885966] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[13318381560] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13319302260] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13322211474] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[13322791383] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13323715680] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[13324243185] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13329758673] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[13351391130] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[13353553653] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0095f70
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[13354573419] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368169600 RFLAGS_BEFORE=130 CR3_BEFORE=55136256 fs_base=0 gs_base=18446744071563776488
[13358359806] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[13360238760] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[13361422767] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[13362261792] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[13367446950] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00978a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[13368705867] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368187088 RFLAGS_BEFORE=130 CR3_BEFORE=55230464 fs_base=0 gs_base=18446744071563776488
[13371909078] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[13372748334] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[13376211354] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00b9748
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[13377410937] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368212960 RFLAGS_BEFORE=130 CR3_BEFORE=55332864 fs_base=0 gs_base=18446744071563776488
[13382117958] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[13389148344] [INFO] [bristle] bristle: registered in graph as svc.Input (id=457)
[13397695278] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=440 backend=BootFB
[13398926046] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[13399448898] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13400545884] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13455272688] [INFO] [kernel::task::loader] Segment: vaddr=2615c0 exec=false
[13456202826] [INFO] [kernel::task::loader]   Overlap at 261000: merging perms to r=true w=false x=true
[13464266046] [INFO] [kernel::task::loader] Segment: vaddr=26d150 exec=false
[13464908457] [INFO] [kernel::task::loader]   Overlap at 26d000: merging perms to r=true w=true x=true
[13471553667] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[13472980290] [INFO] [kernel::task::loader] Loading module: /boot/echo
[13473501525] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13474574586] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13477381170] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[13478373513] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13479409350] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[13480001601] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13485983544] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[13486964238] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[13487746734] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[13498938024] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014c50
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1b8
[13500226641] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368250096 RFLAGS_BEFORE=134 CR3_BEFORE=55439360 fs_base=0 gs_base=18446744071563776488
[13503256965] [INFO] [bloom::logging] bloom: logging initialized
[13506707709] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ee60
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[13507906104] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368267552 RFLAGS_BEFORE=130 CR3_BEFORE=55975936 fs_base=0 gs_base=18446744071563776488
[13511819343] [INFO] [echo] echo: online (handle=12)
[13512592896] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[13519161150] [INFO] [bloom] bloom: [bloom] Bootstrapped via Bytespace ID=440
[13524330963] [INFO] [bloom] bloom: [bloom] starting (arg_req=1 arg_resp=4 bristle=10 font_svc=0)
[13534358145] [INFO] [bloom] bloom: [bloom] spawned wallpaper_loader thread (tid=15)
[13552196097] [INFO] [bloom] bloom: [bloom] spawned cursor_loader thread (tid=16)
[13560227934] [INFO] [bloom] bloom: [bloom] spawned font_loader thread (tid=17)
[13561548033] [INFO] [bloom] bloom: [bloom] discovering compositor target...
T:AEF0 [13567712829] [INFO] [bloom] bloom: [wallpaper_loader] thread started
[13574691669] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[13577023218] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[13579485579] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
T:AB20 [13584632358] [INFO] [bloom] bloom: [cursor_loader] thread started
T:A0B0 [13587045879] [INFO] [bloom] bloom: [font_loader] thread started
[13587848472] [INFO] [bloom] bloom: [font_loader] scanning boot modules for fonts...
[13594202457] [INFO] [bloom] bloom: [font_loader] found 42 boot modules
[13694057652] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/DSEG7Classic-Regular.ttf'
[13701741042] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=179 size=23272 name='/assets/fonts/DSEG7Classic-Regular.ttf'
[13731111570] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:35E0 [13736773644] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[13738033683] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[13741700148] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/Hack-Regular.ttf'
[13747080996] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[13748137227] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[13770784170] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[13774954479] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[13783004895] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=182 size=309408 name='/assets/fonts/Hack-Regular.ttf'
[13787535960] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[13789919550] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSans-Regular.ttf'
[13794644622] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[13795467312] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[14360563437] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[14362043421] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[14366945340] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=185 size=569208 name='/assets/fonts/NotoSans-Regular.ttf'
[14372644110] [INFO] [bloom] bloom: [wallpaper_loader] searching for wallpaper...
[14373696744] [INFO] [bloom] bloom: [wallpaper_loader] trying: /assets/wallpapers/clouds.bmp
[14377972620] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[14380488936] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol-Regular.ttf'
[14386857375] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[14387657427] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[14702154852] [INFO] [bloom] bloom: [cursor_loader] searching for cursor...
[14703231477] [INFO] [bloom] bloom: [cursor_loader] trying: /assets/cursors/plain/Normal.cur
[15022045203] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=188 size=258156 name='/assets/fonts/NotoSansSymbol-Regular.ttf'
[15977077578] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[16631594793] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=191 size=656852 name='/assets/fonts/NotoSansSymbol2-Regular.ttf'
[16962424017] [INFO] [bloom::compositor] bloom: compositor bytespace 376 (1280x720 stride=5120 format=1)
[16968573600] [INFO] [ps2_mouse] ps2_mouse: drained 0x38
[17237725395] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[17239135485] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[17301915543] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (258156 bytes)
[17304424962] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSerif-Regular.ttf'
[17310898044] [INFO] [bloom::asset] [asset_bank] mapped at 0x10463000
[17311739049] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[17620290600] [INFO] [bloom::compositor] bloom: display backend: BootFB
[17946426300] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=194 size=616196 name='/assets/fonts/NotoSerif-Regular.ttf'
[18276780522] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[18283001319] [INFO] [ps2_mouse] ps2_mouse: init done
[18283708047] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[18284573637] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18285254526] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[18610710195] [INFO] [bloom] bloom: [bloom] compositor target: 1280x720 @ 0x104a3000 backend=BootFB
[18612016269] [INFO] [bloom] bloom: [bloom] presenter: driver (req=1 resp=4)
[18613627626] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[18931533852] [INFO] [bloom] bloom: [font_loader] immediate scan complete, entering watch loop
[18934675914] [INFO] [stem::ui] UiBuilder: created root 546
[18935511738] [INFO] [bloom] bloom: [bloom] created UI root node: 546
[18945029268] [INFO] [display_bootfb] display_bootfb: bound bytespace 376
[19261689513] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd5b0
[19262618463] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[19263321825] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19264210086] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[19274894133] [INFO] [clock] Found UI Root: 546 (attempt 4)
[19590336876] [INFO] [bloom] bloom: [bloom] ui watch opened UNFILTERED (userspace filter pred=UI_TEXT id=519)
[20080929000] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[20082677010] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[20083568439] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 191 (656852 bytes)
[20095106493] [INFO] [bloom::asset] [asset_bank] mapped at 0x10bab000
[20095815102] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[21891175152] [INFO] [echo] KeyUp LAlt

```
</details>
