# ❌ Scenario: Verifying System Services

> Last run: 2026-04-05 18:40:52

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 5462ms | - - - |
| 2 | And the anther server is ready | ❌ | 32700ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12036377001] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[12043451640] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[12047591886] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[12050046822] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[12051756519] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[12052800837] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[12053922210] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[12054913926] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[12055910526] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[12056939664] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[12057949695] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[12058991505] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[12060158022] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[12061258638] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[12062431590] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[12063463038] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[12064517619] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[12065547285] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[12066578535] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[12067596684] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[12068593350] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[12069594603] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[12070600245] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[12071613411] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[12072682743] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[12073701585] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[12074732505] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[12075822528] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[12076817445] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[12077867307] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[12078895059] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[12079931457] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[12081055932] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[12082109853] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[12083113350] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[12084297885] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[12085497798] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[12086685732] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[12087885348] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[12089118987] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[12090301608] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[12091517163] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[12093327939] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[12095222469] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[12096553986] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[12097501185] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[12098375718] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[12099243651] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[12100183986] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[12101084457] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[12101945691] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[12102817551] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[12103673670] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[12104603016] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[12105517446] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[12106454811] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[12107362179] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[12108327066] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[12109233015] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[12110174736] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[12111107943] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[12112044318] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[12112953798] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[12113888886] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[12114818595] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[12115759425] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[12116667057] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[12117641448] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[12118555647] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[12119498457] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[12120408333] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[12121499874] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[12122418264] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[12123369126] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[12124306722] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[12125248080] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[12126154194] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[12127094496] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[12128022027] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[12128961174] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[12129873657] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[12130828248] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[12131741358] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[12132682716] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[12133596882] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[12134568105] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[12135484020] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[12136424487] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[12137355648] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[12138292287] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[12139201140] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[12140145171] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[12141092469] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[12142034685] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[12142946970] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[12143853612] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[12144454839] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[12145042866] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[12145608222] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[12146193939] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[12146760417] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[12147379365] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[12147949077] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[12148537995] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[12149108961] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[12149697417] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[12150286137] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[12151159779] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12411782823] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[12422797695] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12427796601] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12429201807] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12430115379] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12434386767] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12436101084] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12437237505] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12437950272] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12438646935] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12439344918] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12440355807] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12441355278] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12442069728] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12442771011] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12443491071] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12444195357] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12445309800] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12446353590] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12447131994] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12448723551] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12449689428] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12450748464] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12452386980] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12453968472] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12454779183] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12455339457] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12456127860] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12847709721] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12848744766] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12852055854] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12853112844] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12854339619] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12856141155] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12869553477] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12870899547] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12871709631] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12873460248] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12874035834] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12877359990] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12885725424] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12887675955] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12902301885] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12902893938] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12919895307] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12920763207] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12923701494] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12925032054] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12926129139] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12928480752] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12929319942] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12964486293] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62455000 ticks/sec), init_cnt=624550 for 100Hz
[12966161967] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12967205031] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12968498565] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12974335275] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[13004900667] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[13005826581] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[13008166215] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[13009545945] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[13010686656] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[13013787666] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[13015170927] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[13034928720] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[13036497012] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[13038157671] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[13039020984] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[13040397447] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[13041165093] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[13042400679] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[13069117050] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[13070394975] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[13071544959] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[13072802490] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[13074025734] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[13075244919] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[13076346921] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[13077423777] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[13085104593] [INFO] [kernel::root] [CPU0] Spawning Root service...
[13086098223] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[13087942098] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[13088805345] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[13093342218] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[13095780885] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[13097250771] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[13099340364] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[13101032538] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[13102035243] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[13115455683] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[13119308895] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[13121188344] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[13122726474] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13146544884] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13166689800] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13169105136] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13172735565] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13174488954] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13176675072] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13179048135] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13180955172] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13181896596] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13183003086] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13189518705] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13191281268] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13194247869] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13196669739] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13199041581] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13202002143] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13202799984] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13214803272] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13215685560] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13222690338] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13223534049] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13251398886] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13252324767] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13631569875] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[14127764937] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[14163734772] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=501 journal=424 symbols=52 drops=0
[14195215320] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[15511312311] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=450 watches=0 history=966 journal=774 symbols=98 drops=0
[16492114386] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[16595914434] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[16597233312] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[16717380075] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[16787684133] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[16815004767] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[16816189500] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[16817151483] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[16821024429] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[16844652165] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[16863817179] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[16866439062] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[16928032044] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[16953578037] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[16954664958] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[16961472066] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[17001939537] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[17030349336] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[17034190338] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[17035766319] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[17109424035] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[17112061758] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[17219298789] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[17292015345] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[17304790173] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[17309238738] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[17311480428] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[17360895156] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=182 drops=0
[17381639715] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[17386972680] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[17388150483] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[17389265157] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[17390200674] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[17390993070] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[17391670296] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[17392346829] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[17392973565] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[17393624490] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[17394328941] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[17394978117] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[17395640163] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[17396320095] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[17397040056] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[17397805722] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[17398489218] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[17399177664] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[17399937654] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[17400631479] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[17401292139] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[17401932702] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[17402625801] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[17403283590] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[17403972135] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[17404676850] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[17405342955] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[17406001470] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[17406718725] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[17407398657] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[17408137791] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[17408818515] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[17409505014] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[17410173957] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[17410885536] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[17411540025] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[17412314568] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[17413304964] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[17414453760] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[17415442572] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[17416409769] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[17417233977] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[17418029706] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[17419918791] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[17421746595] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[17422569054] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17430903534] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[17446535964] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[17451377526] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[17461407480] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[17462144337] [CONTRACT] [kernel] [CPU0] Spawning init process...
[17464261584] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[17466868815] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[17467629630] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [17474022456] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[17476015227] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013120 RFLAGS_BEFORE=130 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[17511345027] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[17515751055] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[17518157250] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[17520315846] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[17533556271] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[17541179601] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[17546713173] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[17548805571] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[17556188628] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[17561487900] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[17563404771] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[17570357442] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[17572160232] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[17574082020] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[17575881675] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[17581770591] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[17583656739] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[17585762931] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[17588412303] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[17590930830] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[17593343394] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[17599650354] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[17644561869] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[17652413625] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[17658548490] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[17664717708] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[17668198680] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[17672473995] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[17678673474] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[17684477019] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[17690140974] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[17696274123] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[17703810399] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[17712585627] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[17721299970] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[17728707678] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[17735295006] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[17743188342] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[17751559452] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[17757639306] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[17763561882] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[17770348167] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[17777205435] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[17783673864] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[17791619142] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[17799610587] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[17808246522] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[17816935290] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[17824821102] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[17833003023] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[17840330937] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[17848676142] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[17854349766] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[17859840867] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[17867701467] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[17875321926] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[17882905161] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[17890851858] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[17899402884] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[17907941205] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[17916287961] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[17924757312] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[17933350380] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[17938239363] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[17963528253] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[18137746671] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[18147762402] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[18149096031] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18152133846] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18159390942] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18161630487] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18174731124] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[18183544863] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18184802361] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18186910434] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078656 RFLAGS_BEFORE=130 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[18192096450] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[18193310190] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18195288012] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[18196301640] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18202710834] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18205428747] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18216328845] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[18221657289] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[18223227099] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[18225441300] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144192 RFLAGS_BEFORE=130 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[18230106147] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[18234620712] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[18237439572] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[18240884244] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18248183943] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:44:32 = 1775439872 unix_secs
[18251115366] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775439872, mono_ns=9125196712, offset=1775439862874803288ns
[18253881327] [INFO] [rtc_cmos] [CPU1] System clock anchored
[18270102840] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[18323106351] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[18339428778] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[18340587837] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18343429797] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18352164105] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18355790475] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[18367425615] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[18373360599] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[18377518236] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18379949478] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210688 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[18387322338] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[18393701931] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[18396338829] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[18397506666] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18400388985] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18408596415] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18410952021] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18418381113] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[18421754505] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18423261219] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18425055462] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277440 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[18429591807] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[18432189105] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[18435591933] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[18446007393] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[18447112332] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[18448233441] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[18449332275] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[18450285579] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[18464108784] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[18465847059] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[19069165005] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19076381016] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[19079920695] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[19081674348] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[19086636954] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[19088447499] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[19090676484] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[19092567186] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[19094192502] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[19097158311] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[19098509133] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[19099591104] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[19100641692] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[19101612783] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[19102633704] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[19103960931] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[19109697816] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[19110996465] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19113693489] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19120894155] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19123472049] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19131362547] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[19134879621] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[19136261925] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[19138639212] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352752 RFLAGS_BEFORE=134 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[19148113512] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[19190331006] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[19195583484] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[19219076877] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[19222175577] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[19229350041] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[19230487287] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[19231795143] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[19233162630] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[19234455405] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[19235702838] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[19236894534] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[19238225589] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[19239518892] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[19243671744] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[19246034313] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[19249983291] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[19254688695] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[19259589657] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[19262865732] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[19264051818] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19267119531] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19273409892] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[19274681745] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[19276305741] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19295640540] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[19300213647] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[19303211367] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[19304430057] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19307001879] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19314041505] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19316056419] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19326448020] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[19330465935] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106688
[19332318093] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[19334982381] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500416 RFLAGS_BEFORE=134 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[19339760319] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[19341135693] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19343992866] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19345241256] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[19347114831] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[19352099679] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19353429183] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[19356112215] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[19357219596] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19360144188] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[19363142733] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[19365175269] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[19367271495] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[19370170083] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[19371816255] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[19374792954] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[19377404145] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[19378516542] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[19381827498] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[19384252734] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[19387183365] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[19389846663] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434176 RFLAGS_BEFORE=130 CR3_BEFORE=60096512 fs_base=0 gs_base=18446744071564586608
[19397750394] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[19833563442] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106688
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[19835857239] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583200 RFLAGS_BEFORE=134 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[19839388734] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19841324349] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[19843492482] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[19844670450] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[19845717144] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19848039948] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[19849053180] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19858983969] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[19861785801] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[19865969541] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[19869681381] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[19873251948] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[19876003455] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[19877686521] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[19878774663] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19880560161] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19901316171] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19905485622] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[20018713176] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[20757456423] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[20760629109] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1c0
[20762065467] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20763863241] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716528 RFLAGS_BEFORE=134 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[20769206997] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[20771624115] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[20772919893] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[20774035524] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[20775234975] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[20777803662] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[20779654962] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369649952 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[20786931924] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[20788671750] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[20795472258] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[20797073220] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[20798609568] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[20800997217] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[20805188052] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[20806409547] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[20819425407] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[20827060386] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[20828492157] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[20829990027] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[20831150802] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[20833217955] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[20834580063] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[20836575210] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4c32000
[20838145713] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[20840259792] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[20842508544] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[20844246918] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[20855329275] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[20864967717] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[20876252958] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[20879032152] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[20881323210] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[20883115704] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[20884796097] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[20886487116] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[20887909482] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[20888998482] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[20890102332] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[20895192087] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=15, read=16)
[20901313554] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=17, read=18)
[20912057232] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20933989428] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=15, RX port=18
[21141340374] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[21143694264] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[21146896221] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[21147985551] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21150396036] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21158039727] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21189265350] [INFO] [netd] [CPU3] NETD: Driver TX port=15, RX port=18, link_up=true, mtu=1500
[21193130871] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[21200220261] [INFO] [netd] [CPU3] NETD: Created socket API port (write=19, read=20)
[21215371485] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[21216727455] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[21218470680] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[21219906708] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[21232495086] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[21242733138] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([219, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[21259367514] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[21261292635] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[21263143902] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[21269281044] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[21271189797] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[21274487850] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f0fe8
[21275877975] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21277779897] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369952192 RFLAGS_BEFORE=134 CR3_BEFORE=80007168 fs_base=0 gs_base=18446744071564586576
[21282082767] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[21283190775] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21285686367] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[21286917993] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21289640031] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[21301401990] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=19, our_write=21, our_read=22)
[21304229265] [INFO] [anther] [CPU1] anther: Connected to network stack
[21313824906] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[21331042986] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[21336422580] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[21350003598] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[21354819057] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[21358179381] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[21359280195] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21361907127] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21370944936] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[21374853258] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21386906706] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[21391706160] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1c0
[21393256863] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21395085756] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370099648 RFLAGS_BEFORE=134 CR3_BEFORE=81473536 fs_base=0 gs_base=18446744071564586640
[21399645894] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[21400750107] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21403173825] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21415526484] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21421151532] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[21423544791] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[21425902740] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[21432928275] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[21437402184] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
[21438436998] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010a1c0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21440652123] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370165888 RFLAGS_BEFORE=130 CR3_BEFORE=81600512 fs_base=0 gs_base=18446744071564586576
[21444046173] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[21444971394] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21446880147] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21457195023] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[21460599072] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[21469207485] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[21473122671] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[21475891305] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[21476837415] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21479083065] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21518872683] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[21526701504] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[21538758087] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[21543461445] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010bc58
[21545015877] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21547113654] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370298240 RFLAGS_BEFORE=130 CR3_BEFORE=81903616 fs_base=0 gs_base=18446744071564586640
[21551901360] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[21552935514] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21554813511] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[21556348836] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21581181501] [INFO] [fontd] [CPU3] FONTD: Service node created, req=23, resp=26
[21582814242] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[21586172916] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[21588866409] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21590558253] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21592207659] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21594270753] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[21596038365] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[21599505477] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f25e8
[21600939756] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21602690835] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370382256 RFLAGS_BEFORE=130 CR3_BEFORE=82178048 fs_base=0 gs_base=18446744071564586576
[21607106037] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[21608015451] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21610224669] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21611266941] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[21613372935] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[21630236100] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21632046414] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21633885471] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21635829138] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=237 pred=0 subj_lo=0
[21644708514] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21646341948] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21647909283] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21649747845] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[21650772528] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=236 subj_lo=0
[21655873008] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21657555480] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21659472681] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21661501521] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[21662458323] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=238 pred=0 subj_lo=0
[21671931336] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[21674716701] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[21678998319] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[21682213872] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f0fe8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21684023658] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370034112 RFLAGS_BEFORE=134 CR3_BEFORE=80740352 fs_base=0 gs_base=18446744071564586608
[21688763580] [INFO] [nectar] [CPU2] NECTAR: Started.
[21690942768] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1190
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21692672166] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370232256 RFLAGS_BEFORE=134 CR3_BEFORE=81747968 fs_base=0 gs_base=18446744071564586608
[21698621373] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[21700697601] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f25e8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21702497520] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370449632 RFLAGS_BEFORE=130 CR3_BEFORE=82386944 fs_base=0 gs_base=18446744071564586608
[21708442008] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[21710515398] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[21712213512] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[21714866118] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1250) for kind 'Asset'
[21717072729] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[21718119588] [INFO] [fontd] [CPU3] FONTD: Service ready
[21719919738] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21721518720] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21723274452] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21725253132] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=240 subj_lo=0
[21745043430] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21746717487] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21748584429] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21750524004] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[21756629796] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[21760555542] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[21764918934] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21766576194] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21768426603] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21770200683] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[21806988423] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21808637466] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21810262518] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21812001651] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=248 subj_lo=0
[21822748530] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21824372592] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21825992727] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21827701665] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=250 subj_lo=0
[21847073919] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21848701215] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21850306665] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21852094572] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=251 pred=0 subj_lo=0
[21861311835] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[21862495215] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[21872314695] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[21875792004] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[21877932153] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[21879513612] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[21884302770] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[21899055915] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1259 backend=VirtIO-GPU
[21901132605] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[21902262789] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21903941730] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21936843027] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[21960641241] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=29, resp=32
[21962774097] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[22004225727] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[22006383927] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[22038968457] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[22055063646] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[22062912333] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[22065586686] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f2a58
[22066970310] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4eb
[22068894639] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370551664 RFLAGS_BEFORE=130 CR3_BEFORE=83570688 fs_base=0 gs_base=18446744071564586640
[22073214405] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[22074102633] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22075845825] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22076798601] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[22080492654] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[22082187732] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22090300320] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[22093097235] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f2a58
[22094591607] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[22096331895] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370617200 RFLAGS_BEFORE=130 CR3_BEFORE=84619264 fs_base=0 gs_base=18446744071564586576
[22100625294] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[22102999710] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[22103992977] [INFO] [echo] [CPU1] echo: starting up
[22104894174] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[22107263343] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[22108580043] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[22133762442] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[22139199291] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[22141989144] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[22151524329] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[22178968614] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[22182418500] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[22187029227] [INFO] [netd] [CPU3] NETD: DHCP configured ��� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[22191758424] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[22198369578] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[22203938526] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[22207419960] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[22211640891] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[22233111318] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[22234470390] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[22239374982] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[22240614627] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22242751509] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[22244267067] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22245810180] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[22250050251] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22252517958] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[22260578241] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[22263730236] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[22265686443] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[22267508901] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[22269579816] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[22270786065] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22273257963] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22280629635] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22284479877] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[22294834683] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[22298142075] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db678
[22299747723] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22301623806] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370752624 RFLAGS_BEFORE=130 CR3_BEFORE=85123072 fs_base=0 gs_base=18446744071564586640
[22305879090] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[22308239943] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[22309058244] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22310785530] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22315363257] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[22317181722] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[22327536066] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[22331786565] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[22333739439] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[22336158372] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[22338288522] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[22344750945] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db678
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22347149121] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[22348353390] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370818864 RFLAGS_BEFORE=134 CR3_BEFORE=85270528 fs_base=0 gs_base=18446744071564586576
[22355215905] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[22356686979] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[22386630387] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[22390873593] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[22393257546] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[22395800988] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[22403653272] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[22407155133] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=3
[22419657876] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=3)
[22446490374] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[22457418687] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[22498538007] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[22503281295] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[22522997244] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[22571350923] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[22594136697] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[22595847087] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[22598683932] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22602269712] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22610351676] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[22614855582] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[22620038628] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=74
[22621383774] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[22622343612] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=17
[22624349682] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[22636306539] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[22643623827] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[22647502383] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[22653335133] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[22655134953] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[22657917579] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22661390367] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[22666113591] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=149
[22668034488] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[22670679075] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[22690885932] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11023000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[22693827057] [INFO] [bloom] [CPU3] bloom: creating surface...
[22695199890] [INFO] [bloom] [CPU3] bloom: surface created!
[22696461513] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[22700799330] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[22712810472] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22715010714] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[22731411813] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1268
[22733417751] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[22755424659] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1268
[22760245365] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[22765022544] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[22772845359] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[22776996990] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[22778619072] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[22780526109] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [22792393371] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[22806721971] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[22819003152] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[22825912329] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 6)
[22831060692] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
T:07D0 T:0640 T:F0B0 [22882602501] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[22896165600] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[22909626663] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[22914830202] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[22918436079] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[22936737516] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
T:1220 [22949575605] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[22952673711] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[22969733424] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[22972555254] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[22995889191] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=284
[22997914434] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[22999549254] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[23001217866] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23003095533] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23005202649] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=284 subj_lo=0
[23037015672] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1280)
[23038649931] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[23045083215] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=251
[23047203960] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[23048504490] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[23049969921] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23051531547] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23053358823] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=251 pred=0 subj_lo=0
[23078064009] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1281)
[23079674277] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[23124925329] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=286
[23127145503] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[23128262652] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[23129400261] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23130506883] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23131799295] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=286 subj_lo=0
[23149013415] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1284)
[23151307674] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[23321600874] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[23437651941] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=643 watches=13 history=1024 journal=1024 symbols=303 drops=0
[23579495808] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[23651784849] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[23682338592] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[23780986944] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[23987164608] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[24355314324] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[24657692961] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[24943390560] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[25166526858] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[25221945507] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[25385943990] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[25636666011] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[25658200458] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=715 watches=13 history=1024 journal=1024 symbols=337 drops=0
[25722418821] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[25728182304] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[25729819863] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[25730999712] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[25732568037] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[25734540315] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=236 subj_lo=0
[25754995068] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[25756641966] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[25758353181] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[25760361165] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=337 pred=0 subj_lo=0
[25889571147] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[25891892400] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[25894534611] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[25978959501] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[26180490600] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[26381259828] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[26581312758] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=0e57f834b3afbcb1)
[26749541808] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[26752100034] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[26765969241] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26768656200] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[26772075594] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[26804400018] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[26871918414] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26957002083] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27018468246] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[27020216553] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[27021994197] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[27023929911] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=349 pred=0 subj_lo=0
[27063969537] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27122404881] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[27129698244] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=c1e9a7b90a789e5c)
[27223987098] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[27247378455] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[27250194147] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[27280805376] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27348372117] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[27406387305] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[27408049977] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[27409930449] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[27411621864] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=351 pred=0 subj_lo=0
[27428784966] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[27441466404] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27466794762] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[27468003321] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[27469362426] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[27470810400] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=352 pred=0 subj_lo=0
[27504010644] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[27553679769] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27586850709] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[27588094479] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[27589525227] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[27590828826] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=353 pred=0 subj_lo=0
[27595652106] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[27623000064] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x122d0000
[27624450909] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[27626060385] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[27689036892] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[27698742951] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[27703533891] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[27706263519] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[27708837915] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[27713435772] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[27735643452] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[27737593983] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[27739701858] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[27760135458] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x122d1000
[27761715597] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[27783653700] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27887635215] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[27958906602] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=784 watches=19 history=1024 journal=1024 symbols=364 drops=0
[28267386378] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[28637786925] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[29028364398] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[29212891884] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[29214833472] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[29217556236] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[29375994813] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[29519437134] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[29523743238] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[29532207375] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[29548505613] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[29551304013] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[29555691528] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[29567460879] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=149
[29570043228] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[29573249838] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[29853504021] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[30105625374] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30109174293] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[30128692704] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[30132151896] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[30160813947] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=830 watches=19 history=1024 journal=1024 symbols=364 drops=0
[30186809961] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[30229499553] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30371300124] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[30404089617] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[30472680018] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30624263934] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30977767161] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[31363587717] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[31834091091] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[32362489434] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[32509764705] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[32521793073] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[32525776833] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[32613415659] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=887 watches=19 history=1024 journal=1024 symbols=366 drops=0
[33028049142] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[33580584411] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[33583750926] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[33675117531] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[34186918920] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2445 ops=1 watches=19
[34235560986] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[34354882551] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=904 watches=19 history=1024 journal=1024 symbols=367 drops=0
[34848427779] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[35704829622] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[35832694722] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[35835236547] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[35837818698] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[36343716585] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[36348084102] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[36391806528] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[36409463508] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[36412343319] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[36415965135] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[36428474412] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=149
[36431105007] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[36433410486] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[36485517552] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[36621176064] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[36626015250] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[36627779298] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[36644967480] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[36649125678] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[36849265101] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=939 watches=19 history=1024 journal=1024 symbols=367 drops=0
[36893617794] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[37110376347] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[37231265280] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[37602665232] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[38893381296] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=985 watches=19 history=1024 journal=1024 symbols=397 drops=0
[39134439036] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[39137952315] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[39141573438] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[39889136430] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[39893507775] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[39897020328] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[41661821916] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1072 watches=19 history=1024 journal=1024 symbols=448 drops=0
[41780432397] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[41894129244] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[42166520814] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[42168546486] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[42204925983] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[42460276287] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[42462474582] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[42464798475] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[42720090633] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[42766666107] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[42767795565] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[42769058574] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[42770490114] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[42779388333] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[42780527262] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[42781682460] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[42783056217] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=241 pred=0 subj_lo=0
[42789884775] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[42791140326] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[42792333408] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[42793658622] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=453 pred=0 subj_lo=0
[42810314085] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[42811496112] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[42812714010] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[42814101528] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=452 pred=0 subj_lo=0
[42822852204] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[42824008590] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[42825429042] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[42827208600] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[42841119222] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[43734327912] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1102 watches=24 history=1024 journal=1024 symbols=454 drops=0
[45149032995] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[45154464795] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[45157714701] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[45171288294] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[45191207391] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[45195289161] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[45208983963] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=149
[45212034780] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[45215388174] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[45235801380] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[45238847643] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[45542151039] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[45546628809] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[45679179216] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=1128 watches=24 history=1024 journal=1024 symbols=454 drops=0
[45755546463] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=70
[45759244014] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[45763076799] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[47156189157] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[47159317524] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[47161774242] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[47535806538] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=1150 watches=24 history=1024 journal=1024 symbols=454 drops=0
[47940302718] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[47944171044] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=2 (16384b)
[47949761013] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[47954460378] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[49085064798] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=70
[49088777892] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[49092805542] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[49185623223] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[49189451355] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[49196247210] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[49205904924] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=70
[49210158690] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[49215952500] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[49228197348] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=149
[49230815832] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[49234311093] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[49489788150] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[49492230810] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[49885518441] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=16000 nodes=1189 watches=24 history=1024 journal=1024 symbols=454 drops=0
[50243002491] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[50253968820] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[51539812236] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=3469 ops=1 watches=24
[51943043922] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=17000 nodes=1219 watches=24 history=1024 journal=1024 symbols=454 drops=0
[52378948248] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=70
[52381485816] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[52383993321] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[53336719722] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[53339844096] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[53341737900] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[53541863826] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=18000 nodes=1243 watches=24 history=1024 journal=1024 symbols=454 drops=0
[55711333887] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=70
[55714996128] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[55718929365] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[55739136189] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[55741787937] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[55744962405] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[55761444024] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=70
[55766742537] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[55771687719] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[55780951974] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=25 len=149
[55783827792] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[55787518380] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[55942767243] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=19000 nodes=1286 watches=24 history=1024 journal=1024 symbols=454 drops=0
[56706090276] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[56711104989] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[58253669562] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=20000 nodes=1314 watches=24 history=1024 journal=1024 symbols=454 drops=0
[59005468599] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=26 len=70
[59007920400] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[59010870864] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[59331439332] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[59333615484] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[59339580267] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[59362507710] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=27 len=149
[59364933045] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[59368202718] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[59382244911] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[59387934573] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[59392611894] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[60669707241] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=21000 nodes=1334 watches=24 history=1024 journal=1024 symbols=454 drops=0
[62327709345] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=28 len=70
[62330751978] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[62333874900] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[63643568406] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=22000 nodes=1377 watches=24 history=1024 journal=1024 symbols=454 drops=0
[63662818692] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[63665596896] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[63668937882] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[63679952292] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=29 len=70
[63683312946] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[63686328354] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[63694766322] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=30 len=149
[63697744539] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[63699986163] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[63701700546] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[63703163535] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[63725485989] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[63732557691] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[65627740398] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=31 len=70
[65632386864] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[65639615316] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[66492786228] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=23000 nodes=1411 watches=24 history=1024 journal=1024 symbols=454 drops=0
[66739469874] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[66742776705] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[66746999286] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[68601476064] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=24000 nodes=1439 watches=24 history=1024 journal=1024 symbols=454 drops=0
[68957166168] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=70
[68960421750] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[68964059010] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[70763884752] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=25000 nodes=1472 watches=24 history=1024 journal=1024 symbols=454 drops=0
[71455575609] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[71463484059] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[71479101045] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[71502419142] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=70
[71505396732] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[71510011419] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[71523850827] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=149
[71528123040] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[71532799470] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[71550008739] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[71554198914] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[71572534143] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[71575558164] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[72266008815] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=70
[72271606242] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[72276831396] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[72299277369] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[72302018580] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[72309716589] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[72580266330] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=2
[74259231453] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=26000 nodes=1520 watches=24 history=1024 journal=1024 symbols=454 drops=0
[75577279899] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[75579739125] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[75595603149] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[75714727704] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[75719774823] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[75723593253] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[75754254213] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=70
[75756063471] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[75759342978] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[75780099648] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=149
[75782806506] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[75786288897] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[76335472026] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[76339168158] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[76354926549] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[76358345316] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[76682159136] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=27000 nodes=1546 watches=24 history=1024 journal=1024 symbols=454 drops=0
[78865158075] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[78869135763] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[78874584822] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[79459866288] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=28000 nodes=1581 watches=24 history=1024 journal=1024 symbols=454 drops=0
[81014132199] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[81018487638] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[81022400481] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[81956339355] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=29000 nodes=1611 watches=24 history=1024 journal=1024 symbols=454 drops=0
[82203893937] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[82207260894] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[82213337316] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[82223877582] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[82229542098] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[82234801407] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[82250486868] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[82256137623] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[82262115078] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[82269775929] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[82272557400] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=149
[82276461762] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[82281724173] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[82290439407] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[82312332531] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[82315540890] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[83540488812] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=4493 ops=1 watches=24
[84746100153] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=30000 nodes=1643 watches=24 history=1024 journal=1024 symbols=454 drops=0
[85490248536] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[85501691715] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[85505609838] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[86282518047] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[86286009381] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[86290432173] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[87726423312] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=31000 nodes=1682 watches=24 history=1024 journal=1024 symbols=454 drops=0
[88822691265] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[88827812634] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[88832484147] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[89513697768] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[89517935727] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[89528200575] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[89545395159] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[89549800626] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[89555382147] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[89574951279] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[89578975200] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=149
[89583729477] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[89588170320] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[90010344369] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[90052777353] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[90064293000] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[90654048375] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=32000 nodes=1718 watches=24 history=1024 journal=1024 symbols=454 drops=0
[92112302700] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[92116922733] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[92121090204] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[92382478056] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[92386267578] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[92388959256] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[93432533766] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=33000 nodes=1744 watches=24 history=1024 journal=1024 symbols=454 drops=0
[95437201002] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[95441015142] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[95444122587] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[96871149672] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[96885316176] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[96905232798] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[96944337369] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=70
[96948133491] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[96952780122] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[96978012681] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=149
[96982048647] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[96988714746] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[97101093177] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=34000 nodes=1779 watches=24 history=1024 journal=1024 symbols=454 drops=0
[97828081626] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[97831968036] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[97856003256] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[97868639121] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[98730072177] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=70
[98735204766] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[98739725172] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[98742902643] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[98747477499] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[98754191448] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[98757934704] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[102072538590] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=70
[102076512582] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[102080923197] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[102151798749] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=35000 nodes=1827 watches=24 history=1024 journal=1024 symbols=454 drops=0
[102615318399] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[102617745681] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[102627387588] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[102665318514] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=70
[102670246635] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[102678768258] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[102708846834] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=149
[102719742543] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[102724134744] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[103173043809] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[103186908396] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[103253521932] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[103280015058] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[105367406529] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=70
[105372906012] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[105378206967] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[105457298430] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[105463151640] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[105473596536] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[106409261310] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=36000 nodes=1865 watches=24 history=1024 journal=1024 symbols=454 drops=0
[108700759332] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=70
[108707852385] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[108721304241] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[109712008098] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=37000 nodes=1896 watches=24 history=1024 journal=1024 symbols=454 drops=0
[109894519416] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[109910324634] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[109915453362] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[109926144438] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=25 len=70
[109940620812] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[109949612850] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[109972315332] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=26 len=149
[109978474419] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[109985285916] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[109996589241] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[110158791006] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[110916919542] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[110921275773] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[111999016206] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=27 len=70
[112002727848] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[112005403554] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[112021684896] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[112026296481] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[112028899191] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[113028358905] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=38000 nodes=1926 watches=24 history=1024 journal=1024 symbols=454 drops=0
[115324375188] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=28 len=70
[115328223516] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[115331018880] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[115673908185] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[115681928307] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[115702586010] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[115739498952] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=29 len=70
[115744441692] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[115749267150] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[115771434471] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=30 len=149
[115775906400] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[115782152244] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[115875176934] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[115882157061] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[115891248132] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[115916868309] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[116411005062] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=39000 nodes=1962 watches=24 history=1024 journal=1024 symbols=454 drops=0
[118625989053] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=31 len=70
[118631857509] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[118636683561] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[119311416147] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=40000 nodes=1986 watches=24 history=1024 journal=1024 symbols=454 drops=0
[119815069869] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[119820232026] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[119822898162] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[123027532221] [INFO] [kern
```
</details>
