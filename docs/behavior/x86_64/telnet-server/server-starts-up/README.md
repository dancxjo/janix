# ✅ Scenario: Server starts up

> Last run: 2026-04-05 19:59:17

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 4048ms | - [📜](./01/serial.log) - |
| 2 | Then I should see a message in the serial output that says "telnetd: listening on guest port 2323" | ✅ | 5578ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12673103421] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[12680095197] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[12684211320] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[12686382324] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[12687681534] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[12688341138] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[12689094231] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[12689788485] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[12690399843] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[12691186926] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=33264
[12691804455] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=973464
[12692422677] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[12693178047] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[12693881442] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[12694654071] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[12695284800] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[12695922294] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[12696534840] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[12697276185] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[12697951497] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[12698553120] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[12699273114] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[12700051650] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[12700737093] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[12701400393] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[12702024159] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[12702640698] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[12703304526] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[12703933242] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[12704557470] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[12705241197] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[12705878262] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[12706593471] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[12707258190] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=168464
[12707903802] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[12708644256] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[12709390089] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[12710125527] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[12710868753] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[12711618183] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[12712348638] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[12713207430] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[12714658671] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[12716207229] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[12717180465] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[12717759549] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[12718330449] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[12718891713] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[12719486901] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[12720025065] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[12720619263] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[12721159407] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[12721726776] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[12722296983] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7795e000 (Usable)
[12722858214] [INFO] [kernel::memory] [CPU0]   [11] 0x7795e000 - 0x779c2000 (Reserved)
[12723438519] [INFO] [kernel::memory] [CPU0]   [12] 0x779c2000 - 0x779c3000 (Other)
[12724116900] [INFO] [kernel::memory] [CPU0]   [13] 0x779c3000 - 0x779c4000 (Reserved)
[12724708392] [INFO] [kernel::memory] [CPU0]   [14] 0x779c4000 - 0x779c5000 (Other)
[12725313975] [INFO] [kernel::memory] [CPU0]   [15] 0x779c5000 - 0x779c6000 (Reserved)
[12725904675] [INFO] [kernel::memory] [CPU0]   [16] 0x779c6000 - 0x779c7000 (Other)
[12726514218] [INFO] [kernel::memory] [CPU0]   [17] 0x779c7000 - 0x779c8000 (Reserved)
[12727302984] [INFO] [kernel::memory] [CPU0]   [18] 0x779c8000 - 0x77a53000 (Other)
[12727884312] [INFO] [kernel::memory] [CPU0]   [19] 0x77a53000 - 0x77a54000 (Reserved)
[12728469402] [INFO] [kernel::memory] [CPU0]   [20] 0x77a54000 - 0x77ed5000 (Other)
[12729031854] [INFO] [kernel::memory] [CPU0]   [21] 0x77ed5000 - 0x77ed6000 (Reserved)
[12729617703] [INFO] [kernel::memory] [CPU0]   [22] 0x77ed6000 - 0x77ff7000 (Other)
[12730211967] [INFO] [kernel::memory] [CPU0]   [23] 0x77ff7000 - 0x77ff8000 (Reserved)
[12730822236] [INFO] [kernel::memory] [CPU0]   [24] 0x77ff8000 - 0x787f8000 (Other)
[12731407557] [INFO] [kernel::memory] [CPU0]   [25] 0x787f8000 - 0x787f9000 (Reserved)
[12731990733] [INFO] [kernel::memory] [CPU0]   [26] 0x787f9000 - 0x788ba000 (Other)
[12732553251] [INFO] [kernel::memory] [CPU0]   [27] 0x788ba000 - 0x788bb000 (Reserved)
[12733244403] [INFO] [kernel::memory] [CPU0]   [28] 0x788bb000 - 0x788e5000 (Other)
[12733883349] [INFO] [kernel::memory] [CPU0]   [29] 0x788e5000 - 0x788e6000 (Reserved)
[12734466228] [INFO] [kernel::memory] [CPU0]   [30] 0x788e6000 - 0x788eb000 (Other)
[12735028185] [INFO] [kernel::memory] [CPU0]   [31] 0x788eb000 - 0x788ec000 (Reserved)
[12735608589] [INFO] [kernel::memory] [CPU0]   [32] 0x788ec000 - 0x788f1000 (Other)
[12736169820] [INFO] [kernel::memory] [CPU0]   [33] 0x788f1000 - 0x788f2000 (Reserved)
[12736777977] [INFO] [kernel::memory] [CPU0]   [34] 0x788f2000 - 0x7891e000 (Other)
[12737340297] [INFO] [kernel::memory] [CPU0]   [35] 0x7891e000 - 0x78920000 (Reserved)
[12737920536] [INFO] [kernel::memory] [CPU0]   [36] 0x78920000 - 0x78929000 (Other)
[12738481932] [INFO] [kernel::memory] [CPU0]   [37] 0x78929000 - 0x7892b000 (Reserved)
[12739061676] [INFO] [kernel::memory] [CPU0]   [38] 0x7892b000 - 0x78933000 (Other)
[12739624821] [INFO] [kernel::memory] [CPU0]   [39] 0x78933000 - 0x78934000 (Reserved)
[12740301783] [INFO] [kernel::memory] [CPU0]   [40] 0x78934000 - 0x7893e000 (Other)
[12740903340] [INFO] [kernel::memory] [CPU0]   [41] 0x7893e000 - 0x7893f000 (Reserved)
[12741488331] [INFO] [kernel::memory] [CPU0]   [42] 0x7893f000 - 0x7894c000 (Other)
[12742051179] [INFO] [kernel::memory] [CPU0]   [43] 0x7894c000 - 0x7894e000 (Reserved)
[12742632771] [INFO] [kernel::memory] [CPU0]   [44] 0x7894e000 - 0x7895c000 (Other)
[12743195520] [INFO] [kernel::memory] [CPU0]   [45] 0x7895c000 - 0x7895d000 (Reserved)
[12743818494] [INFO] [kernel::memory] [CPU0]   [46] 0x7895d000 - 0x78969000 (Other)
[12744473445] [INFO] [kernel::memory] [CPU0]   [47] 0x78969000 - 0x7896a000 (Reserved)
[12745077246] [INFO] [kernel::memory] [CPU0]   [48] 0x7896a000 - 0x7896e000 (Other)
[12745684512] [INFO] [kernel::memory] [CPU0]   [49] 0x7896e000 - 0x7896f000 (Reserved)
[12746360847] [INFO] [kernel::memory] [CPU0]   [50] 0x7896f000 - 0x7897f000 (Other)
[12746952735] [INFO] [kernel::memory] [CPU0]   [51] 0x7897f000 - 0x78980000 (Reserved)
[12747535614] [INFO] [kernel::memory] [CPU0]   [52] 0x78980000 - 0x78a10000 (Other)
[12748142715] [INFO] [kernel::memory] [CPU0]   [53] 0x78a10000 - 0x78a11000 (Reserved)
[12748728366] [INFO] [kernel::memory] [CPU0]   [54] 0x78a11000 - 0x78a1c000 (Other)
[12749422818] [INFO] [kernel::memory] [CPU0]   [55] 0x78a1c000 - 0x78a1d000 (Reserved)
[12750075657] [INFO] [kernel::memory] [CPU0]   [56] 0x78a1d000 - 0x78a42000 (Other)
[12750642927] [INFO] [kernel::memory] [CPU0]   [57] 0x78a42000 - 0x78a43000 (Reserved)
[12751224420] [INFO] [kernel::memory] [CPU0]   [58] 0x78a43000 - 0x78a4f000 (Other)
[12751828023] [INFO] [kernel::memory] [CPU0]   [59] 0x78a4f000 - 0x78a50000 (Reserved)
[12752516733] [INFO] [kernel::memory] [CPU0]   [60] 0x78a50000 - 0x78a5d000 (Other)
[12753082056] [INFO] [kernel::memory] [CPU0]   [61] 0x78a5d000 - 0x78a5e000 (Reserved)
[12753706647] [INFO] [kernel::memory] [CPU0]   [62] 0x78a5e000 - 0x78aa6000 (Other)
[12754269990] [INFO] [kernel::memory] [CPU0]   [63] 0x78aa6000 - 0x78aa7000 (Reserved)
[12755487426] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[13018479309] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485750 free frames
[13030852428] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[13035947529] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[13037277297] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[13038201198] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[13042773744] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[13044638376] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[13045940061] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[13047101661] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[13048249698] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[13049367804] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[13050957876] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[13052401032] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[13053547122] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[13054677768] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[13055797095] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[13056949554] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[13058729871] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[13060137618] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[13060898862] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[13062636972] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[13063638291] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[13064666109] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[13066502757] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[13068447612] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[13069636899] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[13070427447] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[13071558423] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[13489282125] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[13490342019] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[13493824476] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[13494802299] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[13495598853] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[13497168960] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[13510717671] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[13512108126] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[13512943554] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[13515121686] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[13515716478] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[13518395154] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[13526616807] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[13528703133] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[13542848319] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[13543562472] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[13563901758] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[13564923867] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[13567614357] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[13569365667] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[13570692927] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[13573204359] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[13574120934] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[13609303785] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62641400 ticks/sec), init_cnt=626414 for 100Hz
[13611043083] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[13611902931] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[13613342061] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[13619833194] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[13651305030] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[13652424489] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[13654287801] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[13655630406] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[13656907671] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[13659858564] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[13661313402] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[13681897119] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[13683591801] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[13684504977] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[13686057231] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[13686820323] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[13689067260] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[13691494740] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[13715422083] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[13717183887] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[13718124420] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[13720717857] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[13721631627] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[13722542955] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[13723504212] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[13724186784] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[13731081771] [INFO] [kernel::root] [CPU0] Spawning Root service...
[13732226673] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[13734582642] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[13735577856] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[13740987447] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[13743915009] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[13745255337] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[13747655559] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[13749762444] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[13751459304] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[13766090382] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[13771018173] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[13772753379] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[13773642069] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13799026395] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13821482400] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13824630072] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13830312573] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13832516544] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13834986759] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13837639596] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13840338633] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13841156076] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13842378033] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13850548866] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13853995023] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13858705245] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13862905221] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13865943597] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13869348768] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13870291116] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13886889159] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13887830484] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13895289540] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13896166416] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13934673456] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13935551784] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[14346576123] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[14946050625] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[15015591063] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=499 journal=424 symbols=51 drops=0
[15061853565] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[16578912504] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=961 journal=769 symbols=95 drops=0
[17601534027] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[17751313767] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[17752465962] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[17888283897] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[17960778825] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[17990123448] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[17991517236] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[17992422195] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[17998269333] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[18022503807] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[18043459269] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[18045831540] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[18107157552] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[18129786015] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[18130631442] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[18134922201] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[18162500631] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[18185060553] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[18189491133] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[18190603827] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[18274082904] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[18275944137] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[18385618449] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[18460148091] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[18474891996] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[18480023331] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[18482617626] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[18495074697] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[18565519071] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[18571884276] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[18573175500] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[18574185531] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[18575395014] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[18576242421] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[18577215129] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[18578104182] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[18578745537] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[18579378807] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[18580071543] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=33264
[18580710786] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=973464
[18581362635] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[18582035736] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[18582740748] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[18583499187] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[18584354745] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[18585144765] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[18586037448] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[18586779024] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[18587429553] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[18588051603] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[18588682860] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[18589320354] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[18589974282] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[18590661705] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[18591329097] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[18592057374] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[18592898115] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[18593687409] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[18594784659] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[18595465482] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[18596135613] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[18596821815] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[18597482244] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=168464
[18598131420] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[18598884117] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[18599671728] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[18600440991] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[18601552167] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[18602373207] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[18603377430] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[18604162170] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[18606218697] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[18608120817] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[18608934696] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18617979567] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[18634938828] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[18641623539] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[18651829317] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[18652640853] [CONTRACT] [kernel] [CPU0] Spawning init process...
[18654735198] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[18657409650] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[18658234848] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
ThingOS PetalsUSER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000

type 'help' for commands

petals> [18664705092] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[18666573288] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013440 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[18694750899] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[18697982061] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[18699323379] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[18701397990] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[18711533016] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[18718178325] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[18721630818] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[18722885049] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[18726920817] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[18730350804] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[18732355983] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[18738004758] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[18739379406] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[18740626245] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[18741974493] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[18746366265] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[18747677322] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[18749052663] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[18751060647] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[18753123411] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[18754841589] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[18759580587] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[18813624027] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[18824166273] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[18830886921] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[18837747555] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[18841214766] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[18845376297] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[18851878749] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[18857749944] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[18863352717] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[18869284731] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[18876759561] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[18883091007] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[18889415457] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[18895094988] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[18901212264] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[18908046333] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[18915041079] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[18920184591] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[18926948469] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[18933340404] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[18938269944] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[18943594560] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[18949407048] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[18956073114] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[18962614704] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[18969242259] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[18974461341] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[18980777376] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[18986181786] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[18991577616] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[18996239757] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[19000238268] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[19006227999] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[19010172192] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/telnetd
[19013531163] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[19019208549] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[19024674933] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[19030677072] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[19036407423] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[19042571064] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[19048907526] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[19055911116] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[19059799572] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[19085123409] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[19250515251] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[19257838875] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[19258711461] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19260423831] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19265318061] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19266889686] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19275568290] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[19281281547] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[19282320651] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19283877096] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078976 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[19288955631] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[19291339914] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[19292095713] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19293691461] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19298732805] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[19300802664] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19309229016] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[19313872446] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[19315948707] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[19317448128] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[19319179176] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144512 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[19326019911] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[19328007336] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[19330480257] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19335763689] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 02:59:36 = 1775444376 unix_secs
[19337568030] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775444376, mono_ns=9668540673, offset=1775444366331459327ns
[19339198725] [INFO] [rtc_cmos] [CPU1] System clock anchored
[19351510761] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[19408235649] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[19420546299] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[19421862900] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19424529102] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19431745278] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[19434999738] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[19444394079] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[19448254287] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[19452655629] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19454916063] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210880 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[19462022877] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[19468564203] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[19470925848] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[19472149422] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19474752363] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19484713644] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19488552171] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19497782469] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[19501944000] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[19502880045] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19504967031] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[19505853312] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277632 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[19512306594] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[19513194129] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[19520730504] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[19521455514] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[19522357635] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[19523143860] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[19524180291] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[19546551453] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[19548102849] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[20100438534] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20114920551] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[20119051458] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[20121393237] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[20125935357] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[20128008450] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[20130408870] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[20132121141] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[20133279837] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[20135729196] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[20136880467] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[20138062758] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[20139048468] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[20140022562] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[20140938906] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[20141949465] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[20155601103] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[20156948229] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[20159666307] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20167274457] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20169736719] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[20177053314] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[20180483466] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[20181895833] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[20184271965] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369357184 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[20191960602] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[20199871758] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[20220428877] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[20228990265] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[20231247630] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[20237057610] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[20246021367] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[20247416508] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[20280154488] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[20285984004] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[20287739868] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[20289041883] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[20290541502] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[20291813685] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[20293084581] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[20294665380] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[20296036167] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[20759032833] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[20762378901] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[20764987551] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[20767049490] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[20768185482] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20770580589] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20775819405] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[20777856165] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20787379866] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[20791793550] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[20794502982] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[20795404080] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20797220202] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20801966031] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20803672197] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20812465179] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[20817765969] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0105668
[20818752966] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[20823926673] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500880 RFLAGS_BEFORE=130 CR3_BEFORE=68509696 fs_base=0 gs_base=18446744071564586640
[20828710782] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[20830004349] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20832628971] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20833858353] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[20835658569] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[20840644044] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20842058325] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[20844596256] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20847233187] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[20849465142] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[20851637565] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[20854854867] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[20857176648] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[20858776554] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[20861688705] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[20862808989] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[20864616861] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0105668
[20866393713] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[20867739948] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[20869882143] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[20871073146] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583376 RFLAGS_BEFORE=130 CR3_BEFORE=68808704 fs_base=0 gs_base=18446744071564586576
[20874899958] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[20876216658] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[20877432543] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[20879724591] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[20882321625] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[20883332745] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434544 RFLAGS_BEFORE=130 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[20889579744] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[20891339205] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[20899654809] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1241
[20900800008] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1240)
[20903292663] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[20905847853] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[20911388949] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20914694394] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[20916055809] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20918636343] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20926149453] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[20928746520] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[20936602203] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[20939302263] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[20941815213] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[20943221937] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[20944092279] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20946098514] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20967225048] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20971681500] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[21869203257] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[21874602684] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb01095c8
[21875838072] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21877498698] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716640 RFLAGS_BEFORE=134 CR3_BEFORE=69156864 fs_base=0 gs_base=18446744071564586640
[21881999172] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[21884249706] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650560 RFLAGS_BEFORE=130 CR3_BEFORE=69021696 fs_base=0 gs_base=18446744071564586608
[21887787768] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[21889527429] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[21890758989] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[21892346256] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[21893404797] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[21894389088] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[21896032059] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[21897880389] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using cooperative polling loop (2ms interval)
[21898912662] [INFO] [ps2_mouse] [CPU3] ps2_mouse: using cooperative polling loop (2ms interval)
[21902894475] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[21904507614] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[21905600277] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[21907094748] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[21911657691] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[21914108898] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[21916234791] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[21917280759] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21919141761] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21990908643] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[22017187896] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[22025437335] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[22028749446] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0102438
[22030082118] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22031793993] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369782704 RFLAGS_BEFORE=130 CR3_BEFORE=79892480 fs_base=0 gs_base=18446744071564586576
[22035951399] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[22036865004] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22038836820] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22040834310] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[22045459722] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[22052892708] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22055147367] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22066644798] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[22070744652] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[22081840275] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[22085483079] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[22087827333] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[22088580987] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22090068363] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22095903918] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[22098481581] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22106193384] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[22109277300] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010ac38
[22110832227] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22112506350] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915088 RFLAGS_BEFORE=130 CR3_BEFORE=80826368 fs_base=0 gs_base=18446744071564586640
[22117177632] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[22117943463] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22119439749] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22120184064] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[22122522345] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[22126887585] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22130632128] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[22138511472] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[22142010330] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010ac38
[22143530376] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22145617560] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981584 RFLAGS_BEFORE=134 CR3_BEFORE=80953344 fs_base=0 gs_base=18446744071564586576
[22149405168] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[22150796151] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22153352892] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22163954703] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[22167993210] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[22176007227] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[22179719694] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[22182543669] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[22183945905] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22186703616] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22215894690] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[22222905540] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[22233847515] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[22239545889] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
[22242991188] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[22244057253] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22246517007] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cc18
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22248898254] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=tlas-based IPC)
[22259584941] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22321621080] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[22327256127] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[22330863819] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[22340653401] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[22342284591] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22343670954] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[22345367451] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[22347187500] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[22351470504] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[22353719652] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[22354433574] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22356158880] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22363354794] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22365854874] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370197984 RFLAGS_BEFORE=130 CR3_BEFORE=81530880 fs_base=0 gs_base=18446744071564586576
[22373555391] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[22375747284] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[22380960558] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[22382621745] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22384369425] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_wad=230 subj_lo=0
[22412865915] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[22426810461] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[22430433696] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[22432271268] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22434281826] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[22437232422] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[22438685478] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[22441874796] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[22445811333] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22447700550] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849008 RFLAGS_BEFORE=130 CR3_BEFORE=80564224 fs_base=0 gs_base=18446744071564586608
[22453418592] [INFO] [nectar] [CPU2] NECTAR: Started.
[22454038761] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22455270156] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22456464558] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
[22458047832] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22459504287] [INFO] [fontd] [CPU3] FONTD: Service ready
[22460445381] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[22462672782] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010ac38
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22465432605] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047936 RFLAGS_BEFORE=134 CR3_BEFORE=81100800 fs_base=0 gs_base=18446744071564586608
[22477602576] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db4c0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22483988538] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22485689226] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22489473435] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370268288 RFLAGS_BEFORE=130 CR3_BEFORE=81739776 fs_base=0 gs_base=18446744071564586608
[22496776995] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[22498454451] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[22500622386] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[22503356370] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22504757583] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22506209055] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22507674288] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=234 subj_lo=0
[22512241818] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[22515783939] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[22537840578] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[22543249542] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[22546150110] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22547586369] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22548949599] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22550258280] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=239 subj_lo=0
[22557931638] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1251 backend=VirtIO-GPU
[22560351759] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[22561206822] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22563326478] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22565535333] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22566753000] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22568137350] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22569651555] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[22579823310] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22580987814] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22582340484] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22583761464] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[22593717993] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22594919589] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22596686673] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22598201142] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[22631000205] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[22654849569] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22656366117] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22666216353] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[22667614662] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[22716863070] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d6000 exec=false
[22734823584] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ec000 exec=false
[22742800212] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[22745974449] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f0fd8
[22748524359] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e3
[22750392852] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370361824 RFLAGS_BEFORE=130 CR3_BEFORE=82391040 fs_base=0 gs_base=18446744071564586640
[22755392748] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[22756189137] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22757759244] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22759035915] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[22763878434] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22765943805] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[22774400220] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[22777063947] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f0fd8
[22778035005] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[22779502482] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370427360 RFLAGS_BEFORE=130 CR3_BEFORE=83447808 fs_base=0 gs_base=18446744071564586576
[22783358532] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[22785522408] [INFO] [echo] [CPU1] echo: starting up
[22788042123] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[22791965163] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[22801573542] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[22804219779] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[22832945586] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[22835009934] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f5000
[22836987921] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f5000
[22838556279] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f8000
[22841242446] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276787200)
[22842954882] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[22845084207] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f9000 phys=0x5036000
[22846558614] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[22849498452] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[22852291044] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[22855932198] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[22868535723] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[22877907987] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[22880207889] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[22881305205] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[22886363115] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[22891651365] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[22895220612] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[22898782566] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[22900873413] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[22902462891] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[22904189220] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[22905709134] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[22907109621] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[22908775593] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[22909580463] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22911817335] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22912822152] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[22914650484] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[22915605075] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[22916891844] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[22921759179] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[22925556654] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[22927849593] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[22929588231] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[22930337727] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22932035511] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22935899646] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[22937401311] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22939356660] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[22943210367] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[22947505548] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[22950804657] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[22952688693] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[22954449804] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[22956205800] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[22957015884] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22958866095] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22967226414] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22971627063] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[22979896170] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[22983384633] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[22984359684] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[22985431755] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[22987152474] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[22988413800] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[22989305031] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22990865304] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[22992801381] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22995709770] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f11d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22998144279] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370709984 RFLAGS_BEFORE=130 CR3_BEFORE=84340736 fs_base=0 gs_base=18446744071564586640
[23003152821] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[23005032072] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[23014175910] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[23016503928] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[23020750797] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[23021813430] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f11d8
[23023258929] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/telnetd'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23025555861] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370775520 RFLAGS_BEFORE=130 CR3_BEFORE=84500480 fs_base=0 gs_base=18446744071564586576
[23029826358] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/telnetd
[23030949777] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23033857176] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23051988630] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[23060821608] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[23063225295] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[23067594792] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=220000 exec=false
[23076015600] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=228000 exec=false
[23085538080] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[23088751224] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 32 (user task/process) assigned to CPU 2
[23090279355] [INFO] [bloom] [CPU3] bloom: creating surface...
[23092500750] [INFO] [bloom] [CPU3] bloom: surface created!
[23094032115] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[23095000170] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=32)
[23097029835] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[23102403126] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[23110723119] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[23112788985] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1261
[23114507427] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[23116074036] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f11d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23118860160] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=32 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370841056 RFLAGS_BEFORE=130 CR3_BEFORE=84602880 fs_base=0 gs_base=18446744071564586608
[23127617700] [INFO] [telnetd] [CPU2] telnetd: starting on port 2323
[23133414348] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[23137692270] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=33)
[23139460311] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
T:1580 [23144400147] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[23147263359] [INFO] [telnetd] [CPU2] telnetd: waiting for network stack
[23165032077] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[23169165921] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[23170744311] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[23178151491] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[23186781156] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[23191757424] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[23201349567] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[23202905253] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[23205449223] [INFO] [bloom] [CPU3] [bloom] dynamically subscribed to input topic 0 on svc.Input 1240 via port 28
[23207722263] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 28 (legacy was 12)
T:1AE0 T:1950 T:03C0 [23234154801] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1261
[23245733412] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[23273968179] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[23283061824] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 3)
[23286933714] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 37 (user thread) assigned to CPU 3
[23288366046] [INFO] [telnetd] [CPU2] telnetd: waiting for network stack
[23290633608] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=37 (priority=2)
[23294225295] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[23298089001] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
T:2530 [23316902565] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[23319320574] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[23344128027] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[23347632660] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[23362193382] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=274
[23363976537] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[23365100715] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[23366272182] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23367563340] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23369019762] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=274 subj_lo=0
[23374982070] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[23377153932] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=77c80b059e863710)
[23378698959] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[23384170227] [INFO] [netd] [CPU3] NETD: Created socket API port (write=29, read=30)
[23396931393] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1275)
[23398721313] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[23427224502] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=243
[23429411841] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[23430609477] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[23431928850] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[23433413520] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[23434633398] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23436420744] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23438186211] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[23450062779] [INFO] [telnetd] [CPU2] telnetd: waiting for network stack
[23454889788] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1277)
[23456347233] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[23468518953] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=277
[23470222149] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[23471780376] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[23472967518] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23474342628] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23475964380] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=277 subj_lo=0
[23491332711] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1278)
[23492757453] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[23517563718] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([252, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[23567232744] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[23568480672] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[23570813112] [INFO] [telnetd::net_client] [CPU2] telnetd: connected to socket API (netd=29, write=31, read=32)
[23574895146] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[23592308421] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=29, our_write=33, our_read=34)
[23594281491] [INFO] [anther] [CPU1] anther: Connected to network stack
[23607433344] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[23921864934] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=642 watches=13 history=1024 journal=1024 symbols=296 drops=0
[23945940315] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[23977110366] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[24421251096] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[25445833413] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[25448608482] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[25565672484] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[26318704335] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=666 watches=13 history=1024 journal=1024 symbols=335 drops=0
[26406058998] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[26494151739] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[26497930767] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[27898472778] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[27929671374] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[27938291469] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[27940195041] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[27960459978] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28087745829] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[28089286665] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[28090750248] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28092265311] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=334 pred=0 subj_lo=0
[28138498443] [INFO] [bloom::paint_vm] [CPU3] [bloom] process_updates took 2211.945ms (rebuilds=0 pending=true)
[28189088136] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28203866757] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[28205341593] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[28206799698] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28208474316] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=339 pred=0 subj_lo=0
[28260533466] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[28345232256] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[28347008646] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[28352021775] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28354363719] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=340 pred=0 subj_lo=0
[28386483147] [INFO] [bloom] [CPU3] [bloom] acquire_buffer took 60.837ms
[28388833341] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[28399830261] [INFO] [bloom] [CPU3] [bloom] compositor loop gap 2452.270ms
[28402100067] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[28404334365] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[28409913477] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[28452988971] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[28454803542] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[28456607916] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28458538086] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=342 pred=0 subj_lo=0
[28616939373] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[28633936683] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x1200c000
[28635555960] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[28637448147] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[28725727140] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[28736189988] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[28740967596] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[28743468204] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[28745909577] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[28750796679] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[28766837484] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[28768744026] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[28783035600] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[28796443797] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x1200d000
[28798270413] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[28870133787] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[29164864476] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[29166672810] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[29465312877] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[29467944891] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[29471406393] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[29548807068] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[29554546296] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[29556970014] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[29574809220] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=714 watches=17 history=1024 journal=1024 symbols=355 drops=0
[30918304791] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[30919860081] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[31199800665] [
```
</details>
