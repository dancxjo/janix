# ✅ Scenario: Serve telnet connection

> Last run: 2026-04-05 19:59:17

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the telnet server is ready | ✅ | 9324ms | - - - |
| 2 | When I connect to the telnet server and send "match (n) return n;" | ✅ | 17003ms | - [📜](./02/serial.log) - |
| 3 | Then the telnet response should contain "node(" | ✅ | 0ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[15364142079] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[15372998883] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[15378652410] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[15380880636] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[15382129620] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[15382819551] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[15383520306] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[15384127638] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[15384741900] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[15385427277] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=33264
[15386366424] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=973464
[15387298278] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[15388309497] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[15389014377] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[15389761563] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[15390405690] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[15391079715] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[15391705230] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[15392348103] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[15392998731] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[15393602202] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[15394241511] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[15395164092] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[15396143400] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[15397009056] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[15397726575] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[15398362452] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[15399065682] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[15399681528] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[15400310409] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[15400947342] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[15401588763] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[15402323541] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[15403093068] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=168464
[15404042478] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[15405041586] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[15405904998] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[15406654758] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[15407380428] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[15408138438] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[15408937170] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[15409685346] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[15411115236] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[15413416524] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[15414299010] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[15414868524] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[15415401342] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[15415978611] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[15416561787] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[15417109719] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[15417649038] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[15418193703] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[15418732494] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[15419322303] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7795e000 (Usable)
[15419973129] [INFO] [kernel::memory] [CPU0]   [11] 0x7795e000 - 0x779c2000 (Reserved)
[15420878055] [INFO] [kernel::memory] [CPU0]   [12] 0x779c2000 - 0x779c3000 (Other)
[15421648935] [INFO] [kernel::memory] [CPU0]   [13] 0x779c3000 - 0x779c4000 (Reserved)
[15422376090] [INFO] [kernel::memory] [CPU0]   [14] 0x779c4000 - 0x779c5000 (Other)
[15422955735] [INFO] [kernel::memory] [CPU0]   [15] 0x779c5000 - 0x779c6000 (Reserved)
[15423550725] [INFO] [kernel::memory] [CPU0]   [16] 0x779c6000 - 0x779c7000 (Other)
[15424125255] [INFO] [kernel::memory] [CPU0]   [17] 0x779c7000 - 0x779c8000 (Reserved)
[15424717407] [INFO] [kernel::memory] [CPU0]   [18] 0x779c8000 - 0x77a53000 (Other)
[15425290056] [INFO] [kernel::memory] [CPU0]   [19] 0x77a53000 - 0x77a54000 (Reserved)
[15425908146] [INFO] [kernel::memory] [CPU0]   [20] 0x77a54000 - 0x77ed5000 (Other)
[15426478518] [INFO] [kernel::memory] [CPU0]   [21] 0x77ed5000 - 0x77ed6000 (Reserved)
[15427069713] [INFO] [kernel::memory] [CPU0]   [22] 0x77ed6000 - 0x77ff7000 (Other)
[15427641372] [INFO] [kernel::memory] [CPU0]   [23] 0x77ff7000 - 0x77ff8000 (Reserved)
[15428248605] [INFO] [kernel::memory] [CPU0]   [24] 0x77ff8000 - 0x787f8000 (Other)
[15429136899] [INFO] [kernel::memory] [CPU0]   [25] 0x787f8000 - 0x787f9000 (Reserved)
[15429994371] [INFO] [kernel::memory] [CPU0]   [26] 0x787f9000 - 0x788ba000 (Other)
[15430661202] [INFO] [kernel::memory] [CPU0]   [27] 0x788ba000 - 0x788bb000 (Reserved)
[15431335821] [INFO] [kernel::memory] [CPU0]   [28] 0x788bb000 - 0x788e5000 (Other)
[15431909031] [INFO] [kernel::memory] [CPU0]   [29] 0x788e5000 - 0x788e6000 (Reserved)
[15432526659] [INFO] [kernel::memory] [CPU0]   [30] 0x788e6000 - 0x788eb000 (Other)
[15433098681] [INFO] [kernel::memory] [CPU0]   [31] 0x788eb000 - 0x788ec000 (Reserved)
[15433687698] [INFO] [kernel::memory] [CPU0]   [32] 0x788ec000 - 0x788f1000 (Other)
[15434257707] [INFO] [kernel::memory] [CPU0]   [33] 0x788f1000 - 0x788f2000 (Reserved)
[15434848869] [INFO] [kernel::memory] [CPU0]   [34] 0x788f2000 - 0x7891e000 (Other)
[15435483789] [INFO] [kernel::memory] [CPU0]   [35] 0x7891e000 - 0x78920000 (Reserved)
[15436078911] [INFO] [kernel::memory] [CPU0]   [36] 0x78920000 - 0x78929000 (Other)
[15436649976] [INFO] [kernel::memory] [CPU0]   [37] 0x78929000 - 0x7892b000 (Reserved)
[15437388087] [INFO] [kernel::memory] [CPU0]   [38] 0x7892b000 - 0x78933000 (Other)
[15438274797] [INFO] [kernel::memory] [CPU0]   [39] 0x78933000 - 0x78934000 (Reserved)
[15439076697] [INFO] [kernel::memory] [CPU0]   [40] 0x78934000 - 0x7893e000 (Other)
[15439759434] [INFO] [kernel::memory] [CPU0]   [41] 0x7893e000 - 0x7893f000 (Reserved)
[15440355909] [INFO] [kernel::memory] [CPU0]   [42] 0x7893f000 - 0x7894c000 (Other)
[15440927469] [INFO] [kernel::memory] [CPU0]   [43] 0x7894c000 - 0x7894e000 (Reserved)
[15441522426] [INFO] [kernel::memory] [CPU0]   [44] 0x7894e000 - 0x7895c000 (Other)
[15442142298] [INFO] [kernel::memory] [CPU0]   [45] 0x7895c000 - 0x7895d000 (Reserved)
[15442735704] [INFO] [kernel::memory] [CPU0]   [46] 0x7895d000 - 0x78969000 (Other)
[15443307792] [INFO] [kernel::memory] [CPU0]   [47] 0x78969000 - 0x7896a000 (Reserved)
[15443902947] [INFO] [kernel::memory] [CPU0]   [48] 0x7896a000 - 0x7896e000 (Other)
[15444478896] [INFO] [kernel::memory] [CPU0]   [49] 0x7896e000 - 0x7896f000 (Reserved)
[15445077153] [INFO] [kernel::memory] [CPU0]   [50] 0x7896f000 - 0x7897f000 (Other)
[15445745667] [INFO] [kernel::memory] [CPU0]   [51] 0x7897f000 - 0x78980000 (Reserved)
[15446658381] [INFO] [kernel::memory] [CPU0]   [52] 0x78980000 - 0x78a10000 (Other)
[15447511431] [INFO] [kernel::memory] [CPU0]   [53] 0x78a10000 - 0x78a11000 (Reserved)
[15448216080] [INFO] [kernel::memory] [CPU0]   [54] 0x78a11000 - 0x78a1c000 (Other)
[15448835820] [INFO] [kernel::memory] [CPU0]   [55] 0x78a1c000 - 0x78a1d000 (Reserved)
[15449430150] [INFO] [kernel::memory] [CPU0]   [56] 0x78a1d000 - 0x78a42000 (Other)
[15450003888] [INFO] [kernel::memory] [CPU0]   [57] 0x78a42000 - 0x78a43000 (Reserved)
[15450595710] [INFO] [kernel::memory] [CPU0]   [58] 0x78a43000 - 0x78a4f000 (Other)
[15451167369] [INFO] [kernel::memory] [CPU0]   [59] 0x78a4f000 - 0x78a50000 (Reserved)
[15451783512] [INFO] [kernel::memory] [CPU0]   [60] 0x78a50000 - 0x78a5d000 (Other)
[15452359593] [INFO] [kernel::memory] [CPU0]   [61] 0x78a5d000 - 0x78a5e000 (Reserved)
[15452953890] [INFO] [kernel::memory] [CPU0]   [62] 0x78a5e000 - 0x78aa6000 (Other)
[15453531654] [INFO] [kernel::memory] [CPU0]   [63] 0x78aa6000 - 0x78aa7000 (Reserved)
[15454535778] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[15724826832] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485750 free frames
[15743700753] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[15754404237] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[15756411264] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[15757778289] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[15766276185] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[15777297030] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[15782314581] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[15783422721] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[15784521951] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[15790312923] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[15792016647] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[15793562004] [INFO] [:arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[15799780524] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[15801003009] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[15802092207] [INFO] [bran::arch::x86_64::acpi] [CPU0All pins masked
[15814180866] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[15815889540] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[15816831987] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[15818093610] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[16389127953] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[16390781517] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[16396284927] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[16397886813] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[16399165332] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[16401768075] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[16422956088] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[16425290046] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[16426528074] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[16429495302] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[16430295981] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[16432899780] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[16441898187] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[16443781002] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[16458812172] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[16459512168] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[16477700019] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[16478633919] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[16481039124] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[16482406512] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[16483674966] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[16486156632] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[16487435316] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[16523715648] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62430400 ticks/sec), init_cnt=624304 for 100Hz
[16526309415] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[16527865794] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[16529857509] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[16539633528] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[16576858320] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[16578639231] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[16579928805] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[16582158714] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[16584016086] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[16588787061] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[16591662714] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[16613589795] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[16615209765] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[16616183034] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[16617296289] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[16619005557] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[16621171083] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[16622445246] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[16649261046] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[16650610845] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[16651744857] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[16653021231] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[16654220847] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[16655618298] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[16656595428] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[16657676112] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[16668271290] [INFO] [kernel::root] [CPU0] Spawning Root service...
[16670082891] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[16673509413] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[16674924453] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[16681484886] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[16684198938] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[16685321565] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[16687448349] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[16689208701] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[16690697529] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[16712585373] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[16727834871] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[16729889517] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[16731168366] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[16775499114] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[16811385657] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[16814984307] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[16821724359] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[16824546915] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[16829020197] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[16833748932] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[16837437309] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[16838744043] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[16840736649] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[16853557644] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[16856982021] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[16862255124] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[16870066983] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[16874625768] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[16875536535] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[16898172060] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[16899664947] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[16912978071] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[16914428586] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[16963558821] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[16964917530] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[17610121089] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[18322019001] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[18363170166] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=500 journal=424 symbols=52 drops=0
[18434777724] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[20831819250] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=449 watches=0 history=963 journal=772 symbols=97 drops=0
[22208294262] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[22332474582] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[22334033172] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[22473597564] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[22600134711] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[22646340816] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[22647640026] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[22648743678] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[22664943477] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[22736196021] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[22766782797] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[22770674916] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[22876889673] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[22911095361] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[22912377312] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[22920747300] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[22976509347] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[23013885147] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[23019204021] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[23020888275] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[23162782071] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[23165307891] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[23325280341] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[23449490460] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[23463905751] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[23470268250] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[23472690714] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[23517796137] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[23608045626] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[23616152769] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[23617838739] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[23619118479] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[23620687002] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[23622047757] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[23623160583] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[23624245590] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[23625469923] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[23626526253] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[23627671914] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=33264
[23628665478] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=973464
[23629758636] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[23630988777] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[23632140675] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[23633327949] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[23634487635] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[23635613067] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[23636680023] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[23637745395] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[23638846374] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[23639897160] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[23640960618] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[23641968999] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[23643060705] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[23644233558] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[23645285961] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[23646300777] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[23647504518] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[23648592693] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[23649678360] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[23650748814] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[23651886423] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[23652987006] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[23654930046] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=168464
[23656041585] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[23657456394] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[23658714453] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[23659988913] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[23661264792] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[23662546314] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[23663881956] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[23665190769] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[23667804270] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[23670848289] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[23672199969] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23686780161] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[23710283025] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[23717354166] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[23733360255] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[23734693521] [CONTRACT] [kernel] [CPU0] Spawning init process...
[23738439351] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[23743238310] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[23744209830] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands
[23749663245] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode
petals>  tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[23756603541] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[23782970541] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[23788204341] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[23790073263] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[23791592319] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[23802786150] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[23810087004] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[23815216260] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[23817180024] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[23826004587] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[23831621847] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[23833527795] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[23840449413] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[23842353711] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[23844204021] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[23846057202] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[23852350764] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[23854422174] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[23856477414] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[23858948784] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[23861100714] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[23863584459] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[23871203565] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[23929590300] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[23940096873] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[23947606881] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[23954783259] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[23960044053] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[23965791003] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[23973416808] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[23980503624] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[23987964990] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[23994781239] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[24002210397] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[24009964473] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[24017655123] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[24024556215] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[24031549410] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[24038396745] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[24045782013] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[24053522328] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[24060238851] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[24067178256] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[24073881612] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[24081139467] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[24089249514] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[24097354347] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[24104915538] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[24113258730] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[24121445865] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[24130078599] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[24138003054] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[24145822008] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[24152346504] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[24158233869] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[24166910262] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[24172539567] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/telnetd
[24177830160] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[24185069172] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[24192395700] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[24199584816] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[24206553030] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[24216972681] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[24231365796] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[24241052253] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[24247700994] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[24281336376] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[24515301690] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[24528538815] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[24529822911] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24532349886] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24537336516] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[24538897680] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[24548997858] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[24557292408] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24559316133] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
[24560540235] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[24566063511] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[24566926263] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[24569176368] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24570200523] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[24574187847] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[24577960605] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[24589592940] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[24594991245] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[24598965468] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[24601271475] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[24607619553] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[24611329083] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[24614110620] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[24617554401] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 04381687056830ns
[24627858750] [INFO] [rtc_cmos] [CPU1] System clock anchored
[24647569947] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[24706274901] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[24719520078] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[24721137408] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[24723490671] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24732433704] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[24739004070] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[24751176483] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[24756418071] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[24761771166] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24764202672] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[24771410631] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[24780422370] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[24783306834] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[24784534533] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[24787318347] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24798447960] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[24802072185] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[24812138868] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[24817189188] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[24818855886] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24820900533] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[24825604056] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[24828717639] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[24833036250] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[24839057760] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[24842330964] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[24844892886] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[24846217407] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[24848416395] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[24866717601] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[24869302953] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[25522892175] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[25524654804] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[25527755121] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[25539024522] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[25543100781] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[25554812976] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[25560379548] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[25562097462] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[25564874412] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352688 RFLAGS_BEFORE=134 CR3_BEFORE=59662336 fs_base=0 gs_base=18446744071564586576
[25575273669] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[25577099658] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[25590086247] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[25591968435] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 0)
[25594003545] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 1)
[25596780099] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=1
[25601917077] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[25605958422] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[25609447512] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[25611889182] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[25613502519] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[25617309498] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[25618932900] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[25620443211] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[25621839243] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[25623263457] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[25625120268] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[25626524451] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[25627604739] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[25629587181] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[25758760038] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[25763718090] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[25767948327] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[25769980731] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[25771525527] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[25774506021] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[25776664419] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[25784350317] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[25787652759] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[25792498545] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[25794883323] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[25823752284] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[25831409043] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[25834715577] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[25835386863] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[25842463614] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[25850238414] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[25852380873] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[25860568437] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[25868444646] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[25877458827] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
[25887148353] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0105370
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[25889747367] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500256 RFLAGS_BEFORE=134 CR3_BEFORE=59940864 fs_base=0 gs_base=18446744071564586640
[25896311331] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[25897279518] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[25900890444] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[25908276207] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[25912844562] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[25923240618] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[25926717498] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[25928200287] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[25929445278] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[25933039011] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[25936767747] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[25939278453] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[25941904890] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[25943417478] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[25946257722] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[25948844163] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[25949854722] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[25951519473] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[25953813930] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[25956073242] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[25961689974] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[25963999776] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369433920 RFLAGS_BEFORE=130 CR3_BEFORE=59813888 fs_base=0 gs_base=18446744071564586608
[25972657623] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[26525901336] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1237 port=2 model='                                        ' rpc_port=5
[26530612680] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0105370
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[26533086888] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583040 RFLAGS_BEFORE=134 CR3_BEFORE=68349952 fs_base=0 gs_base=18446744071564586576
[26539758894] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[26542747671] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[26547216663] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[26567467377] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[26569954224] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[26571007518] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[26572444272] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[26574051603] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[26575372164] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[26576692395] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[26578377309] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[26582605236] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[26592369243] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[26595079104] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[26597981850] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[26604860865] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[26608129515] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[26609153604] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[26610822678] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[26618307837] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[26620976679] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[26629009836] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[26632938948] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[26635692996] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[26637808692] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[26638813344] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[26640631281] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[26662441608] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[26667694152] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[26722401156] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[27011620779] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[27012971337] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[27016044627] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[27018935262] [INFO] [ps2_mouse] [CPU3] ps2_mouse: using cooperative polling loop (2ms interval)
[27843764850] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[27847907109] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[27849318948] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[27851413557] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369718960 RFLAGS_BEFORE=130 CR3_BEFORE=68890624 fs_base=0 gs_base=18446744071564586640
[27857027781] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0105370
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[27859554030] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653424 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[27866448027] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[27869355954] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[27870514980] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[27873066870] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[27889334880] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using cooperative polling loop (2ms interval)
[27896412258] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[27899038134] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[27901120236] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[27903878145] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[27924412461] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[27936060603] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[27938662620] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[27942206853] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[27944572326] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[27947850084] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[27950026830] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[27953317755] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4c32000
[27955676562] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[27959257722] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[27962789250] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[27965231415] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[27981938292] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[27996901218] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[28013246778] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[28017455532] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[28020983595] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[28023657123] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[28026339297] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[28028876304] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[28033081824] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[28035395949] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[28037181282] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[28045604532] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=15, read=16)
[28055411538] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=17, read=18)
[28059703881] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[28062505845] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[28065701664] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[28067074464] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28069905930] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[28088028672] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[28124556240] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[28165173828] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[28176966939] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[28209445407] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[28216087746] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[28228260951] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[28233774030] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
[28235026710] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0105370
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28237824582] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[28239027234] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28240714392] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369919664 RFLAGS_BEFORE=130 CR3_BEFORE=80007168 fs_base=0 gs_base=18446744071564586576
[28244594730] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[28248093060] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[28251745863] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[28258199805] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[28260552342] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[28272990603] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[28284192321] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[28288167171] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[28296567717] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[28301448747] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[28303939257] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[28304882496] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28306634994] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[28313157081] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[28315859088] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[28323385827] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[28327773474] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b030
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28329788322] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
[28330946754] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370050736 RFLAGS_BEFORE=130 CR3_BEFORE=80941056 fs_base=0 gs_base=18446744071564586640
[28337655357] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[28338791877] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28340819595] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[28341839658] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[28343158404] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[28353404508] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[28359100308] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[28370292687] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[28374555495] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b030
[28376064651] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28378126128] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370116272 RFLAGS_BEFORE=130 CR3_BEFORE=81068032 fs_base=0 gs_base=18446744071564586576
[28383082629] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[28384251687] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28386770742] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[28401147654] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[28406356968] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[28418963166] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[28428083409] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[28432141056] [DEBUG] [kernel::sched::: Started.
[28443312183] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b030
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28447633005] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370181808 RFLAGS_BEFORE=130 CR3_BEFORE=81215488 fs_base=0 gs_base=18446744071564586608
[28458480039] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[28469429703] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[28493879271] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[28497693147] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[28498776075] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28503052446] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[28543090125] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[28550636730] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[28562394003] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[28568164449] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f1068
[28569328458] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28571268264] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370247472 RFLAGS_BEFORE=134 CR3_BEFORE=81371136 fs_base=0 gs_base=18446744071564586640
[28584124998] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[28585604850] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28588403877] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[28599570087] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[28634875698] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[28639752504] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[28645520277] [INFO] [fontd] [CPU3] FONTD: Service node created, req=19, resp=22
[28653414042] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[28658287944] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[28661201844] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[28662564216] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28665757989] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[28668880680] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[28670964366] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[28684873800] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[28688237589] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=227 pred=0 subj_lo=0
[28698853029] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[28710437745] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28715174763] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370332352 RFLAGS_BEFORE=134 CR3_BEFORE=81645568 fs_base=0 gs_base=18446744071564586576
[28728779112] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[28730829468] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[28738105803] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[28741268094] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[28742753985] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[28744334553] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[28746053457] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[28748807901] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[28751617455] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[28754113674] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[28764409410] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[28766017995] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[28767460656] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[28768799895] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[28770358947] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28771650435] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[28773305781] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[28774956738] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=233 pred=0 subj_lo=0
[28777292808] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[28782590826] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[28788519936] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
[28791088128] [INFO] [fontd] [CPU3] FONTD: Service ready
[28793796537] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1010
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28796229132] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370399712 RFLAGS_BEFORE=130 CR3_BEFORE=81854464 fs_base=0 gs_base=18446744071564586608
[28803558630] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[28805714388] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[28807753821] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[28836164676] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[28846628811] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[28861843395] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[28870319016] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=15, RX port=18
[28877485923] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[28879044909] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[28880867796] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28882593564] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=236 subj_lo=0
[28895414724] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[28897124355] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[28898940807] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28900766301] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[28910823216] [INFO] [netd] [CPU3] NETD: Driver TX port=15, RX port=18, link_up=true, mtu=1500
[28913599308] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[28914869082] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[28916154102] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[28917982071] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28919739585] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=244 subj_lo=0
[28922264811] [INFO] [netd] [CPU3] NETD: Created socket API port (write=23, read=24)
[28938297564] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1250 backend=VirtIO-GPU
[28941833250] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[28943292576] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28945930629] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[28966450095] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[28968005352] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[28969779993] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28971911529] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=245 subj_lo=0
[28982471661] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[29000160552] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=25, our_read=26)
[29002289382] [INFO] [anther] [CPU1] anther: Connected to network stack
[29047798791] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[29049123675] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[29050680879] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29052119184] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=248 subj_lo=0
[29061240615] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[29063473956] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[29067188568] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[29068489626] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[29069983305] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29071600008] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=250 pred=0 subj_lo=0
[29073449064] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[29111235681] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[29117752389] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[29144732793] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=27, resp=30
[29146939734] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[29167211436] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d6000 exec=false
[29190251970] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ec000 exec=false
[29202463422] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[29207657358] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[29209734213] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db5b8
[29211232215] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e2
[29212877991 [bloom::logging] [CPU3] bloom: logging initialized
[29220870657] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[29223004998] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[29233528665] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[29239823547] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db5b8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[29242472292] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370625408 RFLAGS_BEFORE=130 CR3_BEFORE=84361216 fs_base=0 gs_base=18446744071564586576
[29247054771] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[29252155482] [INFO] [echo] [CPU1] echo: starting up
[29255736444] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[29260913517] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[29264388483] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[29265548466] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[29269439397] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[29291656416] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[29294928630] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[29311481496] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[29315934582] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[29335477743] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[29336981553] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[29340939342] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[29343762921] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[29344979928] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[29347087869] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[29348589633] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[29354810232] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[29356615002] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[29357896128] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[29359542960] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[29368536978] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[29371298319] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[29376399624] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[29379263793] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[29382044439] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[29384609991] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[29385839505] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[29388454656] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[29397826557] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[29400034983] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[29401555986] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[29404038609] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[29405990592] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[29418968667] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[29423994666] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[29425577181] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f3580
[29426724162] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29429303409] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[29430606018] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370756480 RFLAGS_BEFORE=130 CR3_BEFORE=85135360 fs_base=0 gs_base=18446744071564586640
[29434525956] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[29437331319] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[29438649537] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[29442156282] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[29443754439] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[29450674374] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[29456294010] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[29460423894] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[29462021721] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/telnetd'
[29464783656] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/telnetd
[29466047358] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[29468501997] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[29480015070] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f3580
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29482601313] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370822016 RFLAGS_BEFORE=130 CR3_BEFORE=85282816 fs_base=0 gs_base=18446744071564586576
[29507039694] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[29521172967] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=220000 exec=false
[29525759703] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[29531009145] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[29532485796] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=228000 exec=false
[29544450507] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 32 (user task/process) assigned to CPU 2
[29548898115] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=32)
[29551178481] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[29565017262] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[29566271955] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[29568268488] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[29574547101] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f3580
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29577519840] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=32 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370887808 RFLAGS_BEFORE=130 CR3_BEFORE=85385216 fs_base=0 gs_base=18446744071564586608
[29587545009] [INFO] [telnetd] [CPU2] telnetd: starting on port 2323
[29602442958] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[29606516841] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[29608508952] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[29612146641] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[29632414449] [INFO] [telnetd::net_client] [CPU2] telnetd: connected to socket API (netd=23, write=33, read=34)
[29643392361] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[29646875115] [INFO] [bloom] [CPU3] bloom: creating surface...
[29648705163] [INFO] [bloom] [CPU3] bloom: surface created!
[29650203396] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[29655731292] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[29663880378] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1263
[29665706268] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[29686533261] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[29690872464] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=33)
[29693109831] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
T:1580 [29699147049] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[29713290354] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[29727646311] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[29741345337] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
T:1AE0 T:1950 T:03C0 [29762677923] [INFO] [bloom] [CPU3] [bloom] dynamically subscribed to input topic 0 on svc.Input 1241 via port 36
[29765245158] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 36 (legacy was 12)
[29793400395] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[29798703825] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[29801639109] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[29820869826] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1263
[29837181396] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[29848132908] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 37 (user thread) assigned to CPU 3
[29850625068] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=37 (priority=2)
[29853632424] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[29856081222] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[29862976605] [INFO] [netd] [CPU3] NETD: DHCP configured �� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[29866531299] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[29872279800] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[29876804100] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[29879539239] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[29881157130] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[29882560587] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[29884527453] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 2323 with backlog 64
[29886988857] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 2323, handle=3
[29889980010] [INFO] [telnetd] [CPU2] telnetd: listening on guest port 2323 (handle=3)
[29897667426] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
T:2530 [29902524432] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[29904457539] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[29918611272] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[29920640739] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[29922057627] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[29949136404] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[29952972621] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[29958019080] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[29963945583] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=270
[29965875225] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[29967097677] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[29968119786] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[29969763219] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[29971448133] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29973281316] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=270 subj_lo=0
[30001618713] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1274)
[30003403056] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[30022111416] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=250
[30024318456] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[30025771809] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[30027415935] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[30029238492] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[30031191069] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=250 pred=0 subj_lo=0
[30055409010] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1277)
[30057126330] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[30081800331] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=272
[30083625462] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[30084957837] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[30086507781] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[30088121976] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[30089923776] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=272 subj_lo=0
[30115307310] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1280)
[30117618630] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[30154443429] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=634 watches=13 history=1024 journal=1024 symbols=277 drops=0
[30285504711] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[30288314397] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[30292327692] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30317917410] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30327173514] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[30336234192] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=70
[30338652432] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[30342648666] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30345026943] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[30378421887] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30393200574] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=74
[30396555057] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=17
[30401090610] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[30435833538] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[30443054631] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[30447001530] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[30452917968] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[30503310090] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=70
[30506038134] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[30510283947] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30530734575] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30556987032] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=3 socket state=Established
[30559983894] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:53896 on listener 3
[30603173667] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 60 bytes - TCP ACK
[30604740408] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[30607181055] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[30611038821] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30614100858] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 60 bytes
[30635293953] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30639427599] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[30800216040] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[30813939915] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: TCP_CLOSE handle=4 (initiating close)
[30819016371] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 118 bytes - TCP FIN
[30821697885] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[30824409792] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[30828859941] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30833656953] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[30858880866] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[31073597874] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=77c80b059e863710)
[32519502114] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[33006897693] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=672 watches=13 history=1024 journal=1024 symbols=340 drops=0
[33849027729] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[33852651624] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[33855587535] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 124 bytes - TCP FIN
[33888119562] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[33891417780] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[33902646525] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[33908539236] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[34129466052] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[34131101796] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[34132936233] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34134652101] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=295 pred=0 subj_lo=0
[34345241799] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[34424504829] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[34477811907] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[34479587901] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[34481373498] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34483352541] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=298 pred=0 subj_lo=0
[34539973017] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 2092.799ms (rebuilds=0 pending=true)
[34572506958] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[34611977598] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[34685578587] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[34758128493] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[34759840566] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[34761741168] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34763711499] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=342 pred=0 subj_lo=0
[34800884943] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[34878263376] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[34969936188] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12014000
[34973195796] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[34977703563] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[35080556346] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[35096958864] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[35103830586] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[35106969018] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[35110918392] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[35118045732] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[35149349466] [INFO] [bloom] [CPU3] [bloom] acquire_buffer took 229.174ms
[35151557001] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[35163817788] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[35165696940] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[35167886424] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[35174582454] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[35176502130] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[35178372702] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35185870896] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=344 pred=0 subj_lo=0
[35265258006] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[35278514700] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x12015000
[35295636717] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[35316630987] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[35322807168] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[35350555878] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[35354848881] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[35358086280] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[35365318395] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 2559.508ms
[35368407063] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[35372002446] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[35377562220] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[35382864528] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=1 (16384b)
[35396898801] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[35552561352] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=3 socket state=Established
[35557640382] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:53902 on listener 3
[35566648458] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 124 bytes - TCP FIN
[35579248980] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 60 bytes - TCP ACK
[35604179259] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 124 bytes
[36226760229] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=730 watches=17 history=1024 journal=1024 symbols=361 drops=0
[36587217975] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[36589443330] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[36592387293] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[36784446567] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[36786729507] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[36789725346] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[37378134024] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=85
[37380920874] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 75 byte frame (79 encoded) to netd rx_port=17
[37383908001] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (79 bytes sent)
[37492251027] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[37495380549] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[37497893400] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 75 bytes
[37503716415] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 60 bytes - TCP ACK
[37512412806] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: GC removed explicitly closed TCP socket
[37552531698] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[37999864881] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[38127133473] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 118 bytes - TCP ACK
[38164109280] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[38717684577] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=761 watches=17 history=1024 journal=1024 symbols=363 drops=0
[38731843029] [INFO] [phloem::executor] [CPU2] phloem: entering discover_nodes
[38734409175] [INFO] [phloem::executor] [CPU2] phloem: starting BFS discovery from roots
[38741271822] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 60 bytes
[38762113929] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[38765248071] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[38769352314] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[38775641850] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[38909727318] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 411.729ms (rebuilds=0 pending=true)
[38913524067] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=1
[39078613068] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[39230662515] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[39413993949] [INFO] [bloom] [CPU3] [bloom] acquire_buffer took 161.195ms
[39432082932] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 2096.118ms
[39434583375] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[39436898457] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[39560921697] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 139 bytes - TCP ACK
[39573104274] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 139 bytes
[39593355813] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[39597264234] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[39600838497] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[39610017216] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[39618601209] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[39630417420] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[39715855575] [INFO] [phloem::executor] [CPU2] phloem: BFS seeded with 469 nodes
[40029402237] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[40073593593] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=671285c0c9ba41ed)
[40524918060] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[40572254580] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 33264 bytes, hash=91427bae3069f281)
[40623370062] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 496.791ms (rebuilds=0 pending=true)
[40643444589] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 605.854ms
[41172073833] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[41200579959] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 973464 bytes, hash=41cd8d0df23aabcd)
[41424226338] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 336.524ms (rebuilds=0 pending=true)
[41440598925] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 398.179ms
[41655787998] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[41705436597] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=4e1c9cf03988a560)
[42288067965] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[42324951768] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[42596182695] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 493.086ms (rebuilds=0 pending=true)
[42621246162] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 590.613ms
[42823353936] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[42864426396] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[43409735127] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[43433621583] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[43436600163] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[43438180632] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[43440048168] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[43441386153] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[43443578442] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[43474691667] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[43476341799] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[43478000544] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[43480092876] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=366 pred=0 subj_lo=0
[43499856807] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[43829537763] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 536.623ms (rebuilds=0 pending=true)
[43851384357] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 614.789ms
[44080841508] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[44097194757] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[44560640421] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[44594449185] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[44897197026] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 445.064ms (rebuilds=0 pending=true)
[44921522778] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 534.722ms
[45130725948] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[45157402851] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[45807133218] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[45823847718] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[46059738846] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=946 watches=19 history=1024 journal=1024 symbols=377 drops=0
[46334895915] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 604.122ms (rebuilds=0 pending=true)
[46358208930] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 718.937ms
[46619461350] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=0e57f834b3afbcb1)
[46715802969] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[47330297784] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[47416871667] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[47706908469] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 591.090ms (rebuilds=0 pending=true)
[47736181614] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 688.685ms
[48156508719] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=c1e9a7b90a789e5c)
[48166217154] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[49069843614] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[49098804645] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[49443100311] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 745.636ms (rebuilds=0 pending=true)
[49468690722] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 866.324ms
[49966638480] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[49984553916] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[50037883170] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2432 ops=1 watches=19
[50755715505] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[50762424570] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[50856395733] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 596.863ms (rebuilds=0 pending=true)
[50871491814] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 701.621ms
[51487283430] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[51575007231] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[53197110747] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 1064.050ms (rebuilds=0 pending=true)
[53246047404] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 1187.200ms
[53897080974] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[54066839508] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[55592347239] [INFO] [phloem::executor] [CPU2] phloem: discovered 490 nodes
[55678203966] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 66 bytes - TCP ACK
[55966227636] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 66 bytes
[56462996898] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 1213.168ms (rebuilds=0 pending=true)
[56510310318] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 1632.374ms
[56610629922] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=1117 watches=19 history=1024 journal=1024 symbols=377 drops=0
[57396428661] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 337.694ms (rebuilds=0 pending=true)
[57438499173] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 463.651ms
[58205235132] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[58211307330] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[58217075103] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[58257121989] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[58308252948] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 475 bytes - TCP ACK
[58752314577] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 537.377ms (rebuilds=0 pending=true)
[58766481972] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 664.308ms
[59903767836] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 509.982ms (rebuilds=0 pending=true)
[59921607174] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 577.594ms
[60014644536] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 475 bytes
[60713285655] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=1197 watches=19 history=1024 journal=1024 symbols=377 drops=0
[61389756615] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 631.450ms (rebuilds=0 pending=true)
[61396186500] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 737.313ms
[62017599039] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 1179 bytes - TCP ACK
[62810136378] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 591.888ms (rebuilds=0 pending=true)
[62834974620] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 719.143ms
[63888055671] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 465.598ms (rebuilds=0 pending=true)
[63899318076] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 531.744ms
[64402216362] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1269 watches=19 history=1024 journal=1024 symbols=377 drops=0
[64685523771] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[64690326855] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[64695394038] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[64700685852] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 1179 bytes
[64711331553] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[65242177770] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 589.096ms (rebuilds=0 pending=true)
[65269535166] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 685.763ms
[65509030785] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[65516683287] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[65522218443] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[65539811931] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[65544784701] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 694 bytes - TCP ACK
[66134833809] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 694 bytes
[66380351895] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[66557638389] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 575.992ms (rebuilds=0 pending=true)
[66568082031] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 649.367ms
[68072022876] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 684.108ms (rebuilds=0 pending=true)
[68103703041] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 767.308ms
[68503911861] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=70
[68509204335] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[68516088201] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[68526595566] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[68531313213] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 574 bytes - TCP ACK
[68544428337] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1348 watches=19 history=1024 journal=1024 symbols=377 drops=0
[68606667525] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 574 bytes
[69428797647] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=70
[69437092461] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[69442548945] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[69443930952] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[69448712850] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 210 bytes - TCP ACK
[69585463926] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 633.272ms (rebuilds=0 pending=true)
[69594986373] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 746.082ms
[70477977405] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 210 bytes
[70908105543] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=70
[70911364953] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[70915516155] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[70930086183] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[70934569893] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 306 bytes - TCP ACK
[71038247841] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 633.583ms (rebuilds=0 pending=true)
[71062826076] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 733.933ms
[72228076074] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 306 bytes
[72237302940] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[72322037337] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1418 watches=19 history=1024 journal=1024 symbols=377 drops=0
[72324218637] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 513.420ms (rebuilds=0 pending=true)
[72355257183] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 646.132ms
[73543771521] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 496.663ms (rebuilds=0 pending=true)
[73572095157] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 608.416ms
[74656081995] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 466.921ms (rebuilds=0 pending=true)
[74670658821] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 549.337ms
[75216004611] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=70
[75236246910] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[75241813383] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[75261584112] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[75282969828] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 866 bytes - TCP ACK
[75657578469] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 404.279ms (rebuilds=0 pending=true)
[75675712728] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 866 bytes
[75678374310] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 503.902ms
[76139860203] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=70
[76145724699] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[76151224578] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[76160934003] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[76163550936] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 210 bytes - TCP ACK
[76446739842] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=1498 watches=19 history=1024 journal=1024 symbols=377 drops=0
[76701388797] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 210 bytes
[76863524529] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 476.365ms (rebuilds=0 pending=true)
[76878436140] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 599.958ms
[78017198952] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 530.910ms (rebuilds=0 pending=true)
[78044729598] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 583.177ms
[78671651025] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=70
[78677064543] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[78682902540] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[78693932361] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[78699467154] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 525 bytes - TCP ACK
[78950502180] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 395.410ms (rebuilds=0 pending=true)
[78966927633] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 461.061ms
[79263919317] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 525 bytes
[79861428669] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 357.590ms (rebuilds=0 pending=true)
[79905569568] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 469.314ms
[79953437520] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=70
[79958247963] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[79966254522] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[79986858765] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[79990132860] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - TCP ACK
[80091578985] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=1568 watches=19 history=1024 journal=1024 symbols=377 drops=0
[80313624204] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[81095754399] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 504.685ms (rebuilds=0 pending=true)
[81105412740] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 599.933ms
[81174060330] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=70
[81179010561] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[81185358408] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[81206968623] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[81211010034] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 292 bytes - TCP ACK
[81300207417] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 292 bytes
[82394777124] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 562.795ms (rebuilds=0 pending=true)
[82410836640] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 652.599ms
[83077940847] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=25 len=70
[83081871840] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[83089174344] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[83111046414] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[83115550155] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 404 bytes - TCP ACK
[83474694798] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 404 bytes
[83770021302] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=26 len=70
[83773879167] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[83780443824] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[83790116718] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[83808407562] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 162 bytes - TCP ACK
[83826857961] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 588.598ms (rebuilds=0 pending=true)
[83849743230] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 719.613ms
[84031840497] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 162 bytes
[84492940950] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=27 len=70
[84500233686] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[84518069691] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[84524851554] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[84542839425] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 168 bytes - TCP ACK
[84635378190] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=16000 nodes=1648 watches=19 history=1024 journal=1024 symbols=377 drops=0
[85007107086] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 168 bytes
[85453832541] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 744.745ms (rebuilds=0 pending=true)
[85468481472] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 809.286ms
[85941186738] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX
```
</details>
