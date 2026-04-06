# ✅ Scenario: Inspecting Running Processes

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 5265ms | - - - |
| 2 | And the anther server is ready | ✅ | 2074ms | - [📜](./02/serial.log) - |
| 3 | When I execute the GQL query "MATCH (n:proc.Task) RETURN n" | ✅ | 336ms | - [📜](./03/serial.log) - |
| 4 | Then the GQL result should have at least 1 row | ⏭️ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11431029621] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11436364665] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11440128612] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11442084918] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11443232922] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11443852035] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11444501310] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11445086796] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11445665484] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11446267767] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11446852131] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11447447055] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11448145368] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11448851832] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11449533909] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11450155431] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11450775270] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11451365904] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11452009074] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11452598454] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11453186943] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11453761407] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11454340260] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11454927363] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11455551096] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11456140179] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11456724840] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11457355800] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11457930693] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11458532745] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11459125458] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11459723517] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11460331014] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11460926730] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11461515483] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11462191851] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11462882508] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11463587454] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11464336356] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11465059650] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11465752551] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11466447960] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11467686252] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11469062187] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11469804621] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11470345128] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11470846365] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11471375850] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11471920779] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11472433269] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11472942393] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11473461681] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11473964997] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11474493954] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11475045945] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11475609420] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11476142832] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11476699575] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11477233086] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11477785242] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11478335220] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11478890082] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11479426464] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11479979874] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11480515926] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11481071613] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11481617532] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11482171701] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11482705344] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11483255421] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11483788206] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11484339042] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11484882915] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11485435698] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11485979109] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11486531925] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11487066558] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11487617856] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11488163379] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11488891161] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11489428269] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11489982405] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11490517764] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11491091832] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11491692102] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11492248581] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11492782950] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11493335238] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11493870894] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11494435062] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11494969332] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11495521026] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11496054966] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11496607353] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11497148091] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11497710576] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11498244582] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11498796408] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11499331536] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11499883890] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11500417566] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11500985298] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11501521482] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11502075486] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11502627444] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11503194219] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11503727829] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11504496729] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11736638463] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[11747782596] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11752518624] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11753772954] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11754671676] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11758818159] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11760552969] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11761636425] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11762311308] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11762974971] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11763636654] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11764603290] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11765541645] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11766294441] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11766972624] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11767640082] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11768356974] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11769535371] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11770571373] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11771289882] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11772827946] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11773756368] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11774756202] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11776337826] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11777907636] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11778663402] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11779188432] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11779935552] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12141625122] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12142588359] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12145913274] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12146785365] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12147534069] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12148997520] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12161083044] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12162335361] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12163085352] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12164736969] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12165265233] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12167677896] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12175239879] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12176904729] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12189767766] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12190304511] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12206089995] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12206640072] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12208532853] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12209755305] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12210773289] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12213206907] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12214044579] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12248836281] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62311300 ticks/sec), init_cnt=623113 for 100Hz
[12250284222] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12251083383] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12252221322] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12257661141] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12288010284] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12289062027] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12292067766] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12293416245] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12294556296] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12297266619] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12298631664] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12317115822] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12318522480] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12319869276] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12320889933] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12321950619] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12322733940] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12323286525] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12349140804] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12350919669] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12351722262] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12352602570] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12353347809] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12354204060] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12354826209] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12355491621] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12361776141] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12362715783] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12364492107] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12365299221] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12371437353] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12373446459] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12374418969] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12376385241] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12378161334] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12379425135] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12398936946] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12402432834] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12403825005] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12404609778] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12434815767] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12466907211] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12470626179] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12476815659] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12479743518] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12483531060] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12486751101] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12489554847] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12490801125] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12492958500] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12504545361] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12508047519] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12514175025] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12519265902] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12523886232] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12528541311] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12529840554] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12551551221] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12552791229] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12566034327] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12567135009] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12614331510] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12615631479] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13184233359] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13975763934] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[14001295176] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=497 journal=422 symbols=51 drops=0
[14039539701] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[15284753649] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=961 journal=769 symbols=95 drops=0
[15982341672] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[16076564097] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[16077679299] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[16184871186] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[16247975073] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[16281590391] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[16283229765] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[16284439083] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[16289478183] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[16305986367] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[16326024132] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[16328618526] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[16383917616] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[16405043655] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[16406163906] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[16410651906] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[16439187864] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[16460675319] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[16466094480] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[16467667722] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[16540521789] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[16542541059] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[16628544669] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[16692604632] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[16704610032] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[16708654875] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[16710840003] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[16724108643] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[16777333023] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[16782753735] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[16783998000] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[16784871840] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[16785764754] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[16786438944] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[16787092971] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[16787802933] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[16788411783] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[16789028553] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[16789673241] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[16790298525] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[16791272520] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[16792158900] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[16792854705] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[16793582883] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[16794252552] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[16794949611] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[16795588161] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[16796243244] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[16796877240] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[16797507837] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[16798128072] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[16798757646] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[16799386593] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[16800059298] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[16800715305] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[16801349796] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[16802038374] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[16802661084] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[16803296202] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[16803939801] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[16804606104] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[16805246139] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[16805890794] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[16806510666] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[16807260888] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[16808004246] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[16808755986] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[16809492084] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[16810260588] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[16811023746] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[16811806077] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[16813348695] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[16815237648] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[16816381593] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16825558398] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[16840326459] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[16845089481] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[16855090659] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[16856208897] [CONTRACT] [kernel] [CPU0] Spawning init process...
[16858977432] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[16862077089] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[16862899515] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [16869378768] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013568 RFLAGS_BEFORE=130 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[16873363221] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[16886389641] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[16891428114] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[16892804511] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[16894219419] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[16903874394] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[16908900162] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[16913378691] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[16914941472] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[16918559460] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[16921593480] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[16923443097] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[16928822493] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[16930537866] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[16932172719] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[16933840836] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16938927753] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16940627550] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16942540263] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16944765552] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16946886330] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16954725117] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16961264793] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[17014387632] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[17022309282] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[17027304492] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[17031737184] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[17034473775] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[17038262439] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[17042844489] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[17048126205] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[17052955491] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[17057557638] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[17062752828] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[17068033620] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[17073167364] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[17078011434] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[17085076569] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[17090941032] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[17096147640] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[17101034874] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[17106075921] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[17110597746] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[17115560748] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[17121020895] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[17126509191] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[17131379595] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[17135841987] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[17140465419] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[17146201677] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[17151226983] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[17157664623] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[17163635841] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[17167032630] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[17170875018] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[17176083210] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[17182289454] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[17188661919] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[17195289078] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[17201155851] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[17207330976] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[17212653546] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[17217427095] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[17222927502] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[17226178068] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[17246138052] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[17401080708] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[17410176366] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[17411415351] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17413127886] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17417704689] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17418967896] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17427292905] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[17432293296] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[17433587160] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17435395527] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079104 RFLAGS_BEFORE=130 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[17439424101] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[17440336782] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17442302526] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17443175277] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[17447843160] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[17449842696] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17456860146] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[17460377517] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[17461666794] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[17463079590] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[17464772490] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144640 RFLAGS_BEFORE=130 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[17472370179] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[17474112678] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[17476382682] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17480912394] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:53:33 = 1775436813 unix_secs
[17482610805] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775436813, mono_ns=8741056137, offset=1775436804258943863ns
[17484178272] [INFO] [rtc_cmos] [CPU1] System clock anchored
[17494987125] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[17537898939] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[17552868234] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[17554229550] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17557082961] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17565581253] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[17569255803] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[17581561272] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[17588164011] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[17593369794] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17595999828] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210688 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[17603652099] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[17612604339] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[17615443395] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[17616648654] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17619450750] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17629802058] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17633440935] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17644255035] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[17657825064] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[17659071540] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17661677451] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277440 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[17666844096] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[17671080537] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[17675346018] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[17685322644] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[17686600536] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[17687996865] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[17689276110] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[17691516942] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[17707458747] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[17709628662] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[18309830121] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18318418932] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[18323450211] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[18327119877] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[18334229199] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[18337085943] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[18340624797] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[18343027593] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[18344586843] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[18348476949] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[18350059464] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[18351529185] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[18352937493] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[18354386655] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[18355608150] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[18357031671] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[18363037407] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[18363977940] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18365808120] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18372939222] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18375377262] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18383067846] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[18386314815] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[18387819087] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[18390184824] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352912 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[18401800593] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[18442005549] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[18445754679] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[18466137261] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[18468881640] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[18475315881] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[18481180806] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[18482664915] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[18483897795] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[18485329962] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[18486205914] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[18487820571] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[18488718963] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[18490305603] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[18492002958] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[18493991505] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[18504766830] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[18508041981] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[18509954727] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[18513960531] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[18522154959] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[18523499742] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18526166505] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18531889530] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18534487290] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18545551992] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[18550690851] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[18554035797] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[18555308046] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18558289860] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18565562103] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18567910944] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18578836914] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[18584304552] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[18585922014] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[18588425757] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369503200 RFLAGS_BEFORE=130 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[18597478581] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[18598856331] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18600973149] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[18602090925] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18603332880] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[18610190676] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18611520246] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[18614010030] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18615258981] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[18617771667] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[18620955804] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[18629109939] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[18630821418] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[18634448877] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[18642603210] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[18647524797] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[18650000985] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[18669720267] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[18673451115] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[18675797580] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[18679141701] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[18681680589] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369437664 RFLAGS_BEFORE=130 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[18691073841] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[19078547433] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
[19080452754] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[19083377709] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369585120 RFLAGS_BEFORE=130 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[19087535907] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[19091125977] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[19097494152] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19099430196] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[19101773361] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[19103037096] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[19104176718] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19106794971] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19108043064] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[19118688237] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[19122057801] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[19129305492] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[19133304234] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[19138256544] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[19141586541] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[19143618516] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[19144746555] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19147322205] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19168173156] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19172834868] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[19193959587] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[19197358422] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
[19198275789] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19201168239] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716880 RFLAGS_BEFORE=134 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[19205305845] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[19207693626] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650656 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[19211817174] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[19213905513] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[19241290299] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[19243257759] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[19246614057] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[19248074835] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[19272636504] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[19275591423] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[19277729691] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[19280352894] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[19282046652] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[19306476552] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[19316228943] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[19318446906] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[19321015725] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[19322216133] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19324656021] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19395585660] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[19420796703] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[19428966612] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[19431897969] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[19433484642] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19435202754] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369782944 RFLAGS_BEFORE=134 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[19440073323] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[19441258221] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19444119519] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[19445450376] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19447753347] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[19453997178] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19455579594] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19474773945] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[19478548518] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[19486226397] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[19489246095] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[19491708588] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[19492425909] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19494000207] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19499844012] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19502376135] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19509533208] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[19512414570] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[19513599534] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19515082818] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915328 RFLAGS_BEFORE=134 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[19518580983] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19519614444] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19521199203] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[19522048557] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19523028591] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[19530341820] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19534325415] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19541356890] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[19544214030] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[19545736980] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19547348733] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981856 RFLAGS_BEFORE=134 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[19551853497] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[19552881414] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19554845013] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19565200776] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19569771804] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[19577249868] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[19580037411] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[19582321803] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[19583206533] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19584702951] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19610229342] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[19615484922] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[19623059874] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[19626094191] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cc18
[19627712280] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19629272916] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370114128 RFLAGS_BEFORE=134 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[19632586281] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[19633449495] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19634865987] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[19635921591] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19654006119] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[19657654632] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19658822832] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[19665040395] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19667038182] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19669143318] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19670838561] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[19672528656] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[19675467834] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[19676916633] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19679020878] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370198304 RFLAGS_BEFORE=130 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[19683440436] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[19684394730] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19686310116] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19724306844] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[19734525129] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[19735568688] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19737041478] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19738680753] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19740286665] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[19741486017] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=229 pred=0 subj_lo=0
[19743312204] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[19746326952] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[19749339291] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[19752094758] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[19753717698] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[19755912726] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[19757766831] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[19758670503] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19760082375] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[19760991030] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19762428906] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[19763281560] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19764910143] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[19766559219] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4475000
[19768160115] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[19771061772] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[19772115594] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19773224295] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19774389459] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19776084174] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[19777985073] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[19780545576] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[19787588799] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
[19789553190] [INFO] [fontd] [CPU3] FONTD: Service ready
[19792302882] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[19801503447] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[19812471063] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[19815986289] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[19818784458] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[19821027270] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[19823190651] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[19825278231] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[19827212724] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[19828993041] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[19830712704] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[19836797475] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=19, read=20)
[19840311216] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19842746055] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849248 RFLAGS_BEFORE=134 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[19849049715] [INFO] [nectar] [CPU2] NECTAR: Started.
[19851645000] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19854350307] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370048208 RFLAGS_BEFORE=130 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[19861251300] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[19863957663] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
[19865182590] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19867046793] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370266784 RFLAGS_BEFORE=134 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[19872022269] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19874625243] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[19876582737] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[19879471194] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[19880930355] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19882378494] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19884008727] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19885521876] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[19886230023] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[19887622986] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[19889469171] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[19892782767] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[19894731912] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[19897870509] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=21, read=22)
[19909244883] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([227, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19912150170] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([227, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19918732515] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[19931746329] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1248 backend=VirtIO-GPU
[19933788402] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[19934693955] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19936570863] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19941060150] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19942667679] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19944481425] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19946399154] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[19952166234] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([227, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19956174744] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19957703700] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19959460158] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19961334492] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[19970395005] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19972272111] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19974059622] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19976041701] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[19984967178] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19986503658] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19988225466] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19990236783] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=244 subj_lo=0
[19999987260] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20001749922] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20003636862] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20005530666] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=245 pred=0 subj_lo=0
[20034135792] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([227, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20035652175] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20037556242] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20055302355] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[20067113088] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([227, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20076791889] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=23, resp=26
[20078735556] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[20081462973] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[20095889715] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[20103846312] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[20107091301] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[20109211419] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[20110081398] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20111878875] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20114779905] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db480
[20116329717] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e0
[20117993115] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20118933648] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370494800 RFLAGS_BEFORE=134 CR3_BEFORE=72167424 fs_base=0 gs_base=18446744071564586640
[20125351092] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[20126479395] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[20127888000] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[20128797942] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db480
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[20131130778] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370564432 RFLAGS_BEFORE=134 CR3_BEFORE=73216000 fs_base=0 gs_base=18446744071564586576
[20134788696] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[20136223866] [INFO] [echo] [CPU1] echo: starting up
[20138729193] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[20153892462] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[20155661988] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20157520581] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20158660830] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([227, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20176805088] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[20179245999] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[20201762427] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=19, RX port=22
[20216560716] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[20218432938] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[20220361524] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[20221432275] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20223314133] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20228348844] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20229363231] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[20230453848] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[20232366165] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[20239793277] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[20242906134] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[20244992592] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[20246271474] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([227, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20248141980] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[20249989155] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[20250823560] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20253043008] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20260946838] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20264066526] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[20266034844] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[20275684440] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[20279496567] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[20281692024] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[20283884742] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[20285146464] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20287738581] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20289716832] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f2510
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20291760654] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[20293063329] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370711888 RFLAGS_BEFORE=134 CR3_BEFORE=73981952 fs_base=0 gs_base=18446744071564586640
[20297594658] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[20306348733] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[20310015099] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[20312187522] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[20313618072] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[20314919889] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f2510
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20317376343] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[20318614800] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370777424 RFLAGS_BEFORE=134 CR3_BEFORE=74129408 fs_base=0 gs_base=18446744071564586576
[20322892788] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[20325585951] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[20333666529] [INFO] [netd] [CPU3] NETD: Driver TX port=19, RX port=22, link_up=true, mtu=1500
[20337469812] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[20343514917] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[20359714089] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[20361536415] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[20382958398] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[20398654947] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[20422296675] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[20424235128] [INFO] [anther] [CPU1] anther: Connected to network stack
[20425059303] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[20428229778] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[20435265213] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[20461638417] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[20464770909] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([237, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[20476905339] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[20481998526] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[20505728694] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[20508790698] [INFO] [bloom] [CPU3] bloom: creating surface...
[20509914744] [INFO] [bloom] [CPU3] bloom: surface created!
[20510934906] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[20513392482] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[20515012518] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[20520906912] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[20549966976] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[20556285717] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[20568008010] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1264
[20569073910] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[20577382419] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[20581756701] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[20584119039] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[20587220610] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[20588542029] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[20589713760] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
[20592363198] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
T:0270 [20597670720] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[20607884253] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[20618376933] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[20627164800] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[20628237597] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[20630798199] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
T:07D0 T:0640 T:F0B0 [20651268264] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[20653503387] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[20663956929] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=277
[20665174926] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[20666125029] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20667114699] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20668398267] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20669732952] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=277 subj_lo=0
[20673156603] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[20679571902] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 5)
[20680434819] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[20683264338] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[20687882523] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[20688962382] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[20690601756] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[20692922877] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[20697285543] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[20699621382] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[20702208516] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[20705936460] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[20713768317] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1265)
[20715172665] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[20719896450] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1264
T:1220 [20726181729] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[20728023327] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[20733523503] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[20737220328] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[20739379749] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[20746547415] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=245
[20747875236] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[20748802404] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20749838868] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20750908497] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20752095276] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=245 pred=0 subj_lo=0
[20768466345] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1271)
[20769814296] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[20787318453] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=282
[20788767219] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[20789992740] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20791174866] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20792428404] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20793820839] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=282 subj_lo=0
[20802849606] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[20805991965] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[20811355191] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[20814019116] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[20816741748] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[20819065542] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[20825226213] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[20834881320] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1274)
[20837467464] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[20846817024] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[20849684988] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[20857850574] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[20896377018] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[20948601630] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[20950245921] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[20953951722] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[20959116486] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[20965139943] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[20970572304] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[20976956385] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=74
[20979017862] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=21
[20982957171] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[20995251387] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[21000098427] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[21002718066] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[21008799438] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[21010993608] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[21013847976] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[21023776224] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[21025261818] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=149
[21026648709] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=21
[21028847367] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[21029962404] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[21032886270] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:34646 on listener 1
[21039434394] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[21042648528] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[21056360589] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=3
[21065468919] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 37 (user thread) assigned to CPU 1
[21067748295] [INFO] [anther] [CPU1] anther: Thread spawned TID=37 for conn_handle=3
[21089523015] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[21101930355] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[21208046937] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
T:5EF0 [21257755233] [INFO] [anther] [CPU1] anther: Worker thread TID=37 starting for conn_handle=3
[21263075559] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21309176394] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[21337367304] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[21352596342] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=33, our_read=34)
[21354235551] [INFO] [anther] [CPU1] anther: Worker TID=37 connected to netd OK
[21424894392] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[21589800210] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[21593653521] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=663 watches=13 history=1024 journal=1024 symbols=317 drops=0
[21740361225] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[21931880553] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[22204662810] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[22415042430] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[22432691820] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[22629780624] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[22861500816] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[23102759856] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[23113968273] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[23120810427] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[23123391852] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[23125089438] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[23126931069] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[23129243214] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[23142172878] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[23143891881] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[23145793044] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[23148069384] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=341 pred=0 subj_lo=0
[23293366305] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[23476033086] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[23550609060] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23565432264] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 85 bytes on conn_handle=3
[23577590289] [INFO] [anther] [CPU1] anther: GET /health Http11
[23608213860] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[23611245966] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[23614847619] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[23618532795] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[23622902523] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[23624550180] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23626848465] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23639902374] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[23643271377] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[23663039334] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23668002930] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[23672485155] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[23682276420] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[23684656413] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23691460254] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23692962150] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23701635540] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[23704664709] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23710209336] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23728817871] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23733607755] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[23737006689] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[23754809826] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[23775799476] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23777952429] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=723 watches=15 history=1024 journal=1024 symbols=349 drops=0
[23780903058] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23785332813] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23792346996] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23795895189] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[23800283001] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[23801536968] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[23817771351] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[23820483258] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23825797281] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23827418373] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23840628174] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=179
[23843018298] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 169 byte frame (173 encoded) to netd rx_port=21
[23847308133] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (173 bytes sent)
[23866749522] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 169 bytes
[23894120811] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23895547731] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23897284752] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23899159911] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=307 pred=0 subj_lo=0
[23935018800] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[23938185810] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23989598061] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 0 bytes on conn_handle=3
[24025203378] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[24042756969] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[24045231309] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[24046161843] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[24048636414] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:34650 on listener 1
[24051392112] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[24066809316] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[24068145156] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24069412686] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24070977513] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=308 pred=0 subj_lo=0
[24106550655] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24132782223] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=4
[24142019616] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 38 (user thread) assigned to CPU 1
[24144918204] [INFO] [anther] [CPU1] anther: Thread spawned TID=38 for conn_handle=4
[24268180893] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24400137696] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24405705456] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
T:5EF0 [24434296920] [INFO] [anther] [CPU1] anther: Worker thread TID=38 starting for conn_handle=4
[24479333307] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[24489127707] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[24585102102] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[24586886907] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24588737778] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24590710122] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=309 pred=0 subj_lo=0
[24599462778] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24627407706] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[24637825608] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=35, our_read=36)
[24639636912] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[24641300112] [INFO] [anther] [CPU1] anther: Worker TID=38 connected to netd OK
[24647450190] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x121fd000
[24649172625] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[24650977758] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[24678974199] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[24681341256] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[24686390421] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[24710017497] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 115 bytes on conn_handle=4
[24712634463] [INFO] [anther] [CPU1] anther: POST /api/v1/query Http11
[24713711418] [INFO] [anther] [CPU1] anther: Request body size: 28 bytes
[24725639862] [INFO] [phloem::executor] [CPU1] phloem: calling find for kind: proc.Task
[24761873730] [INFO] [phloem::executor] [CPU1] phloem: find returned 0 candidates
[24763530000] [INFO] [phloem::executor] [CPU1] phloem: discovered 0 nodes
[24764287746] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[24774006114] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[24778515498] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[24780493683] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[24782487708] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[24787065864] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[24797360214] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 209 bytes - TCP ACK
[24802270119] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[24804372681] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[24810866157] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 209 bytes
[24814020033] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[24815082732] [INFO] [virtio_netd::driver] [CPU2] VirtI
```
</details>
