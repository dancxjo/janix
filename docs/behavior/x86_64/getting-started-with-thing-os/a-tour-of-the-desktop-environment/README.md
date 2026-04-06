# ❌ Scenario: A tour of the desktop environment

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for the system to reach ready state | ✅ | 5056ms | - - - |
| 3 | Then I should see log messages on the terminal | ✅ | 501ms | - [📜](./03/serial.log) - |
| 4 | And the serial output should contain "SPROUT: Supervisor starting" | ✅ | 0ms | - - - |
| 5 | And I should see the desktop wallpaper | ❌ | 2530ms | - [📜](./05/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11435759247] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11441415678] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11445213813] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11447446824] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11448760290] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11449424250] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11450104314] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11450705442] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11451308451] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11451965679] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11452577928] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11453201298] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11453923569] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11454605778] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11455339896] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11455966599] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11456605776] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11457217266] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11457849744] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11458461630] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11459150637] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11459817765] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11460429057] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11461035564] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11461686423] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11462517066] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11463343287] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11464287780] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11465222604] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11465880063] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11466636819] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11467505709] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11468356911] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11469384135] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11470364532] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11471530752] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11472750102] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11473937838] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11475124386] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11476333077] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11477520417] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11478720957] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11480537541] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11482471176] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11483796654] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11484719796] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11485586442] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11486439459] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11487371742] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11488385436] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11489111106] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11489792160] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11490582708] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11491249077] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11491868553] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11492522151] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11493088200] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11493670881] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11494237656] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11494852050] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11495416350] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11495998965] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11496563034] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11497147200] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11497709520] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11498302233] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11498866203] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11499449412] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11500011864] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11500594413] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11501183199] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11501791389] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11502353148] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11502936126] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11503499073] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11504079972] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11504692221] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11505280149] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11505843360] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11506423962] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11506986777] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11507570877] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11508159465] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11508742212] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11509305522] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11509887708] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11510451051] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11511031752] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11511607965] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11512193187] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11512846686] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11513431908] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11513995350] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11514704124] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11515268787] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11515902156] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11516485299] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11517068904] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11517815001] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11518542222] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11519112561] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11519698311] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11520262248] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11520847206] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11521457211] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11522044479] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11522609307] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11523506412] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11780259315] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[11793381303] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11798748489] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11800027899] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11800902828] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11805169266] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11806882263] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11808014658] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11808908859] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11809824378] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11810780223] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11812004358] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11813020362] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11813736693] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11814529551] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11815228986] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11815919577] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11817208095] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11818418931] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11819186544] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11820873075] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11821831428] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11822849082] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11824570659] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11826220791] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11827089912] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11828050014] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11829232338] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12238480089] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12239629347] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12243558822] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12244477113] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12245278254] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12246891822] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12262204746] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12263637210] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12264413601] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12266454981] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12267023043] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12269516391] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12277677621] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12280068273] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12294721461] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12295437726] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12311718639] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12312450447] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12314909112] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12316282077] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12317491131] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12319814562] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12320659098] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12355428492] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62278000 ticks/sec), init_cnt=622780 for 100Hz
[12356802843] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12357583656] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12358712751] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12364545864] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12395355258] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12396312786] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12399436863] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12400694163] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12401721321] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12404395542] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12405683565] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12424095717] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12426003414] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12426877650] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12427699548] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12428409312] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12429422478] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12430043505] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12455762154] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12456691929] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12458551512] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12459454524] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12460494717] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12461311500] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12463059180] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12463733766] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12469717722] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12470687955] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12472989210] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12473846748] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12480570234] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12482073648] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12482756352] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12484097571] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12484942503] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12485746218] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12496729806] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12499654695] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12500694162] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12501437487] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12521150730] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12538104447] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12539966868] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12542975775] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12544403751] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12546398502] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12548449518] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12550381041] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12551239272] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12552400476] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12558144126] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12559713837] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12562024761] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12566048979] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12568441776] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12569267370] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12570591726] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12581652468] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12582428496] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12588726975] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12589509306] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12614825949] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12615559671] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[12949562142] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13416677934] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13440288213] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=497 journal=422 symbols=51 drops=0
[13477230426] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[14679919122] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=446 watches=0 history=959 journal=768 symbols=93 drops=0
[15369637602] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15458449941] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15459464955] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[15559718394] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[15616509414] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[15641638947] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[15642453222] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[15643077846] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[15646905978] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[15661671267] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[15679020918] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[15685336920] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[15737325318] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[15758212140] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[15758993778] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[15762539067] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[15785878449] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[15803376072] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[15806814210] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[15807764676] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[15871253871] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[15872865459] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[15953793933] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[16014749025] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[16026625560] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[16030285227] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[16032575559] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[16034516586] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=583 watches=0 history=1024 journal=1024 symbols=181 drops=0
[16096550844] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[16102240341] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[16103233311] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[16104045573] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[16104916509] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[16105584099] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[16106245122] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[16106883441] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[16107475065] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[16108229643] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[16109067084] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[16109791401] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[16110516246] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[16111247823] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[16111935576] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[16112667483] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[16113312138] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[16113958443] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[16114578678] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[16115217162] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[16115838618] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[16116451692] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[16117061862] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[16117674210] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[16118286591] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[16118945073] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[16119587253] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[16120208709] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[16120882866] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[16121498118] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[16122123600] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[16122773667] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[16123417992] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[16124055585] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[16124689020] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[16125304404] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[16126077297] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[16126815408] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[16127558370] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[16128301398] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[16129062411] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[16129915824] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[16130701620] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[16132309149] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[16134790287] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[16135576050] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16143333822] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[16157067003] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[16161537051] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[16170222519] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[16170893574] [CONTRACT] [kernel] [CPU0] Spawning init process...
[16172805099] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[16175360685] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[16176547266] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [16182000516] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[16182818652] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[16197619779] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[16202268522] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[16203446424] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[16204564959] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[16212950622] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[16217504754] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[16220505576] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[16221635529] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[16225342551] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[16228061685] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[16229194740] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[16232925489] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[16234789329] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[16236543015] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[16238192355] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16243039197] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16244168226] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16245451431] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16246907424] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16248633291] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16249928475] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16254042816] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[16297823553] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[16306030851] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[16310710812] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[16315126938] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[16318003317] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[16321670706] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[16326138642] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[16330569090] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[16335140844] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[16339796979] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[16344664842] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[16349783307] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[16356166431] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[16360861770] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[16365157578] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[16370172159] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[16376387808] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[16382198250] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[16388112873] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[16393060035] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[16398141474] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[16403832192] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[16410097935] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[16415719683] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[16421530389] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[16427108544] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[16432940073] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[16438771074] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[16443187893] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[16448768952] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[16452534846] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[16456135311] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[16462331259] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[16467651981] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[16473302538] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[16479901911] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[16485755088] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[16490784717] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[16496165598] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[16500972444] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[16507389096] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[16511091630] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[16533970530] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[16671132192] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[16677754401] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[16678529208] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16680172641] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16684871214] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[16686136632] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16694007363] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[16698663729] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16699770582] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16701025407] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[16705815819] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[16706876901] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16708581450] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[16709782056] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16714364403] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[16716686745] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16724427555] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[16728372606] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[16729829127] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[16731930105] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[16736077941] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[16742022264] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[16744413015] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[16747676451] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[16754565630] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:53:04 = 1775436784 unix_secs
[16757223615] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775436784, mono_ns=8378197476, offset=1775436775621802524ns
[16759709175] [INFO] [rtc_cmos] [CPU1] System clock anchored
[16774116414] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[16827555822] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[16828903872] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[16829934264] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16833885948] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16841041569] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[16845631275] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[16855712511] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[16860895326] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[16865841300] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16867742463] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210784 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[16874115852] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[16878673449] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[16880367669] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[16881211017] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16882903224] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16890314265] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[16892688450] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[16899747876] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[16902910794] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16905549936] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[16909088163] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[16911061134] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[16913130300] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[16917343443] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[16918701558] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[16919892099] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[16921152072] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[16927587303] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[16928965317] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[16942148751] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[16943667345] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[17430178458] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17435000484] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[17437849638] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[17439536301] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[17443913982] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[17445635097] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[17448037299] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[17449632024] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[17450780094] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[17452980072] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[17454017691] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[17455054287] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[17456005611] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[17456979210] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[17457804375] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[17459108568] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[17464188291] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[17465195715] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17467078596] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17474421954] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17477145840] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17484766233] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[17487875262] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fab68
[17488914201] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[17490796521] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352560 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[17498462421] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[17510274309] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[17528058471] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[17537998731] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[17540082318] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[17545977504] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[17554856979] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[17556460482] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[17560347321] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[17561408337] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[17562350982] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[17563201095] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[17564142585] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[17565070248] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[17566102587] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[17567253726] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[17568776379] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[17574134721] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[17576818512] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[17580002583] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[17582517216] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[17583383433] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17584992711] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17588711151] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[17590399761] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17597386917] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[17599861686] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[17601693384] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[17602417239] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17603903823] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17608505640] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17609907249] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17616900147] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[17619723825] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[17620952976] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[17622634689] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500224 RFLAGS_BEFORE=130 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[17626666761] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[17627466087] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17629167633] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17630811231] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[17632483968] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[17634361305] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17636952003] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17640676680] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[17643095778] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[17645844810] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[17647423068] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[17650763559] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[17651702442] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[17652803454] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[17653856121] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[17654848497] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[17657062269] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[17658046461] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[17660339763] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[17661396390] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434144 RFLAGS_BEFORE=130 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[17665929633] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[17667717804] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[17669394435] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[18027501888] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[18029107965] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[18030270951] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583008 RFLAGS_BEFORE=134 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[18033237816] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[18035023776] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[18036100401] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[18037439871] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[18040744425] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[18043200252] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18047330994] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[18048149823] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18049879914] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18055857897] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[18056734278] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[18058991049] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[18066063840] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[18068459640] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[18071286849] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[18073459833] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[18074598861] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18077304300] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18096796047] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18101580651] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[18123437508] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[18124521690] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[18128019855] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a670
[18129452913] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18131525016] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716720 RFLAGS_BEFORE=134 CR3_BEFORE=68882432 fs_base=0 gs_base=18446744071564586640
[18138541014] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[18139413270] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[18140922261] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[18142580775] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650304 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[18149670792] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[18151765038] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[18155454141] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[18171986910] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[18176940045] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[18178942155] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[18180772599] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[18183118305] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[18186174567] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[18188896374] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[18191039691] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[18192275211] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18194825781] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18261916959] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[18286087941] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[18293767866] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[18297318963] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[18299324274] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18301370736] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369782784 RFLAGS_BEFORE=134 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[18304985688] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[18305810028] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[18306884409] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[18308208072] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18309129168] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18311606148] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18312909681] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[18313693266] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[18316335048] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[18320959140] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18322475193] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18338126961] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18341918232] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[18349648053] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[18353473116] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[18356206770] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[18357585906] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18360119910] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18366256227] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18369256752] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18376722342] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[18380261790] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[18381416493] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18383199747] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915152 RFLAGS_BEFORE=130 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[18386306994] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18387661182] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18389780970] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[18390557724] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18391938972] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[18399293682] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18403347039] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18411027492] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[18414569943] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[18416119788] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18418157076] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981712 RFLAGS_BEFORE=130 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[18421630986] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[18422983326] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18425301444] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18434291436] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18438237147] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[18445533216] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[18448255221] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[18450482193] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[18451161531] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18452621715] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18477376962] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18482101440] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[18488946630] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[18491473671] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cc18
[18492391071] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18493665993] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113968 RFLAGS_BEFORE=130 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[18496671006] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[18497413836] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18498992325] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18513982179] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[18516159453] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[18519156711] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18526886433] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[18530126736] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[18531059613] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010cc18
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18533370966] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370197632 RFLAGS_BEFORE=130 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[18537268596] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[18538103166] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18539972418] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18540807285] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[18542396829] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[18547524600] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18549532287] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[18550607658] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18552838458] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18554500965] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=225 subj_lo=0
[18558686190] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18560056845] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18561555573] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18563093340] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=226 pred=0 subj_lo=0
[18579239646] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[18589832349] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[18597899727] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[18601161744] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[18602515503] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18603990999] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18605477781] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18606995550] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18608374224] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18610016238] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18611622579] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[18613462461] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[18616214265] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18618660621] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849168 RFLAGS_BEFORE=134 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[18625553364] [INFO] [nectar] [CPU2] NECTAR: Started.
[18630388128] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18632706774] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047888 RFLAGS_BEFORE=134 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[18637803393] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18638881866] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[18639598593] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18640716732] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18641794677] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=233 pred=0 subj_lo=0
[18644462133] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010e118
[18648438006] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[18649365999] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18650434011] [INFO] [fontd] [CPU3] FONTD: Service ready
[18651027714] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18652760115] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18654017547] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=234 subj_lo=0
[18656792715] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370267056 RFLAGS_BEFORE=130 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[18663020508] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[18663900024] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18664749609] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[18666408717] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18667089309] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[18672900774] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[18683641713] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18684605148] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18685646133] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18686705532] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=236 subj_lo=0
[18687981411] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[18692558775] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[18699389907] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18700434060] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18701505768] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18702700401] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[18705312813] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1251 backend=VirtIO-GPU
[18707369340] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[18708491670] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18710190807] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18711387816] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18712370919] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18713389563] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18714434508] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[18720039855] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18720959037] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18722091993] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18723273426] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[18744846087] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[18757874718] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[18759083970] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[18841699800] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[18857423112] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[18865909326] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[18868738086] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f0fe8
[18869670699] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e3
[18870991920] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370356752 RFLAGS_BEFORE=130 CR3_BEFORE=72052736 fs_base=0 gs_base=18446744071564586640
[18874486323] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[18875354025] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18877025739] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18877931820] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[18882480804] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18884019858] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18892481916] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[18895250319] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f0fe8
[18896371428] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[18897807885] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370422832 RFLAGS_BEFORE=134 CR3_BEFORE=73101312 fs_base=0 gs_base=18446744071564586576
[18901388121] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[18903753759] [INFO] [echo] [CPU1] echo: starting up
[18906764118] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[18918119814] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[18934830453] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[18937215957] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[18948837633] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[18951039822] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f5000
[18953736318] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f5000
[18955691436] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f8000
[18958251312] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276787200)
[18959793270] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[18961506102] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f9000 phys=0x4656000
[18963168147] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[18965661693] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[18968017101] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18968855763] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[18969661425] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18970370397] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[18975613668] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[18977552583] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[18979993956] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[18984836772] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[18989915439] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[18998895069] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[19000036572] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[19001732277] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[19002747291] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[19003718019] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[19005096528] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[19006644195] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[19007968815] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[19009365243] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[19011267198] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[19012490310] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[19013897001] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[19018934715] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[19024666980] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[19027428156] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[19030727595] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[19032255000] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[19034423595] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[19035194310] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19036719438] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[19038422568] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19043606010] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19045502388] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[19053995301] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[19062985887] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19066703700] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[19069355778] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[19070331522] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[19072443027] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[19074219615] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[19075985478] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19077393720] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19078260696] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19079978115] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19081222644] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[19087305204] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19090916955] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19098462735] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[19101238992] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[19102249947] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f2298
[19103388942] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19105259976] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370706976 RFLAGS_BEFORE=130 CR3_BEFORE=73998336 fs_base=0 gs_base=18446744071564586640
[19108810314] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[19109768865] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19111588551] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19114527234] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[19115778594] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[19119088989] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[19124555043] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[19127286090] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[19128691032] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[19131381489] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19132892196] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19134021654] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19144891425] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[19146970260] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[19148697447] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[19155150630] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f2298
[19156234680] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19157911641] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370772512 RFLAGS_BEFORE=130 CR3_BEFORE=74145792 fs_base=0 gs_base=18446744071564586576
[19179439521] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[19181661873] [INFO] [bloom] [CPU3] bloom: creating surface...
[19183022397] [INFO] [bloom] [CPU3] bloom: surface created!
[19183795290] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[19187323716] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[19192564908] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[19203050988] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19225684698] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1261
[19227933615] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[19243176447] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[19245948183] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[19247012730] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[19248106350] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [19257567714] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[19258800066] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 3)
[19267262190] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[19273895784] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19275603237] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19276948284] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[19284879636] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[19295271699] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1261
T:07D0 T:0640 T:F0B0 [19321938801] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[19328817783] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[19330861605] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[19333281693] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[19335491076] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[19337831172] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[19347376983] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
T:1220 [19355776110] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[19357239099] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[19364212428] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[19365782205] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[19371083094] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[19375441206] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[19386723081] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=276
[19387782909] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[19388687076] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19389624243] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19390746804] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19391980113] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=276 subj_lo=0
[19408317159] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1272)
[19409918187] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[19422063606] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=243
[19424443236] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[19425867285] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19427141580] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19428308658] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19429598760] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[19442196048] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1275)
[19443354348] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[19455428355] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=280
[19457233818] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[19458304272] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19459486299] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19460688588] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19462165833] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=280 subj_lo=0
[19475611386] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1276)
[19476976200] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[19484862837] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([245, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[19490481219] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[19492812603] [INFO] [anther] [CPU1] anther: Connected to network stack
[19558434489] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[19560215466] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[19566149691] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[19592797488] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[19594373931] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[19599378711] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[19603994157] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[19630078116] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[19632577140] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19663177644] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19669456356] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[19672304553] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[19677494661] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[19679203467] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[19681486407] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19687687767] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19691299155] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[19693091286] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[19721796996] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[19724894574] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[19729604334] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[19732024191] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[19734325314] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[19736379993] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[19737625578] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[19776331905] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[19778261778] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[19779432123] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[19789907478] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[19898041680] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[19923868338] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[19966375836] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[19987654500] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=656 watches=13 history=1024 journal=1024 symbols=307 drops=0
[20115512901] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[20274619497] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[20432307786] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[20672654145] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[20833860267] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[20999896962] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[21013807452] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[21185514372] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[21298754433] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[21306574179] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[21308060961] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[21309134748] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[21310456101] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[21312255294] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=225 subj_lo=0
[21336599130] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[21337872270] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[21339204876] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[21340739145] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=338 pred=0 subj_lo=0
[21391784733] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[21563255241] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[21672945888] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=712 watches=15 history=1024 journal=1024 symbols=340 drops=0
[21769207317] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[21952761765] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[22118508786] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[22120441563] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[22148054709] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[22212569148] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22267095444] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22268465769] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22269633309] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22271009541] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=306 pred=0 subj_lo=0
[22293807888] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22377821235] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[22419033450] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22447538157] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=fec1d29ef0678173)
[22515025137] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[22521504819] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22557595962] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[22606234761] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22615637847] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22616884422] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22618038894] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22619254185] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=340 pred=0 subj_lo=0
[22624890849] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[22639297461] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[22640882814] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[22709835423] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22751461458] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[22756543227] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[22784092848] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x1222d000
[22786467594] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[22788331104] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[22852970118] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[22863298986] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[22868393031] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[22870959243] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[22873646532] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[22878921219] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[22896667827] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[22898782665] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[22901008746] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[22921727598] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x1222e000
[22923573552] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[23063802432] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=14aa2015f9906e3f)
[23194841340] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23196072174] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23197179588] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23198464938] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=341 pred=0 subj_lo=0
[23307932109] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[23549415450] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[23683006974] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23792650596] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[23802639696] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=799 watches=18 history=1024 journal=1024 symbols=354 drops=0
[24017467452] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[24313472469] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[24593641743] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[24888560103] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[25162968468] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[25452852942] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[25468427919] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=835 watches=18 history=1024 journal=1024 symbols=365 drops=0
[25470391584] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[25595666844] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25629674367] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[25663634403] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[25664894838] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25666096962] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25667424750] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=342 pred=0 subj_lo=0
[25722134988] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25828292886] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[25921144557] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26013925344] [INFO] [fontd] [
```
</details>
