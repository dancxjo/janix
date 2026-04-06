# ✅ Scenario: Server starts up

> Last run: 2026-04-05 18:40:52

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 4051ms | - [📜](./01/serial.log) - |
| 2 | Then I should see a message in the serial output that says "anther: Listening on port 80" | ✅ | 3240ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12707717418] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[12714546900] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[12718872837] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[12721453602] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[12723190062] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[12724253718] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[12725375619] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[12726377598] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[12727373076] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[12728399904] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[12729437094] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[12730477122] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[12731653539] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[12732785439] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[12733953177] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[12734998254] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[12736065276] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[12737082303] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[12738127446] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[12739145034] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[12740127972] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[12741123681] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[12742130610] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[12743148924] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[12744209511] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[12745220301] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[12746245710] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[12747343488] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[12748345335] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[12749381865] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[12750405294] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[12751438062] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[12752525940] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[12753388923] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[12754013382] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[12754885275] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[12755670345] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[12756444690] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[12757181415] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[12757946322] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[12758692716] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[12759461055] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[12760839432] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[12762424917] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[12763275789] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[12763848042] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[12764384094] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[12764930805] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[12765563151] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[12766230906] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[12767011983] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[12767679771] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[12768229617] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[12768859686] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[12769438605] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[12770030229] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[12770599677] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[12771244893] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[12771821469] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[12772479588] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[12773054745] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[12773646171] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[12774218424] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[12774808893] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[12775418931] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[12776037879] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[12776609505] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[12777198093] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[12777796185] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[12778425000] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[12779024907] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[12779617587] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[12780193767] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[12780785094] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[12781358568] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[12781987680] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[12782562837] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[12783156342] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[12783728991] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[12784320516] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[12784893429] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[12785501553] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[12786081792] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[12786671403] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[12787243029] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[12787893855] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[12788482113] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[12789096309] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[12789668463] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[12790259757] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[12790833825] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[12791428320] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[12792020802] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[12792645591] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[12793217547] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[12794047266] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[12794817585] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[12795445509] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[12796052247] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[12796645554] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[12797216091] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[12797862495] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[12798470289] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[12799066566] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[12799638555] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[12800229717] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[12800828304] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[12801743922] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[13058510553] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[13070159157] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[13076117373] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[13077837333] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[13079181324] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[13083770634] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[13085540127] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[13086662721] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[13087377534] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[13088081226] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[13088819304] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[13089859893] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[13090910943] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[13091672616] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[13092485967] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[13093229028] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[13093940607] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[13095219918] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[13096431018] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[13097186784] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[13098837576] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[13099816818] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[13100852655] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[13102540935] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[13104137310] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[13105104342] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[13105942443] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[13106947425] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[13500976269] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[13502109918] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[13505484927] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[13506439254] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[13507299267] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[13508903826] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[13521817386] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[13523231898] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[13524031059] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[13525785669] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[13526381286] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[13529048280] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[13537378536] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[13539177630] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[13553107095] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[13553724096] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[13570328442] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[13570945377] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[13573201521] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[13574464332] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[13575565212] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[13577919630] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[13578750669] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[13613767134] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62391800 ticks/sec), init_cnt=623918 for 100Hz
[13615303317] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[13616150757] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[13617419046] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[13623226815] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[13654240017] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[13655620902] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[13657589781] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[13659286212] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[13660586280] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[13663465959] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[13664896740] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[13686711060] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[13688067822] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[13689447024] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[13690739634] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[13691940537] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[13692967464] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[13693994655] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[13720425840] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[13721748348] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[13723017297] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[13724432634] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[13725741579] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[13727023794] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[13728314952] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[13729223772] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[13736119221] [INFO] [kernel::root] [CPU0] Spawning Root service...
[13737446943] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[13739704869] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[13741074897] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[13747433700] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[13749748518] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[13750847682] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[13752885762] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[13754663472] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[13756093527] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[13776851913] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[13781813232] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[13783941237] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[13785508440] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13820484744] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13850999382] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13854620967] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13860553707] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13863000987] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13867667649] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13871361339] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13874114001] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13875347409] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13876875309] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13883384757] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13887298095] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13891772664] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13894948617] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13896151566] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13897738899] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13900793940] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13911239529] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13912112181] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13919232954] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13920038616] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13949339184] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13950291564] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[14349091614] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[14919099096] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[14964280650] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=499 journal=424 symbols=51 drops=0
[15021514860] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[16468425336] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=449 watches=0 history=964 journal=773 symbols=98 drops=0
[17317919124] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[17469325500] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[17470747800] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[17609942064] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[17672728887] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[17699834031] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[17700727935] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[17701394073] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[17705130795] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[17725292442] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[17742345918] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[17744623908] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[17801098491] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[17826210699] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[17827017087] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[17832481392] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[17875950279] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[17897044308] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[17900772384] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[17901824226] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[17970227550] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[17971998726] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[18063722358] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[18131610024] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[18144153720] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[18148430388] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[18150767679] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[18179606676] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[18243740262] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[18249163416] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[18250285614] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[18251147409] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[18252063423] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[18252786750] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[18253488000] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[18254160276] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[18254785824] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[18255423252] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[18256088697] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[18256800474] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[18257664183] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[18258716850] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[18259442025] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[18260219802] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[18261070047] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[18261805122] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[18262464462] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[18263164425] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[18263819739] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[18264454659] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[18265094661] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[18265742814] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[18266407170] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[18267133764] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[18267795843] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[18268449012] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[18269159172] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[18269849433] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[18270514053] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[18271181214] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[18271855668] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[18272604405] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[18273347796] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[18274001130] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[18274763793] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[18275530020] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[18276334263] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[18277175334] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[18277973967] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[18278745903] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[18279550674] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[18281229516] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[18283085073] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[18284044251] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18293760342] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[18309162366] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[18314713032] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[18325605771] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[18326373549] [CONTRACT] [kernel] [CPU0] Spawning init process...
[18328894221] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[18331510824] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[18332263851] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
TUSER_TRAMPOLINE: PC=0xhin2000gOS Petals00 SP=0x800000 ARG0=0x600000

type 'help' for commands

petals> [18338322321] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[18340614369] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013376 RFLAGS_BEFORE=130 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[18366410370] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[18379245324] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[18381589116] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[18383642739] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[18394981011] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[18401290710] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[18406956051] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[18409011489] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[18415590369] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[18420237132] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[18422444106] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[18428744994] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[18430696152] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[18432756144] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[18434616354] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[18438970110] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[18440286909] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[18441735477] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[18443447022] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[18445106196] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[18446765271] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[18451844136] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[18500789703] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[18509717094] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[18516733620] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[18523991739] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[18528390639] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[18533489898] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[18540158241] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[18546991980] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[18553860369] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[18561030675] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[18568389972] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[18575822793] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[18583352337] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[18589982070] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[18596374203] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[18603080166] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[18609639213] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[18616021215] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[18622950918] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[18629326980] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[18635427987] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[18641880675] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[18648805230] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[18656276265] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[18662911938] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[18669380862] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[18675779133] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[18682462854] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[18688898283] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[18695324670] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[18700000308] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[18704355945] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[18710862126] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[18717071175] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[18724129215] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[18729625827] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[18735163689] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[18740817645] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[18746658348] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[18753215184] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[18759834819] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[18763607247] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[18784408863] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[18945244461] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[18956144361] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[18957053313] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18958802280] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18963440958] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18964826793] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18973161471] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[18978204729] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18979692699] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18981902808] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078912 RFLAGS_BEFORE=130 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[18989082618] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[18990149673] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[18991325628] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18993922266] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19000476858] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[19003036206] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19013382102] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[19018678437] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[19020192213] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[19022400573] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144448 RFLAGS_BEFORE=130 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[19027324668] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[19031687862] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[19034368518] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[19037966607] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19045218852] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:41:10 = 1775439670 unix_secs
[19047967488] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775439670, mono_ns=9523618120, offset=1775439660476381880ns
[19051144992] [INFO] [rtc_cmos] [CPU1] System clock anchored
[19068276876] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[19122478485] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[19129942854] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[19131247905] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19133937240] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19143398802] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[19147144368] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[19159512306] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[19164761022] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[19169661918] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19171789362] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210816 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[19177483611] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[19182263463] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[19184277288] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[19185260160] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19187695560] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19197035451] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19199883714] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19207238094] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[19210072200] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[19211692038] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19213307091] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277568 RFLAGS_BEFORE=130 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[19217523270] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[19219708002] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[19225255269] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[19228842138] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[19230486429] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[19231185600] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[19231986939] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[19233394719] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[19250737770] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[19252230162] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[19772214825] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19778975436] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[19782461391] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[19784052123] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[19788402546] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[19789938003] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[19792212132] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[19793792304] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[19794878961] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[19797249417] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[19798293966] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[19799236215] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[19800123882] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[19801165230] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[19802229909] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[19803385437] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[19808673225] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[19809992169] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19812820467] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19820165442] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19822839894] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19830417156] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[19833745602] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[19835243175] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[19837523640] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352976 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[19845514986] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[19857716967] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[19868032371] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[19882541184] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[19885515705] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[19893668289] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[19895416563] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[19896479427] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[19897502625] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[19898448702] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[19899386958] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[19900413918] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[19901711643] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[19903093122] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[19907104206] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[19909396782] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[19910864457] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[19913717043] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[19916475546] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[19918340112] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[19919252826] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19921012122] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19925054721] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[19926076236] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[19927717062] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19946799642] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[19949955861] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[19952022486] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[19952973150] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19954776303] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19959763989] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19961266281] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19968778434] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[19972280130] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0105668
[19973738796] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[19975762719] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500432 RFLAGS_BEFORE=130 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[19980174918] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[19981342095] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19984023378] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19985199597] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[19987027005] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[19992131742] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19993409964] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[19995649608] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[19996738641] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19998459591] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[20001544827] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[20003682006] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[20005759752] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[20008616661] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[20010114366] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[20013061563] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[20015238705] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[20016442413] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[20018817192] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[20020293150] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[20022297075] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[20024051784] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434192 RFLAGS_BEFORE=134 CR3_BEFORE=60096512 fs_base=0 gs_base=18446744071564586608
[20029767648] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[20464637820] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0105668
[20466167238] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[20468458098] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583216 RFLAGS_BEFORE=130 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[20472848022] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[20475717339] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[20476978434] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[20480587149] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[20481955329] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20485067757] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[20486524014] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[20487697164] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20490298092] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20498976498] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[20501925477] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[20506828749] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[20510694303] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[20513923122] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[20516615130] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[20518412178] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[20519741220] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20522294760] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20544086574] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20549170125] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[20669789943] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[21090691941] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[21092085828] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[21094291119] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[21096488391] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[21592050480] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[21595146969] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb01095c8
[21596818056] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21598433505] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369719072 RFLAGS_BEFORE=130 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[21605295723] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[21607193388] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[21608412111] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[21610269153] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653536 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[21616534368] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[21618208095] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[21621274719] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[21623061834] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[21627005070] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[21628736844] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[21630143502] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[21631916757] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[21637937475] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[21639718452] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[21641651064] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[21642515202] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21644435109] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21715451637] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[21740869194] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[21750623037] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[21754221225] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
[21756215910] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[21757734141] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[21758654346] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21760848483] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21761773374] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369784608 RFLAGS_BEFORE=130 CR3_BEFORE=79892480 fs_base=0 gs_base=18446744071564586576
[21771314796] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[21775631031] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[21782497209] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21784996563] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21789306462] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[21792756183] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[21800915367] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[21803691261] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[21805664793] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[21806510517] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21808052574] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21814386066] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[21816866412] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21824958606] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[21828923688] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010ac38
[21830358825] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21832355424] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915680 RFLAGS_BEFORE=130 CR3_BEFORE=80826368 fs_base=0 gs_base=18446744071564586640
[21836761419] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[21837858471] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21840586251] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[21841742901] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21843862392] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[21853792917] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21857659923] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[21865927446] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[21868909854] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010ac38
[21870525666] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21872464944] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981664 RFLAGS_BEFORE=130 CR3_BEFORE=80953344 fs_base=0 gs_base=18446744071564586576
[21877427088] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[21878449395] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21880656072] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21891156078] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[21894657015] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[21902888601] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[21906275424] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[21910308024] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[21911718576] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21914449557] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21953315208] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[21958905177] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[21967215831] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[21970572558] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010ca80
[21972173949] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21974191800] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113712 RFLAGS_BEFORE=130 CR3_BEFORE=81256448 fs_base=0 gs_base=18446744071564586640
[21979947297] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[21981192816] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21983289471] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[21984769785] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22002880053] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[22006524342] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[22008105438] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[22016965872] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[22020424239] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[22022291379] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f10e8
[22023422586] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[22024686354] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22027704798] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370202192 RFLAGS_BEFORE=134 CR3_BEFORE=81530880 fs_base=0 gs_base=18446744071564586576
[22032365784] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22035294072] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[22036446696] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[22038363204] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[22039416465] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22041775602] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[22043991123] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[22048756356] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22050343194] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22052146083] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22054073712] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=226 subj_lo=0
[22072066599] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[22073720724] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22075403493] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[22076377224] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[22078155891] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[22087098198] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[22097083173] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[22100297934] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[22104154743] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22106683401] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369850144 RFLAGS_BEFORE=130 CR3_BEFORE=80564224 fs_base=0 gs_base=18446744071564586608
[22113315807] [INFO] [nectar] [CPU2] NECTAR: Started.
[22115394576] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010ac38
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22117158657] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047920 RFLAGS_BEFORE=130 CR3_BEFORE=81100800 fs_base=0 gs_base=18446744071564586608
[22122174195] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[22124724501] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22126955994] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370268144 RFLAGS_BEFORE=134 CR3_BEFORE=81739776 fs_base=0 gs_base=18446744071564586608
[22132883619] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[22134151413] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[22135609848] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[22136763990] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[22138374522] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22140098409] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[22141988616] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=233 pred=0 subj_lo=0
[22152500601] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
[22155279432] [INFO] [fontd] [CPU3] FONTD: Service ready
[22164357336] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[22172316507] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[22185735165] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[22187868450] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[22189753872] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[22191445914] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[22194428223] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[22196218869] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[22197168675] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22198705980] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22200301101] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f8000 phys=0x4e93000
[22201344924] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22203238431] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[22204023237] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[22208085636] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[22211205357] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[22212753816] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[22215471531] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22217169909] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22224621870] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[22234805538] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22236545859] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22238271000] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22240263210] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=240 subj_lo=0
[22243282578] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[22257757104] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22259364171] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22260992457] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[22262697732] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22264501479] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[22268177019] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[22272109464] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[22276239183] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[22278624357] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[22281274092] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[22284376158] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[22285670154] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22287207525] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22289017212] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[22290237057] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22292065455] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[22293095583] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[22300418877] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=19, read=20)
[22308398145] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22310043591] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=21, read=22)
[22311366594] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22313083155] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22314993195] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[22332892857] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22334679741] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22336508403] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22338477678] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[22358992359] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[22372682046] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22385796411] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22388687013] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1250 backend=VirtIO-GPU
[22391485248] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[22392659982] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22395304866] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22409636205] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22411862286] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22477226871] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22485621840] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[22515106317] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=23, resp=26
[22517049291] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[22551461757] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[22567041189] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[22575011349] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[22578096453] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db4a8
[22579536771] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e2
[22581589998] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370506336 RFLAGS_BEFORE=134 CR3_BEFORE=82771968 fs_base=0 gs_base=18446744071564586640
[22586017278] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[22587117630] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22589474820] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[22590696711] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22596335388] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[22598352678] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22606689402] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[22609772097] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db4a8
[22610895450] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[22613255412] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370571872 RFLAGS_BEFORE=134 CR3_BEFORE=83820544 fs_base=0 gs_base=18446744071564586576
[22617556401] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[22619554518] [INFO] [echo] [CPU1] echo: starting up
[22622698362] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[22624628565] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[22642306764] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[22645260330] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[22653342525] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[22655440566] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[22660071060] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=19, RX port=22
[22664087424] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22676958447] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[22677950328] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[22679297190] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[22680252078] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[22681104039] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[22682001639] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22684270653] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22689033972] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[22690133433] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22691977374] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[22700137977] [INFO] [netd] [CPU3] NETD: Driver TX port=19, RX port=22, link_up=true, mtu=1500
[22701902982] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[22704213642] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[22706636304] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[22708907331] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[22709826150] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[22712378469] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[22714989198] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[22716281082] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22718700774] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22726546293] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22730501772] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[22738600071] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[22741907925] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f25e8
[22743646860] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22745665008] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370743904 RFLAGS_BEFORE=134 CR3_BEFORE=84320256 fs_base=0 gs_base=18446744071564586640
[22749090045] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[22751588244] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[22752891810] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22755372948] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22758933912] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[22760633544] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[22769073426] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[22772055174] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[22773905451] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[22779954153] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[22782263427] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[22784498517] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[22819106145] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[22831076928] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[22837684122] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22839491301] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22841303991] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[22847163801] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f25e8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22849201221] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370809440 RFLAGS_BEFORE=134 CR3_BEFORE=84467712 fs_base=0 gs_base=18446744071564586576
[22856359944] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[22886855310] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[22891413897] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[22897177149] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[22913211387] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[22918437069] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[22920306816] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[22929183684] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[22973570334] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[22978366884] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[23002890141] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[23010527628] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[23017941177] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[23026318722] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[23042630292] [INFO] [bloom] [CPU3] bloom: creating surface...
[23044366917] [INFO] [bloom] [CPU3] bloom: surface created!
[23045602965] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[23048749713] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[23052126834] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[23055640311] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[23056670769] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[23059636446] [INFO] [anther] [CPU1] anther: Connected to network stack
[23063385807] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[23072690718] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[23076056586] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[23078847495] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[23080647183] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[23082727140] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[23105008971] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1264
[23106860238] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[23118813828] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[23125017201] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[23129875230] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[23132205360] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[23134582383] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
[23141455953] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1264
T:0270 [23146948935] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[23159771679] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[23173012896] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[23184921672] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[23191067064] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[23197103688] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[23200774344] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
T:07D0 T:0640 T:F0B0 [23259016506] [INFO] [netd] [CPU3] NETD: DHCP configured ��� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[23263920966] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[23270481696] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[23273522151] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=1
[23277078825] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[23282134128] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=2
[23289729210] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=2)
[23299376496] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[23312252832] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[23316215373] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[23320116105] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[23324630571] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[23341530102] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[23344342329] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
T:1220 [23348933949] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[23351781486] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[23367137079] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[23370653493] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[23376704868] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[23389632585] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=283
[23392274631] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[23393970666] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[23395497642] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23397207834] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23398891329] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=283 subj_lo=0
[23423606052] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1279)
[23425894998] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[23447277942] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=244
[23449788549] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[23451687534] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[23453267541] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23454985851] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23456875860] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[23471901816] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[23483149239] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1282)
[23486321100] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[23507945142] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=290
[23509975203] [IN
```
</details>
