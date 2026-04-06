# ❌ Scenario: Verifying system services via logs

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ❌ | 1000ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11346458079] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11351774643] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11355390849] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11357395797] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11358662139] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11359315044] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11359970061] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11360547000] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11361130077] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=29168
[11361737178] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=33264
[11362336359] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11362930821] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11363610423] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11364348072] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11365026651] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11365659525] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11366273193] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11366860362] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11367532836] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11368145316] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11368721958] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11369333481] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11369919033] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11370519996] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11371147920] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11371808184] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11372420994] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11373098649] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11373704100] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11374302258] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11374908369] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11375541012] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11376189396] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11376799731] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11377384557] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11378075973] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11378814183] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11379540183] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11380236582] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11380958787] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11381659971] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11382491967] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11383813947] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11385177540] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11385971421] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11386561461] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11387110614] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11387633037] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11388199746] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11388743157] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11389257759] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11389775364] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11390282046] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11390863737] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797a000 (Usable)
[11391407478] [INFO] [kernel::memory] [CPU0]   [11] 0x7797a000 - 0x779de000 (Reserved)
[11392081866] [INFO] [kernel::memory] [CPU0]   [12] 0x779de000 - 0x779df000 (Other)
[11392627521] [INFO] [kernel::memory] [CPU0]   [13] 0x779df000 - 0x779e0000 (Reserved)
[11393188059] [INFO] [kernel::memory] [CPU0]   [14] 0x779e0000 - 0x779e1000 (Other)
[11393730513] [INFO] [kernel::memory] [CPU0]   [15] 0x779e1000 - 0x779e2000 (Reserved)
[11394310950] [INFO] [kernel::memory] [CPU0]   [16] 0x779e2000 - 0x779e3000 (Other)
[11394953988] [INFO] [kernel::memory] [CPU0]   [17] 0x779e3000 - 0x779e4000 (Reserved)
[11395538352] [INFO] [kernel::memory] [CPU0]   [18] 0x779e4000 - 0x77a6f000 (Other)
[11396083842] [INFO] [kernel::memory] [CPU0]   [19] 0x77a6f000 - 0x77a70000 (Reserved)
[11396644941] [INFO] [kernel::memory] [CPU0]   [20] 0x77a70000 - 0x77ef1000 (Other)
[11397185250] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef1000 - 0x77ef2000 (Reserved)
[11397847296] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef2000 - 0x78013000 (Other)
[11398444761] [INFO] [kernel::memory] [CPU0]   [23] 0x78013000 - 0x78014000 (Reserved)
[11399064600] [INFO] [kernel::memory] [CPU0]   [24] 0x78014000 - 0x78814000 (Other)
[11399612565] [INFO] [kernel::memory] [CPU0]   [25] 0x78814000 - 0x78815000 (Reserved)
[11400172344] [INFO] [kernel::memory] [CPU0]   [26] 0x78815000 - 0x788d6000 (Other)
[11400716712] [INFO] [kernel::memory] [CPU0]   [27] 0x788d6000 - 0x788d7000 (Reserved)
[11401277844] [INFO] [kernel::memory] [CPU0]   [28] 0x788d7000 - 0x788e6000 (Other)
[11401835742] [INFO] [kernel::memory] [CPU0]   [29] 0x788e6000 - 0x788e7000 (Reserved)
[11402397336] [INFO] [kernel::memory] [CPU0]   [30] 0x788e7000 - 0x788ec000 (Other)
[11402939064] [INFO] [kernel::memory] [CPU0]   [31] 0x788ec000 - 0x788ed000 (Reserved)
[11403498678] [INFO] [kernel::memory] [CPU0]   [32] 0x788ed000 - 0x788f2000 (Other)
[11404041924] [INFO] [kernel::memory] [CPU0]   [33] 0x788f2000 - 0x788f3000 (Reserved)
[11404604211] [INFO] [kernel::memory] [CPU0]   [34] 0x788f3000 - 0x7891f000 (Other)
[11405177157] [INFO] [kernel::memory] [CPU0]   [35] 0x7891f000 - 0x78921000 (Reserved)
[11405740005] [INFO] [kernel::memory] [CPU0]   [36] 0x78921000 - 0x7892a000 (Other)
[11406283680] [INFO] [kernel::memory] [CPU0]   [37] 0x7892a000 - 0x7892c000 (Reserved)
[11406845472] [INFO] [kernel::memory] [CPU0]   [38] 0x7892c000 - 0x78934000 (Other)
[11407386573] [INFO] [kernel::memory] [CPU0]   [39] 0x78934000 - 0x78935000 (Reserved)
[11407945395] [INFO] [kernel::memory] [CPU0]   [40] 0x78935000 - 0x7893f000 (Other)
[11408504712] [INFO] [kernel::memory] [CPU0]   [41] 0x7893f000 - 0x78940000 (Reserved)
[11409066570] [INFO] [kernel::memory] [CPU0]   [42] 0x78940000 - 0x7894d000 (Other)
[11409608991] [INFO] [kernel::memory] [CPU0]   [43] 0x7894d000 - 0x7894f000 (Reserved)
[11410251303] [INFO] [kernel::memory] [CPU0]   [44] 0x7894f000 - 0x7895d000 (Other)
[11410796463] [INFO] [kernel::memory] [CPU0]   [45] 0x7895d000 - 0x7895e000 (Reserved)
[11411359575] [INFO] [kernel::memory] [CPU0]   [46] 0x7895e000 - 0x7896a000 (Other)
[11411920575] [INFO] [kernel::memory] [CPU0]   [47] 0x7896a000 - 0x7896b000 (Reserved)
[11412481872] [INFO] [kernel::memory] [CPU0]   [48] 0x7896b000 - 0x7896f000 (Other)
[11413023798] [INFO] [kernel::memory] [CPU0]   [49] 0x7896f000 - 0x78970000 (Reserved)
[11413583742] [INFO] [kernel::memory] [CPU0]   [50] 0x78970000 - 0x78980000 (Other)
[11414124084] [INFO] [kernel::memory] [CPU0]   [51] 0x78980000 - 0x78981000 (Reserved)
[11414682576] [INFO] [kernel::memory] [CPU0]   [52] 0x78981000 - 0x78a11000 (Other)
[11415237009] [INFO] [kernel::memory] [CPU0]   [53] 0x78a11000 - 0x78a12000 (Reserved)
[11415796326] [INFO] [kernel::memory] [CPU0]   [54] 0x78a12000 - 0x78a1d000 (Other)
[11416338483] [INFO] [kernel::memory] [CPU0]   [55] 0x78a1d000 - 0x78a1e000 (Reserved)
[11416897074] [INFO] [kernel::memory] [CPU0]   [56] 0x78a1e000 - 0x78a43000 (Other)
[11417438076] [INFO] [kernel::memory] [CPU0]   [57] 0x78a43000 - 0x78a44000 (Reserved)
[11417998746] [INFO] [kernel::memory] [CPU0]   [58] 0x78a44000 - 0x78a50000 (Other)
[11418567204] [INFO] [kernel::memory] [CPU0]   [59] 0x78a50000 - 0x78a51000 (Reserved)
[11419207470] [INFO] [kernel::memory] [CPU0]   [60] 0x78a51000 - 0x78a5e000 (Other)
[11419755699] [INFO] [kernel::memory] [CPU0]   [61] 0x78a5e000 - 0x78a5f000 (Reserved)
[11420317491] [INFO] [kernel::memory] [CPU0]   [62] 0x78a5f000 - 0x78aa7000 (Other)
[11420860440] [INFO] [kernel::memory] [CPU0]   [63] 0x78aa7000 - 0x78aa8000 (Reserved)
[11421655212] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11654755695] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485778 free frames
[11665413936] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11670407727] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11671760925] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11672769306] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11676946941] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11678623044] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11679690495] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11680365543] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11681027028] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11681686005] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11682643170] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11683585518] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11684298549] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11685015111] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11685709695] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11686461369] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11687654583] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11688705006] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11689399986] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11691022464] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11691991047] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11692972104] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11694670317] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11696205642] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11697064995] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11697606921] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11698412286] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12061059186] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12062027208] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12065170524] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12066018261] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12066768153] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12068300343] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12080898291] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12082230930] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12082954752] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12084619404] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12085195749] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12087617190] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12095169174] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12096998595] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12109933341] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12110492559] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12126068394] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12126634443] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12128543064] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12129700044] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12130748652] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12132982587] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12133811943] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12168588003] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62351600 ticks/sec), init_cnt=623516 for 100Hz
[12169952322] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12170749635] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12171853419] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12177291918] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12207851865] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12208890012] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12210284064] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12211503480] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12212699598] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12215329863] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12216568452] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12238230510] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12240057654] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12240913674] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12242856087] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12243698379] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12245898456] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12247356396] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12273179028] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12274486785] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12275437680] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12276578556] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12277595055] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12278356827] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12279697452] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12280381212] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12286061172] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12287062293] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12288976656] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12289911678] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12296820921] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12298435776] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12299149104] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12300336708] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12301405281] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12302619450] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12315441435] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12318245709] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12319308309] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12320076747] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12340519950] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12357848943] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12359868246] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12362865009] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12364365057] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12366282324] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12368459037] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12370247637] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12371065311] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12372148998] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12378395040] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12380141499] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12382585182] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12384877857] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12388764102] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12389554287] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12400729044] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12401558037] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12408384846] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12409220835] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12434844147] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12435856818] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[12791028261] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13354522269] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13385835276] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=499 journal=424 symbols=51 drops=0
[13417938996] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[14617780518] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=961 journal=769 symbols=95 drops=0
[15308728776] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15403008390] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15404373402] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[15500738913] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[15557228709] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[15590739846] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[15591616656] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[15592464360] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[15597318990] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[15611970363] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[15627129705] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[15629302161] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[15678971286] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[15699247707] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[15700052577] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[15703698978] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[15731193753] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[15750279534] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[15755251974] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[15756243723] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[15818313357] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[15820053414] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[15902900079] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[15975909213] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[15987209271] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[15991623714] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[15994032285] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[16025580483] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[16067864175] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[16073143944] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[16074144405] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[16075032831] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[16075935414] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[16076657982] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[16077300855] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[16077939966] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[16078532250] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[16079269437] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=29168
[16079917029] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=33264
[16080531060] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[16081157895] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[16081801758] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[16082521554] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[16083597519] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[16084380873] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[16085187228] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[16086023118] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[16086676089] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[16087378428] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[16088239398] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[16088983317] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[16089600153] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[16090213755] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[16090868772] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[16091504286] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[16092121650] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[16092813066] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[16093426932] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[16094072841] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[16094712579] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[16095358983] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[16096009413] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[16096647138] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[16097266053] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[16097994759] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[16098732078] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[16099486887] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[16100218761] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[16100979246] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[16101916809] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[16102687161] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[16104253473] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[16106011185] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[16106788830] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16114879770] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[16130291265] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[16134692178] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[16144387578] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[16145254686] [CONTRACT] [kernel] [CPU0] Spawning init process...
[16147268445] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[16150800105] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[16151845083] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [16157794323] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[16159285692] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[16182245211] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[16185087633] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[16186267911] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[16187317641] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[16198086168] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[16203470712] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[16206820443] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[16207892052] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[16211042331] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[16214138622] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[16215448293] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[16218904152] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[16220081856] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[16221156831] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[16222404891] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16226002782] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16227253119] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16228341657] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16229471808] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16230576549] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16231772271] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16235645052] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[16277454039] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[16286968170] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[16292604537] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[16298281791] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[16301400126] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[16305091242] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[16309873602] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[16314592932] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[16319692026] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[16325876754] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[16330484313] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[16335948057] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[16341305376] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[16346185548] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[16351888410] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[16357097229] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[16361826921] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[16367705871] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[16372254591] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[16377634317] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[16382817330] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[16387392153] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[16392379674] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[16397758245] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[16403179683] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[16409238813] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[16414215477] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[16421040141] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[16426238268] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[16431826158] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[16435318185] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[16438543011] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[16443280062] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[16448529339] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[16454275926] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[16460244603] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[16465141902] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[16470140742] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[16475583795] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[16480665927] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[16486381098] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[16489779735] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[16510847100] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[16659191109] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[16666497177] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[16667315544] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16668924888] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16673395893] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[16674647946] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16682549895] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[16687260678] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16688495373] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16690196985] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[16694205561] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[16695193416] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16696614693] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[16697603472] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16701755136] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[16703686956] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16713249696] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[16717998066] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[16719063471] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[16721397858] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[16725468672] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[16729154574] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[16730478501] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[16732470942] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[16736562348] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:57:44 = 1775440664 unix_secs
[16738004712] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775440664, mono_ns=8368765152, offset=1775440655631234848ns
[16739529048] [INFO] [rtc_cmos] [CPU1] System clock anchored
[16750321137] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[16789756302] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[16799810115] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[16800850671] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16804072692] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16812721398] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[16816168215] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[16827416067] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[16831594593] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[16835028738] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16836848094] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[16842057474] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[16846299558] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[16848057699] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[16849159767] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16851814782] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16861270767] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[16863597564] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[16870283595] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[16873392690] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16874431827] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16875960651] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[16879540656] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[16882778385] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[16884928797] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[16889980272] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[16891692048] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[16892505003] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[16893407883] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[16894402239] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[16909220130] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[16910967480] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[17396139024] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17401489182] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[17404217127] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[17405862936] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[17410240254] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[17411709975] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[17413664334] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[17415060795] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[17416008852] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[17418167547] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[17419102503] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[17420022807] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[17421015216] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[17421823914] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[17422575984] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[17423523348] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[17427978909] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[17428901193] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17430629931] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17437792185] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17440271475] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17447088813] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[17449747161] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[17450641131] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[17452147878] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352848 RFLAGS_BEFORE=134 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[17458475430] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[17481226686] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[17483444913] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[17493401970] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[17495508657] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[17500698798] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[17509253520] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[17510705256] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[17515802337] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[17516917671] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[17517986475] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[17518860975] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[17519653371] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[17520497379] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[17521361022] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[17522410290] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[17523517011] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[17529919143] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[17532562410] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[17534890560] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[17536753971] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[17537437830] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17539062684] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17543062779] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[17544920184] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17551922817] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[17554015974] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[17555728608] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[17556425832] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17557884300] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17562544560] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17563834992] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17570665563] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[17573694765] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
[17574631140] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[17576822307] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500320 RFLAGS_BEFORE=134 CR3_BEFORE=68505600 fs_base=0 gs_base=18446744071564586640
[17579710533] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[17580436599] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17581844676] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17582532396] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[17583654990] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[17587088211] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17587949940] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[17589338778] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[17589958716] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17591450877] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[17593628283] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[17595559971] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[17597209575] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[17598539739] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[17599918908] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[17601805254] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[17603236035] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[17603893098] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[17605914711] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[17607093669] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[17608746540] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[17610152076] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434240 RFLAGS_BEFORE=130 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[17615052312] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[17967271377] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[17969107332] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583104 RFLAGS_BEFORE=134 CR3_BEFORE=68620288 fs_base=0 gs_base=18446744071564586576
[17972551575] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17975575728] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[17976667368] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17978996310] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17988635511] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[17991993228] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[17993134104] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[17996641509] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[17999719551] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[18003460035] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[18006317175] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[18008454321] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[18010076766] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[18010852827] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18012429402] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18016177740] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[18031560195] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18035871414] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[18085978614] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[18855748395] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[18858290814] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[18859533330] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18861091062] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369719024 RFLAGS_BEFORE=134 CR3_BEFORE=68882432 fs_base=0 gs_base=18446744071564586640
[18864988230] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[18867111615] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[18868319448] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[18869314398] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[18870446925] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[18872383860] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[18874097979] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653488 RFLAGS_BEFORE=134 CR3_BEFORE=68747264 fs_base=0 gs_base=18446744071564586608
[18879768798] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[18881486283] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[18887779086] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[18889528845] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[18890426346] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[18891677178] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[18894182868] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering interrupt-driven loop
[18904078578] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[18907338318] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[18908426394] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[18909775500] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[18911844435] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[18912689796] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18913855389] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[18914685933] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18916059624] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[18917020881] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18980673690] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[19004250210] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[19011935778] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[19015085331] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[19017086121] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19018670187] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369784560 RFLAGS_BEFORE=134 CR3_BEFORE=79896576 fs_base=0 gs_base=18446744071564586576
[19021642266] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[19022443506] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19023870195] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[19024961439] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19028058852] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[19033198470] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19034661855] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19050731436] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[19054349061] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[19061640444] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[19064553783] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[19066528833] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[19067324694] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19068723993] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19074127545] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19076518890] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19083641940] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[19086294282] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b030
[19088272434] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19089870459] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915632 RFLAGS_BEFORE=134 CR3_BEFORE=80830464 fs_base=0 gs_base=18446744071564586640
[19092582696] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19093435515] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19094569428] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[19095221706] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19096519893] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[19106246676] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19110202386] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19117726947] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[19120782384] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b030
[19122154557] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19123997310] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981776 RFLAGS_BEFORE=134 CR3_BEFORE=80957440 fs_base=0 gs_base=18446744071564586576
[19127565996] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[19128463827] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19130203257] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19139673399] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19142855787] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[19150262967] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[19153062555] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[19155242337] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[19155963420] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19157371563] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19182708303] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[19187480070] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[19194980970] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[19197710400] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cc18
[19199244669] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19201762239] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370114032 RFLAGS_BEFORE=134 CR3_BEFORE=81260544 fs_base=0 gs_base=18446744071564586640
[19206729399] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[19207474374] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19209001812] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19209809685] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[19228298892] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[19230423531] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[19231851837] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19236766230] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19239204303] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19241274624] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19248421401] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[19257688923] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[19261559889] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[19264855236] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[19265904933] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19268769267] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19282507596] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19284598278] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370202384 RFLAGS_BEFORE=130 CR3_BEFORE=81534976 fs_base=0 gs_base=18446744071564586576
[19295471976] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[19297317831] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[19306635777] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19318524654] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19320212571] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19321618800] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19323090963] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19324649421] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19326164187] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=229 pred=0 subj_lo=0
[19333034952] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[19340348280] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[19355740866] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[19367141475] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[19371861729] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[19376948976] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19379456052] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369850096 RFLAGS_BEFORE=134 CR3_BEFORE=80568320 fs_base=0 gs_base=18446744071564586608
[19386070275] [INFO] [nectar] [CPU2] NECTAR: Started.
[19388845641] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19390010409] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b030
[19391396937] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19393397232] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19394762112] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047952 RFLAGS_BEFORE=130 CR3_BEFORE=81104896 fs_base=0 gs_base=18446744071564586608
[19398284796] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[19400322249] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19402014489] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19403797479] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19405675476] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[19407703986] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[19411510437] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010df90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19413881124] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370267920 RFLAGS_BEFORE=130 CR3_BEFORE=81743872 fs_base=0 gs_base=18446744071564586608
[19421170098] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[19422511119] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
[19424538540] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[19425953349] [INFO] [fontd] [CPU3] FONTD: Service ready
[19426958661] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[19430490486] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[19432554372] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[19434504045] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[19436051052] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[19438378905] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[19440585615] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[19442545683] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4e93000
[19443914457] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[19446174330] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[19447056420] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19448580492] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19449441924] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[19450884255] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[19456502736] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19458399444] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19460132703] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19462226883] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=235 subj_lo=0
[19463964927] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[19475685207] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19476900828] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19478223699] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19479606399] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=236 subj_lo=0
[19481107734] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[19487136768] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19488436902] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19489697634] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19491478512] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=237 subj_lo=0
[19493432145] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[19496299614] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[19499188632] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[19501606245] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[19503642840] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[19506229380] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[19507541328] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19508754969] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[19509927525] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19511205120] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[19512311511] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19513651542] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[19514392821] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=238 subj_lo=0
[19519714434] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=19, read=20)
[19523461518] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19524594012] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19525790889] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=21, read=22)
[19526730927] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19528047000] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=239 pred=0 subj_lo=0
[19535316735] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[19538754411] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([229, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19560219030] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[19561739934] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([229, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19578624549] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1257 backend=VirtIO-GPU
[19581223299] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[19582336092] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19584704337] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19619289195] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([229, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19624605066] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19627164348] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19633382703] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[19653026118] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=23, resp=26
[19654754790] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[19692738186] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([229, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19743223665] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[19758449799] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[19766779890] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[19769844633] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db468
[19771150014] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e9
[19773143049] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370509392 RFLAGS_BEFORE=134 CR3_BEFORE=82509824 fs_base=0 gs_base=18446744071564586640
[19777331706] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[19778484396] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19781001834] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19782264645] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[19788461154] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19791686673] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[19803052170] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[19807348737] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db468
[19808670321] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[19810423644] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370574928 RFLAGS_BEFORE=134 CR3_BEFORE=83558400 fs_base=0 gs_base=18446744071564586576
[19813582635] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[19815548841] [INFO] [echo] [CPU1] echo: starting up
[19816744860] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[19818163662] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[19828172298] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[19829933178] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[19839633726] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[19842143409] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[19846756050] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[19866618222] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[19868204895] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[19869423156] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[19872419556] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[19873596732] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[19875085197] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[19876213005] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19878820401] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19883618106] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([229, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19886490921] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19889385549] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[19890825867] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19892354064] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19901051214] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[19901962047] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[19907164431] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[19910669856] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[19911804726] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[19914242898] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[19916818416] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19917977145] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19920420102] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19931604231] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19937329269] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19941246237] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[19942495353] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[19945133802] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[19949452710] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[19950882039] [INFO] [netd] [CPU3] NETD: Driver TX port=19, RX port=22, link_up=true, mtu=1500
[19953984435] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[19955179695] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[19957122636] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[19959242061] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[19960522032] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[19961696964] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19964655579] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19966394943] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f11d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19969012503] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370706000 RFLAGS_BEFORE=134 CR3_BEFORE=84332544 fs_base=0 gs_base=18446744071564586640
[19973833836] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[19975802418] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[19986275199] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[19989169563] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[19990528041] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[19991617371] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f11d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19993331160] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370804304 RFLAGS_BEFORE=134 CR3_BEFORE=84492288 fs_base=0 gs_base=18446744071564586576
[19999094676] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[20004569310] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[20005360122] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=19, RX port=22
[20036309832] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[20055248202] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[20063984160] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[20067956733] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[20069604654] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[20104895580] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[20107790142] [INFO] [bloom] [CPU3] bloom: creating surface...
[20109074931] [INFO] [bloom] [CPU3] bloom: surface created!
[20110145847] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[20124239751] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[20136119289] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[20138104932] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[20140184922] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=31, our_read=32)
[20142105522] [INFO] [anther] [CPU1] anther: Connected to network stack
[20146280814] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[20191760028] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[20192629578] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[20194466655] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1263
[20196148896] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[20197680360] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[20209177296] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[20213282100] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[20214958533] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[20216587677] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:08C0 [20225732109] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[20227428540] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[20230508166] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[20246758950] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[20247915798] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 3)
[20253245430] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[20257803291] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[20260205460] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[20265618945] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[20267377779] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[20269611483] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[20273884191] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[20282876394] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[20292241629] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1263
[20295679767] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
T:0E20 T:0C90 T:F700 [20315344599] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[20317411587] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[20334877497] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[20336826345] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[20339575971] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[20347814124] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=278
[20350089672] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[20351683704] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20352848604] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20354233284] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20355955686] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=278 subj_lo=0
[20362987821] [INFO] [netd] [CPU3] NETD: DHCP configured �� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[20366676462] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[20371227162] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[20373834393] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=1
[20377168284] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[20381186727] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=2
[20384351394] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=2)
[20389426992] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1273)
[20390987826] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[20401310292] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=239
[20402682333] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[20403706455] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20404900626] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20406149280] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20407752981] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=239 pred=0 subj_lo=0
[20414011596] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[20422909782] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[20426019141] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[20428823547] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[20431732497] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[20441088492] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[20443194585] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[20447538804] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[20452899159] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1278)
[20454938724] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
T:1870 [20458813518] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[20460782925] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[20476875243] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=284
[20478646419] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[20479948500] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20481002685] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20482245036] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20483574705] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=284 subj_lo=0
[20500732296] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1281)
[20502283791] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[20610765054] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[20858219382] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[20956430517] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[20976032055] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[21005062617] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[21103179141] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[21183404979] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=664 watches=13 history=1024 journal=1024 symbols=317 drops=0
[21253513908] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[21396681702] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 29168 bytes, hash=790243a6d8f2a2ea)
[21565281606] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 33264 bytes, hash=9e7795f4f5f9311a)
[21792146376] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=ae36cd328bcc45e4)
[21961301109] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[22091412552] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[22135638294] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[22294084989] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[22404453291] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[22409522586] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[22410858987] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[22411796979] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[22412884362] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[22414141728] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[22437674028] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[22439002410] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[22440124641] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[22441511301] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=340 pred=0 subj_lo=0
[22492578966] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[22678727511] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[22792975953] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=714 watches=15 history=1024 journal=1024 symbols=343 drops=0
[22898850909] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[23089995291] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[23254704891] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[23256532992] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[23282198346] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[23312569071] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23393010795] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23396146191] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23397142659] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23398213872] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23399448204] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=312 pred=0 subj_lo=0
[23530370061] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23580417267] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=0e57f834b3afbcb1)
[23656262982] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[23662032999] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23733634848] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[23737872543] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23739035298] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23740223133] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23741152248] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=313 pred=0 subj_lo=0
[23783709345] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23895981483] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[23942880093] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[23987406003] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24032673126] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[24036504756] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[24037805484] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24038946459] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24040300680] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=314 pred=0 subj_lo=0
[24053739963] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12235000
[24055423788] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[24057001188] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[24120526320] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[24130248021] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[24134461494] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[24136706913] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[24138932202] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[24142983183] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[24148255032] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[24163222182] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[24164678604] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[24166605408] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[24170782053] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[24173019057] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[24175523163] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[24176991861] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=1 (16384b)
[24193590696] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x122c6000
[24195383421] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[24228520998] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=c1e9a7b90a789e5c)
[24437900850] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24449581596] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[24680870115] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[24916973664] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[25028987445] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=801 watches=18 history=1024 journal=1024 symbols=355 drops=0
[25080625845] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[25199557317] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25205920080] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[25219637553] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[25298649024] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25304109567] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[25305313077] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25306485765] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25307924268] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=343 pred=0 subj_lo=0
[25513764210] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25530644931] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[25634598231] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25847327484] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[26153820330] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[26467118733] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[26754492204] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[27058703727] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[27098594259] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=865 watches=19 history=1024 journal=1024 symbols=367 drops=0
[27343031067] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[27632911383] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[27979892490] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[28673936841] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=886 watches=19 history=1024 journal=1024 symbols=368 drops=0
[28982971083] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[29192917347] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2429 ops=1 watches=19
[29494098810] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[30240410541] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[30499942968] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=912 watches=19 history=1024 journal=1024 symbols=368 drops=0
[30630462027] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[30894886671] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[31034641869] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[31470764721] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[32728129071] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=970 watches=19 history=1024 journal=1024 symbols=401 drops=0
[33633600330] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[34904721324] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[34992782253] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[35225225640] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[35228971767] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[35249478693] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[35322890295] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1051 watches=19 history=1024 journal=1024 symbols=452 drops=0
[35648710482] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[35695137060] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35696530980] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35697908961] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35699029014] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[35705854767] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35706986733] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35708128170] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35709537996] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=234 pred=0 subj_lo=0
[35718324213] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35719404699] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35720524389] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35722143072] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=453 pred=0 subj_lo=0
[35728782870] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35729967537] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35731032282] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35732278428] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=452 pred=0 subj_lo=0
[35739063591] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35740128930] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35741336763] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35742625248] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[35750206338] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[37111619919] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1084 watches=24 history=1024 journal=1024 symbols=454 drops=0
[38538705249] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1104 watches=24 history=1024 journal=1024 symbols=454 drops=0
[40105147929] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=1135 watches=24 history=1024 journal=1024 symbols=454 drops=0
[41633037468] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=1157 watches=24 history=1024 journal=1024 symbols=454 drops=0
[43168815294] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=16000 nodes=1181 watches=24 history=1024 journal=1024 symbols=454 drops=0
[43651629714] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[44571296550] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=3453 ops=1 watches=24
[44771220417] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=17000 nodes=1216 watches=24 history=1024 journal=1024 symbols=454 drops=0
[46210482384] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[46347412881] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=18000 nodes=1238 watches=24 history=1024 journal=1024 symbols=454 drops=0
[47907580281] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=19000 nodes=1260 watches=24 history=1024 journal=1024 symbols=454 drops=0
[49535635578] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=20000 nodes=1284 watches=24 history=1024 journal=1024 symbols=454 drops=0
[51538325850] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=21000 nodes=1315 watches=24 history=1024 journal=1024 symbols=454 drops=0
[53444644770] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=22000 nodes=1345 watches=24 history=1024 journal=1024 symbols=454 drops=0
[55116639435] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=23000 nodes=1371 watches=24 history=1024 journal=1024 symbols=454 drops=0
[57012788250] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=24000 nodes=1400 watches=24 history=1024 journal=1024 symbols=454 drops=0
[58793466864] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=25000 nodes=1428 watches=24 history=1024 journal=1024 symbols=454 drops=0
[59812960581] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=1
[60106890591] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[60120755112] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[60810318096] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=26000 nodes=1468 watches=24 history=1024 journal=1024 symbols=454 drops=0
[62903562315] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=27000 nodes=1513 watches=24 history=1024 journal=1024 symbols=454 drops=0
[64888922604] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=28000 nodes=1541 watches=24 history=1024 journal=1024 symbols=454 drops=0
[66638143836] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=29000 nodes=1565 watches=24 history=1024 journal=1024 symbols=454 drops=0
[68403114384] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=30000 nodes=1594 watches=24 history=1024 journal=1024 symbols=454 drops=0
[69356368257] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=4477 ops=1 watches=24
[70522428768] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=31000 nodes=1628 watches=24 history=1024 journal=1024 symbols=454 drops=0
[73211389779] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=32000 nodes=1652 watches=24 history=1024 journal=1024 symbols=454 drops=0
[76645499172] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=33000 nodes=1691 watches=24 history=1024 journal=1024 symbols=454 drops=0
[79587586452] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=34000 nodes=1725 watches=24 history=1024 journal=1024 symbols=454 drops=0
[81538541535] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=35000 nodes=1751 watches=24 history=1024 journal=1024 symbols=454 drops=0
[84797669286] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=36000 nodes=1810 watches=24 history=1024 journal=1024 symbols=454 drops=0
[87204773040] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=37000 nodes=1832 watches=24 history=1024 journal=1024 symbols=454 drops=0
[89261065212] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=38000 nodes=1854 watches=24 history=1024 journal=1024 symbols=454 drops=0
[91510342968] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=39000 nodes=1891 watches=24 history=1024 journal=1024 symbols=454 drops=0
[93787771605] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=40000 nodes=1919 watches=24 history=1024 journal=1024 symbols=454 drops=0
[96153618297] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=41000 nodes=1943 watches=24 history=1024 journal=1024 symbols=454 drops=0
[98314312899] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=42000 nodes=1969 watches=24 history=1024 journal=1024 symbols=454 drops=0
[100500600981] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=43000 nodes=2006 watches=24 history=1024 journal=1024 symbols=454 drops=0
[101742893769] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=5501 ops=1 watches=24
[102906696783] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=44000 nodes=2034 watches=24 history=1024 journal=1024 symbols=454 drops=0
[104925292197] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=45000 nodes=2058 watches=24 history=1024 journal=1024 symbols=454 drops=0
[107490085857] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=46000 nodes=2097 watches=24 history=1024 journal=1024 symbols=454 drops=0
[109814498442] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=47000 nodes=2125 watches=24 history=1024 journal=1024 symbols=454 drops=0
[112139266041] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=48000 nodes=2159 watches=24 history=1024 journal=1024 symbols=454 drops=0
[114207271827] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=49000 nodes=2188 watches=24 history=1024 journal=1024 symbols=454 drops=0
[117410796591] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=50000 nodes=2238 watches=24 history=1024 journal=1024 symbols=454 drops=0
[119330505987] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=51000 nodes=2262 watches=24 history=1024 journal=1024 symbols=454 drops=0
[121202783196] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=52000 nodes=2286 watches=24 history=1024 journal=1024 symbols=454 drops=0
[123597534060] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=53000 nodes=2323 watches=24 history=1024 journal=1024 symbols=454 drops=0
[125717506992] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=54000 nodes=2351 watches=24 history=1024 journal=1024 symbols=454 drops=0
[127797889263] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=55000 nodes=2375 watches=24 history=1024 journal=1024 symbols=454 drops=0
[129959171991] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=56000 nodes=2412 watches=24 history=1024 journal=1024 symbols=454 drops=0
[130815042204] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=6525 ops=1 watches=24
[132489538170] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=57000 nodes=2446 watches=24 history=1024 journal=1024 symbols=454 drops=0
[134395856793] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=58000 nodes=2472 watches=24 history=1024 journal=1024 symbols=454 drops=0
[136193108865] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=59000 nodes=2492 watches=24 history=1024 journal=1024 symbols=454 drops=0
[138829756296] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=60000 nodes=2535 watches=24 history=1024 journal=1024 symbols=454 drops=0
[140772085509] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=61000 nodes=2561 watches=24 history=1024 journal=1024 symbols=454 drops=0
[143052472728] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=62000 nodes=2587 watches=24 history=1024 journal=1024 symbols=454 drops=0
[144980278641] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=63000 nodes=2622 watches=24 history=1024 journal=1024 symbols=454 drops=0
[146865626523] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=64000 nodes=2648 watches=24 history=1024 journal=1024 symbols=454 drops=0
[149080779210] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=65000 nodes=2672 watches=24 history=1024 journal=1024 symbols=454 drops=0
[151486546200] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=66000 nodes=2710 watches=24 history=1024 journal=1024 symbols=454 drops=0
[153566551017] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=67000 nodes=2743 watches=24 history=1024 journal=1024 symbols=454 drops=0
[155593923573] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=68000 nodes=2771 watches=24 history=1024 journal=1024 symbols=454 drops=0
[157878776088] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=69000 nodes=2797 watches=24 history=1024 journal=1024 symbols=454 drops=0
[160350514797] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=70000 nodes=2827 watches=24 history=1024 journal=1024 symbols=454 drops=0
[160886195481] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=7549 ops=1 watches=24
[163027058601] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=71000 nodes=2866 watches=24 history=1024 journal=1024 symbols=454 drops=0
[165361384122] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=72000 nodes=2896 watches=24 history=1024 journal=1024 symbols=454 drops=0
[167600153325] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=73000 nodes=2924 watches=24 history=1024 journal=1024 symbols=454 drops=0
[169935479670] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=74000 nodes=2957 watches=24 history=1024 journal=1024 symbols=454 drops=0
[172577808282] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=75000 nodes=2995 watches=24 history=1024 journal=1024 symbols=454 drops=0
[174808718139] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=76000 nodes=3023 watches=24 history=1024 journal=1024 symbols=454 drops=0
[177779987715] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=77000 nodes=3063 watches=24 history=1024 journal=1024 symbols=454 drops=0
[180494812278] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=78000 nodes=3104 watches=24 history=1024 journal=1024 symbols=454 drops=0
[183142162236] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=79000 nodes=3136 watches=24 history=1024 journal=1024 symbols=454 drops=0
[185522544999] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=80000 nodes=3160 watches=24 history=1024 journal=1024 symbols=454 drops=0
[188056340637] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=81000 nodes=3192 watches=24 history=1024 journal=1024 symbols=454 drops=0
[191050346850] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=82000 nodes=3233 watches=24 history=1024 journal=1024 symbols=454 drops=0
[192723575979] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=8573 ops=1 watches=24
[193404095610] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=83000 nodes=3259 watches=24 history=1024 journal=1024 symbols=454 drops=0
[195438655896] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=84000 nodes=3279 watches=24 history=1024 journal=1024 symbols=454 drops=0
[198047985927] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=85000 nodes=3305 watches=24 history=1024 journal=1024 symbols=454 drops=0
[201352335426] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=86000 nodes=3338 watches=24 history=1024 journal=1024 symbols=454 drops=0
[203877719595] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=87000 nodes=3362 watches=24 history=1024 journal=1024 symbols=454 drops=0
[207074018085] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=88000 nodes=3396 watches=24 history=1024 journal=1024 symbols=454 drops=0
[210477578487] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=89000 nodes=3433 watches=24 history=1024 journal=1024 symbols=454 drops=0
[215198158197] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=90000 nodes=3483 watches=24 history=1024 journal=1024 symbols=454 drops=0
[218921285748] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=91000 nodes=3523 watches=24 history=1024 journal=1024 symbols=454 drops=0
[221763788466] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=92000 nodes=3547 watches=24 history=1024 journal=1024 symbols=454 drops=0
[225668735424] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=93000 nodes=3586 watches=24 history=1024 journal=1024 symbols=454 drops=0
[228923169585] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=94000 nodes=3614 watches=24 history=1024 journal=1024 symbols=454 drops=0
[232298923857] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=95000 nodes=3648 watches=24 history=1024 journal=1024 symbols=454 drops=0
[234959858925] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=96000 nodes=3678 watches=24 history=1024 journal=1024 symbols=454 drops=0
[235454375145] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=9597 ops=1 watches=24
[239098260489] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=97000 nodes=3725 watches=24 history=1024 journal=1024 symbols=454 drops=0
[242090570997] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=98000 nodes=3751 watches=24 history=1024 journal=1024 symbols=454 drops=0
[245424622905] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=99000 nodes=3777 watches=24 history=1024 journal=1024 symbols=454 drops=0
[248973082347] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=100000 nodes=3815 watches=24 history=1024 journal=1024 symbols=454 drops=0
[253131129669] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=101000 nodes=3858 watches=24 history=1024 journal=1024 symbols=454 drops=0
[256278021945] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=102000 nodes=3884 watches=24 history=1024 journal=1024 symbols=454 drops=0
[259591174524] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=103000 nodes=3916 watches=24 history=1024 journal=1024 symbols=454 drops=0
[264714559032] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=104000 nodes=3977 watches=24 history=1024 journal=1024 symbols=454 drops=0
[268303876071] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=105000 nodes=4005 watches=24 history=1024 journal=1024 symbols=454 drops=0
[272111349906] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=106000 nodes=4041 watches=24 history=1024 journal=1024 symbols=454 drops=0
[275319309969] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=107000 nodes=4069 watches=24 history=1024 journal=1024 symbols=454 drops=0
[280002024258] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=108000 nodes=4110 watches=24 history=1024 journal=1024 symbols=454 drops=0
[280350116475] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=10621 ops=1 watches=24
[284061128472] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=109000 nodes=4142 watches=24 history=1024 journal=1024 symbols=454 drops=0
[286928135472] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=110000 nodes=4166 watches=24 history=1024 journal=1024 symbols=454 drops=0
[290701668147] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=111000 nodes=4196 watches=24 history=1024 journal=1024 symbols=454 drops=0
[295235916921] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=112000 nodes=4235 watches=24 history=1024 journal=1024 symbols=454 drops=0
[300723577407] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=113000 nodes=4289 watches=24 history=1024 journal=1024 symbols=454 drops=0
[304586665317] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=114000 nodes=4321 watches=24 history=1024 journal=1024 symbols=454 drops=0
[307926281157] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=115000 nodes=4343 watches=24 history=1024 journal=1024 symbols=454 drops=0
[311674349316] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=116000 nodes=4382 watches=24 history=1024 journal=1024 symbols=454 drops=0
[315974401050] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=117000 nodes=4420 watches=24 history=1024 journal=1024 symbols=454 drops=0
[319066612293] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=118000 nodes=4444 watches=24 history=1024 journal=1024 symbols=454 drops=0
[322329993315] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=119000 nodes=4470 watches=24 history=1024 journal=1024 symbols=454 drops=0
[326347785225] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=120000 nodes=4505 watches=24 history=1024 journal=1024 symbols=454 drops=0
[331079261139] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=11645 ops=1 watches=24
[331301928507] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=121000 nodes=4545 watches=24 history=1024 journal=1024 symbols=454 drops=0
[335851783146] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=122000 nodes=4589 watches=24 history=1024 journal=1024 symbols=454 drops=0
[339535724649] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=123000 nodes=4613 watches=24 history=1024 journal=1024 symbols=454 drops=0
[343706606496] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=124000 nodes=4660 watches=24 history=1024 journal=1024 symbols=454 drops=0
[351473505933] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=125000 nodes=4706 watches=24 history=1024 journal=1024 symbols=454 drops=0
[367875588036] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=126000 nodes=4762 watches=24 history=1024 journal=1024 symbols=454 drops=0
[370893005307] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=127000 nodes=4790 watches=24 history=1024 journal=1024 symbols=454 drops=0
[374958871221] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=128000 nodes=4833 watches=24 history=1024 journal=1024 symbols=454 drops=0
[378917950587] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=129000 nodes=4871 watches=24 history=1024 journal=1024 symbols=454 drops=0
[381859805976] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=130000 nodes=4899 watches=24 history=1024 journal=1024 symbols=454 drops=0
[384595714338] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=131000 nodes=4923 watches=24 history=1024 journal=1024 symbols=454 drops=0
[388070181972] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=132000 nodes=4951 watches=24 history=1024 journal=1024 symbols=454 drops=0
[389125185438] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[389163990039] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[392096398155] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=12669 ops=1 watches=24
[392918257479] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=133000 nodes=5012 watches=24 history=1024 journal=1024 symbols=454 drops=0
[396749663970] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=134000 nodes=5046 watches=24 history=1024 journal=1024 symbols=454 drops=0
[400165295838] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=135000 nodes=5072 watches=24 history=1024 journal=1024 symbols=454 drops=0
[403317307437] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=136000 nodes=5104 watches=24 history=1024 journal=1024 symbols=454 drops=0
[406990767183] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=137000 nodes=5139 watches=24 history=1024 journal=1024 symbols=454 drops=0
[410632531452] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=138000 nodes=5175 watches=24 history=1024 journal=1024 symbols=454 drops=0
[413987938815] [INFO] [kernel::root::service] [C
```
</details>
