# ❌ Scenario: Morning Health Check

> Last run: 2026-04-05 18:40:52

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 5361ms | - - - |
| 2 | When I wait for the system to reach ready state | ✅ | 0ms | - - - |
| 3 | Then I should see a message in the serial output that says "SPROUT: Entering supervisor loop." within 5s | ❌ | 6081ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11867112180] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11872850220] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11876776923] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11878832592] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11880107844] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11880776523] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11881462494] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11882071377] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11882687157] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11883349434] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11883975873] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11884610364] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11885341776] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11886042762] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11886781269] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11887423416] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11888079060] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11888707017] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11889353817] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11889992103] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11890597059] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11891206206] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11891820600] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11892437304] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11893116345] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11893745721] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11894369058] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11895045492] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11895662922] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11896354437] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11896995528] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11897641008] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11898310908] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11898966255] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11899604673] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11900336844] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11901075747] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11901818874] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11902547217] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11903319285] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11904060531] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11904860220] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11906322285] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11907898332] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11908710792] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11909287005] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11909988750] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11910596544] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11911316373] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11911978551] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11912526285] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11913092697] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11913630102] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11914196712] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11914769262] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11915361183] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11915932479] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11916551856] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11917128267] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11917723026] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11918298414] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11918894196] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11919513540] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11920113018] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11920688010] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11921277027] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11921846211] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11922434502] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11923036521] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11923628772] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11924200464] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11924790306] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11925365826] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11925983487] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11926559865] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11927155119] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11927729352] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11928323253] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11928892833] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11929504587] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11930075685] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11930666550] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11931237417] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11931827292] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11932394628] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11933015424] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11933627310] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11934222366] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11934792012] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11935381392] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11935971102] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11936562231] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11937187845] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11937789138] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11938394523] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11938994232] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11939596482] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11940193386] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11940915459] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11941517808] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11942095803] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11942725971] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11943338847] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11943939678] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11944518894] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11945116128] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11945716893] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11946578127] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12197664468] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[12208820349] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12213893604] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12215180241] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12216089094] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12220312203] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12222075096] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12223222407] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12223940355] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12224641869] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12225344109] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12226365492] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12227373708] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12228098982] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12228811320] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12229539960] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12230250021] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12231388620] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12232451088] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12233199462] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12234792438] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12235852266] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12236908530] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12238602090] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12240393858] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12241225260] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12241798305] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12242606277] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12628856724] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12629881506] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12633220545] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12634153818] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12634963770] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12636537276] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12649479348] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12650840895] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12651635007] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12653495514] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12654113010] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12656714301] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12664888302] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12666653637] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12680121069] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12680722461] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12697381389] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12697962387] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12700025745] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12701260176] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12702335712] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12704641653] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12705511401] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12740359764] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62273500 ticks/sec), init_cnt=622735 for 100Hz
[12741790380] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12742648512] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12743838657] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12749557260] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12780133806] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12781112124] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12783231582] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12785547390] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12787622364] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12792478446] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12794966580] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12811701474] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12812358669] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12813388962] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12814961016] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12816769020] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12818618670] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12819924150] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12845525154] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12847004742] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12847953492] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12849176670] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12850552539] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12851523663] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12853245141] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12854387172] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12860598300] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12861547380] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12863467617] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12864361851] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12871370325] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12874173774] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12875559147] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12878017086] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12879784335] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12881548779] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12903119328] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12907191726] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12908431503] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12909225780] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12931801905] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12961570314] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12964754814] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12970288122] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12973120347] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12976991049] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12983094894] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12983952498] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12985155513] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12995169429] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12998109663] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13002237963] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13006499649] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13010364048] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13014554784] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13015623489] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13033630236] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13034592087] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13045031868] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13046096118] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13089323511] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13090144749] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13489677696] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13974200889] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[14005410507] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=499 journal=424 symbols=51 drops=0
[14038865643] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[15390138027] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=450 watches=0 history=966 journal=774 symbols=98 drops=0
[16113456546] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[16211353521] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[16212411798] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[16321252002] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[16380407340] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[16410547098] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[16411425690] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[16412135223] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[16415698431] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[16436414148] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[16452657408] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[16455161217] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[16506950493] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[16523028522] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[16523848374] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[16527842826] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[16552742316] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[16571888817] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[16579942731] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[16581068823] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[16646884980] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[16648544121] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[16746857226] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[16813163301] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[16825431909] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[16830819258] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[16833037089] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[16891845828] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=182 drops=0
[16909759812] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[16914439278] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[16915658661] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[16916563026] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[16917493395] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[16918370205] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[16919252592] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[16919973543] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[16920612060] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[16921257144] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[16921932060] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[16922583612] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[16923297633] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[16923988521] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[16924710759] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[16925471871] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[16926176091] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[16926868431] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[16927530015] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[16928238327] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[16928906676] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[16929584298] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[16930235421] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[16930892418] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[16931548227] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[16932287691] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[16932993759] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[16933652307] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[16934366526] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[16935017022] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[16935679563] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[16936419159] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[16937104041] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[16937769189] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[16938439914] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[16939111761] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[16939882476] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[16940677479] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[16941551484] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[16942328139] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[16943153502] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[16943936163] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[16944725358] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[16946247318] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[16948251837] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[16949134026] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16957451874] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[16972661805] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[16977393444] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[16986846426] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[16987571568] [CONTRACT] [kernel] [CPU0] Spawning init process...
[16989593973] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[16992132399] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[16992824343] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [16999159287] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[17001216672] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[17027348679] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[17035416618] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[17037754041] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[17039445126] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[17051718420] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[17059201500] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[17064607923] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[17066584128] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[17072537163] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[17077422153] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[17079881445] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[17086628394] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[17088523452] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[17090339442] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[17092086924] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[17096346465] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[17097670260] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[17099059890] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[17100773712] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[17102224755] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[17103723054] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[17108284017] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[17153945820] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[17162047386] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[17168489052] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[17174918211] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[17178553986] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[17183122044] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[17188802532] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[17194643070] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[17200765626] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[17206855842] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[17213606784] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[17220132435] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[17226823350] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[17233961910] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[17240738757] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[17246852832] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[17253101382] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[17259078837] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[17266603365] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[17273852376] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[17281863918] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[17288638191] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[17295462228] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[17301606300] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[17308252731] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[17314584144] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[17320495368] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[17326519980] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[17332682565] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[17338755126] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[17342932134] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[17347357830] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[17353764483] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[17360008380] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[17366348538] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[17372818716] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[17379557943] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[17385876750] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[17392882221] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[17399923134] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[17408940153] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[17414254440] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[17438446740] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[17617459002] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[17624615844] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[17625530934] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17627425233] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17632385760] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17633878812] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17642334171] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[17647501344] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[17648916450] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17650884372] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[17658064644] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[17659337058] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[17660436222] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17663130738] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17668229007] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[17670537060] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17678528637] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[17682379440] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[17683790718] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[17685880443] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[17690412861] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[17694733881] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[17697556932] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[17701108062] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17708369712] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:43:32 = 1775439812 unix_secs
[17711207745] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775439812, mono_ns=8855242836, offset=1775439803144757164ns
[17714159265] [INFO] [rtc_cmos] [CPU1] System clock anchored
[17730765921] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[17780428314] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[17788565355] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[17789808234] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17792491563] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17801617086] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[17805283419] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[17817113391] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[17822387055] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[17826595644] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17828851689] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[17836096773] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[17842919655] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[17845913646] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[17847040431] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17849581035] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17860514199] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17863935870] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17875049775] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[17880299382] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[17881538235] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17883944925] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[17885197407] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[17892582807] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[17893868619] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[17898544851] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[17901673911] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[17904014337] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[17905813563] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[17908280841] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[17932432320] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[17935255008] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[18477038910] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18482324883] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[18483612015] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18486288183] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18497444262] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18501096372] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18511959939] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[18515941554] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[18517517667] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[18519876870] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352560 RFLAGS_BEFORE=130 CR3_BEFORE=59662336 fs_base=0 gs_base=18446744071564586576
[18527271576] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[18530315628] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[18531902103] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[18534347073] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[18541180416] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[18543285981] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[18545174241] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[18547967064] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[18549894528] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[18551904954] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[18555751236] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[18557399256] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[18559190397] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[18560586000] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[18561830100] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[18563101194] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[18564547485] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[18593948769] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[18596704137] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[18602958330] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[18611957463] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[18613583076] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[18625151358] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[18629526300] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[18633135972] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[18635761155] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[18636898269] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[18638456793] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18641062671] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18653685996] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18656611908] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18669154251] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[18673283013] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[18676267236] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[18677478798] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18679980462] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18687532512] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18689779185] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18700707498] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[18704877246] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[18706241730] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[18708352377] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369499968 RFLAGS_BEFORE=130 CR3_BEFORE=68235264 fs_base=0 gs_base=18446744071564586640
[18715720122] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[18716856609] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18718859643] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[18720385002] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18722267619] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[18727870656] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[18729613782] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18730909164] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[18733385781] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[18734564574] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18737011227] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[18739343040] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[18741632943] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[18744468072] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[18746857272] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[18748434309] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[18751408533] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[18754148919] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[18760904481] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[18763110498] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[18767258895] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1236 port=2 model='                                        ' rpc_port=5
[18799656810] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[18800710863] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[18801663771] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[18802583679] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[18803585757] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[18804543384] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[18805583313] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[18806696535] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[18812756127] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[18815046030] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369433728 RFLAGS_BEFORE=130 CR3_BEFORE=59830272 fs_base=0 gs_base=18446744071564586608
[18823059288] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[19106272218] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
[19107789426] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[19109942511] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369582752 RFLAGS_BEFORE=134 CR3_BEFORE=68349952 fs_base=0 gs_base=18446744071564586576
[19114569210] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[19117568184] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[19118657085] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[19122225672] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19123998036] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[19125665130] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[19126826301] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19128332190] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[19129408518] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19136382969] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[19138971621] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[19147105164] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[19148414802] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[19152669426] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[19154698629] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[19156281276] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[19157151354] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19158950712] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19179209016] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19183667052] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[19263681030] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[20085833262] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[20088533817] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[20090037330] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20091707229] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369718848 RFLAGS_BEFORE=130 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[20097270600] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[20099924493] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653312 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[20104524528] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[20106587391] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[20107806939] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[20109543927] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[20114695194] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[20116448121] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[20119628100] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[20121042183] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[20122151874] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[20123611464] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[20137757145] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[20144942466] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[20146556430] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[20148266853] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[20149620117] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[20151696345] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[20153053008] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[20154913515] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4c32000
[20156327499] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[20158574073] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[20160749829] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[20162247402] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[20173066419] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[20182506168] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[20192715741] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[20195158269] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[20198447115] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[20199338808] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[20200645146] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[20201985408] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[20203094901] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[20204497434] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[20205638772] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[20206869045] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[20208102321] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[20210369751] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[20212349784] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[20220338886] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=15, read=16)
[20228298585] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=17, read=18)
[20232512883] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[20234965872] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[20237485752] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[20238587391] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20241102486] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20340022395] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[20365476978] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[20373630321] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[20376065754] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[20377537092] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20379550521] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915456 RFLAGS_BEFORE=130 CR3_BEFORE=80007168 fs_base=0 gs_base=18446744071564586576
[20384759406] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[20385872034] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20388324858] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[20389537872] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20392214436] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[20398432725] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20400524463] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20419626051] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20423202162] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[20430910632] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[20433733551] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[20435896536] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[20436825618] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20438638902] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20445063672] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20447754789] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20455214670] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[20457880839] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[20459713296] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20461546050] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370046816 RFLAGS_BEFORE=134 CR3_BEFORE=80941056 fs_base=0 gs_base=18446744071564586640
[20466486084] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[20467526145] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20469804597] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[20471076120] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20472206997] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[20481996150] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20483974236] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20489360727] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[20500111401] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[20503985601] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[20505113970] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20506933920] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113216 RFLAGS_BEFORE=134 CR3_BEFORE=81068032 fs_base=0 gs_base=18446744071564586576
[20510373675] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[20511535209] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20513906424] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20517514941] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20527905387] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[20532971019] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[20542586691] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20544326847] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20545142079] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20548412049] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[20553185598] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[20556785865] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[20558088243] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20560859220] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20579016315] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20600673390] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20601979365] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20608714104] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[20618416698] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20620344261] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20622495894] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[20627729925] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[20629208919] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20631255579] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370245792 RFLAGS_BEFORE=134 CR3_BEFORE=81371136 fs_base=0 gs_base=18446744071564586640
[20635825188] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[20637167925] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20639260422] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[20640751725] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20660867007] [INFO] [fontd] [CPU3] FONTD: Service node created, req=19, resp=22
[20666927226] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20668931184] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[20670235938] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20672827395] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20674860822] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20675710407] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[20684251335] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[20687072505] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[20688369636] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20691139887] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370331840 RFLAGS_BEFORE=134 CR3_BEFORE=81645568 fs_base=0 gs_base=18446744071564586576
[20695882680] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[20697023688] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20699376588] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20700637089] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[20702324709] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[20706558543] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20708136438] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20709728127] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20711347668] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20713149501] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20714924076] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=229 pred=0 subj_lo=0
[20717005980] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20718857247] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[20725089891] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20754135006] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[20770382787] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[20772127266] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20773852770] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20775699285] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20777551575] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20778749838] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20780097921] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[20782212759] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20783642220] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[20786078643] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[20790387882] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[20796959205] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[20798349693] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20800149315] [INFO] [fontd] [CPU3] FONTD: Service ready
[20801703120] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20803934415] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369980992 RFLAGS_BEFORE=130 CR3_BEFORE=80678912 fs_base=0 gs_base=18446744071564586608
[20808807954] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20810144982] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20811365355] [INFO] [nectar] [CPU2] NECTAR: Started.
[20812127622] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20813630475] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[20815883319] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010cac8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20818040529] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370180000 RFLAGS_BEFORE=134 CR3_BEFORE=81215488 fs_base=0 gs_base=18446744071564586608
[20824386363] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[20828860173] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20831025204] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370398480 RFLAGS_BEFORE=130 CR3_BEFORE=81854464 fs_base=0 gs_base=18446744071564586608
[20837903130] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[20840285301] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[20843329023] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[20847957372] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20849612124] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20851299579] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20858905617] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[20875291470] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20876472771] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20877773235] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20878984335] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=236 subj_lo=0
[20892355440] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[20894383356] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20895633990] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20896902477] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20898221883] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[20904007608] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20917426530] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1252 backend=VirtIO-GPU
[20919905391] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20921108406] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20922421047] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20923898919] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[20924864301] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=246 subj_lo=0
[20926326399] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20929132125] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20938589463] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20939898177] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20941270779] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20942783103] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=247 pred=0 subj_lo=0
[20947867149] [INFO] [netd] [CPU3] NETD: Driver TX port=15, RX port=18, link_up=true, mtu=1500
[20951515200] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[20958243768] [INFO] [netd] [CPU3] NETD: Created socket API port (write=23, read=24)
[20989211463] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[20993579013] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[21006565305] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=29, our_read=30)
[21008480163] [INFO] [anther] [CPU1] anther: Connected to network stack
[21047211405] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[21048862494] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[21054939807] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=25, resp=28
[21056280333] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[21057221889] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[21074910351] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[21091550106] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[21099470832] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[21102134394] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[21103103571] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[21104397633] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f0fe8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e4
[21106792674] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370556272 RFLAGS_BEFORE=130 CR3_BEFORE=82505728 fs_base=0 gs_base=18446744071564586640
[21111439338] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[21112283379] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21113958492] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21114863649] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[21119702439] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[21121282875] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21129692727] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[21132214026] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f0fe8
[21133297944] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[21134744433] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370621808 RFLAGS_BEFORE=130 CR3_BEFORE=84086784 fs_base=0 gs_base=18446744071564586576
[21138146172] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[21139775481] [INFO] [echo] [CPU1] echo: starting up
[21141564180] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=15, RX port=18
[21142708026] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[21145056471] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[21155176812] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[21166704999] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[21169444230] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[21177510618] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[21183324162] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[21185277663] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[21201421164] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[21206655921] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[21209525601] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[21218570901] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[21220003233] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[21221998710] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[21222897399] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21224640063] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21229713846] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21230678370] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[21232029687] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[21233002164] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[21239773500] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[21241015950] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[21243749505] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[21245545926] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[21247272486] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[21249105603] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[21249989343] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21251421774] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[21252683661] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21260444667] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[21262135719] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21266123835] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[21274389081] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[21276752046] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[21278702115] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[21280665054] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[21281635683] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21283767549] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f2a98
[21290602740] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21293956530] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[21295301973] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[21301249695] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370752880 RFLAGS_BEFORE=130 CR3_BEFORE=85118976 fs_base=0 gs_base=18446744071564586640
[21307643049] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[21310029411] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[21311367297] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[21315217803] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[21326723451] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f2a98
[21328137171] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21330275241] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370818416 RFLAGS_BEFORE=130 CR3_BEFORE=85266432 fs_base=0 gs_base=18446744071564586576
[21337455084] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[21340053339] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[21341598135] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[21343759437] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[21348097419] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[21352859682] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[21355211757] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[21360347151] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[21400986849] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[21402416211] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[21425601351] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[21429798060] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[21431381499] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[21447327594] [INFO] [netd] [CPU3] NETD: DHCP configured �� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[21452322243] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[21458791761] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[21464378001] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[21468226527] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[21471211212] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[21477976608] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[21489113514] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[21531199536] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[21550051215] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[21574600443] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[21579418806] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[21585938220] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[21589466118] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[21592760277] [INFO] [bloom] [CPU3] bloom: creating surface...
[21594467862] [INFO] [bloom] [CPU3] bloom: surface created!
[21595836438] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[21609006210] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[21628409649] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1266
[21630087204] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[21649847934] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[21654331314] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[21656464401] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[21658892376] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [21670044231] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[21686611452] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[21699097299] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[21711642447] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[21722629599] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1266
T:07D0 T:0640 T:F0B0 [21747184008] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 5)
[21776662413] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[21778638750] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[21793220295] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=287
[21794689125] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[21795772713] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[21796918572] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21798293616] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21799547847] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=287 subj_lo=0
[21802391556] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[21812570142] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[21814929609] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[21818121336] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[21821593266] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
T:1220 [21835046310] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[21838897014] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[21844782927] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1275)
[21846129459] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[21860334177] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[21864211809] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=247
[21866383572] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[21868145871] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[21869614866] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21871413762] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21873208830] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=247 pred=0 subj_lo=0
[21895784427] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1283)
[21897502341] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[21920291151] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=290
[21922873962] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[21924538812] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[21925853301] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21927589134] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21929293584] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=290 subj_lo=0
[21945759297] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1284)
[21947684121] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[22295833989] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[22512570366] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[22562753532] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[22569521799] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=664 watches=13 history=1024 journal=1024 symbols=315 drops=0
[22588975068] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[22776040683] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[22965189555] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[23204728272] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[23604977055] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[23874171849] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[24070856634] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[24102485649] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=693 watches=13 history=1024 journal=1024 symbols=339 drops=0
[24160223571] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[24430444863] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[24789446715] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[24906814779] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[24914564004] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[24916480083] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[24917929938] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[24919318611] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[24921613332] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[24940961628] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[24942164709] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[24943473060] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[24945050361] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=350 pred=0 subj_lo=0
[25117778928] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[25220876604] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[25223102751] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[25385530863] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[25386758166] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25388083644] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25389608739] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=313 pred=0 subj_lo=0
[25574499885] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[25639349736] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[25640615484] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25641929115] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25643251524] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=314 pred=0 subj_lo=0
[25656154986] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[25695876195] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[25806680856] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[25898097720] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[25966364886] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[25984099086] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[25985967381] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[25991943087] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[26005856481] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x121f4000
[26007210933] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[26008411473] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[26133164607] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[26148117996] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[26154968631] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[26158324203] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[26161723665] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[26168507310] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[26181940224] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26183834523] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26185755816] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26187769608] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=337 pred=0 subj_lo=0
[26200714683] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[26203003332] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[26205555057] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[26223946221] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x12201000
[26225845635] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[26268416361] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[26564020263] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=0e57f834b3afbcb1)
[26750789901] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=779 watches=18 history=1024 journal=1024 symbols=363 drops=0
[26839515879] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[27157487940] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=c1e9a7b90a789e5c)
[27420837312] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[27704119938] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[28028003025] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[28322096748] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[28611815991] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[28677140877] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[28773832659] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[28775602614] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[28777440747] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28779414576] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=351 pred=0 subj_lo=0
[28803649941] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[28987710048] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[29558926584] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[29938018110] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[30082539267] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=850 watches=19 history=1024 journal=1024 symbols=365 drops=0
[30290302218] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[30776497389] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[31125937689] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[31470221904] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[31915894131] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[32238912552] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=890 watches=19 history=1024 journal=1024 symbols=366 drops=0
[32962768443] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2431 ops=1 watches=19
[33597780348] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[34165298178] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=909 watches=19 history=1024 journal=1024 symbols=366 drops=0
[34378999974] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[35602546188] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[36175705896] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=938 watches=19 history=1024 journal=1024 symbols=367 drops=0
[36300407121] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[36702724212] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[36889815831] [INFO]
```
</details>
