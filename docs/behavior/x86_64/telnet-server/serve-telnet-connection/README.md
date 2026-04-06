# ✅ Scenario: Serve telnet connection

> Last run: 2026-04-05 21:54:16

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the telnet server is ready | ✅ | 8703ms | - - - |
| 2 | When I connect to the telnet server and send "match (n) return n;" | ✅ | 15528ms | - [📜](./02/serial.log) - |
| 3 | Then the telnet response should contain "node(" | ✅ | 0ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12092035263] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[12097701495] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[12101400003] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[12103453593] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[12104654562] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[12105285687] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[12105997068] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[12106618293] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[12107232951] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[12107866518] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=33264
[12108486060] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=973464
[12109155201] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[12109895589] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[12110596839] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[12111315183] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[12111949641] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[12112635777] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[12113256177] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[12113900370] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[12114523773] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[12115127277] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[12115779885] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[12116404542] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[12117049659] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[12117712431] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[12118334943] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[12118978971] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[12119648475] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[12120257952] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[12120883071] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[12121668933] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[12122345730] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[12123017511] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[12123650121] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=168464
[12124264383] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[12124979031] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[12125734038] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[12126501981] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[12127216959] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[12127955565] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[12128676516] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[12129442941] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[12130769706] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[12132348855] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[12133122771] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[12133673310] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[12134186658] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[12134710599] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[12135258597] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[12135827616] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[12136344363] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[12136867149] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[12137385348] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[12137928858] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7795e000 (Usable)
[12138478869] [INFO] [kernel::memory] [CPU0]   [11] 0x7795e000 - 0x779c2000 (Reserved)
[12139082802] [INFO] [kernel::memory] [CPU0]   [12] 0x779c2000 - 0x779c3000 (Other)
[12139632153] [INFO] [kernel::memory] [CPU0]   [13] 0x779c3000 - 0x779c4000 (Reserved)
[12140200182] [INFO] [kernel::memory] [CPU0]   [14] 0x779c4000 - 0x779c5000 (Other)
[12140751480] [INFO] [kernel::memory] [CPU0]   [15] 0x779c5000 - 0x779c6000 (Reserved)
[12141341421] [INFO] [kernel::memory] [CPU0]   [16] 0x779c6000 - 0x779c7000 (Other)
[12141964098] [INFO] [kernel::memory] [CPU0]   [17] 0x779c7000 - 0x779c8000 (Reserved)
[12142590768] [INFO] [kernel::memory] [CPU0]   [18] 0x779c8000 - 0x77a53000 (Other)
[12143184306] [INFO] [kernel::memory] [CPU0]   [19] 0x77a53000 - 0x77a54000 (Reserved)
[12143797710] [INFO] [kernel::memory] [CPU0]   [20] 0x77a54000 - 0x77ed5000 (Other)
[12144411840] [INFO] [kernel::memory] [CPU0]   [21] 0x77ed5000 - 0x77ed6000 (Reserved)
[12145028016] [INFO] [kernel::memory] [CPU0]   [22] 0x77ed6000 - 0x77ff7000 (Other)
[12145623402] [INFO] [kernel::memory] [CPU0]   [23] 0x77ff7000 - 0x77ff8000 (Reserved)
[12146203872] [INFO] [kernel::memory] [CPU0]   [24] 0x77ff8000 - 0x787f8000 (Other)
[12146764410] [INFO] [kernel::memory] [CPU0]   [25] 0x787f8000 - 0x787f9000 (Reserved)
[12147343296] [INFO] [kernel::memory] [CPU0]   [26] 0x787f9000 - 0x788ba000 (Other)
[12147903207] [INFO] [kernel::memory] [CPU0]   [27] 0x788ba000 - 0x788bb000 (Reserved)
[12148482060] [INFO] [kernel::memory] [CPU0]   [28] 0x788bb000 - 0x788e5000 (Other)
[12149082165] [INFO] [kernel::memory] [CPU0]   [29] 0x788e5000 - 0x788e6000 (Reserved)
[12149683590] [INFO] [kernel::memory] [CPU0]   [30] 0x788e6000 - 0x788eb000 (Other)
[12150265776] [INFO] [kernel::memory] [CPU0]   [31] 0x788eb000 - 0x788ec000 (Reserved)
[12150866211] [INFO] [kernel::memory] [CPU0]   [32] 0x788ec000 - 0x788f1000 (Other)
[12151447341] [INFO] [kernel::memory] [CPU0]   [33] 0x788f1000 - 0x788f2000 (Reserved)
[12152063187] [INFO] [kernel::memory] [CPU0]   [34] 0x788f2000 - 0x7891e000 (Other)
[12152644977] [INFO] [kernel::memory] [CPU0]   [35] 0x7891e000 - 0x78920000 (Reserved)
[12153237195] [INFO] [kernel::memory] [CPU0]   [36] 0x78920000 - 0x78929000 (Other)
[12153797634] [INFO] [kernel::memory] [CPU0]   [37] 0x78929000 - 0x7892b000 (Reserved)
[12154377015] [INFO] [kernel::memory] [CPU0]   [38] 0x7892b000 - 0x78933000 (Other)
[12154937355] [INFO] [kernel::memory] [CPU0]   [39] 0x78933000 - 0x78934000 (Reserved)
[12155548746] [INFO] [kernel::memory] [CPU0]   [40] 0x78934000 - 0x7893e000 (Other)
[12156110703] [INFO] [kernel::memory] [CPU0]   [41] 0x7893e000 - 0x7893f000 (Reserved)
[12156689490] [INFO] [kernel::memory] [CPU0]   [42] 0x7893f000 - 0x7894c000 (Other)
[12157246827] [INFO] [kernel::memory] [CPU0]   [43] 0x7894c000 - 0x7894e000 (Reserved)
[12157818717] [INFO] [kernel::memory] [CPU0]   [44] 0x7894e000 - 0x7895c000 (Other)
[12158373282] [INFO] [kernel::memory] [CPU0]   [45] 0x7895c000 - 0x7895d000 (Reserved)
[12158960781] [INFO] [kernel::memory] [CPU0]   [46] 0x7895d000 - 0x78969000 (Other)
[12159514719] [INFO] [kernel::memory] [CPU0]   [47] 0x78969000 - 0x7896a000 (Reserved)
[12160085388] [INFO] [kernel::memory] [CPU0]   [48] 0x7896a000 - 0x7896e000 (Other)
[12160638072] [INFO] [kernel::memory] [CPU0]   [49] 0x7896e000 - 0x7896f000 (Reserved)
[12161223789] [INFO] [kernel::memory] [CPU0]   [50] 0x7896f000 - 0x7897f000 (Other)
[12161825049] [INFO] [kernel::memory] [CPU0]   [51] 0x7897f000 - 0x78980000 (Reserved)
[12162419511] [INFO] [kernel::memory] [CPU0]   [52] 0x78980000 - 0x78a10000 (Other)
[12162993282] [INFO] [kernel::memory] [CPU0]   [53] 0x78a10000 - 0x78a11000 (Reserved)
[12163586853] [INFO] [kernel::memory] [CPU0]   [54] 0x78a11000 - 0x78a1c000 (Other)
[12164159733] [INFO] [kernel::memory] [CPU0]   [55] 0x78a1c000 - 0x78a1d000 (Reserved)
[12164753634] [INFO] [kernel::memory] [CPU0]   [56] 0x78a1d000 - 0x78a42000 (Other)
[12165327372] [INFO] [kernel::memory] [CPU0]   [57] 0x78a42000 - 0x78a43000 (Reserved)
[12165904377] [INFO] [kernel::memory] [CPU0]   [58] 0x78a43000 - 0x78a4f000 (Other)
[12166461549] [INFO] [kernel::memory] [CPU0]   [59] 0x78a4f000 - 0x78a50000 (Reserved)
[12167036310] [INFO] [kernel::memory] [CPU0]   [60] 0x78a50000 - 0x78a5d000 (Other)
[12167593317] [INFO] [kernel::memory] [CPU0]   [61] 0x78a5d000 - 0x78a5e000 (Reserved)
[12168170685] [INFO] [kernel::memory] [CPU0]   [62] 0x78a5e000 - 0x78aa6000 (Other)
[12168759405] [INFO] [kernel::memory] [CPU0]   [63] 0x78aa6000 - 0x78aa7000 (Reserved)
[12169640274] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12423863100] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485750 free frames
[12434744421] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12440145762] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12441447513] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12442374087] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12446995077] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12448841658] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12449951778] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12450632964] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12451300092] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12451984974] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12452954382] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12453926628] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12454636755] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12455381664] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12456083079] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12456778653] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12457939923] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12459058095] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12459770763] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12461352816] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12462301929] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12463279356] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12465564771] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12467618493] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12468817020] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12469647498] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12470845332] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12861556290] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12862955556] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12866476557] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12867835200] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12869051415] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12870915420] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12884917617] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12886447959] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12887293815] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12889131585] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12889736673] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12892239327] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12899976870] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12901673136] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12916212309] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12916852080] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12933480681] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12934106955] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12936308022] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12937580139] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12938776059] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12941518623] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12942472191] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12977621877] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62470700 ticks/sec), init_cnt=624707 for 100Hz
[12979235808] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12980050281] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12981190794] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12987348429] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[13018370178] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[13019382783] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[13020511350] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[13021990509] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[13023235005] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[13026699543] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[13028028123] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[13050116838] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[13050995727] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[13051766013] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[13052717601] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[13053914379] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[13055265102] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[13055954406] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[13081402356] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[13083093474] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[13084233162] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[13085583489] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[13086557583] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[13087844946] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[13088840556] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[13090062711] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[13098031716] [INFO] [kernel::root] [CPU0] Spawning Root service...
[13099092600] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[13101109461] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[13101966306] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[13108766946] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[13110575445] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[13111350714] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[13112636988] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[13113577059] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[13114605075] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[13127624301] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[13131267699] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[13132508532] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[13133297298] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13156536294] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13181970747] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13184733342] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13189266288] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13191094554] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13193537511] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13195832793] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13197933705] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13198847739] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13200025740] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13206966333] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13208890629] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13211980815] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13214843070] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13219953021] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13220870355] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13232262417] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13233578457] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13241394012] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13242437505] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13272385599] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13273266039] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13643554782] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[14184865332] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[14217878367] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=499 journal=424 symbols=51 drops=0
[14253153288] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[15618529014] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=963 journal=771 symbols=97 drops=0
[16395997398] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[16509685863] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[16511176308] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[16613516502] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[16684174419] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[16715279790] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[16716367668] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[16717046181] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[16721713206] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[16743235740] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[16765183875] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[16767547467] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[16832133417] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[16858697328] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[16859716599] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[16866676002] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[16905786645] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[16933761438] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[16938034872] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[16939555347] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[17009933160] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[17011762086] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[17105980980] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[17177923125] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[17191436889] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[17197286898] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[17199637950] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[17225221101] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[17273163600] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[17278635099] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[17279672421] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[17280591867] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[17281535106] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[17282248632] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[17282937804] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[17283680040] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[17284396239] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[17285075379] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[17285732508] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=33264
[17286366471] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=973464
[17287031256] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[17287701090] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[17288407554] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[17289147480] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[17289806754] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[17290556382] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[17291209122] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[17291875590] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[17292519420] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[17293144011] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[17293806486] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[17294525424] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[17295263832] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[17296323396] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[17297066985] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[17298084309] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[17298912939] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[17299678341] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[17300497830] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[17301260229] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[17301912606] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[17302550067] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[17303193897] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=168464
[17303918082] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[17305016124] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[17305837263] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[17306708793] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[17307500430] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[17308341237] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[17309155974] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[17309995758] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[17311747563] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[17313530520] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[17314324302] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17322570573] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[17339699718] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[17344449705] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[17354979411] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[17355739071] [CONTRACT] [kernel] [CPU0] Spawning init process...
[17358098868] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[17361731244] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[17362735929] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [17370794364] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[17371887885] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[17397514728] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[17400926334] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[17402901615] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[17404659327] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[17418415443] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[17424097746] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[17427779886] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[17429052333] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[17434314051] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[17439466110] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[17441379252] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[17448644631] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[17450699937] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[17452653405] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[17454623142] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[17460599112] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[17462888124] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[17465244159] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[17468089023] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[17471041698] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[17473044171] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[17477540454] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[17528670126] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[17536873365] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[17543153265] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[17550122337] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[17554737915] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[17558801700] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[17563970985] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[17569238940] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[17573986485] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[17579301927] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[17585729667] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[17591703525] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[17597686821] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[17602576398] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[17608830723] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[17614087590] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[17619628818] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[17625541461] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[17630667912] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[17636280684] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[17642250417] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[17648827548] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[17654989242] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[17660647125] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[17666719488] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[17671956225] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[17676913947] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[17681862330] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[17687272548] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[17693429655] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[17697756747] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[17701226664] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[17706313812] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[17709846099] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/telnetd
[17713174116] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[17719361187] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[17724674121] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[17730457635] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[17736397899] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[17742133398] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[17748348255] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[17753885226] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[17757387054] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[17779152765] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[17925401604] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[17933659359] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[17935072419] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17937258966] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17942219724] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17943689379] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17952327690] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[17957392035] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[17959097277] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17961268281] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[17969543559] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[17970756903] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[17971883985] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17974568601] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17981184771] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[17984000067] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17993662500] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[17997565014] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[17998792911] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[18000747534] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[18004685259] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[18007787919] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[18009729078] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[18011916549] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18016594134] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 04:54:44 = 1775451284 unix_secs
[18018694617] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775451284, mono_ns=9008984589, offset=1775451274991015411ns
[18021234924] [INFO] [rtc_cmos] [CPU1] System clock anchored
[18035134854] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[18075938562] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[18085041315] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[18086003364] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18088084641] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18096562473] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18100254150] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[18109800225] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[18113667429] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[18117915321] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18120340788] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[18126479382] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[18133736148] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[18136426242] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[18137657043] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18140265528] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18151112991] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18154848690] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18166820100] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[18170666415] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18172029513] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18173973444] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[18178366668] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[18180836256] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[18185091573] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[18191208816] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[18193167465] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[18194891484] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[18197475219] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[18199246296] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[18212667264] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[18214548363] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[18733839795] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18740424483] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[18744020889] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[18746010129] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[18750707184] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[18752399754] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[18754537857] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[18755966856] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[18756947781] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[18759459609] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[18760671633] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[18761864022] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[18762917646] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[18763903686] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[18764859300] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[18766039743] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[18771454383] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[18772918890] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18775507146] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18783303858] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18786038370] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18793948008] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[18797274804] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[18798660672] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[18801135969] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352784 RFLAGS_BEFORE=134 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[18810413622] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[18823921215] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[18834579984] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[18865805970] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[18869063664] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[18872872821] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[18874766493] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[18875662542] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18878286768] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18881854695] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[18883253697] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18885112818] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18886038369] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[18892032951] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[18901639779] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[18902742903] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[18904773591] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[18906613737] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[18908871333] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[18909850938] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18911914494] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18917111532] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18918880068] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18922908081] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[18936169824] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[18939203943] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[18940652874] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[18942471669] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500224 RFLAGS_BEFORE=130 CR3_BEFORE=59940864 fs_base=0 gs_base=18446744071564586640
[18947968380] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[18948835026] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18950357943] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[18951393747] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18952620753] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[18956841255] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[18958435122] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18959444559] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[18961193493] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[18962163594] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18963527253] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[18965221143] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[18966898269] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[18969544968] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[18971978949] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[18973457382] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[18975442629] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[18976434807] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[18979374051] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[18981402990] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[19015564557] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[19017033618] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[19018329594] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[19019639925] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[19020956295] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[19022301243] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[19023785550] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[19025494884] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[19035383004] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[19037816061] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434144 RFLAGS_BEFORE=130 CR3_BEFORE=59805696 fs_base=0 gs_base=18446744071564586608
[19045741539] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[19412070348] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
[19413233796] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[19414661871] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583008 RFLAGS_BEFORE=134 CR3_BEFORE=68349952 fs_base=0 gs_base=18446744071564586576
[19418028531] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[19419792117] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[19420892964] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[19424457723] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19425781155] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[19427170785] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[19427957472] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19429209228] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[19429882362] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19437775401] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[19440604557] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[19443414045] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[19449054009] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[19452706614] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[19455968928] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[19458430629] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[19459509366] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19462144350] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19485543957] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19490348988] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[19539564594] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[20432174499] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[20436150471] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[20437911945] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20439288012] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369717024 RFLAGS_BEFORE=130 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[20445157095] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[20446326450] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[20447644569] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[20449626483] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650480 RFLAGS_BEFORE=134 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[20456471376] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[20458201335] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[20462404215] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using cooperative polling loop (2ms interval)
[20466649632] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[20468087541] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[20469196770] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[20470763478] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[20483763201] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[20488658025] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[20492312016] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[20495409594] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[20496313662] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20498164071] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20574344670] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[20601952437] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[20610248340] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[20613938004] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[20615858835] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20617610904] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369782960 RFLAGS_BEFORE=130 CR3_BEFORE=79892480 fs_base=0 gs_base=18446744071564586576
[20621443722] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[20622467547] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20624593242] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20625701448] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[20629712928] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[20634217032] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20635839180] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20656040559] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20657139162] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[20658101541] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[20660137839] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[20661072795] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[20663139651] [INFO] [ps2_mouse] [CPU3] ps2_mouse: using cooperative polling loop (2ms interval)
[20669369589] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[20672809344] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[20675001501] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[20675882073] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20677690605] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20684453757] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20687337363] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20695674186] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[20698923432] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[20700663423] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20702673453] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915344 RFLAGS_BEFORE=130 CR3_BEFORE=80826368 fs_base=0 gs_base=18446744071564586640
[20706123108] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[20706934743] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20708542239] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[20709431952] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20710471848] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[20717961066] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20721876351] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[20730135723] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[20733038799] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[20734747506] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20736662760] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981744 RFLAGS_BEFORE=134 CR3_BEFORE=80953344 fs_base=0 gs_base=18446744071564586576
[20740345296] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[20741485380] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20743982061] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20754842361] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[20758567599] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[20766610029] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[20769495945] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[20771906199] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[20772774660] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20774424264] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20802479148] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20807720076] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[20815982979] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[20819415540] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010d548
[20820913707] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20823078870] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113936 RFLAGS_BEFORE=134 CR3_BEFORE=81256448 fs_base=0 gs_base=18446744071564586640
[20829476217] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[20830537992] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20832236469] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[20833980651] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20853803916] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[20857428768] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20864283957] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[20867479776] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[20870603391] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[20871578904] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010e118
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20873737071] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20875413504] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370202192 RFLAGS_BEFORE=134 CR3_BEFORE=81530880 fs_base=0 gs_base=18446744071564586576
[20879089572] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[20880156231] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20881583844] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[20882339115] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20883305421] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20884939284] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[20885864670] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20888562453] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[20893032336] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20894740053] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20896483344] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20898229374] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=226 subj_lo=0
[20918585292] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20920527474] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20922535722] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20924076789] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[20938643022] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[20950501803] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[20958214299] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20959538622] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20960895120] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20962365534] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[20963620227] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[20967062391] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[20969318898] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20970718890] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20972070273] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20973710439] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[20975521908] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20977362120] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849264 RFLAGS_BEFORE=130 CR3_BEFORE=80564224 fs_base=0 gs_base=18446744071564586608
[20982770127] [INFO] [nectar] [CPU2] NECTAR: Started.
[20986602120] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
[20988333696] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20989855425] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20992043457] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20993767443] [INFO] [fontd] [CPU3] FONTD: Service ready
[20994536541] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20996404011] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047952 RFLAGS_BEFORE=130 CR3_BEFORE=81100800 fs_base=0 gs_base=18446744071564586608
[21001604481] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[21003917154] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[21006221742] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21009054396] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370267728 RFLAGS_BEFORE=134 CR3_BEFORE=81739776 fs_base=0 gs_base=18446744071564586608
[21016568199] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[21019686138] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[21021947661] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[21025706691] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[21027897924] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[21030592506] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[21031565082] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21033259203] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[21034138554] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21035781525] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21037700805] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[21038745750] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=235 subj_lo=0
[21040490097] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[21042501348] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4e92000
[21044325951] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[21046915395] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[21049464051] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[21050607039] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21052009341] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21053852688] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21055659537] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[21056725899] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=237 subj_lo=0
[21065917950] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21067354671] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21068849274] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21070247220] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=238 subj_lo=0
[21073165971] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[21078363306] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21079560315] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21080918859] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21082319478] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=239 pred=0 subj_lo=0
[21084754845] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[21097441299] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[21100823469] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[21103193628] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[21105375984] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[21107019516] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[21109206789] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[21110627901] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21111831939] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[21113015583] [INFO] [anther] [CPUETD: Link is UP
[21122458335] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=19, read=20)
[21129260097] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=21, read=22)
[21136345758] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[21141642192] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([230, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21156650922] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([230, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21171502638] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[21184705278] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1257 backend=VirtIO-GPU
[21187737087] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[21189082563] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21191954982] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21205553193] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[21220325082] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([230, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21240148908] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=23, resp=26
[21242094852] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[21248175465] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21249784809] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21349615617] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d6000 exec=false
[21366607713] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ec000 exec=false
[21375744225] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[21382816686] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db4a0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e9
[21384894564] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370509328 RFLAGS_BEFORE=130 CR3_BEFORE=82505728 fs_base=0 gs_base=18446744071564586640
[21393065892] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[21419716527] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[21422344185] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[21423278316] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21425267820] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21428520432] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[21431295765] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21433631736] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[21439392348] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([230, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21443688552] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[21448297497] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[21449977692] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db668
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[21452774805] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[21453752364] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370574864 RFLAGS_BEFORE=130 CR3_BEFORE=84074496 fs_base=0 gs_base=18446744071564586576
[21460929534] [INFO] [echo] [CPU1] echo: starting up
[21461809215] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=19, RX port=22
[21463986753] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[21465450666] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[21466905669] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[21497916726] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[21499429380] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[21507312552] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[21509764155] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[21514616475] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[21517613931] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[21518738010] [INFO] [netd] [CPU3] NETD: Driver TX port=19, RX port=22, link_up=true, mtu=1500
[21520063191] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[21521283795] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21523616301] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[21524766417] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21526139085] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21527433378] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21528233694] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[21531772053] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21533247021] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[21534424857] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[21542911731] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[21546210048] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[21549079662] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[21551185491] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[21553386492] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[21554656530] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21556718040] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21562652133] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[21564588870] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[21565702752] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21566614212] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[21569681001] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[21579736134] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[21585297459] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[21587322075] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[21588829911] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[21591124500] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f4220
[21592135686] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[21593546172] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21596340315] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21597607944] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370742800 RFLAGS_BEFORE=130 CR3_BEFORE=84336640 fs_base=0 gs_base=18446744071564586640
[21601262199] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[21603661992] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[21606987171] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[21616243737] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[21618125562] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[21622931286] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[21625182381] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/telnetd'
[21628756677] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/telnetd
[21630029850] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21632602530] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21639726768] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f4220
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21642499362] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370808336 RFLAGS_BEFORE=130 CR3_BEFORE=84484096 fs_base=0 gs_base=18446744071564586576
[21654353457] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[21663593556] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=220000 exec=false
[21671022846] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=228000 exec=false
[21679381350] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 32 (user task/process) assigned to CPU 2
[21680645184] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[21683792922] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[21686545122] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[21691621941] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f4220
[21693520728] [INFO] [netd] [CPU3] NETD: Starting DHCP...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21695675331] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[21696869172] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=32 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370873872 RFLAGS_BEFORE=130 CR3_BEFORE=84586496 fs_base=0 gs_base=18446744071564586608
[21702514944] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[21703433763] [INFO] [telnetd] [CPU2] telnetd: starting on port 2323
[21706083399] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=32)
[21707582853] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[21726297417] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[21728311077] [INFO] [anther] [CPU1] anther: Connected to network stack
[21756870993] [INFO] [telnetd::net_client] [CPU2] telnetd: connected to socket API (netd=27, write=31, read=32)
[21759714306] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[21768331233] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[21787204956] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[21828446739] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[21831517356] [INFO] [bloom] [CPU3] bloom: creating surface...
[21833181843] [INFO] [bloom] [CPU3] bloom: surface created!
[21834804486] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[21840765771] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[21846608256] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1263
[21847734744] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[21861694998] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[21864908769] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=33)
[21866181645] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
T:0AF0 [21871300539] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[21881409891] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[21894670314] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[21908338056] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
T:1050 T:0EC0 T:F930 [21938187546] [INFO] [bloom] [CPU3] [bloom] dynamically subscribed to input topic 0 on svc.Input 1241 via port 34
[21941068809] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 34 (legacy was 12)
[21982955313] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1263
[22008968025] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[22011350460] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[22015326663] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[22026673614] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 37 (user thread) assigned to CPU 3
[22029277578] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=37 (priority=2)
[22031352024] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[22033535370] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[22045313136] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=268
[22046937429] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[22048363689] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22049681577] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22051027581] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22052458560] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=268 subj_lo=0
T:1AA0 [22056728694] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[22058713809] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[22072768311] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1269)
[22074676800] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[22090248873] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=239
[22091820366] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[22092964344] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22094092053] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22095368757] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22097003049] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=239 pred=0 subj_lo=0
[22110634293] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1274)
[22112391873] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[22121915904] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=269
[22123313586] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[22125092319] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22126541415] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22127918076] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22129310841] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=269 subj_lo=0
[22140271659] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1275)
[22141970202] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[22403440587] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[22432635456] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=635 watches=13 history=1024 journal=1024 symbols=280 drops=0
[22603398345] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[22635392241] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[22732112766] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[22765412373] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[22768215789] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[22776286533] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[23060131875] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[23062286907] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[23883041556] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[24507622953] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=666 watches=13 history=1024 journal=1024 symbols=326 drops=0
[24952275051] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[24953373951] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[24978935850] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[24986066523] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[24992103147] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[24993691041] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[25078964922] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25103432574] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 1387.264ms (rebuilds=0 pending=true)
[25177120749] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[25213490511] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[25214656203] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25215706032] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25216799586] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=285 pred=0 subj_lo=0
[25249118334] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[25262563194] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 1553.168ms
[25264454325] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[25266103962] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[25320204030] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[25321508322] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25322825913] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25324351272] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=286 pred=0 subj_lo=0
[25433063007] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[25472338848] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[25473837081] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25475201631] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25476620895] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=327 pred=0 subj_lo=0
[25546302870] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[25585375200] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12001000
[25586847363] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[25588205049] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[25662272856] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[25672902486] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[25677415797] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[25679643825] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[25681835850] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[25686290751] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[25697408616] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[25699039641] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[25700958954] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[25713869478] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x12002000
[25715318772] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[25884243726] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[26118698661] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[26119809012] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[26223509730] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=710 watches=16 history=1024 journal=1024 symbols=341 drops=0
[27406316718] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[27408054762] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[27410299158] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[27753904728] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=714 watches=16 history=1024 journal=1024 symbols=343 drops=0
[27796075560] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[27811309647] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[27814754517] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[27816559419] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[27836463270] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[27837819867] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[27839089707] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[27840510522] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=328 pred=0 subj_lo=0
[27863419881] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[27867174654] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[27870577548] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[27874499334] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[27878636247] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[27881185827] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 2323 with backlog 64
[27883609446] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 2323, handle=2
[27884660925] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[27885848661] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[27886886412] [INFO] [telnetd] [CPU2] telnetd: listening on guest port 2323 (handle=2)
[27888582942] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=3
[27890642472] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[27892580067] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=4
[27894454467] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[27896192940] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=5
[27898095555] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[27899834655] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=6
[27921509847] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[27923774373] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[27929325171] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[27967259562] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[27988033095] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=77c80b059e863710)
[28058916567] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 1348.465ms (rebuilds=0 pending=true)
[28239026244] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[28240753662] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[28243477449] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28249768404] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28255503903] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[28275218631] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=70
[28277352279] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[28279900671] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28280892354] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 1508.950ms
[28281661254] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[28282553772] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[28284571491] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=2 (16384b)
[28295007147] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28304018787] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[28305958989] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[28320766815] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=74
[28322439090] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=21
[28325163273] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[28334562762] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[28338565629] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[28341432009] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[28360385493] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[28369775445] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=70
[28371571536] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[28374136824] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28380456720] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28381571658] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[28384049661] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[28388423316] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28400194647] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28402923780] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[28419571488] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[28601350998] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28735489398] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[28953060708] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 254.585ms (rebuilds=0 pending=true)
[28965948000] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=2
[29104968057] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29198858007] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[29216723844] [INFO] [bloom] [CPU3] [bloom] acquire_buffer took 103.090ms
[29231292552] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 475.560ms
[29579953293] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29603311221] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[29736291354] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[29754193656] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[29759894439] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[29761498272] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[29764130121] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[29766287298] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[29798492790] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=Established
[29800382106] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:52144 on listener 2
[29824394226] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 60 bytes - TCP ACK
[29860627896] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 265.926ms (rebuilds=0 pending=true)
[29880740868] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 324.648ms
[29954891670] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 60 bytes
[29965291950] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=85
[29966791998] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 75 byte frame (79 encoded) to netd rx_port=21
[29969068998] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (79 bytes sent)
[29975786544] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 75 bytes
[29979656091] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 118 bytes - TCP ACK
[29996832426] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[30331420218] [INFO] [phloem::executor] [CPU2] phloem: entering discover_nodes
[30333048537] [INFO] [phloem::executor] [CPU2] phloem: starting BFS discovery from roots
[30342396216] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 118 bytes
[30356418411] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[30358253178] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[30361160148] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30364885947] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30367299303] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 75 bytes - TCP ACK
[30382316217] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 75 bytes
[30409921278] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[30413769639] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[30419399307] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30428978844] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30524619246] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 282.323ms (rebuilds=0 pending=true)
[30527413785] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[30539305962] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 329.365ms
[30560375043] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30569888745] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=833 watches=17 history=1024 journal=1024 symbols=366 drops=0
[30768991044] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30808175079] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=671285c0c9ba41ed)
[30992246637] [INFO] [phloem::executor] [CPU2] phloem: BFS seeded with 533 nodes
[31101838911] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[31124438631] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 33264 bytes, hash=91427bae3069f281)
[31186674717] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 282.149ms (rebuilds=0 pending=true)
[31201852242] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 331.089ms
[31542412770] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 973464 bytes, hash=367d1d58d6b9406c)
[31550203971] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[31819188654] [INFO] [phloem::executor] [CPU2] phloem: discovered 50 nodes
[31842882885] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 65 bytes - TCP ACK
[31863847422] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 276.893ms (rebuilds=0 pending=true)
[31878830148] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 338.690ms
[31889227194] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[32345873571] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 65 bytes
[32519665002] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 285.273ms (rebuilds=0 pending=true)
[32528380533] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 324.763ms
[32707824138] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[32710386390] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[32713368138] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[32724696345] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[32726915199] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 307 bytes - TCP ACK
[32740249839] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 307 bytes
[33135987786] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 264.382ms (rebuilds=0 pending=true)
[33145198218] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 308.402ms
[33412940847] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=948 watches=17 history=1024 journal=1024 symbols=374 drops=0
[33743704401] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[33746220024] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[33751964961] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[33763111701] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[33765271584] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 373 bytes - TCP ACK
[33779957838] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 373 bytes
[33822463917] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[33825687456] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[33828511464] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[33833628576] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[33850243119] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=4e1c9cf03988a560)
[33864044643] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 313.369ms (rebuilds=0 pending=true)
[33874984110] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 364.783ms
[34266424566] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[34348497084] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[34546075707] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 296.968ms (rebuilds=0 pending=true)
[34552504602] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 338.844ms
[34795555872] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[34802325987] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[35213601390] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 291.249ms (rebuilds=0 pending=true)
[35224650813] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 336.019ms
[35324793768] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[35331004533] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[35402315982] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[35414253831] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[35416446714] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35417489250] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35418543798] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[35420030085] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=226 subj_lo=0
[35474501073] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35476006797] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35477865918] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[35479957095] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=366 pred=0 subj_lo=0
[36179655204] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[36229323834] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 443.577ms (rebuilds=0 pending=true)
[36241052760] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 508.248ms
[36262512264] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[36841977084] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[36916820028] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[36936698172] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=1080 watches=19 history=1024 journal=1024 symbols=377 drops=0
[36989215263] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 311.943ms (rebuilds=0 pending=true)
[37002355533] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 380.621ms
[37140934545] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2429 ops=1 watches=19
[37457700225] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[37525102560] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[37934632461] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 407.657ms (rebuilds=0 pending=true)
[37946710758] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 472.206ms
[38159144397] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[38252318280] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[38870874273] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 375.463ms (rebuilds=0 pending=true)
[38895095976] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 474.088ms
[38906212653] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[38952302796] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=0e57f834b3afbcb1)
[39548646918] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[39613400310] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[39791799762] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 401.205ms (rebuilds=0 pending=true)
[39808318671] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 456.492ms
[40246837092] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[40295623203] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=fc99b972ad70b9cd)
[40717353039] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 387.929ms (rebuilds=0 pending=true)
[40735723578] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 463.893ms
[40894868949] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1200 watches=19 history=1024 journal=1024 symbols=377 drops=0
[40926371442] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[40957687353] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[41445939612] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[41476863516] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 305.233ms (rebuilds=0 pending=true)
[41489081898] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 376.352ms
[41553709461] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[42307513512] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[42315615408] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 363.287ms (rebuilds=0 pending=true)
[42327124653] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 418.818ms
[42332794911] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[42896879586] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[42942036390] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[43069749954] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 321.704ms (rebuilds=0 pending=true)
[43078366056] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 375.920ms
[43509036087] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[43571759286] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[43827365769] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 335.952ms (rebuilds=0 pending=true)
[43844804124] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 383.486ms
[44162226009] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[44280397293] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[44605082346] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 341.780ms (rebuilds=0 pending=true)
[44623588944] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1332 watches=19 history=1024 journal=1024 symbols=377 drops=0
[44626821525] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 389.493ms
[44911948851] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[44957320485] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[45467858139] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 365.177ms (rebuilds=0 pending=true)
[45484917819] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 430.441ms
[45682314909] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[45758143959] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[46324963245] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 361.043ms (rebuilds=0 pending=true)
[46339810077] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 427.471ms
[46470001644] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[46482628071] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[47186653635] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 355.921ms (rebuilds=0 pending=true)
[47216352315] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 438.335ms
[47365527132] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[47395819350] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[47999632032] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 322.003ms (rebuilds=0 pending=true)
[48006055152] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 394.921ms
[48102160788] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[48174514740] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1454 watches=19 history=1024 journal=1024 symbols=377 drops=0
[48182742828] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[48749595840] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 308.517ms (rebuilds=0 pending=true)
[48761547648] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 377.718ms
[48860156928] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[48953285040] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 168464 bytes, hash=35db16f07de81e7e)
[49458403896] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 301.783ms (rebuilds=0 pending=true)
[49474981875] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 356.699ms
[49884865701] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[49976453736] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[50228855556] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 316.892ms (rebuilds=0 pending=true)
[50248250184] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 386.572ms
[50964057276] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 287.897ms (rebuilds=0 pending=true)
[50967239895] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 359.658ms
[51413447778] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[51546670032] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[51622457898] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=1559 watches=19 history=1024 journal=1024 symbols=378 drops=0
[51683320953] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 315.762ms (rebuilds=0 pending=true)
[51696222633] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 364.168ms
[52472421771] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 341.528ms (rebuilds=0 pending=true)
[52483512312] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 393.423ms
[52609259571] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[52629849459] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[53252377860] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=3453 ops=1 watches=19
[53318074887] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 320.342ms (rebuilds=0 pending=true)
[53321893350] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 419.674ms
[53797312668] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[53845821117] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[53884928988] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 235.950ms (rebuilds=0 pending=true)
[53897423844] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 287.736ms
[54686219280] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 347.939ms (rebuilds=0 pending=true)
[54691036422] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 396.854ms
[54917863539] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[54919400052] [INFO] [fontd] [CPU3] FONTD: Auto-importing font asset ThingId([7, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[55260217551] [INFO] [fontd] [CPU3] FONTD: Auto-imported font 'Noto Sans' style 'Regular' from asset ThingId([7, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[55263515538] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (1 fonts)
[55311324519] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=1687 watches=19 history=1024 journal=1024 symbols=381 drops=0
[55484462022] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 352.467ms (rebuilds=0 pending=true)
[55498962618] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 403.917ms
[55656470925] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[55862875233] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[55973230434] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[56276172744] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 325.205ms (rebuilds=0 pending=true)
[56296100916] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 398.584ms
[56991521961] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[56993938023] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[57162020784] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 381.529ms (rebuilds=0 pending=true)
[57173716578] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 438.126ms
[58115335641] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 418.616ms (rebuilds=0 pending=true)
[58126033548] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 476.855ms
[58985617812] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 375.987ms (rebuilds=0 pending=true)
[58994755842] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 434.316ms
[59441071800] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=16000 nodes=1827 watches=19 history=1024 journal=1024 symbols=409 drops=0
[59843528349] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 370.899ms (rebuilds=0 pending=true)
[59858774745] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 431.723ms
[60600024705] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 317.907ms (rebuilds=0 pending=true)
[60618117384] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 379.887ms
[61358542971] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 322.809ms (rebuilds=0 pending=true)
[61376935983] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 379.251ms
[62129205369] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 334.488ms (rebuilds=0 pending=true)
[62146489350] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 385.006ms
[62992352676] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 383.465ms (rebuilds=0 pending=true)
[63007017612] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 430.099ms
[63766856472] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=17000 nodes=1997 watches=19 history=1024 journal=1024 symbols=458 drops=0
[63857469984] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 360.948ms (rebuilds=0 pending=true)
[63878896191] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 435.905ms
[64536230847] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[64792158387] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[64853571420] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 404.109ms (rebuilds=0 pending=true)
[64860370872] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[64872759864] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 496.787ms
[65655057138] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[65657098815] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[65706280266] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[65789154948] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 394.587ms (rebuilds=0 pending=true)
[65801905422] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 464.880ms
[66705229767] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 400.759ms (rebuilds=0 pending=true)
[66731368638] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 464.191ms
[67065831558] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[67229011278] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[67230067707] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[67231292205] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[67232577654] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[67271246592] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[67272472872] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[67273676877] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[67274855274] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=236 pred=0 subj_lo=0
[67295282175] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[67296347481] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[67297497993] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[67298574948] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=463 pred=0 subj_lo=0
[67336338003] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[67337622297] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[67339020705] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[67340805048] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=367 pred=0 subj_lo=0
[67357661679] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[67358741373] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[67359957126] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[67361075694] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[67381735278] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[67714140516] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 407.542ms (rebuilds=0 pending=true)
[67725780375] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 497.809ms
[68049706032] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=18000 nodes=2116 watches=24 history=1024 journal=1024 symbols=464 drops=0
[68135835570] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=4477 ops=1 watches=24
[68726416968] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 415.919ms (rebuilds=0 pending=true)
[68737534206] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 505.890ms
[69655038420] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 424.071ms (rebuilds=0 pending=true)
[69669268350] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 465.519ms
[70604509338] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 412.784ms (rebuilds=0 pending=true)
[70619590272] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 475.501ms
[71498723802] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 372.132ms (rebuilds=0 pending=true)
[71517190536] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 448.642ms
[71930574144] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=19000 nodes=2224 watches=24 history=1024 journal=1024 symbols=464 drops=0
[72468147609] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 390.338ms (rebuilds=0 pending=true)
[72481786014] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 482.408ms
[74183180709] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 769.985ms (rebuilds=0 pending=true)
[74272408782] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 894.753ms
[76674005430] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 995.834ms (rebuilds=0 pending=true)
[76731287325] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 1229.001ms
[78362937015] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=20000 nodes=2316 watches=24 history=1024 journal=1024 symbols=464 drops=0
[78820518711] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 691.392ms (rebuilds=0 pending=true)
[78834957465] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 1052.884ms
[79370075268]
```
</details>
