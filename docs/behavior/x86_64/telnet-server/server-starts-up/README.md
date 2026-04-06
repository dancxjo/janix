# ✅ Scenario: Server starts up

> Last run: 2026-04-05 21:54:16

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 4563ms | - [📜](./01/serial.log) - |
| 2 | Then I should see a message in the serial output that says "telnetd: listening on guest port 2323" | ✅ | 4755ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[14546338323] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[14553635712] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[14557525983] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[14559640227] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[14560879938] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[14561544459] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[14562226668] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[14562828819] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[14563417572] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[14564146146] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=33264
[14564742687] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=973464
[14565346752] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[14566079121] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[14566745886] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[14567432583] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[14568078426] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[14568875409] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[14569505841] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[14570136966] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[14570751228] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[14571346482] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[14571946521] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[14572554282] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[14573177652] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[14573900517] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[14574795147] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[14575664796] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[14576536656] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[14577389640] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[14578018983] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[14578705284] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[14579381091] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[14580105540] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[14580746862] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=168464
[14581364028] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[14582142432] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[14582976573] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[14583797679] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[14584586346] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[14585403921] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[14586186648] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[14586950136] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[14588366628] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[14589884067] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[14590881624] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[14591650788] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[14592355503] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[14593111236] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[14593974318] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[14594847861] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[14595660387] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[14596510071] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[14597301741] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[14598142350] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7795e000 (Usable)
[14598951312] [INFO] [kernel::memory] [CPU0]   [11] 0x7795e000 - 0x779c2000 (Reserved)
[14599824789] [INFO] [kernel::memory] [CPU0]   [12] 0x779c2000 - 0x779c3000 (Other)
[14600451129] [INFO] [kernel::memory] [CPU0]   [13] 0x779c3000 - 0x779c4000 (Reserved)
[14601091659] [INFO] [kernel::memory] [CPU0]   [14] 0x779c4000 - 0x779c5000 (Other)
[14601679290] [INFO] [kernel::memory] [CPU0]   [15] 0x779c5000 - 0x779c6000 (Reserved)
[14602280583] [INFO] [kernel::memory] [CPU0]   [16] 0x779c6000 - 0x779c7000 (Other)
[14602841088] [INFO] [kernel::memory] [CPU0]   [17] 0x779c7000 - 0x779c8000 (Reserved)
[14603540160] [INFO] [kernel::memory] [CPU0]   [18] 0x779c8000 - 0x77a53000 (Other)
[14604371364] [INFO] [kernel::memory] [CPU0]   [19] 0x77a53000 - 0x77a54000 (Reserved)
[14605012455] [INFO] [kernel::memory] [CPU0]   [20] 0x77a54000 - 0x77ed5000 (Other)
[14605692552] [INFO] [kernel::memory] [CPU0]   [21] 0x77ed5000 - 0x77ed6000 (Reserved)
[14606339847] [INFO] [kernel::memory] [CPU0]   [22] 0x77ed6000 - 0x77ff7000 (Other)
[14606959422] [INFO] [kernel::memory] [CPU0]   [23] 0x77ff7000 - 0x77ff8000 (Reserved)
[14607601008] [INFO] [kernel::memory] [CPU0]   [24] 0x77ff8000 - 0x787f8000 (Other)
[14608219923] [INFO] [kernel::memory] [CPU0]   [25] 0x787f8000 - 0x787f9000 (Reserved)
[14608881045] [INFO] [kernel::memory] [CPU0]   [26] 0x787f9000 - 0x788ba000 (Other)
[14609480028] [INFO] [kernel::memory] [CPU0]   [27] 0x788ba000 - 0x788bb000 (Reserved)
[14610066570] [INFO] [kernel::memory] [CPU0]   [28] 0x788bb000 - 0x788e5000 (Other)
[14610632883] [INFO] [kernel::memory] [CPU0]   [29] 0x788e5000 - 0x788e6000 (Reserved)
[14611219227] [INFO] [kernel::memory] [CPU0]   [30] 0x788e6000 - 0x788eb000 (Other)
[14611786695] [INFO] [kernel::memory] [CPU0]   [31] 0x788eb000 - 0x788ec000 (Reserved)
[14612387724] [INFO] [kernel::memory] [CPU0]   [32] 0x788ec000 - 0x788f1000 (Other)
[14612953311] [INFO] [kernel::memory] [CPU0]   [33] 0x788f1000 - 0x788f2000 (Reserved)
[14613549885] [INFO] [kernel::memory] [CPU0]   [34] 0x788f2000 - 0x7891e000 (Other)
[14614128804] [INFO] [kernel::memory] [CPU0]   [35] 0x7891e000 - 0x78920000 (Reserved)
[14614728942] [INFO] [kernel::memory] [CPU0]   [36] 0x78920000 - 0x78929000 (Other)
[14615310138] [INFO] [kernel::memory] [CPU0]   [37] 0x78929000 - 0x7892b000 (Reserved)
[14615924202] [INFO] [kernel::memory] [CPU0]   [38] 0x7892b000 - 0x78933000 (Other)
[14616504573] [INFO] [kernel::memory] [CPU0]   [39] 0x78933000 - 0x78934000 (Reserved)
[14617103127] [INFO] [kernel::memory] [CPU0]   [40] 0x78934000 - 0x7893e000 (Other)
[14617680033] [INFO] [kernel::memory] [CPU0]   [41] 0x7893e000 - 0x7893f000 (Reserved)
[14618276541] [INFO] [kernel::memory] [CPU0]   [42] 0x7893f000 - 0x7894c000 (Other)
[14618864601] [INFO] [kernel::memory] [CPU0]   [43] 0x7894c000 - 0x7894e000 (Reserved)
[14619461010] [INFO] [kernel::memory] [CPU0]   [44] 0x7894e000 - 0x7895c000 (Other)
[14620036068] [INFO] [kernel::memory] [CPU0]   [45] 0x7895c000 - 0x7895d000 (Reserved)
[14620632378] [INFO] [kernel::memory] [CPU0]   [46] 0x7895d000 - 0x78969000 (Other)
[14621207337] [INFO] [kernel::memory] [CPU0]   [47] 0x78969000 - 0x7896a000 (Reserved)
[14621789226] [INFO] [kernel::memory] [CPU0]   [48] 0x7896a000 - 0x7896e000 (Other)
[14622363954] [INFO] [kernel::memory] [CPU0]   [49] 0x7896e000 - 0x7896f000 (Reserved)
[14623004121] [INFO] [kernel::memory] [CPU0]   [50] 0x7896f000 - 0x7897f000 (Other)
[14623661316] [INFO] [kernel::memory] [CPU0]   [51] 0x7897f000 - 0x78980000 (Reserved)
[14624246472] [INFO] [kernel::memory] [CPU0]   [52] 0x78980000 - 0x78a10000 (Other)
[14624809683] [INFO] [kernel::memory] [CPU0]   [53] 0x78a10000 - 0x78a11000 (Reserved)
[14625443712] [INFO] [kernel::memory] [CPU0]   [54] 0x78a11000 - 0x78a1c000 (Other)
[14626042431] [INFO] [kernel::memory] [CPU0]   [55] 0x78a1c000 - 0x78a1d000 (Reserved)
[14626656627] [INFO] [kernel::memory] [CPU0]   [56] 0x78a1d000 - 0x78a42000 (Other)
[14627249967] [INFO] [kernel::memory] [CPU0]   [57] 0x78a42000 - 0x78a43000 (Reserved)
[14627863008] [INFO] [kernel::memory] [CPU0]   [58] 0x78a43000 - 0x78a4f000 (Other)
[14628456612] [INFO] [kernel::memory] [CPU0]   [59] 0x78a4f000 - 0x78a50000 (Reserved)
[14629292931] [INFO] [kernel::memory] [CPU0]   [60] 0x78a50000 - 0x78a5d000 (Other)
[14630130174] [INFO] [kernel::memory] [CPU0]   [61] 0x78a5d000 - 0x78a5e000 (Reserved)
[14630891121] [INFO] [kernel::memory] [CPU0]   [62] 0x78a5e000 - 0x78aa6000 (Other)
[14631660417] [INFO] [kernel::memory] [CPU0]   [63] 0x78aa6000 - 0x78aa7000 (Reserved)
[14632792218] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[14886925878] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485750 free frames
[14897780898] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[14902767693] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[14904579195] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[14905854381] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[14910465966] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[14912288490] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[14913448869] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[14914150779] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[14914832955] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[14915528067] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[14916498564] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[14917466157] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[14918200902] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[14918946603] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[14919664881] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[14920381410] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[14921574756] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[14922618084] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[14923329234] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[14925007086] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[14926328175] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[14927338767] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[14929026024] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[14930686056] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[14931929001] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[14932729185] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[14933826567] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[15327626094] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[15328669059] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[15331863954] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[15332755548] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[15333548175] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[15335054757] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[15347695110] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[15349103385] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[15349905219] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[15351732330] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[15352320159] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[15354825849] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[15362664207] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[15364429971] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[15378033726] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[15378663432] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[15395730834] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[15396397566] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[15398508048] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[15399801879] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[15400919490] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[15403511706] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[15404427753] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[15439453128] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62356100 ticks/sec), init_cnt=623561 for 100Hz
[15440998683] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[15441813849] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[15443001882] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[15448707021] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[15479613270] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[15481043061] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[15482898915] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[15484404837] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[15485803806] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[15488842545] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[15490383447] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[15511917498] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[15513047979] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[15514080483] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[15515017155] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[15516039561] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[15516842484] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[15518096913] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[15546372138] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[15548051805] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[15549161496] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[15550582938] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[15551785161] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[15553042131] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[15553879836] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[15555090507] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[15562569594] [INFO] [kernel::root] [CPU0] Spawning Root service...
[15563609259] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[15565538439] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[15566472933] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[15571129893] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[15572669838] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[15573384882] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[15574689900] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[15575783916] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[15576907038] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[15590098887] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[15593936886] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[15595519401] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[15596381922] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[15619151559] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15641286573] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15643570899] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15648550104] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15650697183] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15653117073] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15655315764] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15657369948] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[15658236363] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[15659352852] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15666011427] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15668058648] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15671582322] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15675470745] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15678750285] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15681622341] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[15682519842] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[15695001333] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[15695901342] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[15703443954] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[15704412306] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[15734548401] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[15735558399] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[16119518448] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[16640348571] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[16673007285] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=499 journal=424 symbols=51 drops=0
[16713687474] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[18135523626] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=449 watches=0 history=965 journal=773 symbols=98 drops=0
[18967036077] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[19071399798] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[19072524735] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[19180320093] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[19256094495] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[19288652163] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[19289882139] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[19290681234] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[19298254404] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[19315189212] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[19334370990] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[19336725903] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[19390923816] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[19413032694] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[19414004973] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[19419215343] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[19446904059] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[19471037520] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[19476376161] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[19477418928] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[19554636915] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[19556625594] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[19652452050] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[19722727695] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[19739501496] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[19745205711] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[19747780899] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[19802778369] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[19839124602] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[19844815683] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[19845788193] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[19846658271] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[19847544321] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[19848218973] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[19848869964] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[19849558476] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[19850527125] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[19851183330] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[19852200126] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=33264
[19852882599] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=973464
[19853799669] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[19854672717] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[19855473825] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[19856297538] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[19857088119] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[19857806562] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[19858478244] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[19859172564] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[19859955126] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[19860594336] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[19861230972] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[19861876221] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[19862547276] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[19863223215] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[19863861534] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[19864512723] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[19865272350] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[19866069960] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[19866823647] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[19867535226] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[19868230833] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[19868908884] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[19869630726] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=168464
[19870302738] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[19871056821] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[19871825028] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[19872687780] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[19873455129] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[19874247921] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[19875000420] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[19875798855] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[19877433114] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[19879246266] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[19880399847] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19889161875] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[19904676924] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[19909709919] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[19919799273] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[19920833229] [CONTRACT] [kernel] [CPU0] Spawning init process...
[19923079176] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[19925900511] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[19926636147] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [19933886742] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013536 RFLAGS_BEFORE=130 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[19938953826] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[19955636118] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[19961501868] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[19962847014] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[19963950897] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[19974220497] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[19979956524] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[19983531414] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[19984770135] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[19988225796] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[19991682546] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[19992931233] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[19997140581] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[19998420090] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[19999966272] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[20001520407] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[20005628610] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[20006978508] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[20008544985] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[20010205347] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[20012103639] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[20013837987] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[20023922094] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[20070822552] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[20078995629] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[20086881639] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[20093393529] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[20097044286] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[20101727910] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[20106760707] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[20113862406] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[20119344960] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[20124562920] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[20129616375] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[20135397183] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[20142929928] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[20148303615] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[20153224674] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[20158421778] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[20166504039] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[20174211453] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[20181397566] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[20188811808] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[20198076888] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[20203975968] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[20209961904] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[20215496895] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[20222655684] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[20228470020] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[20233208523] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[20238414570] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[20243324871] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[20250891045] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[20255343702] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[20258720427] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[20263737417] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[20267251356] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/telnetd
[20270309400] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[20276373381] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[20283137886] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[20288752803] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[20294065341] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[20300286105] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[20306983257] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[20313713145] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[20316996183] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[20340261909] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[20495286603] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[20503301280] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[20504096316] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20505751893] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20510843925] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20512118781] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20521620603] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[20528133285] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[20529140676] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20530604259] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079072 RFLAGS_BEFORE=130 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[20535326295] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[20536385958] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[20537232738] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[20539000878] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20543572401] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[20545558308] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20555066334] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[20558656404] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[20560119558] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[20561758998] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144608 RFLAGS_BEFORE=130 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[20565792423] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[20569314150] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[20571012000] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[20573249598] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20579576490] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 04:54:27 = 1775451267 unix_secs
[20581638792] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775451267, mono_ns=10290563382, offset=1775451256709436618ns
[20583255462] [INFO] [rtc_cmos] [CPU1] System clock anchored
[20596684845] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[20639557092] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[20650028487] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[20651026506] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[20652912555] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20659834866] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[20663373621] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[20672564385] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[20676085584] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[20679410862] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20681282556] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210656 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[20687519193] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[20693047782] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[20695743651] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[20696905119] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[20699526540] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20710554051] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20714425149] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[20723025576] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[20727193443] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[20728771140] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20730509349] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[20731809021] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277408 RFLAGS_BEFORE=130 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[20738934546] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[20740474755] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[20750492235] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[20752160319] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[20753098377] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[20755455699] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[20757444180] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[20776992225] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[20778775248] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[21296020152] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21309459468] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[21312617172] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[21314471871] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[21320267694] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[21322387317] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[21324739293] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[21326370780] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[21327471264] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[21330035826] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[21331393611] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[21332491818] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[21333454362] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[21334459377] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[21335505708] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[21336714498] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[21350487972] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[21351609807] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[21353477904] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21363538614] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21366154491] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[21374916750] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[21378018519] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[21379162695] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[21381185430] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369357120 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[21404885568] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[21413023896] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[21426082821] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[21438104721] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[21440470953] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[21446119893] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[21455848260] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[21457438530] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[21463469082] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[21464481192] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[21465342624] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[21466157658] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[21466975233] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[21467974902] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[21468698757] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[21469942230] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[21470858607] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[21958755291] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[21962967510] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[21967059213] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[21969343902] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[21970092243] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21971688057] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21976646571] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[21978666732] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21987227658] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[21990562572] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[21992567553] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[21993325662] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21994951572] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21999991629] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22001515404] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[22009860609] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[22012822854] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
[22013812227] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0107648
[22014956172] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[22016666760] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500768 RFLAGS_BEFORE=134 CR3_BEFORE=68608000 fs_base=0 gs_base=18446744071564586640
[22021622865] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[22022600259] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22024343616] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22030296453] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[22033314369] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22034302455] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[22036154877] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[22041440751] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[22042507410] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[22044033891] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0107648
[22045981782] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[22047935844] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[22048833213] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583712 RFLAGS_BEFORE=134 CR3_BEFORE=68894720 fs_base=0 gs_base=18446744071564586576
[22052158689] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[22053316428] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[22055029260] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[22056974181] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[22058006454] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[22059369882] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[22060370145] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[22061984901] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
[22063606125] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[22064611503] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[22066065351] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434976 RFLAGS_BEFORE=134 CR3_BEFORE=68395008 fs_base=0 gs_base=18446744071564586608
[22069477650] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[22071985320] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[22073059998] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[22074587469] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[22080408867] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1241
[22081171959] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1240)
[22082485359] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[22084533306] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[22097724627] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22099961070] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[22100959353] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22102816428] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22110432003] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[22113385437] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[22122708564] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[22126570620] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[22128829701] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[22130478018] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[22131315921] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22133080101] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22156046748] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[22161025194] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[22482609006] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[22484005236] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[22486261809] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[22488710871] [INFO] [ps2_mouse] [CPU3] ps2_mouse: using cooperative polling loop (2ms interval)
[23071390683] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[23075507466] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a670
[23077012695] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23078892870] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716320 RFLAGS_BEFORE=134 CR3_BEFORE=69156864 fs_base=0 gs_base=18446744071564586640
[23083572798] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[23085425913] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650240 RFLAGS_BEFORE=130 CR3_BEFORE=69021696 fs_base=0 gs_base=18446744071564586608
[23089080135] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[23091490620] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[23092547082] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[23094328653] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[23102430120] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[23104404477] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[23105946138] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[23107659531] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[23110634745] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using cooperative polling loop (2ms interval)
[23113375527] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[23114928243] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[23116641603] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[23117384037] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23118895173] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23195367657] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[23222418384] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[23230371846] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[23233209450] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0102438
[23234697321] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23236789323] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369782992 RFLAGS_BEFORE=130 CR3_BEFORE=79892480 fs_base=0 gs_base=18446744071564586576
[23243524821] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[23244886434] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23246837988] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[23248277811] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23251193163] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[23258840187] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[23260819890] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[23279539965] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[23282889762] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[23290409208] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[23293683798] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[23296196154] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[23296931394] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23298796521] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23305236042] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[23307873006] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[23315495115] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[23318250879] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[23320721325] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23323251402] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915216 RFLAGS_BEFORE=134 CR3_BEFORE=80826368 fs_base=0 gs_base=18446744071564586640
[23328513054] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[23329803750] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23331555258] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[23332440747] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23333560107] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[23342059422] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[23345954577] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[23354618628] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[23357307369] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[23358436926] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23360042574] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981616 RFLAGS_BEFORE=130 CR3_BEFORE=80953344 fs_base=0 gs_base=18446744071564586576
[23363534832] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[23364328152] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23366515623] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23379186831] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[23383404396] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[23390964366] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[23393813091] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[23396055837] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[23396928522] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23398633632] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23426920110] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[23432583306] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[23441371899] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[23444039751] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010e370
[23445173697] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23446755486] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113808 RFLAGS_BEFORE=130 CR3_BEFORE=81256448 fs_base=0 gs_base=18446744071564586640
[23449975428] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[23450877912] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23452259523] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[23453337468] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23473408695] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[23476812579] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[23485433268] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[23486879856] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[23489811576] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[23491254798] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23493538761] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370197984 RFLAGS_BEFORE=130 CR3_BEFORE=81530880 fs_base=0 gs_base=18446744071564586576
[23497383426] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[23498344287] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23499809454] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[23500845060] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23501836017] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[23505851391] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[23507775753] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[23509073445] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23510659128] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[23512347969] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[23514280977] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23515865934] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[23518087263] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=226 subj_lo=0
[23532175689] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[23533417809] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23534844003] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[23536295475] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[23547428355] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[23560052274] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[23564279739] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[23565596010] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[23567039859] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23568680784] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[23571885678] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[23574887952] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[23579308962] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23581603551] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[23583139701] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849136 RFLAGS_BEFORE=134 CR3_BEFORE=80564224 fs_base=0 gs_base=18446744071564586608
[23587441119] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23589134283] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[23591034852] [INFO] [nectar] [CPU2] NECTAR: Started.
[23591997495] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[23594404614] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23598281652] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047824 RFLAGS_BEFORE=134 CR3_BEFORE=81100800 fs_base=0 gs_base=18446744071564586608
[23605811229] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[23608391499] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[23609586231] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010f930
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23611902039] [INFO] [fontd] [CPU3] FONTD: Service ready
[23612575140] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370266928 RFLAGS_BEFORE=134 CR3_BEFORE=81739776 fs_base=0 gs_base=18446744071564586608
[23619959154] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[23623505334] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[23625721614] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[23637826707] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[23639136741] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[23640355167] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23641783275] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=234 subj_lo=0
[23658827940] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[23659987956] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[23665171068] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[23677385952] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[23680750137] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[23682385650] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[23683812900] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23685211275] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=240 subj_lo=0
[23699304882] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[23700682302] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[23702018802] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23703512217] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[23712286224] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[23713514418] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[23714821812] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23716096074] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[23723889486] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[23726002575] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[23728113948] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[23729018346] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[23730193773] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[23731367253] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[23732855322] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[23733910134] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23735630952] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[23736735693] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[23739126906] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4ed4000
[23740768590] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[23743478946] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[23745702981] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[23747152242] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[23759797281] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[23771965734] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[23774931774] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[23784002088] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[23787309480] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[23790321654] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[23792344422] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[23794331715] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[23796456123] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[23798065269] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[23799630657] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[23800925445] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[23806455783] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[23812514814] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[23855735343] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[23862612345] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[23887621164] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1252 backend=VirtIO-GPU
[23890568922] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[23891586873] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23893061148] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[23894540274] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23904423609] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[23906576067] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[23917195599] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[23919326244] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[24047612523] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d6000 exec=false
[24063638511] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ec000 exec=false
[24072188976] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[24074950152] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db448
[24076592496] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e4
[24078628893] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370506224 RFLAGS_BEFORE=134 CR3_BEFORE=82771968 fs_base=0 gs_base=18446744071564586640
[24083151345] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[24084464580] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24086636178] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[24087738609] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24094831266] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[24097094835] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[24105417765] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[24108056214] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db448
[24109422876] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[24111200652] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370571760 RFLAGS_BEFORE=134 CR3_BEFORE=83828736 fs_base=0 gs_base=18446744071564586576
[24115726998] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[24118060428] [INFO] [echo] [CPU1] echo: starting up
[24121004919] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[24130220169] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[24131551455] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[24136419384] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[24147207546] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[24154753590] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[24161028771] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[24163952175] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[24186848037] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[24189679635] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[24196290459] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[24199643259] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[24204739350] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[24208047105] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[24219609810] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[24221530443] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[24229672335] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[24232444203] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[24234474693] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[24236043480] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[24237980976] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[24240604080] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[24241826763] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24244761651] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24251428674] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[24254429892] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[24261390945] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[24264045762] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[24267223200] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[24269257584] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[24270203001] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[24271803336] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[24273603420] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[24274619193] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24276419277] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24280864047] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[24285552423] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[24290067153] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[24299005566] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[24301708035] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[24303102450] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[24304916361] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[24305801124] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24307611999] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24311593350] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[24313601103] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[24320100255] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[24321606903] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[24323600565] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[24327049329] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[24327774537] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[24329486676] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[24332503998] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[24334411662] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/telnetd'
[24335774331] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[24337007145] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/telnetd
[24338564613] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0115d00
[24344900118] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24347978952] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24352670067] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370805232 RFLAGS_BEFORE=134 CR3_BEFORE=84500480 fs_base=0 gs_base=18446744071564586576
[24361471299] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[24378275955] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[24380334198] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f2510
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24382516884] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370739696 RFLAGS_BEFORE=134 CR3_BEFORE=84340736 fs_base=0 gs_base=18446744071564586640
[24388908687] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=220000 exec=false
[24396364410] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=228000 exec=false
[24405886956] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 32 (user task/process) assigned to CPU 2
[24409171578] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=32)
[24411296943] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[24422796915] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[24431125455] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f2510
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24432982992] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=32 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370878960 RFLAGS_BEFORE=134 CR3_BEFORE=84606976 fs_base=0 gs_base=18446744071564586608
[24439819635] [INFO] [telnetd] [CPU2] telnetd: starting on port 2323
[24446103792] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[24455980857] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[24464076252] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[24467017344] [INFO] [anther] [CPU1] anther: Connected to network stack
[24479544474] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[24482285487] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[24503611737] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[24533822775] [INFO] [telnetd::net_client] [CPU2] telnetd: connected to socket API (netd=27, write=31, read=32)
[24542945427] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[24586276341] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[24590036988] [INFO] [bloom] [CPU3] bloom: creating surface...
[24591668673] [INFO] [bloom] [CPU3] bloom: surface created!
[24593175552] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[24610344264] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[24621304488] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[24627437505] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[24637470396] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1264
[24639102774] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[24662467731] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[24666601806] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=33)
[24668306421] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
T:0AF0 [24676920807] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[24687331581] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[24698463339] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[24707361789] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
T:1050 T:0EC0 T:F930 [24725603166] [INFO] [bloom] [CPU3] [bloom] dynamically subscribed to input topic 0 on svc.Input 1240 via port 34
[24727728993] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 34 (legacy was 12)
[24755786715] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1264
[24765339291] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[24774826395] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 37 (user thread) assigned to CPU 3
[24778236549] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=37 (priority=2)
[24781899813] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[24784636107] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[24785532849] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
T:1AA0 [24800143269] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[24803757924] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[24817134045] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[24818861595] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[24831678168] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=270
[24833728161] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[24835112808] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[24836420301] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24837807324] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24839450988] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=270 subj_lo=0
[24850199517] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1268)
[24851998776] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[24864412650] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=244
[24865999884] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[24867379911] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[24868665756] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24870011133] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24871625394] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[24883372074] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1275)
[24885122691] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[24902274969] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=271
[24904040634] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[24905832171] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[24907429041] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24909181407] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24911304891] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=271 subj_lo=0
[24926508585] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1276)
[24928486275] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[25198701297] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=636 watches=13 history=1024 journal=1024 symbols=283 drops=0
[25313892516] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[25342566645] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[25709629704] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[25712117376] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[26566221660] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[27779901072] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[27782846157] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[27859526607] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 1394.181ms (rebuilds=0 pending=true)
[27946975749] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[27998824986] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[28000386975] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[28001876166] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28003371693] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=286 pred=0 subj_lo=0
[28022191362] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[28033795251] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 1551.314ms
[28035486270] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[28037056773] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[28251318249] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[28291633491] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=691 watches=14 history=1024 journal=1024 symbols=337 drops=0
[28307318424] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[28309060626] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[28310792961] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28312318254] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=287 pred=0 subj_lo=0
[28412057619] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[28461374997] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12001000
[28463007771] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[28464329124] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[28537319712] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[28549012998] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[28553974581] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[28556380413] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[28558845084] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[28564670112] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[28580188296] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[28581595350] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[28583385567] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[28599350736] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x12002000
[28600829169] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[28795863624] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 5)
[28854132582] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[28855406481] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[29250025299] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[29252830860] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[29255718558] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[29952557580] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[29971438629] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[29975616858] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[29977738857] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[30022472040] [INFO] [netd] [CPU3] NETD: DHCP configured �� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[30027165003] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[30031150611] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[30033366231] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=714 watches=15 history=1024 journal=1024 symbols=343 drops=0
[30035837700] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[30038204823] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 2323 with backlog 64
[30039204954] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[30042297516] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 2323, handle=2
[30045341304] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[30046827987] [INFO] [telnetd] [CPU2] telnetd: listening on guest port 2323 (handle=2)
[30048197685] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=3
[30051020307] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[30053551044] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=4
[30055589355] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[30057163851] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=5
[30066409923] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[30084007701] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[30087187449] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[30104139120] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 980.554ms (rebuilds=0 pending=true)
[30124399800] [
```
</details>
