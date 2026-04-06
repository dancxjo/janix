# ❌ Scenario: Font Explorer displays typography

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
[2J[01;01H[01;01H[2J[01;01H[01;01H[11903921733] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11909287632] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11912837475] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11914830312] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11916012141] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11916636204] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11917308249] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11917879248] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11918454999] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11919050748] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11919630591] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11920238154] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11920922541] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11921590494] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11922308145] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11922944022] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11923575939] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11924162349] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11924766876] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11925359424] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11925930786] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11926509144] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11927109678] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11927692557] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11928321834] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11928917616] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11929509108] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11930157921] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11930737038] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11931330378] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11931928602] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11932535274] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11933209992] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11933819271] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11934399444] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11935079211] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11935768647] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11936519430] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11937207414] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11937922392] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11938614930] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11939314299] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11940771645] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11942197608] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11942969544] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11943532689] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11944039701] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11944554666] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11945097945] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11945611194] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11946118536] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11946648153] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11947154307] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11947685805] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11948221032] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11948775036] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11949313530] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11949887466] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11950425036] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11950979271] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11951516181] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11952070053] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11952606237] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11953174134] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11953716324] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11954274156] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11954813607] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11955371208] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11955911352] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11956481757] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11957023683] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11957578908] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11958117930] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11958674310] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11959213365] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11959783242] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11960321967] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11960879799] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11961418986] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11961977148] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11962518084] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11963087994] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11963627577] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11964184551] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11964721956] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11965276488] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11965811583] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11966380668] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11966917512] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11967473331] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11968011165] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11968566786] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11969104224] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11969673210] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11970211506] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11970770460] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11971310340] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11971866357] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11972405940] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11972971725] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11973508536] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11974064619] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11974601694] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11975158602] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11975696271] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11976262848] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11976804213] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11977578459] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12213162060] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[12224584185] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12229448418] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12230752215] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12231618432] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12235982847] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12237701355] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12238872822] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12239592651] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12240276345] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12240965550] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12241950204] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12242963667] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12243656535] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12244333398] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12245006796] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12245680920] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12246878622] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12247902777] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12248605017] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12250218519] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12251262507] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12252264387] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12254022561] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12255649692] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12256473471] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12257012031] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12257907321] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12629689842] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12630715251] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12634004493] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12634934928] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12635716137] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12637233543] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12649476081] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12651155880] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12651949926] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12653740671] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12654285270] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12656721957] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12664339611] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12666238233] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12679622604] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12680261814] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12696455772] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12697137024] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12699296313] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12700702311] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12701927733] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12704290995] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12705159522] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12740067780] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62389500 ticks/sec), init_cnt=623895 for 100Hz
[12741649008] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12742483875] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12743655276] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12749572473] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12779898087] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12781100805] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12782037708] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12783476079] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12784594812] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12787792116] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12789690144] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12810869082] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12811896075] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12812856507] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12813599964] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12814324050] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12814975536] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12815698632] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12841625940] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12842753715] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12844380846] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12846834462] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12848265870] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12849515877] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12850614150] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12851471952] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12859744029] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12860951928] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12863229357] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12864073662] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12869289972] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12870878493] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12871786092] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12873144207] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12874106124] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12875129718] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12888623253] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12891755679] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12892909887] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12893698191] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12918429447] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12938225091] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12941160078] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12944806908] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12946735296] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12949208019] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12951815382] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12954061659] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12955285464] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12956909196] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12963653901] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12965882160] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12968871564] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12971503380] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12974141895] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12976859709] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12978027975] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12991231737] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12992288925] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12999418113] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13000191666] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13029111810] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13030019112] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13404481299] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[14002226568] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[14036181819] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[14074512441] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[15520471032] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=961 journal=769 symbols=95 drops=0
[16364556186] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[16476181194] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[16477695927] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[16594705479] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[16661401977] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[16692603741] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[16693487085] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[16694119860] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[16698937464] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[16717610712] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[16735062036] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[16737351048] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[16805168787] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[16837929240] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[16839189279] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[16844827329] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[16881151452] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[16900890666] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[16904158095] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[16905145785] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[16976947911] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[16978905141] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[17093726397] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[17176852770] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[17191107681] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[17195621124] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[17197893438] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[17214293415] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[17280000111] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[17288655087] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[17289695544] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[17290542654] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[17291445435] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[17292138930] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[17292813813] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[17293467180] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[17294074215] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[17294694252] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[17295353097] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[17296001283] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[17296644189] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[17297306829] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[17297998575] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[17298726324] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[17299397478] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[17300062098] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[17300704641] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[17301359064] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[17301989628] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[17302617816] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[17303237061] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[17303862972] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[17304489378] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[17305161159] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[17305814295] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[17306446971] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[17307134757] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[17307758820] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[17308398261] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[17309062716] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[17309714037] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[17310545538] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[17311213161] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[17311845573] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[17312616618] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[17313534117] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[17314432938] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[17315175174] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[17316368157] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[17317331922] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[17318193585] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[17320040232] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[17321855100] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[17322663897] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17332258779] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[17348744721] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[17354149758] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[17365017483] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[17366234061] [CONTRACT] [kernel] [CPU0] Spawning init process...
[17368907457] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[17372476902] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[17373351270] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [17378363013] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013376 RFLAGS_BEFORE=130 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[17381861277] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[17399358075] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[17402144232] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[17403480831] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[17404800567] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[17417118048] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[17423004291] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[17427041379] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[17428373919] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[17431981347] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[17435043549] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[17436178188] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[17441360871] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[17442589329] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[17443705851] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[17444903685] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[17448922161] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[17450293047] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[17451742605] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[17453296179] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[17454968883] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[17456580834] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[17463057744] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[17513453034] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[17522835000] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[17528774505] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[17533709358] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[17537984211] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[17543255268] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[17548545432] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[17556654654] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[17564264652] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[17571390210] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[17579296746] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[17585954529] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[17593706097] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[17599046718] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[17607026745] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[17614361457] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[17622014718] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[17628714312] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[17635621872] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[17642440893] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[17649192990] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[17656653795] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[17664208056] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[17670143568] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[17676852369] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[17684645748] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[17691046824] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[17696878980] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[17702702358] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[17708741589] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[17713064061] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[17717767980] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[17724507405] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[17730967122] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[17737054170] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[17744704164] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[17751856485] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[17759067216] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[17766356553] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[17774242431] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[17782199589] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[17787117018] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[17813834313] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[17975476893] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[17982962514] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[17983901298] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17985668976] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17990552184] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17991848127] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17999884386] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[18007046508] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18008507748] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18010682613] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078912 RFLAGS_BEFORE=130 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[18017639871] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[18018662805] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[18019558854] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18021386526] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18026075760] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18028311345] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18035868378] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[18038904873] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[18040653081] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[18041938266] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[18044180484] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144448 RFLAGS_BEFORE=130 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[18051630366] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[18053289441] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[18055360092] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18061053615] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:51:30 = 1775436690 unix_secs
[18062828421] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775436690, mono_ns=9031185388, offset=1775436680968814612ns
[18064607880] [INFO] [rtc_cmos] [CPU1] System clock anchored
[18077631462] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[18116528958] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[18127320453] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[18128236104] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18130196568] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18136175772] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18138614604] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[18147781608] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[18153402399] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[18157843473] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18159778395] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210560 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[18165992163] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[18171367203] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[18173796003] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[18175038651] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18177560742] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18187220733] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18190677648] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18198603225] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[18201238044] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18202199202] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18203578041] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277312 RFLAGS_BEFORE=130 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[18207645852] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[18209211999] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[18214673367] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[18216383493] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[18218604723] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[18220808364] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[18225085824] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[18226453707] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[18240446994] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[18242076996] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[18746407911] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18758636589] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[18761582235] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[18763471584] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[18769925130] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[18772020234] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[18774100125] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[18775533348] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[18776537571] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[18778783452] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[18779793483] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[18780847800] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[18781757511] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[18782704545] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[18783920232] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[18784970193] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[18796627773] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[18797885271] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18800072148] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18808536747] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18811446852] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18822549735] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[18828490956] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fbff0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[18830851479] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369357024 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[18857930685] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[18859555374] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
[18874494210] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[18891243723] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[18908467479] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[18910870407] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[18916349430] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[18925416675] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[18927122346] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[18934912161] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[18936397260] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[18937703103] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[18938768310] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[18939796953] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[18940810449] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[18942065472] [INFO] PC service loop
[18962128746] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[18966140457] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[18969085476] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[18970899750] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[18973493814] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[18974588952] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18977062302] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18990221580] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18993174816] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19005401745] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[19010815791] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[19014103746] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[19015473081] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19018218813] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19026411756] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19028672454] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19040959113] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[19045846380] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[19046913633] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[19048882710] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500144 RFLAGS_BEFORE=134 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[19062544017] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[19075179057] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[19077029697] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19079775924] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[19080834003] [INFO] [kernel::task::00000 exec=true
[19090148385] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19099740033] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[19102442568] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[19104666504] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19115653788] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[19122396909] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[19129178871] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[19142832720] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[19145619075] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[19147519050] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[19149345039] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[19154632959] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[19163188209] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[19166365482] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[19168124019] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[19171382043] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[19173683298] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=184467744071564586608
[19181325372] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[19512289599] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
[19513423578] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[19515256893] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369582928 RFLAGS_BEFORE=134 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[19518528249] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[19520494818] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[19521158976] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[19523234742] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[19526462571] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[19527462339] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19531137120] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[19532248230] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19534743228] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19540003956] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[19544375103] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[19547709687] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[19557994005] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[19561321494] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[19563529227] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[19564966179] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[19565681124] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19567247832] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19586497062] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19590974667] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[19611442719] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[19614476640] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[19616358234] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19618258374] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716656 RFLAGS_BEFORE=130 CR3_BEFORE=68882432 fs_base=0 gs_base=18446744071564586640
[19623965889] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[19624811778] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[19626607176] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650160 RFLAGS_BEFORE=134 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[19630088610] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[19632101610] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[19633108011] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[19634524371] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[19637326863] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[19638810378] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[19655955132] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[19657710567] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[19659108909] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[19660756500] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[19663777122] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[19665626310] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[19667408145] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[19668197802] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19669858527] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19735756260] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[19761714225] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[19769076492] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[19772269671] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[19774209708] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19775889804] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369782832 RFLAGS_BEFORE=134 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[19778898513] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[19779701502] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19782061860] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[19782866664] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19784621406] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[19788791385] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19790321034] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19808467470] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[19811723019] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[19819301238] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[19822288266] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[19824365550] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[19825125045] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19826586813] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19831980267] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19834298880] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19840967619] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[19843436250] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
[19844408925] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19846862838] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915152 RFLAGS_BEFORE=130 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[19850335461] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19851402813] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19853203392] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19856944767] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[19858361094] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[19861550775] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19865474805] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19872288348] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[19874829909] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[19876810503] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19878173865] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981696 RFLAGS_BEFORE=134 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[19881480828] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[19882319193] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19883970480] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19892751219] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19896624891] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[19903414476] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[19906164135] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[19908538122] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[19909315206] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19910872509] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19935129258] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[19939847862] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[19947616557] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[19950850887] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010d670
[19951807854] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19953063768] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113744 RFLAGS_BEFORE=130 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[19955950476] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[19956683340] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19957907904] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[19958787321] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19975634547] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[19976552508] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[19979485251] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19980302595] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19981778652] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19983429906] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19985524383] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[19988348919] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[19990982418] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[19992234075] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19993721319] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370198096 RFLAGS_BEFORE=134 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[19997831337] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[19998603339] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20000417811] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20001311517] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[20003263500] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[20013927120] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20015644473] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20017268898] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20018685291] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20020255398] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20021716275] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[20023544673] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20025197643] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[20038259241] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[20049494586] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[20056802931] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[20059664427] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[20063628123] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20065239876] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849072 RFLAGS_BEFORE=134 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[20068301022] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20069959536] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20071372728] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20072899110] [INFO] [nectar] [CPU2] NECTAR: Started.
[20073448131] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20074923759] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20076428856] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[20078201880] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20079747501] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
[20080764825] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20083728390] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047952 RFLAGS_BEFORE=130 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[20088881373] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[20091696504] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
[20092735047] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20094501207] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370266832 RFLAGS_BEFORE=130 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[20098561032] [INFO] [fontd] [CPU3] FONTD: Service ready
[20100266670] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[20101925052] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[20103380781] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[20116451883] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[20196396594] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[20201713521] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[20218329945] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20219516064] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20220881868] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20222257770] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=234 subj_lo=0
[20225858136] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1251 backend=VirtIO-GPU
[20228683926] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[20229703296] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20231928651] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20233954455] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20235463941] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20241124761] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20242292631] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20243618175] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20244991998] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[20252285889] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20253573483] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20254800159] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20256154413] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[20261561628] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20262935979] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20264453088] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20266145856] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[20274961377] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20277028233] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20278405983] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20279809836] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[20301965343] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[20315940678] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[20317376904] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[20389266051] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[20404880430] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[20412912003] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[20416000572] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[20419106895] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db608
[20420344329] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[20421160287] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e3
[20423620932] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20424723660] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370358656 RFLAGS_BEFORE=130 CR3_BEFORE=72052736 fs_base=0 gs_base=18446744071564586640
[20429654982] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[20431614786] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20433412527] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[20442624444] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[20445768981] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db608
[20446968102] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[20448584706] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[20449441617] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370424192 RFLAGS_BEFORE=130 CR3_BEFORE=73101312 fs_base=0 gs_base=18446744071564586576
[20455630404] [INFO] [echo] [CPU1] echo: starting up
[20458059435] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[20468828523] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[20477361630] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[20479228440] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[20484212529] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[20484931533] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[20485765443] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[20486832960] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f5000
[20488290372] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[20489053728] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f5000
[20489854110] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[20491265025] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f8000
[20493530409] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276787200)
[20494753653] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[20496781437] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f9000 phys=0x4656000
[20497916142] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[20500154532] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[20502480603] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[20503926861] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[20514397134] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[20517381786] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[20519227872] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[20523177642] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[20524035378] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[20532837336] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[20534034906] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[20536097307] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[20537273196] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[20538297417] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[20551125441] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[20553033699] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[20554684557] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[20555430588] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20557019043] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20562009930] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20562813117] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[20564087247] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[20564986266] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20567158326] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20571905574] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[20573901249] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[20576890191] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[20578596291] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[20580134520] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[20581641498] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[20582330934] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20583951003] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20591357160] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20595289143] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[20602310949] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[20605091958] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[20606427303] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[20608128321] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[20608924578] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20610436539] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20613236754] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[20614474683] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[20621484939] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[20622477711] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[20623628751] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[20625980331] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[20627054679] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[20628512190] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[20629576572] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0112d48
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20631884097] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370641936 RFLAGS_BEFORE=130 CR3_BEFORE=74145792 fs_base=0 gs_base=18446744071564586576
[20636934945] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb01111e0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20638823205] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370576400 RFLAGS_BEFORE=130 CR3_BEFORE=73986048 fs_base=0 gs_base=18446744071564586640
[20642339850] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[20643964176] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[20645668725] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[20647279389] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[20648739771] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[20650345815] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[20651535795] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[20655837510] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[20661903438] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[20663398206] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[20666907195] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[20674248705] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20684682777] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[20686058877] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[20689693068] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20702657844] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[20704480302] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[20719109268] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20720746893] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20730945576] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[20733012168] [INFO] [bloom] [CPU3] bloom: creating surface...
[20734137369] [INFO] [bloom] [CPU3] bloom: surface created!
[20734961214] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[20742462543] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[20747823690] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1261
[20748903549] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[20763308577] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[20766289665] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[20767517826] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[20768833536] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
[20774294706] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[20776950051] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[20780817486] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
T:0270 [20786725839] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[20795202714] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[20804443473] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[20813281962] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[20823173283] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
T:07D0 T:0640 T:F0B0 [20868110505] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[20873010345] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[20877940941] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[20880631530] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[20883814941] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[20886722340] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[20901429714] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1261
T:1220 [20903966523] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[20905614411] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[20920252749] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[20922461835] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[20927147835] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([239, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[20940282924] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=276
[20941454292] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[20942618235] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20943639057] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20944759869] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20945927376] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=276 subj_lo=0
[20963103678] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1272)
[20965076022] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[20980410330] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=244
[20982251235] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[20983613310] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20984641029] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20985849159] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20987257434] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[20993056689] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[20995129386] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[21001073709] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[21010112706] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=31, our_read=32)
[21011790327] [INFO] [anther] [CPU1] anther: Connected to network stack
[21033454926] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[21035178747] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[21042410895] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1275)
[21043568865] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[21044465838] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[21061961250] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=281
[21063316065] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[21064274616] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[21065229801] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21066228447] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21067463637] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=281 subj_lo=0
[21075773136] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[21079164612] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[21081371421] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1278)
[21083444943] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[21098197692] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[21105054993] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[21109101288] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[21114998058] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[21118717125] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[21120862917] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[21124591587] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[21149875131] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[21154000197] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[21155980626] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[21195166971] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[21199651638] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[21205100037] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[21207510357] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=1
[21211887609] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[21215610504] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=2
[21234750372] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=2)
[21242655324] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[21245107752] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[21247203978] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[21344464647] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[21450609345] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[21477693402] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[21520850109] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[21653935644] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=663 watches=13 history=1024 journal=1024 symbols=313 drops=0
[21675860316] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[21815217963] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[21973828899] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[22203234537] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[22368213021] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[22530247674] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[22565752803] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[22727277903] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[22913567391] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[22951414629] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[22957249524] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[22958874906] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[22959995883] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[22961179164] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[22962727524] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[22972771998] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[22973919144] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[22975187301] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[22976770212] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=337 pred=0 subj_lo=0
[23100529551] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[23199798072] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=710 watches=15 history=1024 journal=1024 symbols=339 drops=0
[23305871292] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[23470934388] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[23576869305] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[23578506897] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[23684578038] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[23752512729] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23753726073] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23754914337] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23756183385] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=339 pred=0 subj_lo=0
[23875190526] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[24037282170] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=fec1d29ef0678173)
[24098358570] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[24120152793] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[24125671383] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[24126853113] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24127900467] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24129204429] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=340 pred=0 subj_lo=0
[24139506534] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[24141342555] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[24157933701] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[24299818686] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[24316946478] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x1222d000
[24318189885] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[24319544271] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[24358205817] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[24387393591] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[24398292534] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[24402748557] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[24405129210] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[24407744064] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[24412531473] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[24433368267] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[24435323715] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[24437240619] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[24452682210] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x1222e000
[24454228557] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[24665213166] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=14aa2015f9906e3f)
[24930585471] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[25145785995] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[25325736084] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=795 watches=17 history=1024 journal=1024 symbols=353 drops=0
[25382578683] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[25628481747] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[25913641809] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[26175914754] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[26358044229] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[26450128584] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[26482255767] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26483384466] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26484528246] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26485777725] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=341 pred=0 subj_lo=0
[26511943887] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[26558576484] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26559746235] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26560896351] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26561987793] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=342 pred=0 subj_lo=0
[26812340907] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[27117062280] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[27203963787] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=849 watches=19 history=1024 journal=1024 symbols=365 drops=0
[27411541146] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[27704632758] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[28022675010] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[28401224115] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[28850330883] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=888 watches=19 history=1024 journal=1024 symbols=366 drops=0
[29245454337] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2429 ops=1 watches=19
[29312521854] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[29829904071] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[30489827379] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[30505634511] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=915 watches=19 history=1024 journal=1024 symbols=366 drops=0
[30883125042] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[31123793976] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[31247291526] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[31565721297] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[32222442054] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=963 watches=19 history=1024 journal=1024 symbols=386 drops=0
[34420922151] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[34424140245] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1054 watches=19 history=1024 journal=1024 symbols=451 drops=0
[34531819938] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[34828790172] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[34830453207] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[34858133640] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[35322722028] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[35370085113] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35371719240] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35373414285] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35374707918] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[35384643195] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35385990717] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35387197956] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35388610851] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=235 pred=0 subj_lo=0
[35395878144] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35397103467] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35398620576] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35400361689] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=453 pred=0 subj_lo=0
[35407584960] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35408787183] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35410148103] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35411637558] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=452 pred=0 subj_lo=0
[35422588509] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35423770338] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35425075356] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35426790267] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[35435978919] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[36166178862] [INFO] [kernel::roo
```
</details>
