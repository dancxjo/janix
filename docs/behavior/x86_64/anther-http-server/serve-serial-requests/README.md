# ✅ Scenario: Serve serial requests

> Last run: 2026-04-05 17:15:01

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the anther server is ready | ✅ | 8292ms | - [📜](./01/serial.log) - |
| 2 | When I make a GET request to "/health" | ✅ | 491ms | - [📜](./02/serial.log) - |
| 3 | Then the response status should be 200 | ✅ | 16ms | - - - |
| 4 | And the response body should contain "ok" | ✅ | 0ms | - - - |
| 5 | When I make a GET request to "/graph" | ✅ | 861ms | - [📜](./05/serial.log) - |
| 6 | Then the response status should be 200 | ✅ | 0ms | - - - |
| 7 | And the response body should contain "Graph index" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11048821311] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11054518530] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11058004089] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11059965048] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11061431568] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11062074837] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11062729392] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11063305176] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11063884458] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11064515286] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11065101993] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11065718367] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11066419353] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11067075393] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11067801096] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11068410540] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11069043447] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11069646522] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11070266955] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11070861747] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11071473633] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11072078028] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11072666814] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11073254478] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11073884844] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11074483398] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11075089245] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11075748156] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11076338295] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11076938136] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11077544280] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11078155011] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11078801547] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11079411981] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11079997962] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11080689972] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11081423595] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11082162267] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11082861438] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11083584897] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11084287203] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11084995944] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11086273044] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11087602581] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11088350460] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11088922317] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11089430781] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11089952313] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11090506548] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11091027585] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11091550008] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11092086687] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11092606569] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11093144832] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x77983000 (Usable)
[11093688078] [INFO] [kernel::memory] [CPU0]   [11] 0x77983000 - 0x779e7000 (Reserved)
[11094250101] [INFO] [kernel::memory] [CPU0]   [12] 0x779e7000 - 0x779e8000 (Other)
[11094792819] [INFO] [kernel::memory] [CPU0]   [13] 0x779e8000 - 0x779e9000 (Reserved)
[11095371474] [INFO] [kernel::memory] [CPU0]   [14] 0x779e9000 - 0x779ea000 (Other)
[11095916337] [INFO] [kernel::memory] [CPU0]   [15] 0x779ea000 - 0x779eb000 (Reserved)
[11096478591] [INFO] [kernel::memory] [CPU0]   [16] 0x779eb000 - 0x779ec000 (Other)
[11097022266] [INFO] [kernel::memory] [CPU0]   [17] 0x779ec000 - 0x779ed000 (Reserved)
[11097579372] [INFO] [kernel::memory] [CPU0]   [18] 0x779ed000 - 0x77a78000 (Other)
[11098118526] [INFO] [kernel::memory] [CPU0]   [19] 0x77a78000 - 0x77a79000 (Reserved)
[11098691835] [INFO] [kernel::memory] [CPU0]   [20] 0x77a79000 - 0x77efa000 (Other)
[11099235246] [INFO] [kernel::memory] [CPU0]   [21] 0x77efa000 - 0x77efb000 (Reserved)
[11099793540] [INFO] [kernel::memory] [CPU0]   [22] 0x77efb000 - 0x7801c000 (Other)
[11100335202] [INFO] [kernel::memory] [CPU0]   [23] 0x7801c000 - 0x7801d000 (Reserved)
[11100891747] [INFO] [kernel::memory] [CPU0]   [24] 0x7801d000 - 0x7881d000 (Other)
[11101429185] [INFO] [kernel::memory] [CPU0]   [25] 0x7881d000 - 0x7881e000 (Reserved)
[11102003781] [INFO] [kernel::memory] [CPU0]   [26] 0x7881e000 - 0x788df000 (Other)
[11102541153] [INFO] [kernel::memory] [CPU0]   [27] 0x788df000 - 0x788e0000 (Reserved)
[11103096477] [INFO] [kernel::memory] [CPU0]   [28] 0x788e0000 - 0x788ef000 (Other)
[11103633816] [INFO] [kernel::memory] [CPU0]   [29] 0x788ef000 - 0x788f0000 (Reserved)
[11104190592] [INFO] [kernel::memory] [CPU0]   [30] 0x788f0000 - 0x788f5000 (Other)
[11104726248] [INFO] [kernel::memory] [CPU0]   [31] 0x788f5000 - 0x788f6000 (Reserved)
[11105331171] [INFO] [kernel::memory] [CPU0]   [32] 0x788f6000 - 0x788fb000 (Other)
[11105871975] [INFO] [kernel::memory] [CPU0]   [33] 0x788fb000 - 0x788fc000 (Reserved)
[11106428025] [INFO] [kernel::memory] [CPU0]   [34] 0x788fc000 - 0x78928000 (Other)
[11106964638] [INFO] [kernel::memory] [CPU0]   [35] 0x78928000 - 0x7892a000 (Reserved)
[11107519830] [INFO] [kernel::memory] [CPU0]   [36] 0x7892a000 - 0x78933000 (Other)
[11108057202] [INFO] [kernel::memory] [CPU0]   [37] 0x78933000 - 0x78935000 (Reserved)
[11108628696] [INFO] [kernel::memory] [CPU0]   [38] 0x78935000 - 0x7893d000 (Other)
[11109169467] [INFO] [kernel::memory] [CPU0]   [39] 0x7893d000 - 0x7893e000 (Reserved)
[11109727233] [INFO] [kernel::memory] [CPU0]   [40] 0x7893e000 - 0x78948000 (Other)
[11110266255] [INFO] [kernel::memory] [CPU0]   [41] 0x78948000 - 0x78949000 (Reserved)
[11110823394] [INFO] [kernel::memory] [CPU0]   [42] 0x78949000 - 0x78956000 (Other)
[11111365122] [INFO] [kernel::memory] [CPU0]   [43] 0x78956000 - 0x78958000 (Reserved)
[11111946087] [INFO] [kernel::memory] [CPU0]   [44] 0x78958000 - 0x78966000 (Other)
[11112486330] [INFO] [kernel::memory] [CPU0]   [45] 0x78966000 - 0x78967000 (Reserved)
[11113054359] [INFO] [kernel::memory] [CPU0]   [46] 0x78967000 - 0x78973000 (Other)
[11113593909] [INFO] [kernel::memory] [CPU0]   [47] 0x78973000 - 0x78974000 (Reserved)
[11114151807] [INFO] [kernel::memory] [CPU0]   [48] 0x78974000 - 0x78978000 (Other)
[11114691390] [INFO] [kernel::memory] [CPU0]   [49] 0x78978000 - 0x78979000 (Reserved)
[11115265293] [INFO] [kernel::memory] [CPU0]   [50] 0x78979000 - 0x78989000 (Other)
[11115806097] [INFO] [kernel::memory] [CPU0]   [51] 0x78989000 - 0x7898a000 (Reserved)
[11116367262] [INFO] [kernel::memory] [CPU0]   [52] 0x7898a000 - 0x78a1a000 (Other)
[11116908033] [INFO] [kernel::memory] [CPU0]   [53] 0x78a1a000 - 0x78a1b000 (Reserved)
[11117485335] [INFO] [kernel::memory] [CPU0]   [54] 0x78a1b000 - 0x78a26000 (Other)
[11118048282] [INFO] [kernel::memory] [CPU0]   [55] 0x78a26000 - 0x78a27000 (Reserved)
[11118609183] [INFO] [kernel::memory] [CPU0]   [56] 0x78a27000 - 0x78a4c000 (Other)
[11119154244] [INFO] [kernel::memory] [CPU0]   [57] 0x78a4c000 - 0x78a4d000 (Reserved)
[11119720359] [INFO] [kernel::memory] [CPU0]   [58] 0x78a4d000 - 0x78a59000 (Other)
[11120259942] [INFO] [kernel::memory] [CPU0]   [59] 0x78a59000 - 0x78a5a000 (Reserved)
[11120851434] [INFO] [kernel::memory] [CPU0]   [60] 0x78a5a000 - 0x78a67000 (Other)
[11121448833] [INFO] [kernel::memory] [CPU0]   [61] 0x78a67000 - 0x78a68000 (Reserved)
[11122014354] [INFO] [kernel::memory] [CPU0]   [62] 0x78a68000 - 0x78ab0000 (Other)
[11122557897] [INFO] [kernel::memory] [CPU0]   [63] 0x78ab0000 - 0x78ab1000 (Reserved)
[11123304786] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11333272665] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485787 free frames
[11343434322] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11348070063] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11349310104] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11350143387] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11354149917] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11355816483] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11356878192] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11357555616] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11358222942] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11358902907] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11359880235] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11360832879] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11361515055] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11362200531] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11362869903] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11363538054] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11364525414] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11365570524] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11366288439] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11367938439] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11368883922] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11369849040] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11371308696] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11372762148] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11373512436] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11374077000] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11374825473] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[11688366261] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[11689363488] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[11692453146] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[11693306658] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[11694060873] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[11695543068] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[11707368354] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[11708622024] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[11709354492] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[11711207079] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[11711749665] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[11714020923] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[11720935149] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[11722624188] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[11735007933] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[11735546163] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[11750764740] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[11751318876] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[11753199645] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[11754380847] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[11755382232] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[11757472947] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[11758253001] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[11793006819] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62287700 ticks/sec), init_cnt=622877 for 100Hz
[11794376682] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[11795157066] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[11796245406] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[11801540916] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[11830830165] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[11831732682] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[11832992061] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[11834291337] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[11835432807] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[11838169464] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[11839439931] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[11859705462] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[11861767269] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[11862691995] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[11864429445] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[11865102645] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[11866377237] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[11868265332] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[11891813604] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[11893365165] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[11894836635] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[11896550160] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[11897409018] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[11898963417] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[11899711098] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[11900373144] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[11906839791] [INFO] [kernel::root] [CPU0] Spawning Root service...
[11907676011] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[11910296739] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[11911976835] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[11916939738] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[11918452260] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[11919125394] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[11920385796] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[11921254158] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[11922134433] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[11933472111] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[11936432805] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[11937483855] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[11938261434] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[11959100637] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11977222521] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11980055175] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11983779159] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11986323030] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11989241649] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11992291542] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11994798882] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[11995579992] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[11996619657] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12003256386] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12008383629] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12013227534] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12015434376] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12017715699] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12020008473] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12020796843] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12032094162] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12032857848] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12039095871] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12039994857] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12067744524] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12068742939] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[12391578210] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[12848321079] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[12879390018] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=501 journal=424 symbols=52 drops=0
[12911473080] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[14171499936] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=449 watches=0 history=964 journal=773 symbols=98 drops=0
[14905418814] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15000193956] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15001289325] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[15097945731] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[15155968839] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[15180780516] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[15181766457] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[15182464902] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[15185885187] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[15202374528] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[15219241125] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[15221277654] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[15273135471] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[15292055163] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[15292837098] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[15297183297] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[15328902501] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[15347849616] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[15352859379] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[15353809317] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[15421565874] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[15423155913] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[15513237399] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[15578610861] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[15590872605] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[15594655725] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[15596798547] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[15632663673] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[15665160984] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[15669975222] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[15670958226] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[15671749038] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[15672612978] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[15673278027] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[15674157609] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[15674869122] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[15675468864] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[15676135398] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[15676808631] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[15677430483] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[15678065238] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[15678718242] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[15679401144] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[15680134305] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[15680776650] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[15681430611] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[15682055301] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[15682700451] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[15683341080] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[15683949303] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[15684562971] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[15685183272] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[15685807863] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[15686476839] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[15687129480] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[15687759681] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[15688443210] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[15689066613] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[15689704437] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[15690392883] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[15691045194] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[15691681071] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[15692322921] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[15692945895] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[15693696414] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[15694437561] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[15695186397] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[15695921241] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[15696699777] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[15697446864] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[15698199099] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[15699757788] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[15701691621] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[15702632946] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[15710802525] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[15724141092] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[15728209497] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[15737147811] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[15737848137] [CONTRACT] [kernel] [CPU0] Spawning init process...
[15739992609] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[15743405997] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[15744408933] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [15751540002] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369013376 RFLAGS_BEFORE=130 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564572728
[15756299427] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[15774656238] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[15781860369] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[15783722988] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[15785450142] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[15794905830] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[15800019873] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[15803917272] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[15805119528] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[15809304951] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[15812647950] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[15813759159] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[15817999329] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[15819083346] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[15820240425] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[15821274381] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[15825078126] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[15826688658] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[15827979717] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[15829507584] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[15830953842] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[15835487811] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[15840347061] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[15879958974] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[15886712259] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[15892414395] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[15898996740] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[15902319378] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[15906843282] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[15912285246] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[15917914089] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[15923869962] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[15929579061] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[15935686998] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[15942232878] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[15948564159] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[15953967183] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[15959778681] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[15966345483] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[15972999537] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[15978762855] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[15984437997] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[15989945994] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[15996031953] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[16001897769] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[16008330657] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[16013390481] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[16019851518] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[16026012189] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[16031027826] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[16037242815] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[16042348014] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[16047066321] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[16050297384] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[16053904713] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[16059875799] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[16065700431] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[16070264298] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[16074768138] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[16082193369] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[16087369023] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[16093078980] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[16098577737] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[16103986107] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[16107218523] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[16128370500] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[16261261929] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[16267591989] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[16268355807] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16270066890] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16274484831] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[16275774471] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16284304707] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[16289229099] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16290668724] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16292326248] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369078912 RFLAGS_BEFORE=130 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564572760
[16296419007] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[16297348287] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16299390624] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16300269777] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[16304098437] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[16305863244] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16314376287] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[16318002459] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[16319355690] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[16321263123] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369144448 RFLAGS_BEFORE=130 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564572696
[16325662947] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[16330281297] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[16331877672] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[16334104974] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[16338459786] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:15:29 = 1775434529 unix_secs
[16340089293] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775434529, mono_ns=8169830757, offset=1775434520830169243ns
[16341676461] [INFO] [rtc_cmos] [CPU1] System clock anchored
[16355587545] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[16397101347] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[16404980262] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[16406268219] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16408816512] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16415747370] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[16418166402] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[16425804615] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[16431210741] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[16435300893] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16436904924] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564572728
[16441981182] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[16446447732] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[16448202540] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[16448948802] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16450552404] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16458596385] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[16462459695] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[16470277593] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[16473984252] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16474998012] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16476322269] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564572760
[16479357609] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[16482110073] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[16483356219] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[16488779736] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[16489889427] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[16490644467] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[16491445773] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[16492214046] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[16506957390] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[16508641512] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[16985163888] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[16990927404] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[16993926081] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[16996464375] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[17001762525] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[17004272769] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[17006916795] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[17009122977] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[17010747237] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[17013781026] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[17015949654] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[17018295822] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[17021165304] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[17022515763] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[17023305387] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[17024224767] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[17029183842] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[17030535126] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17033098500] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17039934252] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17042341569] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17049191082] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[17052802371] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[17053958592] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[17056588263] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369352688 RFLAGS_BEFORE=134 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564572696
[17062645875] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[17071681242] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[17086256352] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[17108684934] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[17109852309] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[17111732451] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[17112674568] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[17113469571] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[17115313149] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[17116134585] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[17116913385] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[17117791911] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[17118671427] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[17120936943] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[17126951919] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[17128937694] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[17129998677] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[17131315872] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[17133719724] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[17135868948] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[17137055595] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17139476079] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17143558773] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[17145111852] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[17146491945] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17160196449] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[17163921522] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[17166489153] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[17167858224] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17170337316] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17175453075] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17178531546] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17189830614] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[17194030986] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[17195343726] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[17196764937] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369500160 RFLAGS_BEFORE=134 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564572760
[17202880101] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[17203592406] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17205405525] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17206221615] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[17207811621] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[17211021861] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17211927414] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[17213381295] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[17214009087] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17215092807] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[17216959947] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[17218383138] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[17219777355] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[17221324065] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[17222408346] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[17224421478] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[17226102069] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[17227236114] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[17229127344] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[17230416588] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[17232491727] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[17234038734] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369433920 RFLAGS_BEFORE=130 CR3_BEFORE=60096512 fs_base=0 gs_base=18446744071564572728
[17238917784] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[17587214040] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[17589263967] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369582944 RFLAGS_BEFORE=134 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564572696
[17592519318] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17595153972] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[17596288479] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[17597400909] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[17598258513] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17599862346] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17600677479] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[17606619162] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[17609093700] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[17611762806] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[17616399174] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[17619561234] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[17621554731] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[17623419396] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[17624337984] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17625994353] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17645215368] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[17649413265] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[17669257320] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[17672395554] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[17673880092] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17675071986] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369718752 RFLAGS_BEFORE=130 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564572760
[17678610741] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[17680769964] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[17681533683] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[17682719043] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[17683598130] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[17685673599] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[17687440419] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369653216 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564572728
[17692631055] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[17694252873] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[17702905671] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[17703980283] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[17705053707] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[17706250584] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[17708061822] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[17711047860] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[17712612555] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[17714872131] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[17716483422] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[17718044586] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[17718835200] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17720564796] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17756616207] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[17758004946] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[17760206277] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[17762364048] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[17784425439] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[17807996151] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[17815775142] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[17819022936] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[17820831204] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17822212716] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369784288 RFLAGS_BEFORE=130 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564572696
[17825126715] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[17825843244] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17827348440] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17828129715] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[17830901418] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[17834510430] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[17835891249] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[17868466176] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[17873904609] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[17880763230] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[17883669309] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[17885547372] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[17886248325] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17887708212] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17892928746] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17895190797] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17902842309] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[17907174549] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[17908429242] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17910081255] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369915360 RFLAGS_BEFORE=130 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564572760
[17913374919] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[17914086696] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17915214966] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[17915903511] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17916626574] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[17923235550] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17927442621] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[17934062718] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[17937077070] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[17939191149] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17940867681] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369981792 RFLAGS_BEFORE=134 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564572696
[17945474448] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[17946256482] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17947659642] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17957153643] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17961196539] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[17968121787] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[17970900123] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[17973333213] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[17974039974] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17975463627] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17999395755] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18003850194] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[18011181474] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[18014062968] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cac8
[18016002411] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18017751345] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370114032 RFLAGS_BEFORE=134 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564572760
[18020683527] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[18021598518] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18022979766] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[18024031608] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18041051655] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[18042134220] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[18044149596] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18046270770] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18047865000] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18049453125] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18051206118] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[18052350855] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[18055273137] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[18057100677] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18058486050] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370198160 RFLAGS_BEFORE=134 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564572696
[18062669625] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[18063680877] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18065066580] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[18066072354] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18066976983] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[18075530484] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18076701225] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18077951430] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18079311096] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[18083109825] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18084182160] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18085448238] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18086810346] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[18102177027] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[18111457848] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[18113648157] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18114964758] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18116438010] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18117855756] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[18120146253] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[18123556572] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[18125493540] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18126594750] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18127781496] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18129099120] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[18131772483] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18133233327] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369849824 RFLAGS_BEFORE=130 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564572728
[18138234873] [INFO] [nectar] [CPU2] NECTAR: Started.
[18138773895] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18139908567] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[18140925264] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18142012218] [INFO] [fontd] [CPU3] FONTD: Service ready
[18142555068] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18143852562] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[18145288260] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18147165102] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370048048 RFLAGS_BEFORE=134 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564572728
[18151525392] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[18153287724] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18154791732] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370267200 RFLAGS_BEFORE=130 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564572728
[18159438858] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[18160963029] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[18162696651] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[18166800036] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18168205935] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18178865760] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18179869224] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18181046895] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18182282481] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=235 subj_lo=0
[18184830081] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[18192973689] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18193988835] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18195216468] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18196253988] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[18201976947] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18202971006] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18203959059] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18205109142] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[18210772173] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18211926183] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18212993997] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18214046697] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[18219468663] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[18220911720] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[18222549939] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[18223881555] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[18226158456] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[18227453904] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[18229553232] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x44f7000
[18230859405] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[18232871019] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[18234921210] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[18236435118] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[18242729406] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[18245499723] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[18253041345] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[18257233236] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[18258195021] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[18261324147] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[18263728560] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[18265731099] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[18267222369] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[18268670409] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[18270106503] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[18271643511] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[18272872497] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[18273876456] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[18278940405] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[18284009271] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[18290991180] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[18294909534] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18296217984] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18306546720] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[18317236971] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1258 backend=VirtIO-GPU
[18319247793] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[18320031081] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18321706425] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18330473106] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18331314276] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18332490561] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18392573529] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18421528125] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18442419600] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18443705775] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18449935911] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18451815459] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[18465693081] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[18473497086] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[18476732175] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[18478677624] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[18479481702] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18481031052] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18481861629] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f1098
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4ea
[18483542022] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370511040 RFLAGS_BEFORE=134 CR3_BEFORE=72433664 fs_base=0 gs_base=18446744071564572760
[18486452490] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18487828557] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18489290028] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[18494667279] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[18497311701] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[18498727929] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1098
[18499680606] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[18501305493] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370576576 RFLAGS_BEFORE=134 CR3_BEFORE=73482240 fs_base=0 gs_base=18446744071564572696
[18506186886] [INFO] [echo] [CPU1] echo: starting up
[18506924766] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[18508170615] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[18513658944] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[18516183543] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18540912720] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[18559558743] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[18561262698] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[18568753566] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[18579501435] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18580639968] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18583411308] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[18585945576] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[18590025267] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[18592514028] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[18594237024] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[18594870294] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[18596477625] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[18597949194] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[18598807161] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18600402645] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18601741620] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[18605273115] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18607063299] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18612719664] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[18614328843] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[18615438996] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[18617486316] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[18618671907] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[18621571023] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[18623205414] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[18624851322] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18625653849] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18627381234] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18631342125] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[18634729179] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18638472501] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18639417918] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[18645814803] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[18646989570] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[18648327984] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[18649409460] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db598
[18650359992] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18651849282] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370740416 RFLAGS_BEFORE=134 CR3_BEFORE=73986048 fs_base=0 gs_base=18446744071564572760
[18654952107] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[18655704342] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18657365859] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18660228246] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[18661402353] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18667997073] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[18670633212] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[18671790423] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[18674148372] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db598
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18675588591] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370805952 RFLAGS_BEFORE=134 CR3_BEFORE=74133504 fs_base=0 gs_base=18446744071564572696
[18680212815] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[18687077046] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[18690144000] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[18695173398] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[18696826302] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[18702685551] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[18718576206] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[18729131784] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[18729937083] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[18734056341] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[18739896945] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[18741552687] [INFO] [anther] [CPU1] anther: Connected to network stack
[18745902846] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[18757894089] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[18760506930] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[18773208828] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[18781512717] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[18786221520] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[18803355054] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[18805662216] [INFO] [bloom] [CPU3] bloom: creating surface...
[18806684160] [INFO] [bloom] [CPU3] bloom: surface created!
[18807551796] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[18811062666] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[18814293861] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1264
[18815400681] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[18827147526] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[18830289291] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[18831783927] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[18832934076] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [18836103429] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[18843500445] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[18852041010] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[18860243226] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
T:07D0 T:0640 [18867767688] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[18873722637] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[18875337690] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[18878078934] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
T:F0B0 [18887175582] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[18888306855] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1264
[18889081563] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[18891654012] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[18904239288] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[18908168103] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[18909802197] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[18912706593] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[18922194588] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[18923615238] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[18926461092] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[18929398356] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
T:1220 [18943117446] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[18945508494] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[18953321706] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[18960676878] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[18962956254] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[18976794408] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=281
[18978200307] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[18979045833] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[18979941189] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18981017187] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18982332468] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=281 subj_lo=0
[18986233167] [INFO] [netd] [CPU3] NETD: DHCP configured �� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[18989236332] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[18994585335] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[18997600347] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[19000241997] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[19002456759] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[19006326471] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[19020233562] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[19022724006] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[19027037568] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[19027754097] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1272)
[19029295197] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[19040224302] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=244
[19041515658] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[19042506714] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19043605449] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19044582447] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19045777740] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[19056886728] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1279)
[19058154621] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[19067715843] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=285
[19068808902] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[19070045643] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19071157512] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19072217637] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19073405076] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=285 subj_lo=0
[19083798162] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1282)
[19085586465] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[19176462789] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[19310143215] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[19312673523] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[19315158654] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[19320220920] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[19325646615] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[19327686906] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[19329781647] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=74
[19331455275] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=25
[19333949118] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[19342133448] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[19346515848] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[19349223300] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[19350828288] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=650 watches=13 history=1024 journal=1024 symbols=306 drops=0
[19355819307] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[19362772407] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[19363968096] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[19366307037] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[19371213972] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=124
[19372700325] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 114 byte frame (118 encoded) to netd rx_port=25
[19374251061] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (118 bytes sent)
[19376602113] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[19445234853] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[19457555766] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[19468603440] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[19493182896] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[19617048264] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[19759650174] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[19912028301] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[20140587786] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[20313197487] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[20450668788] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[20479936785] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[20660058111] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[20866293822] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[20926147572] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[20931457239] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[20932591911] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[20933514426] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[20934560064] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[20935926165] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[20944774323] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[20945702151] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[20946952455] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[20948383962] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=338 pred=0 subj_lo=0
[21037603356] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[21076322355] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=710 watches=15 history=1024 journal=1024 symbols=339 drops=0
[21245917308] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[21406565598] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[21424839117] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[21426407838] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[21434761821] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[21436125810] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:43018 on listener 1
[21454957656] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21516924000] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21563209371] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[21564396645] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21565554549] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21566837919] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=341 pred=0 subj_lo=0
[21586793019] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21657479679] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[21722419983] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21781503876] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[21829346451] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21838106532] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[21929933124] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=9d85f11f1afc2373)
[21963366777] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[21997593420] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22029349683] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[22042460484] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12227000
[22044092136] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[22045403754] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[22109832756] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[22119309036] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[22123840200] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[22126244976] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[22128542898] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[22132939851] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[22137647895] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[22152404043] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[22154234091] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[22156316028] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[22159300845] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[22161193296] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[22164540123] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[22167147255] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=1 (16384b)
[22198904541] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[22209149622] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x12233000
[22211041545] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[22442508561] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=14aa2015f9906e3f)
[22609028838] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[22610284686] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[22612077378] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22675028211] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[22882786410] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[23010040614] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[23024721621] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23045240196] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=790 watches=16 history=1024 journal=1024 symbols=375 drops=0
[23046980088] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[23055840093] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=3
[23064914334] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 37 (user thread) assigned to CPU 1
[23067139656] [INFO] [anther] [CPU1] anther: Thread spawned TID=37 for conn_handle=3
T:5EF0 [23082684768] [INFO] [anther] [CPU1] anther: Worker thread TID=37 starting for conn_handle=3
[23096189457] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23117770632] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=33, our_read=34)
[23119881642] [INFO] [anther] [CPU1] anther: Worker TID=37 connected to netd OK
[23168591424] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[23219950281] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[23226937767] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=124
[23228610768] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 114 byte frame (118 encoded) to netd rx_port=25
[23232225027] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 114 bytes
[23233112793] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (118 bytes sent)
[23247153963] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23252543523] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[23255802471] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[23270425860] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23271554922] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23272658376] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23273820966] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=342 pred=0 subj_lo=0
[23303937030] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 60 bytes on conn_handle=3
[23311357245] [INFO] [anther] [CPU1] anther: GET /health Http11
[23387403237] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[23394138207] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[23398822656] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[23400281421] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[23402767839] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23404120410] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23409197658] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[23411503764] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[23422200417] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23423060265] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[23424556782] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[23428826091] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23438979696] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23440841160] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[23442324378] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[23445232371] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23532904725] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[23542102848] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23551631895] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 0 bytes on conn_handle=3
[23606583429] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23641315137] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23642487033] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23643656487] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23644876002] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=343 pred=0 subj_lo=0
[23680880751] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23681969124] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23683097889] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23684384592] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=345 pred=0 subj_lo=0
[23696554464] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23763353592] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23870108526] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23944436571] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[23963936865] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24040406214] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24122334060] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=1
[24304853925] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24312264867] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[24426339399] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24429373122] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[24431888118] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[24509704032] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24607794618] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24690178161] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24746672445] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[24774676245] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24872002221] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24961729254] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25036569393] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25123974678] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25177866021] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[25262389944] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25627918074] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[25670886252] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25906122495] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[25908184236] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[25911195882] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[25912304055] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[25915120704] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[25920120897] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[25924922859] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[25926579327] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[25929500388] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[25932205365] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=124
[25933016538] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[25934097816] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 114 byte frame (118 encoded) to netd rx_port=25
[25936657296] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (118 bytes sent)
[25937667789] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[25940669007] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:43034 on listener 1
[26063201307] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26083787103] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[26492894571] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26522245596] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[26560769697] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[26576957088] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 114 bytes
[26581935798] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=4
[26590181772] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 38 (user thread) assigned to CPU 1
[26592085575] [INFO] [anther] [CPU1] anther: Thread spawned TID=38 for conn_handle=4
[26601991548] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[26603920761] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
T:5EF0 [26635009698] [INFO] [anther] [CPU1] anther: Worker thread TID=38 starting for conn_handle=4
[26755255758] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=35, our_read=36)
[26757626577] [INFO] [anther] [CPU1] anther: Worker TID=38 connected to netd OK
[26769415794] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[26772072855] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[26813319522] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 60 bytes on conn_handle=4
[26816320608] [INFO] [anther] [CPU1] anther: GET /health Http11
[26828231199] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[26830612314] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[26838134466] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[26839886271] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[26842665003] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26845662030] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26854668126] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[26856366009] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[26859098145] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26860271658] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26862531135] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[26865681447] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[26880965397] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[26883692088] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[26886137058] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26889711288] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26892283044] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[26894813649] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[26897443617] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[26899161003] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=70
[26901790146] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[26904911913] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26906952501] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[26911104330] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26914459638] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=124
[26916144684] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 114 byte frame (118 encoded) to netd rx_port=25
[26919518472] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (118 bytes sent)
[26922373104] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 114 bytes
[26933676495] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=70
[26936063814] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[26939772981] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26943650085] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26945646552] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[26949959586] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 0 bytes on conn_handle=4
[26951612193] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[27023302680] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[27025225788] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:43036 on listener 1
[27030464934] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27036456645] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[27303419286] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2429 ops=1 watches=19
[27381308526] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=941 watches=19 history=1024 journal=1024 symbols=386 drops=0
[27453204174] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27482684592] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[27569997642] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[27576087363] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=5
[27584863119] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 39 (user thread) assigned to CPU 1
[27586692441] [INFO] [anther] [CPU1] anther: Thread spawned TID=39 for conn_handle=5
T:5EF0 [27673423800] [INFO] [anther] [CPU1] anther: Worker thread TID=39 starting for conn_handle=5
[27777440757] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=37, our_read=38)
[27779376768] [INFO] [anther] [CPU1] anther: Worker TID=39 connected to netd OK
[27791593731] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[27793703058] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[27857570037] [INFO] [anther] [CPU1] anther: Worker TID=39 got first 60 bytes on conn_handle=5
[27859371507] [INFO] [anther] [CPU1] anther: GET /health Http11
[27873711360] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[27878176062] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[27885171864] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=70
[27887090583] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[27889745598] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27890862714] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[27955926471] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[28492034703] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[28506648984] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=70
[28508537475] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[28511352540] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28518800013] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28521408399] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=70
[28523516571] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[28526595867] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28535886423] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28538065215] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[28540327827] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[28559826372] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=70
[28561779873] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[28564430928] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28575448407] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28578373263] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[28580897565] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[28581776091] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[28587775755] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=70
[28590998667] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[28593667641] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28600577742] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28602602325] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=25 len=123
[28604303442] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 113 byte frame (117 encoded) to netd rx_port=25
[28605470223] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28606959876] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (117 bytes sent)
[28608132861] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 113 bytes
[28630316847] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[28632976647] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[28643140746] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[28713470082] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[28715283993] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:43046 on listener 1
[28716876276] [INFO] [anther] [CPU1] anther: Worker TID=39 got first 0 bytes on conn_handle=5
[29213855319] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[29222196036] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29419601475] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[29426720730] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=6
[29435688612] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 40 (user thread) assigned to CPU 1
[29437927200] [INFO] [anther] [CPU1] anther: Thread spawned TID=40 for conn_handle=6
T:5EF0 [29511533073] [INFO] [anther] [CPU1] anther: Worker thread TID=40 starting for conn_handle=6
[29675402427] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=39, our_read=40)
[29677139514] [INFO] [anther] [CPU1] anther: Worker TID=40 connected to netd OK
[29689816695] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[29732101641] [INFO] [anther] [CPU1] anther: Worker TID=40 got first 59 bytes on conn_handle=6
[29734001187] [INFO] [anther] [CPU1] anther: GET /graph Http11
[29746160499] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 209 bytes - TCP ACK
[29923562823] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[29998799193] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30035810607] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[30664722198] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[30671781756] [INFO] [fontd] [CPU3] FONTD: Auto-importing font asset ThingId([160, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[30983921859] [INFO] [fontd] [CPU3] FONTD: Auto-imported font 'Noto Sans' style 'Regular' from asset ThingId([160, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[30987721380] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (1 fonts)
[31074696972] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[31222123614] [INFO] [fontd] [CPU3] F
```
</details>
