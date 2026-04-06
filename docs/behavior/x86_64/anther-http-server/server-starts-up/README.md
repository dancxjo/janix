# ✅ Scenario: Server starts up

> Last run: 2026-04-05 17:15:01

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 3543ms | - [📜](./01/serial.log) - |
| 2 | Then I should see a message in the serial output that says "anther: Listening on port 80" | ✅ | 2432ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[10972575009] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[10978089606] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[10981532595] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[10983454911] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[10984966806] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[10985630733] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[10986291624] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[10986886383] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[10987461045] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[10988059467] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[10988637627] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[10989227634] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[10989930798] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[10990581690] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[10991252547] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[10991851365] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[10992461634] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[10993064478] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[10993668741] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[10994253699] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[10994819748] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[10995391143] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[10995969237] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[10996569408] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[10997194329] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[10997784468] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[10998367842] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[10998996591] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[10999570560] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11000174130] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11000767965] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11001365628] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11001975336] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11002576695] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11003172939] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11003853399] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11004540723] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11005233360] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11005912533] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11006632362] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11007368658] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11008116999] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11009327901] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11010652488] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11011402050] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11011936881] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11012485374] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11013060333] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11013690072] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11014206060] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11014711290] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11015221965] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11015725512] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11016270606] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x77983000 (Usable)
[11016806097] [INFO] [kernel::memory] [CPU0]   [11] 0x77983000 - 0x779e7000 (Reserved)
[11017358682] [INFO] [kernel::memory] [CPU0]   [12] 0x779e7000 - 0x779e8000 (Other)
[11017894833] [INFO] [kernel::memory] [CPU0]   [13] 0x779e8000 - 0x779e9000 (Reserved)
[11018448705] [INFO] [kernel::memory] [CPU0]   [14] 0x779e9000 - 0x779ea000 (Other)
[11018984823] [INFO] [kernel::memory] [CPU0]   [15] 0x779ea000 - 0x779eb000 (Reserved)
[11019552159] [INFO] [kernel::memory] [CPU0]   [16] 0x779eb000 - 0x779ec000 (Other)
[11020090323] [INFO] [kernel::memory] [CPU0]   [17] 0x779ec000 - 0x779ed000 (Reserved)
[11020644756] [INFO] [kernel::memory] [CPU0]   [18] 0x779ed000 - 0x77a78000 (Other)
[11021180610] [INFO] [kernel::memory] [CPU0]   [19] 0x77a78000 - 0x77a79000 (Reserved)
[11021729895] [INFO] [kernel::memory] [CPU0]   [20] 0x77a79000 - 0x77efa000 (Other)
[11022259974] [INFO] [kernel::memory] [CPU0]   [21] 0x77efa000 - 0x77efb000 (Reserved)
[11022821766] [INFO] [kernel::memory] [CPU0]   [22] 0x77efb000 - 0x7801c000 (Other)
[11023352769] [INFO] [kernel::memory] [CPU0]   [23] 0x7801c000 - 0x7801d000 (Reserved)
[11023900602] [INFO] [kernel::memory] [CPU0]   [24] 0x7801d000 - 0x7881d000 (Other)
[11024430813] [INFO] [kernel::memory] [CPU0]   [25] 0x7881d000 - 0x7881e000 (Reserved)
[11024979372] [INFO] [kernel::memory] [CPU0]   [26] 0x7881e000 - 0x788df000 (Other)
[11025508824] [INFO] [kernel::memory] [CPU0]   [27] 0x788df000 - 0x788e0000 (Reserved)
[11026068372] [INFO] [kernel::memory] [CPU0]   [28] 0x788e0000 - 0x788ef000 (Other)
[11026599210] [INFO] [kernel::memory] [CPU0]   [29] 0x788ef000 - 0x788f0000 (Reserved)
[11027147010] [INFO] [kernel::memory] [CPU0]   [30] 0x788f0000 - 0x788f5000 (Other)
[11027677320] [INFO] [kernel::memory] [CPU0]   [31] 0x788f5000 - 0x788f6000 (Reserved)
[11028224988] [INFO] [kernel::memory] [CPU0]   [32] 0x788f6000 - 0x788fb000 (Other)
[11028754506] [INFO] [kernel::memory] [CPU0]   [33] 0x788fb000 - 0x788fc000 (Reserved)
[11029313757] [INFO] [kernel::memory] [CPU0]   [34] 0x788fc000 - 0x78928000 (Other)
[11029844397] [INFO] [kernel::memory] [CPU0]   [35] 0x78928000 - 0x7892a000 (Reserved)
[11030427408] [INFO] [kernel::memory] [CPU0]   [36] 0x7892a000 - 0x78933000 (Other)
[11030962833] [INFO] [kernel::memory] [CPU0]   [37] 0x78933000 - 0x78935000 (Reserved)
[11031515814] [INFO] [kernel::memory] [CPU0]   [38] 0x78935000 - 0x7893d000 (Other)
[11032048071] [INFO] [kernel::memory] [CPU0]   [39] 0x7893d000 - 0x7893e000 (Reserved)
[11032610556] [INFO] [kernel::memory] [CPU0]   [40] 0x7893e000 - 0x78948000 (Other)
[11033145585] [INFO] [kernel::memory] [CPU0]   [41] 0x78948000 - 0x78949000 (Reserved)
[11033697477] [INFO] [kernel::memory] [CPU0]   [42] 0x78949000 - 0x78956000 (Other)
[11034229965] [INFO] [kernel::memory] [CPU0]   [43] 0x78956000 - 0x78958000 (Reserved)
[11034830499] [INFO] [kernel::memory] [CPU0]   [44] 0x78958000 - 0x78966000 (Other)
[11035369620] [INFO] [kernel::memory] [CPU0]   [45] 0x78966000 - 0x78967000 (Reserved)
[11036031204] [INFO] [kernel::memory] [CPU0]   [46] 0x78967000 - 0x78973000 (Other)
[11036565111] [INFO] [kernel::memory] [CPU0]   [47] 0x78973000 - 0x78974000 (Reserved)
[11037115353] [INFO] [kernel::memory] [CPU0]   [48] 0x78974000 - 0x78978000 (Other)
[11037648237] [INFO] [kernel::memory] [CPU0]   [49] 0x78978000 - 0x78979000 (Reserved)
[11038197951] [INFO] [kernel::memory] [CPU0]   [50] 0x78979000 - 0x78989000 (Other)
[11038729944] [INFO] [kernel::memory] [CPU0]   [51] 0x78989000 - 0x7898a000 (Reserved)
[11039295663] [INFO] [kernel::memory] [CPU0]   [52] 0x7898a000 - 0x78a1a000 (Other)
[11039871348] [INFO] [kernel::memory] [CPU0]   [53] 0x78a1a000 - 0x78a1b000 (Reserved)
[11040425649] [INFO] [kernel::memory] [CPU0]   [54] 0x78a1b000 - 0x78a26000 (Other)
[11040957114] [INFO] [kernel::memory] [CPU0]   [55] 0x78a26000 - 0x78a27000 (Reserved)
[11041510920] [INFO] [kernel::memory] [CPU0]   [56] 0x78a27000 - 0x78a4c000 (Other)
[11042045553] [INFO] [kernel::memory] [CPU0]   [57] 0x78a4c000 - 0x78a4d000 (Reserved)
[11042617014] [INFO] [kernel::memory] [CPU0]   [58] 0x78a4d000 - 0x78a59000 (Other)
[11043152109] [INFO] [kernel::memory] [CPU0]   [59] 0x78a59000 - 0x78a5a000 (Reserved)
[11043703275] [INFO] [kernel::memory] [CPU0]   [60] 0x78a5a000 - 0x78a67000 (Other)
[11044236456] [INFO] [kernel::memory] [CPU0]   [61] 0x78a67000 - 0x78a68000 (Reserved)
[11044788942] [INFO] [kernel::memory] [CPU0]   [62] 0x78a68000 - 0x78ab0000 (Other)
[11045323212] [INFO] [kernel::memory] [CPU0]   [63] 0x78ab0000 - 0x78ab1000 (Reserved)
[11046080298] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11261863041] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485787 free frames
[11272166598] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11276957769] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11278122570] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11278933875] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11282960370] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11284571265] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11285617431] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11286307956] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11286966537] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11287621422] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11288570205] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11289543738] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11290215090] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11290870140] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11291524332] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11292180240] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11293198323] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11294173968] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11294876010] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11296582044] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11297495418] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11298490698] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11299981803] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11301389748] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11302133205] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11302716546] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11303452842] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[11620281981] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[11621244162] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[11624329827] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[11625171030] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[11625939798] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[11627403711] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[11639545995] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[11640811380] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[11641549359] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[11643074157] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[11643614037] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[11645891961] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[11652787278] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[11654404707] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[11666637411] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[11667169107] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[11684491764] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[11685075006] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[11686933599] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[11688098070] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[11689098036] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[11691182349] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[11691986460] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[11726858055] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62395100 ticks/sec), init_cnt=623951 for 100Hz
[11728363746] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[11729213793] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[11730343383] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[11735704860] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[11765389779] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[11766249297] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[11768407992] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[11769637605] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[11770716540] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[11773348785] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[11774617833] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[11793540165] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[11795395986] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[11796187095] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[11797036053] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[11797688463] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[11798737533] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[11799355161] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[11824162680] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[11825951181] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[11826992760] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[11827976655] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[11828763045] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[11829537588] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[11830256064] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[11830912896] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[11837363208] [INFO] [kernel::root] [CPU0] Spawning Root service...
[11838200979] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[11839929750] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[11840736765] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[11846034123] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[11847738243] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[11848435005] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[11849680755] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[11850629835] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[11851453647] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[11862841023] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[11865848115] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[11866911870] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[11867745714] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[11886831792] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11904404061] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11906310339] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11909366403] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11910833814] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11913029205] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11916397746] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11919627786] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[11920356855] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[11921426451] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11927384271] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11929081626] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11932028592] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11934437790] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11936766996] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[11939021622] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[11939711916] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[11950757808] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[11951571093] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[11958711105] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[11959436412] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[11985330027] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[11986143015] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[12322506537] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[12783808851] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[12809596404] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[12844038966] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[14068390083] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=446 watches=0 history=959 journal=768 symbols=93 drops=0
[14814612252] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[14906263449] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[14907188769] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[15000709713] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[15058338438] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[15084472326] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[15085397844] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[15086015505] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[15089243994] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[15105694758] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[15121981149] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[15124197066] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[15172684098] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[15193345794] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[15194180067] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[15199801353] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[15230825379] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[15252775329] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[15260683350] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[15261632001] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[15326799774] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[15328394763] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[15414631155] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[15480309603] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[15492113241] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[15496001301] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[15498213885] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[15508568064] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[15572160516] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[15578291817] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[15579299505] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[15580084344] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[15580951089] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[15581663196] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[15582303099] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[15583145853] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[15583810638] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[15584420907] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[15585120375] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[15585736782] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[15586368963] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[15587017281] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[15587696553] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[15588433806] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[15589080969] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[15589730508] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[15590414037] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[15591069252] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[15591709683] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[15592313385] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[15592921542] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[15593535672] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[15594151848] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[15594922860] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[15595573158] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[15596283318] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[15597133761] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[15597890121] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[15598537383] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[15599179398] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[15599827419] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[15600457554] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[15601147056] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[15601768413] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[15602500617] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[15603230709] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[15603970932] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[15604730955] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[15605489064] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[15606509820] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[15607373133] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[15608966010] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[15610645974] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[15611585352] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[15619419750] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[15633172995] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[15637209786] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[15646122987] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[15646781205] [CONTRACT] [kernel] [CPU0] Spawning init process...
[15650228352] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[15654785784] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[15655478784] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [15661708095] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369013232 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564572728
[15664841511] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[15676400157] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[15681059955] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[15684858552] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[15686432289] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[15698850519] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[15703550511] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[15711806517] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[15712852848] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[15716642337] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[15719307285] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[15720366948] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[15724144920] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[15725166666] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[15726206100] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[15727238571] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[15730603482] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[15731641398] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[15732843819] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[15734291100] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[15735751449] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[15737078841] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[15740982147] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[15785962104] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[15793556823] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[15797814054] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[15801896847] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[15805652082] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[15809178858] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[15814913037] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[15820868811] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[15825208674] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[15829968726] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[15834511638] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[15839708214] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[15844612641] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[15849127668] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[15853432782] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[15857988432] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[15862688391] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[15867179097] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[15872355411] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[15877363194] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[15882388566] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[15887529075] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[15894753732] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[15899809728] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[15905191269] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[15909658611] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[15913960425] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[15918510168] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[15922697307] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[15927397332] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[15930591699] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[15933583809] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[15938349900] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[15942619242] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[15947056719] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[15951409254] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[15956144259] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[15960681198] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[15965518569] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[15970323666] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[15975463944] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[15978543603] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[15997425741] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[16122022983] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[16128358191] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[16129096368] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16130753232] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16135152891] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[16136449659] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16144103646] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[16148870529] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16149818091] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16151267979] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369078768 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564572760
[16154062980] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[16154784261] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16155901707] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[16156635858] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16160578797] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[16162173027] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16168648287] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[16171766160] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[16172934030] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[16174627623] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369144304 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564572696
[16177619073] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[16180379094] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[16181888184] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[16183865643] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[16187958798] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:15:16 = 1775434516 unix_secs
[16189564545] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775434516, mono_ns=8094570495, offset=1775434507905429505ns
[16191073008] [INFO] [rtc_cmos] [CPU1] System clock anchored
[16201562157] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[16237070553] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[16250012130] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[16250861451] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16252595271] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16258436304] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[16260690138] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[16268501535] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[16271930466] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[16274810376] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16276481496] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369210672 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564572728
[16281362361] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[16285801587] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[16289140065] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[16290726243] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[16291495572] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16293114618] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16300106196] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[16302333465] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[16308895482] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[16312229010] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[16314292368] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[16316167725] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16318300944] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369276912 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564572760
[16324717431] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[16327121514] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[16328591565] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[16331892192] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[16333022871] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[16334069103] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[16346630256] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[16348562208] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[16810135221] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[16815695688] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[16818381327] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[16819919094] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[16824004560] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[16825693797] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[16829306637] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[16832337984] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[16833279045] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[16835358342] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[16836306795] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[16837220598] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[16837985175] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[16838732262] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[16839461859] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[16840385529] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[16861635450] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[16862513052] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16864127247] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16873704936] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[16875930456] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[16882376676] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[16885131483] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[16887319680] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[16889547807] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369357040 RFLAGS_BEFORE=134 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564572696
[16897136949] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[16905885843] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[16913916723] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[16924043301] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[16925946279] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[16930825428] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[16939497432] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[16941010680] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[16945822575] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[16946744661] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[16947495411] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[16948465479] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[16949179368] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[16949987109] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[16950957639] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[16951642455] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[16952753961] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[17393611455] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[17397521691] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[17401571253] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[17403911778] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[17405143404] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17407948470] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17413328262] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[17416128708] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17426782065] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[17430047778] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[17431768266] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[17432503440] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17433932538] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17438537160] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17440096641] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17445263352] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[17446735779] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[17448935658] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
[17449854081] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010ac50
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[17451552492] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369500704 RFLAGS_BEFORE=130 CR3_BEFORE=68681728 fs_base=0 gs_base=18446744071564572760
[17454542490] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[17455246875] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17456734218] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17458089627] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[17459481600] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[17462390055] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17463444834] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[17465200500] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17465970324] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[17468574354] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[17470350447] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[17471678136] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[17472385755] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[17473902600] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[17474834784] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010ac50
[17476843065] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[17478303315] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[17479171281] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369583648 RFLAGS_BEFORE=130 CR3_BEFORE=68894720 fs_base=0 gs_base=18446744071564572696
[17483215893] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[17484069570] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[17485312317] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[17486167875] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[17489036466] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[17492674254] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[17497271286] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0108b60
[17501909469] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1240)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[17510268864] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369434912 RFLAGS_BEFORE=130 CR3_BEFORE=68395008 fs_base=0 gs_base=18446744071564572728
[17516205762] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[17519316210] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[17523196845] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1241
[17524993233] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[17526589278] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[17534239866] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17536383315] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[17537188218] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17538823731] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17545480755] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[17548535532] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[17557027554] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[17560302078] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[17562311151] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[17563637586] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[17564395728] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17565915081] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17586150120] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[17590146222] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[17609671959] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[17613061686] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
[17613962025] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cc30
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17615479596] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369720384 RFLAGS_BEFORE=130 CR3_BEFORE=69156864 fs_base=0 gs_base=18446744071564572760
[17620024422] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[17620880475] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[17621676105] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[17622451935] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[17623292742] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0108b60
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[17624827902] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369654848 RFLAGS_BEFORE=130 CR3_BEFORE=69021696 fs_base=0 gs_base=18446744071564572728
[17630408796] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[17631978969] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[17635502016] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[17636305797] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[17637303189] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[17638374666] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[17639327805] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[17640254940] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[17640998925] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17642512734] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[17643261603] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17707561707] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[17730195648] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[17736874749] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[17739653580] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0108b60
[17741398059] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17742797028] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369785920 RFLAGS_BEFORE=130 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564572696
[17746099701] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[17746824315] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17748490188] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[17749301427] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17750910606] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[17754602052] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[17756020194] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[17791393290] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[17796785226] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[17803953486] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[17807532930] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[17810249061] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[17811563880] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17814069471] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17819907699] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17822953071] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17830085394] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[17833520199] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010e250
[17835396546] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17837049285] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369916992 RFLAGS_BEFORE=130 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564572760
[17840243982] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[17841596058] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17844020271] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[17844834348] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17846166195] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[17853487113] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17857453086] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[17864437635] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[17867798784] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010e250
[17869625268] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17871301635] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369982528 RFLAGS_BEFORE=130 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564572696
[17874877482] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[17876115510] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17878551240] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17887622676] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17892294618] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[17899486110] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[17902838910] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[17905610514] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[17906308134] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17907755250] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17930960751] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[17935512771] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[17941831050] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[17944388781] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010f948
[17946112866] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17947477746] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370114048 RFLAGS_BEFORE=134 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564572760
[17950561332] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[17951561232] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17953051809] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[17953884300] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17974238601] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[17975063997] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[17978221338] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[17979370530] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[17980066830] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[17981420952] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[17982936543] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[17985892023] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[17988572184] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[17989811433] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17991321018] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370198096 RFLAGS_BEFORE=134 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564572696
[17994448659] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[17995228713] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17996861157] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18029831886] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[18038471781] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18039952458] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[18040978692] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18042329151] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18043451382] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=229 pred=0 subj_lo=0
[18044742210] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[18047070426] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[18051136917] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[18055278978] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[18058945872] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[18059927820] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18061017909] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[18061873731] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18062971146] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[18064023549] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18065301771] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[18066037968] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[18068091657] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0108b60
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18069978069] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369851456 RFLAGS_BEFORE=130 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564572728
[18073503855] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18074613414] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18075824415] [INFO] [nectar] [CPU2] NECTAR: Started.
[18076350699] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18077832234] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[18079848765] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010e250
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18081663105] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370048064 RFLAGS_BEFORE=130 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564572728
[18086539779] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[18087261060] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
[18088749393] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
[18089803545] [INFO] [fontd] [CPU3] FONTD: Service ready
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18091014315] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370266688 RFLAGS_BEFORE=130 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564572728
[18096665961] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[18098274414] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[18099843960] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[18106938003] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[18126941580] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[18128193039] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18129321243] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18133627545] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18134721561] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18135982227] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18137392944] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=234 subj_lo=0
[18144183255] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1249 backend=VirtIO-GPU
[18146003370] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[18146738082] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18148849455] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18154325706] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18155279934] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18156309435] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18157463973] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=239 subj_lo=0
[18164238906] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18165149145] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18166165182] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18167258868] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=240 subj_lo=0
[18174588795] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18175503918] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18176566848] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18177858204] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[18185105730] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18186017850] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18187082199] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18188181561] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[18195002529] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18195947550] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18197110470] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18198233691] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[18224760246] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[18240393072] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[18241410099] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[18260827596] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18261929334] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18275777322] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[18290523471] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[18297488748] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[18300257151] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[18301075749] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db568
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e1
[18302767494] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370358016 RFLAGS_BEFORE=134 CR3_BEFORE=72052736 fs_base=0 gs_base=18446744071564572760
[18306928002] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[18307781085] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18309173487] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[18309943080] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18314580966] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18316230471] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18323406189] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[18325753413] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db568
[18327635337] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[18329373315] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370423552 RFLAGS_BEFORE=134 CR3_BEFORE=73101312 fs_base=0 gs_base=18446744071564572696
[18332354667] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[18334100994] [INFO] [echo] [CPU1] echo: starting up
[18335965989] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[18336803793] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[18340313442] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[18354828822] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[18356552940] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[18372557709] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[18374426268] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[18376209324] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[18377612319] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f5000
[18379112202] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f5000
[18380192490] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f8000
[18382059234] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276787200)
[18382871463] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[18383599542] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[18385284357] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f9000 phys=0x4656000
[18386567760] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[18388527135] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[18390690846] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[18392081169] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[18397002228] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18397901445] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18401482209] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[18404390268] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[18405972618] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[18407562789] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[18410171538] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[18417180507] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[18418608846] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[18421449189] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[18423702198] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[18425251251] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[18426865083] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[18428343384] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[18429543495] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[18430482147] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[18431982690] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[18434400534] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[18437319351] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[18442160913] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[18446280039] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[18447838728] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[18449260830] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[18450047121] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18451750350] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18456663522] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18458515020] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18460914120] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[18463754760] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[18465115284] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[18466324569] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[18470001858] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[18472256682] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[18474295686] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[18476359110] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18477576843] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18479951556] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18482500410] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[18487928580] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18489232047] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[18492231120] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18499135380] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[18501513228] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[18502910283] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[18504317568] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[18505140093] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18506744322] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18509629644] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[18510818898] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18514679040] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[18516732894] [INFO] [bloom] [CPU3] bloom: creating surface...
[18517652835] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[18518557101] [INFO] [bloom] [CPU3] bloom: surface created!
[18519578748] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0113248
[18520553172] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[18521176575] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18522412359] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[18523400148] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370772896 RFLAGS_BEFORE=134 CR3_BEFORE=74162176 fs_base=0 gs_base=18446744071564572696
[18526549965] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010e250
[18527491257] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18529784559] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370707360 RFLAGS_BEFORE=134 CR3_BEFORE=73998336 fs_base=0 gs_base=18446744071564572760
[18535367928] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18537973212] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[18559643685] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1261
[18560711004] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[18565046148] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[18566825706] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[18570950574] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 3)
[18573792732] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[18576405012] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[18577547703] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[18578776689] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
[18583867335] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1261
[18588575874] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18591317085] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
T:0270 [18594905736] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[18602452374] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[18607590969] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18608619348] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18610891860] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[18618331710] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
T:07D0 T:0640 T:F0B0 [18652716786] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[18654523536] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[18659592237] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[18662085816] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[18666772641] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[18670414983] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=269
[18671634498] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[18672611463] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[18673529985] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18674824410] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18676191072] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=269 subj_lo=0
[18681169155] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[18689115720] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[18691409088] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[18695320578] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[18697747035] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
T:1220 [18710096988] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[18712312146] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[18723604119] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1274)
[18725721465] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[18739044126] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=243
[18740621823] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[18741471309] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[18742534536] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18743698809] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18745024221] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[18755368830] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1277)
[18756506472] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[18762807723] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[18769553088] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=279
[18771789036] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[18773515464] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[18774605949] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18775659870] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18777053856] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=279 subj_lo=0
[18787111002] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([249, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[18800021163] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1278)
[18801381786] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[18831790758] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[18833186196] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[18838456263] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[18850446747] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[18852161757] [INFO] [anther] [CPU1] anther: Connected to network stack
[18863680110] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[18864726870] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[18867506691] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[18892058559] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[18894593784] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[18901058682] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[18907263903] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[18912765432] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[18923206797] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[18924696285] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[18926806767] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[18948977619] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[18952743447] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[18954576333] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[18988641375] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[18991430667] [INFO] [netd] [CPU3] NETD: DHCP configured �� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[18994527783] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[18999148014] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[19001955984] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[19004823486] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[19007018217] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[19014794865] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[19028119440] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[19030367400] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[19033203684] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[19176614226] [INFO] [f
```
</details>
