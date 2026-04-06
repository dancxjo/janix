# ❌ Scenario: Serve concurrent requests

> Last run: 2026-04-05 17:56:38

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the anther server is ready | ✅ | 6687ms | - [📜](./01/serial.log) - |
| 2 | When I make 10 concurrent GET requests to "/health" | ❌ | 45266ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11039235669] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11044546986] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11048072805] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11050047558] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11051206221] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11051847081] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11052521370] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11053099959] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11053678152] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11054282712] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11054891100] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11055508662] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11056293534] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11057152293] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11057881824] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11058541296] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11059161795] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11059767576] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11060395170] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11060989797] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11061586074] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11062168722] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11062759026] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11063348373] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11063980917] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11064598776] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11065193040] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11065838190] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11066430111] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11067110406] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11067719487] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11068344309] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11068980846] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11069585934] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11070167988] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11070853992] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11071564053] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11072280417] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11072972460] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11073690969] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11074392318] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11075115117] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11076388950] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11077778448] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11078528109] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11079064623] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11079565398] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11080076436] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11080627734] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11081157747] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11081663637] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11082245856] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11082756069] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11083285851] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11083823289] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11084393001] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11084929812] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11085483948] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11086020594] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11086574697] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11087112498] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11087679933] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11088232452] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11088785235] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11089321452] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11089875918] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11090413191] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11090982342] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11091517635] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11092073421] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11092608813] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11093214627] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11093753682] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11094361245] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11094985968] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11095542447] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11096078136] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11096633196] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11097170766] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11097745560] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11098283361] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11098839213] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11099377443] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11099933625] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11100472020] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11101042623] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11101580523] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11102137266] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11102674341] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11103230193] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11103766839] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11104338762] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11104874418] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11105431458] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11105967609] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11106524055] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11107059447] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11107626321] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11108162967] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11108718159] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11109256719] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11109812076] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11110349382] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11110918137] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11111454849] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11112010371] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11112546951] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11113300374] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11345188074] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[11355726393] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11360467833] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11361721767] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11362565940] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11366700444] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11368381365] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11369453964] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11370129969] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11370802179] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11371487094] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11372454687] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11373398619] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11374079046] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11374768053] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11375437854] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11376105147] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11377223682] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11378262489] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11378957271] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11380478472] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11381422734] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11382406761] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11383956177] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11385505098] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11386263438] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11386792593] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11387555949] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[11749104873] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[11750109789] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[11753650986] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[11754521493] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[11755278249] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[11756777373] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[11768826366] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[11770103928] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[11770836132] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[11772479664] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[11773054128] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[11775394917] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[11782982541] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[11784661449] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[11797369782] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[11797910784] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[11814688743] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[11815235091] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[11817170937] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[11818330260] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[11819367879] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[11821481001] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[11822287455] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[11857017018] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62290200 ticks/sec), init_cnt=622902 for 100Hz
[11858413281] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[11859225246] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[11860335696] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[11865746508] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[11895889368] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[11896760040] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[11898548343] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[11899913553] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[11901012783] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[11903689578] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[11904927870] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[11925322002] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[11927140896] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[11928038265] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[11929225704] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[11929884615] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[11931104427] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[11931895404] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[11957040579] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[11959039455] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[11959916298] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[11960925438] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[11961644838] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[11962567782] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[11963189139] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[11963864154] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[11970270510] [INFO] [kernel::root] [CPU0] Spawning Root service...
[11971320405] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[11973160023] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[11974018716] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[11979290499] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[11980808103] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[11981512917] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[11982733158] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[11983638678] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[11984494005] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[11999149602] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12002316678] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12003464022] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12004213683] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12030369318] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12048049860] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12050294619] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12053363487] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12054988341] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12057304842] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12059789907] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12061874583] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12062849469] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12063879069] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12070033041] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12071707692] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12074175333] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12076465896] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12080743323] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12081519351] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12092604381] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12093393312] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12099669648] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12100488972] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12127168515] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12127911543] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[12468587406] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[12921035787] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[12945839379] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[12978102291] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[14134055034] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=446 watches=0 history=959 journal=768 symbols=93 drops=0
[14818455069] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[14906512731] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[14907575364] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[15009178998] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[15067521777] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[15092491821] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[15095199735] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[15095827890] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[15100262397] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[15116206215] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[15134367072] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[15136487421] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[15188523636] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[15210260703] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[15211039635] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[15214713393] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[15237088713] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[15255047874] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[15258683418] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[15259685364] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[15324893034] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[15326590818] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[15408131343] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[15469931994] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[15481149222] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[15486932934] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[15489107403] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[15493287216] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[15559661370] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[15565557777] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[15566624403] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[15567485307] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[15568700169] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[15569512101] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[15570171012] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[15570869589] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[15571472367] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[15572086728] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[15572726433] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[15573347691] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[15574024422] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[15574680693] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[15575522853] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[15576249744] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[15576892683] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[15577564167] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[15578190870] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[15578838924] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[15579466650] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[15580073190] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[15580699596] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[15581321382] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[15581941188] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[15582670059] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[15583311777] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[15583966233] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[15584647188] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[15585267258] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[15585900000] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[15586539540] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[15587206206] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[15587841687] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[15588483207] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[15589176735] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[15590013153] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[15590779611] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[15591527655] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[15592260585] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[15593100303] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[15593866266] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[15594673776] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[15596252661] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[15597964173] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[15598744227] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[15606890409] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[15621784398] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[15626621340] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[15636063432] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[15636800949] [CONTRACT] [kernel] [CPU0] Spawning init process...
[15639188235] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[15641858727] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[15642619542] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [15648944058] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013248 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[15655032129] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[15667190748] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[15673325382] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[15675187506] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[15676578456] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[15686018403] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[15691749942] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[15695245401] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[15696470229] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[15700822269] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[15704842164] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[15706001256] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[15709897500] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[15710932875] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[15711970725] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[15713081604] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[15716419983] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[15717507069] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[15719270358] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[15721114101] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[15722825712] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[15724273059] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[15728829369] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[15771055740] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[15778434573] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[15783849807] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[15791064894] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[15793895337] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[15797314071] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[15802170780] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[15806669208] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[15811164303] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[15815959599] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[15821419284] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[15826660212] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[15832206819] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[15836374389] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[15840737913] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[15845777343] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[15850726320] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[15855206169] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[15860029746] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[15864343770] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[15869145105] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[15874394646] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[15879658872] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[15884327481] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[15889137957] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[15893581341] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[15899243811] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[15904165695] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[15908706990] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[15913083945] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[15916527099] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[15919849341] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[15924183132] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[15928681164] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[15933264765] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[15937888692] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[15942705570] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[15947424471] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[15953234187] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[15959349615] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[15966169725] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[15970116723] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[15990247878] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[16116130239] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[16122572664] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[16123545339] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16125500556] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16131564207] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[16133136888] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16141383027] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[16146667614] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16147795587] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16149367905] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078784 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[16153436013] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[16154396808] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16155834090] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[16156667505] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16161281466] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[16163082342] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16169974821] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[16173624885] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[16174694778] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[16176628710] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144320 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[16180015368] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[16182919896] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[16184770173] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[16186981569] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[16191584409] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:57:23 = 1775437043 unix_secs
[16193581173] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775437043, mono_ns=8096565856, offset=1775437034903434144ns
[16195424619] [INFO] [rtc_cmos] [CPU1] System clock anchored
[16205652672] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[16242814929] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[16252757697] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[16253832408] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16255530819] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16261386603] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[16264189590] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[16272404940] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[16275821001] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[16279723482] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16281534159] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210880 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[16288561014] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[16294441911] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[16296452073] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[16297301427] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16299135600] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16306053423] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[16308412164] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[16315371501] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[16318274412] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[16319214549] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16320736146] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277632 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[16323978264] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[16326635193] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[16328597538] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[16336399365] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[16337289507] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[16338108204] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[16338999006] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[16340605479] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[16356291006] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[16359056406] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[16832894925] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[16844767962] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[16847821617] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[16850521941] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[16855150950] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[16857591102] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[16860819195] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[16862962281] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[16864428471] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[16867933698] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[16869406653] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[16870821462] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[16872206934] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[16873063977] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[16873979595] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[16875098691] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[16887367464] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[16888828572] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16890583644] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16897876116] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[16900323231] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[16907114895] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[16910431263] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[16911872340] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[16914115020] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369357472 RFLAGS_BEFORE=134 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[16923672414] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[16932796254] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[16945468419] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[16970954286] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[16972303326] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[16973519607] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[16974947022] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[16975995003] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[16977309459] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[16978782777] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[16980004668] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[16982143596] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[16983158511] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[16984095150] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[16993177707] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[16995557436] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[17001078864] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[17003653425] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[17006196405] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[17007849639] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[17008585572] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17009942004] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[17011488054] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17023744518] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[17025678483] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17033072628] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[17036131365] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[17038576038] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[17039744040] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17041615305] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17046975627] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17048726739] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17056038582] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[17058906084] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a150
[17060205756] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[17061826617] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500368 RFLAGS_BEFORE=130 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[17065875783] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[17066740482] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17068480506] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17070244323] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[17072443410] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[17073880560] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17076599199] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17079701958] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[17081815575] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[17083967769] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[17085649812] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[17086769766] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[17089232226] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[17091248691] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[17092120287] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[17093471769] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[17095275417] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[17096562912] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[17097610167] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[17099518227] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
[17100734838] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[17102511723] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434128 RFLAGS_BEFORE=130 CR3_BEFORE=60096512 fs_base=0 gs_base=18446744071564586608
[17108799477] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[17469804627] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010a150
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[17471782944] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583152 RFLAGS_BEFORE=134 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[17475151749] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17477158083] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[17479418880] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[17480738979] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[17481761319] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17484032775] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[17484792171] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17491951818] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[17494943268] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[17498032134] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[17502986325] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[17506450038] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[17509489107] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[17511602592] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[17512858143] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17516077953] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17536097436] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[17540896725] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[17561707680] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[17564225976] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010bbe8
[17565558384] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17567010846] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369718960 RFLAGS_BEFORE=130 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[17572364403] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[17573141190] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[17575376940] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[17577230880] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[17582506821] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[17584288557] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653424 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[17589605088] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[17590491138] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[17592400683] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[17595048735] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[17605183794] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[17608981104] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[17610187353] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[17612341626] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[17615024526] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[17617077159] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[17617994658] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17619941790] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17678579391] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[17679534015] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[17681011524] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[17682460356] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[17687257863] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[17711271072] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[17718926511] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[17722265847] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[17723696001] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17725533639] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369784496 RFLAGS_BEFORE=130 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[17729674512] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[17730838851] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17733766215] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[17734727736] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17736734961] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[17740428189] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[17742075087] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[17759887101] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[17763138426] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[17770382718] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[17773255665] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[17775379050] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[17776270479] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17777968659] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17783496324] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17786227140] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17794091106] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[17797106184] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010df28
[17798499279] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17800515546] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915568 RFLAGS_BEFORE=130 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[17804802873] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[17805828975] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17807841282] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[17809312851] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17810495934] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[17819155167] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17822752959] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[17829647319] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[17832336621] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010df28
[17833902834] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17835387636] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981712 RFLAGS_BEFORE=130 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[17839638729] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[17840597808] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17842235499] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17851107153] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17855071608] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[17861793312] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[17864233695] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[17866614711] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[17867432187] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17869116441] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17893886967] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[17898976854] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[17906452410] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[17910540846] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010e6f0
[17912494446] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17913946875] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113872 RFLAGS_BEFORE=134 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[17917371483] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[17918206515] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17919588654] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[17920716924] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17939299719] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[17944792965] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[17948233677] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[17950034586] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[17951130285] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[17953149423] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[17955517371] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[17961456942] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[17966025033] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[17968484292] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[17969420007] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17971127394] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17972398818] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17974258104] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370197968 RFLAGS_BEFORE=130 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[17979627996] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[17981015316] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[17984332608] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[17985672144] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[17987053359] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[17988520011] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[17989977621] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[17991518325] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[17993208915] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[17994914949] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=229 pred=0 subj_lo=0
[18013642020] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[18024048735] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[18030299463] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18031630089] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18033097467] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18034362423] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18035946753] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18037576590] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18039189498] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[18040597542] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[18042531639] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[18047029143] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[18050217933] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[18051469392] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
[18052721247] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[18054289770] [INFO] [fontd] [CPU3] FONTD: Service ready
[18055733322] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[18060280623] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18063251052] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369850032 RFLAGS_BEFORE=130 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[18068778189] [INFO] [nectar] [CPU2] NECTAR: Started.
[18070987077] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010df28
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18072701625] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047792 RFLAGS_BEFORE=134 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[18077299284] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18078170814] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[18078919122] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18081080853] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010f9d8
[18082138833] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18083832492] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18085012110] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370266880 RFLAGS_BEFORE=134 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[18088311615] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18089982141] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[18096148356] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[18100180098] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[18101655957] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[18103244445] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[18112523649] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18113367624] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18114185232] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18115256214] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[18116339835] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=236 subj_lo=0
[18126463806] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1251 backend=VirtIO-GPU
[18128184030] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[18129249864] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18131025924] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18132069615] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18133089612] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18134175345] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18134915040] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=240 subj_lo=0
[18140147520] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18141296151] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18142520055] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18143902524] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[18149746791] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18150908820] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18152209086] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18153621288] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=242 pred=0 subj_lo=0
[18174293643] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[18186962376] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[18188164302] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[18268911672] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[18283379433] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[18291417903] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[18294143604] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18295231911] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e3
[18296511684] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370356400 RFLAGS_BEFORE=130 CR3_BEFORE=72052736 fs_base=0 gs_base=18446744071564586640
[18299524254] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[18300548970] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18301877022] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[18302565864] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18306842433] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18308143095] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18314940468] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[18317333661] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[18318646929] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[18320472654] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370422192 RFLAGS_BEFORE=130 CR3_BEFORE=73101312 fs_base=0 gs_base=18446744071564586576
[18324748992] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[18326770605] [INFO] [echo] [CPU1] echo: starting up
[18329973816] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[18331358694] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[18336263418] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[18349731807] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[18351612444] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[18361420902] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18363157956] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18369558174] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[18371497122] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[18376907934] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[18384866610] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[18386980821] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f5000
[18389395992] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f5000
[18391610655] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f8000
[18394780635] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276787200)
[18396556992] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[18399176928] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f9000 phys=0x4656000
[18401376609] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[18403791120] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[18405234672] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[18407771514] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[18409187247] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[18410326473] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[18411714981] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[18421986330] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[18432388821] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[18434534019] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[18441889752] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[18448131801] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[18449200077] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[18450341283] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[18451962837] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[18453363654] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[18454691838] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[18456095658] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[18457374078] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[18458657976] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[18463965630] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[18468865140] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[18473630769] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[18474978819] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[18476562588] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[18477290700] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18478758969] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18483731178] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18485201064] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18486047283] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18486844530] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18493720014] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[18494862474] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[18496492311] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[18497489241] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[18498656946] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[18500707599] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[18502186527] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[18503771715] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18504507846] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18506011887] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18513516714] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18517063719] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18518277987] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18524396187] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[18526765290] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[18527985234] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[18529310580] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010df28
[18530413539] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[18531220719] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18532962822] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18533740302] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370706832 RFLAGS_BEFORE=134 CR3_BEFORE=73998336 fs_base=0 gs_base=18446744071564586640
[18536664498] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[18537747261] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18544434513] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[18546382206] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[18547534368] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[18555981312] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f25d8
[18557017974] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18558835251] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370772368 RFLAGS_BEFORE=134 CR3_BEFORE=74145792 fs_base=0 gs_base=18446744071564586576
[18563027967] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18564526200] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[18569401719] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[18596988696] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[18609302382] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[18610557273] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[18621234621] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[18633455082] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18639607305] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[18648555321] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18649519845] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18663947676] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[18665724231] [INFO] [bloom] [CPU3] bloom: creating surface...
[18666603582] [INFO] [bloom] [CPU3] bloom: surface created!
[18667420860] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[18673074354] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[18678005313] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1261
[18678902088] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[18691212243] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[18693947910] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[18695571114] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[18697147821] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [18708517575] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[18710576346] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[18714103023] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[18719043288] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 3)
[18721149975] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[18730557087] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[18738162366] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[18745650660] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[18753116019] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1261
T:07D0 T:0640 T:F0B0 [18763006383] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[18781318512] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[18783051111] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[18791642001] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[18801477552] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[18803823357] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[18806667231] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[18808934958] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[18814306104] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=275
[18815631648] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[18816600759] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[18817597557] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18818820141] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18820036620] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=275 subj_lo=0
T:1220 [18827319720] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[18829097628] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[18837966642] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1272)
[18839036601] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[18844132593] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([239, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[18853326327] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=242
[18854686719] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[18855524193] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[18856511520] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18857636094] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[18858911907] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18860373609] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=242 pred=0 subj_lo=0
[18861760368] [INFO] [anther] [CPU1] anther: Connected to network stack
[18902650239] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1277)
[18904895031] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[18917646231] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=280
[18918907161] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[18920045727] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[18921163602] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18922339293] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18923712258] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=280 subj_lo=0
[18929729478] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[18930939060] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[18936136065] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[18965214510] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[18966188142] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[18973269975] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[18974981520] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1279)
[18976171632] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[18998649252] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[19001328753] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19026692685] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19033552725] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[19037324031] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[19042018743] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[19043483877] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[19045629240] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19057707042] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[19092361035] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19095889989] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[19097567247] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[19129138347] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[19131979647] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[19136628258] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[19138807347] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[19140856977] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[19142745963] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[19147032366] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[19174409034] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[19176343032] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[19183069158] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[19229116038] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[19259882928] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[19261789239] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[19264062345] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[19271913936] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[19278541722] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[19282845516] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[19293555765] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=74
[19295242032] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=25
[19297785507] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[19302480747] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[19307163282] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[19309256868] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[19313415660] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[19314589899] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[19316927256] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[19318858449] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=149
[19319749185] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[19320914712] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=25
[19323435912] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[19326344895] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[19330815768] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[19332613410] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:33938 on listener 1
[19337541927] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[19340274261] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[19343980392] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[19392246753] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[19406547864] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=3
[19415151855] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 37 (user thread) assigned to CPU 1
[19417503501] [INFO] [anther] [CPU1] anther: Thread spawned TID=37 for conn_handle=3
[19418382357] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
T:5EF0 [19443943992] [INFO] [anther] [CPU1] anther: Worker thread TID=37 starting for conn_handle=3
[19467890607] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[19490488941] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=33, our_read=34)
[19492512072] [INFO] [anther] [CPU1] anther: Worker TID=37 connected to netd OK
[19504988382] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=659 watches=13 history=1024 journal=1024 symbols=307 drops=0
[19615819014] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[19742338968] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[19885253190] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[20103895251] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[20260121541] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[20421247473] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[20430655806] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[20584592292] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[20757487707] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[20762427675] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[20769825450] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[20771042028] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[20772227421] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[20773695030] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[20783186127] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[20786720460] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[20787984129] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[20789298024] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[20790859881] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=337 pred=0 subj_lo=0
[20940944145] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[21042924507] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=708 watches=15 history=1024 journal=1024 symbols=339 drops=0
[21127104768] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[21315405969] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[21459963822] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 85 bytes on conn_handle=3
[21483226875] [INFO] [anther] [CPU1] anther: GET /health Http11
[21506692680] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[21508646049] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[21513084714] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[21519155559] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[21522370254] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[21524064705] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[21527912010] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[21529440438] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[21530956425] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[21538377036] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[21543227079] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[21545965782] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[21554989038] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[21556779849] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[21558943197] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[21564065721] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[21565432020] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[21568541346] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[21569835441] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[21571766964] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[21579020001] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[21580830051] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[21582263670] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[21596386383] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[21597877356] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[21600425781] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[21605703042] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[21607886916] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[21614510775] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[21615797775] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 0 bytes on conn_handle=3
[21616889316] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[21618777510] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[21620372037] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[21625779582] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[21626932932] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[21628873794] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[21631886166] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[21632850756] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[21634001070] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[21635250813] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP RST
[21636286683] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[21637294602] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[21638253120] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[21639307107] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP RST
[21651180870] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[21652328775] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[21654301614] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[21655205154] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[21656173869] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[21657601020] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[21659204127] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:33946 on listener 1
[21660436776] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=124
[21663998862] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 114 byte frame (118 encoded) to netd rx_port=25
[21665287743] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[21666400008] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (118 bytes sent)
[21668486202] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 114 bytes
[21692824725] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21717964092] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[21719136582] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21720483939] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21721751634] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=306 pred=0 subj_lo=0
[21790259964] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=4
[21811480581] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[21821030583] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21826634148] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[21830983812] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[21948729231] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21979228227] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=fec1d29ef0678173)
[21997960644] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 38 (user thread) assigned to CPU 1
[22000343574] [INFO] [anther] [CPU1] anther: Thread spawned TID=38 for conn_handle=4
[22014371313] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22015671414] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22016867103] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22018126977] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=340 pred=0 subj_lo=0
[22029292197] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[22053231321] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22064298201] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
T:5EF0 [22095108915] [INFO] [anther] [CPU1] anther: Worker thread TID=38 starting for conn_handle=4
[22126234911] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22127495379] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22128702816] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22129998099] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=341 pred=0 subj_lo=0
[22151771928] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22166050698] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[22222964940] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=35, our_read=36)
[22225431558] [INFO] [anther] [CPU1] anther: Worker TID=38 connected to netd OK
[22230106272] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[22234543617] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22235865069] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22237067985] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22238344029] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=342 pred=0 subj_lo=0
[22243728870] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[22245944490] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[22251429717] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[22257685032] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[22261663116] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[22279156350] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x1222d000
[22281236208] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[22283872149] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[22285626330] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[22296134586] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[22298207514] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22303832364] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[22305266676] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[22306876911] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 60 bytes on conn_handle=4
[22308184965] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22309330032] [INFO] [anther] [CPU1] anther: GET /health Http11
[22335052014] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[22340796984] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22343608551] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[22348819944] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[22350921648] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[22351636395] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[22357675989] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=70
[22359242334] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[22361201643] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22363091916] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[22369037955] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=124
[22370346735] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 114 byte frame (118 encoded) to netd rx_port=25
[22372199388] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (118 bytes sent)
[22374882849] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=70
[22376349699] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[22377869613] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[22378894923] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22387444530] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[22391502870] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[22393660839] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[22396037730] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[22401106497] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[22406292282] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22413208290] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22418518452] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[22420299264] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:33980 on listener 1
[22422067206] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[22429172733] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[22430650110] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[22432226421] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[22451101398] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x1222e000
[22452540561] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[22455914217] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=5
[22465330338] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 39 (user thread) assigned to CPU 1
[22466890908] [INFO] [anther] [CPU1] anther: Thread spawned TID=39 for conn_handle=5
T:5EF0 [22492210389] [INFO] [anther] [CPU1] anther: Worker thread TID=39 starting for conn_handle=5
[22523236626] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=37, our_read=38)
[22525335921] [INFO] [anther] [CPU1] anther: Worker TID=39 connected to netd OK
[22670433951] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=14aa2015f9906e3f)
[22920118089] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[23138411637] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[23310928113] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=781 watches=19 history=1024 journal=1024 symbols=354 drops=0
[23366090187] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[23634742989] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[23911336317] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[24183022314] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 204 bytes - TCP ACK
[24187123158] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 204 bytes
[24190025739] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[24199318341] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=124
[24201545247] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 114 byte frame (118 encoded) to netd rx_port=25
[24204500562] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (118 bytes sent)
[24231601581] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 114 bytes
[24259832883] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 60 bytes on conn_handle=4
[24261376260] [INFO] [anther] [CPU1] anther: GET /health Http11
[24381994560] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[24384769926] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[24392410317] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[24395397906] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[24398428362] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=70
[24400854357] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[24403793964] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[24487129788] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[24743781018] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[24773976480] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[24789760512] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[24794982201] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[24800715621] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=70
[24802545042] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[24804744888] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[24810277041] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=124
[24811802004] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 114 byte frame (118 encoded) to netd rx_port=25
[24813718710] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (118 bytes sent)
[24932689386] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[24943090623] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24949714119] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=70
[24951671613] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[24954672897] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[24959844690] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=25 len=70
[24961292367] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[24962533398] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[24963491553] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[24964807956] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[24968422347] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[24970086669] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=26 len=70
[24971831247] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[24973098480] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[24974332680] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[24976102635] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[24979801572] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[24981396693] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP RST
[24985155723] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=27 len=70
[24986796879] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[24989260296] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[24991463277] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[24996985002] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[24999339255] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP RST
[25000125216] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=28 len=70
[25001665326] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[25004326116] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[25006100295] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[25015924098] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[25064337771] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25071239688] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[25123676886] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[25126108227] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:33964 on listener 1
[25128630648] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[25138048452] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[25164009321] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25179915288] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=6
[25188949434] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 40 (user thread) assigned to CPU 1
[25190471493] [INFO] [anther] [CPU1] anther: Thread spawned TID=40 for conn_handle=6
T:5EF0 [25230231213] [INFO] [anther] [CPU1] anther: Worker thread TID=40 starting for conn_handle=6
[25235486133] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=831 watches=19 history=1024 journal=1024 symbols=366 drops=0
[25264354203] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=39, our_read=40)
[25266090168] [INFO] [anther] [CPU1] anther: Worker TID=40 connected to netd OK
[25496839302] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[25813129320] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[26127446202] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[26511928740] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[26533166385] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=29 len=124
[26535266934] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 114 byte frame (118 encoded) to netd rx_port=25
[26537617821] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (118 bytes sent)
[26864783220] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 114 bytes
[26891162628] [INFO] [anther] [CPU1] anther: Worker TID=39 got first 60 bytes on conn_handle=5
[26892790881] [INFO] [anther] [CPU1] anther: GET /health Http11
[27053673999] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=866 watches=19 history=1024 journal=1024 symbols=367 drops=0
[27400638210] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[27520910142] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[27523114839] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[27531859542] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[27540426903] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[27548233680] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=30 len=70
[27549810915] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[27552562092] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27871959918] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[28340195070] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=31 len=124
[28342071219] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 114 byte frame (118 encoded) to netd rx_port=25
[28344302019] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (118 bytes sent)
[28521097506] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2429 ops=1 watches=19
[28531259823] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[28961243916] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=892 watches=19 history=1024 journal=1024 symbols=367 drops=0
[28972847343] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[29129223981] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[29160986283] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[29165388945] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[29177730648] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=70
[29179780740] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[29182772553] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[29227478346] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[29270099925] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[29403918489] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[29760548796] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[29776853700] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30255072276] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30342654012] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[31218423093] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=962 watches=19 history=1024 journal=1024 symbols=413 drops=0
[32675572017] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[32786400801] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[33112056912] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[33113571843] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[33142877328] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[33167822193] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1027 watches=19 history=1024 journal=1024 symbols=451 drops=0
[33623868300] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[33686034756] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[33687367626] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[33688644363] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[33690223116] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[33697174236] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[33698258253] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[33699424308] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[33700628148] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=235 pred=0 subj_lo=0
[33707337774] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[33708483369] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[33709740636] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[33711033774] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=453 pred=0 subj_lo=0
[33721524012] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[33722656836] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[33724070622] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[33725439330] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=452 pred=0 subj_lo=0
[33733371738] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[33734485653] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[33735651114] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[33737024013] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[33744888210] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[34886501430] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1058 watches=24 history=1024 journal=1024 symbols=454 drops=0
[36612966093] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1078 watches=24 history=1024 journal=1024 symbols=454 drops=0
[38067021597] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=124
[38069181579] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 114 byte frame (118 encoded) to netd rx_port=25
[38071811250] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (118 bytes sent)
[38513169420] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=1111 watches=24 history=1024 journal=1024 symbols=454 drops=0
[40286324562] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=1133 watches=24 history=1024 journal=1024 symbols=454 drops=0
[41371503657] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[41373288528] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[41375508768] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[41380975218] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=70
[41382725538] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[41385430680] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[41880287064] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=16000 nodes=1151 watches=24 history=1024 journal=1024 symbols=454 drops=0
[43616312751] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=17000 nodes=1176 watches=24 history=1024 journal=1024 symbols=454 drops=0
[44371491120] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 114 bytes
[44388702798] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 60 bytes on conn_handle=4
[44390640294] [INFO] [anther] [CPU1] anther: GET /health Http11
[44432399616] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[44436080007] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[44561733150] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[44564387769] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[44570252331] [INFO] [anther] [CPU1] anther: Worker TID=39 got first 4 bytes on conn_handle=5
[44575641132] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[44577114648] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[44579299248] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[44655345735] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=70
[44657028603] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[44659231056] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[45491621208] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=18000 nodes=1198 watches=24 history=1024 journal=1024 symbols=454 drops=0
[45748671573] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[45783936594] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[45785867655] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[45788190591] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[45789630315] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[45792181974] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[45793895202] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[47196829647] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=19000 nodes=1220 watches=24 history=1024 journal=1024 symbols=454 drops=0
[48066928338] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=3453 ops=1 watches=24
[49214461560] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=20000 nodes=1247 watches=24 history=1024 journal=1024 symbols=454 drops=0
[51162941412] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=21000 nodes=1271 watches=24 history=1024 journal=1024 symbols=454 drops=0
[53078361633] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=22000 nodes=1293 watches=24 history=1024 journal=1024 symbols=454 drops=0
[55280948646] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=23000 nodes=1324 watches=24 history=1024 journal=1024 symbols=454 drops=0
[57455276703] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=24000 nodes=1348 watches=24 history=1024 journal=1024 symbols=454 drops=0
[59399236380] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=25000 nodes=1368 watches=24 history=1024 journal=1024 symbols=454 drops=0
[60056761941] [INFO] [anther] [CPU1] anther: Worker TID=40 got first 2 bytes on conn_handle=6
[61785410709] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=26000 nodes=1407 watches=24 history=1024 journal=1024 symbols=454 drops=0
[63855330033] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=27000 nodes=1429 watches=24 history=1024 journal=1024 symbols=454 drops=0
[64100250357] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[65924260248] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=28000 nodes=1449 watches=24 history=1024 journal=1024 symbols=454 drops=0
[68032641072] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[68036605989] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=2 (16384b)
[68039492037] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[68041710132] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[68645122491] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=29000 nodes=1496 watches=24 history=1024 journal=1024 symbols=454 drops=0
[70644578565] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=30000 nodes=1516 watches=24 history=1024 journal=1024 symbols=454 drops=0
[71509630698] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 2 bytes on conn_handle=4
[72725622552] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=31000 nodes=1536 watches=24 history=1024 journal=1024 symbols=454 drops=0
[75079788894] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=32000 nodes=1567 watches=24 history=1024 journal=1024 symbols=454 drops=0
[77721248286] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=33000 nodes=1593 watches=24 history=1024 journal=1024 symbols=454 drops=0
[80930590488] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[80933800860] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[80938255629] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[80947232454] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[80950254759] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[80954055930] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[82366749003] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[82369611027] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[82372299009] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[82374671346] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[82377248811] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[82382392719] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[82383756213] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[82386043410] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=124
[82388146665] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 114 byte frame (118 encoded) to netd rx_port=25
[82393095543] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (118 bytes sent)
[82397031618] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 114 bytes
[82535055900] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[82538245515] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[84243630933] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[84248181567] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[84253245945] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[84255580596] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[84258984546] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP RST
[84261375264] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[93841742511] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=34000 nodes=1619 watches=24 history=1024 journal=1024 symbols=454 drops=0
[95171725407] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[95179847334] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:33982 on listener 1
[95185273491] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[95196107259] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=7
[95209135626] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 41 (user thread) assigned to CPU 1
[95212660686] [INFO] [anther] [CPU1] anther: Thread spawned TID=41 for conn_handle=7
T:5EF0 [95262276582] [INFO] [anther] [CPU1] anther: Worker thread TID=41 starting for conn_handle=7
[95381478093] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=41, our_read=42)
[95385443967] [INFO] [anther] [CPU1] anther: Worker TID=41 connected to netd OK
[95958078909] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[95968099887] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[95989738218] [INFO] [anther] [CPU1] anther: Worker TID=41 got first 60 bytes on conn_handle=7
[95992331490] [INFO] [anther] [CPU1] anther: GET /health Http11
[97192099620] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[97197922800] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[97225281120] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[97228700778] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[97233049815] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[97379558265] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=35000 nodes=1648 watches=24 history=1024 journal=1024 symbols=454 drops=0
[97594895673] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[97614573507] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[97621505883] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[97639542132] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[97642921530] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[97647298980] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[98282743614] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[99485987271] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=4477 ops=1 watches=24
[100540200387] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=36000 nodes=1672 watches=24 history=1024 journal=1024 symbols=454 drops=0
[103161701808] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=37000 nodes=1696 watches=24 history=1024 journal=1024 symbols=454 drops=0
[106230397284] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=38000 nodes=1735 watches=24 history=1024 journal=1024 symbols=454 drops=0
[109128352344] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=39000 nodes=1757 watches=24 history=1024 journal=1024 symbols=454 drops=0
[111962978688] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=40000 nodes=1783 watches=24 history=1024 journal=1024 symbols=454 drops=0
[114694440135] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=41000 nodes=1812 watches=24 history=1024 journal=1024 symbols=454 drops=0
[116978958228] [INFO] [anther] [CPU1] anther: Worker TID=41 got first 2 bytes on conn_handle=7
[118215168753] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=42000 nodes=1840 watches=24 history=1024 journal=1024 symbols=454 drops=0
[120516764115] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[120519454308] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[120522312273] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[120568439409] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[120576112437] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[120586257198] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[120606665784] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[120611016966] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[120618300891] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[120637507386] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=124
[120641451117] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 114 byte frame (118 encoded) to netd rx_port=25
[120645540279] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (118 bytes sent)
[121114813248] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=43000 nodes=1860 watches=24 history=1024 journal=1024 symbols=454 drops=0
[123536839338] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=44000 nodes=1880 watches=24 history=1024 journal=1024 symbols=454 drops=0
[123549094185] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[123563886930] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[123568759380] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:33972 on listener 1
[123575015058] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[123602105187] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=8
[123615911265] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 42 (user thread) assigned to CPU 1
[123618692538] [INFO] [anther] [CPU1] anther: Thread spawned TID=42 for conn_handle=8
T:5EF0 [123676111416] [INFO] [anther] [CPU1] anther: Worker thread TID=42 starting for conn_handle=8
[123738686577] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=43, our_read=44)
[123755770644] [INFO] [anther] [CPU1] anther: Worker TID=42 connected to netd OK
[126850762269] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=45000 nodes=1913 watches=24 history=1024 journal=1024 symbols=454 drops=0
[129187811148] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=46000 nodes=1933 watches=24 history=1024 journal=1024 symbols=454 drops=0
[129762423351] [INFO] [anther] [CPU1] anther: Worker TID=42 got first 2 bytes on conn_handle=8
[132178261842] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=2
[132236078106] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=47000 nodes=1959 watches=24 history=1024 journal=1024 symbols=454 drops=0
[132630109128] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[133187288982] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[136184245032] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=48000 nodes=2004 watches=24 history=1024 journal=1024 symbols=454 drops=0
[138872167038] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=49000 nodes=2028 watches=24 history=1024 journal=1024 symbols=454 drops=0
[141694959549] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=50000 nodes=2048 watches=24 history=1024 journal=1024 symbols=454 drops=0
[145396618818] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=51000 nodes=2085 watches=24 history=1024 journal=1024 symbols=454 drops=0
[148313546766] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=52000 nodes=2107 watches=24 history=1024 journal=1024 symbols=454 drops=0
[149233155852] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=5501 ops=1 watches=24
[151524982224] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=53000 nodes=2131 watches=24 history=1024 journal=1024 symbols=454 drops=0
[160077994395] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=124
[160082194206] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 114 byte frame (118 encoded) to netd rx_port=25
[160087450479] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (118 bytes sent)
[164135560677] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 114 bytes
[165257689146] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[165269477439] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[165290539392] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=74
[165293823849] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=25
[165297453486] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[165326674722] [INFO] [anther] [CPU1] anther: OTHER /health Http11
[166534483599] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[166538811681] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[166542587211] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[166568608503] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 219 bytes - TCP ACK
[166576893615] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 219 bytes
[166598792976] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=70
[166603237614] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[166609597737] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[167105666310] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=54000 nodes=2164 watches=24 history=1024 journal=1024 symbols=454 drops=0
[167300738055] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[167338567968] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 77 bytes - TCP ACK
[167342418639] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 77 bytes
[167368562955] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=70
[167373834936] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[167380970394] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[167382413715] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[167404586745] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=70
[167409979308] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[167415936600] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[167441983632] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[167443566840] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=70
[167448231291] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[167450626992] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[167456146836] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[167465795244] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[167471621724] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[167490239895] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[167513154534] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=70
[167517752655] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[167524230885] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[167527513263] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[167549619138] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[167555609100] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[167557907220] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=70
[167562055683] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[167567439237] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[167570925852] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[167586684573] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=25 len=70
[167591227089] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[167596458843] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[167600419437] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[167606920899] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[167612215287] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[167614433316] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[167622802182] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[167901992643] [INFO
```
</details>
