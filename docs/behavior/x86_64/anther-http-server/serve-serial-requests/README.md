# ❌ Scenario: Serve serial requests

> Last run: 2026-04-05 18:00:52

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the anther server is ready | ❌ | 2839ms | - [📜](./01/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11410236915] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11415595059] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11419185162] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11421135066] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11422293465] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11422915350] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11423587659] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11424162519] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11424741042] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11425345239] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11425945674] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11426571321] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11427266070] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11427924354] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11428605771] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11429210298] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11429848518] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11430444003] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11431054470] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11431644015] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11432218281] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11432794692] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11433401892] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11434077303] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11434711002] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11435438949] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11436163266] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11436838479] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11437426011] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11438026875] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11438635164] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11439301599] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11440007337] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11440626549] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11441251371] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11442037926] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11442742806] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11443490751] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11444184972] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11444908629] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11445612915] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11446358451] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11447731383] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11449152825] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11449956507] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11450505462] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11451015609] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11451534666] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11452076295] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11452600764] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11453130117] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11453648481] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11454157242] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11454691941] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11455233405] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11455792656] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11456350455] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11456995407] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11457542118] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11458101468] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11458641678] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11459201622] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11459783808] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11460344016] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11460886008] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11461481592] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11462025465] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11462584518] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11463148785] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11463707343] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11464248741] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11464807893] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11465348829] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11465907651] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11466465516] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11467025592] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11467626654] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11468189535] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11468728656] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11469455514] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11470004865] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11470572135] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11471116602] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11471678988] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11472223389] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11472801450] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11473344003] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11473902924] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11474443596] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11475005091] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11475545994] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11476116234] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11476661361] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11477218698] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11477756334] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11478314331] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11478855267] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11479427256] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11479966608] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11480524539] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11481067356] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11481624726] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11482164672] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11482736100] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11483275782] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11483832525] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11484419496] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11485238292] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11721005835] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[11731490397] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11737384197] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11738827452] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11739739902] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11744201007] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11745874800] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11746969212] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11747642610] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11748300069] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11748957264] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11749926507] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11750864664] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11751542484] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11752206642] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11752890930] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11753556045] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11754738072] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11755735992] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11756461431] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11758111332] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11759042988] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11760211056] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11761807332] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11763342888] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11764136868] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11764663317] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11765397006] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12231233817] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12232747461] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12236408217] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12237909222] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12239178336] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12241151439] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12255113871] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12257198184] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12258378462] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12260622561] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12261505905] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12264543621] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12273221862] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12275087451] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12289302267] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12290027211] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12308029107] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12308717982] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12310906674] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12312211065] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12313365636] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12315780312] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12317007846] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12352265145] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62499200 ticks/sec), init_cnt=624992 for 100Hz
[12353906862] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12354942072] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12356185116] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12362026215] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12393215769] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12394717269] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12396226821] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12398137521] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12399812172] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12404743296] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12406614165] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12428341530] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12433084191] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12434810817] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12436361619] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12437238066] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12438620700] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12440393790] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12466918629] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12468110754] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12469419369] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12470340861] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12471835464] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12473080851] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12474070785] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12475129986] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12482403054] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12484346061] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12487112913] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12488522013] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12501855927] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12505075110] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12506184900] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12508177704] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12509609013] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12510909741] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12533765607] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12538362144] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12540279873] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12541568919] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12569061219] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12589743474] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12592863492] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12598383237] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12601309050] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12604574796] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12608386791] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12611392200] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12612477405] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12620603688] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12623843331] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12627812637] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12631187382] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12635104911] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12638572980] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12639744645] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12656683017] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12658014732] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12666999345] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12668257998] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12702429597] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12703749036] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13270836480] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13847444094] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13906467168] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[13981335324] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[15299364069] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=961 journal=769 symbols=95 drops=0
[16076421900] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[16172841597] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[16174218027] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[16292421684] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[16354364235] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[16389895830] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[16390846098] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[16391486991] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[16395598956] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[16417938141] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[16436670360] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[16444062822] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[16498473585] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[16523133792] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[16523994003] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[16528322547] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[16554739509] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[16574611482] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[16579000383] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[16580177625] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[16652165574] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[16654051590] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[16746663120] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[16813284906] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[16828489128] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[16834742232] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[16838121531] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[16850742018] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[16924474776] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[16930535919] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[16931693658] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[16932584856] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[16933584921] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[16934374479] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[16935140244] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[16935827568] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[16936491264] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[16937168028] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[16937853075] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[16938549309] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[16939265211] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[16940055759] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[16941015465] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[16942167330] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[16942865973] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[16943533134] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[16944203001] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[16944857754] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[16945494093] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[16946109444] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[16946969622] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[16947800958] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[16948518279] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[16949235534] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[16949974239] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[16950682650] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[16951478412] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[16952162535] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[16952867481] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[16953576915] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[16954315719] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[16955020533] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[16955774220] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[16956431448] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[16957234470] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[16958020728] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[16958806062] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[16959637794] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[16960489458] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[16961260041] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[16962065604] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[16963852323] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[16965607824] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[16966400946] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16975147101] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[16989685680] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[16995269049] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[17005079652] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[17005930821] [CONTRACT] [kernel] [CPU0] Spawning init process...
[17008836900] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[17011991634] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[17013044862] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [17020506261] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[17025401679] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[17044019784] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[17049086670] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[17050333674] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[17051800326] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[17059323996] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[17063687322] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[17068143180] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[17069737080] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[17074654014] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[17078454822] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[17079616521] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[17083893882] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[17084966877] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[17086051455] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[17087120127] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[17090842989] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[17092033596] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[17093355840] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[17094992475] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[17096793483] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[17098232184] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[17102825949] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[17147308431] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[17155368582] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[17160185790] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[17165648379] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[17168643063] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[17172386946] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[17177223954] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[17182059972] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[17186675451] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[17191914762] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[17196401937] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[17201936565] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[17207497593] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[17212437990] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[17217030963] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[17221544736] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[17227960629] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[17233501065] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[17238529110] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[17244150000] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[17249339646] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[17255130288] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[17261387715] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[17268072558] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[17274283521] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[17280813594] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[17285395116] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[17289933177] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[17294193312] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[17298597162] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[17302162416] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[17305853598] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[17310547386] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[17315598564] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[17320391022] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[17325666435] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[17330652207] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[17335510863] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[17341566957] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[17347292094] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[17352541305] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[17355579912] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[17376230553] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[17508286554] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[17515566024] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[17516508273] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17518398084] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17523279576] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17524674882] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17533129680] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[17538746478] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
[17540006847] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17542250814] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[17545934208] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[17546696178] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17548470753] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17551044060] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[17552643636] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[17554325382] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17561128860] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[17563334976] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[17564505684] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[17566375299] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[17570674539] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[17574677010] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[17576437560] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[17578604670] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17583328125] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:57:08 = 1775437028 unix_secs
[17585182296] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775437028, mono_ns=8792370708, offset=1775437019207629292ns
[17586997197] [INFO] [rtc_cmos] [CPU1] System clock anchored
[17601276396] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[17643446139] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[17650346472] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[17651227836] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17652972315] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17659198623] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[17661686493] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[17669169606] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[17673267876] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[17676851049] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17678864907] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210752 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[17684303274] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[17688812196] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[17690428668] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[17691106818] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17692673559] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17702399715] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17705142774] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17713410561] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[17717153025] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[17718094515] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17719844538] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277440 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[17722698444] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[17724985971] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[17731405527] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[17732627220] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[17733862839] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[17735198943] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[17736016947] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[17737447365] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[17753211795] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[17754942810] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[18246633636] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18250238292] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[18251091540] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18252668709] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18259413546] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18261821127] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18268436637] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[18270776007] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[18271831380] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[18273496362] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352624 RFLAGS_BEFORE=130 CR3_BEFORE=59662336 fs_base=0 gs_base=18446744071564586576
[18278201568] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[18280356336] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[18281331519] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[18282837012] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[18287065269] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[18288446220] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[18289440147] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[18291304911] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[18293133870] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[18294057837] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[18296205345] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[18297296259] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[18298260651] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[18299051892] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[18300033543] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[18300848379] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[18301950018] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[18303465708] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[18305579391] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[18310340004] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[18318637458] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[18319910499] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[18342168042] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[18345015546] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[18345784116] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[18347479128] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[18349011780] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[18349683660] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18351201957] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18359215809] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18360847131] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18367923486] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[18370336314] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[18372227874] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[18372935658] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18374484843] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18379489557] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18380936211] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18387816249] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[18390122916] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
[18390947388] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[18393994971] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500384 RFLAGS_BEFORE=130 CR3_BEFORE=68235264 fs_base=0 gs_base=18446744071564586640
[18398125152] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[18399212337] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18400903785] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[18401659188] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18402736704] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[18406678851] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[18408181011] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[18409066005] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18410141541] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[18411991422] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[18412859058] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18413981124] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[18415340823] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[18417098865] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[18418480377] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[18419801400] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[18422654679] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[18425132715] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[18426569073] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[18427725426] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[18430312560] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1236 port=2 model='                                        ' rpc_port=5
[18456756747] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[18457597092] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[18458343816] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[18459095556] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[18459830994] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[18460544685] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[18461354340] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[18462510627] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[18468111024] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[18469598268] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434304 RFLAGS_BEFORE=134 CR3_BEFORE=59830272 fs_base=0 gs_base=18446744071564586608
[18474408876] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[18784498887] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[18786228582] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583168 RFLAGS_BEFORE=130 CR3_BEFORE=68349952 fs_base=0 gs_base=18446744071564586576
[18791987379] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[18794309919] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[18798455181] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[18823863861] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18825365460] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[18826711167] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[18827445615] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18828954870] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18836284170] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[18839028318] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[18846140115] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[18848445825] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[18850261419] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[18852401568] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[18853187364] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18854964975] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18874500579] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18879266571] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[18891612564] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[18901739967] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[18904521999] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[18905944695] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18907630995] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716432 RFLAGS_BEFORE=130 CR3_BEFORE=68890624 fs_base=0 gs_base=18446744071564586640
[18912854862] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[18914373258] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[18915862284] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[18916805226] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[18919627551] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[18922751892] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[18925537125] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650256 RFLAGS_BEFORE=134 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[18933029808] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[18934756071] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[18940422336] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[18941777118] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[18942819885] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[18944390124] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[18947717382] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[18949163706] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[18961156104] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[18963045651] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[18964756899] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[18965608662] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18967502796] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19045113813] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[19069186224] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[19073488335] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[19074520509] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[19075952841] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[19077409428] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[19078631583] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[19082595279] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
[19083728763] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19086973521] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[19088096709] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19089819903] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369783200 RFLAGS_BEFORE=134 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[19093999419] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19098781251] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[19102700496] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[19108431210] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19110158430] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19124255997] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[19128068817] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[19135379538] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[19138369272] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[19140667458] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[19141602249] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19143259245] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19148849082] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19152842280] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19160021562] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[19163017764] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[19164441153] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19166257605] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915072 RFLAGS_BEFORE=134 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[19170469791] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19171467612] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19173289905] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[19174299375] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19175427018] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[19183210068] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19186904022] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19194227019] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[19197108843] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[19198403235] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19200113394] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981632 RFLAGS_BEFORE=134 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[19204239384] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[19205119923] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19207164438] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19216270788] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19220605701] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[19227820458] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[19230733005] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[19233325980] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[19234147548] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19235744517] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19261204710] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[19266404685] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[19275319998] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[19278156513] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010d500
[19279759125] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19281853503] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113776 RFLAGS_BEFORE=134 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[19286528217] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[19287416049] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19288900884] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[19289875539] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19306000230] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[19307543376] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[19309428996] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19310614983] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19311432624] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19312830867] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19314348702] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[19319241711] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[19322070339] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[19323751161] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[19324480890] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19326309222] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19329629319] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19332126264] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370198112 RFLAGS_BEFORE=134 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[19360777755] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[19366154346] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19367950272] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19369797282] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19371682539] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[19372568523] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=229 pred=0 subj_lo=0
[19379924619] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[19382545083] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[19386541251] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19388239068] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849280 RFLAGS_BEFORE=134 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[19393139832] [INFO] [nectar] [CPU2] NECTAR: Started.
[19394982618] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19396597209] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047984 RFLAGS_BEFORE=134 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[19402543842] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[19403313501] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19405433982] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19406998446] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19408131105] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[19410355206] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010e118
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19412360946] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370266448 RFLAGS_BEFORE=134 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[19417675497] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[19418757864] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1246) for kind 'Asset'
[19419778059] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[19420837392] [INFO] [fontd] [CPU3] FONTD: Service ready
[19421414067] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[19422345723] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[19424657373] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[19425636747] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[19445985537] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[19447602702] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[19449317085] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[19450886631] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[19451809839] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19452794427] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[19453618998] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19454641338] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[19455424956] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19457167290] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[19459098549] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4475000
[19461318195] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[19464686505] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[19467982479] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[19470052239] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[19481905839] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[19494554013] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[19508023161] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[19511827632] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[19514944944] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[19517206566] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[19518742683] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[19520330940] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[19521793170] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[19523022486] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[19524424326] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[19529726073] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=19, read=20)
[19534927302] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19536111144] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=21, read=22)
[19537088505] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19538562252] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19540137276] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=236 subj_lo=0
[19547527494] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[19552143006] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([226, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19560039906] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19561472733] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19567706103] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19568893014] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19570020657] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19571292576] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[19576802883] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19577866803] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19579075296] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19580412654] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[19586626653] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19587606126] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19588638333] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19589872896] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=244 subj_lo=0
[19591862070] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[19605153018] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([226, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19606729956] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19607835918] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19609152750] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19610480043] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1252 backend=VirtIO-GPU
[19611508092] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=247 subj_lo=0
[19612926861] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[19613731830] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19615581381] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19621868640] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19623087495] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19624291797] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19625609058] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=249 pred=0 subj_lo=0
[19655781618] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[19670936901] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=23, resp=26
[19672174995] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[19737212451] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([226, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19738484502] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19739629338] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19748313915] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[19762365777] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[19770131832] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[19773856245] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[19776198948] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[19777421862] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19779811920] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19783780830] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db448
[19784887815] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e4
[19787037402] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19788289191] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370507904 RFLAGS_BEFORE=130 CR3_BEFORE=72433664 fs_base=0 gs_base=18446744071564586640
[19795614927] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[19797276015] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[19799396892] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[19800925221] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db448
[19801786488] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[19803392631] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370573440 RFLAGS_BEFORE=130 CR3_BEFORE=73482240 fs_base=0 gs_base=18446744071564586576
[19807998078] [INFO] [echo] [CPU1] echo: starting up
[19809924651] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[19816273884] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=19, RX port=22
[19825158111] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[19826384424] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[19827787815] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[19834021416] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([226, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19855504911] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[19857277737] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[19871761569] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19873091040] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19876960818] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[19878924516] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[19880527425] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[19881284313] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19882557783] [INFO] [netd] [CPU3] NETD: Driver TX port=19, RX port=22, link_up=true, mtu=1500
[19883766210] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19885981269] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[19888996512] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19890072279] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[19890836196] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[19895396037] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[19897531995] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[19900039632] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[19902948384] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[19905101733] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[19907951910] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[19909580163] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[19910537196] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19911401961] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19913293125] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19920867549] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19924363998] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19931577336] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[19933586805] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[19934723160] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[19936218588] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[19937054214] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19938851526] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19941819876] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f26b0
[19943409651] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19945034736] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[19945844325] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370741376 RFLAGS_BEFORE=130 CR3_BEFORE=73986048 fs_base=0 gs_base=18446744071564586640
[19955077956] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[19956729738] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[19957729506] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[19959515895] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[19961166027] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[19962637167] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[19965979341] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f26b0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19967833974] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370806912 RFLAGS_BEFORE=130 CR3_BEFORE=74133504 fs_base=0 gs_base=18446744071564586576
[19972826247] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[19978270818] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[19991684130] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[19998280566] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[20001704778] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[20003185785] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[20009415030] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[20030482065] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=31, our_read=32)
[20032254429] [INFO] [anther] [CPU1] anther: Connected to network stack
[20037363852] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[20038010916] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[20041033419] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[20057786298] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[20060944332] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[20066514963] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[20069524827] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[20071104603] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[20075268378] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[20081515938] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[20082574644] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[20087696376] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[20104677450] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[20109001836] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[20110829640] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[20112836865] [INFO] [bloom] [CPU3] bloom: creating surface...
[20114220654] [INFO] [bloom] [CPU3] bloom: surface created!
[20115152079] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[20118209661] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[20119873719] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[20121096501] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[20122035978] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[20127442665] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1264
[20128600008] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[20142208383] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[20145366252] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[20146461852] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[20147600319] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
[20150981697] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[20151882300] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[20154886389] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[20156544705] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
T:0270 [20164239414] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[20172893862] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[20181667044] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[20189293410] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[20198628450] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1264
T:07D0 T:0640 T:F0B0 [20225056929] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[20228003928] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[20232676761] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[20234693094] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=1
[20237576733] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[20240109285] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=2
[20250235830] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[20252796630] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[20255020896] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[20263377816] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[20265706230] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[20268970689] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[20271685434] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[20277363216] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[20280475083] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[20285409540] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[20292271131] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=282
T:1220 [20295312180] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[20297426688] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[20307546105] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[20308769085] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20309809443] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20310875376] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20312016087] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=282 subj_lo=0
[20317105809] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[20328663168] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1276)
[20329913967] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[20334365073] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=2)
[20348593320] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=249
[20350125708] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[20351111880] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20352086898] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20353378947] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20354637963] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=249 pred=0 subj_lo=0
[20366852814] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1280)
[20368051209] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[20378779377] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=289
[20379971271] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[20380952328] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20382006744] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20383161810] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20384379939] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=289 subj_lo=0
[20398996299] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1281)
[20401096122] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[20566380417] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[20588900871] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[20590070622] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[20596263204] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[20603353155] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[20609333877] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[20612845110] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[20615257905] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=74
[20616333936] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=21
[20618210613] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[20622629214] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[20627210901] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[20630264160] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=SynReceived
[20636640156] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[20642188677] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[20643337407] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[20645601339] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[20651177415] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=149
[20652022710] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[20653083528] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=21
[20654901729] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[20656034982] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[20751396402] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[20783651328] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[20802571614] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[20829612144] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=664 watches=13 history=1024 journal=1024 symbols=314 drops=0
[20945283810] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[21088853577] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[21246843618] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[21501683346] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[21674105574] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[21814625778] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[21857844261] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[22036856688] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[22220162547] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[22243003167] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[22252253562] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[22254137433] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[22255273557] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[22256488848] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[22257925866] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[22268726568] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[22269847413] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[22270952220] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[22272507807] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=338 pred=0 subj_lo=0
[22403926677] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=708 watches=15 history=1024 journal=1024 symbols=348 drops=0
[22431166758] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[22625861775] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[22815315369] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[22848210264] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[22851242238] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[22852931607] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=Established
[22854278007] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:34268 on listener 2
[22859949123] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=2
[22877705862] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=3
[22887347010] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 37 (user thread) assigned to CPU 1
[22889462772] [INFO] [anther] [CPU1] anther: Thread spawned TID=37 for conn_handle=3
T:5EF0 [22918353480] [INFO] [anther] [CPU1] anther: Worker thread TID=37 starting for conn_handle=3
[22926946383] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[22928756103] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[22933241595] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22986066081] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23041676625] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23059874409] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23060976576] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23061986475] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23063217837] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=349 pred=0 subj_lo=0
[23071893933] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[23147882274] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23165371614] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=33, our_read=34)
[23167361943] [INFO] [anther] [CPU1] anther: Worker TID=37 connected to netd OK
[23186459109] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[23191235826] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[23208992994] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 85 bytes on conn_handle=3
[23217259065] [INFO] [anther] [CPU1] anther: GET /health Http11
[23234513280] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[23236670985] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[23250059448] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[23251372254] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23254223685] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23264031120] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23274090576] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23306633262] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[23309810766] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[23317822473] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23318874282] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23319885897] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23321239359] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[23322153558] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=350 pred=0 subj_lo=0
[23324000073] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23327045874] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23328016536] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23335837833] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[23337149022] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23339068038] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23343082026] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[23353056804] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23355364164] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[23359370892] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[23370714180] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[23372957520] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23375445225] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23383323249] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23385765051] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[23389200087] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[23389955127] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=SynReceived
[23396240736] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 0 bytes on conn_handle=3
[23398758306] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[23400075369] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23402038473] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23404155324] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=124
[23405357448] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 114 byte frame (118 encoded) to netd rx_port=21
[23407596993] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (118 bytes sent)
[23410087668] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23424609780] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[23449964802] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23456383137] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=fec1d29ef0678173)
[23489332779] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23490537081] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23491574205] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23492702772] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=351 pred=0 subj_lo=0
[23502683523] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=Established
[23505050844] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:34280 on listener 2
[23507307087] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=2
[23524123722] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=4
[23532478926] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 38 (user thread) assigned to CPU 1
[23534376426] [INFO] [anther] [CPU1] anther: Thread spawned TID=38 for conn_handle=4
T:5EF0 [23555858634] [INFO] [anther] [CPU1] anther: Worker thread TID=38 starting for conn_handle=4
[23566762131] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[23650745712] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=35, our_read=36)
[23652560283] [INFO] [anther] [CPU1] anther: Worker TID=38 connected to netd OK
[23656688286] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[23698846974] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23706698598] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12225000
[23707963224] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[23709619098] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[23773927089] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[23783582163] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[23786670303] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[23788583808] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[23790892158] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[23793132528] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[23797670952] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[23812795578] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[23822694555] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[23824392768] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[23826626637] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[23833656033] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[23835370086] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[23837758593] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[23839288572] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=1 (16384b)
[23851463691] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x12231000
[23852922687] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[24076359681] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=14aa2015f9906e3f)
[24334982859] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[24563924352] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[24775010997] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=788 watches=18 history=1024 journal=1024 symbols=363 drops=0
[24816096426] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[25062392619] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[25312922976] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[25581370584] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[25879875153] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[26201777580] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[26350270617] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=817 watches=18 history=1024 journal=1024 symbols=364 drops=0
[26517965892] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 2 bytes on conn_handle=4
[26539302504] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[26860548033] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[27114724593] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27187778508] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[27501615504] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[27510625527] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[27634731927] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27651240804] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[27680270838] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[27681469563] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[27682688022] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[27684021915] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=352 pred=0 subj_lo=0
[27740442180] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27936172473] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28030071663] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28055744310] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[28124407773] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28144819098] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=124
[28147644129] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 114 byte frame (118 encoded) to netd rx_port=21
[28151139852] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (118 bytes sent)
[28155920463] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 114 bytes
[28175829759] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[28182681879] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[28231223889] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28242962583] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[28251797937] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[28272098316] [INFO] [anther] [CPU1] anther: OTHER /health Http11
[28290803838] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 219 bytes - TCP ACK
[28294284117] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 219 bytes
[28302180258] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[28305113892] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[28308695283] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28314067683] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28355980917] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28367353608] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 77 bytes - TCP ACK
[28370781252] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 77 bytes
[28381905453] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[28384744806] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[28387500174] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28389315867] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28396030410] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[28398398886] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[28401553290] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28410859851] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28413473088] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[28417894329] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[28453561125] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 0 bytes on conn_handle=4
[28469410332] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28558426842] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29006973633] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=882 watches=19 history=1024 journal=1024 symbols=367 drops=0
[29178794469] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[29681190924] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[29928650697] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2429 ops=1 watches=19
[30085343112] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30310781259] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=1
[30376247187] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[30428832819] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30531287325] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30586555956] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[30588598656] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[30632911254] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30783643044] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30876252924] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30956803218] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[30966805155] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[31085598357] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[31260605838] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[31325004381] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[31348706763] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[31452169980] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[31473250644] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[31536575631] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[31673323770] [INFO] [fontd] [CPU
```
</details>
