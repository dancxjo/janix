# ✅ Scenario: Server starts up

> Last run: 2026-04-05 17:41:13

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 3645ms | - [📜](./01/serial.log) - |
| 2 | Then I should see a message in the serial output that says "anther: Listening on port 80" | ✅ | 2529ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11444208534] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11450133618] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11453887797] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11455861164] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11457374247] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11458005405] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11458651413] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11459230893] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11459818062] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11460419652] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11461166640] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11461779714] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11462610258] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11463280554] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11463993750] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11464620981] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11465251413] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11465857623] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11466481323] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11467128024] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11467715919] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11468306520] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11468910420] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11469640578] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11470398060] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11471023839] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11471641929] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11472293217] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11472885072] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11473561737] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11474254737] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11474883783] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11475540714] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11476176360] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11476772109] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11477535795] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11478270870] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11478986475] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11479685745] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11480475204] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11481190677] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11481910110] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11483205591] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11484684849] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11485512522] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11486062533] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11486580369] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11487119259] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11487800874] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11488581720] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11489166810] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11489694084] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11490238683] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11490799188] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x77983000 (Usable)
[11491347450] [INFO] [kernel::memory] [CPU0]   [11] 0x77983000 - 0x779e7000 (Reserved)
[11491912080] [INFO] [kernel::memory] [CPU0]   [12] 0x779e7000 - 0x779e8000 (Other)
[11492458296] [INFO] [kernel::memory] [CPU0]   [13] 0x779e8000 - 0x779e9000 (Reserved)
[11493022035] [INFO] [kernel::memory] [CPU0]   [14] 0x779e9000 - 0x779ea000 (Other)
[11493587358] [INFO] [kernel::memory] [CPU0]   [15] 0x779ea000 - 0x779eb000 (Reserved)
[11494151757] [INFO] [kernel::memory] [CPU0]   [16] 0x779eb000 - 0x779ec000 (Other)
[11494841655] [INFO] [kernel::memory] [CPU0]   [17] 0x779ec000 - 0x779ed000 (Reserved)
[11495436645] [INFO] [kernel::memory] [CPU0]   [18] 0x779ed000 - 0x77a78000 (Other)
[11495987250] [INFO] [kernel::memory] [CPU0]   [19] 0x77a78000 - 0x77a79000 (Reserved)
[11496551979] [INFO] [kernel::memory] [CPU0]   [20] 0x77a79000 - 0x77efa000 (Other)
[11497119678] [INFO] [kernel::memory] [CPU0]   [21] 0x77efa000 - 0x77efb000 (Reserved)
[11497684077] [INFO] [kernel::memory] [CPU0]   [22] 0x77efb000 - 0x7801c000 (Other)
[11498228412] [INFO] [kernel::memory] [CPU0]   [23] 0x7801c000 - 0x7801d000 (Reserved)
[11498804658] [INFO] [kernel::memory] [CPU0]   [24] 0x7801d000 - 0x7881d000 (Other)
[11499351237] [INFO] [kernel::memory] [CPU0]   [25] 0x7881d000 - 0x7881e000 (Reserved)
[11499912435] [INFO] [kernel::memory] [CPU0]   [26] 0x7881e000 - 0x788df000 (Other)
[11500513101] [INFO] [kernel::memory] [CPU0]   [27] 0x788df000 - 0x788e0000 (Reserved)
[11501109906] [INFO] [kernel::memory] [CPU0]   [28] 0x788e0000 - 0x788ef000 (Other)
[11501652294] [INFO] [kernel::memory] [CPU0]   [29] 0x788ef000 - 0x788f0000 (Reserved)
[11502226197] [INFO] [kernel::memory] [CPU0]   [30] 0x788f0000 - 0x788f5000 (Other)
[11502769740] [INFO] [kernel::memory] [CPU0]   [31] 0x788f5000 - 0x788f6000 (Reserved)
[11503362387] [INFO] [kernel::memory] [CPU0]   [32] 0x788f6000 - 0x788fb000 (Other)
[11503920945] [INFO] [kernel::memory] [CPU0]   [33] 0x788fb000 - 0x788fc000 (Reserved)
[11504511876] [INFO] [kernel::memory] [CPU0]   [34] 0x788fc000 - 0x78928000 (Other)
[11505076506] [INFO] [kernel::memory] [CPU0]   [35] 0x78928000 - 0x7892a000 (Reserved)
[11505637044] [INFO] [kernel::memory] [CPU0]   [36] 0x7892a000 - 0x78933000 (Other)
[11506176627] [INFO] [kernel::memory] [CPU0]   [37] 0x78933000 - 0x78935000 (Reserved)
[11506755348] [INFO] [kernel::memory] [CPU0]   [38] 0x78935000 - 0x7893d000 (Other)
[11507295360] [INFO] [kernel::memory] [CPU0]   [39] 0x7893d000 - 0x7893e000 (Reserved)
[11507851740] [INFO] [kernel::memory] [CPU0]   [40] 0x7893e000 - 0x78948000 (Other)
[11508388848] [INFO] [kernel::memory] [CPU0]   [41] 0x78948000 - 0x78949000 (Reserved)
[11508945195] [INFO] [kernel::memory] [CPU0]   [42] 0x78949000 - 0x78956000 (Other)
[11509486065] [INFO] [kernel::memory] [CPU0]   [43] 0x78956000 - 0x78958000 (Reserved)
[11510058351] [INFO] [kernel::memory] [CPU0]   [44] 0x78958000 - 0x78966000 (Other)
[11510598726] [INFO] [kernel::memory] [CPU0]   [45] 0x78966000 - 0x78967000 (Reserved)
[11511154644] [INFO] [kernel::memory] [CPU0]   [46] 0x78967000 - 0x78973000 (Other)
[11511693237] [INFO] [kernel::memory] [CPU0]   [47] 0x78973000 - 0x78974000 (Reserved)
[11512248792] [INFO] [kernel::memory] [CPU0]   [48] 0x78974000 - 0x78978000 (Other)
[11512808868] [INFO] [kernel::memory] [CPU0]   [49] 0x78978000 - 0x78979000 (Reserved)
[11513382276] [INFO] [kernel::memory] [CPU0]   [50] 0x78979000 - 0x78989000 (Other)
[11513922684] [INFO] [kernel::memory] [CPU0]   [51] 0x78989000 - 0x7898a000 (Reserved)
[11514479658] [INFO] [kernel::memory] [CPU0]   [52] 0x7898a000 - 0x78a1a000 (Other)
[11515044585] [INFO] [kernel::memory] [CPU0]   [53] 0x78a1a000 - 0x78a1b000 (Reserved)
[11515613208] [INFO] [kernel::memory] [CPU0]   [54] 0x78a1b000 - 0x78a26000 (Other)
[11516151438] [INFO] [kernel::memory] [CPU0]   [55] 0x78a26000 - 0x78a27000 (Reserved)
[11516729730] [INFO] [kernel::memory] [CPU0]   [56] 0x78a27000 - 0x78a4c000 (Other)
[11517273735] [INFO] [kernel::memory] [CPU0]   [57] 0x78a4c000 - 0x78a4d000 (Reserved)
[11517834042] [INFO] [kernel::memory] [CPU0]   [58] 0x78a4d000 - 0x78a59000 (Other)
[11518374978] [INFO] [kernel::memory] [CPU0]   [59] 0x78a59000 - 0x78a5a000 (Reserved)
[11518934526] [INFO] [kernel::memory] [CPU0]   [60] 0x78a5a000 - 0x78a67000 (Other)
[11519475429] [INFO] [kernel::memory] [CPU0]   [61] 0x78a67000 - 0x78a68000 (Reserved)
[11520073785] [INFO] [kernel::memory] [CPU0]   [62] 0x78a68000 - 0x78ab0000 (Other)
[11520620694] [INFO] [kernel::memory] [CPU0]   [63] 0x78ab0000 - 0x78ab1000 (Reserved)
[11521429722] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11739252525] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485787 free frames
[11750521893] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11755649169] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11756931186] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11757774996] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11762338995] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11764090932] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11765234052] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11765915568] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11766579198] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11767277082] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11768281536] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11769241242] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11769950247] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11770658559] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11771338920] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11772050796] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11773093200] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11774149101] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11774897640] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11776804941] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11777865462] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11778963372] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11780620797] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11782105170] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11782862553] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11783497440] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11784257661] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12109893609] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12110894400] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12113998677] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12114881922] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12115643463] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12117193572] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12130486863] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12131945958] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12132745845] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12134316678] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12134904903] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12137377428] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12144496782] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12146221626] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12159603027] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12160201152] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12175977957] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12176593902] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12178636371] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12179867733] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12180915648] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12183342435] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12184152255] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12219452124] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62919200 ticks/sec), init_cnt=629192 for 100Hz
[12220961577] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12221784861] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12222929466] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12228617742] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12258782646] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12259753869] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12261138648] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12262442016] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12263511018] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12266281599] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12267751419] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12287720412] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12288625965] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12289387935] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12290231349] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12290911677] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12292386348] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12293029914] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12317823870] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12319582440] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12320456643] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12322122087] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12323362656] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12324737238] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12325788387] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12326443569] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12333175008] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12334064985] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12335808606] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12336648126] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12342102432] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12343821831] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12344571954] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12345908256] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12346955511] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12347983329] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12362124159] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12365175603] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12366723633] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12367495635] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12388062522] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12411369135] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12413521494] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12416781432] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12418507299] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12421028598] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12424920189] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12428257083] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12429378852] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12430877118] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12440589084] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12446293992] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12449996889] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12454167330] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12458110533] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12459050637] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12471328320] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12472220607] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12482287356] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12483178620] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12512114736] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12512973858] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[12863133690] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13341615276] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13368181827] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[13402378011] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[14687789985] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=446 watches=0 history=960 journal=768 symbols=94 drops=0
[15455119713] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15547966632] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15549459585] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[15647406984] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[15705577437] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[15733109139] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[15734435310] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[15735437256] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[15739931196] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[15757964277] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[15775443684] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[15782748432] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[15836913114] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[15860728191] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[15861678030] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[15865856622] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[15898848900] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[15918823965] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[15922043148] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[15923107233] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[15989994702] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[15991733439] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[16085338302] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[16150137333] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[16162144383] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[16167010035] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[16169625912] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[16174047648] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[16238665608] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[16244043816] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[16245023982] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[16245814233] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[16246731534] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[16247398068] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[16248046782] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[16248700116] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[16249292565] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[16249915803] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[16250547126] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[16251158319] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[16251801951] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[16252547685] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[16253260452] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[16253973054] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[16254614475] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[16255260978] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[16255882170] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[16256539596] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[16257157224] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[16257754953] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[16258358754] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[16259011791] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[16259900613] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[16260658590] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[16261293246] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[16261912953] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[16262582061] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[16263209952] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[16264043895] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[16264849524] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[16265503188] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[16266159492] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[16266844605] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[16267718049] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[16268449626] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[16269181467] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[16269950070] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[16270678677] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[16271435664] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[16272172092] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[16272930201] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[16274707812] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[16276407972] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[16277339694] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16285665297] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[16300131474] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[16304507241] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[16313568810] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[16314406977] [CONTRACT] [kernel] [CPU0] Spawning init process...
[16316721168] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[16319662161] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[16320379086] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [16326211176] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[16328035020] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564572728
[16349109018] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[16351455879] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[16353039879] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[16354382748] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[16364424846] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[16369262976] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[16372807737] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[16373921685] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[16377777174] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[16380648801] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[16381822776] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[16385610582] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[16386704301] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[16387921044] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[16389259326] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16392454716] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16393579851] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16394862759] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16396416234] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16398241959] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16400192820] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16405783878] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[16449050508] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[16457361723] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[16463236449] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[16469132295] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[16472698209] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[16477079949] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[16483375425] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[16487902101] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[16493854938] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[16499435106] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[16505133183] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[16511857692] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[16517658696] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[16522971795] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[16528020828] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[16533482064] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[16539046953] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[16544323158] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[16548899763] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[16553721954] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[16558176723] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[16564143948] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[16569312672] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[16574625012] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[16579444992] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[16584351234] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[16589358885] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[16594238463] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[16599515427] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[16606307124] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[16609947156] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[16613170233] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[16618815345] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[16623643476] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[16628528565] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[16634249643] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[16639440774] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[16645721928] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[16651123236] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[16656347499] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[16661808405] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[16665217668] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[16685623185] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[16816707567] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[16823244966] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[16824018024] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16825968720] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16831015707] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[16832269509] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16842773409] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[16848492474] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16849688889] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16851080862] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564572760
[16855598298] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[16858644858] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[16859350563] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16861891563] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16866481962] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[16868168031] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16875195513] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[16878342063] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[16879557684] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[16880873658] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564572696
[16884096966] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[16887035352] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[16888558665] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[16891244799] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[16896299904] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:41:30 = 1775436090 unix_secs
[16898325411] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775436090, mono_ns=8448876975, offset=1775436081551123025ns
[16899995343] [INFO] [rtc_cmos] [CPU1] System clock anchored
[16909767567] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[16943727042] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[16959656208] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[16960776228] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16962746394] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16968935544] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[16971582177] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[16980348759] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[16984341330] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[16987655421] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16989521835] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369210752 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564572728
[16995214302] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[16999875750] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[17002187928] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[17003454633] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17006306097] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17013595797] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17016466500] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17023636377] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[17027392668] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[17028549219] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17030434542] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369277344 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564572760
[17033461698] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[17036130408] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[17040898347] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[17042424531] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[17043868776] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[17045463336] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[17046340344] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[17047328958] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[17064280695] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[17066196345] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[17547321495] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17551839294] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[17552815830] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17554538991] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17561759061] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17564252343] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17571358167] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[17574084495] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[17575432908] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[17577634074] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369352896 RFLAGS_BEFORE=134 CR3_BEFORE=59662336 fs_base=0 gs_base=18446744071564572696
[17583361818] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[17586100983] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[17587222389] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[17588450649] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[17592892251] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[17595221787] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[17597801991] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[17598700515] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[17599784961] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[17600754831] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[17603008995] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[17604541020] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[17605610814] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[17606493663] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[17607635991] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[17608394661] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[17609423007] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[17612545302] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[17614869789] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[17620316703] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[17628316563] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[17629796151] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[17649641823] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[17652393462] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[17654817345] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[17655916278] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[17657375208] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[17658416589] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17660567859] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17670637545] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[17672598141] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17680089933] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[17683007925] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[17685152001] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[17686141011] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17689545753] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17694818394] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17696129583] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17702941311] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[17705902401] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[17707260516] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[17708986680] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369500048 RFLAGS_BEFORE=134 CR3_BEFORE=68235264 fs_base=0 gs_base=18446744071564572760
[17712424488] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[17713394754] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17714998455] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17715893910] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[17718055707] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[17720440056] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17722936077] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17724444243] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[17726747577] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[17728245876] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[17730395034] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[17731449879] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[17733664212] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[17735348367] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[17736099909] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[17737795647] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[17738947644] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[17739832407] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[17740899363] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[17743096965] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[17744087988] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1236 port=2 model='                                        ' rpc_port=5
[17768846007] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[17769934182] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[17770711695] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[17771487756] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[17772302493] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[17773072647] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[17773935333] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[17774941437] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[17780738844] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[17782217178] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369433808 RFLAGS_BEFORE=130 CR3_BEFORE=59830272 fs_base=0 gs_base=18446744071564572728
[17787347424] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[18110177052] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[18112042278] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369582832 RFLAGS_BEFORE=134 CR3_BEFORE=68349952 fs_base=0 gs_base=18446744071564572696
[18117032736] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[18119129424] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[18121822554] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[18137358459] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[18139482933] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[18141946317] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[18147087948] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18149774742] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[18150755337] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18153542319] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18156297621] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[18161262669] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[18163804230] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[18171799536] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[18175498143] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[18177532593] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[18179203845] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[18180117351] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18181909185] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18201751062] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18206057397] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[18226770441] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[18230082189] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[18232528413] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18234381924] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369719600 RFLAGS_BEFORE=134 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564572760
[18239433729] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[18240837714] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369654064 RFLAGS_BEFORE=134 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564572728
[18243711849] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[18245176785] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[18245971821] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[18247859817] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[18252643761] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[18254773218] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[18259723152] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[18261110835] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[18262361700] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[18264226365] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[18267978564] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[18269103798] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[18270918765] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[18272710665] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[18273471513] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18275510649] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18341008983] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[18365166732] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[18372642651] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[18375558366] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0102438
[18376703664] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18378320763] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369785136 RFLAGS_BEFORE=134 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564572696
[18381847605] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[18382726461] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18384559479] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[18385463349] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18387238056] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[18392151096] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18393754302] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18426699258] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18431777826] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[18438859692] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[18442060032] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[18444349869] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[18445177080] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18446998581] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18452366427] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18455025270] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18462206763] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[18465578703] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
[18466658991] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18469158180] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18470161116] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18471545400] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369916208 RFLAGS_BEFORE=134 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564572760
[18475099797] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18477126261] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[18478701549] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[18483369003] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18487052397] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18494139081] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[18497301339] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[18498393441] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18499798053] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369981744 RFLAGS_BEFORE=134 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564572696
[18504977964] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[18505720761] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18507453591] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18516675639] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18520361013] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[18521299731] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[18522245544] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[18524300223] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18526220658] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[18529302957] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[18531906228] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[18534055584] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[18534748089] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18536314731] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18559975797] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18565035621] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[18571988391] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[18574810683] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010df80
[18576213579] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18577969641] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370113904 RFLAGS_BEFORE=130 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564572760
[18582417777] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[18583275348] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18585305772] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18586177566] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[18602324367] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[18605524773] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18607900377] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[18611288553] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18612834570] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18614250336] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[18615364878] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18616914360] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[18619244622] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[18620424801] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18623257422] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370198080 RFLAGS_BEFORE=130 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564572696
[18627227718] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[18628465911] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18630484686] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[18631567581] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18632866560] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[18642503319] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18643563345] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18644914860] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18646463418] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[18650343327] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18651676560] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18652878783] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18654194460] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[18668546490] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[18678453453] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[18682952475] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18684286434] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18685710549] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18687163902] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[18688692924] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[18691959363] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[18695139738] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18696528939] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18697768881] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18699261801] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[18701942589] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18703880316] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369850672 RFLAGS_BEFORE=134 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564572728
[18708200742] [INFO] [nectar] [CPU2] NECTAR: Started.
[18710328648] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
[18711515988] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18713793087] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370047824 RFLAGS_BEFORE=134 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564572728
[18717649566] [INFO] [fontd] [CPU3] FONTD: Service ready
[18718242774] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18719393154] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18720607125] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18721773378] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[18722484297] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[18724211418] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18725669985] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370266864 RFLAGS_BEFORE=134 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564572728
[18731095482] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[18733437921] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[18735181311] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[18751805985] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18753433545] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18754152648] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[18758292399] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18759654111] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18761119113] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18762565305] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=236 subj_lo=0
[18771139266] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18772302648] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18773412537] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18774488568] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[18779898753] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18780833808] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18781920696] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18783061539] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[18789177825] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18790117731] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18791137299] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18792193728] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[18798279027] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[18799770462] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[18801527019] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[18802863387] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[18805439499] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[18806644131] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[18808449627] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x44f7000
[18810483549] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[18813456222] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[18815885979] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[18817459023] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[18826993713] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[18832328724] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[18835000734] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[18844788534] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[18847670424] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[18850283034] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[18852065892] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[18853776117] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[18855445092] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[18856415952] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[18857249433] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[18858065523] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[18858694437] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[18859819836] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[18865369941] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[18871282452] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[18879400518] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18896928402] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[18898449240] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[18905288886] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[18920938113] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[18939826191] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18943864302] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18945366528] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18966526458] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[18973930305] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[18977387979] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[18979261158] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1258 backend=VirtIO-GPU
[18981447045] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[18982162353] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18983730744] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18991558806] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[19029798678] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[19048575282] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[19056837459] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[19063148577] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[19074187374] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[19075935087] [INFO] [anther] [CPU1] anther: Connected to network stack
[19090972230] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[19124182275] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[19139096262] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[19146817932] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[19149716124] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[19150596069] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f0fe8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4ea
[19152811194] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370552656 RFLAGS_BEFORE=134 CR3_BEFORE=72437760 fs_base=0 gs_base=18446744071564572760
[19156203693] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[19156998663] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19158689517] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19159593123] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[19162845273] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[19164225003] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19171739532] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[19174389927] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[19176203871] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[19178898849] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f0fe8
[19179820572] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[19181974746] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370622288 RFLAGS_BEFORE=134 CR3_BEFORE=74018816 fs_base=0 gs_base=18446744071564572696
[19186871748] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[19187882505] [INFO] [echo] [CPU1] echo: starting up
[19189919727] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[19190736213] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[19200380265] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[19202133027] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[19216651245] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[19217715825] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[19219089549] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19220050080] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[19224935730] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[19235468340] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[19237142793] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[19238875887] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[19252843599] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[19253863365] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19254895374] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[19256531481] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[19257555438] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19259728125] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19260657273] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[19264779501] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19266871668] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[19270920339] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[19274439756] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[19276761273] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[19278785592] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[19280537232] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[19281716685] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[19282838553] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19283755788] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19285605471] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19292953614] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19296600807] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19303697061] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[19307100945] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[19308674088] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[19311024414] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[19312091337] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19314369327] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19318838979] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[19320163005] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[19323923685] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[19327009152] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[19327915563] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[19328816529] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[19331842596] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[19333426959] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[19336776195] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0111f20
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19339388772] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f8390
[19340457411] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370822992 RFLAGS_BEFORE=134 CR3_BEFORE=74944512 fs_base=0 gs_base=18446744071564572696
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19346345436] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[19347356094] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370757456 RFLAGS_BEFORE=134 CR3_BEFORE=74784768 fs_base=0 gs_base=18446744071564572760
[19352685528] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[19358508114] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[19360198011] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[19362729672] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19367577999] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[19399384488] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[19433483619] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[19440888159] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[19456310808] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[19466371089] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[19469097912] [INFO] [bloom] [CPU3] bloom: creating surface...
[19470905256] [INFO] [bloom] [CPU3] bloom: surface created!
[19472333958] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[19476891423] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[19482953358] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1264
[19484595801] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[19496705019] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[19499694819] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[19500943704] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[19502135169] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [19505964423] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[19517903460] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[19526526921] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[19535232552] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[19536287925] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[19542928812] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19546967715] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[19548915771] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
T:07D0 T:0640 T:F0B0 [19564188666] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1264
[19581218745] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[19591350801] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[19594689279] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[19597803093] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[19600589217] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
T:1220 [19612213500] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[19615061763] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[19621199268] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[19624167684] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[19629392046] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[19632046863] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[19634568657] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[19636680690] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[19642152453] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[19647281511] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[19648658271] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[19649650284] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[19664589879] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=282
[19665938325] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[19666852227] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19668004191] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19669233606] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19670505261] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=282 subj_lo=0
[19685001039] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[19687287444] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[19691891076] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1272)
[19693266186] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[19694009082] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[19705603005] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=244
[19707199644] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[19708263465] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19709332995] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19710548616] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19711783179] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[19726483260] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1279)
[19728075543] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[19741346625] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=284
[19742778330] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[19743855021] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19745071500] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19746519342] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19748142876] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 
```
</details>
