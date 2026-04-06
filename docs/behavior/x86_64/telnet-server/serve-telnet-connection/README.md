# ❌ Scenario: Serve telnet connection

> Last run: 2026-04-05 19:04:08

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the telnet server is ready | ✅ | 7093ms | - - - |
| 2 | When I connect to the telnet server and send "match (n) return n;" | ✅ | 2084ms | - [📜](./02/serial.log) - |
| 3 | Then the telnet response should contain "node(" | ❌ | 1014ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12554602434] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[12560969619] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[12564743961] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[12566866488] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[12568071615] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[12568705710] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[12569375412] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[12569981424] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[12570572025] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=29168
[12571183713] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=33264
[12571776855] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[12572379633] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[12573076230] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[12573959970] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[12574650198] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[12575260467] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[12575882748] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[12576566904] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[12577194993] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[12577790742] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[12578370321] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[12578955972] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[12579551028] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[12580164366] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[12580800276] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[12581405232] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[12582006327] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[12582655305] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[12583266894] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[12583919733] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[12584544060] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[12585203037] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[12585951741] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[12586702590] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=172560
[12587352294] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[12588091593] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[12588799740] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[12589552173] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[12590277348] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[12591007275] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[12591720306] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[12592434921] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[12593829996] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[12595358325] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[12596145870] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[12596718090] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[12597235200] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[12597760263] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[12598316148] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[12598841277] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[12599360961] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[12599911269] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[12600428181] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[12600972780] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7795e000 (Usable)
[12601522989] [INFO] [kernel::memory] [CPU0]   [11] 0x7795e000 - 0x779c2000 (Reserved)
[12602091117] [INFO] [kernel::memory] [CPU0]   [12] 0x779c2000 - 0x779c3000 (Other)
[12602636937] [INFO] [kernel::memory] [CPU0]   [13] 0x779c3000 - 0x779c4000 (Reserved)
[12603220542] [INFO] [kernel::memory] [CPU0]   [14] 0x779c4000 - 0x779c5000 (Other)
[12603814047] [INFO] [kernel::memory] [CPU0]   [15] 0x779c5000 - 0x779c6000 (Reserved)
[12604384287] [INFO] [kernel::memory] [CPU0]   [16] 0x779c6000 - 0x779c7000 (Other)
[12604935519] [INFO] [kernel::memory] [CPU0]   [17] 0x779c7000 - 0x779c8000 (Reserved)
[12605503581] [INFO] [kernel::memory] [CPU0]   [18] 0x779c8000 - 0x77a53000 (Other)
[12606051579] [INFO] [kernel::memory] [CPU0]   [19] 0x77a53000 - 0x77a54000 (Reserved)
[12606637593] [INFO] [kernel::memory] [CPU0]   [20] 0x77a54000 - 0x77ed5000 (Other)
[12607185921] [INFO] [kernel::memory] [CPU0]   [21] 0x77ed5000 - 0x77ed6000 (Reserved)
[12607753620] [INFO] [kernel::memory] [CPU0]   [22] 0x77ed6000 - 0x77ff7000 (Other)
[12608303862] [INFO] [kernel::memory] [CPU0]   [23] 0x77ff7000 - 0x77ff8000 (Reserved)
[12608871000] [INFO] [kernel::memory] [CPU0]   [24] 0x77ff8000 - 0x787f8000 (Other)
[12609439458] [INFO] [kernel::memory] [CPU0]   [25] 0x787f8000 - 0x787f9000 (Reserved)
[12610011711] [INFO] [kernel::memory] [CPU0]   [26] 0x787f9000 - 0x788ba000 (Other)
[12610591059] [INFO] [kernel::memory] [CPU0]   [27] 0x788ba000 - 0x788bb000 (Reserved)
[12611161596] [INFO] [kernel::memory] [CPU0]   [28] 0x788bb000 - 0x788e6000 (Other)
[12611762889] [INFO] [kernel::memory] [CPU0]   [29] 0x788e6000 - 0x788e7000 (Reserved)
[12612538059] [INFO] [kernel::memory] [CPU0]   [30] 0x788e7000 - 0x788ec000 (Other)
[12613351080] [INFO] [kernel::memory] [CPU0]   [31] 0x788ec000 - 0x788ed000 (Reserved)
[12614178291] [INFO] [kernel::memory] [CPU0]   [32] 0x788ed000 - 0x788f2000 (Other)
[12614933364] [INFO] [kernel::memory] [CPU0]   [33] 0x788f2000 - 0x788f3000 (Reserved)
[12615562575] [INFO] [kernel::memory] [CPU0]   [34] 0x788f3000 - 0x7891f000 (Other)
[12616216866] [INFO] [kernel::memory] [CPU0]   [35] 0x7891f000 - 0x78921000 (Reserved)
[12616790736] [INFO] [kernel::memory] [CPU0]   [36] 0x78921000 - 0x7892a000 (Other)
[12617345961] [INFO] [kernel::memory] [CPU0]   [37] 0x7892a000 - 0x7892c000 (Reserved)
[12617918049] [INFO] [kernel::memory] [CPU0]   [38] 0x7892c000 - 0x78934000 (Other)
[12618469743] [INFO] [kernel::memory] [CPU0]   [39] 0x78934000 - 0x78935000 (Reserved)
[12619039257] [INFO] [kernel::memory] [CPU0]   [40] 0x78935000 - 0x7893f000 (Other)
[12619612203] [INFO] [kernel::memory] [CPU0]   [41] 0x7893f000 - 0x78940000 (Reserved)
[12620183400] [INFO] [kernel::memory] [CPU0]   [42] 0x78940000 - 0x7894d000 (Other)
[12620734929] [INFO] [kernel::memory] [CPU0]   [43] 0x7894d000 - 0x7894f000 (Reserved)
[12621304047] [INFO] [kernel::memory] [CPU0]   [44] 0x7894f000 - 0x7895d000 (Other)
[12621856665] [INFO] [kernel::memory] [CPU0]   [45] 0x7895d000 - 0x7895e000 (Reserved)
[12622426971] [INFO] [kernel::memory] [CPU0]   [46] 0x7895e000 - 0x7896a000 (Other)
[12622992327] [INFO] [kernel::memory] [CPU0]   [47] 0x7896a000 - 0x7896b000 (Reserved)
[12623563458] [INFO] [kernel::memory] [CPU0]   [48] 0x7896b000 - 0x7896f000 (Other)
[12624116472] [INFO] [kernel::memory] [CPU0]   [49] 0x7896f000 - 0x78970000 (Reserved)
[12624684996] [INFO] [kernel::memory] [CPU0]   [50] 0x78970000 - 0x78980000 (Other)
[12625236096] [INFO] [kernel::memory] [CPU0]   [51] 0x78980000 - 0x78981000 (Reserved)
[12625804455] [INFO] [kernel::memory] [CPU0]   [52] 0x78981000 - 0x78a11000 (Other)
[12626372220] [INFO] [kernel::memory] [CPU0]   [53] 0x78a11000 - 0x78a12000 (Reserved)
[12626941503] [INFO] [kernel::memory] [CPU0]   [54] 0x78a12000 - 0x78a1d000 (Other)
[12627492801] [INFO] [kernel::memory] [CPU0]   [55] 0x78a1d000 - 0x78a1e000 (Reserved)
[12628063833] [INFO] [kernel::memory] [CPU0]   [56] 0x78a1e000 - 0x78a43000 (Other)
[12628614603] [INFO] [kernel::memory] [CPU0]   [57] 0x78a43000 - 0x78a44000 (Reserved)
[12629200386] [INFO] [kernel::memory] [CPU0]   [58] 0x78a44000 - 0x78a50000 (Other)
[12629784387] [INFO] [kernel::memory] [CPU0]   [59] 0x78a50000 - 0x78a51000 (Reserved)
[12630358653] [INFO] [kernel::memory] [CPU0]   [60] 0x78a51000 - 0x78a5e000 (Other)
[12631045416] [INFO] [kernel::memory] [CPU0]   [61] 0x78a5e000 - 0x78a5f000 (Reserved)
[12631644729] [INFO] [kernel::memory] [CPU0]   [62] 0x78a5f000 - 0x78aa7000 (Other)
[12632199690] [INFO] [kernel::memory] [CPU0]   [63] 0x78aa7000 - 0x78aa8000 (Reserved)
[12633140949] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12900326736] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485750 free frames
[12914222970] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12920773206] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12922588041] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12923807424] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12928781250] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12931155996] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12932644593] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12933758112] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12934866186] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12936029007] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12937094643] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12938138598] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12938975643] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12939797904] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12940559742] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12941545452] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12943538289] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12945069786] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12946240989] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12948411762] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12949772121] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12951181188] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12953434461] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12955505178] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12956642556] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12957544380] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12958449075] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[13411605702] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[13412666718] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[13417154190] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[13418759607] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[13420168674] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[13421904507] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[13436832915] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[13438578615] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[13439896239] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[13442227788] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[13442886402] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[13446165810] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[13455208404] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[13457171772] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[13472524593] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[13473201687] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[13490302551] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[13491050298] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[13493334459] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[13494860214] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[13496177970] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[13498917498] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[13499906112] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[13535224758] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62418200 ticks/sec), init_cnt=624182 for 100Hz
[13537040088] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[13537932078] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[13539152946] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[13545076248] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[13576193268] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[13577473866] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[13578191583] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[13579485216] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[13580588010] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[13583379282] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[13584697566] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[13607660154] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[13608809973] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[13609810599] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[13610575770] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[13611490398] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[13612082583] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[13612783569] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[13638940755] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[13640919369] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[13641793935] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[13642667082] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[13643401299] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[13645175412] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[13646005791] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[13646723244] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[13653579357] [INFO] [kernel::root] [CPU0] Spawning Root service...
[13655075610] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[13657553877] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[13658550675] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[13663727517] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[13665350523] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[13666185720] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[13667608119] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[13668663492] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[13669550235] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[13682829963] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[13688182464] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[13689843090] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[13690735443] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13733099061] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13769772621] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13777516665] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13783770990] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13786555497] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13790482002] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13793941557] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13797159387] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13797960561] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13799389329] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13807200429] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13810066281] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13814433006] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13818875136] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13825335381] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13830717582] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13831742694] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13854296214] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13855229421] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13864991514] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13866238419] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13908550194] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13909689090] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[14322477081] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[14831591973] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[14864253360] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=499 journal=424 symbols=51 drops=0
[14900882898] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[16360495404] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=963 journal=771 symbols=97 drops=0
[17144033181] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[17243999025] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[17245098816] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[17346639981] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[17426905980] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[17457986139] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[17461709133] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[17462358078] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[17468384307] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[17488731084] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[17511023376] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[17513551143] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[17599817664] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[17636879766] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[17638076313] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[17643061326] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[17675563389] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[17695931946] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[17699164626] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[17700143340] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[17765794530] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[17768115123] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[17854565751] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[17924200239] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[17935788882] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[17939516958] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[17941631565] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[17965880064] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[18011487021] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[18017096196] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[18018181863] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[18019331154] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[18020239083] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[18020963400] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[18021637557] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[18022287261] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[18022949736] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[18023575317] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=29168
[18024218652] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=33264
[18024865551] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[18025503342] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[18026158821] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[18026843967] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[18027565545] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[18028263000] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[18028920459] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[18029543961] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[18030188352] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[18030808686] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[18031428030] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[18032035659] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[18032652660] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[18033295368] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[18033958305] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[18034608405] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[18035232237] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[18035906922] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[18036523956] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[18037162242] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[18038036214] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[18038903520] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[18039563916] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[18040209693] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=172560
[18040837749] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[18041597277] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[18042411222] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[18043330899] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[18044352150] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[18045184443] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[18045927438] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[18046673106] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[18048539025] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[18050618124] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[18051437382] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18059694708] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[18076143096] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[18080649609] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[18090176115] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[18090936204] [CONTRACT] [kernel] [CPU0] Spawning init process...
[18093247623] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[18095914320] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[18096403116] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [18101804853] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[18102585963] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013216 RFLAGS_BEFORE=130 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[18120945381] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[18126586962] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[18127831029] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[18128940687] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[18138889758] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[18144314925] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[18148546581] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[18149673168] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[18153856545] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[18157168920] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[18158485719] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[18162664278] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[18163790568] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[18164824557] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[18165894714] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[18169726476] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[18170833362] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[18172003047] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[18173546457] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[18174899589] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[18176605293] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[18183772464] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[18228656655] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[18237444489] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[18244072770] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[18250084413] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[18253460511] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[18257619270] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[18263603160] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[18270044859] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[18275359311] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[18281179422] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[18287354580] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[18294188319] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[18300658794] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[18305893188] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[18311411019] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[18317800776] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[18324056982] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[18329480565] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[18334688988] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[18339912195] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[18347373396] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[18353997882] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[18359700810] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[18364865739] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[18372112539] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[18379628685] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[18385354548] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[18391192776] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[18396945699] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[18403244013] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[18407981526] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[18412106163] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[18417190935] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[18421942605] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/telnetd
[18426159576] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[18432341367] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[18438976281] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[18445935420] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[18453158262] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[18460166967] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[18466846299] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[18473859360] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[18477994590] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[18502522005] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[18655305438] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[18662922432] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[18663880620] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18665811747] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18670871274] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18672642978] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18683246571] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[18689509674] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18690776478] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18692621079] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078752 RFLAGS_BEFORE=130 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[18698602032] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[18703155570] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[18704243811] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18707252751] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18712752465] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18714583602] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18721602801] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[18725480862] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[18726847326] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[18728835543] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144288 RFLAGS_BEFORE=130 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[18732183624] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[18736542990] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[18738310734] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[18740651424] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18745886841] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 02:04:41 = 1775441081 unix_secs
[18747981450] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775441081, mono_ns=9373712535, offset=1775441071626287465ns
[18749607426] [INFO] [rtc_cmos] [CPU1] System clock anchored
[18762931902] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[18800561868] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[18813252480] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[18814546443] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18817421337] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18825571908] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18830081820] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[18865225401] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[18869930805] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[18899098515] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18903600837] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210656 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[18915939372] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[18927243951] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[18929758584] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[18931028391] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18933248697] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18941863974] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18944819190] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18954770010] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[18960302790] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[18961460397] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18964226556] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277408 RFLAGS_BEFORE=130 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[18968553912] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[18971445999] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[18977827341] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[18984431037] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[18986724075] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[18988514589] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[18996190818] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[18998845371] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[19022354010] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[19024484193] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[19500244896] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19507464405] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[19510330191] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[19511923992] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[19517135814] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[19518657444] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[19520933883] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[19523169732] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[19524195108] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[19526845140] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[19528593645] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[19529862825] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[19531092471] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[19532177016] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[19533267303] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[19534547010] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[19559916453] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[19561140093] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19563025053] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19570273899] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19572597528] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19580344443] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[19583375328] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[19584745785] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[19586714730] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369356912 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[19594101846] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[19610462058] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[19614520101] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[19626244506] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[19629133953] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[19635015510] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[19651110864] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[19656916290] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[19662102108] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[19664157711] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[19665857904] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[19667172327] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[19668569844] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[19669537833] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[19670513940] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[19671646863] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[19676673786] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[20170661214] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[20183700471] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[20187425841] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[20189534871] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[20190639843] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20193279843] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20199300033] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[20202148593] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20214073935] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[20218928235] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[20222322219] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[20223605721] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20226060195] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20233883868] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20235854925] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20241073677] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[20243484624] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[20245890918] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0108b48
[20246826336] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[20248213953] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500528 RFLAGS_BEFORE=130 CR3_BEFORE=68669440 fs_base=0 gs_base=18446744071564586640
[20251505208] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[20252251272] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20254123923] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20254904802] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[20256393300] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[20259718314] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[20260646967] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20261563179] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[20263651089] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[20264504799] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20265960990] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[20267941584] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[20269644615] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[20271556305] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[20273123112] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[20274793506] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[20276628603] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[20281428519] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[20282536065] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0108b48
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[20284634403] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583584 RFLAGS_BEFORE=130 CR3_BEFORE=68898816 fs_base=0 gs_base=18446744071564586576
[20287989117] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[20290392210] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[20292975318] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[20297235750] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[20301412527] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0103240
[20306707905] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1240)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[20312768553] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434736 RFLAGS_BEFORE=130 CR3_BEFORE=68403200 fs_base=0 gs_base=18446744071564586608
[20319665586] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[20323477713] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[20357409732] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1241
[20359429728] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[20361220110] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[20372426844] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20374879767] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[20375769744] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20377608438] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20385015090] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[20387509560] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[20395105929] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[20398262643] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[20401734870] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[20403968277] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[20404977549] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20406994113] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20434045995] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20438894454] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[21173443434] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[21174374100] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[21176047893] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[21177766863] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[21297225411] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[21301850757] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[21303119013] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
[21309364197] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering interrupt-driven loop
[21323903403] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0103240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[21326077905] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653776 RFLAGS_BEFORE=130 CR3_BEFORE=69025792 fs_base=0 gs_base=18446744071564586608
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21332675265] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369719312 RFLAGS_BEFORE=130 CR3_BEFORE=69160960 fs_base=0 gs_base=18446744071564586640
[21339496761] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[21341814054] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[21342917211] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[21344323539] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[21350840016] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[21352893804] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[21354476880] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[21357043620] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[21369973680] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[21372118911] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[21373923483] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[21374710995] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21376238895] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21449845923] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[21474868734] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[21482714715] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[21485886840] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0103240
[21487573602] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21489156546] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369784848 RFLAGS_BEFORE=130 CR3_BEFORE=79896576 fs_base=0 gs_base=18446744071564586576
[21492616794] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[21493491591] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21495224058] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[21496505745] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21498931443] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[21504085977] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21505611600] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21524038305] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[21527350350] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[21534858543] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[21537850752] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[21540026805] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[21540752706] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21542192760] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21547766526] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[21550455333] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21558051207] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[21560439582] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
[21561443937] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cc18
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21563482083] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915920 RFLAGS_BEFORE=130 CR3_BEFORE=80830464 fs_base=0 gs_base=18446744071564586640
[21566538114] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[21567332919] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21568822209] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[21569539068] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21570282756] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[21581535195] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21585640098] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[21592820601] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[21595545246] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010cc18
[21596503566] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21598448454] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981776 RFLAGS_BEFORE=134 CR3_BEFORE=80957440 fs_base=0 gs_base=18446744071564586576
[21603339351] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[21604374462] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21606753564] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21619606470] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[21622994283] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[21630204453] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[21632792346] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[21634999782] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[21635704332] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21637106040] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21662613687] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[21667546098] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[21675065346] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[21678360330] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
[21679920735] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010e370
[21680992443] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[21681791373] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21683646567] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21684485394] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113760 RFLAGS_BEFORE=130 CR3_BEFORE=81260544 fs_base=0 gs_base=18446744071564586640
[21690669132] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[21701799273] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[21705202662] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[21713039832] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[21714919248] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[21716977689] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[21718063125] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21719616006] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370198080 RFLAGS_BEFORE=130 CR3_BEFORE=81534976 fs_base=0 gs_base=18446744071564586576
[21724373055] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[21725352165] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21726848418] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[21727819707] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21728897190] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[21732145743] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21734378358] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21736074525] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21737266056] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21739082178] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21740382345] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21741755013] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[21743935257] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=226 subj_lo=0
[21758984346] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21760243758] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21761509704] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21762898080] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[21768757362] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[21780292413] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[21788860566] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[21792005499] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[21796027869] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0103240
[21797364303] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21799413735] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21801073305] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369850384 RFLAGS_BEFORE=130 CR3_BEFORE=80568320 fs_base=0 gs_base=18446744071564586608
[21804727659] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21806302221] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[21807960504] [INFO] [nectar] [CPU2] NECTAR: Started.
[21810767484] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010cc18
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21812450649] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047872 RFLAGS_BEFORE=130 CR3_BEFORE=81104896 fs_base=0 gs_base=18446744071564586608
[21816840573] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21818448894] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21820060779] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[21820963560] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21822762093] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[21825247983] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f10d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21827603193] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370266960 RFLAGS_BEFORE=134 CR3_BEFORE=81743872 fs_base=0 gs_base=18446744071564586608
[21834964470] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[21837520155] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[21839004693] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[21840800916] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[21841831374] [INFO] [fontd] [CPU3] FONTD: Service ready
[21849396558] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[21865965165] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[21869157684] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[21870769338] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[21872405610] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[21874177743] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[21877055673] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[21878621985] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[21881277165] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4e94000
[21882782757] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[21885111699] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[21887213700] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21888254553] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[21890047773] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21891137004] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[21898024863] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21899447262] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21901026345] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21902597409] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=239 subj_lo=0
[21904442769] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[21914593437] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[21917098467] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21918675702] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21920109552] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21921604089] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=240 subj_lo=0
[21926624049] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[21929329059] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[21932495937] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[21933561045] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21935138115] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[21936058551] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21937385283] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[21938671491] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21940088841] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[21941475501] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[21943147116] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[21944920206] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[21947011812] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[21953058369] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21954178587] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21955160799] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=19, read=20)
[21956026092] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21957434829] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[21961107267] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=21, read=22)
[21971170320] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21980674419] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21982009434] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21983323065] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21984703422] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=245 pred=0 subj_lo=0
[22001005983] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22004777718] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[22019395398] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1251 backend=VirtIO-GPU
[22021321377] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[22022185416] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22023964578] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22034512533] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22036441680] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22054251945] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[22070113395] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22073574072] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=23, resp=26
[22074878463] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[22168667037] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[22183017549] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[22190478354] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[22193388063] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[22194130563] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f0fe8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e3
[22196387664] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370505136 RFLAGS_BEFORE=130 CR3_BEFORE=82776064 fs_base=0 gs_base=18446744071564586640
[22200489729] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[22201205268] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22202401353] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[22203269352] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22207997559] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22210514205] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[22218107439] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[22220382129] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[22221087042] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f0fe8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[22223308107] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370570672 RFLAGS_BEFORE=130 CR3_BEFORE=83824640 fs_base=0 gs_base=18446744071564586576
[22226764593] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[22228686843] [INFO] [echo] [CPU1] echo: starting up
[22231176363] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[22232071455] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[22244137245] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[22246293498] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[22247099160] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=19, RX port=22
[22249777770] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[22250785623] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[22259017737] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[22260650841] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[22264959585] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[22267587177] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[22269350829] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[22270990764] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[22271755770] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22273315680] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22278852024] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22280848524] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[22282836444] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22288142085] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[22290472644] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[22292885274] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[22294406706] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[22295430300] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[22296602328] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[22297491183] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[22298548899] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22299969516] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[22301324331] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22308741906] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22309645875] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22310944161] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22313262708] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[22319897028] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[22321188945] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[22323318567] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[22325142444] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[22327597974] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[22328909658] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22331076372] [INFO] [netd] [CPU3] NETD: Driver TX port=19, RX port=22, link_up=true, mtu=1500
[22332006939] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22334838669] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[22335861042] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[22337645187] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[22338866352] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[22340906346] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0112d48
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22342530903] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370709936 RFLAGS_BEFORE=130 CR3_BEFORE=84332544 fs_base=0 gs_base=18446744071564586640
[22348738797] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[22350031275] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[22352920392] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[22354557951] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/telnetd'
[22356387603] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/telnetd
[22357091064] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22358653845] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22363149864] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f2a78
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22364563914] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370808240 RFLAGS_BEFORE=130 CR3_BEFORE=84480000 fs_base=0 gs_base=18446744071564586576
[22369462962] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[22383762324] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=221000 exec=false
[22390219830] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[22391530986] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=229000 exec=false
[22394129307] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[22395284703] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[22401736665] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 32 (user task/process) assigned to CPU 2
[22404626871] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=32)
[22406255817] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[22415843967] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db548
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22417816377] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=32 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370873776 RFLAGS_BEFORE=130 CR3_BEFORE=84582400 fs_base=0 gs_base=18446744071564586608
[22437915720] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[22441686168] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[22450810602] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[22455714534] [INFO] [telnetd] [CPU2] telnetd: starting on port 2323
[22467827580] [INFO] [telnetd::net_client] [CPU2] telnetd: connected to socket API (netd=27, write=29, read=30)
[22484401632] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[22487150202] [INFO] [bloom] [CPU3] bloom: creating surface...
[22488581841] [INFO] [bloom] [CPU3] bloom: surface created!
[22489757763] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[22493488116] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[22495331232] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[22501682148] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[22537103655] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[22539697422] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[22542727383] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[22555274511] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1264
[22556386710] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[22579918812] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[22584898578] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=33)
[22586088327] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[22587659259] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[22589528676] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
[22614743679] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1264
T:08C0 [22621299723] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[22626063603] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[22629076008] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[22631058186] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=31, our_read=32)
[22633527477] [INFO] [anther] [CPU1] anther: Connected to network stack
[22637722008] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[22651500300] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[22666250871] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[22674074775] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=77c80b059e863710)
[22683152085] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[22690560255] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[22693440000] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
T:0E20 T:0C90 T:F700 [22702395507] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[22704981222] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[22708436091] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[22716209934] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[22729835106] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[22733587965] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[22735434183] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[22740545850] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[22750491126] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 37 (user thread) assigned to CPU 3
[22753147032] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=37 (priority=2)
[22763045613] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[22765778739] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
T:1870 [22776538488] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[22779137931] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[22789493892] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[22791648825] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[22811838291] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=279
[22813539276] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[22814737902] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22816003551] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22817415786] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22818684702] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=279 subj_lo=0
[22833198894] [INFO] [netd] [CPU3] NETD: DHCP configured ��� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[22836512754] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[22842115626] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 2323 with backlog 64
[22845962502] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 2323, handle=1
[22848314346] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[22850693415] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=2
[22852544880] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[22854534153] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=3
[22855359615] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=2)
[22860601797] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1274)
[22861828836] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[22864254336] [INFO] [telnetd] [CPU2] telnetd: listening on guest port 2323 (handle=1)
[22878570528] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=245
[22880799678] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[22882315533] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22884220227] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22886340213] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22888474191] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=245 pred=0 subj_lo=0
[22902231561] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1278)
[22903838265] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[22917759084] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=283
[22920225405] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[22922068323] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22923272130] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22924441023] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22925593779] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=283 subj_lo=0
[22936165263] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[22938397350] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[22940829978] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[22942409292] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[22945053120] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22946049324] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1280)
[22946940522] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[22947900162] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[22955922165] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=70
[22957380666] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[22960042512] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22964220543] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22966477710] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22972851495] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[22976415726] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[22985589000] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=74
[22987492935] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=21
[22989878538] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[22999444611] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[23004472854] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[23008103613] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[23009556438] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[23021111949] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=70
[23022928137] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23025824943] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23028558762] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[23032883643] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23035531365] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[23037254526] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23039339862] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23050026813] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23053124589] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[23055026313] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[23056434093] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23059247640] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23061267240] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[23064031617] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[23084796471] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23132231166] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[23135698080] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:58572 on listener 1
[23179053414] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 60 bytes - TCP ACK
[23186671464] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 60 bytes
[23209877064] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=85
[23211812580] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 75 byte frame (79 encoded) to netd rx_port=21
[23228965122] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (79 bytes sent)
[23233090122] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 75 bytes
[23236846677] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 74 bytes - TCP ACK
[23238685536] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 74 bytes
[23254484451] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[23255813625] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23258708880] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23260449894] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23262590505] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 89 bytes - TCP ACK
[23269812621] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 89 bytes
[23289748185] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[23293253181] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23295990564] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23297474574] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23299250337] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[23310937023] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[23343897126] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[23345928672] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23348326749] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23355086040] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23357167185] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 61 bytes - TCP ACK
[23359285224] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 61 bytes
[23362924299] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[23386939521] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[23388608430] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23391107487] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23392713960] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[23393855727] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23398131009] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 55 bytes - TCP ACK
[23426328552] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 55 bytes
[23456003769] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[23457832398] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23460161241] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23468103648] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23471185485] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[23475378795] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[23484007932] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[23500301649] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[23501738733] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23503413120] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23518977075] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[23521576551] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[23544438258] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=658 watches=13 history=1024 journal=1024 symbols=306 drops=0
[23691801672] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[23855191569] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[24060430779] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 29168 bytes, hash=790243a6d8f2a2ea)
[24302760438] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 33264 bytes, hash=91427bae3069f281)
[24577061718] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=ae36cd328bcc45e4)
[24748290831] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[24837730929] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[25057320519] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[25288191852] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[25422591216] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=708 watches=13 history=1024 journal=1024 symbols=338 drops=0
[25540210311] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[25546473777] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[25548312636] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[25549512714] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[25550845815] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[25552298442] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=226 subj_lo=0
[25558662261] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[25566233121] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[25567400760] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[25568533782] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[25569848172] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=338 pred=0 subj_lo=0
[25869021684] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[25940512257] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[25943315805] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 55 bytes - TCP ACK
[25951412289] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 55 bytes
[25981928709] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[25983780999] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[25985796573] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[25994978196] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[25997688816] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 59 bytes - TCP ACK
[26004272580] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 59 bytes
[26021462148] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[26023690968] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[26037053295] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[26038700523] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[26041900599] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26053903953] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26056181745] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP ACK
[26061984267] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[26072673495] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=70
[26074295841] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[26077224855] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26084997345] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26086754826] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 55 bytes - TCP ACK
[26092743864] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 55 bytes
[26130893052] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=70
[26132541567] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[26134830315] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26141329896] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26143998738] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 57 bytes - TCP ACK
[26146893399] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 57 bytes
[26170569645] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26175484467] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=70
[26176897626] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[26179435227] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26192132241] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26194548666] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 55 bytes - TCP ACK
[26221278963] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26223049974] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26224645458] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26226381687] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=348 pred=0 subj_lo=0
[26236145859] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 55 bytes
[26262699573] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26270211165] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=70
[26272763517] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[26275199478] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26276684775] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26279214324] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[26290509795] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[26306734344] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=70
[26309107836] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[26312791593] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26316823731] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26335063128] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 55 bytes - TCP ACK
[26355284472] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 55 bytes
[26365292745] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=70
[26367336831] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[26369830080] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26386126800] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26388167784] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[26394466164] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[26401132857] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26426420493] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=70
[26429383464] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[26433116127] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26435499717] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26438085366] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[26444831424] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[26469428040] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=70
[26472538455] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[26476027182] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26483941902] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26492302320] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[26513966721] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 102 bytes - TCP ACK
[26534144175] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 102 bytes
[26590391223] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=25 len=70
[26592982020] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[26596585950] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26599463682] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26602748700] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 61 bytes - TCP ACK
[26621010405] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 61 bytes
[26647251972] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=26 len=70
[26649948336] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[26653164285] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26657906319] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26677064370] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[26702740647] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26880952956] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[26948219364] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26949771618] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26951545170] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26952976215] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=349 pred=0 subj_lo=0
[27018891636] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27075513729] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[27142997079] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x121ec000
[27144762282] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[27146198079] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[27234551685] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[27241360410] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[27245978067] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[27250832433] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[27253251069] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[27255293538] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[27262290693] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[27342077565] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[27384619218] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[27386668023] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[27389426394] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[27413717793] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x121fa000
[27415478673] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[27576144750] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[27967517061] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=0e57f834b3afbcb1)
[28456010616] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[28514181432] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[28672813455] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=785 watches=17 history=1024 journal=1024 symbols=365 drops=0
[28696681530] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[28704451479] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[28707484344] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28727552502] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[28731005259] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[28733820291] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=1 (16384b)
[28831829829] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[28833613809] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[28835546850] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28837486194] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=350 pred=0 subj_lo=0
[28844813085] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[28974744117] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29025975297] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=c1e9a7b90a789e5c)
[29092365588] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[29093861445] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[29095761849] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29097715614] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=351 pred=0 subj_lo=0
[29118126213] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29255814522] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29467458867] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29604417711] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29621148645] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[29714657148] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29785946652] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=27 len=70
[29788873818] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[29792792799] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[29804939373] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[29808486246] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[29811612501] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[30230273172] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[30300798099] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: TCP_CLOSE handle=4 (initiating close)
[30315115710] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP FIN
[30320036076] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[30341463273] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=28 len=70
[30343869039] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[30346921143] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30703238577] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[30799387740] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30816387591] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: GC removed explicitly closed TCP socket
[31193150802] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[31536062481] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[31749257562] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[32341351812] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[32461039908] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=856 watches=19 history=1024 journal=1024 symbols=367 drops=0
[32902393689] [INFO] [flytrap] [C
```
</details>
