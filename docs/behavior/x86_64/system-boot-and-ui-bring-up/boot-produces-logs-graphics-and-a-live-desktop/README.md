# ❌ Scenario: Boot produces logs, graphics, and a live desktop

> Last run: 2026-04-05 18:40:52

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 4047ms | - [📜](./01/serial.log) - |
| 2 | Then I should see log messages on the terminal | ✅ | 504ms | - [📜](./02/serial.log) - |
| 3 | And each log message should include a monotonically increasing timestamp | ✅ | 0ms | - [📜](./03/serial.log) - |
| 4 | And I should see the system clock tick for several seconds in the serial console | ✅ | 3009ms | - [📜](./04/serial.log) - |
| 5 | And I should see the wallpaper on the screen within 60 seconds | ❌ | 1825ms | - [📜](./05/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12730989843] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[12737004885] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[12740812161] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[12743266701] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[12744659070] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[12745356888] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[12746065134] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[12746673621] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[12747351936] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[12748095129] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[12748987944] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[12749814429] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[12750713085] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[12751454199] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[12752216268] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[12752861385] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[12753583095] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[12754250157] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[12755190657] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[12755890719] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[12756561708] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[12757202700] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[12757824585] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[12758478183] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[12759173823] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[12759806598] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[12760470294] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[12761147223] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[12761760957] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[12762387396] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[12763022415] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[12763698321] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[12764370861] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[12765013008] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[12765631131] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[12766356372] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[12767131146] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[12767894766] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[12768746496] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[12769528101] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[12770309211] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[12771179256] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[12772647624] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[12774206874] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[12775005771] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[12775573965] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[12776106816] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[12776651778] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[12777267558] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[12777814599] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[12778352565] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[12778896867] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[12779431863] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[12779995767] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[12780596103] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[12781186572] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[12781758198] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[12782375166] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[12782947089] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[12783570261] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[12784143603] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[12784733412] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[12785303751] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[12785891448] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[12786484986] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[12787103340] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[12787673184] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[12788262795] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[12788835444] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[12789423537] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[12790021662] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[12790612428] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[12791183163] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[12791794389] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[12792368424] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[12792960114] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[12793559427] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[12794150490] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[12794723040] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[12795314532] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[12795975555] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[12796665882] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[12797246022] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[12797838405] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[12798411021] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[12799152102] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[12799731219] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[12800441148] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[12801019110] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[12801613011] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[12802196979] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[12802858596] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[12803479491] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[12804073392] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[12804646404] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[12805240272] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[12805813251] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[12806407779] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[12807019104] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[12807612246] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[12808184268] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[12808774803] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[12809344416] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[12809963595] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[12810535650] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[12811126086] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[12811695732] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[12812531490] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[13064508765] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[13077237690] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[13082317017] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[13083705063] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[13084600551] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[13088958036] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[13090718091] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[13092016542] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[13092738648] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[13093472436] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[13094178372] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[13095210744] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[13096204077] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[13096956972] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[13097769564] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[13098491637] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[13099313205] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[13100569713] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[13101642543] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[13102375572] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[13104028047] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[13105004352] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[13106039595] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[13107806778] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[13109404968] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[13110362067] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[13110937521] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[13111731138] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[13524094518] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[13525179822] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[13528500777] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[13529410818] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[13530213015] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[13531803087] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[13544872572] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[13546223658] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[13547005329] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[13548747300] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[13549316121] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[13551828543] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[13559756991] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[13561783818] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[13576395063] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[13577148684] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[13594484079] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[13595093424] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[13597250007] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[13598528328] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[13599636006] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[13602134964] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[13603089126] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[13638222840] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62385100 ticks/sec), init_cnt=623851 for 100Hz
[13639729884] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[13640607519] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[13641857526] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[13647776175] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[13679973813] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[13681007307] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[13682853657] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[13685107590] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[13687140720] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[13691779596] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[13693972776] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[13711127529] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[13712404728] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[13713481650] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[13714605333] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[13716120990] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[13718358588] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[13719539526] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[13745019915] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[13747223358] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[13748037666] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[13749750762] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[13750737066] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[13752001593] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[13752904770] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[13754186655] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[13761209187] [INFO] [kernel::root] [CPU0] Spawning Root service...
[13762377354] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[13764516744] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[13765417743] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[13772245476] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[13774895838] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[13776169077] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[13778195772] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[13779380835] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[13780544844] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[13795163976] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[13799849283] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[13801599933] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[13802875548] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13828809225] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13853534277] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13856722704] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13861758141] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13864269573] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13867710879] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13871502084] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13874320317] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13875263358] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13877575305] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13885271103] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13886972055] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13889715939] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13892127513] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13896858393] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13897812126] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13909883559] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13910730108] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13917634665] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13918524444] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13947075648] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13947856428] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[14298434271] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[14855859150] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[14886184170] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[14926337250] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[16377426681] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=449 watches=0 history=964 journal=773 symbols=98 drops=0
[17113323546] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[17208599133] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[17209696416] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[17317463889] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[17377624440] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[17406254976] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[17407236000] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[17407951770] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[17412323940] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[17431767309] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[17458518726] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[17461990590] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[17521345215] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[17543999484] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[17545206327] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[17550246549] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[17577490029] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[17599346556] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[17605318830] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[17609751753] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[17686314789] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[17688205062] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[17780483688] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[17845187448] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[17857061244] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[17861384838] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[17863705068] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[17901640317] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[17932786179] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[17938338627] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[17939843097] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[17941157850] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[17942486826] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[17943599190] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[17944689642] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[17945886222] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[17946890775] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[17947915623] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[17949014457] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[17950041912] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[17951104347] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[17952209880] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[17953864797] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[17955183774] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[17956263105] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[17957620626] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[17958691245] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[17959772457] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[17960816940] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[17961645075] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[17962293228] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[17962943097] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[17963591580] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[17964288045] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[17964963126] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[17965619859] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[17966332164] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[17966982627] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[17967646653] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[17968327047] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[17969002557] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[17969662689] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[17970331401] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[17970977376] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[17971752282] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[17972523096] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[17973300444] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[17974061655] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[17974872267] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[17975649978] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[17976434421] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[17977957008] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[17979733365] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[17980551138] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17989399164] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[18006599358] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[18011230281] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[18021005607] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[18021763452] [CONTRACT] [kernel] [CPU0] Spawning init process...
[18024037020] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[18026721273] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[18027505386] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [18032763903] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[18033573987] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013040 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[18049439826] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[18054430944] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[18055725567] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[18056890797] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[18066233955] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[18072320211] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[18076065975] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[18077332845] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[18081388611] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[18085070751] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[18086324751] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[18090867399] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[18092268843] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[18093557394] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[18094790109] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[18099119181] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[18100546794] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[18101928207] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[18103783071] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[18105436668] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[18107024298] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[18112072803] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[18159208551] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[18167202801] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[18173901174] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[18180243048] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[18184476750] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[18189641085] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[18195761166] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[18202191381] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[18208719771] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[18214923804] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[18221706756] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[18228609267] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[18234959622] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[18240580974] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[18246348384] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[18252540405] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[18258983721] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[18264948537] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[18270909954] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[18276683205] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[18282526449] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[18288770742] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[18294764037] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[18300939162] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[18306962949] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[18312719502] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[18318698178] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[18325188090] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[18332591145] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[18340526622] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[18346153881] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[18352297821] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[18360643719] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[18367862106] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[18374394852] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[18380701152] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[18387876144] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[18394438689] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[18400993677] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[18407717295] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[18414730125] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[18419130873] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[18446089860] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[18621703947] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[18629665626] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[18630609888] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18632573388] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18637464549] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18638909619] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18648158067] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[18653617488] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
[18654841788] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18656648670] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078576 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[18660493863] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[18661395819] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18662887386] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[18663889992] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18668355486] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18670546785] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18680156913] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[18685072032] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[18686366886] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[18688507629] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144112 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[18692941476] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[18701752311] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[18703566354] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[18705680796] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18710191137] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:41:45 = 1775439705 unix_secs
[18711961290] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775439705, mono_ns=9355753242, offset=1775439695644246758ns
[18713877072] [INFO] [rtc_cmos] [CPU1] System clock anchored
[18726989556] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[18781089921] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[18782343756] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[18783446154] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18786385563] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18794992623] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18798204744] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[18809033529] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[18814596966] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[18820038831] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18822158124] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210704 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[18828330378] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[18834360699] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[18837035778] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[18838276545] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18841272846] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18851578317] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18854167002] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18861743802] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[18865201113] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[18866370435] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18868615392] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277296 RFLAGS_BEFORE=130 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[18873392604] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[18876477939] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[18877825890] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[18886833735] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[18888020349] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[18889227720] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[18890419647] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[18891385260] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[18911962278] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[18913728768] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[19442085894] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19447344741] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[19450206666] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[19452019851] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[19456660971] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[19458510456] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[19460629749] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[19462287141] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[19463451447] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[19466150913] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[19467295023] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[19468482990] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[19469562519] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[19470518199] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[19471461933] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[19472726988] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[19477018737] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[19478051373] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19480033683] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19487759478] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19490443203] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19497795141] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[19502303832] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[19503804606] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[19506183048] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352608 RFLAGS_BEFORE=134 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[19512957024] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[19521657309] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[19548426084] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[19554162276] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[19556310873] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[19562595129] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[19571719926] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[19573591323] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[19584110502] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[19586056248] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[19587376149] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[19588659354] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[19589944044] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[19591172205] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[19592403600] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[19593689346] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[19594986576] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[19602427977] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[19605630363] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[19608672336] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[19611193206] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[19612388433] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19614869769] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19619337870] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[19621539366] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19630217772] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[19633658649] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[19636168596] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[19637347422] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19639654617] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19645182018] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19647056121] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19654969950] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[19657836858] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
[19659170949] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb01080c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[19661491971] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500192 RFLAGS_BEFORE=130 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[19667258061] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[19668538791] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19670630628] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[19671638316] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19672845918] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[19678109187] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19679382030] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[19681538118] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19682860527] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[19685141421] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[19687953516] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[19690145805] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[19691789370] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[19693447092] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[19696306707] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[19697612352] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[19699751610] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[19701260667] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[19702484934] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[19703818629] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[19705590531] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0103240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[19707991281] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369433952 RFLAGS_BEFORE=134 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[19714997115] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[20059245690] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb01080c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[20061381978] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369582976 RFLAGS_BEFORE=130 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[20067234363] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[20068372764] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[20070336957] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[20071767705] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[20073118791] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[20074894983] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[20082206859] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20086344795] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[20087629749] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20089720893] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[20091352875] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20101592610] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[20105230002] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[20115731856] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[20120509926] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[20164354485] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0103240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[20166834468] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650272 RFLAGS_BEFORE=134 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[20174528253] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[20176325598] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[20182527057] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[20184984567] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[20186127555] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20188702545] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20219960937] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20226978255] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[20262588126] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[20371132188] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[20372190201] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[20373788094] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[20375311836] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[21172548969] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[21175896984] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0103240
[21177361194] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21179386932] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369720496 RFLAGS_BEFORE=130 CR3_BEFORE=68890624 fs_base=0 gs_base=18446744071564586640
[21183845034] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[21185736858] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[21186673134] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[21188201529] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[21190302903] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[21192542712] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[21194152782] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[21196556403] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[21214848732] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[21224269506] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[21226785327] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[21228889341] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[21230319759] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[21232534851] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[21233975169] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[21235859832] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4c32000
[21237253422] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[21239553258] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[21241954074] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[21243967107] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[21258785295] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[21273364695] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[21289254030] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[21293461893] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[21297104730] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[21299781723] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[21302233524] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[21304701495] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[21307470756] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[21309792768] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[21312297072] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[21320389233] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=15, read=16)
[21329340483] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=17, read=18)
[21334738590] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[21337209234] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[21339623514] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[21340918038] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21343619649] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21414446955] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21436739775] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[21462227985] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[21469795149] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[21473043834] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0103240
[21474398022] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21476151510] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369917104 RFLAGS_BEFORE=130 CR3_BEFORE=80007168 fs_base=0 gs_base=18446744071564586576
[21480336834] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[21481359537] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21483804276] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21484847274] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[21488724246] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[21493762059] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21495507165] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21518393358] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[21523377051] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[21535066311] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[21536589954] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21541699542] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[21545407356] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[21546573444] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21549333201] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21558038502] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[21560760606] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21568788054] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[21571849662] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
[21573295524] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010bc50
[21574966116] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21576546552] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21577925886] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370048176 RFLAGS_BEFORE=130 CR3_BEFORE=80941056 fs_base=0 gs_base=18446744071564586640
[21582993663] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21585714975] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[21588123678] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[21590827599] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21594723282] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[21599429676] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21603259821] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[21606175404] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010df90
[21607603545] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21609148440] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113712 RFLAGS_BEFORE=130 CR3_BEFORE=81068032 fs_base=0 gs_base=18446744071564586576
[21613461639] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[21614401479] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21616056594] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21625473936] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[21628812249] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[21637570284] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[21640416501] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[21643248396] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[21644139660] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21645841173] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21650855094] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21653115990] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21660033681] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21671418780] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[21679149327] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[21690722328] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[21695339061] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010e118
[21696681897] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21698550918] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370245936 RFLAGS_BEFORE=134 CR3_BEFORE=81371136 fs_base=0 gs_base=18446744071564586640
[21703087923] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[21704436699] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21707696637] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[21708961098] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21728327874] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21735651036] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[21740304729] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[21744956871] [INFO] [fontd] [CPU3] FONTD: Service node created, req=19, resp=22
[21749336697] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21750787773] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[21751993857] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21753678474] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21755667252] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1138
[21757104204] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[21757995171] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21760537920] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[21761718858] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21763668366] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370330880 RFLAGS_BEFORE=134 CR3_BEFORE=81645568 fs_base=0 gs_base=18446744071564586576
[21767302920] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21771371820] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[21772909950] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[21782202981] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21783452526] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21784836513] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21786411702] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[21792051171] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21793617714] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21795309888] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21797076345] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[21809235888] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[21820065465] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[21828193068] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[21830977707] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[21835754028] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21838334793] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0103240
[21839443857] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21841953606] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369982640 RFLAGS_BEFORE=130 CR3_BEFORE=80678912 fs_base=0 gs_base=18446744071564586608
[21846635151] [INFO] [nectar] [CPU2] NECTAR: Started.
[21848428206] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21849973299] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21851783679] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21853655274] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[21856374903] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010bc50
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21858839112] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370179856 RFLAGS_BEFORE=130 CR3_BEFORE=81215488 fs_base=0 gs_base=18446744071564586608
[21866340474] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[21867347634] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21870114090] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21871819200] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1158
[21873398283] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21875724915] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370398464 RFLAGS_BEFORE=134 CR3_BEFORE=81854464 fs_base=0 gs_base=18446744071564586608
[21880612149] [INFO] [fontd] [CPU3] FONTD: Service ready
[21882237399] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21883510044] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21884951352] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[21886385895] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21887845155] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[21889372164] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[21891609894] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[21921897195] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21934785444] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=15, RX port=18
[21939835170] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[21952453413] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21954030780] [INFO] [netd] [CPU3] NETD: Driver TX port=15, RX port=18, link_up=true, mtu=1500
[21955554093] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21957166110] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21958875576] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[21960192243] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=244 subj_lo=0
[21965770893] [INFO] [netd] [CPU3] NETD: Created socket API port (write=23, read=24)
[21973354392] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21975067521] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21976920042] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21978956637] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=245 subj_lo=0
[21992763045] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[22000170192] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22001912262] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22003745874] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22005722211] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=249 subj_lo=0
[22017767772] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([231, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[22021787172] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1252 backend=VirtIO-GPU
[22023999921] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[22024900095] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22026965103] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22033965921] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22035454980] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22037136231] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22039007100] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=252 subj_lo=0
[22051473510] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[22052993226] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[22054681902] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=25, our_read=26)
[22057410210] [INFO] [anther] [CPU1] anther: Connected to network stack
[22060960779] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[22101829002] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[22113413487] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22114577298] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22115888652] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22117223832] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=254 pred=0 subj_lo=0
[22146831861] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[22162170822] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=27, resp=30
[22163374035] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[22187272239] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[22210996038] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[22219961940] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[22222671405] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f1158
[22224036615] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e4
[22225676022] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370552064 RFLAGS_BEFORE=134 CR3_BEFORE=82771968 fs_base=0 gs_base=18446744071564586640
[22230142671] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[22231087857] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22232908599] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22237510086] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[22239109794] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22242751575] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[22247163840] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[22249808064] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[22251441663] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[22256713677] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1158
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[22258486800] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370625792 RFLAGS_BEFORE=134 CR3_BEFORE=84353024 fs_base=0 gs_base=18446744071564586576
[22265744886] [INFO] [echo] [CPU1] echo: starting up
[22267647534] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[22268774253] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[22274471340] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[22288414599] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[22301976609] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[22305018978] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[22311851133] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[22314592773] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[22315948842] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[22317093480] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[22330982289] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[22332959055] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[22338071943] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[22349756385] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[22351978968] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[22354428129] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[22367428743] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[22372171470] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[22383158127] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[22385232573] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[22387212738] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[22388545080] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22390296093] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22395682155] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22397553552] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[22405072734] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[22407835032] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[22409419593] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[22411382136] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[22413436617] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[22414364016] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22416033948] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22420392126] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[22423773273] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22425036216] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[22427302194] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[22428784290] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[22432259289] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[22437094812] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[22439754612] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[22441350525] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[22442477772] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[22443734247] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[22444675110] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22446425463] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22449596367] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[22450917753] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[22456014042] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f8128
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22458420039] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370756864 RFLAGS_BEFORE=134 CR3_BEFORE=85139456 fs_base=0 gs_base=18446744071564586640
[22463020173] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[22466783493] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[22469402835] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[22477512420] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[22482512448] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[22485276495] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[22488033084] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[22491145248] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f8128
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22493399577] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[22494721953] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370830592 RFLAGS_BEFORE=134 CR3_BEFORE=85286912 fs_base=0 gs_base=18446744071564586576
[22499425344] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[22501912554] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[22503806688] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[22512389460] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[22552279266] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[22582439484] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[22587421626] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[22594660308] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[22600996737] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[22604632215] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[22605864369] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[22607742630] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[22637554863] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[22662759174] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[22665461115] [INFO] [bloom] [CPU3] bloom: creating surface...
[22666820649] [INFO] [bloom] [CPU3] bloom: surface created!
[22668071151] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[22669769199] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[22676903007] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[22679633790] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[22682518155] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[22685341668] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[22693841808] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1264
[22695473526] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[22708984782] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[22712395266] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[22714210002] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[22716383151] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [22725054792] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[22734635781] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[22743994449] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[22753482213] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
T:07D0 T:0640 T:F0B0 [22773949440] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1264
[22807036989] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[22817722587] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[22820217849] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 5)
[22821325461] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[22823940051] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[22827449667] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
T:1220 [22841325804] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[22844129220] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[22855021035] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[22857983808] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[22875404310] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=284
[22877977914] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[22880068299] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22881659394] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22884186468] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22886328168] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=284 subj_lo=0
[22902762531] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1278)
[22905083190] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[22920471024] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=254
[22922444325] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[22924478082] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22926285261] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22928717229] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22930656177] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=254 pred=0 subj_lo=0
[22953743901] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1280)
[22955222631] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[22968745899] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=289
[22970775267] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[22971922842] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22972943631] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22974052431] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22975747872] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=289 subj_lo=0
[22991880384] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1282)
[22993588134] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[23065954296] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[23334384876] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[23350564908] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[23397270138] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[23503709097] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=663 watches=13 history=1024 journal=1024 symbols=314 drops=0
[23570910363] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[23767147668] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[23961838593] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[24229810143] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[24555635841] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[24610559292] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[24785213508] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[24967977639] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[25147661022] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[25280537469] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=703 watches=13 history=1024 journal=1024 symbols=339 drops=0
[25349066292] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[25396791420] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[25403925129] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[25405906680] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[25407437484] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[25409193150] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[25410778371] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[25421849574] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[25422983553] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[25424342064] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[25425907980] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=339 pred=0 subj_lo=0
[25556259102] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[25763552067] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[26087899662] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[26163362610] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[26165654823] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[26381798223] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26383666881] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26385596688] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26387692386] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=311 pred=0 subj_lo=0
[26448634245] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[26469688377] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26471703984] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26473627026] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26475663159] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=349 pred=0 subj_lo=0
[26628965022] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[26783948301] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26785728123] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26787825207] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26789979051] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=350 pred=0 subj_lo=0
[26810074962] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[26891321622] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[26904971973] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=0e57f834b3afbcb1)
[27092273769] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[27171779217] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[27177641370] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[27180072216] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[27203367840] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12222000
[27205605537] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[27207915339] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[27288890310] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[27311972952] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[27357629541] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[27364866276] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[27368433477] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[27372028101] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[27379625889] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[27384508965] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[27386837841] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[27390326535] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[27421606641] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x1222e000
[27423910866] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[27795447570] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=c1e9a7b90a789e5c)
[28317297921] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[28537054656] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=786 watches=18 history=1024 journal=1024 symbols=363 drops=0
[28639112964] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[28940659110] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[29222499867] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[29543682861] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[29906649135] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[30240766314] [INFO] [
```
</details>
