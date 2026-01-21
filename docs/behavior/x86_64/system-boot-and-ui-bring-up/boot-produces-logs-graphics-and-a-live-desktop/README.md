# ✅ Scenario: Boot produces logs, graphics, and a live desktop

> Last run: 2026-01-19 21:36:59

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 3860ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | Then I should see log messages on the terminal | ✅ | 404ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And each log message should include a monotonically increasing timestamp | ✅ | 405ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And I should see the system clock tick for several seconds in the serial console | ✅ | 1798ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |
| 5 | And I should see the wallpaper on the screen within 15 seconds | ✅ | 5945ms | <a href="./05/after.png"><img src="./05/after.png" width="150" /></a> [📜](./05/serial.log) [💾](./05/registers.txt) |
| 6 | And I should see a cursor centered on the screen | ✅ | 1068ms | <a href="./06/after.png"><img src="./06/after.png" width="150" /></a> [📜](./06/serial.log) [💾](./06/registers.txt) |
| 7 | And I should see the text "thing-os" in the top-left corner of the screen | ✅ | 462ms | <a href="./07/after.png"><img src="./07/after.png" width="150" /></a> [📜](./07/serial.log) [💾](./07/registers.txt) |
| 8 | And I should see frame count information in the top-left corner of the screen | ✅ | 468ms | <a href="./08/after.png"><img src="./08/after.png" width="150" /></a> [📜](./08/serial.log) [💾](./08/registers.txt) |
| 9 | And I should see a clock window displaying a ticking clock | ✅ | 1070ms | <a href="./09/after.png"><img src="./09/after.png" width="150" /></a> [📜](./09/serial.log) [💾](./09/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[10862341710] [CONTRACT] [kernel] thing-os kernel starting...
[10871904417] [INFO] [kernel::memory] Memory map has 64 entries
[10874080899] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[10874687901] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[10875044367] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[10875846960] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[10876184187] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[10876513791] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[10876845639] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[10877174418] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[10877539860] [INFO] [kernel::memory]   [8] 0x1780000 - 0x7866a000 (Usable)
[10877888835] [INFO] [kernel::memory]   [9] 0x7866a000 - 0x786cc000 (Reserved)
[10878279522] [INFO] [kernel::memory]   [10] 0x786cc000 - 0x7884e000 (Other)
[10878635790] [INFO] [kernel::memory]   [11] 0x7884e000 - 0x7884f000 (Reserved)
[10879005456] [INFO] [kernel::memory]   [12] 0x7884f000 - 0x788e6000 (Other)
[10879366080] [INFO] [kernel::memory]   [13] 0x788e6000 - 0x788e7000 (Reserved)
[10879738914] [INFO] [kernel::memory]   [14] 0x788e7000 - 0x78988000 (Other)
[10880098581] [INFO] [kernel::memory]   [15] 0x78988000 - 0x78989000 (Reserved)
[10880471778] [INFO] [kernel::memory]   [16] 0x78989000 - 0x789c9000 (Other)
[10880893155] [INFO] [kernel::memory]   [17] 0x789c9000 - 0x789ca000 (Reserved)
[10881284535] [INFO] [kernel::memory]   [18] 0x789ca000 - 0x78a55000 (Other)
[10881688686] [INFO] [kernel::memory]   [19] 0x78a55000 - 0x78a56000 (Reserved)
[10882052214] [INFO] [kernel::memory]   [20] 0x78a56000 - 0x78aa2000 (Other)
[10885058679] [INFO] [kernel::memory]   [21] 0x78aa2000 - 0x78aa3000 (Reserved)
[10885458804] [INFO] [kernel::memory]   [22] 0x78aa3000 - 0x78aa9000 (Other)
[10885847676] [INFO] [kernel::memory]   [23] 0x78aa9000 - 0x78aaa000 (Reserved)
[10886209323] [INFO] [kernel::memory]   [24] 0x78aaa000 - 0x78f2b000 (Other)
[10886558430] [INFO] [kernel::memory]   [25] 0x78f2b000 - 0x78f2c000 (Reserved)
[10886918988] [INFO] [kernel::memory]   [26] 0x78f2c000 - 0x793ad000 (Other)
[10887301161] [INFO] [kernel::memory]   [27] 0x793ad000 - 0x793ae000 (Reserved)
[10887687591] [INFO] [kernel::memory]   [28] 0x793ae000 - 0x796af000 (Other)
[10888037853] [INFO] [kernel::memory]   [29] 0x796af000 - 0x796b0000 (Reserved)
[10888418607] [INFO] [kernel::memory]   [30] 0x796b0000 - 0x796b9000 (Other)
[10888767087] [INFO] [kernel::memory]   [31] 0x796b9000 - 0x796ba000 (Reserved)
[10889128635] [INFO] [kernel::memory]   [32] 0x796ba000 - 0x796be000 (Other)
[10889518134] [INFO] [kernel::memory]   [33] 0x796be000 - 0x796bf000 (Reserved)
[10889878989] [INFO] [kernel::memory]   [34] 0x796bf000 - 0x796c1000 (Other)
[10890224697] [INFO] [kernel::memory]   [35] 0x796c1000 - 0x796c2000 (Reserved)
[10890592152] [INFO] [kernel::memory]   [36] 0x796c2000 - 0x796c4000 (Other)
[10890950466] [INFO] [kernel::memory]   [37] 0x796c4000 - 0x796c5000 (Reserved)
[10891355772] [INFO] [kernel::memory]   [38] 0x796c5000 - 0x796c7000 (Other)
[10891797972] [INFO] [kernel::memory]   [39] 0x796c7000 - 0x796c8000 (Reserved)
[10892165427] [INFO] [kernel::memory]   [40] 0x796c8000 - 0x796ca000 (Other)
[10892545521] [INFO] [kernel::memory]   [41] 0x796ca000 - 0x796cb000 (Reserved)
[10892924559] [INFO] [kernel::memory]   [42] 0x796cb000 - 0x796cd000 (Other)
[10893310593] [INFO] [kernel::memory]   [43] 0x796cd000 - 0x796ce000 (Reserved)
[10893673065] [INFO] [kernel::memory]   [44] 0x796ce000 - 0x796d8000 (Other)
[10894024779] [INFO] [kernel::memory]   [45] 0x796d8000 - 0x796d9000 (Reserved)
[10894394940] [INFO] [kernel::memory]   [46] 0x796d9000 - 0x796dd000 (Other)
[10894789653] [INFO] [kernel::memory]   [47] 0x796dd000 - 0x796de000 (Reserved)
[10895160936] [INFO] [kernel::memory]   [48] 0x796de000 - 0x796e0000 (Other)
[10895515653] [INFO] [kernel::memory]   [49] 0x796e0000 - 0x796e1000 (Reserved)
[10895885253] [INFO] [kernel::memory]   [50] 0x796e1000 - 0x796e3000 (Other)
[10896245019] [INFO] [kernel::memory]   [51] 0x796e3000 - 0x796e4000 (Reserved)
[10896616104] [INFO] [kernel::memory]   [52] 0x796e4000 - 0x796e8000 (Other)
[10897015470] [INFO] [kernel::memory]   [53] 0x796e8000 - 0x79757000 (Other)
[10897366458] [INFO] [kernel::memory]   [54] 0x79757000 - 0x7990d000 (Other)
[10897717182] [INFO] [kernel::memory]   [55] 0x7990d000 - 0x7a16c000 (Reserved)
[10898112852] [INFO] [kernel::memory]   [56] 0x7a16c000 - 0x7bb6c000 (Usable)
[10898468427] [INFO] [kernel::memory]   [57] 0x7bb6c000 - 0x7bb8d000 (Reserved)
[10898837565] [INFO] [kernel::memory]   [58] 0x7bb8d000 - 0x7bb91000 (Other)
[10899188718] [INFO] [kernel::memory]   [59] 0x7bb91000 - 0x7bb92000 (Reserved)
[10899560793] [INFO] [kernel::memory]   [60] 0x7bb92000 - 0x7bb94000 (Other)
[10899919866] [INFO] [kernel::memory]   [61] 0x7bb94000 - 0x7bb95000 (Reserved)
[10900292205] [INFO] [kernel::memory]   [62] 0x7bb95000 - 0x7bb97000 (Other)
[10900643226] [INFO] [kernel::memory]   [63] 0x7bb97000 - 0x7bb98000 (Reserved)
[10901253066] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[11143809534] [CONTRACT] [kernel::memory] Frame allocator initialized with 495746 free frames
[11150650995] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[11155302477] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[11156463450] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[11157154569] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[11163689328] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[11164166970] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11167139082] [INFO] [bran::arch] IOAPIC: Registers initialized
[11168294181] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[11169755685] [INFO] [bran::arch] IOAPIC: All pins masked
[11171258571] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11171930055] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11172406575] [INFO] [bran::arch] IOAPIC: Init complete
[11172996450] [CONTRACT] [kernel] Initializing global allocator...
[11545052343] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[11546080458] [CONTRACT] [kernel] Initializing SIMD...
[11547833517] [CONTRACT] [kernel] Initializing tasking...
[11553003330] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[11554764276] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[11555759853] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[11562473307] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[11563206996] [INFO] [kernel::task::scheduler]   Initializing boot task...
[11564288670] [INFO] [kernel::task::scheduler]   Creating boot task...
[11569119177] [INFO] [kernel::task::scheduler]   Creating idle task...
[11574625656] [INFO] [kernel::task::scheduler]   Boot task initialized
[11575298031] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[11576456991] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[11582867274] [INFO] [kernel::root] Spawning Root service...
[11591247855] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[11601952923] [INFO] [kernel::root::service] ROOT: started once
[12539235324] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[12540457941] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[12598636578] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[12618734238] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[12648123048] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[12683003619] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[12684765753] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[12725513559] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[12752032029] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[12759126204] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[12761783166] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[12787674603] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[12790604541] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[12791613615] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[12795182994] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[12797686506] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[12798468639] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12807832323] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12820224186] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[12821164521] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[12823726344] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[12824587050] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[12833902323] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[12834640401] [CONTRACT] [kernel] Spawning init process...
[12836238426] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[12870920997] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62205600 ticks/sec), init_cnt=622056 for 100Hz
[12872958615] [CONTRACT] [kernel] Entering scheduler loop.
[12883932336] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[12888932100] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367676144 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563776488
[12899669079] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[12901112532] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[12901910505] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[12902642610] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[12925985886] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[12929923941] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[12933206253] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[12934018020] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[12939236211] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[12941764902] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[12942555813] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[12945692958] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[12946453179] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[12947207955] [INFO] [sprout::devtree] SPROUT: build() called
[12947850993] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[12952510164] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[12953433075] [INFO] [sprout] SPROUT: About to create Supervisor...
[12954127857] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[12954919098] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[12955582068] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[12960565794] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[12999630138] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[13005100548] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[13008459849] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[13011580956] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[13013835846] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[13016746314] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[13021731162] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[13025018292] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[13028229390] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[13031544009] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[13034817246] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[13038752133] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[13042471728] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[13045577094] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[13050039354] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[13053074298] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[13056099441] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[13059066240] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[13061946414] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[13065261792] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[13068626538] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[13071965544] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[13076504430] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[13079612238] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[13082989920] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[13086118947] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[13089235071] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[13092323046] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[13095534738] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[13098575259] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[13103169156] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[13106985012] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[13110276762] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[13113484890] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[13116773538] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[13120169139] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[13123551342] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[13126896717] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[13131701880] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[13135334454] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[13138492323] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[13141734540] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[13145114994] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[13147462845] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[13161629085] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[13236431934] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[13237617558] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[13239524859] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[13240938645] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[13241747640] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[13242599040] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[13245191949] [INFO] [kernel::task::loader] Loading module: /boot/clock
[13245796443] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13246924548] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13250749347] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[13251352323] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[13252703013] [INFO] [kernel::task::loader] Segment: vaddr=204ee8 exec=false
[13253306517] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13260246813] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[13263129858] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[13264077750] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[13264962084] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13266580932] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13271017188] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[13271650095] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[13272498459] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[13273098102] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13279230030] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[13280457399] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[13281425058] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[13281931113] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13282930188] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13293687198] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[13294318818] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[13298809755] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[13299454839] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[13305565350] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[13306340586] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[13307793741] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[13308313194] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13309355763] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13313064369] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[13313673021] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[13315009422] [INFO] [kernel::task::loader] Segment: vaddr=204c88 exec=false
[13315604115] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13322044692] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[13323075942] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[13340286300] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13341615870] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367986928 RFLAGS_BEFORE=134 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563776488
[13345763112] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[13346977050] [INFO] [clock] starting clock publisher
[13357489200] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13358718219] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368009968 RFLAGS_BEFORE=134 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563776488
[13363874502] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ab8
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[13365253737] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368026352 RFLAGS_BEFORE=134 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563776488
[13367433321] [ERROR] [INGESTD] Starting...
[13371264456] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ae8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13372638939] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368053488 RFLAGS_BEFORE=134 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563776488
[13375588182] [INFO] [cambium] cambium starting (v3: catch-up then stream)...
[13382240421] [INFO] [clock] Clock thing created: 356
[13382968599] [INFO] [clock] Waiting for UI Root (Compositor)...
[13387885104] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[13389123000] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[13390209525] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[13391566683] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[13414702191] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[13423518141] [ERROR] [INGESTD] Watch active. Loop start.
[13432419462] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[13433785728] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[13665348312] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[13666110381] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[13667217399] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13670306199] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[13670943726] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13671992169] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[13672605771] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13679523726] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[13686247113] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[13687434981] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368106080 RFLAGS_BEFORE=134 CR3_BEFORE=54927360 fs_base=0 gs_base=18446744071563776488
[13691923542] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[13709334078] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[13718577939] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[13728176979] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[13729475958] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[13730038443] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[13731416589] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13734822882] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[13735445064] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13736788593] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[13737483771] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[13743928242] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[13745096574] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[13746507753] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[13747714266] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[13749129570] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[13750305426] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[13750862928] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13751920677] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13754389308] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[13754994330] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[13756012545] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[13756639215] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[13764009831] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[13765526445] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[13766098203] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13767238650] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13770383352] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[13770986922] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13772276694] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[13772868978] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[13779441093] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[13781223984] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[13782304503] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[13782835671] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13783846296] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13786897245] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[13787719539] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13788850812] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[13789475403] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13796242185] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[13812153069] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044678
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[13813577481] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368129744 RFLAGS_BEFORE=130 CR3_BEFORE=55033856 fs_base=0 gs_base=18446744071563776488
[13817586684] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[13818543420] [INFO] [rtc_cmos] Starting... arg=db
[13820181078] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[13823949645] [INFO] [rtc_cmos] RTC: 2026-01-20 05:37:05 = 1768887425 unix_secs
[13825213842] [INFO] [kernel::time] System clock anchored: unix_secs=1768887425, mono_ns=6912355032, offset=1768887418087644968ns
[13826521500] [INFO] [rtc_cmos] System clock anchored
[13843749942] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[13846239792] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004b958
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[13847359614] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368164272 RFLAGS_BEFORE=130 CR3_BEFORE=55136256 fs_base=0 gs_base=18446744071563776488
[13850841279] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[13852811874] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[13853983077] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[13856992479] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[13862519583] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ee60
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[13879226955] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368181680 RFLAGS_BEFORE=130 CR3_BEFORE=55230464 fs_base=0 gs_base=18446744071563776488
[13883067231] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[13884031062] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[13890026832] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00978a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[13891550079] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368207552 RFLAGS_BEFORE=134 CR3_BEFORE=55332864 fs_base=0 gs_base=18446744071563776488
[13896584955] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[13907649657] [INFO] [bristle] bristle: registered in graph as svc.Input (id=457)
[13917781515] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=431 backend=BootFB
[13919144283] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[13919697990] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13920996507] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13982154219] [INFO] [kernel::task::loader] Segment: vaddr=2615d0 exec=false
[13983169794] [INFO] [kernel::task::loader]   Overlap at 261000: merging perms to r=true w=false x=true
[13991118504] [INFO] [kernel::task::loader] Segment: vaddr=26d1a0 exec=false
[13991847738] [INFO] [kernel::task::loader]   Overlap at 26d000: merging perms to r=true w=true x=true
[13998982437] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[14000500404] [INFO] [kernel::task::loader] Loading module: /boot/echo
[14001197661] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14002295241] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14005558776] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[14006234781] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14007351237] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[14008391265] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[14014663245] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[14015719740] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[14016519330] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[14029180308] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014c50
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1af
[14030513541] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368250144 RFLAGS_BEFORE=130 CR3_BEFORE=55439360 fs_base=0 gs_base=18446744071563776488
[14033797965] [INFO] [bloom::logging] bloom: logging initialized
[14038104366] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ba00
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[14039397471] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368267616 RFLAGS_BEFORE=134 CR3_BEFORE=55975936 fs_base=0 gs_base=18446744071563776488
[14042991303] [INFO] [echo] echo: online (handle=12)
[14043920088] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[14050954995] [INFO] [bloom] bloom: [bloom] Bootstrapped via Bytespace ID=431
[14055027327] [INFO] [bloom] bloom: [bloom] starting (arg_req=1 arg_resp=4 bristle=10 font_svc=0)
[14065691673] [INFO] [bloom] bloom: [bloom] spawned wallpaper_loader thread (tid=15)
[14072066415] [INFO] [bloom] bloom: [bloom] spawned cursor_loader thread (tid=16)
[14077977111] [INFO] [bloom] bloom: [bloom] spawned font_loader thread (tid=17)
[14078703342] [INFO] [bloom] bloom: [bloom] discovering compositor target...
T:AEF0 [14085318984] [INFO] [bloom] bloom: [wallpaper_loader] thread started
T:AB20 [14087504277] [INFO] [bloom] bloom: [cursor_loader] thread started
T:A0B0 [14089464543] [INFO] [bloom] bloom: [font_loader] thread started
[14090519454] [INFO] [bloom] bloom: [font_loader] scanning boot modules for fonts...
[14101591746] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[14102513799] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[14103595143] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
[14107545903] [INFO] [bloom] bloom: [font_loader] found 42 boot modules
[14202646722] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/DSEG7Classic-Regular.ttf'
[14209968498] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=179 size=23272 name='/assets/fonts/DSEG7Classic-Regular.ttf'
[14238979788] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:35F0 [14244731424] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[14245961598] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[14249760492] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/Hack-Regular.ttf'
[14254659342] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[14255571330] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[14278643115] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[14282469399] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[14289994422] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=182 size=309408 name='/assets/fonts/Hack-Regular.ttf'
[14294667255] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[14296978608] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSans-Regular.ttf'
[14301872145] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[14302676421] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[14914490019] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=185 size=569208 name='/assets/fonts/NotoSans-Regular.ttf'
[14920004682] [INFO] [bloom] bloom: [wallpaper_loader] searching for wallpaper...
[14921056722] [INFO] [bloom] bloom: [wallpaper_loader] trying: /assets/wallpapers/clouds.bmp
[14926750410] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[14927838288] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[14928732984] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[14933064333] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol-Regular.ttf'
[14940687729] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[14941565694] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[15277031352] [INFO] [bloom] bloom: [cursor_loader] searching for cursor...
[15278199057] [INFO] [bloom] bloom: [cursor_loader] trying: /assets/cursors/plain/Normal.cur
[15593191647] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=188 size=258156 name='/assets/fonts/NotoSansSymbol-Regular.ttf'
[15940818531] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[16577005467] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=191 size=656852 name='/assets/fonts/NotoSansSymbol2-Regular.ttf'
[17135559969] [INFO] [bloom::compositor] bloom: compositor bytespace 376 (1280x720 stride=5120 format=1)
[17461656267] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSerif-Regular.ttf'
[17963746977] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[17965408395] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[17966809146] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (258156 bytes)
[17971287873] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=194 size=616196 name='/assets/fonts/NotoSerif-Regular.ttf'
[17975291103] [INFO] [bloom::compositor] bloom: display backend: BootFB
[17980764516] [INFO] [ps2_mouse] ps2_mouse: init done
[17981552523] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[17982538596] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[17983376169] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[17988289803] [INFO] [bloom::asset] [asset_bank] mapped at 0x10463000
[17989159716] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[18119491698] [INFO] [bloom] bloom: [font_loader] immediate scan complete, entering watch loop
[18123010884] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[18455549706] [INFO] [bloom] bloom: [bloom] compositor target: 1280x720 @ 0x104a3000 backend=BootFB
[18457264023] [INFO] [bloom] bloom: [bloom] presenter: driver (req=1 resp=4)
[18459130734] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[18774608655] [INFO] [stem::ui] UiBuilder: created root 546
[18775792761] [INFO] [bloom] bloom: [bloom] created UI root node: 546
[18786183405] [INFO] [display_bootfb] display_bootfb: bound bytespace 376
[19102076367] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd5a0
[19103320896] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[19104446427] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19105830447] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[19430208336] [INFO] [bloom] bloom: [bloom] ui watch opened UNFILTERED (userspace filter pred=UI_TEXT id=520)
[19763971656] [INFO] [clock] Found UI Root: 546 (attempt 4)
[21440646216] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[21441871869] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[21442796232] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 191 (656852 bytes)
[21455092692] [INFO] [bloom::asset] [asset_bank] mapped at 0x10bab000
[21455921520] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[22411346925] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c1b40
[22412511000] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[22413339696] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22414160142] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[22739767479] [INFO] [bloom] bloom: [font_loader] watch opened (id=570)
[24721450710] [INFO] [bloom] bloom: [cursor_loader] SUCCESS: found candidate '/assets/cursors/plain/Normal.cur', enqueuing load
[24723428961] [INFO] [bloom] bloom: [cursor_loader] thread done, sleeping forever
[26021422647] [INFO] [bloom] bloom: [wallpaper_loader] SUCCESS: found candidate '/assets/wallpapers/clouds.bmp', enqueuing load
[27821302740] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[27822727251] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol2-Regular.ttf' in slot 4
[27957323691] [INFO] [bloom] bloom: [wallpaper_loader] thread done, sleeping forever
[27963232308] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 194 (616196 bytes)
[27971469108] [INFO] [bloom::asset] [asset_bank] mapped at 0x10c4c000
[27972323676] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSerif-Regular.ttf'...
[30250969848] [INFO] [clock] Binding created: 590 (source=356 target=575)
[30251958528] [INFO] [clock] CLOCK: Entering main loop, publishing to thing_id=356
[30254628129] [INFO] [clock] unix=1768887433 utc=2026-01-20 05:37:13 mono_ns=15126531024
[31595945601] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:37:13' tick=15126531024
[32252297781] [INFO] [cambium] Found 1 bindings
[33235683525] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fedd0
[33236655672] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[33237584556] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[33238524297] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x4 kind=0 pred=0 subj_lo=356
[33562909644] [INFO] [cambium] Opened watch 609 for source 356 (binding 590, start_seq=0)
[33594319143] [INFO] [cambium] CATCH-UP: Draining 1 watches for historical events...
[33924133584] [INFO] [cambium] cambium: drain complete payloads=0 overflows=0 last_seq=none
[33925396230] [INFO] [cambium] Entering event loop with 1 bindings (v3)
[34908212724] [INFO] [clock] unix=1768887435 utc=2026-01-20 05:37:15 mono_ns=17453894865
[36218540523] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:37:15' tick=17453894865
[37521020262] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[37522425930] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSerif-Regular.ttf' in slot 5
[37523381841] [INFO] [bloom::asset] [asset_bank] load_cursor_immediate: /assets/cursors/plain/Normal.cur
[37642143759] [INFO] [bloom::asset] [asset_bank] mapping bytespace 152 (4286 bytes)
[37647891402] [INFO] [bloom::asset] [asset_bank] mapped to 0x10ce3000
[37648749798] [INFO] [bloom::asset] [asset_bank] checking CUR header: len=4286
[37649525694] [INFO] [bloom::asset] [asset_bank] valid CUR header detected
[37650560904] [INFO] [bloom::asset] [asset_bank] CUR: hotspot=(0, 0), img_size=4264, offset=22
[37651443324] [INFO] [bloom::asset] [asset_bank] decoding embedded DIB at offset 22...
[37653194832] [INFO] [bloom::asset] [asset_bank] SUCCESS: cursor DIB decoded 32x32
[37661642964] [INFO] [bloom::asset] [asset_bank] publish_cursor (pending)
[37662472353] [INFO] [bloom::asset] [asset_bank] cursor: Static frame 32x32 hotspot (0, 0)
[37663401732] [INFO] [bloom::asset] [asset_bank] load_wallpaper_immediate: /assets/wallpapers/clouds.bmp
[37789151664] [INFO] [bloom::asset] [asset_bank] mapping bytespace 170 (3145782 bytes)
[37799199372] [INFO] [bloom::asset] [asset_bank] mapped to 0x10ce5000
[37800025164] [INFO] [bloom::asset] [asset_bank] decoding BMP...
[38208976740] [INFO] [bloom::asset] [asset_bank] BMP decoded: 1024x1024
[38662584081] [INFO] [bloom::asset] [asset_bank] bytespace unmapped
[38663559099] [INFO] [bloom::asset] [asset_bank] publish_wallpaper (pending): 1024x1024
[39802025274] [INFO] [cambium] [cambium] write: binding_src=356 target=575 pred=ui.Text(520) val=598 seq=1141
[39807812451] [INFO] [cambium] Updated target 575 with value 598 (seq=1141)
[39824620902] [INFO] [cambium] Updated target 575 with value 617 (seq=1153)
[40215196956] [INFO] [clock] unix=1768887438 utc=2026-01-20 05:37:18 mono_ns=20107422835
[40231708671] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:37:18' tick=20107422835
[40324316142] [INFO] [cambium] Updated target 575 with value 648 (seq=1165)
[41173237413] [INFO] [bloom] bloom: [font_loader] drain complete: payloads=1045 overflows=29
[41181788637] [INFO] [bloom] bloom: [bloom] ui watch drained: 1049 batches
[41212264896] [INFO] [bloom] bloom: [bloom] entering transactional frame loop (acquire -> build -> present)
[41213459166] [INFO] [bloom] bloom: [bloom] reclaimer: budget=33554432 bytes
[41215055574] [INFO] [bloom::reclaimer] [reclaimer] +4194304 bytes (total: 4194304)
[41215858662] [INFO] [bloom::asset] [asset_bank] promoting wallpaper to gen=1 (4194304b)
[41216870970] [INFO] [bloom::reclaimer] [reclaimer] +4096 bytes (total: 4198400)
[41217659835] [INFO] [bloom::asset] [asset_bank] promoting cursor to gen=1 (4096b)
[41218743291] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4300800)
[41219541297] [INFO] [bloom::asset] [asset_bank] promoting font 'DSEG7Classic-Regular.ttf' to gen=1 (102400b) in slot 0
[41220569247] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4403200)
[41221300098] [INFO] [bloom::asset] [asset_bank] promoting font 'Hack-Regular.ttf' to gen=1 (102400b) in slot 1
[41222105595] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4505600)
[41222773647] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=1 (102400b) in slot 2
[41223679101] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4608000)
[41224384080] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol-Regular.ttf' to gen=1 (102400b) in slot 3
[41225235282] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4710400)
[41226030846] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol2-Regular.ttf' to gen=1 (102400b) in slot 4
[41226904818] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4812800)
[41227605672] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSerif-Regular.ttf' to gen=1 (102400b) in slot 5
[41229923394] [INFO] [bloom] bloom: [bloom] frame 1: wallpaper now visible (gen=1)
[41231046417] [INFO] [bloom] bloom: [bloom] frame 1: cursor now visible (gen=1)
[41234854947] [INFO] [bloom] bloom: [bloom] frame 1: fonts available (mode=legacy)
[41683288713] [INFO] [bloom::ui] bloom: [bloom][ui] ENTER_UI_BUILD dirty=true root_present=true windows_seen=3 nodes=10 (text=5 text_str=7) reason=full_build
[41938060230] [INFO] [bloom::ui] bloom: [bloom::ui] WARN: slow UI run: total=341.0ms (snap=214.2ms diff=1.5ms clone=2.2ms layout=87.6ms paint=34.6ms lower=1.0ms)
[42113275710] [INFO] [bloom] bloom: [CONTRACT] [bloom] First frame rendered - compositor ready
[42151007613] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.8ms
[42350364177] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1175
[42938022039] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.5ms
[43492860543] [INFO] [clock] unix=1768887439 utc=2026-01-20 05:37:19 mono_ns=21746237172
[43505403612] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:37:19' tick=21746237172
[43566505752] [INFO] [cambium] [cambium] write: binding_src=356 target=575 pred=ui.Text(520) val=686 seq=1179
[43571302269] [INFO] [cambium] Updated target 575 with value 686 (seq=1179)
[43671677808] [INFO] [bloom] bloom: [bloom][damage] rects=1 frame=18
[43889716134] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1181
[44579018022] [INFO] [bloom::ui] bloom: [bloom::ui] WARN: slow UI run: total=343.8ms (snap=221.9ms diff=0.3ms clone=1.7ms layout=80.7ms paint=38.5ms lower=0.7ms)
[44627461725] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (2 rects) = 20.8ms
[44662988931] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.8ms
[44730266955] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.3ms
[44764729020] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 11.5ms
[44830193265] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 11.2ms
[44970433926] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 14.0ms
[45001224312] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.0ms
[45189573693] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[45226134657] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[45700104021] [INFO] [bloom] bloom: [bloom][damage] rects=1 frame=38
[45876779520] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.9ms
[45909647256] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.2ms
[45947528880] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 12.5ms
[46018954014] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 14.0ms
[46057528869] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 12.5ms
[46205717712] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[46469249871] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[46597312509] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.0ms
[46630532586] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[46696802295] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.0ms
[46769866605] [INFO] [clock] unix=1768887441 utc=2026-01-20 05:37:21 mono_ns=23384717697
[46833365535] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:37:21' tick=23384717697
[46865176215] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[46872359424] [INFO] [cambium] [cambium] write: binding_src=356 target=575 pred=ui.Text(520) val=715 seq=1182
[46878616917] [INFO] [cambium] Updated target 575 with value 715 (seq=1182)
[46905117105] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1184
[47562185913] [INFO] [bloom::ui] bloom: [bloom::ui] WARN: slow UI run: total=327.8ms (snap=215.0ms diff=0.1ms clone=1.5ms layout=74.7ms paint=35.9ms lower=0.6ms)
[47605131882] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (2 rects) = 19.3ms
[48098491089] [INFO] [bloom] bloom: [bloom][damage] rects=1 frame=69
[48283996101] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.9ms
[48335494449] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.0ms
[48972731709] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 14.5ms
[49005129855] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.7ms
[49058602593] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 11.0ms
[49220950317] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.2ms
[49319322129] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[49353318168] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[49386216495] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.0ms
[49517086410] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.4ms
[49551272793] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.8ms
[50098888620] [INFO] [bloom] bloom: [bloom][damage] rects=1 frame=109
[50181611304] [INFO] [clock] unix=1768887443 utc=2026-01-20 05:37:23 mono_ns=25090538764
[50287570608] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 11.1ms
[50297909376] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:37:23' tick=25090538764
[50346357732] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 14.0ms
[50377429575] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.3ms
[50441656452] [INFO] [cambium] [cambium] write: binding_src=356 target=575 pred=ui.Text(520) val=738 seq=1185
[50448228732] [INFO] [cambium] Updated target 575 with value 738 (seq=1185)
[50452857642] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1187
[51211132170] [INFO] [bloom::ui] bloom: [bloom::ui] WARN: slow UI run: total=378.4ms (snap=244.2ms diff=0.1ms clone=2.8ms layout=87.9ms paint=42.8ms lower=0.6ms)
[51254908452] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (2 rects) = 19.4ms
[51486610461] [INFO] [bloom] bloom: [bloom] PERF: 120 frames avg: total=38.08ms build=23.08ms (ui=21.41ms) raster=14.06ms present=0.80ms input=0.14ms
[51581138136] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.0ms
[52117192413] [INFO] [bloom] bloom: [bloom][damage] rects=1 frame=125
[52565633373] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.4ms
[52612178850] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 14.5ms
[52657582593] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 14.5ms
[52690307670] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.5ms
[52729130850] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.5ms
[52763546946] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.4ms
[52809999792] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 14.9ms
[52852491780] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 12.9ms
[52886538177] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.9ms
[52928424285] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.2ms
[53122409868] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[53222400792] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 11.1ms
[53351453925] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.0ms
[53383942953] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[53449420398] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[53556604761] [INFO] [clock] unix=1768887444 utc=2026-01-20 05:37:24 mono_ns=26778024454
[53589991620] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.3ms
[53601753579] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:37:24' tick=26778024454
[53727019599] [INFO] [cambium] [cambium] write: binding_src=356 target=575 pred=ui.Text(520) val=768 seq=1188
[53733193899] [INFO] [cambium] Updated target 575 with value 768 (seq=1188)
[54422579739] [INFO] [bloom::ui] bloom: [bloom::ui] WARN: slow UI run: total=381.5ms (snap=253.5ms diff=0.1ms clone=1.6ms layout=86.4ms paint=39.3ms lower=0.6ms)
[54467159934] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (2 rects) = 19.9ms
[54481168566] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1190
[55167770691] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.4ms
[55417476774] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.5ms
[55483824891] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.2ms
[56340448758] [INFO] [bloom::ui] bloom: [bloom][ui] ENTER_UI_BUILD dirty=false root_present=true windows_seen=3 reason=fast_path_cached
[56564099460] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.1ms
[56631013725] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.7ms
[56671152714] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 11.2ms
[56728752003] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.4ms
[56794898160] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.0ms
[56831178723] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.4ms
[56867753250] [INFO] [clock] unix=1768887446 utc=2026-01-20 05:37:26 mono_ns=28433574906
[56908649721] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:37:26' tick=28433574906
[57000171525] [INFO] [cambium] [cambium] write: binding_src=356 target=575 pred=ui.Text(520) val=788 seq=1191
[57010191909] [INFO] [cambium] Updated target 575 with value 788 (seq=1191)
[57014240778] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1193
[57703724331] [INFO] [bloom::ui] bloom: [bloom::ui] WARN: slow UI run: total=344.0ms (snap=226.2ms diff=0.1ms clone=2.8ms layout=77.4ms paint=36.9ms lower=0.6ms)
[57752536479] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (2 rects) = 21.9ms
[58374749862] [INFO] [bloom::ui] bloom: [bloom][ui] ENTER_UI_BUILD dirty=false root_present=true windows_seen=3 reason=fast_path_cached
[59681315430] [INFO] [bloom] bloom: [bloom] PERF: 120 frames avg: total=29.58ms build=16.17ms (ui=14.55ms) raster=12.61ms present=0.77ms input=0.14ms
[60179092314] [INFO] [clock] unix=1768887448 utc=2026-01-20 05:37:28 mono_ns=30089332779
[60192162591] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:37:28' tick=30089332779
[60406464954] [INFO] [bloom::ui] bloom: [bloom][ui] ENTER_UI_BUILD dirty=false root_present=true windows_seen=3 reason=fast_path_cached
[60432980487] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.9ms
[60442880487] [INFO] [cambium] [cambium] write: binding_src=356 target=575 pred=ui.Text(520) val=799 seq=1194
[60448844841] [INFO] [cambium] Updated target 575 with value 799 (seq=1194)
[60475992885] [INFO] [bloom::raster] bloom: [bloom::raster] WARN: slow rasterize (1 rects) = 10.6ms
[60489405669] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1196

```
</details>
