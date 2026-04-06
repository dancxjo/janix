# ❌ Scenario: Correlating Visual Elements with Graph State

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 19523ms | - - - |
| 2 | And the anther server is ready | ✅ | 1706ms | - [📜](./02/serial.log) - |
| 3 | When I wait for 15 seconds | ✅ | 15002ms | - [📜](./03/serial.log) - |
| 4 | Then I should see the desktop wallpaper | ❌ | 1013ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[59155501812] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[59162863848] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[59168765469] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[59170721676] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[59171892780] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[59172544629] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[59173481334] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[59174240367] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[59174958414] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[59175611682] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[59176186773] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[59176773777] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[59177476479] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[59178123741] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[59178818226] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[59179410741] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[59180012661] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[59180653554] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[59181256926] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[59181889503] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[59182460007] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[59183029686] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[59183710476] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[59184287745] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[59184904383] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[59185527819] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[59186109510] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[59186780565] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[59187360870] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[59187944640] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[59188568967] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[59189169138] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[59189787459] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[59190385122] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[59190958794] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[59191676346] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[59192360568] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[59193052545] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[59193729639] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[59194432605] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[59195135241] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[59195827548] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[59197083561] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[59198436033] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[59199177411] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[59199711516] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[59200374486] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[59201081082] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[59201813649] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[59202406956] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[59202910272] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[59203417152] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[59203917729] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[59204446356] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[59205016266] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[59205568851] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[59206105068] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[59206658775] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[59207193276] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[59207792424] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[59208358638] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[59208908319] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[59209437738] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[59209985142] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[59210515056] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[59211161163] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[59211710613] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[59212260987] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[59212806180] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[59213354475] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[59213884488] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[59214432057] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[59215037574] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[59215599597] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[59216178318] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[59216729847] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[59217263919] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[59217814722] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[59218395918] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[59218967379] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[59219502342] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[59220056082] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[59220680607] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[59221277148] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[59221813728] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[59222365587] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[59222920713] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[59223477588] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[59224013310] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[59224591239] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[59225125245] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[59225674992] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[59226206490] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[59226757887] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[59227285623] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[59228072211] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[59228838570] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[59229549819] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[59230112040] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[59230660434] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[59231230773] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[59231783160] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[59232314130] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[59232891234] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[59233424976] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[59233974756] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[59234518827] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[59235400422] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[59466828168] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[59477253594] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[59482019916] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[59483293584] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[59484149241] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[59488386573] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[59490018225] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[59491049970] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[59491736667] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[59492398053] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[59493053466] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[59494029078] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[59495013336] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[59495694126] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[59496356238] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[59497016832] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[59497673961] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[59498839257] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[59499828366] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[59500512984] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[59502077712] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[59502992109] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[59503951155] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[59505538125] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[59507037777] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[59507815785] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[59508341178] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[59509078233] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[59869547397] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[59870643063] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[59873962038] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[59874840432] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[59875597848] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[59877111492] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[59889605655] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[59890921101] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[59891661225] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[59893361715] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[59893902849] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[59896273767] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[59904045333] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[59905724175] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[59918725284] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[59919343935] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[59935303230] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[59935951977] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[59937927126] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[59939092950] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[59940166605] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[59942449314] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[59943280947] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[59978226132] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62461000 ticks/sec), init_cnt=624610 for 100Hz
[59979753900] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[59980578768] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[59981718258] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[59987155239] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[60017350404] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[60018272226] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[60019828737] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[60021060165] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[60022177677] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[60024810615] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[60026061315] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[60047417727] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[60049173492] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[60049895895] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[60050721291] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[60051394557] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[60052466727] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[60053215233] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[60078400503] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[60080059380] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[60080906688] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[60082070268] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[60082952127] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[60084002616] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[60084800523] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[60085439634] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[60091841469] [INFO] [kernel::root] [CPU0] Spawning Root service...
[60092721447] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[60094396098] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[60095385966] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[60100245249] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[60101706225] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[60102396783] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[60103566567] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[60104368335] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[60105355068] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[60116793363] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[60119949648] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[60121063035] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[60121789101] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[60141354471] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[60158595354] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[60160475859] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[60163487208] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[60164954025] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[60167016426] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[60169058565] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[60170904585] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[60171693747] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[60172663122] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[60178363410] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[60180037698] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[60182392611] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[60186273873] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[60188959116] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[60189830745] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[60191270634] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[60202840038] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[60203619168] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[60209798418] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[60210666087] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[60235473309] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[60236361141] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[60554791440] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[61001122017] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[61026463113] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[61059475158] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[62246244390] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=452 watches=0 history=968 journal=777 symbols=98 drops=0
[62897739993] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[62989742574] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[62990719176] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[63087307470] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[63142529538] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[63165485196] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[63166290726] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[63166910070] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[63170157369] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[63184971168] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[63200133018] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[63202195650] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[63250679250] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[63269514924] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[63270325470] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[63274183929] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[63296839089] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[63315067002] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[63318539856] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[63319577805] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[63382789173] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[63384485571] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[63465091998] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[63525586773] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[63536588775] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[63540897420] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[63543043542] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[63604032327] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[63608705424] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[63609673248] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[63610485279] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[63611386575] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[63612072150] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[63612707334] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[63613345521] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[63613937937] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[63614544081] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[63615235332] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[63615849231] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[63616473888] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[63617115738] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[63617793195] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[63618526158] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[63619159131] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[63619803753] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[63620426760] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[63621064287] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[63621694851] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[63622295022] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[63622898592] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[63623509290] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[63624132132] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[63624787644] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[63625431507] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[63626049102] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[63626715702] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[63627321582] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[63627940827] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[63628580400] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[63629215287] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[63629844135] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[63630495324] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[63631103844] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[63631839249] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[63632678010] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[63633409719] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[63634323027] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[63635089320] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[63636019392] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[63637109085] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[63638517228] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[63640149837] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[63640993548] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[63648653112] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[63662008278] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[63666406386] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[63675454854] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[63676136073] [CONTRACT] [kernel] [CPU0] Spawning init process...
[63678008757] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[63680343210] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[63681501741] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
ThingOS Petals
type 'help' for commands

petals> USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[63686817744] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[63688239945] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[63702512478] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[63711152472] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[63712361559] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[63713417889] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[63721035048] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[63725666268] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[63729176379] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[63730234491] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[63734275737] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[63737993253] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[63739079085] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[63743562036] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[63744616221] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[63745635525] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[63746594439] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[63750043566] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[63751119102] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[63752269515] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[63753696765] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[63755067288] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[63756378543] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[63758951289] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=185 drops=0
[63761144568] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[63805615764] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[63812979846] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[63817440951] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[63822047124] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[63824735271] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[63828229014] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[63832832679] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[63837520560] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[63841843923] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[63846520485] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[63851004756] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[63856066692] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[63860817339] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[63865122057] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[63869751231] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[63876209859] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[63880872297] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[63885311886] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[63889493316] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[63893995011] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[63898877394] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[63903725424] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[63908772642] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[63913344000] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[63917721285] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[63922240536] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[63926428962] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[63931168059] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[63935810169] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[63940503924] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[63943573221] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[63946685088] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[63951052374] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[63955099329] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[63959699892] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[63964487136] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[63969139377] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[63973791948] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[63978583779] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[63983323272] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[63988367685] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[63991417941] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[64008688821] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[64131208713] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[64137682884] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[64138439277] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[64140083997] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[64145295093] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[64147022049] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[64157145525] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[64162058796] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[64162978341] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[64164312036] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[64168826601] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[64171147293] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[64171919658] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[64173599490] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[64177570776] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[64179249057] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[64186059960] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[64188434904] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[64189383159] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[64190643693] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[64193576370] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[64196229207] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[64197763113] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[64199797299] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[64204089345] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:01:23 = 1775437283 unix_secs
[64205933946] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775437283, mono_ns=32102748265, offset=1775437250897251735ns
[64207461054] [INFO] [rtc_cmos] [CPU1] System clock anchored
[64219166583] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[64253328150] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[64262958738] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[64264391334] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[64266986256] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[64273069938] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[64275665454] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[64282699866] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[64285798830] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[64289138892] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[64290754077] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[64295570889] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[64299355791] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[64301331369] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[64302188841] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[64303993710] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[64311051519] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[64313195628] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[64319810808] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[64322281023] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[64324034775] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[64325384079] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[64327077045] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[64332535872] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[64333717569] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[64340714262] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[64341544344] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[64343221767] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[64344133458] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[64345456197] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[64359135159] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[64360851918] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[64830046281] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[64834728717] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[64837570182] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[64839219720] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[64843504275] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[64845219879] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[64847266572] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[64848734643] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[64849806615] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[64852256502] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[64853322303] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[64854545481] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[64855513470] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[64856386419] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[64857322992] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[64858549767] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[64862896890] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[64863967080] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[64865776173] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[64872628788] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[64875176322] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[64882441239] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[64885485588] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fab68
[64887169116] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[64888646163] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352768 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[64894888773] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[64905069867] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[64914629505] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[64922588379] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[64924615008] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[64930182735] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[64938375414] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[64939927140] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[64947505623] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[64948436586] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[64949600925] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[64950186543] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[64951810704] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[64953021474] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[64953806709] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[64954681275] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[64955500731] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[64961307477] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[64963640379] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[64965971334] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[64967935560] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[64968894936] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[64970488242] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[64974121047] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[64975720260] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[64982288448] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[64984351608] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[64986164298] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[64986874458] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[64988258643] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[64993005132] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[64994303352] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[65001047562] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[65003080032] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
[65003993769] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[65006227968] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500288 RFLAGS_BEFORE=130 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[65009777415] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[65010497178] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[65011896312] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[65012622180] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[65013694251] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[65017054839] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[65017827534] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[65019306858] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[65020122222] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[65021081895] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[65023190265] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[65024643453] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[65026059417] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[65027772909] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[65028986550] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[65029903851] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[65031663543] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[65032982817] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[65034411948] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[65035651065] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[65037260673] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[65038791048] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434208 RFLAGS_BEFORE=134 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[65043492360] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[65395087527] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[65395819995] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[65398046373] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583072 RFLAGS_BEFORE=130 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[65401152267] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[65402991555] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[65403647034] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[65405007987] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[65407825956] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[65408535159] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[65411237958] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[65412175158] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[65414324547] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[65420803206] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[65423126043] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[65426741952] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[65430934932] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[65433998355] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[65436478833] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[65437982709] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[65438743458] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[65440438734] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[65458758651] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[65463270708] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[65483427504] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[65485872276] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb01095c8
[65487751923] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[65489192439] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369719024 RFLAGS_BEFORE=134 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[65493111354] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[65494708323] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653488 RFLAGS_BEFORE=134 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[65498356803] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[65499872691] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[65500718679] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[65502350001] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[65561430495] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[65563743993] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[65570271525] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[65572473648] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[65573963268] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[65576590398] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[65588428422] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[65590556361] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[65593451715] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[65594495208] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[65595698058] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[65598388218] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[65664594798] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[65687700903] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[65694376968] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[65697341952] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
[65698087290] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[65701316406] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369784560 RFLAGS_BEFORE=134 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[65704158762] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[65704872123] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[65706522816] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[65707411011] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[65709869544] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[65713360878] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[65714846934] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[65731669938] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[65734669242] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[65741289768] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[65743900926] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[65745733152] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[65746419024] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[65747913660] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[65753158218] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[65755437957] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[65761913910] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[65764122402] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
[65764902753] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010ac38
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[65767885590] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[65768622018] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[65769837111] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915632 RFLAGS_BEFORE=134 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[65772808134] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[65774874429] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[65776700154] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[65781104895] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[65784705195] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[65791529793] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[65794154019] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
[65794884837] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010ac38
[65796503916] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[65797250343] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[65798924895] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[65799658155] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981776 RFLAGS_BEFORE=134 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[65807534331] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[65811540267] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[65818177491] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[65820503001] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[65822533359] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[65823246225] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[65824579656] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[65848809873] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[65853334404] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[65859837252] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[65862314892] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
[65863006935] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010ca80
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[65864581464] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113824 RFLAGS_BEFORE=130 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[65867476917] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[65868205986] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[65869326534] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[65870251359] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[65886625827] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[65889546096] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[65890405713] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[65894342580] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[65895700662] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[65896968951] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[65898384189] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[65899290864] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[65903747349] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[65904790017] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[65906409954] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370198128 RFLAGS_BEFORE=130 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[65909712396] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[65910629994] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[65912054967] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[65913064239] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[65915246892] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[65915946228] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[65916863430] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[65918369286] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[65919755319] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[65924555301] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[65925699609] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[65926863090] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[65928077358] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[65929303440] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[65930472465] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[65931997758] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[65933334720] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[65949295368] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[65959060233] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[65965663434] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[65966873016] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[65969207040] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[65970462261] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[65971615248] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[65972816745] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[65973887694] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[65975108892] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[65976497004] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[65979325863] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[65982946755] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[65983876497] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[65985189534] [INFO] [fontd] [CPU3] FONTD: Service ready
[65985720735] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369850096 RFLAGS_BEFORE=134 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[65990105280] [INFO] [nectar] [CPU2] NECTAR: Started.
[65992157649] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010ac38
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[65993756037] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[65994702081] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370048032 RFLAGS_BEFORE=130 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[65997666075] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[65998739862] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[65999951622] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[66001354782] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[66003385932] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[66005053356] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370266880 RFLAGS_BEFORE=134 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[66009774468] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[66011114136] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[66012975006] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[66020731161] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[66026547477] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[66029472795] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[66030503649] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[66031641522] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[66032888361] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=235 subj_lo=0
[66040391439] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[66042547890] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[66043820865] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[66045055890] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[66046274910] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f4000
[66047604282] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f4000
[66048634872] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f7000
[66050229498] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276783104)
[66051152937] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[66052738257] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f8000 phys=0x44b8000
[66054223554] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[66054901143] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[66056022450] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[66057034824] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[66058134054] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[66058901040] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[66060789432] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[66062199852] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[66064219980] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[66065167080] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[66066166683] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[66067224135] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[66071449521] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[66072755859] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[66073671147] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[66074647056] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[66075696885] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[66079957317] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[66088448778] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[66090759966] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[66092718879] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[66094182759] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[66095674458] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[66096949941] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[66098210640] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[66099298287] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[66100493514] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[66101540142] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[66105634716] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[66110620455] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[66118458615] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1251 backend=VirtIO-GPU
[66119840919] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[66120672321] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[66121386870] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[66122599521] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[66123148806] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[66253374660] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[66254370765] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[66267804240] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[66275140470] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[66277901349] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[66281160264] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db648
[66285193095] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[66285964338] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e3
[66288069474] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[66291595722] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370488816 RFLAGS_BEFORE=134 CR3_BEFORE=72167424 fs_base=0 gs_base=18446744071564586640
[66295838268] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[66297281490] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[66298184601] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[66304435725] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[66307014411] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[66308452056] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db648
[66309399387] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[66310616691] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370559152 RFLAGS_BEFORE=130 CR3_BEFORE=73216000 fs_base=0 gs_base=18446744071564586576
[66314875605] [INFO] [echo] [CPU1] echo: starting up
[66315365292] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[66316852635] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[66326511801] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[66338503308] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[66344425125] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[66346154985] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[66361435899] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[66363185031] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[66364766457] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[66365497176] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[66367278186] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[66369689793] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[66371192613] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[66372285936] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[66374125818] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[66380322492] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[66381764097] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[66384103665] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[66385983477] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[66387538173] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[66388390761] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[66389296842] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[66390082770] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[66391312845] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[66391995747] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[66395156883] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[66399186876] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[66403116483] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[66409278639] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[66410604777] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[66412383840] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[66414105351] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[66415204284] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[66416249823] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[66417716970] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[66418464519] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[66419876193] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[66422570808] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f1480
[66423431283] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[66424678650] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[66425478966] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370710704 RFLAGS_BEFORE=130 CR3_BEFORE=73981952 fs_base=0 gs_base=18446744071564586640
[66431761077] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[66433754541] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[66435217596] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[66443896200] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1480
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[66445878411] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370776240 RFLAGS_BEFORE=130 CR3_BEFORE=74129408 fs_base=0 gs_base=18446744071564586576
[66451456104] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[66452301531] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[66469729788] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[66482862435] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[66485273217] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[66489918132] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[66490832694] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[66510365559] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[66513036711] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[66514469505] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[66533812389] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[66535961019] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[66539854458] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[66547801716] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[66566159484] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([237, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[66572369127] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[66594131109] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[66596449029] [INFO] [bloom] [CPU3] bloom: creating surface...
[66597539316] [INFO] [bloom] [CPU3] bloom: surface created!
[66598441767] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[66602308839] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[66610661865] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[66613225503] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[66614587743] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[66619652088] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[66630273831] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[66632871393] [INFO] [anther] [CPU1] anther: Connected to network stack
[66648979188] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[66649854381] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[66653000568] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[66657817281] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1264
[66658891596] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[66670679328] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[66673695132] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[66675018630] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[66676557849] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
[66679719117] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
T:[666821808738] [INFO2] [vir7tio_n0etd]  [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[66683451219] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[66691998417] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[66700159878] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[66708051201] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
T:07D0 T:0640 T:F0B0 [66733376292] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[66739475055] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[66743150496] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[66744832737] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[66750371391] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[66752390628] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[66754434549] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[66755369802] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[66756695544] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[66758951193] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[66761371644] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[66770888019] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1264
[66772871913] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[66775496370] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[66778834056] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[66780371988] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
T:1220 [66785087919] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[66787840746] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[66797284917] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[66799490241] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[66805294380] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[66816130194] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=279
[66817631133] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[66818631858] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[66819719406] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[66820884537] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[66822062208] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=279 subj_lo=0
[66835680945] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1274)
[66836842380] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[66841316355] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[66844805841] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[66849936450] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[66852436728] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[66854755770] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[66855605454] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[66856972281] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[66868431432] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=243
[66870004047] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[66871275933] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[66872402619] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[66873619560] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[66874958535] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[66891406164] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1275)
[66892708443] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[66903513501] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=284
[66906281277] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[66907279329] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[66908288832] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[66909486798] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[66910608270] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[66912224016] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=284 subj_lo=0
[66914196855] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[66917145537] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[66930896175] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[66936928410] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[66939820893] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[66942112743] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[66943499535] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[66947088021] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1280)
[66948586914] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[66953212227] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=74
[66954629643] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=25
[66956842788] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[66959053854] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[66961526016] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[66973100865] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[66976433238] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[66977127888] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[66983761746] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[66985334097] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[66988102764] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[66997846410] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[66998900100] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=149
[67000727640] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=25
[67002698862] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[67003759515] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[67017389142] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[67019998518] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:33852 on listener 1
[67029654681] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[67033623393] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[67041479241] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[67103382555] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[67118983305] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=3
[67128458232] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 37 (user thread) assigned to CPU 1
[67130704080] [INFO] [anther] [CPU1] anther: Thread spawned TID=37 for conn_handle=3
T:5EE0 [67258868787] [INFO] [anther] [CPU1] anther: Worker thread TID=37 starting for conn_handle=3
[67319596047] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[67323423651] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[67346633079] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[67359587856] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=33, our_read=34)
[67361630589] [INFO] [anther] [CPU1] anther: Worker TID=37 connected to netd OK
[67488120810] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[67558338573] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=663 watches=13 history=1024 journal=1024 symbols=313 drops=0
[67617236544] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[67745822265] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[67986801003] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[68147261853] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[68303062113] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[68376064053] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[68470538433] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[68639728080] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[68730766236] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[68735517939] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[68737117548] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[68738111772] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[68739173481] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[68740382040] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[68754285963] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[68755281903] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[68756364072] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[68757717798] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=337 pred=0 subj_lo=0
[68850050049] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[68998804545] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=711 watches=15 history=1024 journal=1024 symbols=338 drops=0
[69009624222] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[69201795432] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[69358701225] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[69391744521] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 85 bytes on conn_handle=3
[69399066990] [INFO] [anther] [CPU1] anther: GET /health Http11
[69434763783] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[69437229807] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[69446466441] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[69448503531] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[69453915465] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[69455319351] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[69457190220] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[69464903145] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[69466536447] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[69476868615] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[69481045194] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[69488036409] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[69495421347] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[69496895853] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[69499464375] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[69506420016] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[69507866472] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[69510229074] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[69513027408] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[69541636692] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[69603598185] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[69613859403] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[69614963847] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[69616126338] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[69617354466] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=348 pred=0 subj_lo=0
[69679509240] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=fec1d29ef0678173)
[69682202106] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[69798118797] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[69876942333] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[69916861311] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[69928461075] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[69992948751] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[70030962705] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[70100262639] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[70114900218] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[70129932279] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12232000
[70131369495] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[70133097045] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[70193813448] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[70203558447] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[70207959624] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[70209920022] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[70212206196] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[70216103925] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[70227566442] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[70237136904] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[70238504061] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[70240040937] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[70255251825] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[70257285516] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[70259723490] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[70261532418] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=1 (16384b)
[70266082524] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x122c3000
[70267416351] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[70326715074] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=c1e9a7b90a789e5c)
[70566922866] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[70776148344] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[71012124084] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[71114276970] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=789 watches=16 history=1024 journal=1024 symbols=363 drops=0
[71150401509] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[71181292644] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[71280774444] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[71293806276] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[71323088496] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[71372467089] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[71373678321] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[71374872756] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[71376147975] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=349 pred=0 subj_lo=0
[71385832980] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[71487670947] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[71488844955] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[71490025596] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[71491285305] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=350 pred=0 subj_lo=0
[71496917943] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[71520051339] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[71521099188] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[71522177727] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[71523591282] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=351 pred=0 subj_lo=0
[71627062782] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[71709627759] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[71715421404] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[71806259349] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[71919028236] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[72002229915] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=1
[72072029271] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[72125601999] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[72216394734] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[72255113733] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[72256887582] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[72315084171] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[72409402263] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[72464195562] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[72513182610] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[72590975160] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[72676466016] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[72764514141] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[72865895025] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[72882717402] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[72950491416] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[73043668269] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[73146314637] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[73230384711] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[73313487258] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[73324855131] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[73700227062] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[73732393317] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[74067163731] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[74068648962] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[74071443435] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[74080567869] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[74083024455] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[74086718640] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[74104132113] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[74128925772] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[74152195029] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 0 bytes on conn_handle=3
[74571559821] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[74614354584] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[75064696410] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[75088080111] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[75546831261] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=950 watches=19 history=1024 journal=1024 symbols=367 drops=0
[75832918194] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2429 ops=1 watches=19
[76075372098] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[76098315678] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[76718198436] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[76743491649] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[77574060630] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[77579124414] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[78115847865] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (1 fonts)
[78146048343] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[78540157245] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[78702416001] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[78712644120] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[79253748585] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[79273961778] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[80644457058] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=1137 watches=19 history=1024 journal=1024 symbols=398 drops=0
[83546428911] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[83696300919] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[83744620014] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[84119067054] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[84121664022] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[84159936993] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[84885473277] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[84950144268] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[84951270459] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[84952462749] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[84953740806] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[84966807948] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[84967945821] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[84969145635] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[84970204176] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=236 pred=0 subj_lo=0
[84979785891] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[84980801697] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[84981774207] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[84982885482] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=453 pred=0 subj_lo=0
[84990344769] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[84991288569] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[84992291571] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[84993476436] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=452 pred=0 subj_lo=0
[85004499756] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[85005471276] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[85006484079] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[85007700426] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[85017873105] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[86985605652] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=1382 watches=24 history=1024 journal=1024 symbols=454 drops=0
[90490870140] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=3453 ops=1 watches=24
[91702198797] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=1531 watches=24 history=1024 journal=1024 symbols=454 drops=0
[96043020909] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1655 watches=24 history=1024 journal=1024 symbols=454 drops=0
[101032721427] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1796 watches=24 history=1024 journal=1024 symbols=454 drops=0
[106815077292] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1934 watches=24 history=1024 journal=1024 symbols=454 drops=0
[113893705794] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=2075 watches=24 history=1024 journal=1024 symbols=454 drops=0
[119754164277] [INFO] [kernel::root::handl
```
</details>
