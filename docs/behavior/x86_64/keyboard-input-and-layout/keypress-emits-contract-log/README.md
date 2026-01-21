# ❌ Scenario: Keypress emits contract log

> Last run: 2026-01-19 21:36:59

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the clock window is ticking | ✅ | 11189ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I press a key | ❌ | 1016ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11165973687] [CONTRACT] [kernel] thing-os kernel starting...
[11176285725] [INFO] [kernel::memory] Memory map has 64 entries
[11178845832] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[11179711224] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[11180061750] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[11180501079] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[11180854014] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[11181186885] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[11181523386] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[11181852858] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[11182255491] [INFO] [kernel::memory]   [8] 0x1780000 - 0x7866a000 (Usable)
[11182608855] [INFO] [kernel::memory]   [9] 0x7866a000 - 0x786cc000 (Reserved)
[11182975353] [INFO] [kernel::memory]   [10] 0x786cc000 - 0x7884e000 (Other)
[11183326671] [INFO] [kernel::memory]   [11] 0x7884e000 - 0x7884f000 (Reserved)
[11183691057] [INFO] [kernel::memory]   [12] 0x7884f000 - 0x788e6000 (Other)
[11184086661] [INFO] [kernel::memory]   [13] 0x788e6000 - 0x788e7000 (Reserved)
[11184459231] [INFO] [kernel::memory]   [14] 0x788e7000 - 0x78988000 (Other)
[11184819492] [INFO] [kernel::memory]   [15] 0x78988000 - 0x78989000 (Reserved)
[11185187442] [INFO] [kernel::memory]   [16] 0x78989000 - 0x789c9000 (Other)
[11185686864] [INFO] [kernel::memory]   [17] 0x789c9000 - 0x789ca000 (Reserved)
[11186354223] [INFO] [kernel::memory]   [18] 0x789ca000 - 0x78a55000 (Other)
[11187000957] [INFO] [kernel::memory]   [19] 0x78a55000 - 0x78a56000 (Reserved)
[11187763752] [INFO] [kernel::memory]   [20] 0x78a56000 - 0x78aa2000 (Other)
[11188389894] [INFO] [kernel::memory]   [21] 0x78aa2000 - 0x78aa3000 (Reserved)
[11189180178] [INFO] [kernel::memory]   [22] 0x78aa3000 - 0x78aa9000 (Other)
[11189857602] [INFO] [kernel::memory]   [23] 0x78aa9000 - 0x78aaa000 (Reserved)
[11190518955] [INFO] [kernel::memory]   [24] 0x78aaa000 - 0x78f2b000 (Other)
[11191211922] [INFO] [kernel::memory]   [25] 0x78f2b000 - 0x78f2c000 (Reserved)
[11191715898] [INFO] [kernel::memory]   [26] 0x78f2c000 - 0x793ad000 (Other)
[11192245779] [INFO] [kernel::memory]   [27] 0x793ad000 - 0x793ae000 (Reserved)
[11192635674] [INFO] [kernel::memory]   [28] 0x793ae000 - 0x796af000 (Other)
[11193098598] [INFO] [kernel::memory]   [29] 0x796af000 - 0x796b0000 (Reserved)
[11193492981] [INFO] [kernel::memory]   [30] 0x796b0000 - 0x796b9000 (Other)
[11193846279] [INFO] [kernel::memory]   [31] 0x796b9000 - 0x796ba000 (Reserved)
[11194215846] [INFO] [kernel::memory]   [32] 0x796ba000 - 0x796be000 (Other)
[11194567560] [INFO] [kernel::memory]   [33] 0x796be000 - 0x796bf000 (Reserved)
[11195031111] [INFO] [kernel::memory]   [34] 0x796bf000 - 0x796c1000 (Other)
[11195400447] [INFO] [kernel::memory]   [35] 0x796c1000 - 0x796c2000 (Reserved)
[11195857365] [INFO] [kernel::memory]   [36] 0x796c2000 - 0x796c4000 (Other)
[11196350220] [INFO] [kernel::memory]   [37] 0x796c4000 - 0x796c5000 (Reserved)
[11196790374] [INFO] [kernel::memory]   [38] 0x796c5000 - 0x796c7000 (Other)
[11197165782] [INFO] [kernel::memory]   [39] 0x796c7000 - 0x796c8000 (Reserved)
[11197572309] [INFO] [kernel::memory]   [40] 0x796c8000 - 0x796ca000 (Other)
[11197937487] [INFO] [kernel::memory]   [41] 0x796ca000 - 0x796cb000 (Reserved)
[11198305536] [INFO] [kernel::memory]   [42] 0x796cb000 - 0x796cd000 (Other)
[11198708004] [INFO] [kernel::memory]   [43] 0x796cd000 - 0x796ce000 (Reserved)
[11199389652] [INFO] [kernel::memory]   [44] 0x796ce000 - 0x796d8000 (Other)
[11199747834] [INFO] [kernel::memory]   [45] 0x796d8000 - 0x796d9000 (Reserved)
[11200113177] [INFO] [kernel::memory]   [46] 0x796d9000 - 0x796dd000 (Other)
[11200466970] [INFO] [kernel::memory]   [47] 0x796dd000 - 0x796de000 (Reserved)
[11200839144] [INFO] [kernel::memory]   [48] 0x796de000 - 0x796e0000 (Other)
[11201198448] [INFO] [kernel::memory]   [49] 0x796e0000 - 0x796e1000 (Reserved)
[11201569599] [INFO] [kernel::memory]   [50] 0x796e1000 - 0x796e3000 (Other)
[11201929827] [INFO] [kernel::memory]   [51] 0x796e3000 - 0x796e4000 (Reserved)
[11202359223] [INFO] [kernel::memory]   [52] 0x796e4000 - 0x796e8000 (Other)
[11202712785] [INFO] [kernel::memory]   [53] 0x796e8000 - 0x79757000 (Other)
[11203062321] [INFO] [kernel::memory]   [54] 0x79757000 - 0x7990d000 (Other)
[11203413540] [INFO] [kernel::memory]   [55] 0x7990d000 - 0x7a16c000 (Reserved)
[11203777662] [INFO] [kernel::memory]   [56] 0x7a16c000 - 0x7bb6c000 (Usable)
[11204142246] [INFO] [kernel::memory]   [57] 0x7bb6c000 - 0x7bb8d000 (Reserved)
[11204507160] [INFO] [kernel::memory]   [58] 0x7bb8d000 - 0x7bb91000 (Other)
[11204859204] [INFO] [kernel::memory]   [59] 0x7bb91000 - 0x7bb92000 (Reserved)
[11205223953] [INFO] [kernel::memory]   [60] 0x7bb92000 - 0x7bb94000 (Other)
[11205610614] [INFO] [kernel::memory]   [61] 0x7bb94000 - 0x7bb95000 (Reserved)
[11205993018] [INFO] [kernel::memory]   [62] 0x7bb95000 - 0x7bb97000 (Other)
[11206347273] [INFO] [kernel::memory]   [63] 0x7bb97000 - 0x7bb98000 (Reserved)
[11206991598] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[11462986590] [CONTRACT] [kernel::memory] Frame allocator initialized with 495746 free frames
[11470567548] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[11475264504] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[11476369014] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[11477047131] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[11483620995] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[11484104280] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11487083487] [INFO] [bran::arch] IOAPIC: Registers initialized
[11488374150] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[11490079161] [INFO] [bran::arch] IOAPIC: All pins masked
[11491450443] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11492098101] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11492568846] [INFO] [bran::arch] IOAPIC: Init complete
[11493166311] [CONTRACT] [kernel] Initializing global allocator...
[11867720337] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[11868427065] [CONTRACT] [kernel] Initializing SIMD...
[11869836957] [CONTRACT] [kernel] Initializing tasking...
[11874742176] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[11876315187] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[11876916975] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[11882755335] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[11883188295] [INFO] [kernel::task::scheduler]   Initializing boot task...
[11883935580] [INFO] [kernel::task::scheduler]   Creating boot task...
[11888766087] [INFO] [kernel::task::scheduler]   Creating idle task...
[11894014473] [INFO] [kernel::task::scheduler]   Boot task initialized
[11894421165] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[11895258276] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[11901228207] [INFO] [kernel::root] Spawning Root service...
[11908873416] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[11918001645] [INFO] [kernel::root::service] ROOT: started once
[12823904379] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[12824775216] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[12866395773] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[12883792614] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[12910719888] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[12943679364] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[12945705993] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[12987266655] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[13017597582] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[13025692680] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[13028724423] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[13052943420] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[13055742249] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[13056772542] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[13060229193] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[13062730890] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[13063541634] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13071487143] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13084185114] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[13085302989] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[13087982160] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[13088916852] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[13097131344] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[13098044421] [CONTRACT] [kernel] Spawning init process...
[13099704882] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[13134538131] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62262600 ticks/sec), init_cnt=622626 for 100Hz
[13136434047] [CONTRACT] [kernel] Entering scheduler loop.
[13146699753] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[13151793435] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367676144 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563776488
[13162558662] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[13163977992] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[13164771609] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[13165575357] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[13187277312] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[13191118281] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[13194622518] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[13195447122] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[13199162889] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[13202803020] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[13203596307] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[13206902874] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[13207687812] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[13208463543] [INFO] [sprout::devtree] SPROUT: build() called
[13209105921] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[13213569699] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[13214347377] [INFO] [sprout] SPROUT: About to create Supervisor...
[13215036912] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[13215827031] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[13216498614] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[13221345390] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[13261357362] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[13267072236] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[13270336134] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[13273360353] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[13275925344] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[13280455122] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[13284619128] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[13287895764] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[13291051224] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[13294289481] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[13297626177] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[13301594130] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[13305466515] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[13309053780] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[13312206963] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[13315312131] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[13318313481] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[13321371294] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[13324391883] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[13327886484] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[13331413590] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[13335708738] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[13338935016] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[13342096152] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[13345416282] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[13348943223] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[13352229231] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[13355662419] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[13359267273] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[13363515891] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[13367146617] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[13370497866] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[13373685831] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[13377136542] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[13380422715] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[13383780333] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[13387766271] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[13391588661] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[13394833386] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[13398339735] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[13401776949] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[13405064607] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[13408649034] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[13411054503] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[13428666504] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[13527454182] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[13528772796] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[13530949608] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[13532015904] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[13532794044] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[13533632574] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[13536173013] [INFO] [kernel::task::loader] Loading module: /boot/clock
[13536707382] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13537902114] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13541757768] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[13542438228] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[13543758525] [INFO] [kernel::task::loader] Segment: vaddr=204ee8 exec=false
[13544753541] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13551619059] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[13554499266] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[13555440723] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[13555967964] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13557405873] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13561456194] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[13562058444] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[13562885622] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[13563450615] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13569752823] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[13571125722] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[13572100608] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[13572603528] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13573618872] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13584288993] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[13584931866] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[13588930575] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[13589488638] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[13595878989] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[13596661980] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[13598351976] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[13598869449] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13599914724] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13603564656] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[13604161032] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[13605471363] [INFO] [kernel::task::loader] Segment: vaddr=204c88 exec=false
[13606058037] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13612648236] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[13613474193] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[13630377420] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13631691414] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072367986928 RFLAGS_BEFORE=134 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563776488
[13635753450] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[13636840767] [INFO] [clock] starting clock publisher
[13646678034] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13647868806] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368009968 RFLAGS_BEFORE=134 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563776488
[13652943249] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ab8
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[13654149366] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368026352 RFLAGS_BEFORE=134 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563776488
[13656062475] [ERROR] [INGESTD] Starting...
[13659834441] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ae8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13661034816] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368053488 RFLAGS_BEFORE=134 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563776488
[13663960992] [INFO] [cambium] cambium starting (v3: catch-up then stream)...
[13670352102] [INFO] [clock] Clock thing created: 356
[13671074703] [INFO] [clock] Waiting for UI Root (Compositor)...
[13675977480] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[13677201780] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[13678392585] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[13685588334] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[13703172846] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[13712067996] [ERROR] [INGESTD] Watch active. Loop start.
[13719978360] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[13721248959] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[13948459800] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[13949314434] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[13950564342] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13953757917] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[13954469727] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13955753691] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[13956441642] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13964048208] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[13971232308] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[13972542936] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368106128 RFLAGS_BEFORE=134 CR3_BEFORE=54927360 fs_base=0 gs_base=18446744071563776488
[13976409612] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[13997435727] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[14006534058] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[14016046803] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[14017396074] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[14017977963] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[14019324990] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14022691254] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[14023353630] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14024810481] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[14025436029] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[14032467900] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[14033687052] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[14035143705] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[14036521983] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[14038018170] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[14039104992] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[14039626524] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14040709782] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14043148647] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[14043752910] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[14044745055] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[14045384232] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[14051691687] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[14053073892] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[14053768773] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14054894304] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14057956539] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[14058587136] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14059922349] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[14060543805] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[14067395925] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[14069186934] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[14070272271] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[14070803571] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14072277351] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14075168547] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[14075771754] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14076805677] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[14077391460] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[14083422012] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[14099755428] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044678
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[14101128030] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368129664 RFLAGS_BEFORE=130 CR3_BEFORE=55033856 fs_base=0 gs_base=18446744071563776488
[14105463405] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[14106427401] [INFO] [rtc_cmos] Starting... arg=db
[14108015262] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[14111374497] [INFO] [rtc_cmos] RTC: 2026-01-20 05:37:27 = 1768887447 unix_secs
[14112623976] [INFO] [kernel::time] System clock anchored: unix_secs=1768887447, mono_ns=7056055743, offset=1768887439943944257ns
[14113903551] [INFO] [rtc_cmos] System clock anchored
[14132018505] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[14134402788] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ae8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[14135493042] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368164352 RFLAGS_BEFORE=134 CR3_BEFORE=55136256 fs_base=0 gs_base=18446744071563776488
[14138876862] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[14140810695] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[14141963451] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[14142796041] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[14148081090] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004b958
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[14149358256] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368181760 RFLAGS_BEFORE=134 CR3_BEFORE=55230464 fs_base=0 gs_base=18446744071563776488
[14155274727] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[14156146323] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[14160075402] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0095fb0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[14161301781] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368207632 RFLAGS_BEFORE=130 CR3_BEFORE=55332864 fs_base=0 gs_base=18446744071563776488
[14164867200] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[14172004077] [INFO] [bristle] bristle: registered in graph as svc.Input (id=457)
[14180747790] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=431 backend=BootFB
[14181967833] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[14182492038] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14183655618] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14242125150] [INFO] [kernel::task::loader] Segment: vaddr=2615d0 exec=false
[14243023971] [INFO] [kernel::task::loader]   Overlap at 261000: merging perms to r=true w=false x=true
[14250945885] [INFO] [kernel::task::loader] Segment: vaddr=26d1a0 exec=false
[14251618227] [INFO] [kernel::task::loader]   Overlap at 26d000: merging perms to r=true w=true x=true
[14258762496] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[14260282509] [INFO] [kernel::task::loader] Loading module: /boot/echo
[14260839582] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14261932179] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14264899869] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[14265545217] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14266592076] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[14267190432] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[14273327211] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[14274419973] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[14275225338] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[14287721448] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014c50
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1af
[14289033594] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368249616 RFLAGS_BEFORE=130 CR3_BEFORE=55439360 fs_base=0 gs_base=18446744071563776488
[14292114936] [INFO] [bloom::logging] bloom: logging initialized
[14295834036] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004b958
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[14297037282] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132397 RSP_BEFORE=18446744072368267152 RFLAGS_BEFORE=134 CR3_BEFORE=55975936 fs_base=0 gs_base=18446744071563776488
[14300426844] [INFO] [echo] echo: online (handle=12)
[14301196998] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[14307634209] [INFO] [bloom] bloom: [bloom] Bootstrapped via Bytespace ID=431
[14310928764] [INFO] [bloom] bloom: [bloom] starting (arg_req=1 arg_resp=4 bristle=10 font_svc=0)
[14321456589] [INFO] [bloom] bloom: [bloom] spawned wallpaper_loader thread (tid=15)
[14327771172] [INFO] [bloom] bloom: [bloom] spawned cursor_loader thread (tid=16)
[14333608773] [INFO] [bloom] bloom: [bloom] spawned font_loader thread (tid=17)
[14334341373] [INFO] [bloom] bloom: [bloom] discovering compositor target...
T:AEF0 [14340131619] [INFO] [bloom] bloom: [wallpaper_loader] thread started
T:AB20 [14341980279] [INFO] [bloom] bloom: [cursor_loader] thread started
T:A0B0 [14343816564] [INFO] [bloom] bloom: [font_loader] thread started
[14344568370] [INFO] [bloom] bloom: [font_loader] scanning boot modules for fonts...
[14350945587] [INFO] [bloom] bloom: [font_loader] found 42 boot modules
[14390053326] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[14390997654] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[14392030785] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
[14450548662] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/DSEG7Classic-Regular.ttf'
[14457712071] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=179 size=23272 name='/assets/fonts/DSEG7Classic-Regular.ttf'
[14486183118] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:35F0 [14491738767] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[14493004350] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[14495973756] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/Hack-Regular.ttf'
[14500264350] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[14501099283] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[14524535157] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[14527266765] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[14534331933] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=182 size=309408 name='/assets/fonts/Hack-Regular.ttf'
[14538818349] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[14541582759] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSans-Regular.ttf'
[14547030003] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[14547858237] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[15139984299] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=185 size=569208 name='/assets/fonts/NotoSans-Regular.ttf'
[15436873815] [INFO] [bloom] bloom: [wallpaper_loader] searching for wallpaper...
[15438249321] [INFO] [bloom] bloom: [wallpaper_loader] trying: /assets/wallpapers/clouds.bmp
[15466548834] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[15467853720] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[15762531312] [INFO] [bloom] bloom: [cursor_loader] searching for cursor...
[15763726341] [INFO] [bloom] bloom: [cursor_loader] trying: /assets/cursors/plain/Normal.cur
[15776032074] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[15779459916] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol-Regular.ttf'
[15786667545] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[15787543233] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[16418560488] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=188 size=258156 name='/assets/fonts/NotoSansSymbol-Regular.ttf'
[18714394974] [INFO] [bloom::compositor] bloom: compositor bytespace 376 (1280x720 stride=5120 format=1)
[20682899292] [INFO] [bloom::compositor] bloom: display backend: BootFB
[20687946972] [INFO] [ps2_mouse] ps2_mouse: init done
[20688745044] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[20689735275] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[20690596410] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[21339839664] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[22005729273] [INFO] [bloom] bloom: [bloom] compositor target: 1280x720 @ 0x10463000 backend=BootFB
[22007144709] [INFO] [bloom] bloom: [bloom] presenter: driver (req=1 resp=4)
[22008790353] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[22651502577] [INFO] [stem::ui] UiBuilder: created root 536
[22652666091] [INFO] [bloom] bloom: [bloom] created UI root node: 536
[22663968525] [INFO] [display_bootfb] display_bootfb: bound bytespace 376
[23307349032] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd5a0
[23308583331] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[23309400048] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23310460569] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[23641842510] [INFO] [clock] Found UI Root: 536 (attempt 5)
[23964310293] [INFO] [bloom] bloom: [bloom] ui watch opened UNFILTERED (userspace filter pred=UI_TEXT id=506)
[27905708622] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[28546607232] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[28547888226] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[28548699102] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (258156 bytes)
[28555693551] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=191 size=656852 name='/assets/fonts/NotoSansSymbol2-Regular.ttf'
[28562253423] [INFO] [bloom::asset] [asset_bank] mapped at 0x10b6b000
[28563028725] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[28613197734] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSerif-Regular.ttf'
[29263209261] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=194 size=616196 name='/assets/fonts/NotoSerif-Regular.ttf'
[29578843008] [INFO] [bloom] bloom: [font_loader] immediate scan complete, entering watch loop
[30132473943] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[30133891161] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[30134954091] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 191 (656852 bytes)
[30147124722] [INFO] [bloom::asset] [asset_bank] mapped at 0x10bab000
[30147946851] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[30236308410] [INFO] [bloom] bloom: [cursor_loader] SUCCESS: found candidate '/assets/cursors/plain/Normal.cur', enqueuing load
[30887802363] [INFO] [bloom] bloom: [cursor_loader] thread done, sleeping forever
[32210176086] [INFO] [bloom] bloom: [wallpaper_loader] SUCCESS: found candidate '/assets/wallpapers/clouds.bmp', enqueuing load
[32211710421] [INFO] [bloom] bloom: [wallpaper_loader] thread done, sleeping forever
[34172275599] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c1b40
[34173190227] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[34174076310] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34175017998] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[34500197589] [INFO] [bloom] bloom: [font_loader] watch opened (id=586)
[34506172338] [INFO] [cambium] Found 1 bindings
[34681794576] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[34683272877] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol2-Regular.ttf' in slot 4
[34684382700] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 194 (616196 bytes)
[34696370610] [INFO] [bloom::asset] [asset_bank] mapped at 0x10c4c000
[34697378430] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSerif-Regular.ttf'...
[35187233070] [INFO] [clock] Binding created: 579 (source=356 target=563)
[35188498884] [INFO] [clock] CLOCK: Entering main loop, publishing to thing_id=356
[35191235310] [INFO] [clock] unix=1768887457 utc=2026-01-20 05:37:37 mono_ns=17594803875
[35225583624] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fedd0
[35226541185] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[35227444230] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35228409744] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x4 kind=0 pred=0 subj_lo=356
[35556487164] [INFO] [cambium] Opened watch 605 for source 356 (binding 579, start_seq=0)
[35583892410] [INFO] [cambium] CATCH-UP: Draining 1 watches for historical events...
[36566612709] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:37:37' tick=17594803875
[38537521407] [INFO] [cambium] cambium: drain complete payloads=0 overflows=8 last_seq=none
[38538517215] [INFO] [cambium] Entering event loop with 1 bindings (v3)
[39851052879] [INFO] [clock] unix=1768887459 utc=2026-01-20 05:37:39 mono_ns=19925319150
[41162734602] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='05:37:39' tick=19925319150

```
</details>
