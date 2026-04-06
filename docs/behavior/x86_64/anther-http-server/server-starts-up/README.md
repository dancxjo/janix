# ❌ Scenario: Server starts up

> Last run: 2026-04-05 17:52:25

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ❌ | 1000ms | - [📜](./01/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[13971010251] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[13979587479] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[13983564540] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[13985673504] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[13986902259] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[13987560081] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[13988271528] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[13988891202] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[13989516882] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[13990156422] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[13990774875] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[13991431476] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[13992165990] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[13992857373] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[13993572648] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[13994210241] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[13994903241] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[13995531561] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[13996173246] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[13996870767] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[13997480970] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[13998122457] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[13998759720] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[13999425627] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[14000181063] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[14000829942] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[14001531423] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[14002466841] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[14003336490] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[14004262107] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[14005205412] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[14006179473] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[14007167856] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[14007861549] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[14008495215] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[14009222040] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[14010009189] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[14010799143] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[14011575600] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[14012336679] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[14013110496] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[14013853788] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[14015302950] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[14016788412] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[14017614864] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[14018186094] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[14018723466] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[14019272454] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[14019867312] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[14020443855] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[14021139858] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[14021695743] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[14022237009] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[14022805071] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[14023380393] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[14023969674] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[14024643468] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[14025238953] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[14025811932] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[14026404513] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[14026977789] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[14027745567] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[14028326862] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[14028923634] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[14029708671] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[14030537070] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[14031367713] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[14032186113] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[14033040186] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[14033930889] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[14034652962] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[14035249668] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[14035821591] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[14036408661] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[14036977614] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[14037594714] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[14038168287] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[14038758261] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[14039330052] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[14039920125] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[14040493368] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[14041104990] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[14041748523] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[14042341830] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[14042916195] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[14043507885] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[14044139637] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[14044735881] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[14045305956] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[14045894346] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[14046464091] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[14047053405] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[14047653081] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[14048243286] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[14048810457] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[14049398286] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[14049969120] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[14050574010] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[14051174610] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[14051766564] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[14052336705] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[14052925920] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[14053680201] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[14054534868] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[14055192855] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[14055789891] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[14056398840] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[14057651982] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[14328498987] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[14341493760] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[14346710169] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[14348091417] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[14349038352] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[14353496784] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[14355399069] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[14356926804] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[14357835723] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[14358895320] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[14359895418] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[14361486051] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[14362687383] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[14363489250] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[14364238911] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[14364985437] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[14365730016] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[14367085095] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[14368247751] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[14369048496] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[14370849207] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[14371984143] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[14373140628] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[14375058192] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[14376822207] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[14377718652] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[14378361459] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[14379216588] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[14831873232] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[14833485876] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[14838024564] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[14839607409] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[14841064887] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[14843245098] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[14860081137] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[14862186075] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[14863529340] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[14866350147] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[14867319984] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[14870582265] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[14880321852] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[14882703396] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[14899672689] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[14900413275] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[14921558058] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[14922338178] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[14924849445] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[14926447932] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[14927853369] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[14930550195] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[14931524091] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[14967122940] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62832400 ticks/sec), init_cnt=628324 for 100Hz
[14968797228] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[14969735055] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[14970967770] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[14976996738] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[15008618955] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[15010271232] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[15011410491] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[15013494474] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[15015276837] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[15020274258] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[15022242543] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[15045488997] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[15047060886] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[15048402897] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[15050445333] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[15051719067] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[15053752230] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[15054985869] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[15083581722] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[15085059363] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[15086240598] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[15087781566] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[15088591815] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[15090060513] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[15090899406] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[15092134761] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[15099088983] [INFO] [kernel::root] [CPU0] Spawning Root service...
[15100317837] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[15102751983] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[15103903881] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[15108491145] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[15110998782] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[15112457580] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[15113997129] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[15115182720] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[15116280696] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[15131998332] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[15136456929] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[15138073731] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[15139042314] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[15165280383] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15198653910] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15202222464] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15208129596] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15211274529] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15216944721] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15221555745] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15225291675] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[15226637085] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[15228543561] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15241544274] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15246615021] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15252008013] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15256414503] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15260609562] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15266235567] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[15267478743] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[15290530464] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[15292133274] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[15301914771] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[15303210054] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[15341136888] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[15342680661] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[15802364523] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[16435564365] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[16468711710] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[16514706945] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[18790078560] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=961 journal=769 symbols=95 drops=0
[20168272806] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[20341598970] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[20343877422] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[20504140206] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[20595934260] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[20632631118] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[20633867694] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[20634968574] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[20642378031] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[20663279043] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[20688130617] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[20691772893] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[20761390089] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[20792044152] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[20793041940] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[20797819251] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[20838278373] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[20862315738] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[20867357610] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[20869068165] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[20951350530] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[20953251858] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[21089988843] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[21186423159] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[21202033149] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[21208177881] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[21211463097] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[21244999743] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[21340580052] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[21347147085] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[21348608358] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[21349602285] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[21350998482] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[21352044879] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[21353050686] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[21353923668] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[21354874233] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[21355846083] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[21356789289] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[21357575712] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[21358367481] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[21359078334] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[21359862843] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[21360643161] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[21361350615] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[21362080872] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[21362784465] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[21363488586] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[21364192872] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[21364852641] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[21365544684] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[21366286623] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[21366962397] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[21367723707] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[21368428686] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[21369415848] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[21370325196] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[21371357106] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[21372316680] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[21373310937] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[21374045451] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[21374786796] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[21375829134] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[21376683603] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[21377644596] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[21378790851] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[21379878432] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[21380717985] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[21381587766] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[21382392768] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[21383299377] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[21385055439] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[21387046296] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[21387905286] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21396728562] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[21415300104] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[21421063587] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[21432864915] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[21434123205] [CONTRACT] [kernel] [CPU0] Spawning init process...
[21438902298] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[21444722145] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[21446091117] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [21454881789] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[21456238386] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013312 RFLAGS_BEFORE=130 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[21495183633] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[21499871316] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[21502423173] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[21505234509] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[21520663263] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[21528092619] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[21533326815] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[21535229727] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[21543146262] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[21550911723] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[21553061673] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[21559953129] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[21562174656] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[21564408360] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[21566284575] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[21574073829] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[21576092406] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[21578467152] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[21581257170] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[21583951554] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[21586788663] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[21594422421] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[21666604179] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[21678260373] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[21688912641] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[21698205705] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[21704416866] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[21711773259] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[21722239077] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[21730742253] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[21740420922] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[21750360786] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[21759799215] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[21771130128] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[21780232056] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[21787520436] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[21795113769] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[21802990209] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[21810001026] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[21818409129] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[21827876664] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[21836064822] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[21844019670] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[21852495489] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[21861641076] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[21870715581] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[21880490148] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[21887544393] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[21895227684] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[21903114486] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[21911140977] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[21918200766] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[21923817036] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[21929466834] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[21937631925] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[21945853446] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[21953901288] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[21962252730] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[21970488936] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[21979498101] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[21989743017] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[21998335161] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[22007008452] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[22012202586] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[22043538066] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[22267447851] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[22277113782] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[22278540372] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22281464535] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22287575706] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22289697342] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[22300281828] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[22307277663] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[22308779328] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22311060585] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078848 RFLAGS_BEFORE=130 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[22317995106] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[22323781656] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[22325226066] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[22328680176] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22336104912] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[22339113423] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[22352126214] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[22358074002] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[22359590088] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[22361809470] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144384 RFLAGS_BEFORE=130 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[22366544970] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[22370855925] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[22373519487] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[22376350788] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22381378272] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:45:47 = 1775436347 unix_secs
[22383402393] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775436347, mono_ns=11191450099, offset=1775436335808549901ns
[22385641443] [INFO] [rtc_cmos] [CPU1] System clock anchored
[22400888202] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[22449833472] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[22464483096] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[22465793658] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[22468516092] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22475320659] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[22478041905] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[22487785782] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[22490949393] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[22494357072] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22496238369] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369211008 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[22503218331] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[22508837472] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[22511527599] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[22512692730] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[22515300027] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22526443269] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22529849331] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[22540513941] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[22545393915] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[22547399490] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22549525449] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277600 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[22553925768] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[22557808713] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[22561387992] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[22571129064] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[22572277497] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[22573484406] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[22574654355] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[22575972375] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[22599597339] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[22601921925] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[23162355744] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[23169080121] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[23172480606] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[23174467569] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[23179356981] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[23181419382] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[23184468318] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[23186536527] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[23188093071] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[23191138938] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[23193049836] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[23194513254] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[23195954430] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[23197282812] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[23198597400] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[23200244331] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[23205536178] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[23206967487] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[23209680582] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23217992952] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[23221221771] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[23230073394] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[23234082201] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[23235181530] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[23237625840] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352912 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[23247357144] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[23261385015] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[23285349516] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[23303062431] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[23306450079] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[23314640580] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[23329081281] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[23332037454] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[23336123151] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[23337607557] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[23339512548] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[23340937191] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[23342307714] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[23344021998] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[23346654474] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[23348104329] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[23353322421] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[23359513749] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[23363800944] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[23368014615] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[23371128000] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[23372221554] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23374787007] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23381173101] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[23383954572] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[23396734614] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[23411513829] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[23414867124] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[23415759015] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23417929854] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23426394156] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[23428665711] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[23441249502] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[23446143435] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0105668
[23447735520] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[23449906029] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500256 RFLAGS_BEFORE=134 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[23454867711] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[23456048517] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23458597734] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23459769465] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[23462811009] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[23469341940] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[23470457472] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[23473041042] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[23474161491] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[23476528647] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[23479833597] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[23482171350] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[23484500226] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[23487321594] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[23489069340] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[23492304792] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[23495283735] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[23496737088] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[23499537369] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[23501875287] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[23505056091] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[23507350185] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434016 RFLAGS_BEFORE=134 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[23515508280] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[23930712432] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[23931924291] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0105668
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[23934894060] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583040 RFLAGS_BEFORE=134 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[23939322825] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[23942420304] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[23943631008] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[23947184382] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[23948610048] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[23952144480] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[23953204638] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[23954333667] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23956937631] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23967205812] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[23971109448] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[23975568375] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[23982974004] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[23987885592] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[24029477637] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[24032217990] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[24033423612] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24035921580] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24068247225] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[24073353942] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[24098048832] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[24102575079] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb01095c8
[24106452579] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24108491187] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369718896 RFLAGS_BEFORE=130 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[24115427721] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[24116388351] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
[24117978654] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[24120019638] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653360 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[24128768928] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[24131991246] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[24137040345] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[24139895175] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[24142257876] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[24144262197] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[24146016015] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[24147804648] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[24150352611] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[24160480905] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[24162840174] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[24165311280] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[24166617816] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24169252866] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24250259715] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[24282781908] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[24292368606] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[24297147303] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0102438
[24298377312] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24300326457] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369784432 RFLAGS_BEFORE=130 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[24304676220] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[24305673579] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24307589658] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24308606916] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[24313122273] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[24320971851] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[24323517999] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[24324682140] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[24326170077] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[24328484829] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[24330907359] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[24340915731] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[24345309549] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[24356870175] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[24360902874] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[24363677283] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[24365152053] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24367885080] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24375030306] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[24378237708] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[24386475762] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[24389772495] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010ac38
[24391388241] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24393414045] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915504 RFLAGS_BEFORE=130 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[24398097504] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[24399184590] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24401906529] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24403029783] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[24405224349] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[24413983143] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[24418348548] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[24426124041] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[24429638079] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010ac38
[24430978308] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24432957846] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981648 RFLAGS_BEFORE=130 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[24437378790] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[24438524385] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24440986119] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24454870209] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[24461153904] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[24473605563] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[24478848768] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[24481351983] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[24482634825] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24485281161] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24516578559] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[24523125495] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[24531551913] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[24535159440] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010ca80
[24536820924] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24539183691] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113984 RFLAGS_BEFORE=134 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[24544192563] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[24545527875] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24547886451] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[24549306210] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24570976419] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[24572505738] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[24575973378] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[24579457683] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[24582242520] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24584697687] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[24587297031] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[24589174962] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[24593656131] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[24597724734] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24600001041] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370198112 RFLAGS_BEFORE=134 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[24603922629] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[24605489832] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24607844778] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[24608812074] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24610273149] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[24628834725] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24630733611] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24632569698] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[24633901941] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24635404827] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24637584774] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[24639478743] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[24649966506] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[24656902809] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[24668722683] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[24680384520] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[24683798040] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[24688285083] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24690277986] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849968 RFLAGS_BEFORE=130 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[24695906400] [INFO] [nectar] [CPU2] NECTAR: Started.
[24699154260] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010ac38
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24701365227] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370048000 RFLAGS_BEFORE=134 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[24709487121] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[24713434911] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24716657031] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[24718615515] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24720790611] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370266752 RFLAGS_BEFORE=130 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[24726237789] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[24728346357] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24730144230] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[24731496669] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24733158747] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[24734613288] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24736246128] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[24737815443] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=233 pred=0 subj_lo=0
[24739682715] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[24746505531] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[24749188761] [INFO] [fontd] [CPU3] FONTD: Service ready
[24766623288] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[24771848211] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[24784007457] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24785676432] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24787524828] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24789403617] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=235 subj_lo=0
[24798253557] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[24801816303] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[24803588898] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f4000
[24805749573] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f4000
[24807689808] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f7000
[24810222888] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276783104)
[24811744782] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[24816406164] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[24817663728] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f8000 phys=0x44b8000
[24818810841] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[24821292309] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[24823393617] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24824884161] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24826477665] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[24827973357] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24829726878] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[24831647808] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[24833985462] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[24838617408] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24840157914] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24841384656] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24842851407] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[24848473287] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[24852268485] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24853790478] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24855382464] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24857168127] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[24861078594] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[24869985888] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24871482306] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24872947044] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24874339083] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[24875461083] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[24879338385] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[24883371249] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[24886309074] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[24888638247] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[24891481923] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[24893835417] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[24895751364] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[24897698925] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[24906203091] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=19, read=20)
[24915899613] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=21, read=22)
[24953421174] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[24954369396] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([234, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[24971526426] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1251 backend=VirtIO-GPU
[24974216718] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[24975340566] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24978176091] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24995911050] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=23, resp=26
[24997755123] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[25048183413] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[25049577663] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([234, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[25061709651] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[25132043013] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([234, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[25185860205] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[25198116801] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([234, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[25214094906] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[25215483975] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[25218046293] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[25226867259] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[25231769442] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db448
[25233393966] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e3
[25235571702] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370493728 RFLAGS_BEFORE=130 CR3_BEFORE=72433664 fs_base=0 gs_base=18446744071564586640
[25240281825] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[25241558760] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[25244273802] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[25245514008] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[25252917393] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[25255135653] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[25265810790] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[25269390597] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db448
[25271041983] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[25273042938] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370559264 RFLAGS_BEFORE=130 CR3_BEFORE=73482240 fs_base=0 gs_base=18446744071564586576
[25286628708] [INFO] [echo] [CPU1] echo: starting up
[25289097834] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[25290289299] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[25296302295] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([234, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[25300952490] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[25322370975] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([234, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[25329525639] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[25332667140] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[25346179221] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[25348033458] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[25353318771] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=19, RX port=22
[25362349914] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[25365493329] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[25375023663] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[25377763917] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[25380341481] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[25381665078] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[25382889972] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[25385558946] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[25396633251] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[25399860387] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[25403052444] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([234, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[25411661517] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[25417341543] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[25420536702] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[25424140467] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[25426793799] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[25427935962] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[25429590516] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[25432309947] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[25435397559] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[25438434549] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[25440284793] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[25446524961] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[25452108825] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[25454414997] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[25461017703] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[25465598202] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[25468138476] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[25470322119] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[25471436628] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[25475820150] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[25487564652] [INFO] [netd] [CPU3] NETD: Driver TX port=19, RX port=22, link_up=true, mtu=1500
[25493780994] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010e758
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[25507497972] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[25509825792] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[25511200803] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370710816 RFLAGS_BEFORE=130 CR3_BEFORE=73981952 fs_base=0 gs_base=18446744071564586640
[25521296691] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[25524928011] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[25527326121] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[25532090727] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[25534997301] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010e758
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[25537460355] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370776352 RFLAGS_BEFORE=130 CR3_BEFORE=74129408 fs_base=0 gs_base=18446744071564586576
[25545031611] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[25549237230] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[25554737472] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[25562448186] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[25647460971] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[25649389425] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[25652625768] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[25670263245] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[25692953220] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[25705284759] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([237, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[25739913969] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[25742351712] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[25743817737] [INFO] [bloom] [CPU3] bloom: creating surface...
[25744818330] [INFO] [anther] [CPU1] anther: Connected to network stack
[25745637192] [INFO] [bloom] [CPU3] bloom: surface created!
[25748434371] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[25763842269] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[25794225765] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[25808246937] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1264
[25809938022] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[25825050273] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[25830105906] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[25839311058] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[25840893375] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
[25849376454] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[25850719455] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[25856946819] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[25890934179] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[25891979784] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[25895094654] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1264
T:0270 [25899228003] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[25905946473] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[25910712036] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[25923240057] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[25934734749] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[25940416425] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[25942847898] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
T:07D0 T:0640 T:F0B0 [25962390003] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[25972706925] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[25976055996] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[25978016559] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[25979971809] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[25982785092] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[26000168502] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[26004465399] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 5)
[26013789879] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[26017462086] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[26020566858] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[26024319882] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[26027653047] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[26034836190] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[26040985575] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[26044709625] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
T:1220 [26062341030] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[26066394486] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[26080927620] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[26084256759] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[26102980134] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=280
[26105156847] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[26106930993] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[26108533011] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26110316166] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26112475224] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=280 subj_lo=0
[26131610736] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[26137061511] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[26143002831] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[26146702956] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[26151393444] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[26154230355] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[26159130954] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[26169946176] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1274)
[26172353394] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[26193213849] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[26197231236] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[262093
```
</details>
