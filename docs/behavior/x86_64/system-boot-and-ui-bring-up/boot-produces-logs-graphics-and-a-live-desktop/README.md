# ✅ Scenario: Boot produces logs, graphics, and a live desktop

> Last run: 2026-01-20 19:33:04

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 3857ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | Then I should see log messages on the terminal | ✅ | 403ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And each log message should include a monotonically increasing timestamp | ✅ | 404ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And I should see the system clock tick for several seconds in the serial console | ✅ | 1803ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |
| 5 | And I should see the wallpaper on the screen within 15 seconds | ✅ | 5383ms | <a href="./05/after.png"><img src="./05/after.png" width="150" /></a> [📜](./05/serial.log) [💾](./05/registers.txt) |
| 6 | And I should see a cursor centered on the screen | ✅ | 1073ms | <a href="./06/after.png"><img src="./06/after.png" width="150" /></a> [📜](./06/serial.log) [💾](./06/registers.txt) |
| 7 | And I should see the text "thing-os" in the top-left corner of the screen | ✅ | 491ms | <a href="./07/after.png"><img src="./07/after.png" width="150" /></a> [📜](./07/serial.log) [💾](./07/registers.txt) |
| 8 | And I should see frame count information in the top-left corner of the screen | ✅ | 468ms | <a href="./08/after.png"><img src="./08/after.png" width="150" /></a> [📜](./08/serial.log) [💾](./08/registers.txt) |
| 9 | And I should see a clock window displaying a ticking clock | ✅ | 968ms | <a href="./09/after.png"><img src="./09/after.png" width="150" /></a> [📜](./09/serial.log) [💾](./09/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[10891749561] [CONTRACT] [kernel] thing-os kernel starting...
[10901004081] [INFO] [kernel::memory] Memory map has 64 entries
[10903087272] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[10903703217] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[10904069451] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[10904458224] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[10904787069] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[10905109875] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[10905440931] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[10905787233] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[10906178217] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78656000 (Usable)
[10906520724] [INFO] [kernel::memory]   [9] 0x78656000 - 0x786b8000 (Reserved)
[10906872636] [INFO] [kernel::memory]   [10] 0x786b8000 - 0x7883a000 (Other)
[10907225175] [INFO] [kernel::memory]   [11] 0x7883a000 - 0x7883b000 (Reserved)
[10907589825] [INFO] [kernel::memory]   [12] 0x7883b000 - 0x788d2000 (Other)
[10907945466] [INFO] [kernel::memory]   [13] 0x788d2000 - 0x788d3000 (Reserved)
[10908302295] [INFO] [kernel::memory]   [14] 0x788d3000 - 0x78974000 (Other)
[10908645858] [INFO] [kernel::memory]   [15] 0x78974000 - 0x78975000 (Reserved)
[10909000410] [INFO] [kernel::memory]   [16] 0x78975000 - 0x789b5000 (Other)
[10909360506] [INFO] [kernel::memory]   [17] 0x789b5000 - 0x789b6000 (Reserved)
[10909718094] [INFO] [kernel::memory]   [18] 0x789b6000 - 0x78a41000 (Other)
[10910061723] [INFO] [kernel::memory]   [19] 0x78a41000 - 0x78a42000 (Reserved)
[10910417562] [INFO] [kernel::memory]   [20] 0x78a42000 - 0x78a8e000 (Other)
[10910759772] [INFO] [kernel::memory]   [21] 0x78a8e000 - 0x78a8f000 (Reserved)
[10911115479] [INFO] [kernel::memory]   [22] 0x78a8f000 - 0x78a95000 (Other)
[10911472143] [INFO] [kernel::memory]   [23] 0x78a95000 - 0x78a96000 (Reserved)
[10911829995] [INFO] [kernel::memory]   [24] 0x78a96000 - 0x78f17000 (Other)
[10912175538] [INFO] [kernel::memory]   [25] 0x78f17000 - 0x78f18000 (Reserved)
[10912550385] [INFO] [kernel::memory]   [26] 0x78f18000 - 0x79399000 (Other)
[10912904574] [INFO] [kernel::memory]   [27] 0x79399000 - 0x7939a000 (Reserved)
[10913261832] [INFO] [kernel::memory]   [28] 0x7939a000 - 0x7969b000 (Other)
[10913608431] [INFO] [kernel::memory]   [29] 0x7969b000 - 0x7969c000 (Reserved)
[10913977701] [INFO] [kernel::memory]   [30] 0x7969c000 - 0x796a5000 (Other)
[10914335751] [INFO] [kernel::memory]   [31] 0x796a5000 - 0x796a6000 (Reserved)
[10914703404] [INFO] [kernel::memory]   [32] 0x796a6000 - 0x796aa000 (Other)
[10915058748] [INFO] [kernel::memory]   [33] 0x796aa000 - 0x796ab000 (Reserved)
[10915424784] [INFO] [kernel::memory]   [34] 0x796ab000 - 0x796ad000 (Other)
[10915784055] [INFO] [kernel::memory]   [35] 0x796ad000 - 0x796ae000 (Reserved)
[10916154150] [INFO] [kernel::memory]   [36] 0x796ae000 - 0x796b0000 (Other)
[10916525796] [INFO] [kernel::memory]   [37] 0x796b0000 - 0x796b1000 (Reserved)
[10916885265] [INFO] [kernel::memory]   [38] 0x796b1000 - 0x796b3000 (Other)
[10917230115] [INFO] [kernel::memory]   [39] 0x796b3000 - 0x796b4000 (Reserved)
[10917587571] [INFO] [kernel::memory]   [40] 0x796b4000 - 0x796b6000 (Other)
[10917932949] [INFO] [kernel::memory]   [41] 0x796b6000 - 0x796b7000 (Reserved)
[10918292451] [INFO] [kernel::memory]   [42] 0x796b7000 - 0x796b9000 (Other)
[10918638159] [INFO] [kernel::memory]   [43] 0x796b9000 - 0x796ba000 (Reserved)
[10918996242] [INFO] [kernel::memory]   [44] 0x796ba000 - 0x796c4000 (Other)
[10919366898] [INFO] [kernel::memory]   [45] 0x796c4000 - 0x796c5000 (Reserved)
[10919726334] [INFO] [kernel::memory]   [46] 0x796c5000 - 0x796c9000 (Other)
[10920072702] [INFO] [kernel::memory]   [47] 0x796c9000 - 0x796ca000 (Reserved)
[10920432831] [INFO] [kernel::memory]   [48] 0x796ca000 - 0x796cc000 (Other)
[10920780684] [INFO] [kernel::memory]   [49] 0x796cc000 - 0x796cd000 (Reserved)
[10921139691] [INFO] [kernel::memory]   [50] 0x796cd000 - 0x796cf000 (Other)
[10921485234] [INFO] [kernel::memory]   [51] 0x796cf000 - 0x796d0000 (Reserved)
[10921845099] [INFO] [kernel::memory]   [52] 0x796d0000 - 0x796d4000 (Other)
[10922190180] [INFO] [kernel::memory]   [53] 0x796d4000 - 0x796d5000 (Reserved)
[10922597598] [INFO] [kernel::memory]   [54] 0x796d5000 - 0x79750000 (Other)
[10922945088] [INFO] [kernel::memory]   [55] 0x79750000 - 0x79907000 (Other)
[10923291621] [INFO] [kernel::memory]   [56] 0x79907000 - 0x7a16c000 (Reserved)
[10923652047] [INFO] [kernel::memory]   [57] 0x7a16c000 - 0x7bb6c000 (Usable)
[10924003563] [INFO] [kernel::memory]   [58] 0x7bb6c000 - 0x7bb8c000 (Reserved)
[10924360788] [INFO] [kernel::memory]   [59] 0x7bb8c000 - 0x7bb90000 (Other)
[10924706991] [INFO] [kernel::memory]   [60] 0x7bb90000 - 0x7bb91000 (Reserved)
[10925064150] [INFO] [kernel::memory]   [61] 0x7bb91000 - 0x7bb93000 (Other)
[10925408076] [INFO] [kernel::memory]   [62] 0x7bb93000 - 0x7bb94000 (Reserved)
[10925929047] [INFO] [kernel::memory]   [63] 0x7bb94000 - 0x7bb96000 (Other)
[10926641715] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[11166068958] [CONTRACT] [kernel::memory] Frame allocator initialized with 495726 free frames
[11173042683] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[11177676807] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[11178730431] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[11179435839] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[11187469953] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[11188008249] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11190923634] [INFO] [bran::arch] IOAPIC: Registers initialized
[11191980030] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[11193430776] [INFO] [bran::arch] IOAPIC: All pins masked
[11194715268] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11195317485] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11195804532] [INFO] [bran::arch] IOAPIC: Init complete
[11196409125] [CONTRACT] [kernel] Initializing global allocator...
[11567611506] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[11568518973] [CONTRACT] [kernel] Initializing SIMD...
[11569999287] [CONTRACT] [kernel] Initializing tasking...
[11574766566] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[11576459037] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[11577403068] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[11583577038] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[11585072400] [INFO] [kernel::task::scheduler]   Initializing boot task...
[11586123021] [INFO] [kernel::task::scheduler]   Creating boot task...
[11592116877] [INFO] [kernel::task::scheduler]   Creating idle task...
[11599001106] [INFO] [kernel::task::scheduler]   Boot task initialized
[11599684536] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[11601168942] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[11608875597] [INFO] [kernel::root] Spawning Root service...
[11616930501] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[11627769219] [INFO] [kernel::root::service] ROOT: started once
[12552176241] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[12553644114] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[12597901932] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[12617098428] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[12645311844] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[12678739128] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[12680449584] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[12716545842] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[12740311221] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[12746484201] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[12749090112] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[12772978350] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[12775562052] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[12776574591] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[12779930823] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[12782782980] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[12783598938] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12791216955] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12803931789] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[12804936672] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[12807608253] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[12808464108] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[12816707970] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[12817434795] [CONTRACT] [kernel] Spawning init process...
[12819049848] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[12854352126] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (63257300 ticks/sec), init_cnt=632573 for 100Hz
[12857538639] [CONTRACT] [kernel] Entering scheduler loop.
[12867130056] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[12872107083] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072367676144 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563780584
[12882910128] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[12884372457] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[12885136242] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[12885940023] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[12906802425] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[12912799416] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[12916649295] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[12917712918] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[12921796998] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[12924460164] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[12925536195] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[12928970934] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[12930245394] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[12931261761] [INFO] [sprout::devtree] SPROUT: build() called
[12932236317] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[12938012571] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[12939356364] [INFO] [sprout] SPROUT: About to create Supervisor...
[12940304718] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[12941425860] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[12942396588] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[12947372922] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[12984822213] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[12991754424] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[12995575329] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[12998968059] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[13001355510] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[13005509253] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[13009982799] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[13013824428] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[13017864552] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[13021789143] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[13025082180] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[13028718120] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[13032305946] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[13035354222] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[13038356166] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[13041528786] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[13044741402] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[13047925275] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[13050838449] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[13054197288] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[13057692285] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[13060943214] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[13063906350] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[13066885029] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[13070051247] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[13073618514] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[13076687052] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[13079737143] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[13082954148] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[13085989257] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[13089298959] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[13092470094] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[13095489429] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[13098695379] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[13102019205] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[13105227036] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[13108473279] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[13111685103] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[13114777533] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[13117918737] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[13121049744] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[13124324565] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[13127443032] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[13129893612] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[13142241090] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[13214880987] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[13215869997] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[13217737104] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[13218798813] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[13219556988] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[13220380800] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[13223153262] [INFO] [kernel::task::loader] Loading module: /boot/clock
[13223713140] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13224818145] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13228842858] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[13229424021] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[13230757683] [INFO] [kernel::task::loader] Segment: vaddr=204ee8 exec=false
[13231346898] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13238251785] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[13241171526] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[13242147996] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[13242677415] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13244055132] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13249209732] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[13250063343] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[13251017769] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[13251602298] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13258338126] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[13259499759] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[13260475140] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[13260996111] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13261958391] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13272840801] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[13273501923] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[13277770176] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[13278342858] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[13284636948] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[13285347702] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[13286786370] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[13287309651] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13288444224] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13294605093] [INFO] [kernel::task::loader] Segment: vaddr=204360 exec=false
[13295306046] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[13296640599] [INFO] [kernel::task::loader] Segment: vaddr=205168 exec=false
[13297364652] [INFO] [kernel::task::loader]   Overlap at 205000: merging perms to r=true w=true x=true
[13304695140] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[13305519249] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[13322443002] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13323851508] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072367986928 RFLAGS_BEFORE=134 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563780584
[13327994064] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[13329029439] [INFO] [clock] starting clock publisher
[13338926139] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13340121960] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368009968 RFLAGS_BEFORE=134 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563780584
[13345023252] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ab8
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[13346225937] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368026352 RFLAGS_BEFORE=134 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563780584
[13348468056] [ERROR] [INGESTD] Starting...
[13352236161] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ae8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13353491844] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368053488 RFLAGS_BEFORE=134 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563780584
[13356515634] [INFO] [cambium] cambium starting (v4: no-op suppression)...
[13362780321] [INFO] [clock] Clock thing created: 356
[13363524735] [INFO] [clock] Waiting for UI Root (Compositor)...
[13368360126] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[13369614522] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[13370698209] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[13380230721] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[13398543906] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[13407460869] [ERROR] [INGESTD] Watch active. Loop start.
[13415456010] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[13416746640] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[13640795586] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[13641545115] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[13642639098] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13645896363] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[13646582862] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13647581871] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[13648158843] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13655672646] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[13662055704] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[13663246740] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368106288 RFLAGS_BEFORE=134 CR3_BEFORE=54931456 fs_base=0 gs_base=18446744071563780584
[13666721805] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[13683091257] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[13692146424] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[13701257295] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[13702541523] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[13703914290] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[13705998603] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13709180034] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[13709776377] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13711114230] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[13711682391] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[13718300970] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[13719695781] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[13721963673] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[13723788738] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[13725749895] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[13726784709] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[13727301885] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13728298419] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13730711511] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[13731298185] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[13732243734] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[13732814436] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[13739366091] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[13740711534] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[13741244583] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13742279925] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13745288634] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[13745862372] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13747179501] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[13747738356] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[13754495139] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[13756817646] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[13757971458] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[13758512163] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13759477083] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13762487871] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[13763089164] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13764085731] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[13764643332] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13771133937] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[13786716702] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044678
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[13788194970] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368129856 RFLAGS_BEFORE=130 CR3_BEFORE=55037952 fs_base=0 gs_base=18446744071563780584
[13792587468] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[13793516484] [INFO] [rtc_cmos] Starting... arg=db
[13795109856] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[13798273896] [INFO] [rtc_cmos] RTC: 2026-01-21 03:33:10 = 1768966390 unix_secs
[13799451963] [INFO] [kernel::time] System clock anchored: unix_secs=1768966390, mono_ns=6899487969, offset=1768966383100512031ns
[13800647190] [INFO] [rtc_cmos] System clock anchored
[13816806300] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[13819248663] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ba00
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[13820340303] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368164432 RFLAGS_BEFORE=134 CR3_BEFORE=55140352 fs_base=0 gs_base=18446744071563780584
[13847557614] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[13849694958] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[13850944074] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[13851785805] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[13857601626] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0095ee8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[13858999572] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368181840 RFLAGS_BEFORE=134 CR3_BEFORE=55234560 fs_base=0 gs_base=18446744071563780584
[13863429063] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[13864481565] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[13868429421] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00a9b30
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[13869782487] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368207712 RFLAGS_BEFORE=134 CR3_BEFORE=55336960 fs_base=0 gs_base=18446744071563780584
[13873384041] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[13881030240] [INFO] [bristle] bristle: registered in graph as svc.Input (id=457)
[13890906414] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=431 backend=BootFB
[13892167311] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[13892727123] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13893854040] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13965753318] [INFO] [kernel::task::loader] Segment: vaddr=26e020 exec=false
[13966765296] [INFO] [kernel::task::loader]   Overlap at 26e000: merging perms to r=true w=false x=true
[13974001998] [INFO] [kernel::task::loader] Segment: vaddr=278d58 exec=false
[13974682590] [INFO] [kernel::task::loader]   Overlap at 278000: merging perms to r=true w=true x=true
[13982536986] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[13984065942] [INFO] [kernel::task::loader] Loading module: /boot/echo
[13984620012] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13985702082] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13988820582] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[13989499689] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13990572915] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[13991186220] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13998868224] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[14000086188] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[14000878485] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[14012578437] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014c50
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1af
[14013920613] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368250320 RFLAGS_BEFORE=130 CR3_BEFORE=55443456 fs_base=0 gs_base=18446744071563780584
[14017029114] [INFO] [bloom::logging] bloom: logging initialized
[14041131489] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ee60
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[14042448486] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368267776 RFLAGS_BEFORE=134 CR3_BEFORE=56029184 fs_base=0 gs_base=18446744071563780584
[14045930844] [INFO] [echo] echo: online (handle=12)
[14046720039] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[14081475408] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[14082389739] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[14083358091] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
T:E580 T:E410 T:D530 [14194760250] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:25D0 [14199969432] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[14201363286] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[14206880094] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[14207822409] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[14233278246] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[14236257849] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[14245985061] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[14251491606] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[14252481045] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[15244286145] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[15245528001] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[15502506162] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[15512148102] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[15512978943] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[18523372824] [INFO] [bloom::compositor] bloom: compositor bytespace 376 (1280x720 stride=5120 format=1)
[19086916431] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[19089095487] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[19090223757] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (258156 bytes)
[19099590444] [INFO] [bloom::asset] [asset_bank] mapped at 0x10463000
[19100399604] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[19194192798] [INFO] [bloom::compositor] bloom: display backend: BootFB
[19199581962] [INFO] [ps2_mouse] ps2_mouse: init done
[19200379572] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[19201403826] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[19202162892] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[19856610411] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[20534378634] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[20859322407] [INFO] [stem::ui] UiBuilder: created root 516
[20869129281] [INFO] [display_bootfb] display_bootfb: bound bytespace 376
[21191849085] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd610
[21192790344] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[21193574358] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21194493243] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=487 subj_lo=0
[22191365559] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c1b90
[22192316718] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[22193102019] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22194186267] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[22370432700] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[22371986373] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[22373019339] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 191 (656852 bytes)
[22385443311] [INFO] [bloom::asset] [asset_bank] mapped at 0x10bab000
[22386369060] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[22864977729] [INFO] [clock] Found UI Root: 516 (attempt 5)
[25758342984] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[25760296122] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol2-Regular.ttf' in slot 4
[25761419805] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 194 (616196 bytes)
[25773930336] [INFO] [bloom::asset] [asset_bank] mapped at 0x10c4c000
[25775437281] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSerif-Regular.ttf'...
[27525768831] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 102400)
[27526806351] [INFO] [bloom::asset] [asset_bank] promoting font 'DSEG7Classic-Regular.ttf' to gen=1 (102400b) in slot 0
[27527903205] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 204800)
[27529201656] [INFO] [bloom::asset] [asset_bank] promoting font 'Hack-Regular.ttf' to gen=1 (102400b) in slot 1
[27530581353] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 307200)
[27531549672] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=1 (102400b) in slot 2
[27532756977] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 409600)
[27533753577] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol-Regular.ttf' to gen=1 (102400b) in slot 3
[27535055988] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 512000)
[27535956327] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol2-Regular.ttf' to gen=1 (102400b) in slot 4
[32066731290] [INFO] [bloom::ui] bloom: [bloom][ui] Initializing cached UI symbols (one-time)
[33738013023] [INFO] [cambium] Found 1 bindings
[34730883528] [INFO] [clock] Binding created: 569 (source=356 target=552)
[34731914118] [INFO] [clock] CLOCK: Entering main loop, publishing to thing_id=356
[34734654075] [INFO] [clock] unix=1768966400 utc=2026-01-21 03:33:20 mono_ns=17366498275
[34765708758] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fedd0
[34766769708] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[34767622230] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34768469538] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x4 kind=0 pred=0 subj_lo=356
[35100441288] [INFO] [cambium] Opened watch 584 for source 356 (binding 569, start_seq=0)
[35129763207] [INFO] [cambium] CATCH-UP: Draining 1 watches for historical events...
[36129575328] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='03:33:20' tick=17366498275
[38129487231] [INFO] [cambium] cambium: drain complete payloads=0 overflows=8 last_seq=none
[38130397800] [INFO] [cambium] Entering event loop with 1 bindings (v4)
[38641070688] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[38642326800] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSerif-Regular.ttf' in slot 5
[38643256806] [INFO] [bloom::asset] [asset_bank] load_cursor_immediate: /assets/cursors/plain/Normal.cur
[38788036386] [INFO] [bloom::asset] [asset_bank] mapping bytespace 152 (4286 bytes)
[38793700770] [INFO] [bloom::asset] [asset_bank] mapped to 0x10ce3000
[38794500888] [INFO] [bloom::asset] [asset_bank] checking CUR header: len=4286
[38795261604] [INFO] [bloom::asset] [asset_bank] valid CUR header detected
[38796165903] [INFO] [bloom::asset] [asset_bank] CUR: hotspot=(0, 0), img_size=4264, offset=22
[38796941601] [INFO] [bloom::asset] [asset_bank] decoding embedded DIB at offset 22...
[38798916651] [INFO] [bloom::asset] [asset_bank] SUCCESS: cursor DIB decoded 32x32
[38807862720] [INFO] [bloom::asset] [asset_bank] publish_cursor (pending)
[38808856647] [INFO] [bloom::asset] [asset_bank] cursor: Static frame 32x32 hotspot (0, 0)
[38810007885] [INFO] [bloom::asset] [asset_bank] load_wallpaper_immediate: /assets/wallpapers/clouds.bmp
[38989130994] [INFO] [bloom::asset] [asset_bank] mapping bytespace 170 (3145782 bytes)
[38999109336] [INFO] [bloom::asset] [asset_bank] mapped to 0x10ce5000
[39000107850] [INFO] [bloom::asset] [asset_bank] decoding BMP...
[39504949572] [INFO] [bloom::asset] [asset_bank] BMP decoded: 1024x1024
[40015693707] [INFO] [bloom::asset] [asset_bank] bytespace unmapped
[40017057993] [INFO] [bloom::asset] [asset_bank] publish_wallpaper (pending): 1024x1024
[40294706958] [INFO] [clock] unix=1768966403 utc=2026-01-21 03:33:23 mono_ns=20147118667
[40313666250] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='03:33:23' tick=20147118667
[40530568023] [INFO] [bloom] bloom: [CONTRACT] [bloom] First frame rendered
[40545379281] [INFO] [bloom::reclaimer] [reclaimer] +4194304 bytes (total: 4706304)
[40548113364] [INFO] [bloom::asset] [asset_bank] promoting wallpaper to gen=2 (4194304b)
[40549175733] [INFO] [bloom::reclaimer] [reclaimer] +4096 bytes (total: 4710400)
[40549922886] [INFO] [bloom::asset] [asset_bank] promoting cursor to gen=2 (4096b)
[40550756268] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4812800)
[40551445374] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSerif-Regular.ttf' to gen=2 (102400b) in slot 5
[42038018616] [INFO] [cambium] [cambium] write: binding_src=356 target=552 pred=ui.Text(487) val=578 seq=1159
[42135544803] [INFO] [bloom::ui] bloom: [bloom][ui] ENTER_UI_BUILD dirty=false root_present=true windows_seen=3 reason=fast_path_cached
[43626242682] [INFO] [clock] unix=1768966404 utc=2026-01-21 03:33:24 mono_ns=21812892238
[43643890554] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='03:33:24' tick=21812892238
[44166798126] [INFO] [bloom::ui] bloom: [bloom][ui] ENTER_UI_BUILD dirty=false root_present=true windows_seen=3 reason=fast_path_cached
[45305837643] [INFO] [bloom::perf] bloom: [PERF] f=60 work=4.0ms build=32.1ms snap=0.0ms raster=0.0ms present=0.6ms
[45307164672] [INFO] [bloom::perf] bloom: [PERF]   ops: fill=0 blit=0 text=0 | text: 0.00ms (L=0.00 R=0.00 B=0.00) glyphs=0
[46198802001] [INFO] [bloom::ui] bloom: [bloom][ui] ENTER_UI_BUILD dirty=false root_present=true windows_seen=3 reason=fast_path_cached
[46961689830] [INFO] [clock] unix=1768966406 utc=2026-01-21 03:33:26 mono_ns=23480622247
[46971604845] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='03:33:26' tick=23480622247
[47098241289] [INFO] [cambium] [cambium] write: binding_src=356 target=552 pred=ui.Text(487) val=641 seq=1268
[48200729808] [INFO] [bloom::ui] bloom: [bloom][ui] ENTER_UI_BUILD dirty=false root_present=true windows_seen=3 reason=fast_path_cached
[49555131648] [INFO] [bloom::perf] bloom: --- PERF REPORT (120 frames) ---
[49564997064] [INFO] [bloom::perf] bloom:   build                     avg= 91.34ms p50= 33.08ms p95=143.22ms max=6503ms
[49570747776] [INFO] [bloom::perf] bloom:   present                   avg=  0.66ms p50=  0.61ms p95=  1.15ms max=   1ms
[49573088202] [INFO] [bloom::perf] bloom:   raster                    avg=  2.28ms p50= 33.51ms p95= 99.48ms max=  99ms
[49574937588] [INFO] [bloom::perf] bloom:   raster.rect.total         avg=  0.33ms p50= 13.01ms p95= 14.21ms max=  14ms
[49576538946] [INFO] [bloom::perf] bloom:   ui.clone                  avg=  0.14ms p50=  3.10ms p95=  3.62ms max=   3ms
[49581199206] [INFO] [bloom::perf] bloom:   ui.diff                   avg=  0.02ms p50=  0.17ms p95=  1.17ms max=   1ms
[49582439808] [INFO] [bloom::perf] bloom:   ui.init.intern_keys       avg= 27.47ms p50=3296.65ms p95=3296.65ms max=3296ms
[49583832705] [INFO] [bloom::perf] bloom:   ui.init.intern_kinds      avg=  0.11ms p50= 12.98ms p95= 12.98ms max=  12ms
[49586032980] [INFO] [bloom::perf] bloom:   ui.layout                 avg=  7.75ms p50= 92.58ms p95=669.83ms max= 669ms
[49587453432] [INFO] [bloom::perf] bloom:   ui.layout.measure_text    avg=  0.22ms p50=  3.65ms p95= 16.91ms max=  16ms
[49588892067] [INFO] [bloom::perf] bloom:   ui.layout.solve           avg=  7.74ms p50= 92.26ms p95=669.74ms max= 669ms
[49592040300] [INFO] [bloom::perf] bloom:   ui.layout.tree_flow       avg=  7.73ms p50= 92.23ms p95=669.08ms max= 669ms
[49608051075] [INFO] [bloom::perf] bloom:   ui.lower                  avg=  0.03ms p50=  0.63ms p95=  0.99ms max=   0ms
[49609357083] [INFO] [bloom::perf] bloom:   ui.paint                  avg=  1.55ms p50= 39.47ms p95= 70.82ms max=  70ms
[49610578644] [INFO] [bloom::perf] bloom:   ui.snap                   avg=  2.74ms p50= 40.99ms p95=139.48ms max= 139ms
[49611663024] [INFO] [bloom::perf] bloom:   ui.snap.get_edges         avg=  0.61ms p50= 10.24ms p95= 21.81ms max=  21ms
[49612850199] [INFO] [bloom::perf] bloom:   ui.snap.get_kind          avg=  0.56ms p50=  9.99ms p95= 19.57ms max=  19ms
[49613860461] [INFO] [bloom::perf] bloom:   ui.snap.prop_get          avg=  0.69ms p50= 11.95ms p95= 25.19ms max=  25ms
[49615836765] [INFO] [bloom::perf] bloom:   ui.snap.read_string       avg=  0.68ms p50=  3.83ms p95= 68.28ms max=  68ms
[49617044928] [INFO] [bloom::perf] bloom:   ui.snap.traverse_all      avg=  2.74ms p50= 40.92ms p95=139.12ms max= 139ms
[49619709612] [INFO] [bloom::perf] bloom:   frame.work_ns             avg=65760148.0
[49621399938] [INFO] [bloom::perf] bloom:   raster.execute_ns         avg=331458.1
[49622529891] [INFO] [bloom::perf] bloom:   snap.syscalls.get_edges   avg=   0.6
[49623321792] [INFO] [bloom::perf] bloom:   snap.syscalls.get_kind    avg=   0.6
[49624083498] [INFO] [bloom::perf] bloom:   snap.syscalls.prop_get    avg=   0.6
[49625009379] [INFO] [bloom::perf] bloom:   snap.syscalls.read_string avg=   0.2
[49626847677] [INFO] [bloom::perf] bloom:   ui.init.syscalls.intern_keys avg=   0.2
[49628935521] [INFO] [bloom::perf] bloom:   ui.init.syscalls.intern_kinds avg=   0.1
[49630180545] [INFO] [bloom::perf] bloom:   ui.layout.cache_misses    avg=   0.0
[49631324721] [INFO] [bloom::perf] bloom:   ui.nodes                  avg=   0.6
[49632441771] [INFO] [bloom::perf] bloom:   ui.snap.nodes_total       avg=   0.6
[49633893837] [INFO] [bloom::perf] bloom:   ui.snap.prop_get.calls    avg=   0.6
[49634994486] [INFO] [bloom::perf] bloom:   ui.snap.string_cache_hit  avg=   0.4
[49636044513] [INFO] [bloom::perf] bloom:   ui.snap.string_cache_miss avg=   0.1
[49637101173] [INFO] [bloom::perf] bloom:   ui.snap.string_cache_size avg=   0.6
[49638199941] [INFO] [bloom::perf] bloom:   ui.snap.text_nodes        avg=   0.3
[49715318565] [INFO] [bloom::perf] bloom: [PERF] f=120 work=5.1ms build=87.4ms snap=0.0ms raster=0.0ms present=0.6ms
[49716987408] [INFO] [bloom::perf] bloom: [PERF]   ops: fill=0 blit=0 text=0 | text: 0.00ms (L=0.00 R=0.00 B=0.00) glyphs=0
[50201989398] [INFO] [bloom::ui] bloom: [bloom][ui] ENTER_UI_BUILD dirty=false root_present=true windows_seen=3 reason=fast_path_cached
[50294290497] [INFO] [clock] unix=1768966408 utc=2026-01-21 03:33:28 mono_ns=25146796125
[50305888017] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='03:33:28' tick=25146796125
[50366397675] [INFO] [cambium] [cambium] write: binding_src=356 target=552 pred=ui.Text(487) val=687 seq=1318
[52202825862] [INFO] [bloom::ui] bloom: [bloom][ui] ENTER_UI_BUILD dirty=false root_present=true windows_seen=3 reason=fast_path_cached
[53627925516] [INFO] [clock] unix=1768966409 utc=2026-01-21 03:33:29 mono_ns=26813621571
[53646909129] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='03:33:29' tick=26813621571
[53800177992] [INFO] [cambium] [cambium] write: binding_src=356 target=552 pred=ui.Text(487) val=693 seq=1362
[54234940452] [INFO] [bloom::ui] bloom: [bloom][ui] ENTER_UI_BUILD dirty=false root_present=true windows_seen=3 reason=fast_path_cached
[54305626782] [INFO] [bloom::perf] bloom: [PERF] f=180 work=4.5ms build=33.7ms snap=0.0ms raster=0.0ms present=0.6ms
[54307311564] [INFO] [bloom::perf] bloom: [PERF]   ops: fill=0 blit=0 text=0 | text: 0.00ms (L=0.00 R=0.00 B=0.00) glyphs=0
[56268279672] [INFO] [bloom::ui] bloom: [bloom][ui] ENTER_UI_BUILD dirty=false root_present=true windows_seen=3 reason=fast_path_cached
[57028391706] [INFO] [clock] unix=1768966411 utc=2026-01-21 03:33:31 mono_ns=28513929724
[57049136991] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='03:33:31' tick=28513929724
[57199365828] [INFO] [cambium] [cambium] write: binding_src=356 target=552 pred=ui.Text(487) val=702 seq=1408
[58304358552] [INFO] [bloom::ui] bloom: [bloom][ui] ENTER_UI_BUILD dirty=false root_present=true windows_seen=3 reason=fast_path_cached

```
</details>
