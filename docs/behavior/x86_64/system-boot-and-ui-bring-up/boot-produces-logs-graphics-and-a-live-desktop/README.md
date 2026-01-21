# ✅ Scenario: Boot produces logs, graphics, and a live desktop

> Last run: 2026-01-20 17:49:56

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 3545ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | Then I should see log messages on the terminal | ✅ | 384ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And each log message should include a monotonically increasing timestamp | ✅ | 386ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And I should see the system clock tick for several seconds in the serial console | ✅ | 1836ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |
| 5 | And I should see the wallpaper on the screen within 15 seconds | ✅ | 6487ms | <a href="./05/after.png"><img src="./05/after.png" width="150" /></a> [📜](./05/serial.log) [💾](./05/registers.txt) |
| 6 | And I should see a cursor centered on the screen | ✅ | 1004ms | <a href="./06/after.png"><img src="./06/after.png" width="150" /></a> [📜](./06/serial.log) [💾](./06/registers.txt) |
| 7 | And I should see the text "thing-os" in the top-left corner of the screen | ✅ | 445ms | <a href="./07/after.png"><img src="./07/after.png" width="150" /></a> [📜](./07/serial.log) [💾](./07/registers.txt) |
| 8 | And I should see frame count information in the top-left corner of the screen | ✅ | 437ms | <a href="./08/after.png"><img src="./08/after.png" width="150" /></a> [📜](./08/serial.log) [💾](./08/registers.txt) |
| 9 | And I should see a clock window displaying a ticking clock | ✅ | 1002ms | <a href="./09/after.png"><img src="./09/after.png" width="150" /></a> [📜](./09/serial.log) [💾](./09/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[10055205204] [CONTRACT] [kernel] thing-os kernel starting...
[10064292711] [INFO] [kernel::memory] Memory map has 64 entries
[10066320561] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[10066909314] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[10067228457] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[10067580171] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[10067894067] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[10068636765] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[10069005837] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[10069320987] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[10069679367] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78659000 (Usable)
[10070011710] [INFO] [kernel::memory]   [9] 0x78659000 - 0x786bb000 (Reserved)
[10070355834] [INFO] [kernel::memory]   [10] 0x786bb000 - 0x7883d000 (Other)
[10070690685] [INFO] [kernel::memory]   [11] 0x7883d000 - 0x7883e000 (Reserved)
[10071038208] [INFO] [kernel::memory]   [12] 0x7883e000 - 0x788d5000 (Other)
[10071404211] [INFO] [kernel::memory]   [13] 0x788d5000 - 0x788d6000 (Reserved)
[10071752526] [INFO] [kernel::memory]   [14] 0x788d6000 - 0x78977000 (Other)
[10072087245] [INFO] [kernel::memory]   [15] 0x78977000 - 0x78978000 (Reserved)
[10072433745] [INFO] [kernel::memory]   [16] 0x78978000 - 0x789b8000 (Other)
[10072769322] [INFO] [kernel::memory]   [17] 0x789b8000 - 0x789b9000 (Reserved)
[10073114865] [INFO] [kernel::memory]   [18] 0x789b9000 - 0x78a44000 (Other)
[10073450145] [INFO] [kernel::memory]   [19] 0x78a44000 - 0x78a45000 (Reserved)
[10073797305] [INFO] [kernel::memory]   [20] 0x78a45000 - 0x78a91000 (Other)
[10074132486] [INFO] [kernel::memory]   [21] 0x78a91000 - 0x78a92000 (Reserved)
[10074477534] [INFO] [kernel::memory]   [22] 0x78a92000 - 0x78a98000 (Other)
[10074837003] [INFO] [kernel::memory]   [23] 0x78a98000 - 0x78a99000 (Reserved)
[10075180236] [INFO] [kernel::memory]   [24] 0x78a99000 - 0x78f1a000 (Other)
[10075511028] [INFO] [kernel::memory]   [25] 0x78f1a000 - 0x78f1b000 (Reserved)
[10075854591] [INFO] [kernel::memory]   [26] 0x78f1b000 - 0x7939c000 (Other)
[10076185482] [INFO] [kernel::memory]   [27] 0x7939c000 - 0x7939d000 (Reserved)
[10076529507] [INFO] [kernel::memory]   [28] 0x7939d000 - 0x7969e000 (Other)
[10076860926] [INFO] [kernel::memory]   [29] 0x7969e000 - 0x7969f000 (Reserved)
[10077203532] [INFO] [kernel::memory]   [30] 0x7969f000 - 0x796a8000 (Other)
[10077534489] [INFO] [kernel::memory]   [31] 0x796a8000 - 0x796a9000 (Reserved)
[10077923922] [INFO] [kernel::memory]   [32] 0x796a9000 - 0x796ad000 (Other)
[10078258080] [INFO] [kernel::memory]   [33] 0x796ad000 - 0x796ae000 (Reserved)
[10078601610] [INFO] [kernel::memory]   [34] 0x796ae000 - 0x796b0000 (Other)
[10078932699] [INFO] [kernel::memory]   [35] 0x796b0000 - 0x796b1000 (Reserved)
[10079277747] [INFO] [kernel::memory]   [36] 0x796b1000 - 0x796b3000 (Other)
[10079611113] [INFO] [kernel::memory]   [37] 0x796b3000 - 0x796b4000 (Reserved)
[10079957118] [INFO] [kernel::memory]   [38] 0x796b4000 - 0x796b6000 (Other)
[10080291870] [INFO] [kernel::memory]   [39] 0x796b6000 - 0x796b7000 (Reserved)
[10080638073] [INFO] [kernel::memory]   [40] 0x796b7000 - 0x796b9000 (Other)
[10080972825] [INFO] [kernel::memory]   [41] 0x796b9000 - 0x796ba000 (Reserved)
[10081352655] [INFO] [kernel::memory]   [42] 0x796ba000 - 0x796bc000 (Other)
[10081686153] [INFO] [kernel::memory]   [43] 0x796bc000 - 0x796bd000 (Reserved)
[10082031135] [INFO] [kernel::memory]   [44] 0x796bd000 - 0x796c7000 (Other)
[10082364765] [INFO] [kernel::memory]   [45] 0x796c7000 - 0x796c8000 (Reserved)
[10082709054] [INFO] [kernel::memory]   [46] 0x796c8000 - 0x796cc000 (Other)
[10083041925] [INFO] [kernel::memory]   [47] 0x796cc000 - 0x796cd000 (Reserved)
[10083387930] [INFO] [kernel::memory]   [48] 0x796cd000 - 0x796cf000 (Other)
[10083722055] [INFO] [kernel::memory]   [49] 0x796cf000 - 0x796d0000 (Reserved)
[10084070106] [INFO] [kernel::memory]   [50] 0x796d0000 - 0x796d2000 (Other)
[10084431588] [INFO] [kernel::memory]   [51] 0x796d2000 - 0x796d3000 (Reserved)
[10084780563] [INFO] [kernel::memory]   [52] 0x796d3000 - 0x796d7000 (Other)
[10085115447] [INFO] [kernel::memory]   [53] 0x796d7000 - 0x79750000 (Other)
[10085449242] [INFO] [kernel::memory]   [54] 0x79750000 - 0x79907000 (Other)
[10085783928] [INFO] [kernel::memory]   [55] 0x79907000 - 0x7a16c000 (Reserved)
[10086131319] [INFO] [kernel::memory]   [56] 0x7a16c000 - 0x7bb6c000 (Usable)
[10086469041] [INFO] [kernel::memory]   [57] 0x7bb6c000 - 0x7bb8d000 (Reserved)
[10086816333] [INFO] [kernel::memory]   [58] 0x7bb8d000 - 0x7bb91000 (Other)
[10087149633] [INFO] [kernel::memory]   [59] 0x7bb91000 - 0x7bb92000 (Reserved)
[10087496529] [INFO] [kernel::memory]   [60] 0x7bb92000 - 0x7bb94000 (Other)
[10087852929] [INFO] [kernel::memory]   [61] 0x7bb94000 - 0x7bb95000 (Reserved)
[10088201013] [INFO] [kernel::memory]   [62] 0x7bb95000 - 0x7bb97000 (Other)
[10088536095] [INFO] [kernel::memory]   [63] 0x7bb97000 - 0x7bb98000 (Reserved)
[10089085776] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[10320790821] [CONTRACT] [kernel::memory] Frame allocator initialized with 495729 free frames
[10327922880] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[10332535356] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[10333599606] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[10334275083] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[10340784333] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[10341277320] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[10344269628] [INFO] [bran::arch] IOAPIC: Registers initialized
[10345724961] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[10347331962] [INFO] [bran::arch] IOAPIC: All pins masked
[10348846233] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[10349770695] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[10350539397] [INFO] [bran::arch] IOAPIC: Init complete
[10351378785] [CONTRACT] [kernel] Initializing global allocator...
[10701769188] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[10702791660] [CONTRACT] [kernel] Initializing SIMD...
[10704306921] [CONTRACT] [kernel] Initializing tasking...
[10708865475] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[10710386445] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[10710949491] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[10716356937] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[10716761022] [INFO] [kernel::task::scheduler]   Initializing boot task...
[10717491741] [INFO] [kernel::task::scheduler]   Creating boot task...
[10721737785] [INFO] [kernel::task::scheduler]   Creating idle task...
[10727102991] [INFO] [kernel::task::scheduler]   Boot task initialized
[10727501103] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[10728342636] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[10733843208] [INFO] [kernel::root] Spawning Root service...
[10740996684] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[10750285095] [INFO] [kernel::root::service] ROOT: started once
[11633032884] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[11633864748] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[11673265725] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[11690504067] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[11716656897] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[11753615907] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[11755930989] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[11791852017] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[11814885951] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[11821797174] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[11824770144] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[11847742929] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[11850472227] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[11851448367] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[11854667352] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[11857035465] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[11857767966] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[11864942397] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[11877266346] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[11878131936] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[11880455829] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[11881255947] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[11888732526] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[11889442059] [CONTRACT] [kernel] Spawning init process...
[11891040513] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[11925949431] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (61988300 ticks/sec), init_cnt=619883 for 100Hz
[11927721696] [CONTRACT] [kernel] Entering scheduler loop.
[11937376968] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[11942819691] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072367676144 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563780584
[11952919473] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[11954203371] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[11954922210] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[11955752490] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[11975991687] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[11979466950] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[11982365538] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[11983135362] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[11986482717] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[11989498026] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[11990234916] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[11993998698] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[11994712983] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[11995413804] [INFO] [sprout::devtree] SPROUT: build() called
[11996025426] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[12000130098] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[12000844185] [INFO] [sprout] SPROUT: About to create Supervisor...
[12001502271] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[12002231241] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[12002840025] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[12007127649] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[12042487842] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[12046736922] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[12049719891] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[12052654647] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[12054744174] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[12057758592] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[12061521021] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[12064472079] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[12067478907] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[12070394787] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[12073354260] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[12076684257] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[12080161005] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[12083546244] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[12086413383] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[12089298507] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[12092336421] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[12095307840] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[12098093304] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[12101262624] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[12104252193] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[12107838270] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[12110753622] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[12113653926] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[12116741109] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[12119686194] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[12123048960] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[12126061629] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[12129339156] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[12132701229] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[12135640407] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[12138658290] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[12141565755] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[12144566709] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[12147567201] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[12150750051] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[12154359492] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[12158103144] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[12161115219] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[12164059380] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[12166989219] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[12169870878] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[12172838700] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[12174897405] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[12187294878] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[12249784602] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[12250798857] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[12252645207] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[12297368094] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[12298612623] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[12299434950] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[12301779171] [INFO] [kernel::task::loader] Loading module: /boot/clock
[12302384160] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12303429138] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12306928821] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[12307515759] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[12308810976] [INFO] [kernel::task::loader] Segment: vaddr=204ee8 exec=false
[12309374187] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12315980193] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[12318751863] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[12319817400] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[12320418660] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12321499278] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12325379649] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[12325936293] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[12327202899] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[12327776208] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12333241140] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[12334398054] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[12335304267] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[12335784186] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12336672249] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12347458233] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[12348115956] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[12352421268] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[12353011572] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[12358661172] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[12359317608] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[12360218607] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[12360725916] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12361635891] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12365911800] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[12366460755] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[12368097225] [INFO] [kernel::task::loader] Segment: vaddr=204c88 exec=false
[12368787321] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12375347424] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[12376060323] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[12390791985] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0042c18
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12392123535] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072367987360 RFLAGS_BEFORE=134 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563780584
[12396824121] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[12397724724] [INFO] [clock] starting clock publisher
[12405067950] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12406121574] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368010848 RFLAGS_BEFORE=134 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563780584
[12410531892] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0043308
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[12411593040] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368027232 RFLAGS_BEFORE=134 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563780584
[12413357253] [ERROR] [INGESTD] Starting...
[12417349164] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12418435821] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368054368 RFLAGS_BEFORE=134 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563780584
[12421290948] [INFO] [cambium] cambium starting (v3: catch-up then stream)...
[12426955101] [INFO] [clock] Clock thing created: 356
[12427604640] [INFO] [clock] Waiting for UI Root (Compositor)...
[12432146067] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[12433347102] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[12434333010] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[12435579849] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[12442422861] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[12453204258] [ERROR] [INGESTD] Watch active. Loop start.
[12460451817] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[12461593947] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[12674321385] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[12675016992] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[12676067349] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12679506378] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[12680216802] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12681255378] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[12681855714] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[12688256889] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[12694179795] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[12695280180] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368106208 RFLAGS_BEFORE=130 CR3_BEFORE=54927360 fs_base=0 gs_base=18446744071563780584
[12698747655] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[12716317251] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[12724829964] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[12733655913] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[12734845233] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[12735401217] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[12736651488] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12739854831] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[12740449524] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12741753156] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[12742325277] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[12748855944] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[12750127038] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[12751530693] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[12752674638] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[12754002525] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[12755066148] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[12755674866] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12756698064] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12759051822] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[12759625527] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[12760566456] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[12761119107] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[12767293176] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[12768503220] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[12769055013] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12770006073] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12773234925] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[12774148233] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12775504038] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[12776129322] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[12782255574] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[12783902604] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[12784958043] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[12785510595] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12786520395] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12789481815] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[12790070139] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12791067267] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[12791636814] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[12797624301] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[12812350023] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044678
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[12813569901] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368129680 RFLAGS_BEFORE=134 CR3_BEFORE=55033856 fs_base=0 gs_base=18446744071563780584
[12817699158] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[12818610684] [INFO] [rtc_cmos] Starting... arg=db
[12820319490] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[12823604211] [INFO] [rtc_cmos] RTC: 2026-01-21 01:50:01 = 1768960201 unix_secs
[12824782938] [INFO] [kernel::time] System clock anchored: unix_secs=1768960201, mono_ns=6412149414, offset=1768960194587850586ns
[12826612590] [INFO] [rtc_cmos] System clock anchored
[12844479252] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[12847518948] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00459e8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[12848631840] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368164368 RFLAGS_BEFORE=130 CR3_BEFORE=55136256 fs_base=0 gs_base=18446744071563780584
[12851975136] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[12853850592] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[12855017010] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[12855834519] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[12861188076] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045d70
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[12862558764] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368181776 RFLAGS_BEFORE=130 CR3_BEFORE=55230464 fs_base=0 gs_base=18446744071563780584
[12866094351] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[12867159657] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[12871415337] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ee60
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[12872608188] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368207728 RFLAGS_BEFORE=130 CR3_BEFORE=55332864 fs_base=0 gs_base=18446744071563780584
[12877876374] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[12885011469] [INFO] [bristle] bristle: registered in graph as svc.Input (id=457)
[12894512796] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=431 backend=BootFB
[12895682217] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[12896200152] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12897279615] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12959549922] [INFO] [kernel::task::loader] Segment: vaddr=26b530 exec=false
[12960240315] [INFO] [kernel::task::loader]   Overlap at 26b000: merging perms to r=true w=false x=true
[12968151174] [INFO] [kernel::task::loader] Segment: vaddr=277420 exec=false
[12968762730] [INFO] [kernel::task::loader]   Overlap at 277000: merging perms to r=true w=true x=true
[12975302373] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[12976644879] [INFO] [kernel::task::loader] Loading module: /boot/echo
[12977168325] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12978154893] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12981011637] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[12981597882] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12982568808] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[12983180760] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[12989409312] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[12990375915] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[12991102509] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[13001811603] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014c50
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1af
[13002997557] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368250128 RFLAGS_BEFORE=130 CR3_BEFORE=55439360 fs_base=0 gs_base=18446744071563780584
[13006487505] [INFO] [bloom::logging] bloom: logging initialized
[13028899356] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045d70
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[13030145139] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368267584 RFLAGS_BEFORE=130 CR3_BEFORE=56016896 fs_base=0 gs_base=18446744071563780584
[13033408179] [INFO] [echo] echo: online (handle=12)
[13034162526] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[13040701806] [INFO] [bloom] bloom: [bloom] Bootstrapped via Bytespace ID=431
[13043685996] [INFO] [bloom] bloom: [bloom] starting (arg_req=1 arg_resp=4 bristle=10 font_svc=0)
[13052800695] [INFO] [bloom] bloom: [bloom] spawned wallpaper_loader thread (tid=15)
[13058574705] [INFO] [bloom] bloom: [bloom] spawned cursor_loader thread (tid=16)
[13063914270] [INFO] [bloom] bloom: [bloom] spawned font_loader thread (tid=17)
[13064574996] [INFO] [bloom] bloom: [bloom] discovering compositor target...
T:DB40 [13069275417] [INFO] [bloom] bloom: [wallpaper_loader] thread started
T:D770 [13071684846] [INFO] [bloom] bloom: [cursor_loader] thread started
T:CD00 [13073378571] [INFO] [bloom] bloom: [font_loader] thread started
[13074070713] [INFO] [bloom] bloom: [font_loader] scanning boot modules for fonts...
[13083525345] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[13084300680] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[13085252532] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
[13088459208] [INFO] [bloom] bloom: [font_loader] found 42 boot modules
[13173398832] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/DSEG7Classic-Regular.ttf'
[13180075623] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=179 size=23272 name='/assets/fonts/DSEG7Classic-Regular.ttf'
[13187237877] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:CB60 [13191607473] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[13192734984] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[13195406862] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/Hack-Regular.ttf'
[13199395869] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[13200736725] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[13222113795] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[13224514215] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[13230706830] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=182 size=309408 name='/assets/fonts/Hack-Regular.ttf'
[13235276901] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[13237440678] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSans-Regular.ttf'
[13241526870] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[13242250329] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[13801447176] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[13802727873] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[13806304545] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=185 size=569208 name='/assets/fonts/NotoSans-Regular.ttf'
[13811065092] [INFO] [bloom] bloom: [wallpaper_loader] searching for wallpaper...
[13811949360] [INFO] [bloom] bloom: [wallpaper_loader] trying: /assets/wallpapers/clouds.bmp
[13815887910] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[13818266616] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol-Regular.ttf'
[13824665712] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[13825378545] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[14217442899] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=188 size=258156 name='/assets/fonts/NotoSansSymbol-Regular.ttf'
[14223894960] [INFO] [bloom] bloom: [cursor_loader] searching for cursor...
[14224786422] [INFO] [bloom] bloom: [cursor_loader] trying: /assets/cursors/plain/Normal.cur
[14550338286] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[15193799643] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=191 size=656852 name='/assets/fonts/NotoSansSymbol2-Regular.ttf'
[15536118213] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSerif-Regular.ttf'
[15539518830] [INFO] [bloom::compositor] bloom: compositor bytespace 376 (1280x720 stride=5120 format=1)
[15551536539] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[15552954516] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[15553812285] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (258156 bytes)
[15561167028] [INFO] [bloom::asset] [asset_bank] mapped at 0x10463000
[15561875175] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[15848883204] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=194 size=616196 name='/assets/fonts/NotoSerif-Regular.ttf'
[16509973194] [INFO] [bloom::compositor] bloom: display backend: BootFB
[16514746083] [INFO] [ps2_mouse] ps2_mouse: init done
[16515411792] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[16516341930] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[16517057403] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[16828264629] [INFO] [bloom] bloom: [font_loader] immediate scan complete, entering watch loop
[16831257795] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[17160720423] [INFO] [bloom] bloom: [bloom] compositor target: 1280x720 @ 0x104a3000 backend=BootFB
[17161885191] [INFO] [bloom] bloom: [bloom] presenter: driver (req=1 resp=4)
[17163366825] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[17482029609] [INFO] [stem::ui] UiBuilder: created root 546
[17482916253] [INFO] [bloom] bloom: [bloom] created UI root node: 546
[17491635876] [INFO] [display_bootfb] display_bootfb: bound bytespace 376
[17804636256] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[17805990609] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[17813450919] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd5b0
[17814228630] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[17814885660] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[17815768641] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[17823593205] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 191 (656852 bytes)
[17826548520] [INFO] [bloom] bloom: [bloom] ui watch opened UNFILTERED (userspace filter pred=UI_TEXT id=519)
[17834944809] [INFO] [bloom::asset] [asset_bank] mapped at 0x10bab000
[17835854916] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[18793818285] [INFO] [clock] Found UI Root: 546 (attempt 4)
[21073658232] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c1b40
[21074711691] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[21075765381] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21077320473] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[21400956852] [INFO] [bloom] bloom: [font_loader] watch opened (id=570)
[23692765635] [INFO] [bloom] bloom: [cursor_loader] SUCCESS: found candidate '/assets/cursors/plain/Normal.cur', enqueuing load
[23694190740] [INFO] [bloom] bloom: [cursor_loader] thread done, sleeping forever
[24671896161] [INFO] [bloom] bloom: [wallpaper_loader] SUCCESS: found candidate '/assets/wallpapers/clouds.bmp', enqueuing load
[24673454982] [INFO] [bloom] bloom: [wallpaper_loader] thread done, sleeping forever
[25005703899] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[25006732443] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol2-Regular.ttf' in slot 4
[25007681325] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 194 (616196 bytes)
[25016573769] [INFO] [bloom::asset] [asset_bank] mapped at 0x10c4c000
[25017323562] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSerif-Regular.ttf'...
[27613705977] [INFO] [cambium] Found 1 bindings
[28591632960] [INFO] [clock] Binding created: 590 (source=356 target=575)
[28592576430] [INFO] [clock] CLOCK: Entering main loop, publishing to thing_id=356
[28595118981] [INFO] [clock] unix=1768960208 utc=2026-01-21 01:50:08 mono_ns=14296793923
[28621069686] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fedd0
[28621793772] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[28622632995] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28623454134] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x4 kind=0 pred=0 subj_lo=356
[28952437041] [INFO] [cambium] Opened watch 605 for source 356 (binding 590, start_seq=0)
[28974276936] [INFO] [cambium] CATCH-UP: Draining 1 watches for historical events...
[29931322509] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='01:50:08' tick=14296793923
[31893823467] [INFO] [cambium] cambium: drain complete payloads=0 overflows=8 last_seq=none
[31895088885] [INFO] [cambium] Entering event loop with 1 bindings (v3)
[33200580897] [INFO] [clock] unix=1768960211 utc=2026-01-21 01:50:11 mono_ns=16600075734
[34505095614] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='01:50:11' tick=16600075734
[37772179104] [INFO] [clock] unix=1768960213 utc=2026-01-21 01:50:13 mono_ns=18885927027
[38942292147] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[38943710652] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSerif-Regular.ttf' in slot 5
[38944581720] [INFO] [bloom::asset] [asset_bank] load_cursor_immediate: /assets/cursors/plain/Normal.cur
[38951664642] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='01:50:13' tick=18885927027
[39039616341] [INFO] [bloom::asset] [asset_bank] mapping bytespace 152 (4286 bytes)
[39044299008] [INFO] [bloom::asset] [asset_bank] mapped to 0x10ce3000
[39045013161] [INFO] [bloom::asset] [asset_bank] checking CUR header: len=4286
[39045725334] [INFO] [bloom::asset] [asset_bank] valid CUR header detected
[39046575348] [INFO] [bloom::asset] [asset_bank] CUR: hotspot=(0, 0), img_size=4264, offset=22
[39047361507] [INFO] [bloom::asset] [asset_bank] decoding embedded DIB at offset 22...
[39048917424] [INFO] [bloom::asset] [asset_bank] SUCCESS: cursor DIB decoded 32x32
[39055422087] [INFO] [bloom::asset] [asset_bank] publish_cursor (pending)
[39056144985] [INFO] [bloom::asset] [asset_bank] cursor: Static frame 32x32 hotspot (0, 0)
[39056989983] [INFO] [bloom::asset] [asset_bank] load_wallpaper_immediate: /assets/wallpapers/clouds.bmp
[39169461639] [INFO] [bloom::asset] [asset_bank] mapping bytespace 170 (3145782 bytes)
[39177896340] [INFO] [bloom::asset] [asset_bank] mapped to 0x10ce5000
[39178579077] [INFO] [bloom::asset] [asset_bank] decoding BMP...
[39530182824] [INFO] [bloom::asset] [asset_bank] BMP decoded: 1024x1024
[39913626258] [INFO] [bloom::asset] [asset_bank] bytespace unmapped
[39914645793] [INFO] [bloom::asset] [asset_bank] publish_wallpaper (pending): 1024x1024
[40068928977] [INFO] [cambium] [cambium] write: binding_src=356 target=575 pred=ui.Text(519) val=599 seq=1141
[40074423114] [INFO] [cambium] Updated target 575 with value 599 (seq=1141)
[40089850746] [INFO] [cambium] Updated target 575 with value 617 (seq=1153)
[40104557856] [INFO] [cambium] Updated target 575 with value 626 (seq=1163)
[42271219386] [INFO] [bloom] bloom: [font_loader] drain complete: payloads=1045 overflows=31
[42276190704] [INFO] [bloom] bloom: [bloom] ui watch drained: 1055 batches
[42312064971] [INFO] [bloom] bloom: [bloom] entering transactional frame loop (acquire -> build -> present)
[42313117044] [INFO] [bloom] bloom: [bloom] reclaimer: budget=33554432 bytes
[42314143905] [INFO] [bloom::reclaimer] [reclaimer] +4194304 bytes (total: 4194304)
[42314866605] [INFO] [bloom::asset] [asset_bank] promoting wallpaper to gen=1 (4194304b)
[42315773676] [INFO] [bloom::reclaimer] [reclaimer] +4096 bytes (total: 4198400)
[42316454532] [INFO] [bloom::asset] [asset_bank] promoting cursor to gen=1 (4096b)
[42317524821] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4300800)
[42318661572] [INFO] [bloom::asset] [asset_bank] promoting font 'DSEG7Classic-Regular.ttf' to gen=1 (102400b) in slot 0
[42319606494] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4403200)
[42320256792] [INFO] [bloom::asset] [asset_bank] promoting font 'Hack-Regular.ttf' to gen=1 (102400b) in slot 1
[42321068229] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4505600)
[42321694305] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=1 (102400b) in slot 2
[42322446573] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4608000)
[42323071296] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol-Regular.ttf' to gen=1 (102400b) in slot 3
[42323844156] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4710400)
[42324510987] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol2-Regular.ttf' to gen=1 (102400b) in slot 4
[42325360836] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4812800)
[42325986483] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSerif-Regular.ttf' to gen=1 (102400b) in slot 5
[42327451617] [INFO] [bloom] bloom: [bloom] frame 1: wallpaper now visible (gen=1)
[42328409574] [INFO] [bloom] bloom: [bloom] frame 1: cursor now visible (gen=1)
[42331947141] [INFO] [bloom] bloom: [bloom] frame 1: fonts available (mode=legacy)
[42351853368] [INFO] [bloom::ui] bloom: [bloom][ui] Initializing cached UI symbols (one-time)
[42799377159] [INFO] [clock] unix=1768960215 utc=2026-01-21 01:50:15 mono_ns=21399541152
[42811481394] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='01:50:15' tick=21399541152
[42869563473] [INFO] [cambium] [cambium] write: binding_src=356 target=575 pred=ui.Text(519) val=682 seq=1186
[42873381045] [INFO] [cambium] Updated target 575 with value 682 (seq=1186)
[43221907839] [INFO] [bloom] bloom: [CONTRACT] [bloom] First frame rendered - compositor ready
[43619957271] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1175
[44733207321] [INFO] [bloom] bloom: [bloom][damage] rects=1 frame=10
[44927130171] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1179
[45807306534] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1185
[46066604199] [INFO] [clock] unix=1768960217 utc=2026-01-21 01:50:17 mono_ns=23033165892
[46078432224] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='01:50:17' tick=23033165892
[46136314191] [INFO] [cambium] [cambium] write: binding_src=356 target=575 pred=ui.Text(519) val=693 seq=1206
[46140187962] [INFO] [cambium] Updated target 575 with value 693 (seq=1206)
[47081103135] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1188
[47624283300] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1189
[48159442980] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1190
[48701513487] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1191
[49092692319] [INFO] [bloom] bloom: [bloom][damage] rects=1 frame=24
[49235665776] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1192
[49332964956] [INFO] [clock] unix=1768960219 utc=2026-01-21 01:50:19 mono_ns=24666304162
[49349282994] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='01:50:19' tick=24666304162
[49403078637] [INFO] [cambium] [cambium] write: binding_src=356 target=575 pred=ui.Text(519) val=705 seq=1216
[49407063948] [INFO] [cambium] Updated target 575 with value 705 (seq=1216)
[49793678154] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1193
[50458470348] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1194
[51000256890] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1195
[51401032584] [INFO] [bloom] bloom: [bloom][damage] rects=1 frame=28
[51549611883] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1196
[52094038590] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1197
[52629708450] [INFO] [clock] unix=1768960220 utc=2026-01-21 01:50:20 mono_ns=26314661026
[52638223506] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1198
[52648797861] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='01:50:20' tick=26314661026
[52800971949] [INFO] [cambium] [cambium] write: binding_src=356 target=575 pred=ui.Text(519) val=717 seq=1225
[52805671578] [INFO] [cambium] Updated target 575 with value 717 (seq=1225)
[53207846571] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1199
[53886173550] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects, 22011 px, 2% screen) = 10.4ms
[53887936707] [INFO] [bloom::raster] bloom:   rect[0]: 253x87 (22011 px, 2%) @ 933,571
[53897124963] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1200
[54452956140] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1201
[54995270550] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1202
[55539849222] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1203
[55899391449] [INFO] [clock] unix=1768960222 utc=2026-01-21 01:50:22 mono_ns=27949487098
[55912186836] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='01:50:22' tick=27949487098
[55968176022] [INFO] [bloom] bloom: [bloom][damage] rects=1 frame=36
[56114421858] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1204
[56119403340] [INFO] [cambium] [cambium] write: binding_src=356 target=575 pred=ui.Text(519) val=731 seq=1233
[56125357167] [INFO] [cambium] Updated target 575 with value 731 (seq=1233)
[56781866625] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1205
[57471473961] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1208
[58011325053] [INFO] [bloom] bloom: [bloom][uiwatch] UI_TEXT event: seq=1209 subj=658 pred=519
[58012326603] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1209
[58549778925] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1210
[59092360272] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1211
[59166564369] [INFO] [clock] unix=1768960224 utc=2026-01-21 01:50:24 mono_ns=29583069846
[59178566931] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='01:50:24' tick=29583069846
[59235766062] [INFO] [cambium] [cambium] write: binding_src=356 target=575 pred=ui.Text(519) val=745 seq=1244
[59239653726] [INFO] [cambium] Updated target 575 with value 745 (seq=1244)
[59639932242] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1212
[60282010470] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects, 22011 px, 2% screen) = 10.4ms
[60283276779] [INFO] [bloom::raster] bloom:   rect[0]: 253x87 (22011 px, 2%) @ 933,571
[60295214892] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1213

```
</details>
