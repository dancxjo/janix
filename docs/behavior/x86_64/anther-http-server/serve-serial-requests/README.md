# ✅ Scenario: Serve serial requests

> Last run: 2026-04-05 17:41:13

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the anther server is ready | ✅ | 14822ms | - [📜](./01/serial.log) - |
| 2 | When I make a GET request to "/health" | ✅ | 481ms | - [📜](./02/serial.log) - |
| 3 | Then the response status should be 200 | ✅ | 16ms | - - - |
| 4 | And the response body should contain "ok" | ✅ | 0ms | - - - |
| 5 | When I make a GET request to "/graph" | ✅ | 513ms | - [📜](./05/serial.log) - |
| 6 | Then the response status should be 200 | ✅ | 0ms | - - - |
| 7 | And the response body should contain "Graph index" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11458279800] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11464335399] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11467892403] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11469866199] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11471526429] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11472164682] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11472899724] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11473484583] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11474068749] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11474736636] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11475336114] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11475978129] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11476725414] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11477387295] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11478066897] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11478671259] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11479308192] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11480000862] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11480702079] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11481386664] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11482045806] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11482706796] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11483331750] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11483967561] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11484603867] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11485205325] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11485837275] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11486485428] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11487075633] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11487681447] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11488422033] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11489061738] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11489777343] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11490431370] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11491046490] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11491739853] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11492468361] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11493175155] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11493862578] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11494580196] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11495279565] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11496010680] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11497288275] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11498875509] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11499663318] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11500202406] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11500758060] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11501278965] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11501911377] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11502455118] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11503007142] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11503595037] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11504178939] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11504792739] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x77983000 (Usable)
[11505426537] [INFO] [kernel::memory] [CPU0]   [11] 0x77983000 - 0x779e7000 (Reserved)
[11506060863] [INFO] [kernel::memory] [CPU0]   [12] 0x779e7000 - 0x779e8000 (Other)
[11506607376] [INFO] [kernel::memory] [CPU0]   [13] 0x779e8000 - 0x779e9000 (Reserved)
[11507171148] [INFO] [kernel::memory] [CPU0]   [14] 0x779e9000 - 0x779ea000 (Other)
[11507739408] [INFO] [kernel::memory] [CPU0]   [15] 0x779ea000 - 0x779eb000 (Reserved)
[11508392775] [INFO] [kernel::memory] [CPU0]   [16] 0x779eb000 - 0x779ec000 (Other)
[11508966282] [INFO] [kernel::memory] [CPU0]   [17] 0x779ec000 - 0x779ed000 (Reserved)
[11509525005] [INFO] [kernel::memory] [CPU0]   [18] 0x779ed000 - 0x77a78000 (Other)
[11510065545] [INFO] [kernel::memory] [CPU0]   [19] 0x77a78000 - 0x77a79000 (Reserved)
[11510621166] [INFO] [kernel::memory] [CPU0]   [20] 0x77a79000 - 0x77efa000 (Other)
[11511160650] [INFO] [kernel::memory] [CPU0]   [21] 0x77efa000 - 0x77efb000 (Reserved)
[11511718218] [INFO] [kernel::memory] [CPU0]   [22] 0x77efb000 - 0x7801c000 (Other)
[11512280274] [INFO] [kernel::memory] [CPU0]   [23] 0x7801c000 - 0x7801d000 (Reserved)
[11512837710] [INFO] [kernel::memory] [CPU0]   [24] 0x7801d000 - 0x7881d000 (Other)
[11513377062] [INFO] [kernel::memory] [CPU0]   [25] 0x7881d000 - 0x7881e000 (Reserved)
[11513971788] [INFO] [kernel::memory] [CPU0]   [26] 0x7881e000 - 0x788df000 (Other)
[11514515793] [INFO] [kernel::memory] [CPU0]   [27] 0x788df000 - 0x788e0000 (Reserved)
[11515073592] [INFO] [kernel::memory] [CPU0]   [28] 0x788e0000 - 0x788ef000 (Other)
[11515652280] [INFO] [kernel::memory] [CPU0]   [29] 0x788ef000 - 0x788f0000 (Reserved)
[11516220474] [INFO] [kernel::memory] [CPU0]   [30] 0x788f0000 - 0x788f5000 (Other)
[11516773752] [INFO] [kernel::memory] [CPU0]   [31] 0x788f5000 - 0x788f6000 (Reserved)
[11517337359] [INFO] [kernel::memory] [CPU0]   [32] 0x788f6000 - 0x788fb000 (Other)
[11517923868] [INFO] [kernel::memory] [CPU0]   [33] 0x788fb000 - 0x788fc000 (Reserved)
[11518512918] [INFO] [kernel::memory] [CPU0]   [34] 0x788fc000 - 0x78928000 (Other)
[11519061411] [INFO] [kernel::memory] [CPU0]   [35] 0x78928000 - 0x7892a000 (Reserved)
[11519622873] [INFO] [kernel::memory] [CPU0]   [36] 0x7892a000 - 0x78933000 (Other)
[11520172059] [INFO] [kernel::memory] [CPU0]   [37] 0x78933000 - 0x78935000 (Reserved)
[11520756060] [INFO] [kernel::memory] [CPU0]   [38] 0x78935000 - 0x7893d000 (Other)
[11521301121] [INFO] [kernel::memory] [CPU0]   [39] 0x7893d000 - 0x7893e000 (Reserved)
[11521881261] [INFO] [kernel::memory] [CPU0]   [40] 0x7893e000 - 0x78948000 (Other)
[11522425959] [INFO] [kernel::memory] [CPU0]   [41] 0x78948000 - 0x78949000 (Reserved)
[11522986299] [INFO] [kernel::memory] [CPU0]   [42] 0x78949000 - 0x78956000 (Other)
[11523528786] [INFO] [kernel::memory] [CPU0]   [43] 0x78956000 - 0x78958000 (Reserved)
[11524111500] [INFO] [kernel::memory] [CPU0]   [44] 0x78958000 - 0x78966000 (Other)
[11524657683] [INFO] [kernel::memory] [CPU0]   [45] 0x78966000 - 0x78967000 (Reserved)
[11525242476] [INFO] [kernel::memory] [CPU0]   [46] 0x78967000 - 0x78973000 (Other)
[11525792553] [INFO] [kernel::memory] [CPU0]   [47] 0x78973000 - 0x78974000 (Reserved)
[11526509247] [INFO] [kernel::memory] [CPU0]   [48] 0x78974000 - 0x78978000 (Other)
[11527140669] [INFO] [kernel::memory] [CPU0]   [49] 0x78978000 - 0x78979000 (Reserved)
[11527793937] [INFO] [kernel::memory] [CPU0]   [50] 0x78979000 - 0x78989000 (Other)
[11528415063] [INFO] [kernel::memory] [CPU0]   [51] 0x78989000 - 0x7898a000 (Reserved)
[11529042129] [INFO] [kernel::memory] [CPU0]   [52] 0x7898a000 - 0x78a1a000 (Other)
[11529666654] [INFO] [kernel::memory] [CPU0]   [53] 0x78a1a000 - 0x78a1b000 (Reserved)
[11530235079] [INFO] [kernel::memory] [CPU0]   [54] 0x78a1b000 - 0x78a26000 (Other)
[11530782516] [INFO] [kernel::memory] [CPU0]   [55] 0x78a26000 - 0x78a27000 (Reserved)
[11531346717] [INFO] [kernel::memory] [CPU0]   [56] 0x78a27000 - 0x78a4c000 (Other)
[11531915670] [INFO] [kernel::memory] [CPU0]   [57] 0x78a4c000 - 0x78a4d000 (Reserved)
[11532481158] [INFO] [kernel::memory] [CPU0]   [58] 0x78a4d000 - 0x78a59000 (Other)
[11533026285] [INFO] [kernel::memory] [CPU0]   [59] 0x78a59000 - 0x78a5a000 (Reserved)
[11533589793] [INFO] [kernel::memory] [CPU0]   [60] 0x78a5a000 - 0x78a67000 (Other)
[11534134887] [INFO] [kernel::memory] [CPU0]   [61] 0x78a67000 - 0x78a68000 (Reserved)
[11534698923] [INFO] [kernel::memory] [CPU0]   [62] 0x78a68000 - 0x78ab0000 (Other)
[11535263586] [INFO] [kernel::memory] [CPU0]   [63] 0x78ab0000 - 0x78ab1000 (Reserved)
[11536219464] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11758215117] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485787 free frames
[11768877945] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11773904901] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11775255690] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11776244073] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11780533809] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11782471734] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11783621619] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11784321285] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11784989337] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11785680720] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11786635575] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11787572445] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11788252542] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11788951317] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11789617917] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11790309795] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11791357116] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11792433246] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11793148620] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11795070639] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11796162180] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11797298997] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11798913423] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11800391790] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11801247909] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11801845242] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11802608730] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12148738833] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12149793612] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12152946300] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12153827334] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12154661409] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12156247620] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12169816692] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12171245262] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12172085574] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12173805402] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12174418476] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12176937102] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12185541192] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12187444599] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12201205830] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12201881934] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12219179478] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12219860433] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12221993784] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12223393842] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12224640978] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12227245008] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12228130266] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12263207814] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62466500 ticks/sec), init_cnt=624665 for 100Hz
[12264786534] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12265575960] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12266726439] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12272833551] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12303056898] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12304285521] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12306657099] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12308268093] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12309445401] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12312193377] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12313475361] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12332063799] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12333060960] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12333977634] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12334719738] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12335468211] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12336047658] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12336755442] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12362459175] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12364256421] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12365311959] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12367144482] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12368139366] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12370094979] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12370861437] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12371851701] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12378585978] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12379545486] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12381844629] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12382814895] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12387981804] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12389889798] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12390624576] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12391956324] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12392997309] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12393844386] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12405682905] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12409038213] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12410145462] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12410888655] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12432157584] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12452347215] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12455120106] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12459256392] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12461736111] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12464652783] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12467662119] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12470477382] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12471528267] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12472616343] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12479832618] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12482544327] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12485840631] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12488381829] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12490913919] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12493305726] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12494210223] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12507147444] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12507903474] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12514545516] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12515412261] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12543253140] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12544179219] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[12893875533] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13372669530] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13400036331] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[13436155953] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[14717858364] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=446 watches=0 history=959 journal=768 symbols=93 drops=0
[15482546937] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15576508233] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15577893837] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[15676189881] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[15753684804] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[15794111619] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[15798549294] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[15799206984] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[15805270041] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[15824164026] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[15844998180] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[15847232214] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[15898698948] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[15920129148] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[15921147132] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[15925229826] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[15957343215] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[15977251422] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[15980365698] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[15981356391] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[16048999725] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[16050845679] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[16142527236] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[16209082593] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[16221770301] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[16225865535] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[16228705482] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[16230856224] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=583 watches=0 history=1024 journal=1024 symbols=181 drops=0
[16303916673] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[16309886571] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[16310861160] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[16311686094] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[16312677150] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[16313711106] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[16314528846] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[16315296558] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[16316154591] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[16316821719] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[16317465186] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[16318138320] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[16318779807] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[16319478582] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[16320172308] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[16320928041] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[16321582563] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[16322238768] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[16323079113] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[16323774885] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[16324443663] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[16325058519] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[16325720499] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[16326376935] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[16327086897] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[16327860252] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[16328571270] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[16329290472] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[16330015977] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[16330636014] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[16331327430] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[16331982150] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[16332633207] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[16333268490] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[16333909284] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[16334564961] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[16335300498] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[16336038840] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[16336817838] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[16337597892] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[16338367584] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[16339113879] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[16339869249] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[16341500769] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[16343226438] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[16344307452] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16353634836] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[16368283965] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[16373143644] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[16383339786] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[16384160034] [CONTRACT] [kernel] [CPU0] Spawning init process...
[16386505707] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[16390099044] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[16390958958] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [16398241332] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[16399120683] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564572728
[16415445321] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[16420493100] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[16421743668] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[16422816135] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[16433056662] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[16437639570] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[16441137438] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[16442302107] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[16446129546] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[16449013647] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[16450207587] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[16454445843] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[16455977373] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[16457070927] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[16458193059] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16461602553] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16462849095] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16464412569] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16466254959] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16467772200] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16469369664] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16473856509] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[16514729022] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[16521727200] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[16526464482] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[16531148271] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[16534697454] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[16538668806] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[16543819941] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[16548779808] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[16554008691] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[16559315058] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[16565029635] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[16571668542] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[16577843304] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[16584507588] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[16591038981] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[16596890640] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[16603552020] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[16610198187] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[16616236857] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[16623067989] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[16630243740] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[16637769027] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[16643253297] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[16650322029] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[16655884047] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[16662648354] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[16668428568] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[16674652731] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[16680451590] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[16685313711] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[16690019676] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[16694701485] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[16700656104] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[16705853769] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[16713181221] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[16722244242] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[16730553312] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[16738319730] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[16748778189] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[16755507285] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[16764811107] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[16768448169] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[16792842594] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[16950006843] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[16957113228] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[16957959018] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16959739896] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16965489816] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[16966853376] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16975960518] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[16980996054] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16982006547] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16983571242] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564572760
[16988030169] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[16991948721] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[16992832527] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16994653995] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16998926010] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[17000780907] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17008589598] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[17011607877] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[17012638632] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[17014064430] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564572696
[17017043604] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[17020104849] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[17021663439] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[17023754022] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17029869120] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:41:43 = 1775436103 unix_secs
[17031943962] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775436103, mono_ns=8515746508, offset=1775436094484253492ns
[17033664747] [INFO] [rtc_cmos] [CPU1] System clock anchored
[17044776639] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[17083659978] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[17092228560] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[17093222223] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17094937563] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17103428331] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[17107073412] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[17117987502] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[17123864901] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[17128826550] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17130977061] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564572728
[17136499446] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[17141659821] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[17143470366] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[17144539665] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17146569264] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17155526355] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17158319574] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17165547036] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[17168438133] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[17169530004] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17171266431] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564572760
[17174915505] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[17178266490] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[17179631766] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[17191950402] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[17193645018] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[17195128104] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[17199774339] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[17201399952] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[17215473231] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[17217295359] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[17700384801] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17706528081] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[17709994599] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[17712075315] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[17717077191] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[17718663204] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[17720792265] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[17722412664] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[17723567598] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[17726202945] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[17727443118] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[17728752294] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[17729786943] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[17730863832] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[17731808193] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[17732987844] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[17761886406] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[17763172515] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17765635536] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17772923883] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17775368589] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17782667892] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[17786352837] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[17787843183] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[17790166515] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369357040 RFLAGS_BEFORE=134 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564572696
[17797838553] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[17814254337] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[17834935404] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[17853216678] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[17864606562] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[17865548349] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[17866360380] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[17867224650] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[17867947779] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[17868989787] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[17869975794] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[17871241014] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[17923689432] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[17929909008] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[17939654766] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[17941742049] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[17948790090] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[17952800943] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[17956395996] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[17958963528] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[17960118198] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17963410080] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17968971636] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[17970234876] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[17971633482] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17986372602] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[17989493082] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[17991361707] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[17992334547] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17994387675] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17999014110] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18000302562] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18007316877] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[18010892790] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[18012258858] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[18014122368] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369503120 RFLAGS_BEFORE=134 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564572760
[18019012605] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[18020290959] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18022806747] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[18023734014] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18024829416] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[18029503008] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18031533663] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[18032640252] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18033762912] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[18036231576] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[18039313479] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[18041021460] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[18042220713] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[18044860185] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[18046853286] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[18047838501] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[18049108572] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[18050101575] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[18051802527] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[18052660131] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[18054558687] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[18056917560] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369437584 RFLAGS_BEFORE=134 CR3_BEFORE=60096512 fs_base=0 gs_base=18446744071564572728
[18066157164] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[18441845730] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
[18443035446] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[18445183878] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369585040 RFLAGS_BEFORE=134 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564572696
[18449136024] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[18451505886] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[18452348376] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[18456001806] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[18457147665] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18459969396] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[18460808421] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18462186237] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[18463111656] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18470669745] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[18473027364] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[18482954787] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[18484280067] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[18488563632] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[18491289927] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[18493770438] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[18494593722] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18496365426] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18519084507] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18528962826] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[18550198590] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[18553470408] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[18555008472] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18556967847] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369717008 RFLAGS_BEFORE=130 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564572760
[18563548542] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[18565276818] [INFO] [netd] [CPU3] NETD: Starting network stack service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[18567352782] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369650576 RFLAGS_BEFORE=134 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564572728
[18571924107] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[18574818669] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[18576891597] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[18585346692] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[18588091137] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[18592643025] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[18594272103] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[18595329489] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[18596484093] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[18597444426] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[18607290735] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[18609163848] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[18611163681] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[18612001683] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18613872717] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18679726131] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[18703089966] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[18711712866] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[18714824799] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[18717081702] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18719218881] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369783232 RFLAGS_BEFORE=134 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564572696
[18723867855] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[18725005959] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18726802017] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[18727771326] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18729929097] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[18734301795] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18735819069] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18769074522] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18774261396] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[18781731837] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[18785774634] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[18788101200] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[18788840961] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18790421892] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18795871017] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18798281106] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18805379670] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[18808321290] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[18809598819] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18811372404] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369915392 RFLAGS_BEFORE=134 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564572760
[18814457046] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18815246769] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18816842418] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18820848090] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[18822250986] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[18824788488] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18828381198] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18835261203] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[18838114977] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[18839239353] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18840712572] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369981888 RFLAGS_BEFORE=134 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564572696
[18844049697] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[18844923669] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18847083420] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18857606361] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18861934113] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[18869627304] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[18872807646] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[18875180577] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[18876059895] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18877840608] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18904174608] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18909267003] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[18917575281] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[18920860926] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cc18
[18921885180] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18923361567] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370114080 RFLAGS_BEFORE=130 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564572760
[18926663811] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[18927397071] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18928857651] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[18930014400] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18947949636] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[18951223533] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18956072850] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[18959197851] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[18964361262] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[18965458545] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18967813689] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18969483621] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[18970680993] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18972545856] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18974247798] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18975341055] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[18977120316] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010e118
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18979359927] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370198352 RFLAGS_BEFORE=134 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564572696
[18986758263] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[18988401927] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[18989295864] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[18990346155] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[18994387599] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18996255003] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[19004818569] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19005838863] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19007143518] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19009073886] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[19015283232] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19016757045] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19018329132] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19019628804] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[19021320747] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[19030986612] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[19039013961] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[19042741377] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[19046884890] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19049019528] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369849312 RFLAGS_BEFORE=130 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564572728
[19055170596] [INFO] [nectar] [CPU2] NECTAR: Started.
[19057767696] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19059683148] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370048096 RFLAGS_BEFORE=134 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564572728
[19063304337] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19064398287] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19065587838] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[19066359807] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19067701422] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[19069826028] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19071732834] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370266976 RFLAGS_BEFORE=134 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564572728
[19077155955] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[19078241193] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19079690124] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[19080712992] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19082380713] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
[19083500436] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19085413908] [INFO] [fontd] [CPU3] FONTD: Service ready
[19086120207] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[19092377964] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[19095020142] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[19111393653] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[19117154562] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[19119563430] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19120797102] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19129166925] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1250 backend=VirtIO-GPU
[19130491281] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19131930807] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[19133106465] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19135234800] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19136590539] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19137908955] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19139362242] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=238 subj_lo=0
[19145978907] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19147345899] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19148468097] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19149980949] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=239 subj_lo=0
[19155664704] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19156617612] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19157649984] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19158775581] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=240 subj_lo=0
[19164791217] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19165865796] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19166967567] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19168513980] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[19177088469] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19178535420] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19179569607] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19181066190] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=242 pred=0 subj_lo=0
[19204566678] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[19220833929] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[19222865343] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[19272971487] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[19288739547] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[19295836560] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[19298787024] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db460
[19300137384] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e2
[19301798736] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370357760 RFLAGS_BEFORE=134 CR3_BEFORE=72052736 fs_base=0 gs_base=18446744071564572760
[19305659835] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[19306537734] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19308346695] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[19309075236] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19313222643] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[19314735000] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19321682193] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[19326352683] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[19328598234] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[19332918165] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[19334595654] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f4000
[19336431840] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f4000
[19337319672] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db460
[19338704583] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f7000
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[19340272182] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370436288 RFLAGS_BEFORE=134 CR3_BEFORE=73101312 fs_base=0 gs_base=18446744071564572696
[19344997815] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276783104)
[19347112125] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[19348156146] [INFO] [echo] [CPU1] echo: starting up
[19350158553] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f8000 phys=0x460c000
[19352588310] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[19353551844] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[19356395025] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[19358533656] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[19359794553] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[19362138279] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[19373401872] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[19382181984] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[19383029391] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[19385539833] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[19390719150] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[19393320639] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[19395506559] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[19397375580] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[19398774285] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[19400698680] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[19403222355] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[19404630366] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[19406106885] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[19408819419] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[19411488459] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[19412346492] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[19417209867] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[19420826964] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[19431374028] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19438101210] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19440268617] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19447516770] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[19449415161] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[19451312892] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[19459739310] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[19461911700] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[19464522000] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[19465838337] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19468199058] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19472227830] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[19473489222] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19475521032] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[19479382263] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[19483233891] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[19486535541] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[19488968796] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[19490559792] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[19492041591] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19492817883] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19494938628] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19502199585] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19505727120] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19513346721] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[19515952302] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[19518817824] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[19519655892] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[19520631966] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[19521976056] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[19523920977] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[19524770265] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19526755644] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19527757491] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f1098
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19529734950] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370699104 RFLAGS_BEFORE=134 CR3_BEFORE=73986048 fs_base=0 gs_base=18446744071564572760
[19533049998] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[19534363464] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[19542737610] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[19545921450] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[19547249502] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[19555713078] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1098
[19557357171] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19559105511] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370764640 RFLAGS_BEFORE=134 CR3_BEFORE=74145792 fs_base=0 gs_base=18446744071564572696
[19562191770] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19564100754] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[19575371178] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[19601981718] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[19610155620] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[19611348768] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[19621340574] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[19626524940] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[19647929367] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[19650444528] [INFO] [bloom] [CPU3] bloom: creating surface...
[19652153697] [INFO] [bloom] [CPU3] bloom: surface created!
[19653763470] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[19662708087] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[19669980495] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1261
[19671613665] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[19684580586] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[19690165440] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[19692403896] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[19694307765] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [19705487571] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[19710935343] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19712643060] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19714073280] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 3)
[19716077667] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[19724974071] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[19735627725] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
T:07D0 T:0640 T:F0B0 [19757544939] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1261
[19764544041] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19766108208] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[19785819207] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[19795347000] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[19797854967] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[19799986668] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[19802485164] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
T:1220 [19816196235] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[19828873680] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[19830942714] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[19835945316] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[19848301737] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=277
[19861739667] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19862762403] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[19863805005] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19865033331] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[19870158165] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[19873778826] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[19875092424] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19876247226] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19877568117] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19878978471] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=277 subj_lo=0
[19896129396] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1276)
[19897482858] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[19913452713] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=242
[19915879896] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[19916922696] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19918034334] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19919412678] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19920889032] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=242 pred=0 subj_lo=0
[19934284524] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1277)
[19935934095] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[19952195934] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=283
[19953666249] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[19954887942] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19956073137] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19957346574] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19958871768] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=283 subj_lo=0
[19976056485] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1279)
[19978657512] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[19993661523] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([251, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[20022600180] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[20042644809] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[20044539999] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[20051297211] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[20072406420] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=31, our_read=32)
[20074850103] [INFO] [anther] [CPU1] anther: Connected to network stack
[20081815809] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[20082895503] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[20084974503] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[20110298802] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[20112839604] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[20158836885] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[20167747248] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[20170888023] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[20175399585] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[20187367926] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[20189437422] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[20204934123] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[20208733083] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[20210363382] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[20217816432] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[20232520473] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[20236488690] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[20243027376] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[20245084563] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=1
[20247604575] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[20249942790] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=2
[20270177268] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=2)
[20272827597] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[20275046154] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[20278397502] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[20405415525] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[20426384253] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[20454069240] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[20555571696] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[20580778449] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[20582059311] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[20583979383] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[20601941085] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=664 watches=13 history=1024 journal=1024 symbols=310 drops=0
[20728343394] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[20920969608] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[21180242886] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[21382073790] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[21484556148] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[21572556489] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[21761987445] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[21964021233] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[21969556587] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[21971274765] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[21972490419] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[21973772568] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[21975519621] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[21995887419] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[21997100895] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[21998373870] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[21999886326] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=337 pred=0 subj_lo=0
[22003644003] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[22217424900] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[22416850929] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=715 watches=15 history=1024 journal=1024 symbols=340 drops=0
[22461899559] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[22522231116] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22528199166] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[22532313507] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=SynReceived
[22534982085] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[22541975511] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=74
[22542982737] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=25
[22544449554] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[22555371630] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=SynReceived
[22579325604] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[22583640915] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[22587676023] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=SynReceived
[22596285426] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[22601722506] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[22603498335] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[22607424675] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[22608488463] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[22615045794] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22618032393] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22619976291] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=149
[22621154061] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=25
[22622889630] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[22623938931] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[22638536316] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=Established
[22640127741] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:51118 on listener 2
[22737091608] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22739048838] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22740951255] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22743047745] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=306 pred=0 subj_lo=0
[22759315392] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[22886274213] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[23075226702] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[23092213584] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[23150562633] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[23183665692] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23185106637] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23186308794] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23187608763] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=340 pred=0 subj_lo=0
[23259586779] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[23276310585] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[23278697409] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[23321365617] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23322592293] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23323716174] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23325125802] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=341 pred=0 subj_lo=0
[23346155613] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[23357027859] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12222000
[23358676176] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[23360024028] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[23426399502] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=9d85f11f1afc2373)
[23427761643] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[23438840634] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[23444308206] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[23446962891] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[23449692816] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[23455141083] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[23476232868] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[23497943832] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[23500602048] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[23519805309] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x12223000
[23522258595] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[23700936963] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[23919432669] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[23921221665] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[23923427253] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23988076926] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=14aa2015f9906e3f)
[24242718258] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[24488374746] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[24515661951] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[24516800088] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24518026071] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24519280236] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=342 pred=0 subj_lo=0
[24739635129] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[24762393117] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=806 watches=19 history=1024 journal=1024 symbols=383 drops=0
[25028344110] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[25303401882] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[25544778006] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[25821984111] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[26106314256] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=2
[26112912078] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=3
[26122615167] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 37 (user thread) assigned to CPU 1
[26125095249] [INFO] [anther] [CPU1] anther: Thread spawned TID=37 for conn_handle=3
T:5EF0 [26144750610] [INFO] [anther] [CPU1] anther: Worker thread TID=37 starting for conn_handle=3
[26164062144] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26167258194] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[26169464706] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[26185051200] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=33, our_read=34)
[26187367998] [INFO] [anther] [CPU1] anther: Worker TID=37 connected to netd OK
[26200639311] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[26471859777] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[26515676352] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 85 bytes on conn_handle=3
[26527763031] [INFO] [anther] [CPU1] anther: GET /health Http11
[26550316947] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[26552343411] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[26558352645] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[26576578017] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[26578199109] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[26580818682] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26583074298] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26612248377] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[26617426275] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[26626891632] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[26628859917] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[26632081344] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26639608974] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[26641348668] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[26644472679] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26645649228] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26648561742] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26673238350] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 0 bytes on conn_handle=3
[26677772814] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[26735620461] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=851 watches=19 history=1024 journal=1024 symbols=385 drops=0
[26972388465] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[27199785360] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[27201689196] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[27204561417] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27296326596] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[27608761521] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[27716186256] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[27719200641] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[27722204499] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[27723749691] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=SynReceived
[27734195808] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[27735906462] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[27738097992] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27745067427] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=149
[27746202858] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=SynReceived
[27747402606] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=25
[27750869784] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[27759185619] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[28017934659] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[28343305254] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[28348372668] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=Established
[28350408735] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:51122 on listener 2
[28599104721] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=882 watches=19 history=1024 journal=1024 symbols=386 drops=0
[28815874395] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2429 ops=1 watches=19
[29005956375] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[29540439951] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[30235058931] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[30469974315] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=904 watches=19 history=1024 journal=1024 symbols=386 drops=0
[30495095268] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[30497307489] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[30499858620] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30686234436] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[31000638999] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[31196231187] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[31423725102] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=149
[31425684312] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=25
[31429390410] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[31647544269] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[32873958282] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=970 watches=19 history=1024 journal=1024 symbols=420 drops=0
[33819857445] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[33821289348] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[33822901992] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[34954426716] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[35079290730] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[35335116531] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1049 watches=19 history=1024 journal=1024 symbols=471 drops=0
[35412991845] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[35415087180] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[35447285643] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[35953527522] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[36006865290] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[36008193144] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[36009468396] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36010909242] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[36019731627] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[36020773602] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[36021887022] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36022985196] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=234 pred=0 subj_lo=0
[36030782931] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[36032384091] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[36033879882] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36035850609] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=473 pred=0 subj_lo=0
[36048869307] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[36050037540] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[36051332526] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36052626093] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=472 pred=0 subj_lo=0
[36058731258] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[36059888997] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[36061229787] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36062648028] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[36074991909] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[37313017038] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1082 watches=24 history=1024 journal=1024 symbols=474 drops=0
[39342471795] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1102 watches=24 history=1024 journal=1024 symbols=474 drops=0
[40447263318] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[40449539493] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[40452415641] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[41301691497] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=149
[41307424191] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=25
[41311484016] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[41670921732] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=2
[41686611186] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[41689720149] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[41694981438] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[41695851912] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=4
[41706688617] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 38 (user thread) assigned to CPU 1
[41708793621] [INFO] [anther] [CPU1] anther: Thread spawned TID=38 for conn_handle=4
T:5EF0 [41752852086] [INFO] [anther] [CPU1] anther: Worker thread TID=38 starting for conn_handle=4
[41787879375] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=35, our_read=36)
[41789862246] [INFO] [anther] [CPU1] anther: Worker TID=38 connected to netd OK
[42029607906] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 85 bytes on conn_handle=4
[42032336478] [INFO] [anther] [CPU1] anther: GET /health Http11
[42049258086] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[42051599766] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[42072631029] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=70
[42074897007] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[42077672109] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[42088199736] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[42134149332] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[42138030957] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[42143844666] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=70
[42146011149] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[42148813080] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[42150598149] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[42159929097] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=70
[42163686444] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[42168155997] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[42169383201] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[42267135636] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 0 bytes on conn_handle=4
[42324287346] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[42343978017] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=1139 watches=24 history=1024 journal=1024 symbols=474 drops=0
[42444811860] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[42735016335] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[43010033154] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[43164759726] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[43244140104] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[43256495898] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=2 (16384b)
[43260637266] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[43262791440] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[43266813711] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[43361448240] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[43480880355] [INFO] [fontd] [CPU3] FONTD: Auto-importing font asset ThingId([22, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[44048474514] [INFO] [fontd] [CPU3] FONTD: Auto-imported font 'Noto Sans' style 'Regular' from asset ThingId([22, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[44052361023] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (1 fonts)
[44160528192] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[44300354736] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[44593050282] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=2
[44973857643] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[45028745421] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[47064701904] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=70
[47066950029] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[47070978801] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[47080070895] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[47082700566] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[47084643804] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[47087709504] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=70
[47090253078] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[47093036364] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[47101790307] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=149
[47106016980] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=25
[47107199106] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[47108784426] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[47109922332] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[47114265594] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=Established
[47117633442] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:51136 on listener 2
[47275119210] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=1260 watches=24 history=1024 journal=1024 symbols=476 drops=0
[47378825208] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=3453 ops=1 watches=24
[47898425454] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=2
[47901714663] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=5
[47910615621] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 39 (user thread) assigned to CPU 1
[47912885493] [INFO] [anther] [CPU1] anther: Thread spawned TID=39 for conn_handle=5
[47915617893] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[47918498925] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
T:5EF0 [47979047622] [INFO] [anther] [CPU1] anther: Worker thread TID=39 starting for conn_handle=5
[48105502302] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=37, our_read=38)
[48107390298] [INFO] [anther] [CPU1] anther: Worker TID=39 connected to netd OK
[48121289535] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[48126187230] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[48176449299] [INFO] [anther] [CPU1] anther: Worker TID=39 got first 85 bytes on conn_handle=5
[48179137314] [INFO] [anther] [CPU1] anther: GET /health Http11
[48192854259] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[48195885507] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[48221095725] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=70
[48227884683] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[48230682687] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[48236879295] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[48306235428] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[48311988978] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[48324694836] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=25 len=70
[48327069351] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[48330315594] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[48331592628] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[48338309514] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=26 len=70
[48340956312] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[48343996701] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[48349010028] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[48351750480] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[48354614649] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[48376123983] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=27 len=70
[48378332079] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[48381042600] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[48382133976] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[48385981248] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[48390250920] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[48393424827] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=28 len=70
[48395297016] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[48397443105] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[48398367435] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[48402248631] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=29 len=124
[48403879029] [INFO] [anther] [CPU1] anther: Worker TID=39 got first 0 bytes on conn_handle=5
[48405290010] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 114 byte frame (118 encoded) to netd rx_port=25
[48407657859] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (118 bytes sent)
[48408723165] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 114 bytes
[48427342986] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[48438207147] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[48582306894] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=Established
[48586004016] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:51138 on listener 2
[49393430691] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=2
[49425944832] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=6
[49440026427] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 40 (user thread) assigned to CPU 1
[49443836640] [INFO] [anther] [CPU1] anther: Thread spawned TID=40 for conn_handle=6
T:5EF0 [49531919712] [INFO] [anther] [CPU1] anther: Worker thread TID=40 starting for conn_handle=6
[49703624520] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=39, our_read=40)
[49707370020] [INFO] [anther] [CPU1] anther: Worker TID=40 connected to netd OK
[49721833227] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[49727762436] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[49861622943] [INFO] [anther] [CPU1] anther: Worker TID=40 got first 60 bytes on conn_handle=6
[49863217503] [INFO] [anther] [CPU1] anther: GET /health Http11
[49878461787] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[49881344865] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[49893258030] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=30 len=70
[49896446919] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[49900141269] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[49901162454] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[49949814552] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[49957277040] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[49960760619] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=31 len=70
[49963194369] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[49966827174] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[49968079986] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[49996161600] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=70
[49998507075] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[50001560499] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[50011245240] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[50014504485] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[50016216195] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[50021579982] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=70
[50023661622] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[50027016963] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[50036355468] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[50039891154] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[50043593787] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[50061796158] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[50064490707] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[50069440674] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[50078700540] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=123
[50079719646] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[50081361132] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 113 byte frame (117 encoded) to netd rx_port=25
[50084785410] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (117 bytes sent)
[50098592709] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 113 bytes
[50115808050] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=Established
[50118237774] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:54162 on listener 2
[50186134944] [INFO] [anther] [CPU1] anther: Worker TID=40 got first 0 bytes on conn_handle=6
[51090782952] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=2
[51112289349] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[51114872424] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[51154260267] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=7
[51164417766] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 41 (user thread) assigned to CPU 1
[51166586328] [INFO] [anther] [CPU1] anther: Thread spawned TID=41 for conn_handle=7
T:5EF0 [51266102118] [INFO] [anther] [CPU1] anther: Worker thread TID=41 starting for conn_handle=7
[51480572319] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=41, our_read=42)
[51483105036] [INFO] [anther] [CPU1] anther: Worker TID=41 connected to netd OK
[51500933451] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[51506386404] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[51562708560] [INFO] [anther] [CPU1] anther: Worker TID=41 got first 59 bytes on conn_handle=7
[51564403770] [INFO] [anther] [CPU1] anther: GET /graph Http11
[51585013161] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 209 bytes - TCP ACK
[51586986660] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 209 bytes
[51610493781] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[51613359336] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[51617320623] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[51629142510] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[51700004697] [INFO] [
```
</details>
