# ✅ Scenario: Boot produces logs, graphics, and a live desktop

> Last run: 2026-01-20 17:35:35

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 3620ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | Then I should see log messages on the terminal | ✅ | 381ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And each log message should include a monotonically increasing timestamp | ✅ | 390ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And I should see the system clock tick for several seconds in the serial console | ✅ | 1882ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |
| 5 | And I should see the wallpaper on the screen within 15 seconds | ✅ | 7434ms | <a href="./05/after.png"><img src="./05/after.png" width="150" /></a> [📜](./05/serial.log) [💾](./05/registers.txt) |
| 6 | And I should see a cursor centered on the screen | ✅ | 1040ms | <a href="./06/after.png"><img src="./06/after.png" width="150" /></a> [📜](./06/serial.log) [💾](./06/registers.txt) |
| 7 | And I should see the text "thing-os" in the top-left corner of the screen | ✅ | 452ms | <a href="./07/after.png"><img src="./07/after.png" width="150" /></a> [📜](./07/serial.log) [💾](./07/registers.txt) |
| 8 | And I should see frame count information in the top-left corner of the screen | ✅ | 453ms | <a href="./08/after.png"><img src="./08/after.png" width="150" /></a> [📜](./08/serial.log) [💾](./08/registers.txt) |
| 9 | And I should see a clock window displaying a ticking clock | ✅ | 1018ms | <a href="./09/after.png"><img src="./09/after.png" width="150" /></a> [📜](./09/serial.log) [💾](./09/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[10325949051] [CONTRACT] [kernel] thing-os kernel starting...
[10335220203] [INFO] [kernel::memory] Memory map has 64 entries
[10337434239] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[10338039888] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[10338663192] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[10339020318] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[10339334049] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[10339705497] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[10340019129] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[10340325534] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[10340673354] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78661000 (Usable)
[10341000450] [INFO] [kernel::memory]   [9] 0x78661000 - 0x786c3000 (Reserved)
[10341391401] [INFO] [kernel::memory]   [10] 0x786c3000 - 0x78845000 (Other)
[10341723513] [INFO] [kernel::memory]   [11] 0x78845000 - 0x78846000 (Reserved)
[10342061763] [INFO] [kernel::memory]   [12] 0x78846000 - 0x788dd000 (Other)
[10342394040] [INFO] [kernel::memory]   [13] 0x788dd000 - 0x788de000 (Reserved)
[10342738296] [INFO] [kernel::memory]   [14] 0x788de000 - 0x7897f000 (Other)
[10343070573] [INFO] [kernel::memory]   [15] 0x7897f000 - 0x78980000 (Reserved)
[10343419317] [INFO] [kernel::memory]   [16] 0x78980000 - 0x789c0000 (Other)
[10343753904] [INFO] [kernel::memory]   [17] 0x789c0000 - 0x789c1000 (Reserved)
[10344100734] [INFO] [kernel::memory]   [18] 0x789c1000 - 0x78a4c000 (Other)
[10344435915] [INFO] [kernel::memory]   [19] 0x78a4c000 - 0x78a4d000 (Reserved)
[10344819474] [INFO] [kernel::memory]   [20] 0x78a4d000 - 0x78a99000 (Other)
[10345153071] [INFO] [kernel::memory]   [21] 0x78a99000 - 0x78a9a000 (Reserved)
[10345495875] [INFO] [kernel::memory]   [22] 0x78a9a000 - 0x78aa0000 (Other)
[10345826898] [INFO] [kernel::memory]   [23] 0x78aa0000 - 0x78aa1000 (Reserved)
[10346167821] [INFO] [kernel::memory]   [24] 0x78aa1000 - 0x78f22000 (Other)
[10346498217] [INFO] [kernel::memory]   [25] 0x78f22000 - 0x78f23000 (Reserved)
[10346840064] [INFO] [kernel::memory]   [26] 0x78f23000 - 0x793a4000 (Other)
[10347170592] [INFO] [kernel::memory]   [27] 0x793a4000 - 0x793a5000 (Reserved)
[10347511911] [INFO] [kernel::memory]   [28] 0x793a5000 - 0x796a6000 (Other)
[10347841746] [INFO] [kernel::memory]   [29] 0x796a6000 - 0x796a7000 (Reserved)
[10348220256] [INFO] [kernel::memory]   [30] 0x796a7000 - 0x796b0000 (Other)
[10348551444] [INFO] [kernel::memory]   [31] 0x796b0000 - 0x796b1000 (Reserved)
[10348895271] [INFO] [kernel::memory]   [32] 0x796b1000 - 0x796b5000 (Other)
[10349228571] [INFO] [kernel::memory]   [33] 0x796b5000 - 0x796b6000 (Reserved)
[10349572926] [INFO] [kernel::memory]   [34] 0x796b6000 - 0x796b8000 (Other)
[10349904213] [INFO] [kernel::memory]   [35] 0x796b8000 - 0x796b9000 (Reserved)
[10350248337] [INFO] [kernel::memory]   [36] 0x796b9000 - 0x796bb000 (Other)
[10350580317] [INFO] [kernel::memory]   [37] 0x796bb000 - 0x796bc000 (Reserved)
[10350924276] [INFO] [kernel::memory]   [38] 0x796bc000 - 0x796be000 (Other)
[10351277277] [INFO] [kernel::memory]   [39] 0x796be000 - 0x796bf000 (Reserved)
[10351621830] [INFO] [kernel::memory]   [40] 0x796bf000 - 0x796c1000 (Other)
[10351954173] [INFO] [kernel::memory]   [41] 0x796c1000 - 0x796c2000 (Reserved)
[10352297802] [INFO] [kernel::memory]   [42] 0x796c2000 - 0x796c4000 (Other)
[10352628660] [INFO] [kernel::memory]   [43] 0x796c4000 - 0x796c5000 (Reserved)
[10352972388] [INFO] [kernel::memory]   [44] 0x796c5000 - 0x796cf000 (Other)
[10353304533] [INFO] [kernel::memory]   [45] 0x796cf000 - 0x796d0000 (Reserved)
[10353648360] [INFO] [kernel::memory]   [46] 0x796d0000 - 0x796d4000 (Other)
[10353981396] [INFO] [kernel::memory]   [47] 0x796d4000 - 0x796d5000 (Reserved)
[10354326180] [INFO] [kernel::memory]   [48] 0x796d5000 - 0x796d7000 (Other)
[10354694889] [INFO] [kernel::memory]   [49] 0x796d7000 - 0x796d8000 (Reserved)
[10355040300] [INFO] [kernel::memory]   [50] 0x796d8000 - 0x796da000 (Other)
[10355371884] [INFO] [kernel::memory]   [51] 0x796da000 - 0x796db000 (Reserved)
[10355718417] [INFO] [kernel::memory]   [52] 0x796db000 - 0x796df000 (Other)
[10356050232] [INFO] [kernel::memory]   [53] 0x796df000 - 0x79757000 (Other)
[10356382410] [INFO] [kernel::memory]   [54] 0x79757000 - 0x7990d000 (Other)
[10356714324] [INFO] [kernel::memory]   [55] 0x7990d000 - 0x7a16c000 (Reserved)
[10357058613] [INFO] [kernel::memory]   [56] 0x7a16c000 - 0x7bb6c000 (Usable)
[10357394982] [INFO] [kernel::memory]   [57] 0x7bb6c000 - 0x7bb8d000 (Reserved)
[10357737225] [INFO] [kernel::memory]   [58] 0x7bb8d000 - 0x7bb91000 (Other)
[10358104944] [INFO] [kernel::memory]   [59] 0x7bb91000 - 0x7bb92000 (Reserved)
[10358592783] [INFO] [kernel::memory]   [60] 0x7bb92000 - 0x7bb94000 (Other)
[10358983470] [INFO] [kernel::memory]   [61] 0x7bb94000 - 0x7bb95000 (Reserved)
[10359386400] [INFO] [kernel::memory]   [62] 0x7bb95000 - 0x7bb97000 (Other)
[10359820812] [INFO] [kernel::memory]   [63] 0x7bb97000 - 0x7bb98000 (Reserved)
[10360354587] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[10598742099] [CONTRACT] [kernel::memory] Frame allocator initialized with 495737 free frames
[10605455487] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[10609872900] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[10610889630] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[10611555339] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[10618499298] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[10618972221] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[10621858962] [INFO] [bran::arch] IOAPIC: Registers initialized
[10623081414] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[10624490217] [INFO] [bran::arch] IOAPIC: All pins masked
[10625777118] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[10626352572] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[10626793980] [INFO] [bran::arch] IOAPIC: Init complete
[10627356498] [CONTRACT] [kernel] Initializing global allocator...
[10976494518] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[10977191049] [CONTRACT] [kernel] Initializing SIMD...
[10978493955] [CONTRACT] [kernel] Initializing tasking...
[10982866422] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[10984392936] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[10984941264] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[10990277166] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[10990675872] [INFO] [kernel::task::scheduler]   Initializing boot task...
[10991367057] [INFO] [kernel::task::scheduler]   Creating boot task...
[10995622374] [INFO] [kernel::task::scheduler]   Creating idle task...
[11000306526] [INFO] [kernel::task::scheduler]   Boot task initialized
[11000693385] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[11001502875] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[11006948766] [INFO] [kernel::root] Spawning Root service...
[11014064160] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[11022467577] [INFO] [kernel::root::service] ROOT: started once
[11896034040] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[11896898739] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[11941686867] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[11960826240] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[11988199212] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[12021835353] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[12024093807] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[12063369483] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[12086982072] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[12094029585] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[12097620249] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[12120242244] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[12123083610] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[12124117170] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[12127469772] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[12129835806] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[12130794291] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12137975190] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12150026031] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[12150970128] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[12153360285] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[12154179048] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[12161774064] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[12162374796] [CONTRACT] [kernel] Spawning init process...
[12163892631] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[12198431520] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62176600 ticks/sec), init_cnt=621766 for 100Hz
[12199872432] [CONTRACT] [kernel] Entering scheduler loop.
[12209001750] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[12213533046] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367676144 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563776488
[12223569237] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[12224868744] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[12225637347] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[12226333152] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[12245541990] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[12248924127] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[12251842251] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[12252568284] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[12255760935] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[12257908872] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[12258665001] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[12261621999] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[12263283582] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[12264139899] [INFO] [sprout::devtree] SPROUT: build() called
[12264771090] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[12269031489] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[12269793756] [INFO] [sprout] SPROUT: About to create Supervisor...
[12270463260] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[12271238892] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[12271938492] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[12276466059] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[12309977394] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[12314108400] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[12317377050] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[12320238612] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[12322362393] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[12325102119] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[12328378359] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[12331551837] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[12334472997] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[12337531701] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[12340481274] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[12344136222] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[12347585976] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[12350438859] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[12353266068] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[12356115057] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[12358878972] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[12361692057] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[12364738584] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[12367935162] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[12370842396] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[12373962348] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[12376902153] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[12379808298] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[12382883469] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[12385825056] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[12388784760] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[12391766343] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[12395004633] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[12398339448] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[12401325651] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[12404418081] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[12407319507] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[12410331384] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[12413286831] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[12416412789] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[12419554851] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[12423027903] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[12425924148] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[12429019878] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[12431941632] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[12434921796] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[12437945652] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[12440070819] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[12451567689] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[12512983362] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[12513796482] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[12515550531] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[12516542874] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[12517271349] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[12518067705] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[12520437534] [INFO] [kernel::task::loader] Loading module: /boot/clock
[12520953918] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12521993616] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12525545934] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[12526182438] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[12527531346] [INFO] [kernel::task::loader] Segment: vaddr=204ee8 exec=false
[12528102477] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12584214291] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12598746666] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367986928 RFLAGS_BEFORE=134 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563776488
[12603558429] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[12604629048] [INFO] [clock] starting clock publisher
[12609696231] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[12612624915] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[12613639335] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[12614203998] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12615223500] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12619107039] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[12619699686] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[12620564418] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[12621149805] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12626884446] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[12627789801] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[12628714758] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[12629215203] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12630145671] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12640260237] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[12640849452] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[12644775495] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[12645803610] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[12651599301] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[12652223628] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[12653101065] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[12653600982] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12654474195] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12658168215] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[12658720074] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[12659926257] [INFO] [kernel::task::loader] Segment: vaddr=204c88 exec=false
[12660735351] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12666931431] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[12667614762] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[12682038633] [INFO] [clock] Clock thing created: 327
[12682629696] [INFO] [clock] Waiting for UI Root (Compositor)...
[12685522938] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044678
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12686626194] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368012096 RFLAGS_BEFORE=130 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563776488
[12691274244] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045680
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[12692335755] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368034864 RFLAGS_BEFORE=134 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563776488
[12694229955] [ERROR] [INGESTD] Starting...
[12697750692] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045718
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12698767224] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368051248 RFLAGS_BEFORE=134 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563776488
[12701214570] [INFO] [cambium] cambium starting (v3: catch-up then stream)...
[12706466586] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[12709647852] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[12710761074] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[12711736125] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[12713018835] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[12727905498] [ERROR] [INGESTD] Watch active. Loop start.
[12735087948] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[12736249812] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[12954284046] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[12954987177] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[12956024466] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12958955724] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[12959543157] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12960566256] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[12961139631] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[12967508202] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[12973640790] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00141a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[12974693688] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368100640 RFLAGS_BEFORE=130 CR3_BEFORE=54927360 fs_base=0 gs_base=18446744071563776488
[12977989530] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[12994512663] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[13002876084] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[13011047610] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[13012168026] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[13012715694] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[13013764005] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13016742849] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[13017328401] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13033583706] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[13035201300] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[13043735430] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[13045100343] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[13046495022] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[13047612171] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[13049017872] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[13050240093] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[13051341633] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13052808945] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13055428155] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[13056371592] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[13057310607] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[13057985688] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[13065096990] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[13066529322] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[13067057685] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13068028512] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13071043986] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[13071656664] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13072968909] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[13073568750] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[13079895708] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[13081786476] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[13084495908] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[13085431689] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13087347504] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13093239753] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[13093795374] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13094701752] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[13095254997] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13101033693] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[13120057170] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014318
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[13121315757] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368124400 RFLAGS_BEFORE=134 CR3_BEFORE=55033856 fs_base=0 gs_base=18446744071563776488
[13125470292] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[13126371225] [INFO] [rtc_cmos] Starting... arg=db
[13127865927] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[13131112665] [INFO] [rtc_cmos] RTC: 2026-01-21 01:35:41 = 1768959341 unix_secs
[13132313766] [INFO] [kernel::time] System clock anchored: unix_secs=1768959341, mono_ns=6565922863, offset=1768959334434077137ns
[13133493615] [INFO] [rtc_cmos] System clock anchored
[13149367968] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[13151585634] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045dc0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[13152626751] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368158976 RFLAGS_BEFORE=134 CR3_BEFORE=55136256 fs_base=0 gs_base=18446744071563776488
[13155785610] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[13157649912] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[13158701226] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[13159455639] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[13164526980] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ba00
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[13165682541] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368176384 RFLAGS_BEFORE=134 CR3_BEFORE=55230464 fs_base=0 gs_base=18446744071563776488
[13168911129] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[13169640825] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[13173167700] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0095f70
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[13174287819] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368212416 RFLAGS_BEFORE=134 CR3_BEFORE=55332864 fs_base=0 gs_base=18446744071563776488
[13177513305] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[13184863197] [INFO] [bristle] bristle: registered in graph as svc.Input (id=457)
[13192960869] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=431 backend=BootFB
[13194175764] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[13194698715] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13195784778] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13257423828] [INFO] [kernel::task::loader] Segment: vaddr=26a4a0 exec=false
[13258080396] [INFO] [kernel::task::loader]   Overlap at 26a000: merging perms to r=true w=false x=true
[13265450550] [INFO] [kernel::task::loader] Segment: vaddr=2762c0 exec=false
[13266068739] [INFO] [kernel::task::loader]   Overlap at 276000: merging perms to r=true w=true x=true
[13273097970] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[13274399457] [INFO] [kernel::task::loader] Loading module: /boot/echo
[13274916435] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13275958476] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13278788358] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[13279383117] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13280533959] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[13281158286] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13287623547] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[13288596783] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[13289386902] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[13300144341] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014c40
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1af
[13301239413] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368250144 RFLAGS_BEFORE=130 CR3_BEFORE=55439360 fs_base=0 gs_base=18446744071563776488
[13304088831] [INFO] [bloom::logging] bloom: logging initialized
[13326744783] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004b958
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[13327852494] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368267696 RFLAGS_BEFORE=130 CR3_BEFORE=56012800 fs_base=0 gs_base=18446744071563776488
[13331087979] [INFO] [echo] echo: online (handle=12)
[13331821800] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[13337935611] [INFO] [bloom] bloom: [bloom] Bootstrapped via Bytespace ID=431
[13340949963] [INFO] [bloom] bloom: [bloom] starting (arg_req=1 arg_resp=4 bristle=10 font_svc=0)
[13350834387] [INFO] [bloom] bloom: [bloom] spawned wallpaper_loader thread (tid=15)
[13356928596] [INFO] [bloom] bloom: [bloom] spawned cursor_loader thread (tid=16)
[13362468207] [INFO] [bloom] bloom: [bloom] spawned font_loader thread (tid=17)
[13363181337] [INFO] [bloom] bloom: [bloom] discovering compositor target...
[13369519548] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[13370329269] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[13371282111] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
T:DB40 [13374522579] [INFO] [bloom] bloom: [wallpaper_loader] thread started
T:D770 [13376370909] [INFO] [bloom] bloom: [cursor_loader] thread started
T:CD00 [13378822710] [INFO] [bloom] bloom: [font_loader] thread started
[13379580423] [INFO] [bloom] bloom: [font_loader] scanning boot modules for fonts...
[13385499303] [INFO] [bloom] bloom: [font_loader] found 42 boot modules
[13469228190] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/DSEG7Classic-Regular.ttf'
[13476010218] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=179 size=23272 name='/assets/fonts/DSEG7Classic-Regular.ttf'
[13484435613] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:BA70 [13488870549] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[13489987269] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[13492760259] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/Hack-Regular.ttf'
[13496864799] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[13497662541] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[13519590513] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[13522037727] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[13528385574] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=182 size=309408 name='/assets/fonts/Hack-Regular.ttf'
[13532542353] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[13534666827] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSans-Regular.ttf'
[13538766516] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[13539514461] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[14113563948] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[14114787390] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[14117839296] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=185 size=569208 name='/assets/fonts/NotoSans-Regular.ttf'
[14122477149] [INFO] [bloom] bloom: [wallpaper_loader] searching for wallpaper...
[14123393229] [INFO] [bloom] bloom: [wallpaper_loader] trying: /assets/wallpapers/clouds.bmp
[14127283698] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[14129509086] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol-Regular.ttf'
[14135904123] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[14136913461] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[14493573996] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=188 size=258156 name='/assets/fonts/NotoSansSymbol-Regular.ttf'
[14727620160] [INFO] [bloom] bloom: [cursor_loader] searching for cursor...
[14728651278] [INFO] [bloom] bloom: [cursor_loader] trying: /assets/cursors/plain/Normal.cur
[16034135766] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[16362277140] [INFO] [bloom::compositor] bloom: compositor bytespace 376 (1280x720 stride=5120 format=1)
[16689986379] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=191 size=656852 name='/assets/fonts/NotoSansSymbol2-Regular.ttf'
[16873912242] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[16875348006] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[16876474560] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (258156 bytes)
[16880550225] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSerif-Regular.ttf'
[16887720267] [INFO] [bloom::asset] [asset_bank] mapped at 0x10463000
[16888594701] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[17066206113] [INFO] [bloom::compositor] bloom: display backend: BootFB
[17071542906] [INFO] [ps2_mouse] ps2_mouse: init done
[17072252670] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[17073147762] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[17073900426] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[17376929889] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=194 size=616196 name='/assets/fonts/NotoSerif-Regular.ttf'
[17705877156] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[18369159435] [INFO] [bloom] bloom: [bloom] compositor target: 1280x720 @ 0x104a3000 backend=BootFB
[18370473462] [INFO] [bloom] bloom: [bloom] presenter: driver (req=1 resp=4)
[18371956548] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[19016629038] [INFO] [stem::ui] UiBuilder: created root 545
[19018164066] [INFO] [bloom] bloom: [bloom] created UI root node: 545
[19028731260] [INFO] [display_bootfb] display_bootfb: bound bytespace 376
[19344338211] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd630
[19345306035] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[19346027481] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19346955111] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[19427332881] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[19428793395] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[19429815867] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 191 (656852 bytes)
[19433511801] [INFO] [bloom] bloom: [font_loader] immediate scan complete, entering watch loop
[19436401413] [INFO] [bloom] bloom: [bloom] ui watch opened UNFILTERED (userspace filter pred=UI_TEXT id=517)
[19444964913] [INFO] [bloom::asset] [asset_bank] mapped at 0x10bab000
[19445672433] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[20364260037] [INFO] [clock] Found UI Root: 545 (attempt 4)
[23962821003] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c1b40
[23963965740] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[23964736356] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23965504860] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[24295580265] [INFO] [bloom] bloom: [font_loader] watch opened (id=572)
[25279206612] [INFO] [bloom] bloom: [cursor_loader] SUCCESS: found candidate '/assets/cursors/plain/Normal.cur', enqueuing load
[25280575617] [INFO] [bloom] bloom: [cursor_loader] thread done, sleeping forever
[25839451011] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[25841605020] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol2-Regular.ttf' in slot 4
[25843459851] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 194 (616196 bytes)
[25856423835] [INFO] [bloom::asset] [asset_bank] mapped at 0x10c4c000
[25857305298] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSerif-Regular.ttf'...
[25935384486] [INFO] [bloom] bloom: [wallpaper_loader] SUCCESS: found candidate '/assets/wallpapers/clouds.bmp', enqueuing load
[32486675001] [INFO] [cambium] Found 1 bindings
[34451580834] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fedd0
[34452448371] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[34453502919] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34454609838] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x4 kind=0 pred=0 subj_lo=327
[34776623607] [INFO] [clock] Binding created: 589 (source=327 target=575)
[34777774944] [INFO] [clock] CLOCK: Entering main loop, publishing to thing_id=327
[34780340265] [INFO] [clock] unix=1768959351 utc=2026-01-21 01:35:51 mono_ns=17389431097
[35140102800] [INFO] [cambium] Opened watch 599 for source 327 (binding 589, start_seq=0)
[35170320273] [INFO] [cambium] CATCH-UP: Draining 1 watches for historical events...
[35829627108] [INFO] [cambium] cambium: drain complete payloads=0 overflows=0 last_seq=none
[35830706439] [INFO] [cambium] Entering event loop with 1 bindings (v3)
[36484202106] [INFO] [bloom] bloom: [wallpaper_loader] thread done, sleeping forever
[36811091889] [INFO] [clock] CLOCK PUBLISH: thing=327 now_text='01:35:51' tick=17389431097
[40091747751] [INFO] [clock] unix=1768959354 utc=2026-01-21 01:35:54 mono_ns=20045680462
[41400443205] [INFO] [clock] CLOCK PUBLISH: thing=327 now_text='01:35:54' tick=20045680462
[41826703413] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[41828424132] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSerif-Regular.ttf' in slot 5
[41829474654] [INFO] [bloom::asset] [asset_bank] load_cursor_immediate: /assets/cursors/plain/Normal.cur
[41946235485] [INFO] [bloom::asset] [asset_bank] mapping bytespace 152 (4286 bytes)
[41952854757] [INFO] [bloom::asset] [asset_bank] mapped to 0x10ce3000
[41954440440] [INFO] [bloom::asset] [asset_bank] checking CUR header: len=4286
[41955982695] [INFO] [bloom::asset] [asset_bank] valid CUR header detected
[41956892703] [INFO] [bloom::asset] [asset_bank] CUR: hotspot=(0, 0), img_size=4264, offset=22
[41957653980] [INFO] [bloom::asset] [asset_bank] decoding embedded DIB at offset 22...
[41959510164] [INFO] [bloom::asset] [asset_bank] SUCCESS: cursor DIB decoded 32x32
[41967153591] [INFO] [bloom::asset] [asset_bank] publish_cursor (pending)
[41967945822] [INFO] [bloom::asset] [asset_bank] cursor: Static frame 32x32 hotspot (0, 0)
[41968843356] [INFO] [bloom::asset] [asset_bank] load_wallpaper_immediate: /assets/wallpapers/clouds.bmp
[42094636782] [INFO] [bloom::asset] [asset_bank] mapping bytespace 170 (3145782 bytes)
[42104471046] [INFO] [bloom::asset] [asset_bank] mapped to 0x10ce5000
[42105276444] [INFO] [bloom::asset] [asset_bank] decoding BMP...
[42508632540] [INFO] [bloom::asset] [asset_bank] BMP decoded: 1024x1024
[42939419622] [INFO] [bloom::asset] [asset_bank] bytespace unmapped
[42940501230] [INFO] [bloom::asset] [asset_bank] publish_wallpaper (pending): 1024x1024
[43606419549] [INFO] [cambium] [cambium] write: binding_src=327 target=575 pred=ui.Text(517) val=603 seq=1140
[43611998133] [INFO] [cambium] Updated target 575 with value 603 (seq=1140)
[43624256577] [INFO] [cambium] Updated target 575 with value 17389431097 (seq=1142)
[43636974711] [INFO] [cambium] Updated target 575 with value 617 (seq=1153)
[43648491381] [INFO] [cambium] Updated target 575 with value 20045680462 (seq=1155)
[45362588436] [INFO] [clock] unix=1768959357 utc=2026-01-21 01:35:57 mono_ns=22681122931
[45378347091] [INFO] [clock] CLOCK PUBLISH: thing=327 now_text='01:35:57' tick=22681122931
[45438887934] [INFO] [cambium] Updated target 575 with value 650 (seq=1167)
[45450152220] [INFO] [bloom] bloom: [font_loader] drain complete: payloads=1047 overflows=27
[45452352759] [INFO] [cambium] Updated target 575 with value 22681122931 (seq=1168)
[45459896229] [INFO] [bloom] bloom: [bloom] ui watch drained: 1058 batches
[45495262560] [INFO] [bloom] bloom: [bloom] entering transactional frame loop (acquire -> build -> present)
[45496329021] [INFO] [bloom] bloom: [bloom] reclaimer: budget=33554432 bytes
[45497347236] [INFO] [bloom::reclaimer] [reclaimer] +4194304 bytes (total: 4194304)
[45498066372] [INFO] [bloom::asset] [asset_bank] promoting wallpaper to gen=1 (4194304b)
[45498978657] [INFO] [bloom::reclaimer] [reclaimer] +4096 bytes (total: 4198400)
[45499703898] [INFO] [bloom::asset] [asset_bank] promoting cursor to gen=1 (4096b)
[45500800455] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4300800)
[45501533022] [INFO] [bloom::asset] [asset_bank] promoting font 'DSEG7Classic-Regular.ttf' to gen=1 (102400b) in slot 0
[45502478835] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4403200)
[45503155170] [INFO] [bloom::asset] [asset_bank] promoting font 'Hack-Regular.ttf' to gen=1 (102400b) in slot 1
[45503895327] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4505600)
[45504504705] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=1 (102400b) in slot 2
[45505242783] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4608000)
[45505906974] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol-Regular.ttf' to gen=1 (102400b) in slot 3
[45506706597] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4710400)
[45507328812] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol2-Regular.ttf' to gen=1 (102400b) in slot 4
[45508107513] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 4812800)
[45508728936] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSerif-Regular.ttf' to gen=1 (102400b) in slot 5
[45510221559] [INFO] [bloom] bloom: [bloom] frame 1: wallpaper now visible (gen=1)
[45511179153] [INFO] [bloom] bloom: [bloom] frame 1: cursor now visible (gen=1)
[45514773876] [INFO] [bloom] bloom: [bloom] frame 1: fonts available (mode=legacy)
[46356749868] [INFO] [bloom] bloom: [CONTRACT] [bloom] First frame rendered - compositor ready
[46739425920] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1178
[47493314121] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1182
[47927346027] [INFO] [bloom] bloom: [bloom][damage] rects=1 frame=12
[48647098764] [INFO] [clock] unix=1768959358 utc=2026-01-21 01:35:58 mono_ns=24323313580
[48663150954] [INFO] [clock] CLOCK PUBLISH: thing=327 now_text='01:35:58' tick=24323313580
[48815996064] [INFO] [cambium] [cambium] write: binding_src=327 target=575 pred=ui.Text(517) val=688 seq=1202
[48822587616] [INFO] [cambium] Updated target 575 with value 688 (seq=1202)
[48831943149] [INFO] [cambium] Updated target 575 with value 24323313580 (seq=1203)
[48902748543] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1188
[49558936614] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1189
[50003913828] [INFO] [bloom] bloom: [bloom][damage] rects=1 frame=19
[50172314907] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1190
[50868594447] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1191
[51462319524] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1192
[51915104472] [INFO] [clock] unix=1768959360 utc=2026-01-21 01:36:00 mono_ns=25957351051
[51928224876] [INFO] [clock] CLOCK PUBLISH: thing=327 now_text='01:36:00' tick=25957351051
[52104884040] [INFO] [bloom] bloom: [bloom][uiwatch] UI_TEXT event: seq=1193 subj=661 pred=517
[52106039139] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1193
[52111963365] [INFO] [cambium] [cambium] write: binding_src=327 target=575 pred=ui.Text(517) val=701 seq=1214
[52125515211] [INFO] [cambium] Updated target 575 with value 701 (seq=1214)
[52135574535] [INFO] [cambium] Updated target 575 with value 25957351051 (seq=1215)
[52798024653] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1194
[53404172514] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1195
[54032025432] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1196
[54500278569] [INFO] [bloom] bloom: [bloom][damage] rects=1 frame=26
[54646399500] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1197
[55251271383] [INFO] [clock] unix=1768959362 utc=2026-01-21 01:36:02 mono_ns=27625389561
[55259847192] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1198
[55269840021] [INFO] [clock] CLOCK PUBLISH: thing=327 now_text='01:36:02' tick=27625389561
[55393043739] [INFO] [cambium] [cambium] write: binding_src=327 target=575 pred=ui.Text(517) val=715 seq=1224
[55397853852] [INFO] [cambium] Updated target 575 with value 715 (seq=1224)
[55407030294] [INFO] [cambium] Updated target 575 with value 27625389561 (seq=1225)
[55874940792] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1199
[56494846881] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1200
[57122550375] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1201
[58405590177] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1204
[58534377561] [INFO] [clock] unix=1768959363 utc=2026-01-21 01:36:03 mono_ns=29266977795
[58548342633] [INFO] [clock] CLOCK PUBLISH: thing=327 now_text='01:36:03' tick=29266977795
[58670376270] [INFO] [cambium] [cambium] write: binding_src=327 target=575 pred=ui.Text(517) val=727 seq=1234
[58675816155] [INFO] [cambium] Updated target 575 with value 727 (seq=1234)
[58684961379] [INFO] [cambium] Updated target 575 with value 29266977795 (seq=1235)
[58884402687] [INFO] [bloom] bloom: [bloom][damage] rects=1 frame=34
[59037205392] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1205
[59676450801] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1206
[60323114412] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1207
[60945693303] [INFO] [bloom] bloom: [bloom][uiwatch] UI_TEXT event: seq=1208 subj=661 pred=517
[60947008386] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1208
[61588573299] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1209
[61810781835] [INFO] [clock] unix=1768959365 utc=2026-01-21 01:36:05 mono_ns=30905186548
[61823799774] [INFO] [clock] CLOCK PUBLISH: thing=327 now_text='01:36:05' tick=30905186548
[61916926434] [INFO] [cambium] [cambium] write: binding_src=327 target=575 pred=ui.Text(517) val=741 seq=1243
[61923419481] [INFO] [cambium] Updated target 575 with value 741 (seq=1243)
[61935102570] [INFO] [cambium] Updated target 575 with value 30905186548 (seq=1244)
[62188383114] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1210
[62753395386] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1211
[63191572554] [INFO] [bloom] bloom: [bloom][damage] rects=1 frame=41
[63334547991] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1212
[63911162304] [INFO] [bloom] bloom: [bloom][ui] DIRTY reason=ui_text watch seq=1213

```
</details>
