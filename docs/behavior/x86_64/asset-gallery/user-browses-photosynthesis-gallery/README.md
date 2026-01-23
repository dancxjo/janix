# ✅ Scenario: User browses Photosynthesis gallery

> Last run: 2026-01-23 22:35:28

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 49155ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | Then I should see a window with title "Photosynthesis (SVG Grid)" | ✅ | 70144ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And I should see at least 6 asset tiles | ✅ | 2215ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And I should see the "meta.graph.svg" icon in the grid | ✅ | 2156ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[104246124001] [CONTRACT] [kernel] thing-os kernel starting...
[104737621319] [INFO] [kernel::memory] Memory map has 64 entries
[104749440183] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[104750727590] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[104769114542] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[104769753478] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[104770243291] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[104770715761] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[104771111994] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[104771562043] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[104772378629] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78391000 (Usable)
[104772864087] [INFO] [kernel::memory]   [9] 0x78391000 - 0x783f2000 (Reserved)
[104773314688] [INFO] [kernel::memory]   [10] 0x783f2000 - 0x783f3000 (Other)
[104773740539] [INFO] [kernel::memory]   [11] 0x783f3000 - 0x783f4000 (Reserved)
[104781477673] [INFO] [kernel::memory]   [12] 0x783f4000 - 0x783f5000 (Other)
[104782079820] [INFO] [kernel::memory]   [13] 0x783f5000 - 0x783f6000 (Reserved)
[104785172198] [INFO] [kernel::memory]   [14] 0x783f6000 - 0x783f7000 (Other)
[104791945087] [INFO] [kernel::memory]   [15] 0x783f7000 - 0x783f8000 (Reserved)
[104792500532] [INFO] [kernel::memory]   [16] 0x783f8000 - 0x783f9000 (Other)
[104792943433] [INFO] [kernel::memory]   [17] 0x783f9000 - 0x783fa000 (Reserved)
[104793628691] [INFO] [kernel::memory]   [18] 0x783fa000 - 0x783fb000 (Other)
[104794034182] [INFO] [kernel::memory]   [19] 0x783fb000 - 0x783fc000 (Reserved)
[104794510359] [INFO] [kernel::memory]   [20] 0x783fc000 - 0x783fd000 (Other)
[104794972127] [INFO] [kernel::memory]   [21] 0x783fd000 - 0x783fe000 (Reserved)
[104795330015] [INFO] [kernel::memory]   [22] 0x783fe000 - 0x783ff000 (Other)
[104795845576] [INFO] [kernel::memory]   [23] 0x783ff000 - 0x78400000 (Reserved)
[104806463473] [INFO] [kernel::memory]   [24] 0x78400000 - 0x78401000 (Other)
[104806866346] [INFO] [kernel::memory]   [25] 0x78401000 - 0x78402000 (Reserved)
[104807737573] [INFO] [kernel::memory]   [26] 0x78402000 - 0x78403000 (Other)
[104808176938] [INFO] [kernel::memory]   [27] 0x78403000 - 0x78404000 (Reserved)
[104808682465] [INFO] [kernel::memory]   [28] 0x78404000 - 0x78405000 (Other)
[104809019272] [INFO] [kernel::memory]   [29] 0x78405000 - 0x78406000 (Reserved)
[104809607279] [INFO] [kernel::memory]   [30] 0x78406000 - 0x78407000 (Other)
[104810133346] [INFO] [kernel::memory]   [31] 0x78407000 - 0x78408000 (Reserved)
[104810553733] [INFO] [kernel::memory]   [32] 0x78408000 - 0x78409000 (Other)
[104811025212] [INFO] [kernel::memory]   [33] 0x78409000 - 0x7840a000 (Reserved)
[104811465387] [INFO] [kernel::memory]   [34] 0x7840a000 - 0x7840b000 (Other)
[104811979845] [INFO] [kernel::memory]   [35] 0x7840b000 - 0x7840c000 (Reserved)
[104812332961] [INFO] [kernel::memory]   [36] 0x7840c000 - 0x7840d000 (Other)
[104812754055] [INFO] [kernel::memory]   [37] 0x7840d000 - 0x7840e000 (Reserved)
[104813165824] [INFO] [kernel::memory]   [38] 0x7840e000 - 0x7840f000 (Other)
[104813600579] [INFO] [kernel::memory]   [39] 0x7840f000 - 0x78410000 (Reserved)
[104837725499] [INFO] [kernel::memory]   [40] 0x78410000 - 0x78411000 (Other)
[104838233048] [INFO] [kernel::memory]   [41] 0x78411000 - 0x78412000 (Reserved)
[104838663095] [INFO] [kernel::memory]   [42] 0x78412000 - 0x78413000 (Other)
[104839000393] [INFO] [kernel::memory]   [43] 0x78413000 - 0x78414000 (Reserved)
[104839479750] [INFO] [kernel::memory]   [44] 0x78414000 - 0x78415000 (Other)
[104839907587] [INFO] [kernel::memory]   [45] 0x78415000 - 0x78416000 (Reserved)
[104840320038] [INFO] [kernel::memory]   [46] 0x78416000 - 0x78417000 (Other)
[104840867541] [INFO] [kernel::memory]   [47] 0x78417000 - 0x78418000 (Reserved)
[104841309890] [INFO] [kernel::memory]   [48] 0x78418000 - 0x78419000 (Other)
[104841802880] [INFO] [kernel::memory]   [49] 0x78419000 - 0x7841a000 (Reserved)
[104861034749] [INFO] [kernel::memory]   [50] 0x7841a000 - 0x7841b000 (Other)
[104861399265] [INFO] [kernel::memory]   [51] 0x7841b000 - 0x7841c000 (Reserved)
[104861844076] [INFO] [kernel::memory]   [52] 0x7841c000 - 0x7841d000 (Other)
[104862268605] [INFO] [kernel::memory]   [53] 0x7841d000 - 0x7841e000 (Reserved)
[104863944096] [INFO] [kernel::memory]   [54] 0x7841e000 - 0x7841f000 (Other)
[104864377019] [INFO] [kernel::memory]   [55] 0x7841f000 - 0x78420000 (Reserved)
[104868125350] [INFO] [kernel::memory]   [56] 0x78420000 - 0x78421000 (Other)
[104870194541] [INFO] [kernel::memory]   [57] 0x78421000 - 0x78422000 (Reserved)
[104870646346] [INFO] [kernel::memory]   [58] 0x78422000 - 0x78423000 (Other)
[104871053125] [INFO] [kernel::memory]   [59] 0x78423000 - 0x78424000 (Reserved)
[104871697714] [INFO] [kernel::memory]   [60] 0x78424000 - 0x78425000 (Other)
[104872104608] [INFO] [kernel::memory]   [61] 0x78425000 - 0x78426000 (Reserved)
[104872678047] [INFO] [kernel::memory]   [62] 0x78426000 - 0x78427000 (Other)
[104873169087] [INFO] [kernel::memory]   [63] 0x78427000 - 0x78428000 (Reserved)
[104874234153] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[106575470487] [CONTRACT] [kernel::memory] Frame allocator initialized with 488361 free frames
[106622141792] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[106647830952] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[106661199822] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[106662199766] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[106687470065] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[106706465521] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[106725839445] [INFO] [bran::arch] IOAPIC: Registers initialized
[106740035208] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[106752752462] [INFO] [bran::arch] IOAPIC: All pins masked
[106772853966] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[106775013342] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[106775931717] [INFO] [bran::arch] IOAPIC: Init complete
[106776649056] [CONTRACT] [kernel] Initializing global allocator...
[113896980638] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[113912742095] [CONTRACT] [kernel] Initializing SIMD...
[113915860524] [CONTRACT] [kernel] Initializing tasking...
[113942842098] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[113963898193] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[113965300865] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[114016363365] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[114018020536] [INFO] [kernel::task::scheduler]   Initializing boot task...
[114020605762] [INFO] [kernel::task::scheduler]   Creating boot task...
[114115605223] [INFO] [kernel::task::scheduler]   Creating idle task...
[114122160885] [INFO] [kernel::task::scheduler]   Boot task initialized
[114122901870] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[114152055712] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[114179321048] [INFO] [kernel::root] Spawning Root service...
[114379851325] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[114467235818] [INFO] [kernel::root::service] ROOT: started once
[124386727274] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[124392839381] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[124659653552] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[124798200045] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[124983655145] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[125259392869] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[125265246122] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[125495644222] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[125593376257] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[125645075405] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[125670304576] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=290, idx=3) BAR5=0x810c4000
[125806140304] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[125824725810] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[125845750311] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[125850128495] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[125863689930] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[125865228254] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[125896480651] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[126731044274] [INFO] [kernel::task::loader] Segment: vaddr=20e830 exec=false
[126732871919] [INFO] [kernel::task::loader]   Overlap at 20e000: merging perms to r=true w=false x=true
[126754127446] [INFO] [kernel::task::loader] Segment: vaddr=213100 exec=false
[126755868356] [INFO] [kernel::task::loader]   Overlap at 213000: merging perms to r=true w=true x=true
[126796644334] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[126797857007] [CONTRACT] [kernel] Spawning init process...
[126815769783] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[126842045480] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62044300 ticks/sec), init_cnt=620443 for 100Hz
[126844168050] [CONTRACT] [kernel] Entering scheduler loop.
[126891091989] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[126911024190] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562091133 RSP_BEFORE=18446744072367777584 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563790480
[127319131946] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9b0 rip=0x2003af rflags=0x202
[127339778122] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[127341202704] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[127342149845] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[127430648648] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[127461375013] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[127483621948] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[127485054445] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[127508385227] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[127523343741] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[127524543703] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[127592228844] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[127593634718] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[127594767416] [INFO] [sprout::devtree] SPROUT: build() called
[127611729204] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[127648762829] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[127650227187] [INFO] [sprout] SPROUT: About to create Supervisor...
[127655063599] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[127661900442] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[127663343528] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[127698587781] [INFO] [sprout::supervisor] SPROUT: Found 64 modules
[127881239284] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[127917619886] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[128117074741] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[128137260389] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[128159855182] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[128183787407] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[128226880838] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[128240446686] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[128263773621] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[128295236307] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[128317870727] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[128342819179] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[128370647889] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[128408138535] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[128442611458] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[128454773576] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[128470969094] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[128501425784] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[128724672895] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/boot/photosynthesis'
[128746265769] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Alternate.cur'
[128779895927] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Busy.cur'
[128814178526] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal1.ani'
[128843226821] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Diagonal2.ani'
[128857923831] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Handwriting.cur'
[128885289788] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Help.cur'
[128914914400] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Horizontal.ani'
[128937776195] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Link.ani'
[128966671736] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Move.cur'
[129002716684] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Normal.cur'
[129041410386] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Precision.cur'
[129056906742] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Text.cur'
[129092397508] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Unavailabe.cur'
[129118378363] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Vertical.ani'
[129125196395] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/cursors/plain/Working.ani'
[129167202652] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/clouds.bmp'
[129188954216] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/leather.bmp'
[129225741183] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/wallpapers/linen.bmp'
[129246851041] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[129286848960] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/Hack-Regular.ttf'
[129314322225] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSans-Regular.ttf'
[129338309769] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[129365885152] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[129389084974] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/fonts/NotoSerif-Regular.ttf'
[129433908299] [INFO] [sprout::supervisor] SPROUT: Module[42] = '/assets/pci/pci.ids'
[129440097165] [INFO] [sprout::supervisor] SPROUT: Module[43] = '/assets/icons/thingos/bran.bran.svg'
[129461007825] [INFO] [sprout::supervisor] SPROUT: Module[44] = '/assets/icons/thingos/dev.host.svg'
[129467850162] [INFO] [sprout::supervisor] SPROUT: Module[45] = '/assets/icons/thingos/dev.input.svg'
[129542354788] [INFO] [sprout::supervisor] SPROUT: Module[46] = '/assets/icons/thingos/dev.network.svg'
[129549828714] [INFO] [sprout::supervisor] SPROUT: Module[47] = '/assets/icons/thingos/dev.output.svg'
[129573752277] [INFO] [sprout::supervisor] SPROUT: Module[48] = '/assets/icons/thingos/dev.storage.svg'
[129600729601] [INFO] [sprout::supervisor] SPROUT: Module[49] = '/assets/icons/thingos/kind.bytespace.svg'
[129628524411] [INFO] [sprout::supervisor] SPROUT: Module[50] = '/assets/icons/thingos/mem.heap.svg'
[129740340846] [INFO] [sprout::supervisor] SPROUT: Module[51] = '/assets/icons/thingos/mem.page.svg'
[129784300888] [INFO] [sprout::supervisor] SPROUT: Module[52] = '/assets/icons/thingos/mem.stack.svg'
[129798418768] [INFO] [sprout::supervisor] SPROUT: Module[53] = '/assets/icons/thingos/meta.alert.svg'
[129830895851] [INFO] [sprout::supervisor] SPROUT: Module[54] = '/assets/icons/thingos/meta.annotation.svg'
[129869401628] [INFO] [sprout::supervisor] SPROUT: Module[55] = '/assets/icons/thingos/meta.graph.svg'
[129902946355] [INFO] [sprout::supervisor] SPROUT: Module[56] = '/assets/icons/thingos/meta.metric.svg'
[129939073186] [INFO] [sprout::supervisor] SPROUT: Module[57] = '/assets/icons/thingos/meta.namespace.svg'
[129945322696] [INFO] [sprout::supervisor] SPROUT: Module[58] = '/assets/icons/thingos/meta.trace.svg'
[129988487000] [INFO] [sprout::supervisor] SPROUT: Module[59] = '/assets/icons/thingos/meta.version.svg'
[130042959584] [INFO] [sprout::supervisor] SPROUT: Module[60] = '/assets/icons/thingos/proc.job.svg'
[130053131045] [INFO] [sprout::supervisor] SPROUT: Module[61] = '/assets/icons/thingos/proc.kernel.svg'
[130106933267] [INFO] [sprout::supervisor] SPROUT: Module[62] = '/assets/icons/thingos/proc.task.svg'
[130152612590] [INFO] [sprout::supervisor] SPROUT: Module[63] = '/assets/icons/thingos/proc.thread.svg'
[130168149222] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[130253015729] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[131027694520] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[131030196250] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[131051283495] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[131053601304] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[131062087037] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[131063780246] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/photosynthesis'
[131066094120] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[131089481079] [INFO] [kernel::task::loader] Loading module: /boot/clock
[131090795515] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[131092283200] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[131119573806] [INFO] [kernel::task::loader] Segment: vaddr=206ab0 exec=false
[131130549053] [INFO] [kernel::task::loader]   Overlap at 206000: merging perms to r=true w=false x=true
[131134665878] [INFO] [kernel::task::loader] Segment: vaddr=209d48 exec=false
[131138308437] [INFO] [kernel::task::loader]   Overlap at 209000: merging perms to r=true w=true x=true
[131177380710] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[131200098994] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[131201611688] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[131204433424] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[131205949399] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[131244165676] [INFO] [kernel::task::loader] Segment: vaddr=2061e0 exec=false
[131245370285] [INFO] [kernel::task::loader]   Overlap at 206000: merging perms to r=true w=false x=true
[131248112860] [INFO] [kernel::task::loader] Segment: vaddr=2087f0 exec=false
[131249023840] [INFO] [kernel::task::loader]   Overlap at 208000: merging perms to r=true w=true x=true
[131285532257] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[131287460802] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[131302500320] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[131303797132] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[131305082300] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[131363593701] [INFO] [kernel::task::loader] Segment: vaddr=2117e0 exec=false
[131364815005] [INFO] [kernel::task::loader]   Overlap at 211000: merging perms to r=true w=false x=true
[131388374862] [INFO] [kernel::task::loader] Segment: vaddr=218c40 exec=false
[131393078741] [INFO] [kernel::task::loader]   Overlap at 218000: merging perms to r=true w=true x=true
[131429575967] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[131432837186] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[131441274099] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[131442312452] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[131443449932] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[131465816476] [INFO] [kernel::task::loader] Segment: vaddr=2058d0 exec=false
[131475282635] [INFO] [kernel::task::loader]   Overlap at 205000: merging perms to r=true w=false x=true
[131484993280] [INFO] [kernel::task::loader] Segment: vaddr=206c58 exec=false
[131486244417] [INFO] [kernel::task::loader]   Overlap at 206000: merging perms to r=true w=true x=true
[131524616825] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[131525952756] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/photosynthesis'
[131527561284] [INFO] [kernel::task::loader] Loading module: /boot/photosynthesis
[131528419674] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[131547004143] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[131568553561] [INFO] [kernel::task::loader] Segment: vaddr=205ec0 exec=false
[131575961787] [INFO] [kernel::task::loader]   Overlap at 205000: merging perms to r=true w=false x=true
[131585073647] [INFO] [kernel::task::loader] Segment: vaddr=208a28 exec=false
[131586484747] [INFO] [kernel::task::loader]   Overlap at 208000: merging perms to r=true w=true x=true
[131767323111] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0038878
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[131851762565] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562091133 RSP_BEFORE=18446744072367838928 RFLAGS_BEFORE=130 CR3_BEFORE=50757632 fs_base=0 gs_base=18446744071563790480
[131877261655] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff4e0 rip=0x20002f rflags=0x202
[131879425215] [INFO] [clock] starting clock publisher
[131963911117] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[131966036648] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562091133 RSP_BEFORE=18446744072367861968 RFLAGS_BEFORE=130 CR3_BEFORE=50884608 fs_base=0 gs_base=18446744071563790480
[132003275081] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0030a58
USER_TRAMPOLINE: PC=0x208460 SP=0x800000 ARG0=0x0
[132010712262] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2131040 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562091133 RSP_BEFORE=18446744072367878352 RFLAGS_BEFORE=130 CR3_BEFORE=51007488 fs_base=0 gs_base=18446744071563790480
[132020377107] [ERROR] [INGESTD] Starting...
[132049633895] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0030a88
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[132051886485] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562091133 RSP_BEFORE=18446744072367905488 RFLAGS_BEFORE=130 CR3_BEFORE=51195904 fs_base=0 gs_base=18446744071563790480
[132069256663] [INFO] [cambium] cambium starting (v5.2: drain-to-eagain + smart-resync)...
[132102852235] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0030ff0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[132119974408] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562091133 RSP_BEFORE=18446744072367921872 RFLAGS_BEFORE=130 CR3_BEFORE=51310592 fs_base=0 gs_base=18446744071563790480
[132137202784] [INFO] [photosynthesis] Photosynthesis starting...
[132165341659] [INFO] [sprout::supervisor] SPROUT: App launched (PID=8)
[132167151091] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[132202073559] [INFO] [clock] Clock thing created: 452
[132203789401] [INFO] [clock] Waiting for UI Root (Compositor)...
[132250548444] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[132256489914] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[132258286914] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[132265944304] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[132319333000] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[132383877781] [ERROR] [INGESTD] Watch active. Loop start.
[132464682275] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[132473244103] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[133739312560] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[133747001947] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[133749321457] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[133769198594] [INFO] [kernel::task::loader] Segment: vaddr=203010 exec=false
[133770763670] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[133781053940] [INFO] [kernel::task::loader] Segment: vaddr=203e00 exec=false
[133783761275] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[133808665943] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=9)
[133868298100] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004230
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[133870595091] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562091133 RSP_BEFORE=18446744072368471232 RFLAGS_BEFORE=134 CR3_BEFORE=55119872 fs_base=0 gs_base=18446744071563790480
[133893041730] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[134188189757] [INFO] [kernel::syscall::handlers::device] DEVICE: task 9 claimed device 265 (handle 0)
[134230637240] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[134316677982] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[134327120442] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[134328523880] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[134342236660] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[134348777942] [INFO] [kernel::task::loader] Segment: vaddr=203cb0 exec=false
[134350326618] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[134352714797] [INFO] [kernel::task::loader] Segment: vaddr=204d70 exec=false
[134369915450] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[134407062356] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=10)
[134409312011] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[134426888295] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[134430256880] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[134450201044] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[134452925623] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[134454702214] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[134461910042] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[134477424911] [INFO] [kernel::task::loader] Segment: vaddr=202580 exec=false
[134479095888] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[134481387316] [INFO] [kernel::task::loader] Segment: vaddr=2030e0 exec=false
[134499547739] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[134528324188] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=11)
[134531357214] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[134532384740] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[134533962150] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[134562027106] [INFO] [kernel::task::loader] Segment: vaddr=202d60 exec=false
[134563537456] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[134574355587] [INFO] [kernel::task::loader] Segment: vaddr=203af0 exec=false
[134575808899] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[134682367071] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00366f8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x11d
[134701338786] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562091133 RSP_BEFORE=18446744072368495040 RFLAGS_BEFORE=134 CR3_BEFORE=55230464 fs_base=0 gs_base=18446744071563790480
[134714701556] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[134716793813] [INFO] [rtc_cmos] Starting... arg=11d
[134742009279] [INFO] [rtc_cmos] Serving device ID: ThingId([29, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[134760918394] [INFO] [rtc_cmos] RTC: 2026-01-23 22:36:50 = 1769207810 unix_secs
[134762992229] [INFO] [kernel::time] System clock anchored: unix_secs=1769207810, mono_ns=67381122501, offset=1769207742618877499ns
[134765294836] [INFO] [rtc_cmos] System clock anchored
[134871000299] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[134890338649] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb003d1f0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[134892087197] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562091133 RSP_BEFORE=18446744072368529552 RFLAGS_BEFORE=134 CR3_BEFORE=55336960 fs_base=0 gs_base=18446744071563790480
[134913770903] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[134917177354] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[134943459901] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[134945297765] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[134977545139] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb003dc20
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[134989688612] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562091133 RSP_BEFORE=18446744072368547056 RFLAGS_BEFORE=134 CR3_BEFORE=55439360 fs_base=0 gs_base=18446744071563790480
[135014588931] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[135019746861] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[135049702684] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=12)
[135072210916] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[135074258686] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[135078871053] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[135091151967] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[135096746793] [INFO] [kernel::task::loader] Segment: vaddr=202e80 exec=false
[135109898524] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[135124634773] [INFO] [kernel::task::loader] Segment: vaddr=203ef8 exec=false
[135126565814] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[135156027726] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=13)
[135229194966] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb003dc20
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[135231454775] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562091133 RSP_BEFORE=18446744072368593744 RFLAGS_BEFORE=134 CR3_BEFORE=55541760 fs_base=0 gs_base=18446744071563790480
[135262571427] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[135353369872] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[135355564216] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[135363565635] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
[135399224760] [INFO] [bristle] bristle: registered in graph as svc.Input (id=558)
[135441598274] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=553 backend=BootFB
[135464730286] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[135465967089] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[135483092371] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[135943635882] [INFO] [kernel::task::loader] Segment: vaddr=2a8fa0 exec=false
[135976904513] [INFO] [kernel::task::loader]   Overlap at 2a8000: merging perms to r=true w=false x=true
[136008363480] [INFO] [kernel::task::loader] Segment: vaddr=2b9950 exec=false
[136023859778] [INFO] [kernel::task::loader]   Overlap at 2b9000: merging perms to r=true w=true x=true
[136052936778] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=14)
[136069427874] [INFO] [kernel::task::loader] Loading module: /boot/echo
[136084425206] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[136086500887] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[136091442135] [INFO] [kernel::task::loader] Segment: vaddr=2027b0 exec=false
[136104563153] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[136112749496] [INFO] [kernel::task::loader] Segment: vaddr=2036a8 exec=false
[136114180965] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[136145504219] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=15)
[136159620531] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[136219135413] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb003d1f0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x229
[136242750909] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562091133 RSP_BEFORE=18446744072368620592 RFLAGS_BEFORE=134 CR3_BEFORE=55652352 fs_base=0 gs_base=18446744071563790480
[136269679531] [INFO] [bloom::logging] bloom: logging initialized
[136874211787] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb003e998
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[136876441444] [INFO] [task.user_enter] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562091133 RSP_BEFORE=18446744072368638064 RFLAGS_BEFORE=130 CR3_BEFORE=56504320 fs_base=0 gs_base=18446744071563790480
[136882242285] [INFO] [echo] echo: online (handle=12)
[136909196956] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[136943314020] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
T:A8A0 T:A5E0 T:9A50 [137631860814] [INFO] [ps2_mouse] ps2_mouse: init done
[137633285680] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[137634813135] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[137666719161] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[137965871710] [INFO] [bloom] bloom cursor_loader started
[138579809575] [INFO] [bloom::asset] [asset_bank] worker spawned tid=19 (priority=2)
T:53D0 [138629067348] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[138631798950] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (23272 bytes)
[138697258725] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[138699447179] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[138814638253] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[138818864564] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[138931941588] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (309408 bytes)
[138981722030] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[138983445047] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[142342414997] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[142364260672] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[142366069891] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (569208 bytes)
[142527391668] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[142531759064] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[150505929759] [INFO] [bloom::compositor] bloom: compositor bytespace 477 (1280x720 stride=5120 format=1)
[151679214354] [INFO] [bloom::compositor] bloom: display backend: BootFB
[152003433092] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[152426993588] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[152760311325] [INFO] [display_bootfb] display_bootfb: bound bytespace 477
[153300597148] [INFO] [stem::ui] UiBuilder: created root 613
[153306843030] [INFO] [bloom::ui] bloom: [bloom][ui] Initializing cached UI symbols (one-time)
[153767525365] [INFO] [clock] Found UI Root: 613 (attempt 10)
[153823444661] [INFO] [photosynthesis] Found UI Root: 613
[157509234221] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c2930
[157510807248] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[157511741843] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[157513216600] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=533 pred=0 subj_lo=0
[157578459706] [INFO] [photosynthesis] Searching boot modules...
[158086818036] [INFO] [photosynthesis] Found 64 boot modules
[160781940831] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c2930
[160788899204] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[160790254781] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[160806588803] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=536 pred=0 subj_lo=0
[164065012553] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c2930
[164066651658] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[164067703361] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[164068714352] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=538 pred=0 subj_lo=0
[170911460839] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c2930
[170913057650] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[170914177250] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[170915189658] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=544 pred=0 subj_lo=0
[178562574174] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[178572216198] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[178573158406] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[178576209489] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=543 subj_lo=0
[178716681784] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[178733545707] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[178742419497] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 191 (258156 bytes)
[178824437540] [INFO] [bloom::asset] [asset_bank] mapped at 0x10b6b000
[178826077866] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[178934043615] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[178952600653] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[178953689740] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[178954724018] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=545 subj_lo=0
[184222564234] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[184232722114] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[184242133485] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[184243120617] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=550 subj_lo=0
[192791084638] [INFO] [clock] Clock icon not found
[198371182031] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[198372560275] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[198373591526] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[198374545866] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=553 subj_lo=0
[199085430115] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[199087734213] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[199088986749] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[199089949738] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=554 subj_lo=0
[199825466338] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[199832392456] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[199837896821] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[199839165238] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=556 subj_lo=0
[200516335315] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[200520323076] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[200521280983] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[200522413535] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=557 subj_lo=0
[200580973265] [INFO] [clock] CLOCK: Entering main loop, publishing to thing_id=452
[200596284931] [INFO] [photosynthesis] Found 21 SVG assets
[200888506470] [INFO] [clock] unix=1769207842 utc=2026-01-23 22:37:22 mono_ns=100291423309
[201280468751] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[201289108256] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[201293219671] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[201294400226] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=559 subj_lo=0
[201639488903] [INFO] [bloom] bloom cursor_loader found asset: /assets/cursors/plain/Normal.cur
[202071773182] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[202078698748] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[202088595499] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[202089588763] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=561 subj_lo=0
[204794400879] [INFO] [clock] CLOCK PUBLISH: thing=452 now_text='22:37:22' tick=100291423309
[207906212689] [INFO] [clock] unix=1769207846 utc=2026-01-23 22:37:26 mono_ns=103952731788
[211270210555] [INFO] [photosynthesis] Tile /assets/icons/thingos/bran.bran.svg -> 727 (203)
[213001814686] [INFO] [clock] CLOCK PUBLISH: thing=452 now_text='22:37:26' tick=103952731788
[214296122495] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[214297694945] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[214299027582] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[214300226260] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=563 subj_lo=0
[214964507957] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[214975901636] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[214983084837] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[214993581868] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=565 subj_lo=0
[215254294802] [INFO] [photosynthesis] Tile /assets/icons/thingos/dev.host.svg -> 730 (206)
[215519339806] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[215534334938] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[215535533836] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[215538526138] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=566 subj_lo=0
[215581849470] [INFO] [clock] unix=1769207850 utc=2026-01-23 22:37:30 mono_ns=107790639077
[216194978937] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[216197417922] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[216217736423] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[216219203208] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=568 subj_lo=0
[216840814347] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[216851047918] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[216866844494] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[216868039105] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=570 subj_lo=0
[217423910142] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[217425478718] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[217426524754] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[217427648088] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=576 subj_lo=0
[217472125554] [INFO] [clock] CLOCK PUBLISH: thing=452 now_text='22:37:30' tick=107790639077
[217827733648] [INFO] [photosynthesis] Tile /assets/icons/thingos/dev.input.svg -> 743 (209)
[218086247568] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[218089208063] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[218097437827] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[218098599810] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=578 subj_lo=0
[218649802703] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[218669379899] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[218671289102] [INFO] [bloom::asset] [asset_bank] load_wallpaper_immediate: /assets/wallpapers/clouds.bmp
[218697605736] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[218728087391] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[218729418949] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[218730467791] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=579 subj_lo=0
[218808589064] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[218827346085] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[218828521497] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[218829725500] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=581 subj_lo=0
[218908416623] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[218920898271] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[218922207815] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[218923438904] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=583 subj_lo=0
[218972061977] [INFO] [photosynthesis] Tile /assets/icons/thingos/dev.network.svg -> 767 (212)
[219012247980] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[219020797750] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[219022597753] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[219032010559] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=584 subj_lo=0
[219105375324] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[219107060837] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[219108333938] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[219109780750] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=585 subj_lo=0
[219200515879] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[219235552125] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[219236932774] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[219238256822] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=586 subj_lo=0
[219322527253] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[219324176716] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[219325370878] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[219326530802] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=588 subj_lo=0
[219409529599] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[219410898014] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[219411925052] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[219437061591] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=590 subj_lo=0
[219456959355] [INFO] [photosynthesis] Tile /assets/icons/thingos/dev.output.svg -> 792 (215)
[219524884418] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[219526335937] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[219544696656] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[219546061586] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=591 subj_lo=0
[219633854624] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[219635335667] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[219636516903] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[219637826314] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=592 subj_lo=0
[219708570358] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[219730787006] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[219732624603] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[219740535236] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=594 subj_lo=0
[219843776113] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[219845343976] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[219846546561] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[219857575498] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=595 subj_lo=0
[219904486108] [INFO] [photosynthesis] Tile /assets/icons/thingos/dev.storage.svg -> 819 (218)
[219970081343] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[219972214350] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[219973462217] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[219974553140] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=596 subj_lo=0
[220088522381] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[220090170058] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[220091574001] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[220092816899] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=597 subj_lo=0
[220184276207] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7ff0c0
[220186038665] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[220187320185] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[220188594583] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=598 subj_lo=0
[220277891468] [INFO] [photosynthesis] Tile /assets/icons/thingos/kind.bytespace.svg -> 841 (221)
[220313254666] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 102400)
[220328605434] [INFO] [bloom::asset] [asset_bank] promoting font 'DSEG7Classic-Regular.ttf' to gen=1 (102400b) in slot 0
[220340890001] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 204800)
[220342447620] [INFO] [bloom::asset] [asset_bank] promoting font 'Hack-Regular.ttf' to gen=1 (102400b) in slot 1
[220343947077] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 307200)
[220345149263] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=1 (102400b) in slot 2
[220346622254] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 409600)
[220358497671] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol-Regular.ttf' to gen=1 (102400b) in slot 3
[220416106673] [INFO] [clock] unix=1769207852 utc=2026-01-23 22:37:32 mono_ns=110207736015
[220667229692] [INFO] [clock] CLOCK PUBLISH: thing=452 now_text='22:37:32' tick=110207736015
[220750784255] [INFO] [photosynthesis] Tile /assets/icons/thingos/mem.heap.svg -> 858 (224)
[220771192173] [INFO] [bloom::asset] [asset_bank] mapping bytespace 173 (3145782 bytes)
[220847979078] [INFO] [bloom::asset] [asset_bank] mapped to 0x10bab000
[220849874386] [INFO] [bloom::asset] [asset_bank] decoding BMP...
[222844306211] [INFO] [bloom::asset] [asset_bank] BMP decoded: 1024x1024
[224797223216] [INFO] [bloom::asset] [asset_bank] bytespace unmapped
[224799085125] [INFO] [bloom::asset] [asset_bank] publish_wallpaper (pending): 1024x1024
[224882407946] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 194 (656852 bytes)
[224926239873] [INFO] [bloom::asset] [asset_bank] mapped at 0x10eac000
[224947198231] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[225700064692] [INFO] [photosynthesis] Tile /assets/icons/thingos/mem.page.svg -> 870 (227)
[226913948080] [INFO] [clock] unix=1769207856 utc=2026-01-23 22:37:36 mono_ns=113456674132
[227685594962] [INFO] [photosynthesis] Tile /assets/icons/thingos/mem.stack.svg -> 881 (230)
[228495150378] [INFO] [clock] CLOCK PUBLISH: thing=452 now_text='22:37:36' tick=113456674132
[229777180807] [INFO] [photosynthesis] Tile /assets/icons/thingos/meta.alert.svg -> 884 (233)
[231365867857] [INFO] [clock] unix=1769207858 utc=2026-01-23 22:37:38 mono_ns=115682645115
[233483491858] [INFO] [photosynthesis] Tile /assets/icons/thingos/meta.annotation.svg -> 887 (236)
[235164467462] [INFO] [clock] CLOCK PUBLISH: thing=452 now_text='22:37:38' tick=115682645115
[237804671188] [INFO] [photosynthesis] Tile /assets/icons/thingos/meta.graph.svg -> 890 (239)
[238624628323] [INFO] [clock] unix=1769207861 utc=2026-01-23 22:37:41 mono_ns=119311955423
[241173827176] [INFO] [clock] CLOCK PUBLISH: thing=452 now_text='22:37:41' tick=119311955423
[241193212911] [INFO] [photosynthesis] Tile /assets/icons/thingos/meta.metric.svg -> 893 (242)
[244546092818] [INFO] [clock] unix=1769207864 utc=2026-01-23 22:37:44 mono_ns=122272689608
[244858387312] [INFO] [photosynthesis] Tile /assets/icons/thingos/meta.namespace.svg -> 897 (245)
[247110881707] [INFO] [clock] CLOCK PUBLISH: thing=452 now_text='22:37:44' tick=122272689608
[248585379599] [INFO] [photosynthesis] Tile /assets/icons/thingos/meta.trace.svg -> 900 (248)
[250317322687] [INFO] [clock] unix=1769207867 utc=2026-01-23 22:37:47 mono_ns=125158279606
[253107044965] [INFO] [photosynthesis] Tile /assets/icons/thingos/meta.version.svg -> 903 (251)
[253655824815] [INFO] [clock] CLOCK PUBLISH: thing=452 now_text='22:37:47' tick=125158279606
[256549478561] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[256571440480] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol2-Regular.ttf' in slot 4
[256576573475] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 197 (616196 bytes)
[257019487736] [INFO] [clock] unix=1769207871 utc=2026-01-23 22:37:51 mono_ns=128506115974
[257050724243] [INFO] [bloom::asset] [asset_bank] mapped at 0x10f4d000
[257067467550] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSerif-Regular.ttf'...
[257190057611] [INFO] [photosynthesis] Tile /assets/icons/thingos/proc.job.svg -> 906 (254)
[261361804464] [INFO] [clock] CLOCK PUBLISH: thing=452 now_text='22:37:51' tick=128506115974
[262925896578] [INFO] [photosynthesis] Tile /assets/icons/thingos/proc.kernel.svg -> 915 (257)
[265088466207] [INFO] [clock] unix=1769207875 utc=2026-01-23 22:37:55 mono_ns=132543581246
[267589713403] [INFO] [photosynthesis] Tile /assets/icons/thingos/proc.task.svg -> 918 (260)
[268644793476] [INFO] [clock] CLOCK PUBLISH: thing=452 now_text='22:37:55' tick=132543581246
[272238323745] [INFO] [photosynthesis] Tile /assets/icons/thingos/proc.thread.svg -> 921 (263)
[272270373080] [INFO] [photosynthesis] Photosynthesis ready. Floating...
[272314919892] [INFO] [clock] unix=1769207878 utc=2026-01-23 22:37:58 mono_ns=136142381422
[275353478621] [INFO] [clock] CLOCK PUBLISH: thing=452 now_text='22:37:58' tick=136142381422
[278703153581] [INFO] [clock] unix=1769207881 utc=2026-01-23 22:38:01 mono_ns=139351199540
[282714181255] [INFO] [clock] CLOCK PUBLISH: thing=452 now_text='22:38:01' tick=139351199540
[285836621899] [INFO] [clock] unix=1769207885 utc=2026-01-23 22:38:05 mono_ns=142917927485
[288982019731] [INFO] [clock] CLOCK PUBLISH: thing=452 now_text='22:38:05' tick=142917927485
[292600940860] [INFO] [clock] unix=1769207888 utc=2026-01-23 22:38:08 mono_ns=146300132219

```
</details>
