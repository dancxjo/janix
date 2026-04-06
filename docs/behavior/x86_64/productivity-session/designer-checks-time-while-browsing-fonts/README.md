# ❌ Scenario: Designer checks time while browsing fonts

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
[2J[01;01H[01;01H[2J[01;01H[01;01H[11400540657] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11405893455] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11409629682] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11411589552] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11412751548] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11413364391] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11414002149] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11414566251] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11415151341] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11415744450] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11416320036] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11416907304] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11417590206] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11418237336] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11418922515] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11419521498] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11420130612] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11420717748] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11421321120] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11421919608] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11422486647] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11423117013] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11423714610] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11424294288] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11424932904] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11425524990] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11426110080] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11426743944] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11427322830] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11427914916] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11428526109] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11429130009] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11429750343] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11430357147] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11430937155] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11431761924] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11432545575] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11433258969] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11433945072] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11434660677] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11435374896] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11436076773] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11437361298] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11438706345] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11439457260] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11440001034] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11440504647] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11441018457] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11441584077] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11442098217] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11442601830] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11443113462] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11443617471] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11444145867] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11444693634] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11445244536] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11445780225] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11446338255] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11446876023] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11447433657] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11447984196] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11448539850] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11449076034] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11449632678] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11450205624] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11450789691] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11451354288] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11451913539] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11452450944] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11453006829] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11453544795] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11454205158] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11454850737] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11455413651] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11455952640] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11456513904] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11457051870] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11457707976] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11458516641] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11459201259] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11459888583] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11460452850] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11461132980] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11461742952] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11462283558] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11462859111] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11463400014] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11463956262] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11464513962] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11465069880] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11465605833] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11466160596] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11466696813] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11467254744] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11467817526] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11468374797] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11468911311] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11469465282] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11470003215] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11470554843] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11471098716] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11471647506] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11472177882] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11472726837] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11473256850] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11473805277] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11474349447] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11475335454] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11712457548] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[11722900662] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11727898908] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11729152875] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11729996091] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11734087299] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11735764524] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11736915267] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11737590150] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11738280477] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11738940543] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11739910017] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11740849164] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11741541339] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11742209358] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11742867411] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11743527411] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11744649642] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11745645714] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11746326867] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11747892915] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11748805134] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11749805562] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11751389001] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11753118564] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11753907495] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11754458331] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11755409325] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12124471986] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12125468190] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12128617578] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12129489207] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12130270053] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12131756406] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12143593902] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12144856515] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12145589643] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12147276240] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12147816021] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12150181494] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12157624710] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12159301407] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12173040330] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12173609745] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12189130008] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12189732984] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12191712753] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12193033809] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12194112645] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12196376907] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12197194284] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12232292094] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62490800 ticks/sec), init_cnt=624908 for 100Hz
[12233830686] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12234631761] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12235813425] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12241285617] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12271799595] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12272800254] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12275677029] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12277037784] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12278142261] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12280838625] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12282167304] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12300819003] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12302108907] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12303672249] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12304687461] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12305908527] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12307296738] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12308086956] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12333740892] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12335131479] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12336431448] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12337705644] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12338989113] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12340194801] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12341149590] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12342402501] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12348903765] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12350256072] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12352494759] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12353842413] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12361219563] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12362948268] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12363805905] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12365280708] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12366409308] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12367569423] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12379352601] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12382482783] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12384063912] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12385161492] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12406372143] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12426193593] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12428483727] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12431760099] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12433588200] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12436570179] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12439122201] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12441402435] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12442210374] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12443234628] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12449280756] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12450865614] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12453251382] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12455528085] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12459746838] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12460456602] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12474481470] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12475389102] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12481819152] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12482729457] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12511659765] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12512532945] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[12878612373] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13352558769] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13382602398] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=499 journal=424 symbols=51 drops=0
[13415333316] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[14670253191] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=449 watches=0 history=964 journal=773 symbols=98 drops=0
[15379009800] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15474816423] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15475842459] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[15578014122] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[15636606381] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[15668872296] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[15669940506] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[15670606149] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[15674740851] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[15692120400] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[15711708078] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[15714012072] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[15767803590] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[15785972103] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[15786747900] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[15790444098] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[15815442654] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[15836109432] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[15839654523] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[15840621555] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[15924358296] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[15926189763] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[16045757739] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[16164750624] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[16196862363] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[16204078374] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[16208194365] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[16278961578] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[16310071998] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[16316486274] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[16317838515] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[16319186697] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[16320481452] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[16321410501] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[16322295561] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[16323197715] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[16324018491] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[16324862433] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[16325736801] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[16326680238] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[16327657368] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[16329362082] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[16331065344] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[16332016536] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[16332713529] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[16333384947] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[16334027160] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[16334742699] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[16335640728] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[16336541562] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[16337386164] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[16338031644] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[16338678774] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[16339504500] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[16340161299] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[16340801631] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[16341496743] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[16342129023] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[16342789518] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[16343443446] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[16344102654] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[16344748827] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[16345401864] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[16346102784] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[16346850531] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[16347607749] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[16348399551] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[16349314344] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[16350353118] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[16351427334] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[16352997045] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[16354588437] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[16356387498] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[16357291566] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16366727223] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[16381866699] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[16387408125] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[16400069103] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[16400921064] [CONTRACT] [kernel] [CPU0] Spawning init process...
[16403420946] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[16406373555] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[16407169746] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [16415278077] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013552 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[16421603352] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[16451127363] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[16455504153] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[16457558238] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[16459299945] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[16472396787] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[16479986820] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[16495450851] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[16498267335] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[16507067973] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[16512340053] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[16514156934] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[16520641962] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[16522334796] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[16524077460] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[16525815240] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16533134277] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16534941885] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16536950430] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16539390285] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16541717808] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16543860201] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16550774856] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[16614089547] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[16625302518] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[16632872520] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[16640363388] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[16649341401] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[16662537210] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[16671328773] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[16685586060] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[16693118970] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[16705092558] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[16713210129] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[16722815703] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[16730782761] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[16737910332] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[16745212143] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[16753076439] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[16760814081] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[16768189383] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[16777882803] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[16786077264] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[16793697624] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[16801097247] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[16809352461] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[16817821911] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[16826115141] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[16834253502] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[16841709291] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[16849812441] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[16856979282] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[16864520937] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[16870123512] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[16875634545] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[16883448417] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[16892882391] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[16903848720] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[16913860821] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[16922126133] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[16930493481] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[16938991641] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[16947952197] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[16956550116] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[16962050985] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[16997267298] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[17196249675] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[17207068725] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[17208988599] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17211750996] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17219237442] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17221076235] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17234218881] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[17243972295] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[17245303812] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17247453861] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079088 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[17254491507] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[17255992479] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17262191925] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17265219015] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[17270617617] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[17273908839] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17284867149] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[17310609690] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[17315885763] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[17317369773] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[17320973769] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369148720 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[17334120243] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[17336536140] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[17340097599] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17346950280] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:00:28 = 1775437228 unix_secs
[17349125541] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775437228, mono_ns=8674329873, offset=1775437219325670127ns
[17351249850] [INFO] [rtc_cmos] [CPU1] System clock anchored
[17374099710] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[17470113210] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[17493304620] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[17496256602] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17501524491] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17516308854] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[17524899711] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[17544597180] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[17553392472] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[17557358874] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17562449619] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369214256 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[17569494888] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[17585462136] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[17588147775] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[17589334653] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17591878227] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17602564452] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17606411130] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17616123063] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[17621099859] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[17622791142] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17624932974] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369279792 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[17629231851] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[17632253793] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[17636048958] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[17645342253] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[17648192661] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[17649321789] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[17650888728] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[17664371043] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[17729343456] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[17732489082] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[18263875344] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18283143846] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[18288359397] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[18290272374] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[18294730179] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[18296649987] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[18298722024] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[18300175278] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[18301145379] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[18303365652] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[18304310772] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[18305191839] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[18306051423] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[18306828573] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[18307603479] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[18308507217] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[18322848390] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[18324350319] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18327217029] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18335186397] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18339772275] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18347472924] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[18352403916] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[18354119223] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[18356134896] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369357616 RFLAGS_BEFORE=134 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[18363800037] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[18384510012] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[18398553063] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[18408284895] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[18410719965] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[18417679005] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[18429255603] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[18431562567] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[18433297575] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[18434511216] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[18435363177] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[18436187748] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[18436984269] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[18438107193] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[18438974730] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[18439830420] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[18446199552] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[18927047502] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[18930339021] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[18933092244] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[18935344824] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[18936236781] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18938071713] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18941949642] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18944233770] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18951402393] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[18955717803] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[18958245009] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[18959337078] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18961296585] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18966210120] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18967736568] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18976109163] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[18980371047] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb01080c8
[18981501792] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[18982994943] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500992 RFLAGS_BEFORE=130 CR3_BEFORE=68513792 fs_base=0 gs_base=18446744071564586640
[18986907588] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[18987906135] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[18988832214] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18990556926] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[18991322361] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18992235405] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[18997215270] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[18998087493] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18999027333] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[19000472865] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[19001299680] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19002464514] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[19004104185] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[19005346833] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[19007197704] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[19008819159] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[19010421738] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[19011658215] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[19015247757] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb01080c8
[19016447472] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[19018063812] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583936 RFLAGS_BEFORE=130 CR3_BEFORE=68894720 fs_base=0 gs_base=18446744071564586576
[19022745159] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[19023779940] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[19024656552] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[19026625497] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0103240
[19027702485] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[19029242562] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434944 RFLAGS_BEFORE=130 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[19032801315] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[19035107124] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[19042434840] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1241
[19043299539] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1240)
[19044505458] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[19046337849] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[19050607719] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19053631377] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[19054800303] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19056651735] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19063312752] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[19066243185] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[19073892684] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[19077029730] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[19079858886] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[19081750842] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[19083084570] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19084743546] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19105192326] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19109475660] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[19130213025] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[19133261895] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
[19134111777] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010ac38
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19136101710] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716880 RFLAGS_BEFORE=134 CR3_BEFORE=69156864 fs_base=0 gs_base=18446744071564586640
[19139583408] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0103240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[19141293600] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650704 RFLAGS_BEFORE=130 CR3_BEFORE=69021696 fs_base=0 gs_base=18446744071564586608
[19145933598] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[19147371573] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[19148418399] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[19150120770] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[19154502147] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[19156300152] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[19159876428] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[19161745548] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[19163016345] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[19164469764] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[19166552361] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[19168521570] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[19170512856] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[19171263639] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19172916807] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19240259577] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[19265640933] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[19273362471] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[19277028045] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0103240
[19278685008] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19280634219] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369783104 RFLAGS_BEFORE=130 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[19283644908] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[19284994014] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19287096543] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[19288315101] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19290199995] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[19294484781] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19295861937] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19316037774] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[19319579004] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[19326615627] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[19329651396] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[19331807418] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[19332571038] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19334202261] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19339704615] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19342297689] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19349689491] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[19352470764] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010c048
[19353965763] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19355603025] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915424 RFLAGS_BEFORE=130 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[19360256916] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19361083368] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19362584571] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19363310967] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[19364601894] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[19367473092] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[19368252651] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[19369919844] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[19370808105] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19371548031] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[19374309009] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19381115127] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[19383814989] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010c048
[19385677344] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19387088853] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981824 RFLAGS_BEFORE=130 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[19389991830] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[19390688823] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19392152505] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19401015051] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19404840312] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[19411448199] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[19413682068] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[19416130767] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[19416874686] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19418359719] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19443017352] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[19447913364] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[19454761128] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[19457994006] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010dfc8
[19459185009] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19460779899] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113984 RFLAGS_BEFORE=134 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[19465297368] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[19466559684] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19468759893] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[19470193050] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19488309588] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[19491527517] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19499411316] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[19501873776] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[19502826717] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[19503566808] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[19504496847] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[19505275977] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19507213011] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19508239047] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370197968 RFLAGS_BEFORE=130 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[19514847198] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[19516779183] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[19521468813] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19522987044] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19523860422] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19524927642] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19525953117] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19528479300] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=226 subj_lo=0
[19530494940] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19532524902] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[19545361209] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[19555401789] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[19562457750] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[19565005350] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[19569274527] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0103240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19570798170] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849344 RFLAGS_BEFORE=134 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[19575233502] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19576742691] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19578059226] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19579605342] [INFO] [nectar] [CPU2] NECTAR: Started.
[19580186241] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19581810699] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19583305731] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[19585175808] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19587250188] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010c048
[19591308627] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19597164708] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047904 RFLAGS_BEFORE=134 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[19602507210] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19603697718] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[19604452890] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19605690060] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19606955742] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[19608761337] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010e758
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19610632503] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370266592 RFLAGS_BEFORE=130 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[19613817432] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19615923327] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19617302331] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19618827327] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[19619933487] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[19621412811] [INFO] [fontd] [CPU3] FONTD: Service ready
[19622079345] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[19623758847] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[19625554971] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[19643883864] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[19645281315] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19646412819] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19647626196] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19648903593] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=235 subj_lo=0
[19657202070] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[19658852961] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[19660406271] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[19661737161] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19663287105] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[19664037657] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19665880740] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[19666796721] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19668674256] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[19669631454] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[19672248651] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x44b6000
[19673570037] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[19675867068] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[19678021110] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[19678930920] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19680574188] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19682316093] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19684174092] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[19684888311] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=244 subj_lo=0
[19691180784] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19692222495] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19693294500] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19694426235] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[19695174939] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=245 pred=0 subj_lo=0
[19702751079] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[19712182644] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[19714572834] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[19716739878] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[19718584479] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[19719748224] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[19720543458] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[19722157389] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[19723988823] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[19725216588] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[19726695582] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[19731966639] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=21, read=22)
[19737281058] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[19769456751] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[19782905736] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19812134232] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19815103605] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[19827871206] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1257 backend=VirtIO-GPU
[19829806590] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[19830720954] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19832463915] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19839018606] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19840453644] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19858645356] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=24
[19859842827] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[19868141370] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19947114891] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19948293618] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19949577846] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19963309905] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[19978107666] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[19985852172] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[19988658954] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[19989417888] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db488
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e9
[19991888895] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370514160 RFLAGS_BEFORE=134 CR3_BEFORE=72433664 fs_base=0 gs_base=18446744071564586640
[19994871105] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[19995620106] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19997142495] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19998212520] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[20001626139] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[20003210601] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20012327412] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[20014997244] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db488
[20016028362] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[20017648695] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370579696 RFLAGS_BEFORE=134 CR3_BEFORE=73482240 fs_base=0 gs_base=18446744071564586576
[20020934769] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[20023004232] [INFO] [echo] [CPU1] echo: starting up
[20025260277] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[20026137450] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[20027055378] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[20030932680] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[20038973163] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=21, RX port=26
[20041389555] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20049440598] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[20051166300] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[20064228162] [INFO] [netd] [CPU3] NETD: Driver TX port=21, RX port=26, link_up=true, mtu=1500
[20066499816] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[20071067148] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[20073936531] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[20075688666] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[20079390474] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[20088933381] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[20090238267] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[20091751647] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[20092517544] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20094167610] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20096460120] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[20098046034] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[20099043921] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20099867106] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[20100917067] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[20107994148] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[20110802019] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[20113456176] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[20115039087] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[20116707963] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[20117496102] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20119308825] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[20121047496] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20126031090] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[20132461635] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20133821928] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[20137766814] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[20141535447] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[20143263426] [INFO] [anther] [CPU1] anther: Connected to network stack
[20148845475] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[20152448547] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[20154223683] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[20156415114] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[20157492861] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20159596809] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20163881529] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[20165470809] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[20166927495] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[20169873240] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[20172052923] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f3580
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20173575180] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370759920 RFLAGS_BEFORE=134 CR3_BEFORE=73986048 fs_base=0 gs_base=18446744071564586640
[20177781855] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[20181493398] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[20183591505] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[20188161576] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[20191522428] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[20192883117] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[20198351151] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[20202868488] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[20208343056] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f3580
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20210379486] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370825456 RFLAGS_BEFORE=134 CR3_BEFORE=74162176 fs_base=0 gs_base=18446744071564586576
[20214900156] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[20229175956] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[20230214928] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[20235289008] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[20238250131] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[20259212754] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[20261314029] [INFO] [bloom] [CPU3] bloom: creating surface...
[20262591624] [INFO] [bloom] [CPU3] bloom: surface created!
[20263656798] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[20271727575] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[20277357243] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[20280139836] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[20283450594] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1264
[20284760925] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[20302512648] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[20307515514] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[20309591676] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[20311301967] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
[20319965358] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[20329667919] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
T:0270 [20334459552] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[20336646000] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[20346376215] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[20354983935] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[20358627630] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[20366044710] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[20368613793] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[20370807171] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[20372957187] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[20374372425] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[20389241169] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[20394555786] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[20397037650] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
T:07D0 T:0640 T:F0B0 [20408692392] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1264
[20431807473] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[20440995168] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[20443313880] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[20446011498] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[20448655029] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[20455608063] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[20458565193] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[20463410715] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[20465965212] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[20468265048] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[20470428264] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[20475523464] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[20476326618] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[20477807229] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
T:1220 [20481839796] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[20483442408] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[20493730224] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[20496213903] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[20498887860] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[20502912342] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=282
[20504317515] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[20505360183] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20506567323] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20507852079] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20509236066] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=282 subj_lo=0
[20522598492] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1273)
[20523931197] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[20536963623] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=245
[20538243066] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[20539582404] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20540699751] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20541893130] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20543324736] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=245 pred=0 subj_lo=0
[20554158075] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1276)
[20555642547] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[20566333326] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=286
[20567930757] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[20569175088] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20570226798] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20571213894] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20572424466] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=286 subj_lo=0
[20585087358] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1282)
[20586246384] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[20630575284] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[20850008949] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[20904934281] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[20931933396] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[21025589244] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[21035034207] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=664 watches=13 history=1024 journal=1024 symbols=313 drops=0
[21172092183] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[21318868065] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[21478267536] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[21727068759] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[21905251005] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[22021262868] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[22081354911] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[22267525203] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[22459804422] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[22465079175] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[22466581797] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[22467610374] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[22468591299] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[22469931231] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=226 subj_lo=0
[22474430715] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[22481654415] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[22482818886] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[22484126643] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[22485624678] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=338 pred=0 subj_lo=0
[22647321543] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=712 watches=15 history=1024 journal=1024 symbols=340 drops=0
[22655491452] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[22854800166] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[23024917179] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[23096349837] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[23098079928] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[23211166605] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23215108323] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23216467956] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23218110663] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23219481351] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=334 pred=0 subj_lo=0
[23292787155] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[23314312659] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23429050392] [INFO] [fontd] [CPU3
```
</details>
