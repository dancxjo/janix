# ❌ Scenario: Checking System Resources for Heavy Tasks

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ❌ | 1002ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11372492901] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11378319447] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11381956641] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11383921692] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11385124542] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11385758274] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11386425534] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11387006004] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11387592645] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11388203244] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11388815592] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11389525554] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11390355372] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11391152982] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11392019067] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11392709295] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11393526342] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11394186639] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11394810207] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11395456710] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11396036916] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11396622765] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11397220626] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11397940950] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11398607781] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11399256495] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11399901348] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11400552933] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11401145217] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11401782249] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11402446341] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11403067698] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11403968598] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11404635099] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11405275596] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11405974767] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11406680472] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11407396143] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11408124684] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11408862564] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11409576783] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11410296381] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11411721651] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11413278030] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11414049999] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11414712408] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11415316803] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11415928986] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11416572585] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11417179026] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11417759628] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11418313830] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11418832755] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11419394382] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11419948188] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11420516547] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11421063918] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11421658677] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11422208721] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11422776783] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11423325144] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11423892315] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11424440643] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11425021674] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11425571355] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11426137734] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11426703486] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11427331014] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11427933132] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11428548648] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11429103906] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11429677083] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11430391302] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11431327974] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11432138322] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11432991207] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11433761559] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11434388229] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11435060208] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11435671731] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11436296487] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11436934773] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11437559100] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11438223357] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11438817027] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11439386640] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11439938103] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11440509267] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11441088318] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11441660571] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11442216258] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11442785904] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11443338324] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11443909290] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11444479695] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11445089238] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11445642582] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11446214670] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11446775373] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11447368548] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11447939646] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11448515727] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11449066926] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11449637859] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11450191665] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11450764644] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11451345345] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11452239975] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11706675585] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[11718586605] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11723565942] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11724831657] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11725692693] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11730610617] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11733020409] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11734418421] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11735138811] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11735938005] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11736823560] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11737919325] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11738904969] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11739600114] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11740285656] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11741012844] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11741693634] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11742952287] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11743998618] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11744900574] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11746510710] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11747605617] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11748656040] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11750357586] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11752142325] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11753018739] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11753625114] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11754482553] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12134159631] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12135381885] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12138994659] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12139943673] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12140786889] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12142316505] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12154703781] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12156042987] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12156894915] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12158831619] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12159489276] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12161909793] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12169778247] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12171551172] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12185433876] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12186025731] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12202807386] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12203572821] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12205857279] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12207258030] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12208376070] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12210751971] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12211560801] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12246768402] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62496900 ticks/sec), init_cnt=624969 for 100Hz
[12248649204] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12249571521] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12250884888] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12257255670] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12287361834] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12288370776] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12289258014] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12290521089] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12291822873] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12295674039] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12297757758] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12319033056] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12321344244] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12322291410] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12323853036] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12324523431] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12326340378] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12327541908] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12353738661] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12354619134] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12355907553] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12356663616] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12358467594] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12359235042] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12361151121] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12361817325] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12367932489] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12368902260] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12370738446] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12371567868] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12375503217] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12377088834] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12377788962] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12379092264] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12380015373] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12380851560] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12393298500] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12396226194] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12397333773] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12398087196] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12419767701] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12439255983] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12441794409] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12445514004] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12447494466] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12449936433] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12452591613] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12454710444] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12455535081] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12456544881] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12463120329] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12464745447] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12467182398] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12469571862] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12471914367] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12475657392] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12476488992] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12489851352] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12490567980] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12499083630] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12499856820] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12527164551] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12528104787] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[12887857719] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13378214619] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13404765660] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[13441133046] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[14674552233] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=446 watches=0 history=959 journal=768 symbols=93 drops=0
[15367223685] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15455167464] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15456191883] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[15551828358] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[15606985284] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[15633656478] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[15634403268] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[15635030499] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[15638263179] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[15652627155] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[15667969746] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[15670203582] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[15718799976] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[15736669542] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[15737522691] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[15741321123] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[15764299518] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[15782203536] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[15785834328] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[15786778689] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[15847525881] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[15849593562] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[15935925489] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[15999938757] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[16013816478] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[16018468752] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[16020684669] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[16024536825] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[16084960287] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[16089486567] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[16090442049] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[16091252496] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[16092139470] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[16092797391] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[16093429770] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[16094062380] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[16094676081] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[16095319317] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[16095947043] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[16096562724] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[16097200878] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[16097842200] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[16098530250] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[16099309974] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[16100032146] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[16100874933] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[16101871599] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[16102535790] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[16103151339] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[16103750586] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[16104357885] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[16104972147] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[16105599873] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[16106259411] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[16107145956] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[16107871494] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[16108557663] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[16109492982] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[16110409854] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[16111249836] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[16111925148] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[16112553039] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[16113182943] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[16113794037] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[16114523865] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[16115269137] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[16116003024] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[16116731433] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[16117484262] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[16118234088] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[16118979987] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[16120490298] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[16122452082] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[16123308135] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16131037890] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[16145153442] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[16149498354] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[16158329682] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[16159019514] [CONTRACT] [kernel] [CPU0] Spawning init process...
[16161143064] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[16163819265] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[16164563052] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
ThingOS PetalsUSER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000

type 'help' for commands

petals> [16171378707] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[16174808100] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[16194625656] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[16196566353] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[16197721683] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[16198807812] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[16209049395] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[16213976823] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[16217145681] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[16218227091] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[16221565668] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[16224273153] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[16225302753] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[16228763430] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[16229826822] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[16230950208] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[16231937766] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16235219517] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16236325545] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16237464606] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16239035736] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16240411275] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16241788134] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16245895578] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[16285135185] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[16293168144] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[16298284134] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[16302910569] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[16305941190] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[16309864263] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[16314376848] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[16318567122] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[16322714001] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[16327667598] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[16332581694] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[16337516217] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[16342811496] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[16347191520] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[16351408458] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[16356195735] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[16361250378] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[16365992247] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[16370314521] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[16375222017] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[16379727903] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[16383958470] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[16388786139] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[16393509198] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[16398246975] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[16402764675] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[16407616665] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[16411949862] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[16416276261] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[16420721856] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[16424045352] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[16427030103] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[16431333204] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[16435498035] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[16440521724] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[16445164725] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[16449750174] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[16454640543] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[16460192397] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[16465330101] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[16470586176] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[16473680454] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[16492472106] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[16628316870] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[16635101043] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[16636001019] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16637920926] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16642420311] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[16643656161] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16651596126] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[16656401751] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16657628295] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16659403695] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[16665658944] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[16666562781] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16668017982] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[16669507800] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16673874162] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[16675663884] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16682788188] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[16687722678] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[16689368124] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[16690404060] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[16691885727] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[16697381877] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[16698941721] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[16701062103] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[16706385762] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:54:09 = 1775436849 unix_secs
[16708050018] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775436849, mono_ns=8353785346, offset=1775436840646214654ns
[16709592471] [INFO] [rtc_cmos] [CPU1] System clock anchored
[16720366410] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[16757576847] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[16767397515] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[16768243866] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16769989368] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16775764203] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[16778117202] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[16785597048] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[16788148344] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[16791396963] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16793139825] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[16798631685] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[16803097839] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[16804936632] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[16805782125] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16807635207] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16814757003] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[16817089278] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[16823641329] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[16826055939] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16827059799] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16828472067] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[16831958352] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[16833741012] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[16840008999] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[16840784367] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[16841517396] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[16842726087] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[16844755455] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[16846212273] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[16858541073] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[16860052704] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[17344317309] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17348054724] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[17349062313] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17350835007] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17358124905] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17360780085] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17367832416] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[17370680877] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[17372521650] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[17374345065] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352688 RFLAGS_BEFORE=134 CR3_BEFORE=59662336 fs_base=0 gs_base=18446744071564586576
[17377791651] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[17380505967] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[17381701425] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[17382810093] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[17387220345] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[17388959016] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[17390594067] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[17391871497] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[17393325510] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[17394783516] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[17397315408] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[17398294485] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[17399217000] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[17400106746] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[17400901023] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[17401797303] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[17402700777] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[17405988699] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[17407953057] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[17412927609] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[17420964759] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[17422298190] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[17490462528] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[17493239841] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[17495324352] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[17496886143] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[17497577691] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17499218220] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17502819444] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[17503524225] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[17504612697] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17518449465] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[17520521337] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[17522392536] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[17523183348] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17524657656] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17529253797] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17530529346] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17537473140] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[17539844784] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[17541045588] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[17542690770] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500048 RFLAGS_BEFORE=134 CR3_BEFORE=68235264 fs_base=0 gs_base=18446744071564586640
[17546570943] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[17547459402] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17549183718] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17550656607] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[17552270208] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[17554471506] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17557392930] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17559301386] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[17561393751] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[17563726983] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[17565596400] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[17566831524] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[17569727868] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[17570721795] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[17571803667] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[17573210721] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[17574250287] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[17575119903] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[17575843065] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[17577128217] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[17577853887] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1236 port=2 model='                                        ' rpc_port=5
[17602671273] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[17603490828] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[17604271641] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[17605002030] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[17605720539] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[17606507886] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[17607327705] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[17608288104] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[17613727956] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[17615110491] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369433968 RFLAGS_BEFORE=130 CR3_BEFORE=59830272 fs_base=0 gs_base=18446744071564586608
[17619782433] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[17945917242] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[17947588263] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369582832 RFLAGS_BEFORE=134 CR3_BEFORE=68349952 fs_base=0 gs_base=18446744071564586576
[17950494408] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17952233475] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[17953200738] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[17954007291] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17955330492] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[17956138134] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17958927129] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[17962520136] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[17964905277] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[17970784953] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[17972908008] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[17975598894] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[17978151840] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[17979753495] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[17980671291] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17982494640] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18001732485] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18006641400] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[18027631149] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[18030526998] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[18031564287] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18032986653] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369718960 RFLAGS_BEFORE=130 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[18036238836] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[18038309553] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[18039133926] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[18040289421] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[18041174415] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[18043135176] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[18045396402] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653424 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[18050829489] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[18052611885] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[18069883821] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[18071998824] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[18072992949] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[18074677929] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[18077200548] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[18080279415] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[18081943935] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[18098748558] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[18106855107] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[18108733170] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[18111243843] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[18112756959] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[18115298619] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[18117458667] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[18119849286] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4256000
[18121418370] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[18123442854] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[18125571585] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[18126950457] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[18136068192] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[18144036273] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[18153135066] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[18155555517] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[18157653129] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[18159306132] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[18160708731] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[18161930820] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[18163048365] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[18163983090] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[18165048099] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[18170322423] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=15, read=16)
[18177312384] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=17, read=18)
[18181267467] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[18183574332] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[18185163480] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[18185981616] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18187938021] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18193870431] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18254687385] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[18277913016] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[18285004617] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[18287776716] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0102438
[18290212248] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18291890364] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915568 RFLAGS_BEFORE=130 CR3_BEFORE=69668864 fs_base=0 gs_base=18446744071564586576
[18296633058] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[18297313287] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18298846005] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18300480363] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[18303413073] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[18307077360] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18308377560] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18322420215] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18325450143] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[18332079381] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[18334460199] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[18336314964] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[18337137357] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18338764884] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18344386698] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18346946937] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18350539119] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[18352150476] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[18353855487] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18355010256] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[18356080875] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[18358425690] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
[18360420738] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[18361651275] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18362309757] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18363948900] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18364681830] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370046768 RFLAGS_BEFORE=134 CR3_BEFORE=70602752 fs_base=0 gs_base=18446744071564586640
[18369237843] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[18370678524] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[18371572593] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18374796891] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18376346901] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18383748537] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[18386218158] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[18387334581] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18388735629] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113328 RFLAGS_BEFORE=134 CR3_BEFORE=70729728 fs_base=0 gs_base=18446744071564586576
[18392066583] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[18392894850] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18394549107] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18403593186] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18406704063] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[18413515164] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[18416487210] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[18418493874] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[18419154996] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18420464832] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18443703564] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18448418274] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[18455426220] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[18457918941] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010d880
[18459603690] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18460872903] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370245808 RFLAGS_BEFORE=130 CR3_BEFORE=71032832 fs_base=0 gs_base=18446744071564586640
[18464094693] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[18464789574] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18465968334] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[18466855374] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18481809225] [INFO] [fontd] [CPU3] FONTD: Service node created, req=19, resp=22
[18483636006] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[18485934852] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18487828887] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18488531721] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18490643952] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18493186503] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[18497774790] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[18500688888] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[18502693275] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[18503521773] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18505388814] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18508809891] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1108
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18510414582] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370330816 RFLAGS_BEFORE=134 CR3_BEFORE=71307264 fs_base=0 gs_base=18446744071564586576
[18515511168] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[18516257661] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18518158758] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[18520838985] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18522422556] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18524177562] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18525265902] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18526532310] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18528407007] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18529770138] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=229 pred=0 subj_lo=0
[18531865044] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[18544317132] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[18554233830] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[18560902305] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[18563164191] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[18566313744] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18567414690] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18568711557] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18569866821] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18571303674] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18572795472] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18573823323] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[18575243907] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18576586776] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[18578258985] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18579844173] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981104 RFLAGS_BEFORE=130 CR3_BEFORE=70340608 fs_base=0 gs_base=18446744071564586608
[18584286006] [INFO] [nectar] [CPU2] NECTAR: Started.
[18586280691] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
[18587263992] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18589615308] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370179728 RFLAGS_BEFORE=134 CR3_BEFORE=70877184 fs_base=0 gs_base=18446744071564586608
[18592684836] [INFO] [fontd] [CPU3] FONTD: Service ready
[18594180198] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[18595726644] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1108
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18597100764] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370398432 RFLAGS_BEFORE=130 CR3_BEFORE=71516160 fs_base=0 gs_base=18446744071564586608
[18601728057] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[18603431418] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18604416237] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[18605320239] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18606393069] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[18607205166] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18608424681] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=235 subj_lo=0
[18617866179] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18623715627] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[18639151179] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[18645655512] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=15, RX port=18
[18650158989] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18651389724] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18653002566] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18653926566] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18654937455] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18655967385] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=244 subj_lo=0
[18661261377] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18662183001] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18663184485] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18664241640] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=246 subj_lo=0
[18669003771] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18670005816] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18671120589] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18672136065] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=247 subj_lo=0
[18679972344] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18681215586] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18682212615] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18683484402] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=248 pred=0 subj_lo=0
[18687711801] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1253 backend=VirtIO-GPU
[18690429549] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[18691271643] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18692846667] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18694844421] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18717402099] [INFO] [netd] [CPU3] NETD: Driver TX port=15, RX port=18, link_up=true, mtu=1500
[18719689494] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[18723923031] [INFO] [netd] [CPU3] NETD: Created socket API port (write=23, read=24)
[18724889799] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[18744788073] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=25, resp=28
[18746115927] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[18754501953] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[18768231438] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[18769314663] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[18774368712] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[18800714064] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[18825597483] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=29, our_read=30)
[18827249100] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[18828017769] [INFO] [anther] [CPU1] anther: Connected to network stack
[18841021386] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[18848391837] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[18850728765] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f1108
[18852622668] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e5
[18854459184] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370554496 RFLAGS_BEFORE=130 CR3_BEFORE=72433664 fs_base=0 gs_base=18446744071564586640
[18857346255] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[18858032127] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18859535310] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18860269065] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[18863713968] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18865168608] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18872388315] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[18875167542] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[18876866778] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[18880374678] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[18881468166] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1108
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[18887501853] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[18888462483] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[18891243690] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370620032 RFLAGS_BEFORE=130 CR3_BEFORE=73920512 fs_base=0 gs_base=18446744071564586576
[18897170391] [INFO] [echo] [CPU1] echo: starting up
[18898989318] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[18915735663] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[18916923762] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[18918844956] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[18919845813] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[18925010247] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[18926083737] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[18927696645] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[18928562499] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18930208539] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18931988559] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[18934923810] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18936811872] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18938449959] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[18944439294] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[18947041047] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[18949198818] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[18950866572] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[18952451694] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18953303655] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18954788259] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18956239467] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[18957694140] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[18961860753] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18964139007] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[18965388354] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18973641291] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[18976816947] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[18978262743] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[18979665771] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db5e0
[18980747082] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[18981599604] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18983382561] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18984241188] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370756544 RFLAGS_BEFORE=134 CR3_BEFORE=74780672 fs_base=0 gs_base=18446744071564586640
[18987476112] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[18988662099] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18995583123] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[18997161282] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[18998154714] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[18999041325] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[18999997500] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[19001089041] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[19007546448] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[19008518166] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db5e0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19010389134] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370822080 RFLAGS_BEFORE=134 CR3_BEFORE=74928128 fs_base=0 gs_base=18446744071564586576
[19014695238] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[19016467569] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[19017833109] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[19019731962] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19028499897] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[19029671760] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[19034473524] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[19038369801] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19042783782] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[19044908091] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[19051716717] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[19077236739] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[19080788826] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[19082537925] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[19089478980] [INFO] [netd] [CPU3] NETD: DHCP configured ��� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[19092247152] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[19097063997] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[19099435113] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[19105440717] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[19113023061] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[19134867675] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[19140125433] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[19142578587] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[19162885599] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[19164384393] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[19165540713] [INFO] [bloom] [CPU3] bloom: creating surface...
[19166489760] [INFO] [bloom] [CPU3] bloom: surface created!
[19167303012] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[19169392869] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[19175407284] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1264
[19180113546] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[19182763413] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[19184673090] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[19186229733] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1264
[19187188317] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[19198922952] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[19201618623] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[19202872788] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[19204311159] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [19211896935] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[19219867887] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[19228835307] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[19236303966] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
T:07D0 T:0640 T:F0B0 [19263161346] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[19264965489] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[19267722969] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[19276850868] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[19283989329] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[19287765453] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[19294755645] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=74
[19295867778] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[19297367100] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=17
[19298482632] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[19300100886] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[19310965245] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[19315014873] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[19319922732] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[19320949164] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 5)
[19323450102] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[19326877581] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[19328345058] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[19329598167] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[19331789928] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[19332805008] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[19335228297] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[19337171106] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=149
[19338032967] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[19340369994] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[19342683657] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[19346978376] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=285
[19348695168] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[19349744964] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19350732159] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19351743015] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19352926758] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=285 subj_lo=0
T:1220 [19359240450] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[19361108448] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[19366415343] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[19370118174] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[19371832656] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:55994 on listener 1
[19378219872] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[19381595508] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[19387443471] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1276)
[19388462214] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[19391058357] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=3
[19399396797] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 37 (user thread) assigned to CPU 1
[19401307695] [INFO] [anther] [CPU1] anther: Thread spawned TID=37 for conn_handle=3
[19416398628] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=248
[19418315202] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[19419543033] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19420859733] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19422251574] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19423670904] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=248 pred=0 subj_lo=0
[19441297590] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1284)
[19444122654] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[19457387499] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=290
[19458647010] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[19459937409] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19460866161] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19461867744] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19462975290] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=290 subj_lo=0
[19475573139] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1287)
T:5EF0 [19478214624] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[19479413283] [INFO] [anther] [CPU1] anther: Worker thread TID=37 starting for conn_handle=3
[19584285699] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=33, our_read=34)
[19586355129] [INFO] [anther] [CPU1] anther: Worker TID=37 connected to netd OK
[19698365247] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[19892118048] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[19919230056] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[19935906507] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[20066001450] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=665 watches=13 history=1024 journal=1024 symbols=314 drops=0
[20075681967] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[20216470428] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[20350834185] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[20566346823] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[20743872600] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[20912415249] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[20959386921] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[21098805585] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[21292318377] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[21365673747] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[21371639586] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[21373229559] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[21374173293] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[21375201936] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[21376641561] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[21385585089] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[21386608452] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[21387707847] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[21389215386] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=338 pred=0 subj_lo=0
[21505735350] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[21678733263] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=711 watches=15 history=1024 journal=1024 symbols=339 drops=0
[21700956684] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[21852332040] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[22041980664] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 2 bytes on conn_handle=3
[22048192650] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[22049880171] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[22115567166] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[22195665822] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22196731326] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22197770298] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22198873521] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=313 pred=0 subj_lo=0
[22202608329] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22282346361] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22358378097] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[22377014517] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=fec1d29ef0678173)
[22419456708] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[22487595999] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[22492428882] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22493600481] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22495276188] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22496938893] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=340 pred=0 subj_lo=0
[22522091823] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22561580580] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[22563915660] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[22567119003] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22579671543] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22595543982] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[22613726949] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[22616716683] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[22648400346] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22654256757] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22655979126] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22657273914] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22658589327] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=341 pred=0 subj_lo=0
[22703678052] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[22759174482] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[22770169719] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22771296108] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22772513280] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22773793680] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=342 pred=0 subj_lo=0
[22777332336] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x1222d000
[22779761664] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[22781383251] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[22847152779] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[22858562991] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[22863305652] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[22865489625] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[22867630962] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[22872107445] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[22893012846] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[22895106036] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[22897175268] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[22914545940] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x1222e000
[22916080440] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[23029703466] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=14aa2015f9906e3f)
[23218450563] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=149
[23220167718] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[23229976242] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[23311049784] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[23568377811] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[23837080014] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[23868551652] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=783 watches=19 history=1024 journal=1024 symbols=355 drops=0
[23927298681] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[23944709646] [INFO] [anther] [CPU1] anther: OTHER /health Http11
[23951720826] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[23953642878] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[23957770683] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 219 bytes - TCP ACK
[23962141566] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 219 bytes
[23967531522] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[23968836111] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[23971067241] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[24152282616] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[24451349208] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[24782145531] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[25014093489] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[25031648169] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[25059542046] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 77 bytes - TCP ACK
[25062722751] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 77 bytes
[25066780002] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 0 bytes on conn_handle=3
[25071032514] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[25072809201] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[25075466361] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[25077054948] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25077803817] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[25079270733] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[25081351548] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[25082429526] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[25127618538] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[25178391678] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[25194713214] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25326966357] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25528262793] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[25879940724] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[25889376216] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[25892117625] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[25924405749] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[25957498776] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=837 watches=19 history=1024 journal=1024 symbols=366 drops=0
[26279911449] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[26594403792] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[26732153943] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26734413618] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[26737101237] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[26739390348] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[26741779317] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[26743767072] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[26745991833] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26751487521] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=149
[26753256552] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[26755246881] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[26760755142] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[26764460844] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26949172074] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[27092644656] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[27094549878] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:56004 on listener 1
[27096569016] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[27103885083] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=4
[27112633713] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 38 (user thread) assigned to CPU 1
[27114390468] [INFO] [anther] [CPU1] anther: Thread spawned TID=38 for conn_handle=4
T:5EF0 [27140905506] [INFO] [anther] [CPU1] anther: Worker thread TID=38 starting for conn_handle=4
[27168990585] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=35, our_read=36)
[27170738925] [INFO] [anther] [CPU1] anther: Worker TID=38 connected to netd OK
[27332094108] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[27854162490] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=878 watches=19 history=1024 journal=1024 symbols=367 drops=0
[28033746081] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28239121284] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[28697817324] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[28942273338] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2431 ops=1 watches=19
[29300351553] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29320899465] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[29323342719] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[29326040040] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[29333224635] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[29379015336] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29433889716] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[29822846889] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=903 watches=19 history=1024 journal=1024 symbols=367 drops=0
[29893222854] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[30170003226] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[30300248286] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[30673256427] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[31820999430] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=962 watches=19 history=1024 journal=1024 symbols=403 drops=0
[32501050494] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[32502562620] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[32504813946] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[33584432244] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[33672093906] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[33854432634] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1041 watches=19 history=1024 journal=1024 symbols=451 drops=0
[33863968413] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[33866084274] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[33871629066] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[33873754563] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[33879066804] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[33880555434] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[33882518967] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[33884210580] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=149
[33885676044] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[33887905227] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[33901543665] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[33916495338] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[33921720591] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[33923471538] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:51760 on listener 1
[33925760220] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[33931479846] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[33964818195] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=5
[33967867098] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[33969165186] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[33973277943] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 39 (user thread) assigned to CPU 1
[33975087399] [INFO] [anther] [CPU1] anther: Thread spawned TID=39 for conn_handle=5
T:5EF0 [33993817209] [INFO] [anther] [CPU1] anther: Worker thread TID=39 starting for conn_handle=5
[34006086147] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[34033271646] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=37, our_read=38)
[34035508716] [INFO] [anther] [CPU1] anther: Worker TID=39 connected to netd OK
[34490696262] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[34526957421] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34528047510] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34529201355] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34530730443] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[34537940349] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34539018888] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34540181676] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34541423235] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=236 pred=0 subj_lo=0
[34549000662] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34550103456] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34551434016] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34552610169] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=453 pred=0 subj_lo=0
[34559684610] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34560817599] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34562002629] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34563333585] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=452 pred=0 subj_lo=0
[34571780034] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34572854580] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34574013045] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34575225201] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[34582257336] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[35568395181] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1068 watches=24 history=1024 journal=1024 symbols=454 drops=0
[35790705060] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=70
[35792734098] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[35795683638] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[36397011255] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=149
[36399039402] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[36400979967] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[36472126548] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[37041114759] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[37239093078] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1088 watches=24 history=1024 journal=1024 symbols=454 drops=0
[39036803025] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=1119 watches=24 history=1024 journal=1024 symbols=454 drops=0
[39110023986] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=70
[39112054476] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[39115188222] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[40037738631] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[40041005433] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[40046756871] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[40049522568] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[40060680891] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=70
[40062616011] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[40064747448] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[40070687217] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[40074782055] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=149
[40076364141] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[40078563261] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[40081066443] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[40462738173] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[40465007946] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:51762 on listener 1
[40467503934] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[40476415254] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=6
[40491551859] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 40 (user thread) assigned to CPU 1
[40494617988] [INFO] [anther] [CPU1] anther: Thread spawned TID=40 for conn_handle=6
T:5EF0 [40536852114] [INFO] [anther] [CPU1] anther: Worker thread TID=40 starting for conn_handle=6
[40570744731] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=39, our_read=40)
[40573523166] [INFO] [anther] [CPU1] anther: Worker TID=40 connected to netd OK
[40974242991] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=1145 watches=24 history=1024 journal=1024 symbols=454 drops=0
[42041554170] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[42406781142] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=70
[42408927033] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[42411822189] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[42492501018] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[42784336254] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=16000 nodes=1165 watches=24 history=1024 journal=1024 symbols=454 drops=0
[42977305635] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=149
[42979405590] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[42981958140] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[43118830227] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[43142256498] [INFO] [anther] [CPU1] anther: Worker TID=39 got first 85 bytes on conn_handle=5
[43143353154] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[43144266066] [INFO] [anther] [CPU1] anther: GET /health Http11
[43145748657] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[43257869919] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[43259951196] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[43264358841] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=25 len=70
[43266989766] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[43269814104] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[43910416968] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[43934167827] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[43935987150] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[43938088788] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=26 len=70
[43940641635] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[43942136238] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[43943495277] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[43951414023] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=27 len=70
[43952895393] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[43955012079] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[43961419227] [INFO] [anther] [CPU1] anther: Worker TID=39 got first 0 bytes on conn_handle=5
[44534928075] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[44755228155] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=17000 nodes=1198 watches=24 history=1024 journal=1024 symbols=454 drops=0
[45725654667] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=28 len=70
[45728240811] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[45730757853] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[45732171606] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[45734661852] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[45738118668] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[45746094273] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[45751309659] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=29 len=70
[45752993847] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[45755974869] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[45758927247] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[45764977368] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=30 len=149
[45768610701] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[45771631983] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[45778837203] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[45797506029] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[45803144244] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[45885836238] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[45887891874] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=2 (16384b)
[45890785182] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[45892812537] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[45899675712] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[45902630664] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:51776 on listener 1
[45905890833] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[45945700482] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=7
[45955564842] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 41 (user thread) assigned to CPU 1
[45957945330] [INFO] [anther] [CPU1] anther: Thread spawned TID=41 for conn_handle=7
T:5EF0 [46042840965] [INFO] [anther] [CPU1] anther: Worker thread TID=41 starting for conn_handle=7
[46192664001] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=41, our_read=42)
[46194919023] [INFO] [anther] [CPU1] anther: Worker TID=41 connected to netd OK
[46920704886] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=18000 nodes=1230 watches=24 history=1024 journal=1024 symbols=454 drops=0
[46948677039] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=3455 ops=1 watches=24
[47077021035] [INFO] [anther] [CPU1] anther: Worker TID=41 got first 85 bytes on conn_handle=7
[47078988396] [INFO] [anther] [CPU1] anther: GET /health Http11
[47095866807] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[47097769422] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[47100963261] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[47109936588] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[47114918565] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=31 len=70
[47118543648] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[47123132364] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[48481941057] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=19000 nodes=1250 watches=24 history=1024 journal=1024 symbols=454 drops=0
[49021728228] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=70
[49023382683] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[49025843625] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[49282350018] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[49340796978] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[49343285574] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[49345034706] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=70
[49347094533] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[49350078591] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[49354326417] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[49356676413] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[49363275291] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[49386027570] [INFO] [anther] [CPU1] anther: Worker TID=40 got first 2 bytes on conn_handle=6
[49566775731] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=149
[49568741607] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[49571987520] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[50145420039] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[50312517156] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=20000 nodes=1279 watches=24 history=1024 journal=1024 symbols=454 drops=0
[50584709901] [INFO] [anther] [CPU1] anther: Worker TID=41 got first 0 bytes on conn_handle=7
[50590927431] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[50592579609] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[50605910751] [INFO] [anther] [CPU1] anther: OTHER /health Http11
[51013061583] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 219 bytes - TCP ACK
[51016660926] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 219 bytes
[51019417878] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=70
[51021582876] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[51024403320] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[51755507166] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[51757769745] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 77 bytes - TCP ACK
[51764432445] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 77 bytes
[51768751155] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[51770610672] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[51772903908] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[51778870836] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=70
[51780428799] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[51782777706] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[51783666132] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[51785070909] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[51785902014] [INFO] [anther] [CPU1] anther: Worker TID=40 got first 0 bytes on conn_handle=6
[52117148820] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=21000 nodes=1299 watches=24 history=1024 journal=1024 symbols=454 drops=0
[52344788067] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[52347419520] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[52350096084] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[52939556868] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[52942319265] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[52946953917] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[52949998233] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[52955569656] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[52957791249] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[52959961857] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[52966234794] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=149
[52968311022] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[52970886078] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[52973961150] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[52979050608] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[52981181187] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:51782 on listener 1
[52983521514] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[53023564242] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=8
[53033163876] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 42 (user thread) assigned to CPU 1
[53034926406] [INFO] [anther] [CPU1] anther: Thread spawned TID=42 for conn_handle=8
T:5EF0 [53079368199] [INFO] [anther] [CPU1] anther: Worker thread TID=42 starting for conn_handle=8
[53111649294] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=43, our_read=44)
[53114938998] [INFO] [anther] [CPU1] anther: Worker TID=42 connected to netd OK
[54018282021] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=22000 nodes=1319 watches=24 history=1024 journal=1024 symbols=454 drops=0
[55639949343] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[55642053951] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[55644954915] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[56154028161] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=149
[56156354034] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[56159897508] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[56241301941] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=23000 nodes=1356 watches=24 history=1024 journal=1024 symbols=454 drops=0
[58062980250] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=24000 nodes=1378 watches=24 history=1024 journal=1024 symbols=454 drops=0
[58964294202] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[58966554702] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[58978061175] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[59669922279] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[59801825292] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 2 bytes on conn_handle=4
[59877318237] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=25000 nodes=1398 watches=24 history=1024 journal=1024 symbols=454 drops=0
[61919191719] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=26000 nodes=1429 watches=24 history=1024 journal=1024 symbols=454 drops=0
[62740176774] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=149
[62742696456] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[62746012329] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[63976036212] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=27000 nodes=1451 watches=24 history=1024 journal=1024 symbols=454 drops=0
[65229665985] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[65274759990] [INFO] [anther] [CPU1] anther: Worker TID=42 got first 85 bytes on conn_handle=8
[65276268915] [INFO] [anther] [CPU1] anther: GET /health Http11
[65290392552] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[65292640248] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[65303314527] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[65305676568] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[65320910259] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[65322895737] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[65325930516] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[65335645089] [INFO] [anther::net_client] [CPU1] anther: tcp_send got unexpected resp_type 0x0006 len=6
[65337788175] [INFO] [anther::net_client] [CPU1] anther: tcp_send chunk timeout
[65570111475] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[65572298352] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[65574823545] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[65754119178] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=28000 nodes=1469 watches=24 history=1024 journal=1024 symbols=454 drops=0
[66747076143] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[66788010135] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[66790838994] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[66801507432] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[66803673288] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[66805866996] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[66820322514] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[66823048281] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[66825156420] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[67021923474] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[67035328866] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[67040082846] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[67046813064] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=70
[67049262390] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[67053858663] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[67058623368] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[67218044058] [INFO] [anther] [CPU1] anther: Worker TID=42 got first 0 bytes on conn_handle=8
[67912297893] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=29000 nodes=1504 watches=24 history=1024 journal=1024 symbols=454 drops=0
[70003342992] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=30000 nodes=1526 watches=24 history=1024 journal=1024 symbols=454 drops=0
[71655667116] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=31000 nodes=1544 watches=24 history=1024 journal=1024 symbols=454 drops=0
[72193334994] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=70
[72195432606] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[72198087588] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[73455600339] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=32000 nodes=1573 watches=24 history=1024 journal=1024 symbols=454 drops=0
[74349907698] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[74353242711] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[74359496970] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[74364167592] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[74367726411] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=70
[74369664336] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[74371983147] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[74377518006] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=149
[74380340430] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[74382688017] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[74395077306] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[74413096395] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[74589935970] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[74592526140] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:55260 on listener 1
[74595194256] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[74628882141] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=9
[74637643410] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 43 (user thread) assigned to CPU 1
[74640553977] [INFO] [anther] [CPU1] anther: Thread spawned TID=43 for conn_handle=9
T:5EF0 [74692658106] [INFO] [anther] [CPU1] anther: Worker thread TID=43 starting for conn_handle=9
[74820311346] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=45, our_read=46)
[74823221748] [INFO] [anther] [CPU1] anther: Worker TID=43 connected to netd OK
[75484095489] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=70
[75487308699] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[75490091061] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[75492747198] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[75793621134] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=33000 nodes=1599 watches=24 history=1024 journal=1024 symbols=454 drops=0
[77561931876] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=70
[77564673318] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[77568270912] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[77593127106] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=34000 nodes=1617 watches=24 history=1024 journal=1024 symbols=454 drops=0
[78804807807] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=70
[78807393093] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[78809816613] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[79251093009] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[79254341034] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[79259013999] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[79261599780] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[79262831307] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=70
[79266155793] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[79268910501] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[79276366455] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=25 len=149
[79278285009] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[79280826273] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[79286524317] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=26 len=70
[79288502337] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[79290886851] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[79297713726] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[79317085122] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[79336638018] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[79338857202] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:51796 on listener 1
[79341354213] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[79352057334] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=35000 nodes=1646 watches=24 history=1024 journal=1024 symbols=454 drops=0
[81094164096] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=4479 ops=1 watches=24
[81854955798] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=36000 nodes=1668 watches=24 history=1024 journal=1024 symbols=454 drops=0
[83526803613] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=37000 nodes=1686 watches=24 history=1024 journal=1024 symbols=454 drops=0
[84154735170] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=27 len=70
[84156609240] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[84159657615] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[85384328007] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=38000 nodes=1710 watches=24 history=1024 journal=1024 symbols=454 drops=0
[85421019783] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=28 len=70
[85423509204] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[85426123464] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[86038559343] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[86041808853] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[86044929201] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[86052908370] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[86053866360] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=29 len=70
[86056477452] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[86059026174] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[86067565650] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=30 len=149
[86069664153] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[86072200698] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[86082440928] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=31 len=70
[86084951205] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[86086010802] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[86088916551] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[86107680483] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[86613658329] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[86615885037] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:55256 on listener 1
[86618642748] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[86651564373] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=11
[86660833215] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 44 (user thread) assigned to CPU 1
[86662888587] [INFO] [anther] [CPU1] anther: Thread spawned TID=44 for conn_handle=11
T:5EF0 [86756957298] [INFO] [anther] [CPU1] anther: Worker thread TID=44 starting for conn_handle=11
[86806424331] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=47, our_read=48)
[86808635694] [INFO] [anther] [CPU1] anther: Worker TID=44 connected to netd OK
[87669612129] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=39000 nodes=1739 watches=24 history=1024 journal=1024 symbols=454 drops=0
[89089960278] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=149
[89092338819] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[89095621956] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[89473215942] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=40000 nodes=1759 watches=24 history=1024 journal=1024 symbols=454 drops=0
[90989977089] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[91050751803] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[91052576637] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[91065573225] [INFO] [anther] [CPU1] anther: Worker TID=43 got first 85 bytes on conn_handle=9
[91069111254] [INFO] [anther] [CPU1] anther: GET /health Http11
[91085723289] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[91087700682] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[91094105190] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=70
[91096390110] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[91099762842] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[91416853275] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=41000 nodes=1779 watches=24 history=1024 journal=1024 symbols=454 drops=0
[92053332690] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[92056121916] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[92061056241] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[93689468136] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=42000 nodes=1806 watches=24 history=1024 journal=1024 symbols=454 drops=0
[94415807178] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[94517486514] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[94529509800] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[94565361132] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=70
[94568349117] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[94574765439] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[94591846239] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[94597813398] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[94601419869] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[95530356207] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[95641903830] [INFO] [anther] [CPU1] anther: Worker TID=43 got first 0 bytes on conn_handle=9
[95675846970] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=149
[95678744733] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[95681728923] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[95819229528] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[95847202902] [INFO] [anther] [CPU1] anther: OTHER /health Http11
[96146274180] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[96149325393] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[96159688911] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 219 bytes - TCP ACK
[96166196379] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 219 bytes
[96182577018] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[96184454454] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[96186910149] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[96622044486] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=43000 nodes=1832 watches=24 history=1024 journal=1024 symbols=454 drops=0
[97204605927] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[97231666653] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 77 bytes - TCP ACK
[97233561348] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 77 bytes
[97246206156] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[97249408146] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[97251991320] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[97256736918] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[97258675998] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[97261180038] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[97324428003] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[97326249537] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[97329892803] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[97746719466] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[97756997679] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 0 bytes on conn_handle=4
[97834504416] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=2
[98555938107] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[98665858731] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[98668650795] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[98671847604] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[98679358371] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[98683264449] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[98687611638] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[98694443793] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[98699149065] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[98701497180] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[98704880175] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[98708638380] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=149
[98711768298] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[98714886468] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[98723045586] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[98742678309] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[98744689032] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[98816681997] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[98899798107] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[98903298021] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:34708 on listener 1
[98907545814] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[99044542674] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=12
[99053798085] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 45 (user thread) assigned to CPU 1
[99055709874] [INFO] [anther] [CPU1] anther: Thread spawned TID=45 for conn_handle=12
T:5EF0 [99191789169] [INFO] [anther] [CPU1] anther: Worker thread TID=45 starting for conn_handle=12
[99223376505] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[99532222614] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=49, our_read=50)
[99536613660] [INFO] [anther] [CPU1] anther: Worker TID=45 connected to netd OK
[99560535921] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[99563196348] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[99668132025] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[99717018258] [INFO] [anther] [CPU1] anther: Worker TID=45 got first 85 bytes on conn_handle=12
[99720935358] [INFO] [anther] [CPU1] anther: GET /health Http11
[99736923462] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[99745794687] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[99752117619] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[99754333272] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[99757183020] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[99765370716] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[99941305794] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[99948213618] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[99958843545] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[99961936998] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[99967707378] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[99976336713] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[99977343081] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[99978591273] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[99982613577] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[99983654430] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[99986742603] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[99991008150] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[100000000122] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[100003231185] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[100006458552] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[100009785249] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[100012915530] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[100015895034] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[100018878234] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=70
[100022748045] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[100026306996] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[100033206867] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=217
[100037853630] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 207 byte frame (211 encoded) to netd rx_port=17
[100039058625] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[100041314571] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (211 bytes sent)
[100042809471] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 207 bytes
[100059476022] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[100061403486] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[100133045958] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[100135854918] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:34720 on listener 1
[100140827754] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[100234402059] [INFO] [anther] [CPU1] anther: Worker TID=45 got first 0 bytes on conn_handle=12
[100257097941] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=13
[100266919863] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 46 (user thread) assigned to CPU 1
[100269430536] [INFO] [anther] [CPU1] anther: Thread spawned TID=46 for conn_handle=13
T:5EF0 [100327924191] [INFO] [anther] [CPU1] anther: Worker thread TID=46 starting for conn_handle=13
[100373925102] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=51, our_read=52)
[100376380236] [INFO] [anther] [CPU1] anther: Worker TID=46 connected to netd OK
[100466051862] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=44000 nodes=1892 watches=24 history=1024 journal=1024 symbols=454 drops=0
[100656589902] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[100658601186] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[100709696307] [INFO] [anther] [CPU1] anther: Worker TID=46 got first 153 bytes on conn_handle=13
[100713462828] [INFO] [anther] [CPU1] anther: POST /api/v1/query Http11
[100714794411] [INFO] [anther] [CPU1] anther: Request body size: 66 bytes
[102300023496] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 209 bytes - TCP ACK
[102302022438] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 209 bytes
[102311948079] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=70
[102314011569] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[102316746708] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[102631098438] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=45000 nodes=1917 watches=24 history=1024 journal=1024 symbols=454 drops=0
[102718860255] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[102725543646] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 106 bytes - TCP ACK
[102730458468] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 106 bytes
[102741850596] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=70
[102744140202] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[102747675558] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[102759103425] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=70
[102761432994] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[102764622345] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[102795876810] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=70
[102798968943] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[102802024248] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[103475272758] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[103913900706] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=70
[103916703231] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[103919302014] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[104034330114] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[104037365289] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[104043264732] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[104046470847] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[104052784275] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=70
[104055022467] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[104058197562] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[104060743908] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[104068883094] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=25 len=149
[104072594670] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[104075677299] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[104086244427] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=26 len=70
[104089583664] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[104093630982] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[104817752061] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[104828205339] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[104830762509] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:55272 on listener 1
[104833305423] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[104836886913] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=14
[104846891787] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 47 (user thread) assigned to CPU 1
[104849174859] [INFO] [anther] [CPU1] anther: Thread spawned TID=47 for conn_handle=14
[104850752655] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[104852801889] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[104884375860] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=46000 nodes=1939 watches=24 history=1024 journal=1024 symbols=454 drops=0
T:5EF0 [104958934014] [INFO] [anther] [CPU1] anther: Worker thread TID=47 starting for conn_handle=14
[105025709910] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=53, our_read=54)
[105028166793] [INFO] [anther] [CPU1] anther: Worker TID=47 connected to netd OK
[106759297491] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=47000 nodes=1957 watches=24 history=1024 journal=1024 symbols=454 drops=0
[107204055420] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=27 len=70
[107206751223] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[107209385844] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[107226346788] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[107230023153] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[107233665528] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[107281333500] [INFO] [anther] [CPU1] anther: Worker TID=46 got first 0 bytes on conn_handle=13
[107287755531] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[107289716160] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[107346827577] [INFO] [anther] [CPU1] anther: Worker TID=47 got first 85 bytes on conn_handle=14
[107349223344] [INFO] [anther] [CPU1] anther: GET /health Http11
[107782703754] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[107785362036] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[107796985923] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=28 len=70
[107799331530] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[107802068748] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[107805182067] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[107821106085] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[107823723249] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[107833500951] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=29 len=70
[107835786630] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[107839252356] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[107847504732] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[107850367350] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=30 len=70
[107852402460] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[107857027377] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[107867545830] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[108144812292] [INFO] [anther] [CPU1] anther: Worker TID=47 got first 0 bytes on conn_handle=14
[109223433462] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=48000 nodes=1988 watches=24 history=1024 journal=1024 symbols=454 drops=0
[110510019543] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=31 len=70
[110513426595] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[110517705540] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[110743267140] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[110746995975] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[110753253435] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[110756641809] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[110761884519] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=70
[110764294179] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[110769396177] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[110775805008] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=149
[110779804113] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[110783679732] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[110790318771] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[110797790598] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[110799962097] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[110803496133] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[111622919298] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=49000 nodes=2010 watches=24 history=1024 journal=1024 symbols=454 drops=0
[112060286217] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[112070889975] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[112073340555] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:55288 on listener 1
[112076402526] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[112123289091] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=15
[112134280203] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 48 (user thread) assigned to CPU 1
[112137076326] [INFO] [anther] [CPU1] anther: Thread spawned TID=48 for conn_handle=15
T:5EF0 [112261169163] [INFO] [anther] [CPU1] anther: Worker thread TID=48 starting for conn_handle=15
[112326204342] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=55, our_read=56)
[112328579418] [INFO] [anther] [CPU1] anther: Worker TID=48 connected to netd OK
[113509961961] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=50000 nodes=2024 watches=24 history=1024 journal=1024 symbols=454 drops=0
[115729424151] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=51000 nodes=2047 watches=24 history=1024 journal=1024 symbols=454 drops=0
[118299090228] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=52000 nodes=2077 watches=24 history=1024 journal=1024 symbols=454 drops=0
[118735301520] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=149
[118737029994] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[118739965443] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[118814147628] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[120133596066] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[120135824985] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[120405895005] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=53000 nodes=2093 watches=24 history=1024 journal=1024 symbols=454 drops=0
[122024951862] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[122034260073] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[122039038572] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[122137386657] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=54000 nodes=2109 watches=24 history=1024 journal=1024 symbols=454 drops=0
[122536567758] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[122540124729] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[122544941310] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[122550209925] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[122554586385] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=70
[122557248693] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[122561684982] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[122572826442] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=218
[122575742289] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 208 byte frame (212 encoded) to netd rx_port=17
[122578894119] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (212 bytes sent)
[122706257982] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[122719296711] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[122732510604] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[122735141925] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:34734 on listener 1
[122738049258] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[122908145679] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=16
[122917151016] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 49 (user thread) assigned to CPU 1
[122919000171] [INFO] [anther] [CPU1] anther: Thread spawned TID=49 for conn_handle=16
T:5EF0 [123096501231] [INFO] [anther] [CPU1] anther: Worker thread TID=49 starting for conn_handle=16
[123218347461] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=57, our_read=58)
[123221403162] [INFO] [anther] [CPU1] anther: Worker TID=49 connected to netd OK
[123380279055] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=5503 ops=1 watches=24
[124954521087] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=55000 nodes=2140 watches=24 history=1024 journal=1024 symbols=454 drops=0
[125324101518] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=149
[125340963396] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[125344557723] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[126317325453] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[126360770382] [INFO] [anther] [CPU1] anther: Worker TID=44 got first 85 bytes on conn_handle=11
[126363367779] [INFO] [anther] [CPU1] anther: GET /health Http11
[126368158587] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[126370018995] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[126375338034] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[126377613516] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[126389114082] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[126394388043] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[126399187695] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[127055045766] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=56000 nodes=2160 watches=24 history=1024 journal=1024 symbols=454 drops=0
[127273394061] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[127303726275] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[127307061750] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[127313176584] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[127315482888] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[127319244789] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[127324367643] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[127327206435] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[127329579333] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[128260552266] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[128369485002] [INFO] [anther] [CPU1] anther: Worker TID=44 got first 0 bytes on conn_handle=11
[129184818180] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=57000 nodes=2176 watches=24 history=1024 journal=1024 symbols=454 drops=0
[131154443112] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[131412666231] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[131668565358] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[132114904053] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[132234794373] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=58000 nodes=2225 watches=24 history=1024 journal=1024 symbols=454 drops=0
[132588058251] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[132845803464] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[135262186026] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[135547348167] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[135814918998] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[136126876215] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[136518657792] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[136845763857] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[136849478766] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[136853845524] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[136864507230] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[136868341137] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[136872283746] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[136876302915] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[136879452996] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[136883123982] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[136891455327] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[136892326032] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=149
[136896024639] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[136897047210] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[136899768621] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[136900750536] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:55266 on listener 1
[136903186695] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[136907525205] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[136913266743] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[136915827411] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[136920219183] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[136931819079] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[136935985032] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[136937729148] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[137119961022] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=17
[137130051432] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 50 (user thread) assigned to CPU 1
[137133210192] [INFO] [anther] [CPU1] anther: Thread spawned TID=50 for conn_handle=17
T:5EF0 [137415558324] [INFO] [anther] [CPU1] anther: Worker thread TID=50 starting for conn_handle=17
[137863254969] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=59, our_read=60)
[137865861606] [INFO] [anther] [CPU1] anther: Worker TID=50 connected to netd OK
[138070800516] [INFO] [anther] [CPU1] anther: Worker TID=50 got first 85 bytes on conn_handle=17
[138072707883] [INFO] [anther] [CPU1] anther: GET /health Http11
[138090808317] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[138095839992] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[138105186384] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[138110512353] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[138117936066] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[138129406503] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[138399566448] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[138402055671] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[138419637906] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[138422334732] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[138425107062] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[138426050301] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[138437261820] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=70
[138440534859] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[138443603595] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[138444724539] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[138875712261] [INFO] [anther] [CPU1] anther: Worker TID=50 got first 0 bytes on conn_handle=17
[139916791608] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=59000 nodes=2371 watches=24 history=1024 journal=1024 symbols=454 drops=0
[148844154525] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=60000 nodes=2550 watches=24 history=1024 journal=1024 symbols=454 drops=0
[150026045532] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=149
[150031877259] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[150035793270] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[150041344398] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[150071402646] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[150075666048] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[150078838338] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=74
[150082428573] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=17
[150086333628] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[150095145651] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[150098091990] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[150099887850] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[161550513102] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=218
[161555494386] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 208 byte frame (212 encoded) to netd rx_port=17
[161563876815] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (212 bytes sent)
[161567735571] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 208 bytes
[200350129089] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[200354339196] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[237739442589] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[237742086549] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[237995231331] [INFO] [anther] [CPU1] anther: Worker TID=49 got first 154 bytes on conn_handle=16
[237998839914] [INFO] [anther] [CPU1] anther: POST /api/v1/query Http11
[237999807771] [INFO] [anther] [CPU1] anther: Request body size: 67 bytes
[238016733207] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 209 bytes - TCP ACK
[238022421252] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 209 bytes
[238024241433] [INFO] [anther] [CPU1] anther: Worker TID=48 got first 85 bytes on conn_handle=15
[238028556876] [INFO] [anther] [CPU1] anther: GET /health Http11
[238049419179] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[238050263814] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=70
[238053641892] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[238058332677] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[238063030689] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[238079248473] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[238080290250] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=70
[238084484121] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[238088416863] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[238102561026] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[238276188447] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 106 bytes - TCP ACK
[238278972129] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 106 bytes
[238287957402] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=70
[238293070092] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[238294280664] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[238298067480] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[238299128001] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[238300114404] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[238310555175] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=70
[238314101421] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[238319008983] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[238320513288] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[238323809229] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[238331150079] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=25 len=70
[238334638344] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[238339300452] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[238342973187] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[238345093008] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[238356692178] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=26 len=70
[238360654818] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[238370549538] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[238383740463] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[238392602085] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=27 len=70
[238397218092] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[238404200991] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[238422043101] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[238425496254] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[238430184564] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[238438134363] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=28 len=70
[238442600682] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[238447812801] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[238460669073] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[238466567790] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=29 len=265
[238470621015] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 255 byte frame (259 encoded) to netd rx_port=17
[238474737864] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (259 bytes sent)
[238497387876] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 255 bytes
[238517403267] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[238520764383] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[238671575241] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[238674651171] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:50216 on listener 1
[238678046508] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[238784113260] [INFO] [anther] [CPU1] anther: Worker TID=48 got first 0 bytes on conn_handle=15
[238864007448] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=18
[238873974966] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 51 (user thread) assigned to CPU 1
[238876601964] [INFO] [anther] [CPU1] anther: Thread spawned TID=51 for conn_handle=18
[239100031170] [INFO] [anther] [CPU1] anther: Worker TID=49 got first 0 bytes on conn_handle=16
T:5EF0 [239213675745] [INFO] [anther] [CPU1] anther: Worker thread TID=51 starting for conn_handle=18
[239737836987] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=61, our_read=62)
[239740358022] [INFO] [anther] [CPU1] anther: Worker TID=51 connected to netd OK
[239757392226] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[239760098490] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[239989037046] [INFO] [anther] [CPU1] anther: Worker TID=51 got first 201 bytes on conn_handle=18
[239991890886] [INFO] [anther] [CPU1] anther: POST /api/v1/query Http11
[239992873329] [INFO] [anther] [CPU1] anther: Request body size: 113 bytes
[240010269147] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 209 bytes - TCP ACK
[240019604649] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 209 bytes
[240030381591] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=30 len=70
[240036207675] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[240041230605] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[240048188985] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[240293741424] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 106 bytes - TCP ACK
[240303160350] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 106 bytes
[240331684527] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=31 len=70
[240340111308] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[240348645603] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[240353610981] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[240363374988] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=70
[240373119954] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[240379786020] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[240381223698] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[240384428658] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[240397227576] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=70
[240401800782] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[240412811232] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[240413822814] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[240421731165] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[240422960316] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[240439838199] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[240462817914] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[240468227439] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[240475926537] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[240477504036] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[240484867590] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=258
[240489849171] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 248 byte frame (252 encoded) to netd rx_port=17
[240497990667] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (252 bytes sent)
[240510801333] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 248 bytes
[240531516951] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[240533910408] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[240779151657] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[240782284677] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:50228 on listener 1
[240785781555] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[240839535486] [INFO] [anther] [CPU1] anther: Worker TID=51 got first 0 bytes on conn_handle=18
[241048275699] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=19
[241058036538] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 52 (user thread) assigned to CPU 1
[241060126164] [INFO] [anther] [CPU1] anther: Thread spawned TID=52 for conn_handle=19
T:5EF0 [241279198611] [INFO] [anther] [CPU1] anther: Worker thread TID=52 starting for conn_handle=19
[241829479881] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=63, our_read=64)
[241832781201] [INFO] [anther] [CPU1] anther: Worker TID=52 connected to netd OK
[241856305548] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[241861410747] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[242114360598] [INFO] [anther] [CPU1] anther: Worker TID=52 got first 194 bytes on conn_handle=19
[242118054651] [INFO] [anther] [CPU1] anther: POST /api/v1/query Http11
[242119623768] [INFO] [anther] [CPU1] anther: Request body size: 106 bytes
[242135112879] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 209 bytes - TCP ACK
[242137506831] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 209 bytes
[242153146884] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[242156083653] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[242160950394] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[242168970120] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[242453522091] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 106 bytes - TCP ACK
[242461180104] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 106 bytes
[242485905420] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=70
[242489780874] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[242494680912] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[242507487090] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[242511248595] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[242517179718] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[242523754671] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[242534661138] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[242537636979] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[242540708916] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[242991078495] [INFO] [anther] [CPU1] anther: Worker TID=52 got first 0 bytes on conn_handle=19
[244489486725] [INFO] 
```
</details>
