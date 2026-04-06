# ❌ Scenario: Serve telnet connection

> Last run: 2026-04-05 19:06:17

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the telnet server is ready | ✅ | 6783ms | - - - |
| 2 | When I connect to the telnet server and send "match (n) return n;" | ✅ | 3172ms | - [📜](./02/serial.log) - |
| 3 | Then the telnet response should contain "node(" | ❌ | 1012ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12239904963] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[12245706924] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[12249416157] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[12251485059] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[12252702297] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[12253390479] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[12254129811] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[12254729784] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[12255386583] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=29168
[12256039851] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=33264
[12256717011] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[12257393643] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[12258132678] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[12258828219] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[12259638171] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[12260270550] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[12260954178] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[12261581871] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[12262217880] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[12262845804] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[12263433468] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[12264028623] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[12264630213] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[12265279719] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[12266009679] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[12266630475] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[12267240645] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[12267897708] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[12268496130] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[12269122272] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[12269741715] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[12270392442] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[12271064553] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[12271739370] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=172560
[12272374389] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[12273086034] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[12273791574] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[12274506981] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[12275207505] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[12275956671] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[12276724350] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[12277485891] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[12278872980] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[12280376559] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[12281155062] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[12281761503] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[12282306663] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[12282889410] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[12283467702] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[12284001873] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[12284528058] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[12285072987] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[12285624912] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[12286179180] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7795e000 (Usable)
[12286775787] [INFO] [kernel::memory] [CPU0]   [11] 0x7795e000 - 0x779c2000 (Reserved)
[12287388135] [INFO] [kernel::memory] [CPU0]   [12] 0x779c2000 - 0x779c3000 (Other)
[12287947848] [INFO] [kernel::memory] [CPU0]   [13] 0x779c3000 - 0x779c4000 (Reserved)
[12288574551] [INFO] [kernel::memory] [CPU0]   [14] 0x779c4000 - 0x779c5000 (Other)
[12289170564] [INFO] [kernel::memory] [CPU0]   [15] 0x779c5000 - 0x779c6000 (Reserved)
[12289750506] [INFO] [kernel::memory] [CPU0]   [16] 0x779c6000 - 0x779c7000 (Other)
[12290321241] [INFO] [kernel::memory] [CPU0]   [17] 0x779c7000 - 0x779c8000 (Reserved)
[12290956029] [INFO] [kernel::memory] [CPU0]   [18] 0x779c8000 - 0x77a53000 (Other)
[12291670182] [INFO] [kernel::memory] [CPU0]   [19] 0x77a53000 - 0x77a54000 (Reserved)
[12292316553] [INFO] [kernel::memory] [CPU0]   [20] 0x77a54000 - 0x77ed5000 (Other)
[12292938999] [INFO] [kernel::memory] [CPU0]   [21] 0x77ed5000 - 0x77ed6000 (Reserved)
[12293519964] [INFO] [kernel::memory] [CPU0]   [22] 0x77ed6000 - 0x77ff7000 (Other)
[12294134688] [INFO] [kernel::memory] [CPU0]   [23] 0x77ff7000 - 0x77ff8000 (Reserved)
[12294717336] [INFO] [kernel::memory] [CPU0]   [24] 0x77ff8000 - 0x787f8000 (Other)
[12295284441] [INFO] [kernel::memory] [CPU0]   [25] 0x787f8000 - 0x787f9000 (Reserved)
[12295881840] [INFO] [kernel::memory] [CPU0]   [26] 0x787f9000 - 0x788ba000 (Other)
[12296440860] [INFO] [kernel::memory] [CPU0]   [27] 0x788ba000 - 0x788bb000 (Reserved)
[12297017469] [INFO] [kernel::memory] [CPU0]   [28] 0x788bb000 - 0x788e6000 (Other)
[12297573090] [INFO] [kernel::memory] [CPU0]   [29] 0x788e6000 - 0x788e7000 (Reserved)
[12298182699] [INFO] [kernel::memory] [CPU0]   [30] 0x788e7000 - 0x788ec000 (Other)
[12298767591] [INFO] [kernel::memory] [CPU0]   [31] 0x788ec000 - 0x788ed000 (Reserved)
[12299348259] [INFO] [kernel::memory] [CPU0]   [32] 0x788ed000 - 0x788f2000 (Other)
[12299955624] [INFO] [kernel::memory] [CPU0]   [33] 0x788f2000 - 0x788f3000 (Reserved)
[12300534279] [INFO] [kernel::memory] [CPU0]   [34] 0x788f3000 - 0x7891f000 (Other)
[12301092672] [INFO] [kernel::memory] [CPU0]   [35] 0x7891f000 - 0x78921000 (Reserved)
[12301666575] [INFO] [kernel::memory] [CPU0]   [36] 0x78921000 - 0x7892a000 (Other)
[12302358981] [INFO] [kernel::memory] [CPU0]   [37] 0x7892a000 - 0x7892c000 (Reserved)
[12302939451] [INFO] [kernel::memory] [CPU0]   [38] 0x7892c000 - 0x78934000 (Other)
[12303496326] [INFO] [kernel::memory] [CPU0]   [39] 0x78934000 - 0x78935000 (Reserved)
[12304111182] [INFO] [kernel::memory] [CPU0]   [40] 0x78935000 - 0x7893f000 (Other)
[12304668420] [INFO] [kernel::memory] [CPU0]   [41] 0x7893f000 - 0x78940000 (Reserved)
[12305261595] [INFO] [kernel::memory] [CPU0]   [42] 0x78940000 - 0x7894d000 (Other)
[12305861502] [INFO] [kernel::memory] [CPU0]   [43] 0x7894d000 - 0x7894f000 (Reserved)
[12306438738] [INFO] [kernel::memory] [CPU0]   [44] 0x7894f000 - 0x7895d000 (Other)
[12306994227] [INFO] [kernel::memory] [CPU0]   [45] 0x7895d000 - 0x7895e000 (Reserved)
[12307568064] [INFO] [kernel::memory] [CPU0]   [46] 0x7895e000 - 0x7896a000 (Other)
[12308131836] [INFO] [kernel::memory] [CPU0]   [47] 0x7896a000 - 0x7896b000 (Reserved)
[12308722041] [INFO] [kernel::memory] [CPU0]   [48] 0x7896b000 - 0x7896f000 (Other)
[12309324984] [INFO] [kernel::memory] [CPU0]   [49] 0x7896f000 - 0x78970000 (Reserved)
[12309904101] [INFO] [kernel::memory] [CPU0]   [50] 0x78970000 - 0x78980000 (Other)
[12310506978] [INFO] [kernel::memory] [CPU0]   [51] 0x78980000 - 0x78981000 (Reserved)
[12311085369] [INFO] [kernel::memory] [CPU0]   [52] 0x78981000 - 0x78a11000 (Other)
[12311640792] [INFO] [kernel::memory] [CPU0]   [53] 0x78a11000 - 0x78a12000 (Reserved)
[12312233736] [INFO] [kernel::memory] [CPU0]   [54] 0x78a12000 - 0x78a1d000 (Other)
[12312786882] [INFO] [kernel::memory] [CPU0]   [55] 0x78a1d000 - 0x78a1e000 (Reserved)
[12313358871] [INFO] [kernel::memory] [CPU0]   [56] 0x78a1e000 - 0x78a43000 (Other)
[12313913271] [INFO] [kernel::memory] [CPU0]   [57] 0x78a43000 - 0x78a44000 (Reserved)
[12314524629] [INFO] [kernel::memory] [CPU0]   [58] 0x78a44000 - 0x78a50000 (Other)
[12315082263] [INFO] [kernel::memory] [CPU0]   [59] 0x78a50000 - 0x78a51000 (Reserved)
[12315738996] [INFO] [kernel::memory] [CPU0]   [60] 0x78a51000 - 0x78a5e000 (Other)
[12316296267] [INFO] [kernel::memory] [CPU0]   [61] 0x78a5e000 - 0x78a5f000 (Reserved)
[12316868157] [INFO] [kernel::memory] [CPU0]   [62] 0x78a5f000 - 0x78aa7000 (Other)
[12317422029] [INFO] [kernel::memory] [CPU0]   [63] 0x78aa7000 - 0x78aa8000 (Reserved)
[12318280887] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12558776538] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485750 free frames
[12569921166] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12575157738] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12576528063] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12577398042] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12581639070] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12583379886] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12584462781] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12585140469] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12585830994] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12586496538] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12587452515] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12588393708] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12589093176] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12589762812] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12590427069] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12591091227] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12592242993] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12593246391] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12594020373] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12595575597] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12596561901] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12597559095] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12599194773] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12600744882] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12601515993] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12602066895] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12602812860] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12977988804] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12979008075] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12982412025] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12983314773] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12984109743] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12985633716] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12998360034] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12999712968] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[13000510743] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[13002260700] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[13002804573] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[13005252744] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[13013061138] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[13014773805] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[13029538335] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[13030207575] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[13046445720] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[13047046056] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[13049044470] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[13050307215] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[13051380804] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[13053638235] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[13054459869] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[13089256422] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62316200 ticks/sec), init_cnt=623162 for 100Hz
[13090688754] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[13091495307] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[13092658920] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[13098286113] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[13128420030] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[13129356834] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[13132251726] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[13134331980] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[13135566048] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[13138279209] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[13139537103] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[13158584076] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[13160519988] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[13161377691] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[13162251663] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[13162904931] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[13164056961] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[13164852822] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[13190296449] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[13192007763] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[13193768082] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[13194851043] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[13196636244] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[13197434217] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[13198841040] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[13199636835] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[13206966960] [INFO] [kernel::root] [CPU0] Spawning Root service...
[13207997253] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[13210505055] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[13211851257] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[13217895306] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[13220262528] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[13221316152] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[13222752708] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[13223985984] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[13225167846] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[13237590234] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[13240497831] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[13241626101] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[13242405726] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13263136986] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13282365657] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13284923091] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13289760198] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13291630440] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13293768180] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13296071877] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13299747912] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13300635843] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13302090483] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13310837232] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13312908147] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13315737501] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13318347768] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13323577476] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13324545993] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13337945148] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13338993030] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13346265669] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13347086247] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13376642697] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13377623391] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13784693010] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[14266411632] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[14294149287] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[14328460608] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[15696679746] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=446 watches=0 history=960 journal=768 symbols=94 drops=0
[16520671266] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[16614614379] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[16615652592] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[16711169046] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[16783420566] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[16817078421] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[16818056574] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[16818695817] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[16823354064] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[16840569768] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[16861659309] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[16863799788] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[16925790750] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[16951856097] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[16952988591] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[16960911363] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[16996750881] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[17027123487] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[17031907431] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[17033266371] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[17131805163] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[17134473015] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[17226336600] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[17289743856] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[17302634613] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[17307007212] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[17309503761] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[17337834822] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[17379557481] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[17384888169] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[17385881568] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[17386713531] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[17387654328] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[17388336108] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[17389018152] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[17389671519] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[17390326239] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[17390971224] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=29168
[17391740784] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=33264
[17392370226] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[17393011317] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[17393710026] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[17394706296] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[17395929804] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[17396715204] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[17397378405] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[17398032300] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[17398689165] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[17399456877] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[17400082194] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[17400770541] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[17401503537] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[17402241186] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[17402918577] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[17403594021] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[17404295337] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[17404987611] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[17405616228] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[17406257715] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[17406905010] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[17407576461] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[17408213361] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[17408854419] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=172560
[17409483069] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[17410216923] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[17410972128] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[17411722119] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[17412459471] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[17413226292] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[17413986645] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[17414746899] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[17416243779] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[17418072738] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[17418983076] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17427424575] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[17444253387] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[17450611959] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[17460670755] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[17461484469] [CONTRACT] [kernel] [CPU0] Spawning init process...
[17463590529] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[17466357777] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[17467185912] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [17473557321] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[17474358627] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013072 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[17489806422] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[17493797046] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[17498291646] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[17499565578] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[17510060601] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[17515860516] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[17520162792] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[17521394913] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[17525794506] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[17534350152] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[17535631245] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[17540230785] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[17541415947] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[17542589823] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[17543680539] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[17547116631] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[17548235991] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[17549605689] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[17551097949] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[17552565624] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[17553890244] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[17558392863] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[17601090738] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[17609733141] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[17614700631] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[17619847245] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[17623844667] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[17628695040] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[17634751695] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[17640060471] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[17645067000] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[17649903480] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[17655312312] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[17661050946] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[17666147928] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[17672323185] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[17676999285] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[17682384984] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[17687321784] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[17692793547] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[17698410279] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[17703014472] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[17707573488] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[17712402114] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[17717182131] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[17722082433] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[17727459057] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[17732315667] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[17737283652] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[17742092445] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[17746739043] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[17751607995] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[17755235190] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[17758737117] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[17763775161] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[17767175712] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/telnetd
[17770235505] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[17775293085] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[17780134185] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[17785128768] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[17791066458] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[17797039656] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[17802421461] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[17808039414] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[17811187020] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[17831453475] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[17968815843] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[17975976117] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[17976944436] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17978838471] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17983696599] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17985062832] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17993651907] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[17998929696] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18000343746] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18002039187] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078608 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[18007184349] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[18011485767] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[18012473919] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18014329674] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18018897831] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18020831697] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18028209144] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[18031844688] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[18033015198] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[18034919694] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[18036670443] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144144 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[18044227740] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[18045994791] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[18048093459] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18052450119] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 02:06:49 = 1775441209 unix_secs
[18054294984] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775441209, mono_ns=9026899992, offset=1775441199973100008ns
[18055831233] [INFO] [rtc_cmos] [CPU1] System clock anchored
[18068213394] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[18105017832] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[18122670159] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[18123898782] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18125879244] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18132542142] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18135145809] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[18143330040] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[18147752964] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[18151421970] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18153811137] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210832 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[18159261714] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[18163955271] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[18166337640] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[18167219763] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18169056807] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18176983605] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18179293935] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18187181430] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[18190486413] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[18193652169] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18194672100] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18195989163] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277296 RFLAGS_BEFORE=130 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[18201895833] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[18203623086] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[18209704788] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[18211174509] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[18212954562] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[18213914994] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[18215913441] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[18233408457] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[18235080435] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[18754211421] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18759875145] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[18763256358] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[18765218835] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[18770025318] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[18771650172] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[18773930637] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[18776160381] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[18777582813] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[18780854697] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[18782350125] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[18784223238] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[18785927622] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[18787112850] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[18788320056] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[18789569172] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[18795118287] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[18796456932] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18798872499] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18805991919] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18808620270] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18816007287] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[18818901519] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[18820542906] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[18823389849] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352768 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[18831393801] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[18840555756] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[18874034553] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[18885299136] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[18887373945] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[18893185245] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[18902145702] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[18903637302] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[18908857209] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[18910006566] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[18910979472] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[18911965446] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[18912947955] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[18914534463] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[18915516642] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[18916410447] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[18921197031] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[18923411298] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[18925866267] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[18928309950] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[18932951664] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[18933848604] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18935686011] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18939990003] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18941806323] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18949094241] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[18952179048] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[18954613425] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[18955549140] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18957244119] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18962164023] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18963484221] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18970534638] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[18973577997] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0107648
[18974696532] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[18976269510] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500224 RFLAGS_BEFORE=130 CR3_BEFORE=68505600 fs_base=0 gs_base=18446744071564586640
[18979954488] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[18980817768] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18982519776] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18983446251] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[18985188057] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[18988500498] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18989489706] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[18991181055] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[18992076147] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18993189864] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[18995255136] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[18996952722] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[18998419539] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[19000546719] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[19001754123] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[19002953376] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[19004760753] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[19005653370] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[19007402040] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[19008955911] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[19010919939] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[19012566573] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434144 RFLAGS_BEFORE=130 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[19018357512] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[19383482976] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0107648
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[19385365527] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583008 RFLAGS_BEFORE=134 CR3_BEFORE=68620288 fs_base=0 gs_base=18446744071564586576
[19388059812] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[19391252001] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[19393947408] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[19403075637] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19406201133] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[19407340623] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19409188557] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19415866800] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[19418197326] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[19427501016] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[19431066072] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[19433547474] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[19435852656] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[19436662311] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19438440318] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19455712980] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[19458695223] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[19459851906] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19462082310] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[19464656838] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[19479492318] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[19585571412] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[20367288876] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[20370210795] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a670
[20371486014] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20372974248] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716768 RFLAGS_BEFORE=130 CR3_BEFORE=68882432 fs_base=0 gs_base=18446744071564586640
[20376625368] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[20378118816] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369649456 RFLAGS_BEFORE=134 CR3_BEFORE=68747264 fs_base=0 gs_base=18446744071564586608
[20381174187] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[20382878109] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[20383607376] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[20385862233] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[20387791908] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[20388741912] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[20391152562] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[20393045871] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[20395008315] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[20396002110] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[20397562779] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[20400059064] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[20403913134] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering interrupt-driven loop
[20415893982] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[20423301789] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[20424928854] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[20426582220] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[20427937200] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[20429912217] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[20431465923] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[20433800574] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4c33000
[20435437671] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[20437684674] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[20439764862] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[20441146473] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[20451896586] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[20461492458] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[20471606595] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[20474210856] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[20476500429] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[20478152838] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[20479587810] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[20481156597] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[20482756008] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[20484081981] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[20485180518] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[20490365709] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=15, read=16)
[20495934426] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=17, read=18)
[20500841196] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[20503708599] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[20506226202] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[20507064864] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20508602004] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20509340511] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20578072878] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[20602830171] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[20610402483] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[20614070829] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0102438
[20615344728] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20617318623] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369914720 RFLAGS_BEFORE=134 CR3_BEFORE=80011264 fs_base=0 gs_base=18446744071564586576
[20622722439] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[20623762995] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20625383790] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[20626314522] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20628088404] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[20631924918] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20633092524] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20634540729] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20654041749] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20657490909] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[20664723915] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[20667530103] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[20669628045] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[20670383250] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20671921149] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20677598964] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20680119537] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20687673633] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[20690462595] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010bc50
[20692363527] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20694147738] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[20695140840] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20696474865] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370046880 RFLAGS_BEFORE=134 CR3_BEFORE=80945152 fs_base=0 gs_base=18446744071564586640
[20699734110] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20701670583] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[20703115059] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[20708122776] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20711828676] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[20719276611] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[20722163979] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010bc50
[20723550606] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20725418010] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113280 RFLAGS_BEFORE=134 CR3_BEFORE=81072128 fs_base=0 gs_base=18446744071564586576
[20729630130] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[20730943200] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20733230496] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20746135641] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[20749357365] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[20757207240] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[20759879217] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[20762101140] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[20762805624] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20764232445] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20790795036] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20795410581] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20797014810] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[20805963453] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[20810531247] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
[20813667369] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010d548tlas-based IPC)
[20842768452] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[20848105740] [INFO] [fontd] [CPU3] FONTD: Service node created, req=19, resp=22
[20848934898] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20855309772] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20857648152] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20859952773] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20877132144] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[20879439405] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[20884774746] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[20892407580] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[20896113777] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20897948181] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20904526896] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20906384532] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20908064496] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20910728652] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f10c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20912422773] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370330832 RFLAGS_BEFORE=130 CR3_BEFORE=81649664 fs_base=0 gs_base=18446744071564586576
[20919458571] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[20921459130] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[20925672339] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20927099919] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20928577923] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20930066454] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=229 pred=0 subj_lo=0
[20941254906] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20942653182] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20944329021] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20946120030] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[20950996836] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20952352179] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[20953558263] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20955165627] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20956884696] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[20966680119] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[20974312656] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[20976910251] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[20980581303] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20983817646] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20985663171] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369980800 RFLAGS_BEFORE=130 CR3_BEFORE=80683008 fs_base=0 gs_base=18446744071564586608
[20990464935] [INFO] [nectar] [CPU2] NECTAR: Started.
[20992237893] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010bc50
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20993986530] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370179808 RFLAGS_BEFORE=130 CR3_BEFORE=81219584 fs_base=0 gs_base=18446744071564586608
[20998020219] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[20999020185] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[20999790372] [INFO] [fontd] [CPU3] FONTD: Service ready
[21000863499] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f10c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21002368266] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370398272 RFLAGS_BEFORE=130 CR3_BEFORE=81858560 fs_base=0 gs_base=18446744071564586608
[21008008857] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[21009977934] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[21011209791] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21012634896] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[21019419861] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[21038225670] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[21049114383] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21050382771] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1250 backend=VirtIO-GPU
[21051449595] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21052687425] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[21053477940] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21054822888] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21056211957] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[21057678675] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21065545281] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21066774135] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21068152182] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21069517689] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[21076508541] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21077751618] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21078967371] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21080370894] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=244 subj_lo=0
[21087312147] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21088646799] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21090102660] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21091667817] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=245 subj_lo=0
[21098626659] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21099837099] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21100955634] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21102249300] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=246 subj_lo=0
[21114481245] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21115855761] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21116779761] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21118078245] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21119118108] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21120260700] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21121630134] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=247 pred=0 subj_lo=0
[21142078089] [INFO] [netd] [CPU3] NETD: Driver TX port=15, RX port=18, link_up=true, mtu=1500
[21144590775] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[21149293110] [INFO] [netd] [CPU3] NETD: Created socket API port (write=23, read=24)
[21161063319] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[21183601230] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[21187323267] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=25, resp=28
[21188546709] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[21203463270] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[21206835111] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[21208061490] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[21214015812] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[21220440384] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[21229721700] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[21232370676] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[21234513498] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[21235278735] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21236822607] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21241894377] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21243928761] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[21249930207] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[21252673794] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[21256308645] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[21257519217] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db538
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e2
[21260035335] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370538128 RFLAGS_BEFORE=134 CR3_BEFORE=82509824 fs_base=0 gs_base=18446744071564586640
[21263895180] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[21265338501] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1298
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[21267963750] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[21268871151] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370603664 RFLAGS_BEFORE=134 CR3_BEFORE=83726336 fs_base=0 gs_base=18446744071564586576
[21277401684] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=15, RX port=18
[21280870743] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[21293071800] [INFO] [echo] [CPU1] echo: starting up
[21295878945] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[21301764726] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[21312648060] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[21317380128] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=29, our_read=30)
[21320828463] [INFO] [anther] [CPU1] anther: Connected to network stack
[21321792492] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[21324446253] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[21343959120] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[21346478109] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[21368044698] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[21369851085] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[21371866659] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[21372892629] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21374644533] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[21375687300] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21381081876] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21383027688] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[21385223706] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[21390764373] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[21393119748] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[21394964217] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[21396605175] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[21398405127] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[21399236265] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21401192076] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21403819866] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[21405282327] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[21409305324] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21410271399] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[21413477118] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[21418975116] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[21420721872] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[21422475294] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[21423604719] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[21427439583] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[21428336952] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f2510
[21429610917] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21431527062] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370759312 RFLAGS_BEFORE=134 CR3_BEFORE=85131264 fs_base=0 gs_base=18446744071564586640
[21435673182] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[21436728753] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21438683079] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21442191870] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[21443686803] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[21452038872] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[21455265282] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[21457071636] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/telnetd'
[21459715662] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/telnetd
[21460841358] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21462518715] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[21463749549] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21465014274] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f2510
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21466778388] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370824848 RFLAGS_BEFORE=134 CR3_BEFORE=85278720 fs_base=0 gs_base=18446744071564586576
[21472021098] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[21484365408] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[21499013646] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=221000 exec=false
[21508171773] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=229000 exec=false
[21519854895] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 32 (user task/process) assigned to CPU 2
[21524114766] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=32)
[21525251946] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[21526628376] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[21528076350] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[21530100702] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[21538970145] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[21548170941] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f2510
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21551058144] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=32 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370890384 RFLAGS_BEFORE=134 CR3_BEFORE=85381120 fs_base=0 gs_base=18446744071564586608
[21560072226] [INFO] [telnetd] [CPU2] telnetd: starting on port 2323
[21561823338] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[21574432143] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[21578267271] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[21581622909] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[21590127702] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[21593431365] [INFO] [telnetd::net_client] [CPU2] telnetd: connected to socket API (netd=23, write=31, read=32)
[21600998628] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[21606186360] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[21608842827] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[21640055712] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[21642438510] [INFO] [bloom] [CPU3] bloom: creating surface...
[21643341786] [INFO] [bloom] [CPU3] bloom: surface created!
[21644293110] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[21652452063] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[21660194391] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[21661604712] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1263
[21662409021] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[21663682491] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[21678061119] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[21681810183] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=33)
[21683147343] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[21684662736] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
[21688828590] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[21692316525] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[21697163367] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[21700999650] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[21703403106] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 2323 with backlog 64
[21706114155] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 2323, handle=2
[21708465207] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[21710537541] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=3
[21713542818] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
T:08C0 [21721434933] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[21730175808] [INFO] [telnetd] [CPU2] telnetd: listening on guest port 2323 (handle=2)
[21732407598] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[21741890049] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[21750272874] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[21758466312] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[21761243988] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[21763799739] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[21768216261] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[21769000803] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1263
T:0E20 T:0C90 T:F700 [21814744347] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[21824650320] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 37 (user thread) assigned to CPU 3
[21827401794] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=37 (priority=2)
[21830513925] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[21835202367] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[21842124546] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[21845084646] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
T:1870 [21854661114] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[21856706619] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[21868455906] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=278
[21870128379] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[21871277736] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[21872299746] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21873423990] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21874699671] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=278 subj_lo=0
[21889679625] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1280)
[21890983257] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[21905540052] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=77c80b059e863710)
[21907009245] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=247
[21908461344] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[21909260241] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[21910208892] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[21911542620] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[21912736065] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21914286339] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[21915393885] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21916995672] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=247 pred=0 subj_lo=0
[21933603912] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[21938562426] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=70
[21940493982] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[21942308124] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[21946327359] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[21947956569] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[21948828132] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[21953281614] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1281)
[21954616266] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[21969388155] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=284
[21970572492] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[21971617371] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[21972613872] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21974039109] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21975536352] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=284 subj_lo=0
[21995442315] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1282)
[21996964308] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[22001601699] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=74
[22003416567] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=17
[22006523385] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[22016967060] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[22020880827] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[22023184359] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[22031934144] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[22041613836] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=70
[22043780715] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[22047439524] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22052316792] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22059416775] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[22061251344] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[22064633052] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22078774278] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22083071934] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[22090389981] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[22407221562] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[22504791738] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[22518640683] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x1180c000
[22519940718] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[22521740835] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[22527167883] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=660 watches=13 history=1024 journal=1024 symbols=311 drops=0
[22588574250] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[22598998785] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[22603503912] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[22605479523] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[22607472261] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[22612653888] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[22625780562] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[22627180455] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[22628746602] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[22636047324] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[22799725773] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22800951690] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22802136621] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22803483351] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=306 pred=0 subj_lo=0
[22946543862] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22947847362] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22948917783] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22949988204] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=307 pred=0 subj_lo=0
[22961998257] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[22997331918] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[23016200955] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[23104166712] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[23280426312] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[23287244541] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[23291022414] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[23306179941] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 16384)
[23308509708] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=1 (16384b)
[23342216799] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[23520445443] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 29168 bytes, hash=790243a6d8f2a2ea)
[23726387124] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 33264 bytes, hash=91427bae3069f281)
[24022708248] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=ae36cd328bcc45e4)
[24217991655] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[24420911856] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[24500643519] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[24670867716] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[24729464397] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=732 watches=15 history=1024 journal=1024 symbols=353 drops=0
[24930305499] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[24937290081] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[24939170784] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[24940324398] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[24941428314] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[24942914601] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[24958376586] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[24959448327] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[24960627186] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[24962012229] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=353 pred=0 subj_lo=0
[24968340540] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[25211215953] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[25436984001] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[25623643221] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[25648426947] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[25650600690] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[25659356316] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[25660922661] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[25662815013] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[25695794949] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[25700564406] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=Established
[25702419633] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:38324 on listener 2
[25735474842] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[25737550542] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[25750886733] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 60 bytes - TCP ACK
[25758871281] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x121f5000
[25760334567] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[25761497487] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 60 bytes
[25770221400] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=85
[25771683663] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 75 byte frame (79 encoded) to netd rx_port=17
[25774092465] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (79 bytes sent)
[25912230465] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[25950539439] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 75 bytes
[25961996577] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 72 bytes - TCP ACK
[25971342441] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 72 bytes
[25987975662] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[25989563787] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[25991710371] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26276577525] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=0e57f834b3afbcb1)
[26567515128] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=781 watches=17 history=1024 journal=1024 symbols=363 drops=0
[26594606379] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26596237371] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26597675841] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26598976008] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=308 pred=0 subj_lo=0
[26629838697] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26634469191] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[26645998368] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP ACK
[26648211183] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[26670937656] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[26672916270] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[26675540892] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26859041649] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[26873483505] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26875562043] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 89 bytes - TCP ACK
[26903335074] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 89 bytes
[26924549619] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[26926566612] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[26930147706] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26938934616] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26941079418] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 63 bytes - TCP ACK
[26946715422] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 63 bytes
[26947433040] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[26970477633] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[26973417273] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[26976976158] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26979634704] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26980678263] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[26984236455] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 55 bytes - TCP ACK
[26988731286] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[27011536629] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 55 bytes
[27028149720] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[27030637590] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[27033179481] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27041245473] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[27044074662] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 55 bytes - TCP ACK
[27048434688] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 55 bytes
[27070438956] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[27072379785] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[27074466804] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27075468849] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[27078282495] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[27087989082] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[27100864956] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[27102900726] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[27105163734] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27110682951] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=c1e9a7b90a789e5c)
[27113846463] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[27118944897] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 55 bytes - TCP ACK
[27127753257] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 55 bytes
[27161656566] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[27163798167] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[27167492286] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27177883821] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[27203157630] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 55 bytes - TCP ACK
[27205328832] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 55 bytes
[27221982645] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[27223855065] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[27225628782] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=70
[27226704417] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[27228895287] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=309 pred=0 subj_lo=0
[27231652074] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[27235595442] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27237555939] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[27243547716] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[27249038982] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[27274536630] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=70
[27276579528] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[27280057365] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27293497110] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[27295217136] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[27302776578] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[27323042769] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=70
[27324589842] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[27327389265] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27580275129] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[27925720977] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[28277760060] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[28586649729] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[28734114024] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=842 watches=19 history=1024 journal=1024 symbols=366 drops=0
[29024485380] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[29369923143] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[29695358286] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[29698202028] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 55 bytes - TCP ACK
[29701345542] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 55 bytes
[29720457921] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=70
[29722203885] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[29724588531] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[29756634303] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[29758349445] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 59 bytes - TCP ACK
[29763657297] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 59 bytes
[29765790120] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[29788237545] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=70
[29790461745] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[29793597174] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30182999583] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[30219993012] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30221810421] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 57 bytes - TCP ACK
[30237521952] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 57 bytes
[30263673726] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=70
[30266480178] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[30270036159] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30282359976] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30284794287] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 57 bytes - TCP ACK
[30292418673] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 57 bytes
[30331012239] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=70
[30333041376] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[30336234885] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30339065988] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30341037177] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 105 bytes - TCP ACK
[30353347695] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 105 bytes
[30387352545] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=70
[30389532723] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[30392045706] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30397977720] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30401049030] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 61 bytes - TCP ACK
[30406252800] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 61 bytes
[30418630704] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=25 len=70
[30420475272] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[30423297333] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30424446558] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30710067564] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[30815433198] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=888 watches=19 history=1024 journal=1024 symbols=366 drops=0
[31068636588] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[31439766369] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[31813397583] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 172560 bytes, hash=7642f9046c0b65a2)
[31836032976] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2431 ops=1 watches=19
[32296288596] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[32361946980] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=26 len=70
[32364177978] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[32367516687] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[32603340495] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=920 watches=19 history=1024 journal=1024 symbols=367 drops=0
[33345922104] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[33400876971] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[33402747345] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[33404463576] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[33416104953] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[33429889944] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[33447927480] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=27 len=70
[33450010737] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[33452826198] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[33760441836] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[33762608715] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[33802525284] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[33828842916] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=28 len=70
[33832023984] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[33835305372] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[33842988663] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=29 len=70
[33845411919] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[33848811810] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[33913088847] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[34031902179] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[34034541552] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[34049038848] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: TCP_CLOSE handle=4 (initiating close)
[34055474970] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: GC removed explicitly closed TCP socket
[34321672209] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=941 watches=19 history=1024 journal=1024 symbols=367 drops=0
[34664847129] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[35097362025] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[35481875088] [INFO] [flytr
```
</details>
