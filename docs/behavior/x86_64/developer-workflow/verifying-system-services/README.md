# ❌ Scenario: Verifying System Services

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ❌ | 1001ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11135376549] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11140715157] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11144213289] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11146177680] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11147365614] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11148000996] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11148647598] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11149236252] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11149816590] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11150417949] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11151000102] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11151595290] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11152282647] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11152950930] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11153639607] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11154246015] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11154860442] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11155454904] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11156078076] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11156665377] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11157234495] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11157809652] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11158393785] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11158978347] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11159619669] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11160215649] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11160808263] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11161458891] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11162042364] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11162654745] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11163259173] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11163867627] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11164488324] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11165096712] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11165694936] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11166383646] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11167082553] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11167799280] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11168492907] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11169232833] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11169935667] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11170644012] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11171878311] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11173201875] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11173936356] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11174468118] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11174968959] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11175479403] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11176037565] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11176550286] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11177056176] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11177566158] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11178069375] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11178593151] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11179141017] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11179692777] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11180225364] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11180778741] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11181310008] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11181859887] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11182407225] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11182956840] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11183490681] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11184040428] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11184570903] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11185131144] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11185676304] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11186225259] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11186764248] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11187312675] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11187864303] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11188413720] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11188968780] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11189529912] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11190065238] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11190616932] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11191155162] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11191712730] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11192268087] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11192827668] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11193372135] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11193928614] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11194467273] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11195021970] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11195575842] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11196132651] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11196669891] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11197225479] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11197766910] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11198324016] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11198880363] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11199429846] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11199962301] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11200510662] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11201054040] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11201605173] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11202160035] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11202712257] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11203243656] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11203797561] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11204332722] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11204887023] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11205441390] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11205995361] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11206526793] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11207076078] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11207608368] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11208326118] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11432181552] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[11442509991] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11447259978] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11448497379] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11449549848] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11453555256] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11455186710] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11456261850] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11456933301] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11457593532] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11458249011] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11459211027] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11460152451] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11460825486] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11461489908] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11462171853] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11462835285] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11463882441] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11464878645] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11465588739] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11467090503] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11468002161] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11468986650] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11470529202] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11472041262] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11472817818] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11473344399] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11474080068] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[11837740662] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[11838755577] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[11841961428] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[11842830945] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[11843608920] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[11845136391] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[11857040646] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[11858311542] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[11859048663] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[11860733148] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[11861272137] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[11863614345] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[11871245232] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[11872916715] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[11886017220] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[11886634815] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[11902110396] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[11902721721] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[11904756468] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[11905956216] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[11907043731] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[11909280834] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[11910098673] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[11944904895] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62314300 ticks/sec), init_cnt=623143 for 100Hz
[11946302544] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[11947100022] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[11948228457] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[11953741899] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[11983764309] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[11984649567] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[11987361705] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[11989002432] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[11990131659] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[11992772682] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[11994266889] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12012858561] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12015542979] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12017257593] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12018047448] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12019354281] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12019971579] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12021255642] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12047305611] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12049097511] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12050003031] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12051699957] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12052419126] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12053297487] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12053988375] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12054619467] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12060927582] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12061902930] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12063678429] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12064497159] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12069233055] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12070737228] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12071430591] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12072663141] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12073534209] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12074347395] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12086335371] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12089218350] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12090391236] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12091249500] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12112653993] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12130366149] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12132939225] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12136196292] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12137687991] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12139649544] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12141854175] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12143668383] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12144697059] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12145796388] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12152567262] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12154918743] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12158653320] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12162057897] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12168300276] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12169216983] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12185402889] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12186575841] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12194147559] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12194892138] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12221597223] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12222404337] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[12558980940] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13030851636] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13061294796] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=499 journal=424 symbols=51 drops=0
[13094697759] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[14368996620] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=446 watches=0 history=960 journal=768 symbols=94 drops=0
[15101327241] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15194420208] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15195574449] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[15309553908] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[15370096467] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[15397211214] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[15398334963] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[15398980509] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[15402861738] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[15417727446] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[15434013408] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[15436456233] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[15485949105] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[15503387724] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[15505387755] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[15510010989] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[15533579490] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[15551529048] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[15555385230] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[15556417503] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[15624890292] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[15626858643] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[15711594492] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[15773260206] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[15783088794] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[15786728661] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[15788963058] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[15796005687] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[15855055227] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[15860617575] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[15861599985] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[15862646448] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[15863613216] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[15864311793] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[15864960045] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[15865683273] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[15866326608] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[15866947371] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[15867588066] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[15868211535] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[15868848798] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[15869540214] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[15870230244] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[15870951162] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[15871593507] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[15872249745] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[15872891496] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[15873540012] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[15874167936] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[15874774113] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[15875391741] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[15876023790] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[15876645642] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[15877314585] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[15877950627] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[15878578089] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[15879285642] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[15879909672] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[15880544262] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[15881187663] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[15881838423] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[15882504495] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[15883143804] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[15883764336] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[15884496771] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[15885239040] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[15886106511] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[15886844622] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[15887613588] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[15888593160] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[15889416345] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[15891114921] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[15892963548] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[15893768748] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[15901836258] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[15915445755] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[15919704570] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[15929357202] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[15930059112] [CONTRACT] [kernel] [CPU0] Spawning init process...
[15932092440] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[15934566912] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[15935424780] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [15941052336] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[15941847339] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013360 RFLAGS_BEFORE=130 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[15957580254] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[15962208735] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[15963409935] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[15964522002] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[15973918620] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[15979182846] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[15982468227] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[15983532807] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[15987238113] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[15990103239] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[15991242432] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[15994945461] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[15996006180] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[15997244439] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[15998362347] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16001396829] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16002514803] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16003679868] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16005101178] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16006465827] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16007782527] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16011386688] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[16056569760] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[16063655223] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[16068226416] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[16072839387] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[16075753749] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[16079320686] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[16083896301] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[16088947611] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[16094088780] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[16100464875] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[16107200835] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[16114201851] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[16121179041] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[16127636217] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[16133000928] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[16137822954] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[16142988114] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[16147604286] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[16151884221] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[16156524450] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[16160762508] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[16165178667] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[16170133980] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[16175663097] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[16181389752] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[16187060208] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[16193088780] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[16198745838] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[16204078275] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[16209430413] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[16213157829] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[16216687311] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[16222861908] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[16228876092] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[16233403527] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[16238654388] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[16244484531] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[16250430603] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[16256695488] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[16262640537] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[16268734581] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[16271763057] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[16290222828] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[16420105383] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[16427476362] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[16428484281] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16430424945] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16434885324] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[16436182158] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16444358040] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[16450219401] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16451542569] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16453520292] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078896 RFLAGS_BEFORE=130 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[16463018781] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[16463979345] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[16464779133] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16466762070] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16470762198] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[16472910498] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16483180263] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[16488692979] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[16490487717] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[16491909588] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[16493846589] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144432 RFLAGS_BEFORE=130 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[16501723458] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[16503338115] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[16505410845] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[16509667812] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:52:12 = 1775436732 unix_secs
[16511251944] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775436732, mono_ns=8255406027, offset=1775436723744593973ns
[16512995697] [INFO] [rtc_cmos] [CPU1] System clock anchored
[16527074883] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[16564616475] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[16576872543] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[16578082059] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16580990382] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16587055716] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[16589403897] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[16596799098] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[16599781242] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[16602588783] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16604123382] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210544 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[16609817235] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[16614575538] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[16616334471] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[16617122610] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16618897680] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16625478936] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[16627832496] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[16634865654] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[16637490672] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16638645837] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16640285211] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277296 RFLAGS_BEFORE=130 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[16643798094] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[16645939068] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[16649771259] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[16653207285] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[16654823229] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[16656705351] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[16657670568] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[16659293376] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[16674144102] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[16675952502] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[17130402597] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17135827698] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[17138828058] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[17140409484] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[17144621109] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[17146092810] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[17148048357] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[17149422213] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[17150367597] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[17152593051] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[17153500221] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[17154439731] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[17155322415] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[17156093493] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[17156858961] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[17158040064] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[17162293467] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[17163322836] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17165099292] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17172081201] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17174302299] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17180924772] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[17183746569] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fab68
[17185244802] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[17187438873] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352720 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[17194489158] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[17202963558] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[17226807939] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[17234838522] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[17237157102] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[17242783404] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[17251183389] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[17252614797] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[17261873376] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[17263512090] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[17264407908] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[17265337782] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[17266160109] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[17266931187] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[17267798262] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[17268664215] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[17269569603] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[17275695327] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[17278122213] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[17280458118] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[17282399475] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[17283092574] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17284601697] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17288039142] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[17289646044] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17296484931] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[17298595512] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[17300386158] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[17301116580] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17302826970] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17307219336] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17308553526] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17314957242] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[17317448643] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
[17318249619] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[17321218398] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500160 RFLAGS_BEFORE=134 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[17325260766] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[17326217370] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17327897004] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17329659765] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[17331437574] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[17333816577] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17336608179] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17338529439] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[17340733047] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[17343003513] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[17344749081] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[17345870652] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[17348394261] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[17350116366] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[17351007399] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[17351990535] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[17353454580] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[17354382705] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[17355999375] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[17357094777] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[17358718113] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[17359564167] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369433920 RFLAGS_BEFORE=130 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[17365228617] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[17692403487] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[17694309336] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369582944 RFLAGS_BEFORE=134 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[17699169279] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[17702157924] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[17704446507] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[17716411680] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[17718499260] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[17721144573] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[17727158856] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17731507332] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[17732739321] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17735272302] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[17736195906] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17745950112] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[17749447584] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[17760186378] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[17764737771] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[17767395360] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[17768870856] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[17769656454] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17771262201] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17788948881] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[17792921586] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[17812930014] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[17815607799] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[17817383925] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17819159391] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369719312 RFLAGS_BEFORE=130 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[17823822522] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[17824740582] [INFO] [netd] [CPU3] NETD: Starting network stack service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[17825982570] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653776 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[17829214260] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[17831046024] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[17832227391] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[17836910322] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[17839031232] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[17843057232] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[17843935494] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[17844971793] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[17846220876] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[17847580311] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[17865456840] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[17871916458] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[17873860455] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[17876480754] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[17878604370] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[17881802796] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[17883672147] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[17886034914] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4256000
[17887375572] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[17889831927] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[17892207630] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[17893792455] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[17902966191] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[17911268232] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[17920062171] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[17922507636] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[17924819550] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[17926498821] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[17927953626] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[17929288344] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[17930686224] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[17932430472] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[17934274050] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[17941769472] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=15, read=16)
[17947101150] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=17, read=18)
[17951273010] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[17953818135] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[17955472062] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[17956269276] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17957994021] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17959190337] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18018457314] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[18040270479] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[18047102403] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[18050018811] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[18051681153] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18052912350] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915920 RFLAGS_BEFORE=130 CR3_BEFORE=69668864 fs_base=0 gs_base=18446744071564586576
[18055962276] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[18056715171] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18058567923] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18059342004] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[18062078496] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[18066364008] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18067717635] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18082476390] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18085431507] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[18092394474] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[18095054439] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[18097057374] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[18097730277] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18099154458] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18104251143] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18106551144] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18112905096] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[18115244037] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
[18116115765] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010aff8
[18117677820] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18118489785] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18120472557] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18121358607] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370046992 RFLAGS_BEFORE=130 CR3_BEFORE=70602752 fs_base=0 gs_base=18446744071564586640
[18129929730] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[18131291376] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[18132087270] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18134764692] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18136277742] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18144673206] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[18147180876] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010aff8
[18149127315] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18150850674] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113296 RFLAGS_BEFORE=130 CR3_BEFORE=70729728 fs_base=0 gs_base=18446744071564586576
[18154816449] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[18156141102] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18158001279] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[18159496542] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18160673058] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[18162557061] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18164417700] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[18173591634] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18176508438] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[18183078309] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[18185414214] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[18187671810] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[18188492982] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18190032564] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18215466027] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18220032930] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[18227772816] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[18230447499] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cc18
[18231741429] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18233365392] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370245808 RFLAGS_BEFORE=130 CR3_BEFORE=71032832 fs_base=0 gs_base=18446744071564586640
[18237482967] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[18238426470] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18240442704] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18241425873] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[18260588181] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[18264718296] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18268802145] [INFO] [fontd] [CPU3] FONTD: Service node created, req=19, resp=22
[18272444025] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[18275006739] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1120
[18276013800] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[18276815073] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18279012741] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[18279802728] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18281054550] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370330784 RFLAGS_BEFORE=134 CR3_BEFORE=71307264 fs_base=0 gs_base=18446744071564586576
[18284333892] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18286058241] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[18286742793] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18287676066] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[18288340092] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18291137073] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[18295178121] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18296214750] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18297569730] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18298938702] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=226 subj_lo=0
[18310577109] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18312388314] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18314276508] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18316390587] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[18326992695] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[18337231110] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[18338212200] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18339613611] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18341190285] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18342652284] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[18346294659] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18347443191] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18348815232] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18350167737] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[18352369827] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[18355784601] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[18360119778] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18362116311] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1249) for kind 'Asset'
[18364691565] [INFO] [fontd] [CPU3] FONTD: Service ready
[18365266821] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18366968136] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981456 RFLAGS_BEFORE=130 CR3_BEFORE=70340608 fs_base=0 gs_base=18446744071564586608
[18371794386] [INFO] [nectar] [CPU2] NECTAR: Started.
[18372992418] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18374040300] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18375092670] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18376278558] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[18377549553] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010aff8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18379349868] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370179824 RFLAGS_BEFORE=134 CR3_BEFORE=70877184 fs_base=0 gs_base=18446744071564586608
[18382440516] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18383946339] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[18385876608] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010e118
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18387442887] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370398288 RFLAGS_BEFORE=134 CR3_BEFORE=71516160 fs_base=0 gs_base=18446744071564586608
[18392920755] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[18394984575] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[18396532011] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[18427768689] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=15, RX port=18
[18430428126] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18431679453] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18433439739] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18434402877] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18435497487] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18436566324] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=244 subj_lo=0
[18443066532] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18444317859] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18445603242] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18446679240] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=246 subj_lo=0
[18452451402] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18453509745] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18454621614] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18455805456] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=247 subj_lo=0
[18465946488] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18467038722] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18468185043] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18469262592] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=249 pred=0 subj_lo=0
[18480320100] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[18481343826] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[18493158354] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[18502277343] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18506811543] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[18521651049] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[18530428389] [INFO] [netd] [CPU3] NETD: Driver TX port=15, RX port=18, link_up=true, mtu=1500
[18532925103] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[18537009282] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18538091286] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[18539020764] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18547264296] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1257 backend=VirtIO-GPU
[18554319333] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=23, resp=26
[18555782685] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[18556734933] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18558043350] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[18559009194] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18579132462] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[18595357605] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[18596658168] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[18601724031] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[18627486372] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[18689206536] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[18697620315] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[18699522831] [INFO] [anther] [CPU1] anther: Connected to network stack
[18703591137] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[18710475201] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[18712764378] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db630
[18714471897] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e9
[18716103846] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370559408 RFLAGS_BEFORE=130 CR3_BEFORE=72437760 fs_base=0 gs_base=18446744071564586640
[18718958115] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[18719824926] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18721377807] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18723410079] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[18725499375] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18726898014] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18733776534] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[18736063368] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[18738326013] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[18743242386] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[18748836843] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[18749668146] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db630
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[18752288610] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370624944 RFLAGS_BEFORE=130 CR3_BEFORE=73826304 fs_base=0 gs_base=18446744071564586576
[18759237651] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[18773455404] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[18775871466] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[18777458832] [INFO] [echo] [CPU1] echo: starting up
[18779612478] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[18789895608] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[18792469179] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[18798189861] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[18799586421] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[18800686146] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[18802141776] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[18802961595] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18804664263] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18808892190] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[18810228492] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18812093553] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18819385233] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[18822453804] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[18823272501] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[18825268737] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[18826648764] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[18827750502] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[18830128317] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18831209628] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18833535798] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18839217507] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[18844941984] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18850539510] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18861381990] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[18862538673] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[18864040338] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[18867272457] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[18868686936] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[18870560775] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[18872723463] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[18873655581] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18875249448] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18876669636] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f3580
[18881765199] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[18882904953] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18888592635] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370756016 RFLAGS_BEFORE=130 CR3_BEFORE=74784768 fs_base=0 gs_base=18446744071564586640
[18892762647] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[18895451553] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[18896705553] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[18900540582] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f3580
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18902147121] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370821552 RFLAGS_BEFORE=130 CR3_BEFORE=74932224 fs_base=0 gs_base=18446744071564586576
[18905196651] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[18906627498] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[18913218027] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[18914763351] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[18918985965] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[18939658155] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[18944737977] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[18946735830] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[18950373354] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[18956581908] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[18984153078] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[18988673286] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[18989959989] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[19000144383] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[19001625687] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[19004758014] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[19010444079] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[19012808727] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[19015232973] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[19016555745] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[19017769914] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[19030663839] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[19037337627] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[19061683509] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[19063996281] [INFO] [bloom] [CPU3] bloom: creating surface...
[19066410693] [INFO] [bloom] [CPU3] bloom: surface created!
[19067874111] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[19074363132] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[19077732663] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[19084358964] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[19085944383] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[19095047697] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1266
[19096043043] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[19107442794] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[19110668247] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[19111916637] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[19113136548] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [19119798027] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[19127544183] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[19137101577] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
T:07D0 T:0640 [19155525708] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[19162084062] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1266
T:F0B0 [19172381118] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 5)
[19186631739] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[19194772509] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[19197513819] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[19199959581] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[19202615520] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[19213633362] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[19215576072] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[19219304346] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
T:1220 [19221858117] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[19223502012] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[19234421580] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=288
[19235779167] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[19237065936] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19238184339] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19239462363] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19240862355] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=288 subj_lo=0
[19256055126] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1279)
[19258436010] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[19263893748] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[19266161112] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[19269841041] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[19277609406] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=249
[19279711836] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[19281194658] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19282351473] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19283507496] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19284787005] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=249 pred=0 subj_lo=0
[19289410272] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[19295171082] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[19305566148] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1282)
[19306686960] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[19319923260] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=289
[19321184784] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[19322095584] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[19322823432] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19324066278] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19325344005] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19326522765] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=289 subj_lo=0
[19336328946] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=74
[19337954097] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=17
[19339080948] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1285)
[19340745237] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[19342037385] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[19349054934] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[19353244977] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[19359134124] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[19369124247] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[19370986206] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[19373669700] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[19381250526] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=149
[19383387441] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[19384420869] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[19385410110] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[19387297149] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[19518371565] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[19522694268] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[19526457852] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:37714 on listener 1
[19530435573] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[19535746593] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[19581144726] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[19869347025] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=3
[19883068359] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 37 (user thread) assigned to CPU 1
[19886494947] [INFO] [anther] [CPU1] anther: Thread spawned TID=37 for conn_handle=3
[19946161719] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
T:5EF0 [19981430403] [INFO] [anther] [CPU1] anther: Worker thread TID=37 starting for conn_handle=3
[19989754653] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[20005539840] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[20044543794] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=33, our_read=34)
[20046684702] [INFO] [anther] [CPU1] anther: Worker TID=37 connected to netd OK
[20209048761] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[20226681651] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=665 watches=13 history=1024 journal=1024 symbols=313 drops=0
[20461433784] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[20740254645] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[21127184067] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[21397703481] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[21425989101] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[21623950854] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[21818325969] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[22024655994] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[22099908006] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[22106343336] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[22108064352] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[22118748135] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[22120320849] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[22122266925] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=226 subj_lo=0
[22137937338] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[22139407752] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[22141060161] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[22142886546] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=337 pred=0 subj_lo=0
[22234704162] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[22320414468] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=721 watches=15 history=1024 journal=1024 symbols=338 drops=0
[22390170033] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[22463293611] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 85 bytes on conn_handle=3
[22496820159] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[22499436036] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[22501212591] [INFO] [anther] [CPU1] anther: GET /health Http11
[22515560397] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[22517503668] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[22532028321] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[22535440719] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[22540799094] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[22543165557] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[22546580529] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22548225282] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22553559699] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[22559670738] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[22561937871] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[22564796364] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22567013403] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[22576034052] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22578565218] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[22582658274] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[22584084567] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[22586626986] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22588669554] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[22595845734] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22598107917] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[22599743397] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[22633189557] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 0 bytes on conn_handle=3
[22672911393] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[22726456203] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22727942259] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22729216950] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22730592786] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=339 pred=0 subj_lo=0
[22745307024] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22879952568] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[22922839995] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22962208995] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[22986560025] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22987725420] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22988581704] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22989747561] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=340 pred=0 subj_lo=0
[23061955587] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[23087725188] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23088977472] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23090674596] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23092375680] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=341 pred=0 subj_lo=0
[23112741168] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23116717470] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[23155091520] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[23168024022] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[23169965280] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[23195361948] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23196500151] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23197756923] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23198759001] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=342 pred=0 subj_lo=0
[23224117521] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23247376746] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[23262927930] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12222000
[23264430420] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[23265732270] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[23298429957] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=fec1d29ef0678173)
[23333663694] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[23343913263] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[23348595699] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[23350579857] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[23352615924] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[23357602719] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[23378826537] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[23380912401] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[23383411260] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[23426758245] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x12223000
[23428039833] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[23517493428] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[23796232614] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=14aa2015f9906e3f)
[24036382062] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[24266545677] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[24498017775] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[24531997512] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=792 watches=19 history=1024 journal=1024 symbols=364 drops=0
[24794466675] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[25267590777] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[25789408029] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[25900534704] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[25903074846] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[25907529582] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26051109579] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26060587377] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26065507017] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[26071080189] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[26072878689] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[26085435519] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[26088619953] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[26092109868] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26093648559] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26096272554] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=149
[26098876749] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[26103791505] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[26161601433] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[26277249669] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[26309963988] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[26312729784] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[26316844323] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[26321186034] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:60852 on listener 1
[26325566223] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[26352075288] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[26354177256] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=4
[26367005808] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 38 (user thread) assigned to CPU 1
[26370309339] [INFO] [anther] [CPU1] anther: Thread spawned TID=38 for conn_handle=4
[26460555429] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
T:5EF0 [26547080109] [INFO] [anther] [CPU1] anther: Worker thread TID=38 starting for conn_handle=4
[26560992612] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[26656411155] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=35, our_read=36)
[26659787484] [INFO] [anther] [CPU1] anther: Worker TID=38 connected to netd OK
[26929856811] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[27263142885] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[27409035291] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=835 watches=19 history=1024 journal=1024 symbols=366 drops=0
[27590614854] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[27937634934] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[28307935986] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[28660534551] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[28953035727] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 85 bytes on conn_handle=4
[28954682724] [INFO] [anther] [CPU1] anther: GET /health Http11
[28982810241] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[28986955701] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[28993878441] [INFO] [anther::net_client] [CPU1] anther: tcp_send got unexpected resp_type 0x0003 len=6
[28995629256] [INFO] [anther::net_client] [CPU1] anther: tcp_send chunk timeout
[29325259293] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[29327208240] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[29329700697] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[29331830946] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[29369878494] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[29373323364] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[29500362309] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=870 watches=19 history=1024 journal=1024 symbols=367 drops=0
[29509351938] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[29519847555] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[29522142243] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[29524805244] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[29604098370] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[29987652816] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29998151139] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30028246908] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 0 bytes on conn_handle=4
[30248184549] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[30964818357] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2431 ops=1 watches=19
[31018027161] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[31488879708] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[31857774399] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[31993921014] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[32009243607] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=906 watches=19 history=1024 journal=1024 symbols=368 drops=0
[32395448217] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[32509553538] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[32511860700] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[32515869639] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[34515162264] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=971 watches=19 history=1024 journal=1024 symbols=424 drops=0
[36121039812] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[36250076973] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[36581342754] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[36583290150] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[36606121200] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[36775346256] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[36778415619] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[36787296249] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[36795057585] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[36800526279] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[36803214921] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[36806639199] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[36816124983] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=149
[36819807717] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[36823478208] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[36833305476] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[36834812322] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=70
[36837853206] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[36841375725] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[36853132338] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[36880666317] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[36883535139] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:60854 on listener 1
[36886182498] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[37086212427] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1031 watches=19 history=1024 journal=1024 symbols=451 drops=0
[37325093685] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[37385973471] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[37387660662] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[37389303072] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[37391031645] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[37397855352] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[37399300191] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[37400629464] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[37401943359] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=236 pred=0 subj_lo=0
[37412647008] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[37413986280] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[37415335947] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[37416778377] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=453 pred=0 subj_lo=0
[37423140678] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[37424293005] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[37425578553] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[37426934391] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=452 pred=0 subj_lo=0
[37437922170] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[37439151981] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[37440361794] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[37441775349] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[37450369077] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[38918362494] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1062 watches=24 history=1024 journal=1024 symbols=454 drops=0
[39131796693] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=70
[39133655253] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[39136178169] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[40010525247] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[40013925600] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[40018671561] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[40021223187] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[40032602511] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=70
[40035399228] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[40038419058] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[40045084464] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[40046402352] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=149
[40049426076] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[40052512830] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[40063868658] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[40076090043] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[40115362947] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[40118565498] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:60868 on listener 1
[40120698222] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[40163766951] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=6
[40173718959] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 39 (user thread) assigned to CPU 1
[40176045657] [INFO] [anther] [CPU1] anther: Thread spawned TID=39 for conn_handle=6
T:5EF0 [40227744315] [INFO] [anther] [CPU1] anther: Worker thread TID=39 starting for conn_handle=6
[40329667158] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=37, our_read=38)
[40332415530] [INFO] [anther] [CPU1] anther: Worker TID=39 connected to netd OK
[41082808470] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1095 watches=24 history=1024 journal=1024 symbols=454 drops=0
[42431683899] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=70
[42433901169] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[42436731909] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[42819873558] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[42827409933] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[42930355974] [INFO] [anther] [CPU1] anther: Worker TID=39 got first 2 bytes on conn_handle=6
[43026324825] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=1119 watches=24 history=1024 journal=1024 symbols=454 drops=0
[44848432233] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=1143 watches=24 history=1024 journal=1024 symbols=454 drops=0
[45746568912] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=70
[45748408959] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[45750662331] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[46814397267] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=16000 nodes=1174 watches=24 history=1024 journal=1024 symbols=454 drops=0
[48731988129] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=17000 nodes=1196 watches=24 history=1024 journal=1024 symbols=454 drops=0
[49562655087] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=149
[49565353959] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[49568494899] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[49982228538] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[49984839135] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[49990421052] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[49991628060] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[50008893264] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=70
[50012478318] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[50014794225] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[50016955593] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[50029337325] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=25 len=149
[50041814691] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[50045390769] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[50048011035] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[50049189333] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[50055062673] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=26 len=70
[50057428641] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[50060562585] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[50073735129] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[50077648599] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:60876 on listener 1
[50081531874] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[50135563170] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[50488704684] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=3455 ops=1 watches=24
[50664353553] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=18000 nodes=1218 watches=24 history=1024 journal=1024 symbols=454 drops=0
[50742284307] [INFO] [fontd] [CPU3] FONTD: Auto-importing font asset ThingId([0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[51096642300] [INFO] [fontd] [CPU3] FONTD: Auto-imported font 'Noto Sans' style 'Regular' from asset ThingId([0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[51100214352] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (1 fonts)
[51200712354] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[51332937282] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[52239605649] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[52333031025] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[52336095867] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=2 (16384b)
[52339167573] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[52342842486] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[52363682712] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=27 len=70
[52366413165] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[52370165727] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[52378838589] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[52382153901] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[52386039750] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[52388407467] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[52394167023] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=28 len=70
[52397051223] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[52400112237] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[52402898328] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[52412807172] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=29 len=149
[52415133045] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[52417867788] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[52422517026] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[52441465824] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[52460240877] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[52551264579] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[52554173430] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:60886 on listener 1
[52557263781] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[52610684115] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=8
[52621349484] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 40 (user thread) assigned to CPU 1
[52623705585] [INFO] [anther] [CPU1] anther: Thread spawned TID=40 for conn_handle=8
T:5EF0 [52651246065] [INFO] [anther] [CPU1] anther: Worker thread TID=40 starting for conn_handle=8
[52738293333] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=39, our_read=40)
[52740638544] [INFO] [anther] [CPU1] anther: Worker TID=40 connected to netd OK
[52844590227] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[52846218117] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[52851720933] [INFO] [anther] [CPU1] anther: Worker TID=40 got first 85 bytes on conn_handle=8
[52853380767] [INFO] [anther] [CPU1] anther: GET /health Http11
[52869356892] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[52877786841] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[52887281931] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=30 len=70
[52889041953] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[52891204377] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[52894047888] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[52940564886] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[52943167035] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[52960727754] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=31 len=70
[52963536549] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[52967424246] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[52979680512] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=70
[52980613389] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[52981782117] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[52984606554] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[52985696742] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[52988007072] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[52992879918] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[53036195949] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=70
[53038903104] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[53042639298] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[53052372582] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[53054944206] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[53062033695] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[53078801358] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[53080907550] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[53084150889] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[53092024953] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=184
[53095042242] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 174 byte frame (178 encoded) to netd rx_port=17
[53096446062] [INFO] [anther] [CPU1] anther: Worker TID=40 got first 0 bytes on conn_handle=8
[53099485230] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (178 bytes sent)
[53114193462] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[53347528674] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[53350274736] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:60900 on listener 1
[53352826857] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[53472874323] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=9
[53482284537] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 41 (user thread) assigned to CPU 1
[53484464550] [INFO] [anther] [CPU1] anther: Thread spawned TID=41 for conn_handle=9
T:5EF0 [53629075797] [INFO] [anther] [CPU1] anther: Worker thread TID=41 starting for conn_handle=9
[53673087270] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=2
[53733765228] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=41, our_read=42)
[53737971705] [INFO] [anther] [CPU1] anther: Worker TID=41 connected to netd OK
[53846862564] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[54074518674] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[56154884148] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=149
[56160873384] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[56166339900] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[56167826187] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[56186765613] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[56189876358] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[56380200987] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=19000 nodes=1341 watches=24 history=1024 journal=1024 symbols=456 drops=0
[56678869137] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[57799588473] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=184
[57801938733] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 174 byte frame (178 encoded) to netd rx_port=17
[57804553191] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (178 bytes sent)
[57821759655] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 174 bytes
[57843075642] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[57847809426] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[57989256336] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[57996867126] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[58064068722] [INFO] [anther] [CPU1] anther: Worker TID=41 got first 120 bytes on conn_handle=9
[58070072874] [INFO] [anther] [CPU1] anther: POST /api/v1/query Http11
[58074519228] [INFO] [anther] [CPU1] anther: Request body size: 33 bytes
[58085285907] [INFO] [phloem::executor] [CPU1] phloem: calling find for kind: proc.Task
[58203927408] [INFO] [phloem::executor] [CPU1] phloem: find returned 0 candidates
[58205323605] [INFO] [phloem::executor] [CPU1] phloem: discovered 0 nodes
[58226414268] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 209 bytes - TCP ACK
[58238918892] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 209 bytes
[58255309398] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[58259392059] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[58263753537] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[58274814972] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[58332608301] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 124 bytes - TCP ACK
[58337066997] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 124 bytes
[58351356195] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[58353997383] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[58356470337] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[58365196164] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[58371302979] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[58374327759
```
</details>
