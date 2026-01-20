# ✅ Scenario: Boot produces logs, graphics, and a live desktop

> Last run: 2026-01-19 21:32:34

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 4010ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | Then I should see log messages on the terminal | ✅ | 423ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And each log message should include a monotonically increasing timestamp | ✅ | 424ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And I should see the system clock tick for several seconds in the serial console | ✅ | 1745ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |
| 5 | And I should see the wallpaper on the screen within 15 seconds | ✅ | 901ms | <a href="./05/after.png"><img src="./05/after.png" width="150" /></a> [📜](./05/serial.log) [💾](./05/registers.txt) |
| 6 | And I should see a cursor centered on the screen | ✅ | 4314ms | <a href="./06/after.png"><img src="./06/after.png" width="150" /></a> [📜](./06/serial.log) [💾](./06/registers.txt) |
| 7 | And I should see the text "thing-os" in the top-left corner of the screen | ✅ | 467ms | <a href="./07/after.png"><img src="./07/after.png" width="150" /></a> [📜](./07/serial.log) [💾](./07/registers.txt) |
| 8 | And I should see frame count information in the top-left corner of the screen | ✅ | 459ms | <a href="./08/after.png"><img src="./08/after.png" width="150" /></a> [📜](./08/serial.log) [💾](./08/registers.txt) |
| 9 | And I should see a clock window displaying a ticking clock | ✅ | 899ms | <a href="./09/after.png"><img src="./09/after.png" width="150" /></a> [📜](./09/serial.log) [💾](./09/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11526637254] [CONTRACT] [kernel] thing-os kernel starting...
[11537563422] [INFO] [kernel::memory] Memory map has 64 entries
[11541366837] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[11544291990] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[11544908232] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[11545512627] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[11546103492] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[11546724420] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[11547264399] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[11547854769] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[11548521534] [INFO] [kernel::memory]   [8] 0x1780000 - 0x7866a000 (Usable)
[11549143650] [INFO] [kernel::memory]   [9] 0x7866a000 - 0x786cc000 (Reserved)
[11549743260] [INFO] [kernel::memory]   [10] 0x786cc000 - 0x7884e000 (Other)
[11550326403] [INFO] [kernel::memory]   [11] 0x7884e000 - 0x7884f000 (Reserved)
[11550726396] [INFO] [kernel::memory]   [12] 0x7884f000 - 0x788e6000 (Other)
[11551101375] [INFO] [kernel::memory]   [13] 0x788e6000 - 0x788e7000 (Reserved)
[11551591425] [INFO] [kernel::memory]   [14] 0x788e7000 - 0x78988000 (Other)
[11552009106] [INFO] [kernel::memory]   [15] 0x78988000 - 0x78989000 (Reserved)
[11552509188] [INFO] [kernel::memory]   [16] 0x78989000 - 0x789c9000 (Other)
[11552962410] [INFO] [kernel::memory]   [17] 0x789c9000 - 0x789ca000 (Reserved)
[11553482325] [INFO] [kernel::memory]   [18] 0x789ca000 - 0x78a55000 (Other)
[11553892020] [INFO] [kernel::memory]   [19] 0x78a55000 - 0x78a56000 (Reserved)
[11554294554] [INFO] [kernel::memory]   [20] 0x78a56000 - 0x78aa2000 (Other)
[11554702137] [INFO] [kernel::memory]   [21] 0x78aa2000 - 0x78aa3000 (Reserved)
[11555076192] [INFO] [kernel::memory]   [22] 0x78aa3000 - 0x78aa9000 (Other)
[11555514960] [INFO] [kernel::memory]   [23] 0x78aa9000 - 0x78aaa000 (Reserved)
[11556081570] [INFO] [kernel::memory]   [24] 0x78aaa000 - 0x78f2b000 (Other)
[11556652338] [INFO] [kernel::memory]   [25] 0x78f2b000 - 0x78f2c000 (Reserved)
[11557025040] [INFO] [kernel::memory]   [26] 0x78f2c000 - 0x793ad000 (Other)
[11557465326] [INFO] [kernel::memory]   [27] 0x793ad000 - 0x793ae000 (Reserved)
[11557959006] [INFO] [kernel::memory]   [28] 0x793ae000 - 0x796af000 (Other)
[11558573367] [INFO] [kernel::memory]   [29] 0x796af000 - 0x796b0000 (Reserved)
[11559230331] [INFO] [kernel::memory]   [30] 0x796b0000 - 0x796b9000 (Other)
[11559841887] [INFO] [kernel::memory]   [31] 0x796b9000 - 0x796ba000 (Reserved)
[11560469646] [INFO] [kernel::memory]   [32] 0x796ba000 - 0x796be000 (Other)
[11561078364] [INFO] [kernel::memory]   [33] 0x796be000 - 0x796bf000 (Reserved)
[11561715231] [INFO] [kernel::memory]   [34] 0x796bf000 - 0x796c1000 (Other)
[11562341175] [INFO] [kernel::memory]   [35] 0x796c1000 - 0x796c2000 (Reserved)
[11577769698] [INFO] [kernel::memory]   [36] 0x796c2000 - 0x796c4000 (Other)
[11578323768] [INFO] [kernel::memory]   [37] 0x796c4000 - 0x796c5000 (Reserved)
[11579070921] [INFO] [kernel::memory]   [38] 0x796c5000 - 0x796c7000 (Other)
[11579653041] [INFO] [kernel::memory]   [39] 0x796c7000 - 0x796c8000 (Reserved)
[11580250539] [INFO] [kernel::memory]   [40] 0x796c8000 - 0x796ca000 (Other)
[11580820548] [INFO] [kernel::memory]   [41] 0x796ca000 - 0x796cb000 (Reserved)
[11581371714] [INFO] [kernel::memory]   [42] 0x796cb000 - 0x796cd000 (Other)
[11581928424] [INFO] [kernel::memory]   [43] 0x796cd000 - 0x796ce000 (Reserved)
[11582475300] [INFO] [kernel::memory]   [44] 0x796ce000 - 0x796d8000 (Other)
[11583008448] [INFO] [kernel::memory]   [45] 0x796d8000 - 0x796d9000 (Reserved)
[11583598455] [INFO] [kernel::memory]   [46] 0x796d9000 - 0x796dd000 (Other)
[11584251756] [INFO] [kernel::memory]   [47] 0x796dd000 - 0x796de000 (Reserved)
[11584925187] [INFO] [kernel::memory]   [48] 0x796de000 - 0x796e0000 (Other)
[11585586705] [INFO] [kernel::memory]   [49] 0x796e0000 - 0x796e1000 (Reserved)
[11586050916] [INFO] [kernel::memory]   [50] 0x796e1000 - 0x796e3000 (Other)
[11586529185] [INFO] [kernel::memory]   [51] 0x796e3000 - 0x796e4000 (Reserved)
[11586944226] [INFO] [kernel::memory]   [52] 0x796e4000 - 0x796e8000 (Other)
[11587309008] [INFO] [kernel::memory]   [53] 0x796e8000 - 0x79757000 (Other)
[11587677321] [INFO] [kernel::memory]   [54] 0x79757000 - 0x7990d000 (Other)
[11588044677] [INFO] [kernel::memory]   [55] 0x7990d000 - 0x7a16c000 (Reserved)
[11588465988] [INFO] [kernel::memory]   [56] 0x7a16c000 - 0x7bb6c000 (Usable)
[11588845422] [INFO] [kernel::memory]   [57] 0x7bb6c000 - 0x7bb8d000 (Reserved)
[11589215352] [INFO] [kernel::memory]   [58] 0x7bb8d000 - 0x7bb91000 (Other)
[11589574194] [INFO] [kernel::memory]   [59] 0x7bb91000 - 0x7bb92000 (Reserved)
[11589953430] [INFO] [kernel::memory]   [60] 0x7bb92000 - 0x7bb94000 (Other)
[11590364841] [INFO] [kernel::memory]   [61] 0x7bb94000 - 0x7bb95000 (Reserved)
[11590798626] [INFO] [kernel::memory]   [62] 0x7bb95000 - 0x7bb97000 (Other)
[11591179677] [INFO] [kernel::memory]   [63] 0x7bb97000 - 0x7bb98000 (Reserved)
[11591937720] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[11901921273] [CONTRACT] [kernel::memory] Frame allocator initialized with 495746 free frames
[11909864901] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[11915804736] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[11917285050] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[11918395764] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[11925286461] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[11926034406] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11929401858] [INFO] [bran::arch] IOAPIC: Registers initialized
[11930872800] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[11933020143] [INFO] [bran::arch] IOAPIC: All pins masked
[11934913914] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11935837155] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11936623347] [INFO] [bran::arch] IOAPIC: Init complete
[11937484152] [CONTRACT] [kernel] Initializing global allocator...
[12337060659] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[12337805172] [CONTRACT] [kernel] Initializing SIMD...
[12339195198] [CONTRACT] [kernel] Initializing tasking...
[12344271852] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[12346101372] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[12346720122] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[12352586928] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[12353027445] [INFO] [kernel::task::scheduler]   Initializing boot task...
[12353783838] [INFO] [kernel::task::scheduler]   Creating boot task...
[12358474590] [INFO] [kernel::task::scheduler]   Creating idle task...
[12363814980] [INFO] [kernel::task::scheduler]   Boot task initialized
[12364247544] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[12365123529] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[12371423559] [INFO] [kernel::root] Spawning Root service...
[12379612971] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[12389205147] [INFO] [kernel::root::service] ROOT: started once
[13346034075] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[13346921445] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[13403658444] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[13425247803] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[13454765973] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[13488007896] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[13489787586] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[13527416001] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[13552506561] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[13560103062] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[13562808831] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[13587217743] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[13590047526] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[13591094616] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[13594821867] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[13597395108] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[13598217930] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13609200132] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13628830776] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[13630622280] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[13633675242] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[13635038637] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[13644955203] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[13645842936] [CONTRACT] [kernel] Spawning init process...
[13647629259] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[13682489667] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62219400 ticks/sec), init_cnt=622194 for 100Hz
[13684459371] [CONTRACT] [kernel] Entering scheduler loop.
[13701385863] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[13706684772] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367676144 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563776488
[13719811248] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[13730140710] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[13731032700] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[13731771141] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[13747692882] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[13752386406] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[13755834873] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[13756791972] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[13761747186] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[13764266967] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[13765082991] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[13768470243] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[13769665536] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[13770711603] [INFO] [sprout::devtree] SPROUT: build() called
[13771415295] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[13776281079] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[13777077435] [INFO] [sprout] SPROUT: About to create Supervisor...
[13777747170] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[13778635893] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[13779342555] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[13784411751] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[13820363271] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[13825628421] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[13829092728] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[13832356956] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[13834709229] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[13837667019] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[13841259168] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[13844459871] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[13847810988] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[13850962323] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[13854106332] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[13857649278] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[13861258323] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[13864201923] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[13867174959] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[13870221387] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[13873510728] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[13877361564] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[13880723274] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[13884619518] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[13887816393] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[13891337097] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[13894345674] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[13897371213] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[13900554855] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[13903584948] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[13906744764] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[13909887948] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[13913391063] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[13916470095] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[13919539062] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[13922694819] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[13925729004] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[13928814009] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[13931983725] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[13935390612] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[13938750210] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[13942053576] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[13945389051] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[13948466994] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[13951549557] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[13954610109] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[13957732998] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[13960001847] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[13972820598] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[14046843492] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[14047824978] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[14049763332] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[14050838241] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[14051625324] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[14052507348] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[14055149163] [INFO] [kernel::task::loader] Loading module: /boot/clock
[14055722010] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14056843449] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14060915550] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[14061708408] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[14063526873] [INFO] [kernel::task::loader] Segment: vaddr=204ee8 exec=false
[14064167898] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[14072094828] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[14075387535] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[14076726807] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[14077306353] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14078827719] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14082927144] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[14083514082] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[14084336970] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[14084895627] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[14091045012] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[14092307856] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[14093302014] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[14093820378] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14094824172] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14105945766] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[14106618933] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[14111203260] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[14111986515] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[14118324363] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[14119103790] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[14120580177] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[14121106230] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14122167840] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14126109855] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[14126783451] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[14128356594] [INFO] [kernel::task::loader] Segment: vaddr=204c88 exec=false
[14128980030] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[14135387343] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[14136372558] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[14154022740] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[14155630038] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367986928 RFLAGS_BEFORE=134 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563776488
[14159790711] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[14160893670] [INFO] [clock] starting clock publisher
[14171020347] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[14172235638] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368009968 RFLAGS_BEFORE=134 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563776488
[14177415978] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ab8
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[14178660474] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368026352 RFLAGS_BEFORE=134 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563776488
[14181041721] [ERROR] [INGESTD] Starting...
[14185072671] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ae8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[14186271462] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368053488 RFLAGS_BEFORE=134 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563776488
[14189025906] [INFO] [cambium] cambium starting (v3: catch-up then stream)...
[14195332074] [INFO] [clock] Clock thing created: 356
[14196050616] [INFO] [clock] Waiting for UI Root (Compositor)...
[14201037741] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[14202423081] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[14203503765] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[14204903427] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[14227873176] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[14236795320] [ERROR] [INGESTD] Watch active. Loop start.
[14245567314] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[14246974731] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[14479619880] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[14480468640] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[14481792930] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14485157346] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[14485833978] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14487033825] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[14487835890] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[14494710483] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[14501510529] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[14502871911] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368106080 RFLAGS_BEFORE=134 CR3_BEFORE=54927360 fs_base=0 gs_base=18446744071563776488
[14507009517] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[14524793745] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[14533841883] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[14544058716] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[14545447356] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[14546019873] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[14547272388] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14550539289] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[14551149822] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14552499324] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[14553114906] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[14559349398] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[14560526475] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[14562008076] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[14563435821] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[14565160071] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[14566338435] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[14566981242] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14568167922] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14570653977] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[14571290910] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[14572298235] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[14572949325] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[14579824281] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[14581255689] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[14581819428] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14582913708] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14585965119] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[14586566742] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14587891329] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[14588486715] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[14594977320] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[14596856439] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[14598002595] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[14598573033] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14599645929] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14602736445] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[14603380836] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14604436935] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[14605039449] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[14611573515] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[14628427242] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044678
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[14629981344] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368129744 RFLAGS_BEFORE=130 CR3_BEFORE=55033856 fs_base=0 gs_base=18446744071563776488
[14634388890] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[14635433472] [INFO] [rtc_cmos] Starting... arg=db
[14637015327] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[14640329022] [INFO] [rtc_cmos] RTC: 2026-01-20 05:32:41 = 1768887161 unix_secs
[14641571340] [INFO] [kernel::time] System clock anchored: unix_secs=1768887161, mono_ns=7320534936, offset=1768887153679465064ns
[14642966019] [INFO] [rtc_cmos] System clock anchored
[14660311677] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[14662625175] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004b958
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[14663701008] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368164272 RFLAGS_BEFORE=130 CR3_BEFORE=55136256 fs_base=0 gs_base=18446744071563776488
[14669459772] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[14671462674] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[14673219627] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[14674088781] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[14680158570] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ee60
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[14681612451] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368181680 RFLAGS_BEFORE=130 CR3_BEFORE=55230464 fs_base=0 gs_base=18446744071563776488
[14685335610] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[14686202586] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[14690022930] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00978a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[14691297225] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368207552 RFLAGS_BEFORE=134 CR3_BEFORE=55332864 fs_base=0 gs_base=18446744071563776488
[14694831195] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[14703534516] [INFO] [bristle] bristle: registered in graph as svc.Input (id=457)
[14712596250] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=431 backend=BootFB
[14713812168] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[14714380725] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14715513813] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14775950310] [INFO] [kernel::task::loader] Segment: vaddr=2615d0 exec=false
[14777190087] [INFO] [kernel::task::loader]   Overlap at 261000: merging perms to r=true w=false x=true
[14785470678] [INFO] [kernel::task::loader] Segment: vaddr=26d1a0 exec=false
[14786283567] [INFO] [kernel::task::loader]   Overlap at 26d000: merging perms to r=true w=true x=true
[14793620622] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[14795420112] [INFO] [kernel::task::loader] Loading module: /boot/echo
[14796019095] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14797186206] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14800410405] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[14801096376] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14802215736] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[14802868476] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[14847058446] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014c50
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1af
[14848618587] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368250160 RFLAGS_BEFORE=134 CR3_BEFORE=55439360 fs_base=0 gs_base=18446744071563776488
[14851797345] [INFO] [bloom::logging] bloom: logging initialized
[14855658642] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ba00
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[14857187730] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368267696 RFLAGS_BEFORE=130 CR3_BEFORE=55975936 fs_base=0 gs_base=18446744071563776488
[14860625439] [INFO] [echo] echo: online (handle=12)
[14861430276] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[14867551281] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[14869548111] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[14870426670] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[14876561238] [INFO] [bloom] bloom: [bloom] Bootstrapped via Bytespace ID=431
[14883521004] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[14884514337] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[14885579775] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
[14890200600] [INFO] [bloom] bloom: [bloom] starting (arg_req=1 arg_resp=4 bristle=10 font_svc=0)
[14901071262] [INFO] [bloom] bloom: [bloom] spawned wallpaper_loader thread (tid=15)
[14907444717] [INFO] [bloom] bloom: [bloom] spawned cursor_loader thread (tid=16)
[14913396597] [INFO] [bloom] bloom: [bloom] spawned font_loader thread (tid=17)
[14914166916] [INFO] [bloom] bloom: [bloom] discovering compositor target...
T:AEF0 [14920202946] [INFO] [bloom] bloom: [wallpaper_loader] thread started
T:AB20 [14922164466] [INFO] [bloom] bloom: [cursor_loader] thread started
T:A0B0 [14924582970] [INFO] [bloom] bloom: [font_loader] thread started
[14925361737] [INFO] [bloom] bloom: [font_loader] scanning boot modules for fonts...
[14931904416] [INFO] [bloom] bloom: [font_loader] found 42 boot modules
[15023334183] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/DSEG7Classic-Regular.ttf'
[15032041629] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=179 size=23272 name='/assets/fonts/DSEG7Classic-Regular.ttf'
[15061595109] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:35F0 [15067498941] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[15068785809] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[15071979978] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/Hack-Regular.ttf'
[15076714950] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[15077614365] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[15102921405] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[15105968394] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[15118990095] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=182 size=309408 name='/assets/fonts/Hack-Regular.ttf'
[15128626293] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[15132616818] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSans-Regular.ttf'
[15139683834] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[15140571765] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[15741024783] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[15742660164] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[15746283729] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=185 size=569208 name='/assets/fonts/NotoSans-Regular.ttf'
[15752260755] [INFO] [bloom] bloom: [wallpaper_loader] searching for wallpaper...
[15753254649] [INFO] [bloom] bloom: [wallpaper_loader] trying: /assets/wallpapers/clouds.bmp
[15757897320] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[15760888143] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol-Regular.ttf'
[15767779203] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[15768911499] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[16145615343] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=188 size=258156 name='/assets/fonts/NotoSansSymbol-Regular.ttf'
[16152693513] [INFO] [bloom] bloom: [cursor_loader] searching for cursor...
[16153720803] [INFO] [bloom] bloom: [cursor_loader] trying: /assets/cursors/plain/Normal.cur
[16492918035] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[17139101661] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=191 size=656852 name='/assets/fonts/NotoSansSymbol2-Regular.ttf'
[17454793059] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSerif-Regular.ttf'
[17458929048] [INFO] [bloom::compositor] bloom: compositor bytespace 376 (1280x720 stride=5120 format=1)
[18111866751] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=194 size=616196 name='/assets/fonts/NotoSerif-Regular.ttf'
[18136895205] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[18138197550] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[18139164516] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (258156 bytes)
[18143841045] [INFO] [bloom] bloom: [font_loader] immediate scan complete, entering watch loop
[18148051548] [INFO] [bloom::compositor] bloom: display backend: BootFB
[18155594259] [INFO] [bloom::asset] [asset_bank] mapped at 0x10463000
[18157028505] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[18475658883] [INFO] [ps2_mouse] ps2_mouse: init done
[18477415572] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[18479163087] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18480586971] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[18489607488] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[18840287070] [INFO] [bloom] bloom: [bloom] compositor target: 1280x720 @ 0x104a3000 backend=BootFB
[18841535394] [INFO] [bloom] bloom: [bloom] presenter: driver (req=1 resp=4)
[18843057651] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[19161749013] [INFO] [stem::ui] UiBuilder: created root 546
[19162970442] [INFO] [bloom] bloom: [bloom] created UI root node: 546
[19176065634] [INFO] [display_bootfb] display_bootfb: bound bytespace 376
[19493359578] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd5a0
[19494668325] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[19495772571] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19497166227] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[19829343864] [INFO] [bloom] bloom: [bloom] ui watch opened UNFILTERED (userspace filter pred=UI_TEXT id=521)
[19989876720] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[19991720199] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[19992956544] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 191 (656852 bytes)
[20005037085] [INFO] [clock] Found UI Root: 546 (attempt 4)
[20010030150] [INFO] [bloom::asset] [asset_bank] mapped at 0x10bab000
[20011748328] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[22471107813] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c1b40
[22472299641] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[22473074778] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22473874302] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[22806044283] [INFO] [bloom] bloom: [font_loader] watch opened (id=570)
[24617351814] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[24619353330] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol2-Regular.ttf' in slot 4
[24620377947] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 194 (616196 bytes)
[24633750273] [INFO] [bloom::asset] [asset_bank] mapped at 0x10c4c000
[24635345229] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSerif-Regular.ttf'...
[25101206823] [INFO] [bloom] bloom: [cursor_loader] SUCCESS: found candidate '/assets/cursors/plain/Normal.cur', enqueuing load
[25102678722] [INFO] [bloom] bloom: [cursor_loader] thread done, sleeping forever
[26091161118] [INFO] [bloom] bloom: [wallpaper_loader] SUCCESS: found candidate '/assets/wallpapers/clouds.bmp', enqueuing load
[26092667007] [INFO] [bloom] bloom: [wallpaper_loader] thread done, sleeping forever
[29036087322] [INFO] [cambium] Found 1 bindings
[29689592955] [INFO] [clock] Binding created: 590 (source=356 target=575)
[29690605494] [INFO] [clock] CLOCK: Entering main loop, publishing to thing_id=356
[29693410758] [INFO] [clock] unix=1768887168 utc=2026-01-20 05:32:48 mono_ns=14845896813
[30052143561] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fedd0
[30053066373] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[30054039279] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[30054896784] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x4 kind=0 pred=0 subj_lo=356
[30383081916] [INFO] [cambium] Opened watch 605 for source 356 (binding 590, start_seq=0)
[30418418580] [INFO] [cambium] CATCH-UP: Draining 1 watches for historical events...
[31067500134] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:32:48' tick=14845896813
[33034769625] [INFO] [cambium] cambium: drain complete payloads=0 overflows=7 last_seq=none
[33035775036] [INFO] [cambium] Entering event loop with 1 bindings (v3)
[34347462798] [INFO] [clock] unix=1768887170 utc=2026-01-20 05:32:50 mono_ns=17173522740
[35658120366] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:32:50' tick=17173522740
[36857859951] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[36859183515] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSerif-Regular.ttf' in slot 5
[36860099067] [INFO] [bloom::asset] [asset_bank] load_cursor_immediate: /assets/cursors/plain/Normal.cur
[36974091000] [INFO] [bloom::asset] [asset_bank] mapping bytespace 152 (4286 bytes)
[36979672818] [INFO] [bloom::asset] [asset_bank] mapped to 0x10ce3000
[36980501745] [INFO] [bloom::asset] [asset_bank] checking CUR header: len=4286
[36981280578] [INFO] [bloom::asset] [asset_bank] valid CUR header detected
[36982291005] [INFO] [bloom::asset] [asset_bank] CUR: hotspot=(0, 0), img_size=4264, offset=22
[36983114784] [INFO] [bloom::asset] [asset_bank] decoding embedded DIB at offset 22...
[36984904935] [INFO] [bloom::asset] [asset_bank] SUCCESS: cursor DIB decoded 32x32
[36993899844] [INFO] [bloom::asset] [asset_bank] publish_cursor (pending)
[36994917267] [INFO] [bloom::asset] [asset_bank] cursor: Static frame 32x32 hotspot (0, 0)
[36995835624] [INFO] [bloom::asset] [asset_bank] load_wallpaper_immediate: /assets/wallpapers/clouds.bmp
[37119471708] [INFO] [bloom::asset] [asset_bank] mapping bytespace 170 (3145782 bytes)
[37133048370] [INFO] [bloom::asset] [asset_bank] mapped to 0x10ce5000
[37133919834] [INFO] [bloom::asset] [asset_bank] decoding BMP...
[37536051102] [INFO] [bloom::asset] [asset_bank] BMP decoded: 1024x1024
[38004988956] [INFO] [bloom::asset] [asset_bank] bytespace unmapped
[38005954008] [INFO] [bloom::asset] [asset_bank] publish_wallpaper (pending): 1024x1024
[39015801825] [INFO] [cambium] [cambium] write: binding_src=356 target=575 pred=ui.Text(521) val=599 seq=1141
[39021711894] [INFO] [cambium] Updated target 575 with value 599 (seq=1141)
[39040542222] [INFO] [cambium] Updated target 575 with value 617 (seq=1153)
[39622919523] [INFO] [clock] unix=1768887173 utc=2026-01-20 05:32:53 mono_ns=19811279317
[39640087014] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:32:53' tick=19811279317
[39699026928] [INFO] [cambium] Updated target 575 with value 648 (seq=1165)
[40809928797] [INFO] [bloom] bloom: [font_loader] drain complete: payloads=1045 overflows=30
[40818300864] [INFO] [bloom] bloom: [bloom] ui watch drained: 1049 batches
[40850599053] [INFO] [bloom] bloom: [bloom] entering transactional frame loop (acquire -> build -> present)
[40851865065] [INFO] [bloom] bloom: [bloom] reclaimer: budget=33554432 bytes
[40853114643] [INFO] [bloom::reclaimer] [reclaimer] +4194304 bytes (total: 4194304)
[40854162525] [INFO] [bloom::asset] [asset_bank] promoting wallpaper to gen=1 (4194304b)
[40855380918] [INFO] [bloom::reclaimer] [reclaimer] +4096 bytes (total: 4198400)
[40856201595] [INFO] [bloom::asset] [asset_bank] promoting cursor to gen=1 (4096b)
[40857539976] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4300800)
[40858625643] [INFO] [bloom::asset] [asset_bank] promoting font 'DSEG7Classic-Regular.ttf' to gen=1 (102400b) in slot 0
[40859791830] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4403200)
[40860631647] [INFO] [bloom::asset] [asset_bank] promoting font 'Hack-Regular.ttf' to gen=1 (102400b) in slot 1
[40861631415] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4505600)
[40862509545] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=1 (102400b) in slot 2
[40863425691] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4608000)
[40864296693] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol-Regular.ttf' to gen=1 (102400b) in slot 3
[40865427570] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4710400)
[40867065921] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol2-Regular.ttf' to gen=1 (102400b) in slot 4
[40868179638] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4812800)
[40869063543] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSerif-Regular.ttf' to gen=1 (102400b) in slot 5
[40871023248] [INFO] [bloom] bloom: [bloom] frame 1: wallpaper now visible (gen=1)
[40872438618] [INFO] [bloom] bloom: [bloom] frame 1: cursor now visible (gen=1)
[40876589523] [INFO] [bloom] bloom: [bloom] frame 1: fonts available (mode=legacy)
[41336822604] [INFO] [bloom::ui] bloom: [bloom][ui] ENTER_UI_BUILD dirty=true root_present=true windows_seen=3 nodes=10 (text=5 text_str=7) reason=full_build
[41618480409] [INFO] [bloom::ui] bloom: [bloom::ui] WARN: slow UI run: total=359.3ms (snap=219.1ms diff=1.4ms clone=1.9ms layout=92.4ms paint=43.1ms lower=1.2ms)
[41808995085] [INFO] [bloom] bloom: [CONTRACT] [bloom] First frame rendered - compositor ready
[41856679260] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 13.2ms
[41890806210] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 11.2ms
[41988778392] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[42073822263] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.4ms
[42087561252] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1175
[42864806424] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.9ms
[42902446719] [INFO] [clock] unix=1768887175 utc=2026-01-20 05:32:55 mono_ns=21450787380
[42933881793] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:32:55' tick=21450787380
[43008455457] [INFO] [cambium] [cambium] write: binding_src=356 target=575 pred=ui.Text(521) val=689 seq=1179
[43015992624] [INFO] [cambium] Updated target 575 with value 689 (seq=1179)
[43728004848] [INFO] [bloom::ui] bloom: [bloom::ui] WARN: slow UI run: total=422.8ms (snap=296.8ms diff=0.3ms clone=2.0ms layout=85.4ms paint=37.5ms lower=0.8ms)
[43773861648] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (2 rects) = 20.1ms
[43886604036] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1181
[44761018830] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.2ms
[45518544357] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 14.4ms
[45529696146] [INFO] [bloom::ui] bloom: [bloom][ui] ENTER_UI_BUILD dirty=false root_present=true windows_seen=3 reason=fast_path_cached
[45715520334] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 11.9ms
[45942974517] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.5ms
[46114836372] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 13.7ms
[46149028233] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.3ms
[46212444069] [INFO] [clock] unix=1768887176 utc=2026-01-20 05:32:56 mono_ns=23105988790
[46275127107] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:32:56' tick=23105988790
[46310732919] [INFO] [cambium] [cambium] write: binding_src=356 target=575 pred=ui.Text(521) val=705 seq=1182
[46316940054] [INFO] [cambium] Updated target 575 with value 705 (seq=1182)
[46320559758] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1184
[47025230208] [INFO] [bloom::ui] bloom: [bloom::ui] WARN: slow UI run: total=351.5ms (snap=227.6ms diff=0.1ms clone=1.9ms layout=81.6ms paint=39.6ms lower=0.7ms)
[47071851684] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (2 rects) = 20.0ms
[47544090495] [INFO] [bloom] bloom: [bloom][damage] rects=1 frame=53
[47726765130] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[48206784351] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[48335275362] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.4ms
[48400897776] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[48503759931] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 12.4ms
[48543608850] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.3ms
[48637179789] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 12.2ms
[48729449802] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[48796849992] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.6ms
[49092576060] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.3ms
[49544810865] [INFO] [bloom] bloom: [bloom][damage] rects=1 frame=93
[49695175431] [INFO] [clock] unix=1768887178 utc=2026-01-20 05:32:58 mono_ns=24847017921
[49741843899] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:32:58' tick=24847017921
[49881937908] [INFO] [cambium] [cambium] write: binding_src=356 target=575 pred=ui.Text(521) val=726 seq=1185
[49889859921] [INFO] [cambium] Updated target 575 with value 726 (seq=1185)
[49894763523] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1187
[50665442256] [INFO] [bloom::ui] bloom: [bloom::ui] WARN: slow UI run: total=384.5ms (snap=258.3ms diff=0.1ms clone=3.5ms layout=84.1ms paint=37.9ms lower=0.6ms)
[50710502865] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (2 rects) = 19.3ms
[50752654128] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 11.5ms
[50795379525] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.4ms
[50829573135] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.0ms
[50863854987] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.4ms
[50926573170] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.3ms
[50992779222] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[51026501559] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.3ms
[51090181626] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[51642732636] [INFO] [bloom] bloom: [bloom][damage] rects=1 frame=110
[51833909061] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.8ms
[51877579149] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.6ms
[51978282378] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[52173029667] [INFO] [bloom] bloom: [bloom] PERF: 120 frames avg: total=42.29ms build=26.79ms (ui=25.04ms) raster=14.45ms present=0.92ms input=0.14ms
[52532792961] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[52598921793] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.2ms
[52960709934] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[52999906410] [INFO] [clock] unix=1768887180 utc=2026-01-20 05:33:00 mono_ns=26499679057
[53068972803] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:33:00' tick=26499679057
[53134823214] [INFO] [cambium] [cambium] write: binding_src=356 target=575 pred=ui.Text(521) val=751 seq=1188
[53140850730] [INFO] [cambium] Updated target 575 with value 751 (seq=1188)
[53167323099] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1190
[53948728779] [INFO] [bloom::ui] bloom: [bloom::ui] WARN: slow UI run: total=389.9ms (snap=253.5ms diff=0.1ms clone=3.3ms layout=90.2ms paint=42.1ms lower=0.6ms)
[53993278086] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (2 rects) = 18.2ms
[54369850854] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms

```
</details>
