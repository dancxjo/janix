# ✅ Scenario: Cursor responds to mouse movement

> Last run: 2026-01-20 19:32:43

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given a cursor is visible on the screen | ✅ | 5087ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | When I move the mouse | ✅ | 674ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the serial log should contain 'CONTRACT: input pointer_move' | ✅ | 424ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And the cursor should move correspondingly on the screen | ✅ | 447ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[10962696855] [CONTRACT] [kernel] thing-os kernel starting...
[10974576393] [INFO] [kernel::memory] Memory map has 64 entries
[10977116370] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[10977754689] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[10978240449] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[10978623744] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[10978960674] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[10979285757] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[10979742477] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[10980192465] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[10980590841] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78656000 (Usable)
[10980936747] [INFO] [kernel::memory]   [9] 0x78656000 - 0x786b8000 (Reserved)
[10981293840] [INFO] [kernel::memory]   [10] 0x786b8000 - 0x7883a000 (Other)
[10981640076] [INFO] [kernel::memory]   [11] 0x7883a000 - 0x7883b000 (Reserved)
[10982000766] [INFO] [kernel::memory]   [12] 0x7883b000 - 0x788d2000 (Other)
[10982346441] [INFO] [kernel::memory]   [13] 0x788d2000 - 0x788d3000 (Reserved)
[10982705349] [INFO] [kernel::memory]   [14] 0x788d3000 - 0x78974000 (Other)
[10983268395] [INFO] [kernel::memory]   [15] 0x78974000 - 0x78975000 (Reserved)
[10986978486] [INFO] [kernel::memory]   [16] 0x78975000 - 0x789b5000 (Other)
[10987332873] [INFO] [kernel::memory]   [17] 0x789b5000 - 0x789b6000 (Reserved)
[10987695312] [INFO] [kernel::memory]   [18] 0x789b6000 - 0x78a41000 (Other)
[10988046003] [INFO] [kernel::memory]   [19] 0x78a41000 - 0x78a42000 (Reserved)
[10988409333] [INFO] [kernel::memory]   [20] 0x78a42000 - 0x78a8e000 (Other)
[10988758803] [INFO] [kernel::memory]   [21] 0x78a8e000 - 0x78a8f000 (Reserved)
[10989123519] [INFO] [kernel::memory]   [22] 0x78a8f000 - 0x78a95000 (Other)
[10989474342] [INFO] [kernel::memory]   [23] 0x78a95000 - 0x78a96000 (Reserved)
[10989852390] [INFO] [kernel::memory]   [24] 0x78a96000 - 0x78f17000 (Other)
[10990201035] [INFO] [kernel::memory]   [25] 0x78f17000 - 0x78f18000 (Reserved)
[10990560867] [INFO] [kernel::memory]   [26] 0x78f18000 - 0x79399000 (Other)
[10990910568] [INFO] [kernel::memory]   [27] 0x79399000 - 0x7939a000 (Reserved)
[10991273700] [INFO] [kernel::memory]   [28] 0x7939a000 - 0x7969b000 (Other)
[10991622114] [INFO] [kernel::memory]   [29] 0x7969b000 - 0x7969c000 (Reserved)
[10991981286] [INFO] [kernel::memory]   [30] 0x7969c000 - 0x796a5000 (Other)
[10992328644] [INFO] [kernel::memory]   [31] 0x796a5000 - 0x796a6000 (Reserved)
[10992686430] [INFO] [kernel::memory]   [32] 0x796a6000 - 0x796aa000 (Other)
[10993047516] [INFO] [kernel::memory]   [33] 0x796aa000 - 0x796ab000 (Reserved)
[10993405665] [INFO] [kernel::memory]   [34] 0x796ab000 - 0x796ad000 (Other)
[10993749459] [INFO] [kernel::memory]   [35] 0x796ad000 - 0x796ae000 (Reserved)
[10994106222] [INFO] [kernel::memory]   [36] 0x796ae000 - 0x796b0000 (Other)
[10994451666] [INFO] [kernel::memory]   [37] 0x796b0000 - 0x796b1000 (Reserved)
[10994810079] [INFO] [kernel::memory]   [38] 0x796b1000 - 0x796b3000 (Other)
[10995155028] [INFO] [kernel::memory]   [39] 0x796b3000 - 0x796b4000 (Reserved)
[10995512550] [INFO] [kernel::memory]   [40] 0x796b4000 - 0x796b6000 (Other)
[10995859479] [INFO] [kernel::memory]   [41] 0x796b6000 - 0x796b7000 (Reserved)
[10996244325] [INFO] [kernel::memory]   [42] 0x796b7000 - 0x796b9000 (Other)
[10996668672] [INFO] [kernel::memory]   [43] 0x796b9000 - 0x796ba000 (Reserved)
[10997158722] [INFO] [kernel::memory]   [44] 0x796ba000 - 0x796c4000 (Other)
[10997516244] [INFO] [kernel::memory]   [45] 0x796c4000 - 0x796c5000 (Reserved)
[10997876043] [INFO] [kernel::memory]   [46] 0x796c5000 - 0x796c9000 (Other)
[10998223533] [INFO] [kernel::memory]   [47] 0x796c9000 - 0x796ca000 (Reserved)
[10998583101] [INFO] [kernel::memory]   [48] 0x796ca000 - 0x796cc000 (Other)
[10998930261] [INFO] [kernel::memory]   [49] 0x796cc000 - 0x796cd000 (Reserved)
[10999289334] [INFO] [kernel::memory]   [50] 0x796cd000 - 0x796cf000 (Other)
[10999678074] [INFO] [kernel::memory]   [51] 0x796cf000 - 0x796d0000 (Reserved)
[11000040315] [INFO] [kernel::memory]   [52] 0x796d0000 - 0x796d4000 (Other)
[11000390412] [INFO] [kernel::memory]   [53] 0x796d4000 - 0x796d5000 (Reserved)
[11000752851] [INFO] [kernel::memory]   [54] 0x796d5000 - 0x79750000 (Other)
[11001103773] [INFO] [kernel::memory]   [55] 0x79750000 - 0x79907000 (Other)
[11001455718] [INFO] [kernel::memory]   [56] 0x79907000 - 0x7a16c000 (Reserved)
[11001819477] [INFO] [kernel::memory]   [57] 0x7a16c000 - 0x7bb6c000 (Usable)
[11002173567] [INFO] [kernel::memory]   [58] 0x7bb6c000 - 0x7bb8c000 (Reserved)
[11002534917] [INFO] [kernel::memory]   [59] 0x7bb8c000 - 0x7bb90000 (Other)
[11002901778] [INFO] [kernel::memory]   [60] 0x7bb90000 - 0x7bb91000 (Reserved)
[11003262864] [INFO] [kernel::memory]   [61] 0x7bb91000 - 0x7bb93000 (Other)
[11003611443] [INFO] [kernel::memory]   [62] 0x7bb93000 - 0x7bb94000 (Reserved)
[11003972628] [INFO] [kernel::memory]   [63] 0x7bb94000 - 0x7bb96000 (Other)
[11004586362] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[11347285191] [CONTRACT] [kernel::memory] Frame allocator initialized with 495726 free frames
[11361185451] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[11375604108] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[11377932918] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[11379162993] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[11390494533] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[11391468000] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11397385791] [INFO] [bran::arch] IOAPIC: Registers initialized
[11401553724] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[11405184252] [INFO] [bran::arch] IOAPIC: All pins masked
[11409420528] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11410908960] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11411717592] [INFO] [bran::arch] IOAPIC: Init complete
[11412697989] [CONTRACT] [kernel] Initializing global allocator...
[11926779810] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[11931358164] [CONTRACT] [kernel] Initializing SIMD...
[11935986513] [CONTRACT] [kernel] Initializing tasking...
[11946337656] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[11951829912] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[11952808560] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[11963427234] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[11970106236] [INFO] [kernel::task::scheduler]   Initializing boot task...
[11971562394] [INFO] [kernel::task::scheduler]   Creating boot task...
[11979051876] [INFO] [kernel::task::scheduler]   Creating idle task...
[11987990355] [INFO] [kernel::task::scheduler]   Boot task initialized
[11988813936] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[11990338536] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[12001697796] [INFO] [kernel::root] Spawning Root service...
[12021902574] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[12042637794] [INFO] [kernel::root::service] ROOT: started once
[13354550055] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[13355475804] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[13399590798] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[13418281899] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[13446477033] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[13482631206] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[13484573157] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[13524596448] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[13553672286] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[13561908987] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[13564949343] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[13591304892] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[13594468503] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[13595596575] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[13599792690] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[13603678011] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[13604940393] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13613424198] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13626097518] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[13627401348] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[13629985314] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[13630923966] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[13640268114] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[13641592899] [CONTRACT] [kernel] Spawning init process...
[13643581875] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[13678768719] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62381400 ticks/sec), init_cnt=623814 for 100Hz
[13680753999] [CONTRACT] [kernel] Entering scheduler loop.
[13690550577] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[13697542782] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072367676144 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563780584
[13709182344] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[13710884847] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[13716865701] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[13718146530] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[13736325603] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[13740566829] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[13744418127] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[13745506830] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[13751060598] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[13754936514] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[13755841374] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[13759548363] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[13760431245] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[13761166815] [INFO] [sprout::devtree] SPROUT: build() called
[13761818664] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[13766789883] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[13767667089] [INFO] [sprout] SPROUT: About to create Supervisor...
[13768357020] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[13769161065] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[13769915709] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[13775436741] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[13815544413] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[13821667068] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[13825128438] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[13828283931] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[13830605613] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[13834123182] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[13838693088] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[13843414992] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[13847059941] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[13850804484] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[13854379572] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[13858485729] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[13863797508] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[13867546935] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[13871202246] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[13874265339] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[13877641932] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[13880702847] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[13883820852] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[13887793656] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[13891392504] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[13894924527] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[13898030157] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[13901202348] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[13904994180] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[13908700443] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[13912167258] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[13915651398] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[13919471544] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[13922665746] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[13925873511] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[13929276801] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[13932919341] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[13936104864] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[13939236927] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[13943274972] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[13947205470] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[13950783891] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[13954548795] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[13957711515] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[13960826253] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[13963946997] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[13967315802] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[13969905312] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[13990952646] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[14083191177] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[14084292420] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[14086409106] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[14087604597] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[14088438969] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[14089358085] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[14091911328] [INFO] [kernel::task::loader] Loading module: /boot/clock
[14092501731] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14093632146] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14097404871] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[14098014018] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[14099423415] [INFO] [kernel::task::loader] Segment: vaddr=204ee8 exec=false
[14100005172] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[14107724928] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[14111074593] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[14112530718] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[14113111815] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14114611995] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14119087752] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[14119684788] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[14120506257] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[14121059403] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[14127036891] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[14128255713] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[14129285907] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[14129838954] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14130923565] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14141912004] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[14142853890] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[14147080530] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[14147719773] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[14154070359] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[14154925059] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[14156703000] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[14157449097] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14158719003] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14163114669] [INFO] [kernel::task::loader] Segment: vaddr=204360 exec=false
[14163990489] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[14165659728] [INFO] [kernel::task::loader] Segment: vaddr=205168 exec=false
[14166277059] [INFO] [kernel::task::loader]   Overlap at 205000: merging perms to r=true w=true x=true
[14173284444] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[14174191218] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[14191527174] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[14192954655] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072367986928 RFLAGS_BEFORE=134 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563780584
[14197634484] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[14198760312] [INFO] [clock] starting clock publisher
[14210289258] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[14211863655] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368009968 RFLAGS_BEFORE=134 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563780584
[14217692049] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ab8
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[14219295420] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368026352 RFLAGS_BEFORE=134 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563780584
[14221449033] [ERROR] [INGESTD] Starting...
[14225488068] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ae8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[14226743091] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368053488 RFLAGS_BEFORE=134 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563780584
[14229545979] [INFO] [cambium] cambium starting (v4: no-op suppression)...
[14236185909] [INFO] [clock] Clock thing created: 356
[14237017344] [INFO] [clock] Waiting for UI Root (Compositor)...
[14242684533] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[14244064758] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[14245242099] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[14252332974] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[14260698309] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[14270418657] [ERROR] [INGESTD] Watch active. Loop start.
[14289690096] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[14291315742] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[14536986288] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[14539266225] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[14541052878] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14544206061] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[14544883386] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14546300571] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[14547149463] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[14554501038] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[14562142485] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[14563840203] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368106288 RFLAGS_BEFORE=134 CR3_BEFORE=54931456 fs_base=0 gs_base=18446744071563780584
[14568152280] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[14588824767] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[14598119151] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[14611149234] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[14612654067] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[14613287073] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[14614443591] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14617599348] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[14618223642] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14619619608] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[14620195722] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[14628324414] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[14630499939] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[14633909994] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[14635998531] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[14638264575] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[14639829897] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[14640429276] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14641524249] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14644032942] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[14644662450] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[14645716899] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[14646372609] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[14653276275] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[14655129357] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[14655803910] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14657458629] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14661124599] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[14662137237] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14663840730] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[14664504294] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[14673262362] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[14675589291] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[14677115772] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[14677971858] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14679659544] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14682901398] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[14683837971] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14685305349] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[14686212255] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[14693740248] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[14710913943] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044678
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[14713110258] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368129888 RFLAGS_BEFORE=134 CR3_BEFORE=55037952 fs_base=0 gs_base=18446744071563780584
[14718921921] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[14720058639] [INFO] [rtc_cmos] Starting... arg=db
[14721929310] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[14725381902] [INFO] [rtc_cmos] RTC: 2026-01-21 03:33:47 = 1768966427 unix_secs
[14727689988] [INFO] [kernel::time] System clock anchored: unix_secs=1768966427, mono_ns=7363453449, offset=1768966419636546551ns
[14729610126] [INFO] [rtc_cmos] System clock anchored
[14751175164] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[14754010821] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ba00
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[14755722564] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368164464 RFLAGS_BEFORE=130 CR3_BEFORE=55140352 fs_base=0 gs_base=18446744071563780584
[14760335370] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[14762628738] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[14766794130] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[14768508513] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[14775567477] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0095f70
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[14777123526] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368181872 RFLAGS_BEFORE=130 CR3_BEFORE=55234560 fs_base=0 gs_base=18446744071563780584
[14780636343] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[14781510480] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[14785636668] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00aabb0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[14786940003] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368207824 RFLAGS_BEFORE=130 CR3_BEFORE=55336960 fs_base=0 gs_base=18446744071563780584
[14790495324] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[14800321569] [INFO] [bristle] bristle: registered in graph as svc.Input (id=457)
[14810036307] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=431 backend=BootFB
[14811383763] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[14811933576] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14813076234] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14880785700] [INFO] [kernel::task::loader] Segment: vaddr=26e020 exec=false
[14881891728] [INFO] [kernel::task::loader]   Overlap at 26e000: merging perms to r=true w=false x=true
[14888800773] [INFO] [kernel::task::loader] Segment: vaddr=278d58 exec=false
[14889550104] [INFO] [kernel::task::loader]   Overlap at 278000: merging perms to r=true w=true x=true
[14897162346] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[14898674901] [INFO] [kernel::task::loader] Loading module: /boot/echo
[14899272630] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14900377866] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14903437032] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[14904472176] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14905668393] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[14906706969] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[14913130551] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[14914282581] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[14915089068] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[14927908875] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014c50
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1af
[14929532805] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368250384 RFLAGS_BEFORE=130 CR3_BEFORE=55443456 fs_base=0 gs_base=18446744071563780584
[14934272595] [INFO] [bloom::logging] bloom: logging initialized
[14958695169] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ee60
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[14961208152] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368267856 RFLAGS_BEFORE=134 CR3_BEFORE=56029184 fs_base=0 gs_base=18446744071563780584
[14966995395] [INFO] [echo] echo: online (handle=12)
[14967918966] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
T:E580 T:E410 [15006124221] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[15007082508] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[15008161047] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
T:D530 [15122559771] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:25D0 [15128933193] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[15130317048] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[15135700635] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[15136717365] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[15162208083] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[15165320049] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[15175692477] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[15181944426] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[15183062334] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[15512469621] [INFO] [ps2_mouse] ps2_mouse: drained 0x28
[16118458686] [INFO] [ps2_mouse] ps2_mouse: drained 0x32
[16244049657] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[16245491955] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[16455109737] [INFO] [ps2_mouse] ps2_mouse: drained 0xce
[16458048618] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[16469546874] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[16470716295] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[18414672672] [INFO] [bloom::compositor] bloom: compositor bytespace 376 (1280x720 stride=5120 format=1)
[19364408151] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[19367716137] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[19369134312] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (258156 bytes)
[19376609274] [INFO] [bloom::compositor] bloom: display backend: BootFB
[19382383251] [INFO] [ps2_mouse] ps2_mouse: init done
[19383673518] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[19385174094] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[19386347013] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[19391487357] [INFO] [bloom::asset] [asset_bank] mapped at 0x10463000
[19392785742] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[19410559509] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[19732256907] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c1b90
[19733705970] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[19734896973] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19736001483] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[19750531350] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[20062444743] [INFO] [stem::ui] UiBuilder: created root 524
[20072714607] [INFO] [display_bootfb] display_bootfb: bound bytespace 376
[20389203846] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd610
[20390154444] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[20390911497] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20391699702] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=495 subj_lo=0
[20725679073] [INFO] [clock] Found UI Root: 524 (attempt 4)
[21245143128] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[21246835665] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[21247945455] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 191 (656852 bytes)
[21265495020] [INFO] [bloom::asset] [asset_bank] mapped at 0x10bab000
[21266440899] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...

```
</details>
