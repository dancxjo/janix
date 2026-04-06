# ❌ Scenario: Serve telnet connection

> Last run: 2026-04-05 19:22:36

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the telnet server is ready | ✅ | 10031ms | - - - |
| 2 | When I connect to the telnet server and send "match (n) return n;" | ✅ | 3708ms | - [📜](./02/serial.log) - |
| 3 | Then the telnet response should contain "node(" | ❌ | 1014ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12969677094] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[12976004448] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[12979805553] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[12981834954] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[12983105058] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[12983765058] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[12984433374] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[12985015329] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[12985639029] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[12986305728] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=33264
[12986899728] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=969224
[12987501615] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[12988217979] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[12988907415] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[12989593188] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[12990229593] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[12990883092] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[12991485672] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[12992159961] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[12992872596] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[12993573318] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[12994165833] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[12994819629] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[12995445210] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[12996087225] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[12996738645] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[12997411383] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[12998064486] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[12998678187] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[12999436692] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[13000284000] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[13000912848] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[13001612349] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[13002350823] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=168464
[13002959079] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[13003661880] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[13004368179] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[13005130677] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[13005828693] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[13006550469] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[13007253996] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[13007964255] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[13009376688] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[13010879937] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[13011677547] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[13012226667] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[13012734834] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[13013252604] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[13013794563] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[13014313224] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[13014880758] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[13015439382] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[13016027409] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[13016660778] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7795f000 (Usable)
[13017212373] [INFO] [kernel::memory] [CPU0]   [11] 0x7795f000 - 0x779c3000 (Reserved)
[13017778092] [INFO] [kernel::memory] [CPU0]   [12] 0x779c3000 - 0x779c4000 (Other)
[13018343448] [INFO] [kernel::memory] [CPU0]   [13] 0x779c4000 - 0x779c5000 (Reserved)
[13018950813] [INFO] [kernel::memory] [CPU0]   [14] 0x779c5000 - 0x779c6000 (Other)
[13019497887] [INFO] [kernel::memory] [CPU0]   [15] 0x779c6000 - 0x779c7000 (Reserved)
[13020135315] [INFO] [kernel::memory] [CPU0]   [16] 0x779c7000 - 0x779c8000 (Other)
[13020871215] [INFO] [kernel::memory] [CPU0]   [17] 0x779c8000 - 0x779c9000 (Reserved)
[13021467327] [INFO] [kernel::memory] [CPU0]   [18] 0x779c9000 - 0x77a54000 (Other)
[13022071986] [INFO] [kernel::memory] [CPU0]   [19] 0x77a54000 - 0x77a55000 (Reserved)
[13022779638] [INFO] [kernel::memory] [CPU0]   [20] 0x77a55000 - 0x77ed6000 (Other)
[13023443103] [INFO] [kernel::memory] [CPU0]   [21] 0x77ed6000 - 0x77ed7000 (Reserved)
[13024014036] [INFO] [kernel::memory] [CPU0]   [22] 0x77ed7000 - 0x77ff8000 (Other)
[13024559955] [INFO] [kernel::memory] [CPU0]   [23] 0x77ff8000 - 0x77ff9000 (Reserved)
[13025246685] [INFO] [kernel::memory] [CPU0]   [24] 0x77ff9000 - 0x787f9000 (Other)
[13025796432] [INFO] [kernel::memory] [CPU0]   [25] 0x787f9000 - 0x787fa000 (Reserved)
[13026603150] [INFO] [kernel::memory] [CPU0]   [26] 0x787fa000 - 0x788bb000 (Other)
[13027329150] [INFO] [kernel::memory] [CPU0]   [27] 0x788bb000 - 0x788bc000 (Reserved)
[13027900050] [INFO] [kernel::memory] [CPU0]   [28] 0x788bc000 - 0x788e6000 (Other)
[13028470521] [INFO] [kernel::memory] [CPU0]   [29] 0x788e6000 - 0x788e7000 (Reserved)
[13029103890] [INFO] [kernel::memory] [CPU0]   [30] 0x788e7000 - 0x788ec000 (Other)
[13029653373] [INFO] [kernel::memory] [CPU0]   [31] 0x788ec000 - 0x788ed000 (Reserved)
[13030216155] [INFO] [kernel::memory] [CPU0]   [32] 0x788ed000 - 0x788f2000 (Other)
[13030762833] [INFO] [kernel::memory] [CPU0]   [33] 0x788f2000 - 0x788f3000 (Reserved)
[13031353863] [INFO] [kernel::memory] [CPU0]   [34] 0x788f3000 - 0x7891f000 (Other)
[13032121080] [INFO] [kernel::memory] [CPU0]   [35] 0x7891f000 - 0x78921000 (Reserved)
[13032875097] [INFO] [kernel::memory] [CPU0]   [36] 0x78921000 - 0x7892a000 (Other)
[13033429794] [INFO] [kernel::memory] [CPU0]   [37] 0x7892a000 - 0x7892c000 (Reserved)
[13033995546] [INFO] [kernel::memory] [CPU0]   [38] 0x7892c000 - 0x78934000 (Other)
[13034642049] [INFO] [kernel::memory] [CPU0]   [39] 0x78934000 - 0x78935000 (Reserved)
[13035391413] [INFO] [kernel::memory] [CPU0]   [40] 0x78935000 - 0x7893f000 (Other)
[13036203246] [INFO] [kernel::memory] [CPU0]   [41] 0x7893f000 - 0x78940000 (Reserved)
[13037036133] [INFO] [kernel::memory] [CPU0]   [42] 0x78940000 - 0x7894d000 (Other)
[13037731410] [INFO] [kernel::memory] [CPU0]   [43] 0x7894d000 - 0x7894f000 (Reserved)
[13038427248] [INFO] [kernel::memory] [CPU0]   [44] 0x7894f000 - 0x7895d000 (Other)
[13038985476] [INFO] [kernel::memory] [CPU0]   [45] 0x7895d000 - 0x7895e000 (Reserved)
[13039551789] [INFO] [kernel::memory] [CPU0]   [46] 0x7895e000 - 0x7896a000 (Other)
[13040102031] [INFO] [kernel::memory] [CPU0]   [47] 0x7896a000 - 0x7896b000 (Reserved)
[13040672040] [INFO] [kernel::memory] [CPU0]   [48] 0x7896b000 - 0x7896f000 (Other)
[13041241488] [INFO] [kernel::memory] [CPU0]   [49] 0x7896f000 - 0x78970000 (Reserved)
[13041806118] [INFO] [kernel::memory] [CPU0]   [50] 0x78970000 - 0x78980000 (Other)
[13042352565] [INFO] [kernel::memory] [CPU0]   [51] 0x78980000 - 0x78981000 (Reserved)
[13042952802] [INFO] [kernel::memory] [CPU0]   [52] 0x78981000 - 0x78a11000 (Other)
[13043781036] [INFO] [kernel::memory] [CPU0]   [53] 0x78a11000 - 0x78a12000 (Reserved)
[13044601614] [INFO] [kernel::memory] [CPU0]   [54] 0x78a12000 - 0x78a1d000 (Other)
[13045241319] [INFO] [kernel::memory] [CPU0]   [55] 0x78a1d000 - 0x78a1e000 (Reserved)
[13045814859] [INFO] [kernel::memory] [CPU0]   [56] 0x78a1e000 - 0x78a43000 (Other)
[13046362527] [INFO] [kernel::memory] [CPU0]   [57] 0x78a43000 - 0x78a44000 (Reserved)
[13046928312] [INFO] [kernel::memory] [CPU0]   [58] 0x78a44000 - 0x78a50000 (Other)
[13047554883] [INFO] [kernel::memory] [CPU0]   [59] 0x78a50000 - 0x78a51000 (Reserved)
[13048244715] [INFO] [kernel::memory] [CPU0]   [60] 0x78a51000 - 0x78a5e000 (Other)
[13048799940] [INFO] [kernel::memory] [CPU0]   [61] 0x78a5e000 - 0x78a5f000 (Reserved)
[13049369817] [INFO] [kernel::memory] [CPU0]   [62] 0x78a5f000 - 0x78aa7000 (Other)
[13050005265] [INFO] [kernel::memory] [CPU0]   [63] 0x78aa7000 - 0x78aa8000 (Reserved)
[13051154490] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[13305205386] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485751 free frames
[13316900751] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[13321911306] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[13323392214] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[13324386372] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[13329058116] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[13331381910] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[13332531795] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[13333223574] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[13333946439] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[13334666466] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[13335718110] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[13336701708] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[13337424243] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[13338140574] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[13338828492] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[13339512615] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[13340752722] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[13341847596] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[13342562739] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[13344253263] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[13345223826] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[13346236299] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[13347958833] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[13349730207] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[13350719613] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[13351306650] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[13352241474] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[13751686245] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[13752784617] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[13756395609] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[13757284761] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[13758058842] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[13759691715] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[13774330746] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[13775776707] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[13776679884] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[13778803962] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[13779473763] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[13782126039] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[13790906118] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[13793023563] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[13808428326] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[13809177162] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[13826735439] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[13827422796] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[13829627427] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[13830939441] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[13832135328] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[13834436121] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[13835323887] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[13871341737] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62724400 ticks/sec), init_cnt=627244 for 100Hz
[13873764861] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[13875008862] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[13876780071] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[13884077691] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[13914470790] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[13915508112] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[13917852135] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[13919121348] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[13920213549] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[13923974130] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[13925425734] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[13943830098] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[13945430136] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[13946235270] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[13947121650] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[13947834846] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[13949383470] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[13950138312] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[13975597779] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[13977079446] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[13978447824] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[13979635923] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[13980916818] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[13982218272] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[13983179133] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[13984406832] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[13991806125] [INFO] [kernel::root] [CPU0] Spawning Root service...
[13992957396] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[13995003000] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[13995850242] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[13999925676] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[14001847629] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[14002685730] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[14004646755] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[14006317941] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[14007804657] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[14028114540] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[14033279403] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[14034601449] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[14035521324] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[14062070022] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14085373863] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14087819328] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14092529616] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14094925713] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14097533043] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14100899637] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14104102122] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[14105292597] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[14106837789] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14115600114] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14117906880] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14121607401] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14124746427] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14127939738] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14131328046] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[14132173440] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[14149767555] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[14150895924] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[14161836843] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[14162749953] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[14200579800] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[14201753676] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[14614571268] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[15301162866] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[15341112105] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=499 journal=424 symbols=51 drops=0
[15391051896] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[17769744135] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=961 journal=769 symbols=95 drops=0
[19411824234] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[19542508293] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[19544032497] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[19681148355] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[19774456086] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[19805844201] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[19807019628] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[19807787010] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[19811879604] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[19830985053] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[19851250683] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[19853772576] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[19934321979] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[19953991959] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[19954932327] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[19959362643] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[19988618859] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[20013893493] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[20018766306] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[20020172304] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[20095043958] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[20096883081] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[20219186724] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[20309076480] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[20322261003] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[20327796126] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[20330054349] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[20341627581] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[20417507286] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[20423092470] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[20424597435] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[20425920042] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[20427356334] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[20428225620] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[20429031051] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[20429723061] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[20431746786] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[20432497503] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[20433153774] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=33264
[20433782622] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=969224
[20434427739] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[20435088894] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[20435806050] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[20436542214] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[20437191027] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[20437854195] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[20438486574] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[20439164988] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[20439877689] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[20440510332] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[20441232141] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[20442072354] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[20442831486] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[20443671336] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[20444418951] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[20445124689] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[20445843990] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[20446475643] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[20447120694] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[20447767791] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[20448422577] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[20449140888] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[20449807521] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=168464
[20450507649] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[20451512400] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[20452487319] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[20453415972] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[20454265062] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[20455252818] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[20456123754] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[20456888067] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[20458559583] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[20460649803] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[20461847604] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20470565445] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[20487174972] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[20492871168] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[20503197363] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[20503933494] [CONTRACT] [kernel] [CPU0] Spawning init process...
[20506468290] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[20509381101] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[20510232435] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [20517260643] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013520 RFLAGS_BEFORE=130 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[20522176521] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[20541186336] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[20544291405] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[20545977606] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[20547088881] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[20557595223] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[20563518855] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[20567272110] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[20568615606] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[20573756907] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[20577820758] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[20579033706] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[20584185006] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[20585295588] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[20586395742] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[20587672182] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[20593310991] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[20594531892] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[20596042764] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[20597717184] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[20599113216] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[20600456019] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[20605942632] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[20652197775] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[20661165921] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[20667301083] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[20672738427] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[20676266919] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[20681118447] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[20686393299] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[20693261061] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[20700139746] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[20706873198] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[20714338392] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[20724515295] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[20732922078] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[20739879204] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[20746812438] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[20751845301] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[20759135166] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[20767245741] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[20773391595] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[20779738320] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[20786732142] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[20793478629] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[20799535317] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[20805778290] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[20812718421] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[20820684258] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[20827230831] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[20833221915] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[20839279923] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[20845340208] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[20850207972] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[20854384749] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[20860340061] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[20863964649] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/telnetd
[20867850003] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[20875015062] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[20880890415] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[20887314657] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[20893546740] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[20900980089] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[20907956487] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[20914047099] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[20918787516] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[20952622251] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[21120974259] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[21128822847] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[21129697380] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21131725230] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21136784460] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21138172011] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[21147542262] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[21152955153] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[21154206744] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21155937990] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079056 RFLAGS_BEFORE=130 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[21161484564] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[21165326325] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[21166351602] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[21168375327] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21174052911] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[21176507517] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[21184909317] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[21190803315] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[21191960559] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[21194499216] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[21195657219] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144592 RFLAGS_BEFORE=130 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[21203734068] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[21205521183] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[21207666447] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21212741913] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 02:23:04 = 1775442184 unix_secs
[21214455669] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775442184, mono_ns=10607001702, offset=1775442173392998298ns
[21216394749] [INFO] [rtc_cmos] [CPU1] System clock anchored
[21231641409] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[21285060951] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[21295987482] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[21297298341] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[21300086775] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21309517779] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[21313149297] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[21324644847] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[21329982300] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[21333951507] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21336250287] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210816 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[21343862364] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[21350761542] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[21353409891] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[21354587991] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[21356839053] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21364565244] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21367601838] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[21374956053] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[21378505005] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[21380365611] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21382164837] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[21383382009] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277568 RFLAGS_BEFORE=130 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[21389866773] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[21392490108] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[21402169041] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[21402998859] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[21404235402] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[21406221507] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[21407516625] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[21421783812] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[21424176444] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[21995460960] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22007253279] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[22013016993] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[22015974981] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[22023008832] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[22025839110] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[22029296157] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[22031528244] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[22033103202] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[22036880679] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[22038430029] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[22039945356] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[22041315846] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[22042749828] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[22044005214] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[22045149753] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[22052077674] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[22053480471] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[22056163470] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22067503161] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22071167019] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[22080744048] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[22085255346] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[22087539771] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352752 RFLAGS_BEFORE=134 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[22090898247] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
[22094926986] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[22110523578] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[22126246989] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[22138720197] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[22141490712] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[22147940793] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[22157949000] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[22160636916] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[22166454387] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[22168084092] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[22169640339] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[22170903579] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[22172210973] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[22173491538] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[22179446388] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[22180709694] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[22184101434] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[22189196106] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[22192341831] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[22195237746] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[22199118876] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[22200042645] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22201978590] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22207030791] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[22209153648] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22218095361] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[22221745359] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[22223733642] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[22224498417] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22226480529] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22232795343] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22234489365] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[22243467939] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[22247647785] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[22249088202] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[22251040053] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500528 RFLAGS_BEFORE=130 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[22256072355] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[22257489540] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22260157293] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22261403208] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[22263209100] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[22267942026] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[22269193386] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[22271264268] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[22272145599] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22274753127] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[22276955316] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[22278561954] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[22280339334] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[22281989796] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[22283175354] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[22285841622] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[22287878646] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[22288765818] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[22291854882] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[22293826170] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[22297039743] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[22299458478] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434448 RFLAGS_BEFORE=134 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[22312164765] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[22726786698] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[22727635161] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
[22729570842] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[22731515994] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[22732573314] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583312 RFLAGS_BEFORE=130 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[22739565948] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22742866311] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[22743711177] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22746381570] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22756276125] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[22759819005] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[22768903443] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[22772992011] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[22775388075] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[22778731932] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[22781281083] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[22784172972] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[22786826139] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[22788102810] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22790822241] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22800212853] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[22820053707] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[22826567247] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[22915828287] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[23765427081] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[23769447669] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[23770932570] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23772767997] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369717008 RFLAGS_BEFORE=130 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[23778805314] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[23781110067] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369649680 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[23785548963] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[23787600837] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[23788773591] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[23791080258] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[23799889674] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[23801945772] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[23803700382] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[23807015298] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[23812357338] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using cooperative polling loop (2ms interval)
[23814710865] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[23816328426] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[23817935295] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[23818791645] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23820343569] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23892668943] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[23918244834] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[23926614360] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[23930286963] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[23931285279] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23933039328] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369783232 RFLAGS_BEFORE=134 CR3_BEFORE=79892480 fs_base=0 gs_base=18446744071564586576
[23937345432] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[23938570755] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23941038066] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23941942860] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[23946074427] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[23950896717] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[23952335055] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[23968109055] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[23972190231] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[23980591041] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[23983836228] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[23986827546] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[23988181437] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23990743326] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23997247560] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[24000407607] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[24008892501] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[24012136896] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[24013453464] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24015049377] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915392 RFLAGS_BEFORE=134 CR3_BEFORE=80826368 fs_base=0 gs_base=18446744071564586640
[24019080129] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[24019995021] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24021737058] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24022648485] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[24024879186] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[24031284651] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[24037451592] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[24049615788] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[24054555954] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[24055932681] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24057782793] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981792 RFLAGS_BEFORE=134 CR3_BEFORE=80953344 fs_base=0 gs_base=18446744071564586576
[24063774438] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[24064940658] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24067358106] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24079740201] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[24083515533] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[24091933404] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[24095837469] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[24098223831] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[24098991576] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24100966164] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24129926832] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[24135330384] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[24143597775] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[24147359775] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cc18
[24148411320] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24149849361] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113952 RFLAGS_BEFORE=134 CR3_BEFORE=81256448 fs_base=0 gs_base=18446744071564586640
[24153074352] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[24153844275] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24155035245] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[24155933868] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24174353841] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[24178147785] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[24179765214] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[24185043993] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[24187615683] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24189883278] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[24191880042] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[24193610694] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[24197531490] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[24199527759] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24201330384] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370198032 RFLAGS_BEFORE=130 CR3_BEFORE=81530880 fs_base=0 gs_base=18446744071564586576
[24206318499] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[24207374466] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24209329320] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24210337503] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[24212129733] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[24222971784] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24224919378] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24226705107] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24228637026] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[24230678703] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[24232203930] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[24234280191] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[24237402189] [INFO] [ps2_mouse] [CPU3] ps2_mouse: using cooperative polling loop (2ms interval)
[24244404558] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[24246078582] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24248031852] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[24250006737] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[24254666040] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[24266004444] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[24275046081] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[24278961036] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[24283428510] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24285193878] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849312 RFLAGS_BEFORE=130 CR3_BEFORE=80564224 fs_base=0 gs_base=18446744071564586608
[24290226642] [INFO] [nectar] [CPU2] NECTAR: Started.
[24292708242] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24294486216] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047872 RFLAGS_BEFORE=130 CR3_BEFORE=81100800 fs_base=0 gs_base=18446744071564586608
[24299098593] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24300714801] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[24301711500] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24303130962] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24304410570] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[24320064252] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24322983927] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370270944 RFLAGS_BEFORE=130 CR3_BEFORE=81739776 fs_base=0 gs_base=18446744071564586608
[24327611154] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[24329697249] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24330922803] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[24331831260] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[24333696123] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=233 pred=0 subj_lo=0
[24335444331] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[24337468386] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[24341208441] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[24343031658] [INFO] [fontd] [CPU3] FONTD: Service ready
[24352337955] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[24380688717] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[24383006769] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24385217076] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24386863776] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24388584858] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[24390299967] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=235 subj_lo=0
[24421556907] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1251 backend=VirtIO-GPU
[24423438138] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[24425344020] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[24426385500] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[24427575942] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24430268808] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24440902332] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24442304898] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24443730597] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24445325751] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[24453178431] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24454346565] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24455785959] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24457321746] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[24465660483] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24467148981] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24468941772] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24470411724] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[24485125698] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24486636900] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24488186943] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24490015407] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[24546670566] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[24586628154] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[24588111933] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[24596531355] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[24598371171] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[24630682494] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d5000 exec=false
[24652245618] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2eb000 exec=false
[24665905539] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[24669609030] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db488
[24680740821] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e3
[24683221002] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370357952 RFLAGS_BEFORE=134 CR3_BEFORE=82391040 fs_base=0 gs_base=18446744071564586640
[24693676128] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[24698462448] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24700987344] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[24702029847] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24709003902] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[24712179987] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[24724039758] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[24728325468] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db488
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[24730442352] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[24731366484] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370423744 RFLAGS_BEFORE=134 CR3_BEFORE=83443712 fs_base=0 gs_base=18446744071564586576
[24736213227] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[24737947938] [INFO] [echo] [CPU1] echo: starting up
[24741329712] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[24753963861] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[24785288418] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[24787373754] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f5000
[24789444867] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f5000
[24790889211] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f8000
[24793470735] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276787200)
[24795174492] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[24799330578] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f9000 phys=0x5035000
[24801549201] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[24805878372] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[24807686541] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[24810203220] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[24811354953] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[24812901762] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[24825899505] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[24832399977] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[24835483926] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[24842640075] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[24843838404] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[24856136547] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[24858589107] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[24860004015] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[24861337281] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[24865158582] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[24868480395] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[24871114818] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[24873293610] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[24875696406] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[24877188930] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[24878841966] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[24880027722] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[24882264396] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[24884633664] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[24892543797] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[24894218811] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[24899282661] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[24900594741] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[24910074288] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[24911853450] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[24913448472] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[24914232981] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24916033659] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24921322272] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[24923285112] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[24931101921] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[24934847223] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[24937508508] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[24939192432] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[24940150488] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[24941298294] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[24942371421] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24944257272] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[24945867111] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24955171725] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[24957753315] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[24960997446] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[24963336651] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[24975427983] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[24976906449] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[24980252517] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[24981591327] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24983563737] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[24984965049] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370711824 RFLAGS_BEFORE=130 CR3_BEFORE=84348928 fs_base=0 gs_base=18446744071564586640
[24989752392] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[24990893301] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24993583989] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24998168679] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[24999890091] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[25011795336] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[25013542851] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[25017755961] [INFO] [bloom] [CPU3] bloom: creating surface...
[25018749030] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[25020202713] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[25021296960] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/telnetd'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[25023196374] [INFO] [bloom] [CPU3] bloom: surface created!
[25024050249] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370777360 RFLAGS_BEFORE=130 CR3_BEFORE=84496384 fs_base=0 gs_base=18446744071564586576
[25027888512] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[25029068955] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/telnetd
[25029912897] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[25032097992] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[25039821576] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[25063469706] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[25065056973] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=220000 exec=false
[25072225464] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=228000 exec=false
[25073348883] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[25081946010] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 32 (user task/process) assigned to CPU 2
[25085567892] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=32)
[25087006725] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[25094682624] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[25100846298] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010e3f0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[25102617474] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=32 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370842896 RFLAGS_BEFORE=130 CR3_BEFORE=84615168 fs_base=0 gs_base=18446744071564586608
[25110466656] [INFO] [telnetd] [CPU2] telnetd: starting on port 2323
[25115316699] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1261
[25117485987] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[25135679085] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[25140574701] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=33)
[25142504904] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[25144307430] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0AF0 [25154804334] [INFO] [telnetd] [CPU2] telnetd: waiting for network stack
[25156341705] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[25170267309] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[25173933048] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[25175047590] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[25185212811] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[25189371603] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[25193833434] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 3)
[25198254642] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[25202871573] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[25204449237] [INFO] [anther] [CPU1] anther: Waiting for network stack...
T:1050 T:0EC0 T:F930 [25229314572] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1261
[25235255628] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[25270192596] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[25273008321] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[25291061895] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[25292471556] [INFO] [telnetd] [CPU2] telnetd: waiting for network stack
[25301068320] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 37 (user thread) assigned to CPU 3
[25304327433] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=37 (priority=2)
[25308189357] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[25312898853] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[25320566766] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=266
[25325654970] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[25326987114] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[25328143632] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25329238770] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25330506597] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=266 subj_lo=0
T:1AA0 [25342638420] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[25344838200] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[25362786834] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1273)
[25365059709] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[25372536849] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[25376692011] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[25382876541] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[25404843387] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=244
[25407096231] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[25408655514] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[25410237435] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25411954194] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25413805461] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[25436217147] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1275)
[25438913907] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[25456828980] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=277
[25458579696] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[25459764957] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[25460926590] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25462300248] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25463810724] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=277 subj_lo=0
[25476322278] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1276)
[25478834964] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=77c80b059e863710)
[25480815228] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[25530748155] [INFO] [telnetd::net_client] [CPU2] telnetd: connected to socket API (netd=27, write=29, read=30)
[25536769764] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([250, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[25546955775] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[25548606402] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[25600894341] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[25602177777] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[25609911921] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[25654707540] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[25717560462] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=31, our_read=32)
[25720490334] [INFO] [anther] [CPU1] anther: Connected to network stack
[26053711101] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[26083508748] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=644 watches=13 history=1024 journal=1024 symbols=298 drops=0
[26097164544] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[27226258191] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[27229435035] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[27335267190] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[27465833670] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[27897391533] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[27940719246] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[27944065446] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[28321185915] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=670 watches=13 history=1024 journal=1024 symbols=336 drops=0
[28644076263] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[28650905415] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[28678380885] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[28680959703] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[28840292283] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[28841982873] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[28843629375] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28845436554] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=295 pred=0 subj_lo=0
[29062815573] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[29162765214] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29199650337] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[29211896274] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[29214051042] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[29239888194] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[29241325278] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[29243058636] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29244614883] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=296 pred=0 subj_lo=0
[29262824712] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[29331777024] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[29333062737] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[29334165432] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29335383363] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=297 pred=0 subj_lo=0
[29429201835] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[29430386403] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[29432785866] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29434468074] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=336 pred=0 subj_lo=0
[29504937264] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[29562596283] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12001000
[29565271857] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[29567564301] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[29670320229] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[29702588685] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[29710164363] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[29714271774] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[29717080305] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[29723482338] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[29742733911] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[29744608212] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[29746768590] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[29767032636] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x12002000
[29769621849] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[29816423373] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[30309028563] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[30310733112] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[30313387170] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[30364884495] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[30366643824] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[31155430647] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=715 watches=17 history=1024 journal=1024 symbols=354 drops=0
[32013079131] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[32018548419] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[32021266464] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[32369927205] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[32396714064] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[32419214487] [INFO] [netd] [CPU3] NETD: DHCP configured �� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[32427086637] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[32434443624] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 2323 with backlog 64
[32439208494] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 2323, handle=1
[32442033294] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[32443076325] [INFO] [telnetd] [CPU2] telnetd: listening on guest port 2323 (handle=1)
[32444532780] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=2
[32446576998] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[32448230760] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=3
[32450494758] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=2)
[32451662430] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[32453743641] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=4
[32455763538] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[32457448881] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=5
[32484255969] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[32487597747] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[32551732818] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[32573585550] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[32595619617] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[32598193815] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[32602500447] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[32610554262] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[32621169669] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=70
[32623177554] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[32625506331] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[32629670271] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[32632097223] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[32649684606] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[32667237900] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=74
[32669786556] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=25
[32672981616] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[32674540173] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[32681150700] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[32684868414] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[32714778855] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[32719031037] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[32801836056] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=70
[32804099361] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[32809035204] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[32930325945] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[32937664914] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[32940646827] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:44518 on listener 1
[33119885304] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[33122140260] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[33201250203] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[33205806678] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[33213598275] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 60 bytes - TCP ACK
[33286900977] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[33289444023] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[33407844624] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=742 watches=17 history=1024 journal=1024 symbols=359 drops=0
[33883467981] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[34382397456] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[34937760099] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[35077126029] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 80 bytes - TCP ACK
[35378191926] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=756 watches=17 history=1024 journal=1024 symbols=361 drops=0
[35961641232] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[35963399406] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[35965813719] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[35967902124] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 80 bytes
[37238068329] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[37240836171] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 115 bytes - TCP ACK
[37457365539] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[37458921852] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: TCP_CLOSE handle=6 (initiating close)
[37460485788] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) tonetd rx_port=25
[37463339397] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[37464847200] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[37466084601] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 115 bytes
[37467753708bytes - TCP FIN
[37492045932] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[37494120840] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[37497148788] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[37499691966] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 63 bytes
[37623489486] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[37626291714] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[37627673886] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:44530 on listener 1
[38030555640] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 60 bytes - TCP ACK
[38198295366] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[38200216428] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[38203238700] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[38205312783] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 60 bytes
[38214532389] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[38220233403] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: GC removed explicitly closed TCP socket
[38254964616] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[38257576896] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[38260928211] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[38271006312] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=85
[38274298392] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 75 byte frame (79 encoded) to netd rx_port=25
[38277992214] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (79 bytes sent)
[38288011212] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[38291071005] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 75 bytes
[38295963222] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 118 bytes - TCP ACK
[38319098565] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[38413408902] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 118 bytes
[38475172494] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=776 watches=17 history=1024 journal=1024 symbols=361 drops=0
[39464936280] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[39467183481] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[39469940169] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[39871390911] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[39874349790] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 57 bytes - TCP ACK
[39994875129] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 57 bytes
[40857939837] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=793 watches=17 history=1024 journal=1024 symbols=361 drops=0
[42341095500] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[42343186842] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[42345886737] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[42745677876] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[42748970550] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 59 bytes - TCP ACK
[42766788900] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[42904747677] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 59 bytes
[43127385543] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[43129913508] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=2 (16384b)
[43133072697] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[43135205652] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[43173206307] [INFO] [phloem::executor] [CPU2] phloem: entering discover_nodes
[43174708137] [INFO] [phloem::executor] [CPU2] phloem: starting BFS discovery from roots
[43191205728] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[43193687064] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[43197216942] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[43214538939] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[43218645096] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 67 bytes - TCP ACK
[43235862516] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 67 bytes
[43257597075] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[43259634759] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[43261943142] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[43263405702] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[43310801556] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[43741421691] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[44033958936] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=825 watches=17 history=1024 journal=1024 symbols=372 drops=0
[44082877443] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[44109015984] [INFO] [phloem::executor] [CPU2] phloem: BFS seeded with 515 nodes
[44633069844] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=671285c0c9ba41ed)
[44810678781] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=70
[44813256180] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[44815759659] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[44837301927] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[44840382708] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[44866561245] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[45089083578] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 33264 bytes, hash=91427bae3069f281)
[45632414289] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 969224 bytes, hash=bf04a3a0a7e7228b)
[46161798903] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=4e1c9cf03988a560)
[46602853473] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[46964600694] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[47402284347] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[47413702743] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=888 watches=17 history=1024 journal=1024 symbols=376 drops=0
[47419892553] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[47425165821] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[47427182946] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[47428709163] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[47430472815] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[47432764104] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[47460802686] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[47462267952] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[47464059192] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[47466129117] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=366 pred=0 subj_lo=0
[47961923295] [IN
```
</details>
