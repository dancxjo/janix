# ❌ Scenario: Boot produces logs, graphics, and a live desktop

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 3943ms | - [📜](./01/serial.log) - |
| 2 | Then I should see log messages on the terminal | ✅ | 503ms | - [📜](./02/serial.log) - |
| 3 | And each log message should include a monotonically increasing timestamp | ✅ | 0ms | - [📜](./03/serial.log) - |
| 4 | And I should see the system clock tick for several seconds in the serial console | ✅ | 3009ms | - [📜](./04/serial.log) - |
| 5 | And I should see the wallpaper on the screen within 60 seconds | ❌ | 1826ms | - [📜](./05/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12466307799] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[12471938985] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[12476352801] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[12478895814] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[12480821727] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[12481857696] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[12482981841] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[12484075494] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[12485060214] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[12486113343] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[12487142184] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[12488151390] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[12489308238] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[12490428588] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[12491568177] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[12492582432] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[12493645692] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[12494680011] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[12495723339] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[12496766535] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[12497747196] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[12498734919] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[12499759800] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[12500755278] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[12501811806] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[12502851108] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[12503857806] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[12504943143] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[12505955253] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[12506969244] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[12508002045] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[12509043855] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[12510131502] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[12511161531] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[12512169417] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[12513345636] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[12514517961] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[12515761500] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[12516934155] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[12518151129] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[12519355629] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[12520559040] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[12521937747] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[12523436739] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[12524759709] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[12525720933] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[12526581969] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[12527449671] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[12528401952] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[12529296714] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[12530156727] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[12531018687] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[12531875037] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[12532795143] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[12533700498] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[12534637533] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[12535564998] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[12536508996] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[12537421347] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[12538362507] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[12539297694] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[12540240636] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[12541150545] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[12542103783] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[12543021579] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[12543956700] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[12544860867] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[12545816052] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[12546719196] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[12547660422] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[12548571684] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[12549528024] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[12550440573] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[12551378202] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[12552305205] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[12553241943] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[12554143833] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[12555076347] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[12556005231] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[12556945830] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[12557853660] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[12558808350] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[12559713837] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[12560649816] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[12561559923] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[12562511841] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[12563423895] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[12564362613] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[12565297437] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[12566237871] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[12567143886] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[12568080162] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[12569005680] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[12569938260] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[12570841866] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[12571793421] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[12572701614] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[12573767316] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[12574682241] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[12575541363] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[12576123846] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[12576906639] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[12577824600] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[12578794899] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[12579717447] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[12580659795] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[12581575314] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[12582980124] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12851493006] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[12863473986] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12869502954] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12871213311] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12872445267] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12877542150] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12879900363] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12881040117] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12881895543] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12882602337] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12883408395] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12884464032] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12885956424] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12887097663] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12888256491] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12889394760] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12890517684] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12892248831] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12893641827] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12894803394] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12896908101] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12898287567] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12899736564] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12902112861] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12904164801] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12905347455] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12906246540] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12907361874] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[13290717990] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[13292141709] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[13295942649] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[13297454115] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[13298740884] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[13300699137] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[13314319755] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[13316158119] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[13317351993] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[13319517882] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[13320410235] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[13323316941] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[13331848563] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[13334446026] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[13349022456] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[13349933454] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[13366975182] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[13367890536] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[13370311317] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[13371965838] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[13374711834] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[13378524456] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[13379717604] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[13415113140] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62233700 ticks/sec), init_cnt=622337 for 100Hz
[13416669882] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[13417527750] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[13418777889] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[13424501937] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[13455375252] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[13456752012] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[13457792535] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[13459726731] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[13461389964] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[13466009403] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[13467962904] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[13490272620] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[13492268427] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[13493110290] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[13494094383] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[13495670496] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[13497651882] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[13498959837] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[13524421548] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[13525829592] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[13526764416] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[13528350363] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[13529262186] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[13530767052] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[13531368708] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[13532575947] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[13539700812] [INFO] [kernel::root] [CPU0] Spawning Root service...
[13540745394] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[13542563760] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[13543501917] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[13547696745] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[13549226790] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[13549933650] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[13551236787] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[13552312356] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[13553361822] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[13566048705] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[13569135591] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[13570453578] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[13571264322] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13594750323] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13613893689] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13615903950] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13619170521] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13620724953] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13623250443] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13625510052] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13627541334] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13628454642] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13629755139] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13636060548] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13637733648] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13640300619] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13642676355] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13646961207] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13647749412] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13648775052] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13666929342] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13667730813] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13678245207] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13679247879] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13718152272] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13719236454] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[14202894783] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[14924104602] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[14956820835] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=499 journal=424 symbols=51 drops=0
[14992131264] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[16467898953] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=961 journal=769 symbols=95 drops=0
[17322188301] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[17460932841] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[17462011182] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[17576569263] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[17650334361] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[17676471747] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[17677355058] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[17678011758] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[17681612322] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[17699793078] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[17716930539] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[17723018775] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[17776747494] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[17798258511] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[17799222408] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[17803142379] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[17829393021] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[17847559818] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[17854586805] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[17855644092] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[17921696034] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[17923674219] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[18009148410] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[18091817172] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[18104093568] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[18110019741] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[18112442172] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[18157819713] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[18197372127] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[18202282098] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[18203308134] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[18204606783] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[18205532532] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[18206223618] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[18206965986] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[18207671724] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[18208295391] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[18208929783] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[18209815206] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[18210551469] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[18211267371] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[18211950042] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[18212661984] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[18213812628] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[18214714551] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[18215550342] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[18216204567] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[18216871959] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[18217541397] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[18218168397] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[18218802426] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[18219439920] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[18220080945] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[18220786221] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[18221440083] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[18222088302] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[18222790377] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[18223429983] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[18224093745] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[18224761896] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[18225435921] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[18226096614] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[18226763577] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[18227423280] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[18228187494] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[18228956493] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[18229732686] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[18230504622] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[18231298866] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[18232069383] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[18232843365] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[18234379284] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[18236119209] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[18237071292] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18245446032] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[18260382129] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[18265830693] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[18275751219] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[18276472896] [CONTRACT] [kernel] [CPU0] Spawning init process...
[18278539620] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[18283634358] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[18284615811] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [18291522216] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[18292630719] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013216 RFLAGS_BEFORE=130 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[18315782199] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[18318807210] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[18320093649] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[18321366690] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[18330705855] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[18336762279] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[18340855005] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[18342623046] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[18348038115] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[18352684911] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[18354426816] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[18360154296] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[18361851321] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[18363715755] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[18365397336] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[18371287440] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[18373091220] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[18374999049] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[18377467053] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[18380147940] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[18382248753] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[18386607525] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[18432022884] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[18441117717] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[18449084148] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[18456263859] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[18461326851] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[18467519730] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[18476052441] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[18483493215] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[18491605176] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[18499750698] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[18508315815] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[18519246735] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[18527317974] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[18534523029] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[18542287104] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[18550856049] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[18558612270] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[18565498380] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[18573682809] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[18581485527] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[18588978606] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[18596663976] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[18604372479] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[18611917764] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[18619006131] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[18625983618] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[18632173098] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[18638830815] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[18645195195] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[18651859512] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[18657022989] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[18661747467] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[18669035616] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[18676052109] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[18683134734] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[18689914320] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[18697027965] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[18704052510] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[18711128964] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[18718715433] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[18726580818] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[18731449737] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[18758250984] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[18940852755] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[18951527496] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[18952803342] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18955591545] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18962015358] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18963540222] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18974751774] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[18982852119] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18984000750] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18985777371] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078752 RFLAGS_BEFORE=130 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[18992300514] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[18995505375] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[18996935826] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18999854874] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19006425405] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[19009013265] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19019964909] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[19024653054] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[19026195705] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[19028269062] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144288 RFLAGS_BEFORE=130 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[19033066866] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[19036871667] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[19039126194] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[19041649143] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19046848326] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:47:34 = 1775436454 unix_secs
[19049060745] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775436454, mono_ns=9524296864, offset=1775436444475703136ns
[19051460307] [INFO] [rtc_cmos] [CPU1] System clock anchored
[19063899195] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[19103192922] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[19129749837] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[19131009612] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19135163982] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19144443252] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[19148043354] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[19159940151] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[19165298889] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[19169917833] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19172112597] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210784 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[19178843178] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[19186179639] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[19191419412] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[19193872137] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[19195038225] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19197791283] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19208932776] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19212400845] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19223448519] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[19228837650] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[19230457257] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19232370861] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369276736 RFLAGS_BEFORE=130 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[19236874008] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[19239745833] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[19251791790] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[19252925076] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[19253981868] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[19255105089] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[19257023379] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[19274148630] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[19276412496] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[19843649001] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19863863844] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[19868342142] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[19871069295] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[19877872971] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[19880165778] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[19883362620] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[19885555272] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[19887498543] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[19891173324] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[19892663439] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[19894142928] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[19895476920] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[19896746034] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[19898017194] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[19899511104] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[19915432878] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[19916812839] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19919614110] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19930932153] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19934604393] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19945691931] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[19950197124] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[19951683576] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[19954333080] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369356960 RFLAGS_BEFORE=134 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[19964742336] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[19976937024] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[19994611428] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[20008873236] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[20011913592] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[20020059873] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[20033354253] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[20035352238] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[20042436348] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[20043736548] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[20044946460] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[20046094926] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[20047316916] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[20048585271] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[20050016448] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[20051637507] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[20053410234] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[20542079679] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[20546798745] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[20551140423] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[20553842958] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[20554984329] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20557703397] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20563391673] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[20566048668] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20577681993] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[20582388882] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[20585537511] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[20586699276] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20589128142] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20596707945] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20599024479] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20611729149] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[20616194280] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[20618543418] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[20620680234] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500688 RFLAGS_BEFORE=130 CR3_BEFORE=68521984 fs_base=0 gs_base=18446744071564586640
[20626587069] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[20629088865] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[20630462688] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[20631610362] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20634001179] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[20637197328] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20643925929] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20647490061] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20649840288] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[20651610045] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[20653551996] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[20656334490] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[20658091806] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[20659140612] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[20661153183] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[20663111337] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[20664149220] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[20665916667] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
[20667184395] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[20669245179] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[20670206403] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[20671311474] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583632 RFLAGS_BEFORE=130 CR3_BEFORE=68894720 fs_base=0 gs_base=18446744071564586576
[20676018792] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[20678934078] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb01006b0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[20681046111] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434896 RFLAGS_BEFORE=130 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[20685064257] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[20687338716] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[20689248228] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[20694043359] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[20703580458] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[20705701632] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[20708231544] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[20713129107] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[20714815737] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20717176854] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[20718081615] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20719586613] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20727127080] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[20730526080] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[20741104329] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[20744969982] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[20760180540] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[20762242347] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[20763220104] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20765440542] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20785727424] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20790300663] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[20810818908] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[20814160224] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
[20817300900] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[20818811739] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb01006b0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[20821244070] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716816 RFLAGS_BEFORE=134 CR3_BEFORE=69156864 fs_base=0 gs_base=18446744071564586640
[20826040950] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650608 RFLAGS_BEFORE=130 CR3_BEFORE=69021696 fs_base=0 gs_base=18446744071564586608
[20830153542] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[20831704707] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[20833459152] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[20835563727] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[20840496633] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[20843832933] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[20849729175] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[20852199060] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[20853925785] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[20856741972] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[20860380057] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[20863395531] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[20865882444] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[20866932834] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20869842675] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20894503278] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[20895682929] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[20897872083] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[20900008041] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[20961626466] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[20985787812] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[20993122425] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[20996517300] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb01006b0
[20998033056] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21000101001] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369782784 RFLAGS_BEFORE=134 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[21004566858] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[21005753109] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21008721690] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[21009887151] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21012563715] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[21019906479] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21022248786] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21041050899] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[21044474088] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[21052099068] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[21055217601] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[21057534168] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[21058414509] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21060275214] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21066232374] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[21069010281] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21076213191] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[21079219359] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010aff8
[21080666475] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21082469496] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915168 RFLAGS_BEFORE=130 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[21086921493] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[21087698610] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21089660229] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21090758568] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[21093162486] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[21098974710] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21102764199] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[21110343672] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[21113042907] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010aff8
[21114398250] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21116070360] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981712 RFLAGS_BEFORE=130 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[21120537603] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[21121412136] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21123770085] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21137351499] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[21141871014] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[21149250342] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[21151952646] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[21154109691] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[21154848231] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21156450381] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21181052970] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[21185918754] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[21193303725] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[21195925113] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010d510
[21197414370] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21199361304] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113760 RFLAGS_BEFORE=130 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[21205524219] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[21206675358] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21208667502] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[21209985489] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21235643022] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[21237147228] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[21241603779] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21243155340] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[21244397955] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21246035844] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21247882095] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[21255074742] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[21259740612] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[21261963921] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[21263441793] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[21264666258] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21267716184] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370199456 RFLAGS_BEFORE=134 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[21272538441] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21275824779] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[21278041950] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[21282215031] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21283874172] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21285543807] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21287124738] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21288837900] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21290555352] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21292473543] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=229 pred=0 subj_lo=0
[21294466314] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[21330269961] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[21345584931] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[21346908165] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21348869190] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21350592054] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21352126851] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21353848494] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21355428765] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21357157635] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[21358977618] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[21362253066] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[21367327806] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[21371541939] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
[21373145211] [INFO] [fontd] [CPU3] FONTD: Service ready
[21373805145] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb01006b0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21376150059] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849184 RFLAGS_BEFORE=134 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[21382967958] [INFO] [nectar] [CPU2] NECTAR: Started.
[21386551065] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010aff8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21389117772] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047968 RFLAGS_BEFORE=130 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[21395801262] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21396788292] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[21397902570] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21399280353] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21400820199] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=234 subj_lo=0
[21402956454] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010df80
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21405441915] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370267008 RFLAGS_BEFORE=130 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[21413675217] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[21416522292] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[21425277456] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21426942174] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21435370143] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[21437973876] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21439403832] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21440828838] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21442402179] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=235 subj_lo=0
[21445344360] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[21461194062] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21462504789] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21463757403] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21465066249] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=238 subj_lo=0
[21469912728] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[21475944666] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21477452007] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21479255424] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21481060458] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=240 subj_lo=0
[21493360416] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1251 backend=VirtIO-GPU
[21497194323] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[21498339456] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21500877057] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21506035122] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21507623346] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21509270937] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21510960438] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=242 pred=0 subj_lo=0
[21558705432] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[21585285282] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21587205189] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21603052548] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[21604576488] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[21705315654] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[21722555448] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21724068828] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21729445980] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[21740649414] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[21745444479] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f0fe8
[21747054879] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e3
[21749387583] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370364912 RFLAGS_BEFORE=134 CR3_BEFORE=72052736 fs_base=0 gs_base=18446744071564586640
[21752793381] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[21754002798] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21756854262] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21763700739] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[21765177753] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[21766041759] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21777078048] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[21781356234] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[21782544894] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f0fe8
[21783927990] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[21785837700] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370434544 RFLAGS_BEFORE=134 CR3_BEFORE=73101312 fs_base=0 gs_base=18446744071564586576
[21793306161] [INFO] [echo] [CPU1] echo: starting up
[21794068923] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[21795847128] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[21796887288] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[21810818205] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[21812865195] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[21826161060] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[21828579531] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[21833744130] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[21847174602] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[21849805164] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[21852638709] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[21876924960] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[21879428241] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f5000
[21882438996] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f5000
[21884801829] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f8000
[21889762356] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276787200)
[21892270917] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[21895399977] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f9000 phys=0x4656000
[21897468813] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[21900036642] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[21901238238] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[21904599156] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[21906867312] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[21909907767] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[21914685045] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21916461105] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21924205875] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[21938000106] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[21951896175] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[21956083545] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[21959372655] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[21961683183] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[21963848742] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[21966026313] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[21968198307] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[21969972585] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[21971931432] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[21979475001] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[21980811897] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[21982530867] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[21985595775] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[21988350945] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[21993820167] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[21995941902] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[21998636253] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[21999833427] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22002415809] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22010138700] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22013314059] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[22016208126] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22021863765] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[22024731465] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[22026850890] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[22028974275] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[22029901839] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[22032391095] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[22033456038] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22035805275] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22040913675] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[22046860044] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22052462289] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[22061702454] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[22064387070] [INFO] [bloom] [CPU3] bloom: creating surface...
[22065096768] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[22066544940] [INFO] [bloom] [CPU3] bloom: surface created!
[22068263448] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[22070213418] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[22071388944] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f11d8
[22072582884] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22074878364] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370708976 RFLAGS_BEFORE=134 CR3_BEFORE=73998336 fs_base=0 gs_base=18446744071564586640
[22078687059] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[22079933271] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22082343162] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22084540566] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[22087201818] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[22088948343] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[22093933158] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1261
[22095459507] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[22100285361] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[22103931465] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[22105638786] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[22108665381] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f11d8
[22115413716] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22119141099] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[22120296825] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[22121497926] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
[22126624641] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370774512 RFLAGS_BEFORE=134 CR3_BEFORE=74162176 fs_base=0 gs_base=18446744071564586576
[22132482570] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1261
[22133720235] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22135631463] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
T:0270 [22141748574] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[22154696289] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[22167042348] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[22180500705] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
T:07D0 T:0640 T:F0B0 [22229511513] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22283322006] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[22285214952] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[22300921599] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[22302076995] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[22304814378] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[22336183848] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=265
[22340843283] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[22342158300] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22343773584] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22345338675] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22347200040] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=265 subj_lo=0
[22359040704] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[22370779662] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22372339242] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22374572550] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[22378675572] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[22382109684] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[22385165748] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[22395803298] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1268)
[22397572164] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
T:1220 [22407975942] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[22410835590] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[22424932464] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 3)
[22426156236] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22431003870] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=242
[22433685120] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[22435147548] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22436790882] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22438574895] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22440730983] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=242 pred=0 subj_lo=0
[22465361754] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1273)
[22473952512] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[22495295460] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=269
[22497321198] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[22499788047] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22501249617] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22502957829] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22504679571] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=269 subj_lo=0
[22527848904] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1277)
[22530766764] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[22569859752] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[22574026596] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[22580769783] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[22608335343] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[22645573038] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[22647580923] [INFO] [anther] [CPU1] anther: Connected to network stack
[22725466632] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([255, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[22768174440] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[22769908095] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[22777409820] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[22809323757] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[22812449946] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[22822884975] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[22863904140] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[22868327856] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[22887076971] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[22894268265] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[22897710198] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[22904951487] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[22907363589] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[22911667680] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[22920734463] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[22924508640] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[22926676278] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[22968640035] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22988956419] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[22993510089] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[23001171039] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[23005396920] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[23009258217] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[23012616495] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[23017441623] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[23056252296] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[23066741643] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[23070449820] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[23087175540] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[23131738443] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23218942890] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[23229000531] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=649 watches=13 history=1024 journal=1024 symbols=309 drops=0
[23267260302] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[23316439014] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[23547180822] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[23775152907] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[24018085047] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[24299487828] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[24714123786] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[24772310838] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[24983295480] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[25257588741] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[25367131416] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=702 watches=13 history=1024 journal=1024 symbols=342 drops=0
[25536034788] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[25691767497] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[25701409635] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[25704096264] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[25705810746] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[25707503382] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[25709446785] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[25727452971] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[25729030437] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[25730719410] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[25732792470] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=342 pred=0 subj_lo=0
[25852240887] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[25914999363] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[25917985335] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[25968680100] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26060862168] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26079028107] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26080335567] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26081758791] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26083440207] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=305 pred=0 subj_lo=0
[26197519392] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[26221755483] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26250883230] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[26323792407] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26377017909] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26378607486] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26380431330] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26382441723] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=306 pred=0 subj_lo=0
[26394752274] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[26442040251] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[26464986273] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26475933495] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[26493263610] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[26495045412] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[26559232755] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26649081261] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26661143784] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[26673979893] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x121e4000
[26676802944] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[26678528778] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[26692342083] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[26747050935] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[26758588296] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[26764990824] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[26767873572] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[26770582014] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[26776015893] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[26796197010] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[26797991022] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[26800068273] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[26827832130] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x121e5000
[26829819621] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[27009551151] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[27327275382] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[27690360456] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=9d85f11f1afc2373)
[28026153870] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[28470705945] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=14aa2015f9906e3f)
[28599050832] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=784 watches=17 history=1024 journal=1024 symbols=364 drops=0
[28863216360] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[29265772635] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[29719812573] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[30032243340] [DEBUG] [bloom
```
</details>
