# ❌ Scenario: Inspecting CPU Configuration

> Last run: 2026-04-05 18:40:52

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 5261ms | - - - |
| 2 | And the anther server is ready | ✅ | 3773ms | - [📜](./02/serial.log) - |
| 3 | When I execute the GQL query "MATCH (c:dev.Cpu) RETURN c" | ✅ | 376ms | - [📜](./03/serial.log) - |
| 4 | Then the response status should be 200 | ✅ | 0ms | - - - |
| 5 | And the response body should contain "dev.Cpu" | ❌ | 1015ms | - [📜](./05/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11802736440] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11808411417] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11812213017] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11814269445] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11815510773] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11816170707] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11816863179] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11817472656] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11818082694] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11818756455] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11819378637] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11820013425] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11820749391] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11821446417] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11822187894] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11822827731] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11823482286] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11824115160] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11824757406] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11825391765] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11825995764] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11826603591] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11827216962] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11827839441] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11828525280] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11829162378] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11829795252] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11830480959] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11831104758] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11831752944] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11832394035] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11833039350] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11833693179] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11834334039] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11834954076] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11835711030] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11836454025] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11837216622] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11837952258] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11838743334] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11839487055] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11840235363] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11841552360] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11842957665] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11843753097] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11844328452] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11844866715] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11845432599] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11846011122] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11846562057] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11847103884] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11847651222] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11848206381] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11848774146] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11849350953] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11850039927] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11850619209] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11851215981] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11851809453] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11852402991] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11852978874] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11853570531] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11854144071] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11854737444] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11855324844] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11855921814] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11856498522] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11857093974] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11857672530] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11858278707] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11858857725] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11859457368] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11860036089] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11860634082] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11861212242] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11861821455] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11862398856] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11862993351] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11863564515] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11864155578] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11864736180] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11865325164] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11865895866] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11866485807] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11867059644] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11867652060] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11868235533] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11868828840] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11869401654] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11869994301] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11870569227] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11871163062] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11871746733] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11872342152] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11872918497] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11873514345] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11874086961] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11874692346] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11875262718] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11875854870] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11876425011] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11877016041] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11877586050] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11878188927] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11878760355] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11879350263] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11879955549] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11880745602] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12129288006] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[12140198829] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12145198065] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12146478300] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12147378210] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12151630953] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12153376191] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12154480833] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12155207163] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12155905146] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12156598245] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12157606824] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12158622234] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12159338730] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12160041201] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12160745586] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12161463270] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12162597975] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12163641501] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12164368425] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12165984831] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12166952919] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12167999646] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12169627371] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12171223911] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12172030827] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12172599384] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12173386137] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12559276851] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12560343510] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12563768052] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12564673143] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12565468113] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12567069306] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12579947787] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12581391669] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12582192711] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12583990848] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12584563167] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12587091660] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12595354332] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12597133824] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12610706790] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12611270298] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12627888438] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12628473363] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12630520584] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12631738680] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12632825865] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12635059965] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12635913114] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12670775502] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62309700 ticks/sec), init_cnt=623097 for 100Hz
[12672147015] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12672995577] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12674172093] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12679882974] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12710474964] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12711402528] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12713612373] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12715605078] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12717351570] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12720791622] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12722760237] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12742683426] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12744266799] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12745907889] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12747070611] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12748295934] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12749256102] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12750880593] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12779036127] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12781123542] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12782117403] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12784360050] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12785271708] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12786733905] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12787358496] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12788508678] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12795734358] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12796646313] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12798446298] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12799306377] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12806117577] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12808711740] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12810083781] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12812752128] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12814405923] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12816075360] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12829401288] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12833196618] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12835121145] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12836645415] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12858852006] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12878334513] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12880368105] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12883560195] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12885084201] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12887291142] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12889508676] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12891448878] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12892260744] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12893368587] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12899562720] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12901239681] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12903743094] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12908212053] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12910886175] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12911764173] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12912954219] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12923878803] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12924694695] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12931233909] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12932063199] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12959324466] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12960169794] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13316216001] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13802589339] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13830656961] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[13867252674] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[15167772708] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=449 watches=0 history=963 journal=772 symbols=97 drops=0
[15919774629] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[16012678671] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[16013707215] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[16123109640] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[16182727407] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[16212124005] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[16213033287] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[16213696917] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[16217206632] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[16233697194] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[16253960778] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[16256577018] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[16332271296] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[16356537549] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[16357602690] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[16363162167] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[16387551246] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[16406551128] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[16409785524] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[16410830304] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[16476112983] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[16477785489] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[16572045072] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[16637849283] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[16649081262] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[16652792244] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[16655125080] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[16724859063] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=184 drops=0
[16735849482] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[16740495123] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[16741498125] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[16742363022] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[16743498816] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[16744216599] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[16744895145] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[16745579169] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[16746232470] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[16746880986] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[16747558575] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[16748215638] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[16748883624] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[16749941538] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[16750674765] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[16751433600] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[16752111420] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[16753231539] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[16754037333] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[16754723634] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[16755385779] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[16756099470] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[16756752111] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[16757406996] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[16758061980] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[16758764880] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[16759444383] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[16760106099] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[16760824278] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[16761478140] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[16762146588] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[16762833549] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[16763516385] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[16764183513] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[16764859749] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[16765514436] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[16766304291] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[16767083817] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[16767870636] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[16768646334] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[16769542053] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[16770334647] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[16771125888] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[16772667384] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[16774488324] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[16775311443] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16783751589] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[16798777875] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[16803295179] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[16812716745] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[16813427070] [CONTRACT] [kernel] [CPU0] Spawning init process...
[16816313613] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[16821111978] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[16822122273] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [16828531632] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[16829441112] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013568 RFLAGS_BEFORE=130 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[16853768118] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[16858444218] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[16859999937] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[16861234962] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[16869734277] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[16875111495] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[16878928176] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[16880192868] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[16884741984] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[16888342152] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[16889623146] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[16894229319] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[16895353332] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[16896539847] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[16897998678] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16902048801] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16903216770] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16904548452] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16906094799] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16907740806] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16909117302] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16914422316] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[16962660495] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[16971516738] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[16977352986] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[16982951040] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[16986893682] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[16991489361] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[16997030754] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[17002783974] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[17008753410] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[17014845111] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[17021201637] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[17027453751] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[17034308115] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[17041222176] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[17050757856] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[17058331917] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[17066354349] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[17074001505] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[17081591109] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[17089121874] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[17096381049] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[17103998868] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[17112254412] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[17120602818] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[17127985842] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[17134228419] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[17140269300] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[17146833132] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[17153126199] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[17159382867] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[17163497637] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[17167538190] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[17173712886] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[17180069412] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[17185966578] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[17192265915] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[17198520207] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[17204689392] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[17211360606] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[17218093134] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[17224707489] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[17228559315] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[17251940310] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[17420230311] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[17427918453] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[17428796319] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17430777606] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17435778393] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17437316160] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17446298133] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[17451442866] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[17452832793] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17454718842] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079104 RFLAGS_BEFORE=130 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[17461616370] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[17464257723] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[17465302866] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17467560132] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17473099941] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[17475677439] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17485301856] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[17489749068] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[17492636766] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[17494137342] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[17496167469] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144640 RFLAGS_BEFORE=130 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[17503930125] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[17506183167] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[17508508182] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17513075877] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:44:15 = 1775439855 unix_secs
[17515038189] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775439855, mono_ns=8757288226, offset=1775439846242711774ns
[17516981427] [INFO] [rtc_cmos] [CPU1] System clock anchored
[17531208618] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[17581197711] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[17594009136] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[17595129915] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17597186772] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17603664936] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[17606427498] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[17614726701] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[17618180811] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[17621670792] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17623291554] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210688 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[17628299172] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[17632481460] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[17634332991] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[17635228413] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17637260751] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17644422015] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17646870648] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17655293964] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[17658807144] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[17660652900] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[17661577230] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17664096549] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277440 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[17669017938] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[17671592994] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[17677795311] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[17680356507] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[17686283901] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[17688575058] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[17690677191] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[17704940781] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[17707167885] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[18226350021] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18232591212] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[18236194416] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[18237804684] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[18242068284] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[18243570246] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[18245557176] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[18247078641] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[18248098440] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[18250390785] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[18251403885] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[18252341778] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[18253192947] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[18254023293] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[18254838822] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[18255911454] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[18261310551] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[18262642002] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18265170990] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18272768118] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18275145108] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18282942480] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[18286026660] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fab68
[18287487438] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[18290008473] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352832 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[18299037702] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[18313315086] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[18335436999] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[18345545361] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[18348740718] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[18356877033] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[18370660209] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[18372939585] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[18381964359] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[18383872056] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[18385280166] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[18386640690] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[18388245843] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[18389993622] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[18391605408] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[18393200694] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[18394830894] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[18405015981] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[18409209951] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[18413600964] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[18416826516] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[18418216938] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18420991677] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18427498254] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18430497294] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18442902258] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[18447410124] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[18450439458] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[18451774704] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18454438629] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18462868545] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18465316815] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18477765933] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[18482435499] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[18483832389] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[18485905911] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500336 RFLAGS_BEFORE=130 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[18491585838] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[18492693615] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18494706219] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[18495701565] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18496856664] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[18502736703] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[18504819696] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18506062014] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[18508361751] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[18509435175] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18511631127] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[18513839520] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[18516037320] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[18518863176] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[18520968906] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[18522112587] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[18523704738] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[18528242733] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[18531156039] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[18533702088] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[18536736834] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[18539371884] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434256 RFLAGS_BEFORE=134 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[18547828992] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[18908559945] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[18910574727] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583120 RFLAGS_BEFORE=130 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[18916296828] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[18917088300] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[18918594849] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[18920244486] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[18922867392] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[18924032490] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[18968687991] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[18973455897] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18977620233] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[18978762759] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18981228057] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18991807230] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[18995415912] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[19006732437] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[19011245286] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[19014507369] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[19017085329] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[19018134696] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19020678402] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19048157799] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[19051354179] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19057849041] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[19950609921] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[19953566655] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[19955057034] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19956911931] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716800 RFLAGS_BEFORE=130 CR3_BEFORE=68890624 fs_base=0 gs_base=18446744071564586640
[19961645484] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[19963309080] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[19965145662] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[19966249017] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[19967418933] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[19969220700] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650848 RFLAGS_BEFORE=134 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[19976687709] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[19979046153] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[19985101653] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[19986680373] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[19988094720] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[19989849099] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[20006778924] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[20009734239] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[20012659425] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[20014144656] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20017097859] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20088995817] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[20115042123] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[20122681095] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[20125580442] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[20127025116] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20128844736] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369783040 RFLAGS_BEFORE=134 CR3_BEFORE=79892480 fs_base=0 gs_base=18446744071564586576
[20133427380] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[20134197006] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20136204363] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20137320357] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[20141612436] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[20147820165] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20150148117] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20163357687] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20166779886] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[20178215178] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[20182368360] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[20185173195] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[20186635359] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20189282883] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20195808567] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20199038442] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20207573595] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[20211032160] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[20212430007] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20214583752] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915264 RFLAGS_BEFORE=130 CR3_BEFORE=80826368 fs_base=0 gs_base=18446744071564586640
[20218968792] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[20220151149] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20222812368] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20224187544] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[20226322248] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[20231873871] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20235918252] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[20243153106] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[20245647081] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[20247099312] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20248757166] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981760 RFLAGS_BEFORE=130 CR3_BEFORE=80953344 fs_base=0 gs_base=18446744071564586576
[20253385911] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[20254135638] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20255650536] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20265631089] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[20270568054] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[20281619919] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[20285999745] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[20289489924] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[20290806393] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20293375872] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20334294354] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20342089416] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[20350102575] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[20354000964] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cc18
[20355474348] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20357462895] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113920 RFLAGS_BEFORE=130 CR3_BEFORE=81256448 fs_base=0 gs_base=18446744071564586640
[20362026630] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[20363133120] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20365046328] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[20366986134] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20392146555] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[20394221034] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[20398792029] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20400771798] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20401907493] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20404369590] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20407056219] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[20413676316] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[20417795838] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[20420778477] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[20421962946] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20424301920] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[20425889682] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20427791604] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370198160 RFLAGS_BEFORE=134 CR3_BEFORE=81530880 fs_base=0 gs_base=18446744071564586576
[20437197231] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[20439500664] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[20444601375] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20446309158] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20448163560] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20450055945] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20451650637] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20453278593] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=229 pred=0 subj_lo=0
[20455554603] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20457484806] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[20467182252] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20468857992] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20470550694] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20472415458] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[20473398693] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[20477783898] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[20478947016] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[20480666382] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[20482204974] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[20485328325] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[20494314225] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[20496740550] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[20499154335] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[20501391372] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20503094073] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849184 RFLAGS_BEFORE=134 CR3_BEFORE=80564224 fs_base=0 gs_base=18446744071564586608
[20507872638] [INFO] [nectar] [CPU2] NECTAR: Started.
[20509840956] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20511718689] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047840 RFLAGS_BEFORE=134 CR3_BEFORE=81100800 fs_base=0 gs_base=18446744071564586608
[20517014430] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[20518910808] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db4e0
[20520121743] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20522156886] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370272320 RFLAGS_BEFORE=130 CR3_BEFORE=81739776 fs_base=0 gs_base=18446744071564586608
[20526126753] [INFO] [fontd] [CPU3] FONTD: Service ready
[20527498794] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[20530269243] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[20531937195] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[20542152114] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[20544339915] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[20546195802] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[20548075614] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[20550200352] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[20551406931] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[20553116595] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4e92000
[20555161902] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[20557308948] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[20560672209] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[20562077250] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[20564858226] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20566605246] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20568312600] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20570125356] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[20575099281] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[20582723370] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20584309977] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20586551865] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[20591841501] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20593146915] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20594677125] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20596473744] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=235 subj_lo=0
[20601961446] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[20607169671] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[20609462544] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20611179270] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20613013047] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[20614364826] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20616709113] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[20618111811] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=236 subj_lo=0
[20620451544] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[20623128108] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[20625736857] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[20628123978] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[20629781436] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[20631841824] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20633601087] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20635439814] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20637391533] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=237 subj_lo=0
[20640723873] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=19, read=20)
[20646770166] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20648334498] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20650060365] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=21, read=22)
[20651438709] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20653423296] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=238 subj_lo=0
[20655543216] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[20667337119] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([230, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20682330108] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20684075709] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20685928758] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20687978223] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[20691763257] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[20706886761] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([230, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20718570345] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1256 backend=VirtIO-GPU
[20721281526] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[20722495893] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20725349469] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20745847254] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20747686443] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20764528323] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[20771181585] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([230, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20786117220] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=23, resp=26
[20787904335] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[20852700297] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([230, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20855146422] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20856803187] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20934075525] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[20948738217] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[20956481667] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[20959052763] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db598
[20960351907] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e8
[20962134633] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370502016 RFLAGS_BEFORE=130 CR3_BEFORE=82505728 fs_base=0 gs_base=18446744071564586640
[20966465421] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[20967340185] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20969057109] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20969984904] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[20973661929] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[20975220387] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20984445075] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[20987140053] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[20988138039] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db598
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[20990530176] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370567552 RFLAGS_BEFORE=130 CR3_BEFORE=83554304 fs_base=0 gs_base=18446744071564586576
[20995191756] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[21004024701] [INFO] [echo] [CPU1] echo: starting up
[21007635066] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[21009588501] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[21022822161] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([230, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21028523802] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[21030905016] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[21047816658] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[21050008815] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[21058390617] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[21060593268] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[21063053121] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[21064119747] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21066091695] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[21067735425] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21075563586] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21078388452] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[21080351688] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([230, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21088993200] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[21091618317] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[21093177369] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[21095770146] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[21099073776] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[21102292893] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[21104883261] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[21107342355] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[21108474882] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21111030237] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21122680029] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21128272605] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[21139460232] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[21142749144] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00fff90
[21144330141] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21146326509] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[21147751647] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370704768 RFLAGS_BEFORE=130 CR3_BEFORE=84320256 fs_base=0 gs_base=18446744071564586640
[21152720886] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[21153911361] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21156163743] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21160733550] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[21162436284] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[21168243624] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[21173668857] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[21177159597] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[21179043732] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[21188916111] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=19, RX port=22
[21200633454] [INFO] [netd] [CPU3] NETD: Driver TX port=19, RX port=22, link_up=true, mtu=1500
[21203470398] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[21208189959] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[21215877309] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[21246579717] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[21248720955] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[21262645008] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21265248378] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370770304 RFLAGS_BEFORE=130 CR3_BEFORE=84467712 fs_base=0 gs_base=18446744071564586576
[21272328957] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[21280718679] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[21285698808] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[21312195498] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[21319267068] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[21321289275] [INFO] [anther] [CPU1] anther: Connected to network stack
[21325777638] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[21360491427] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([237, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[21373007832] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[21408137124] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[21411272553] [INFO] [bloom] [CPU3] bloom: creating surface...
[21413019276] [INFO] [bloom] [CPU3] bloom: surface created!
[21414516123] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[21418815396] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[21433366680] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1264
[21434891148] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[21454119423] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[21459402261] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[21461311344] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[21463138323] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
[21464496570] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[21473133627] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[21474853224] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[21483672408] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[21529096215] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[21533762514] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1264
T:0270 [21543521538] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[21546021420] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[21558437637] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[21560999130] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[21573817353] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[21586727877] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[21598044534] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 5)
T:07[D0 21607039245] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
T:0640 [21609998685] [INFO] [virtio_neT:td] [CPU2F0B0 ] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[21625855647] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[21635843130] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[21641139828] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[21644245656] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[21646647165] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[21649743390] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[21667346547] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[21669857220] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[21684730023] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=282
[21686381937] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[21687875055] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[21689333061] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21690979596] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21692793804] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=282 subj_lo=0
[21706294170] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[21719862351] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[21723061569] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[21726275901] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[21729476439] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[21736320936] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[21740693535] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[21742105506] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[21744726234] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[21752968908] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1278)
[21754559442] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
T:1220 [21768946452] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[21771770460] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[21784835820] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=243
[21787097541] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[21813656436] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[21815235387] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21816955248] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21818894196] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[21838147749] [INFO] [netd] [CPU3] NETD: DHCP configured ��� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[21844077915] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[21851059296] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[21857040579] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[21861339324] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[21865281141] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[21870092442] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[21875953935] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1279)
[21877928358] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[21901464024] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[21905387064] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[21909490284] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[21912774246] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=292
[21915314883] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[21917013855] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[21918671709] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21920461068] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21922581714] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=292 subj_lo=0
[21945499950] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[21948611652] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[21952773414] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1281)
[21953994348] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[21957336621] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[21978360591] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[21984971811] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[21990888084] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[22011310893] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=74
[22014243933] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=21
[22019318409] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[22023584187] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[22028514618] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[22034680041] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[22051056456] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[22053911748] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[22058294280] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22066922790] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22069793988] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=149
[22071579024] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=21
[22077161337] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[22081954125] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[22125373149] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[22130451552] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[22132689150] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:58692 on listener 1
[22443336795] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22488664836] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[22493693376] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[22539783453] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[22547354346] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=3
[22560290907] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 37 (user thread) assigned to CPU 1
[22562996346] [INFO] [anther] [CPU1] anther: Thread spawned TID=37 for conn_handle=3
T:5EE0 [22585359918] [INFO] [anther] [CPU1] anther: Worker thread TID=37 starting for conn_handle=3
[22617977250] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=33, our_read=34)
[22620504951] [INFO] [anther] [CPU1] anther: Worker TID=37 connected to netd OK
[22635144048] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=664 watches=13 history=1024 journal=1024 symbols=313 drops=0
[22761756072] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[22915729584] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[23082128487] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[23483166102] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[23761126257] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[23763135627] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[23770645899] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[23773118061] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 85 bytes on conn_handle=3
[23785071453] [INFO] [anther] [CPU1] anther: GET /health Http11
[23801622768] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[23805044373] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[23810506170] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[23823368811] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[23826466224] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[23828765400] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[23832281946] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23901343422] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[24062368968] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[24304665363] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[24537961404] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[24607213257] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[24617619510] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[24619337754] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[24620677620] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[24622033887] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[24623832684] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[24635077236] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[24636277281] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[24637619688] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[24639307308] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=342 pred=0 subj_lo=0
[24746713794] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[24809179560] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=715 watches=15 history=1024 journal=1024 symbols=343 drops=0
[24950640297] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[25154949336] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[25227661305] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[25229476107] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[25232068686] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[25447996200] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[25475542422] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25482536376] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[25484551290] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[25487976162] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[25493488680] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[25545006663] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[25549713090] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[25553781627] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[25556547522] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[25560070536] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[25570022379] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 0 bytes on conn_handle=3
[25582176312] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[25584691671] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[25601400165] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[25705434447] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25767704424] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[25769541171] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25771275321] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25773248259] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=307 pred=0 subj_lo=0
[25837170546] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=0e57f834b3afbcb1)
[25838677128] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25973438601] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26116792548] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[26177169645] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26181982101] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[26271435168] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[26290080432] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26372446716] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[26525494446] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26545515480] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[26562085704] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12232000
[26563961490] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[26565978219] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[26635558158] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[26646579597] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[26651546790] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[26654143197] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[26656880811] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[26661597171] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[26667307227] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[26682421194] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[26684462442] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[26686918005] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[26690304630] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[26692286280] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[26695429299] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[26696956935] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=1 (16384b)
[26717611338] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x122c3000
[26719529892] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[26819357499] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=c1e9a7b90a789e5c)
[27239750868] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[27602858415] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[27704987706] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=783 watches=16 history=1024 journal=1024 symbols=364 drops=0
[27914248692] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[28287258813] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[28551853407] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[28554267918] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[28558062819] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28588080972] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28591429482] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[28597928568] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[28599697368] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[28601346873] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[28603640571] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28606327464] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=149
[28608362805] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=21
[28611107778] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[28627473930] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28629403803] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[28728657507] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[28908824604] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[28912386954] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[28913647554] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[28914551028] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:58700 on listener 1
[28922729649] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=4
[28932970704] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 38 (user thread) assigned to CPU 1
[28934832069] [INFO] [anther] [CPU1] anther: Thread spawned TID=38 for conn_handle=4
T:5EE0 [28975300068] [INFO] [anther] [CPU1] anther: Worker thread TID=38 starting for conn_handle=4
[28993740831] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[29001867840] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29008038180] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=35, our_read=36)
[29011254426] [INFO] [anther] [CPU1] anther: Worker TID=38 connected to netd OK
[29025873294] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[29028405582] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[29134830384] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[29138333169] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29142163116] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 85 bytes on conn_handle=4
[29144789784] [INFO] [anther] [CPU1] anther: GET /health Http11
[29158531413] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[29160594804] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[29165781777] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[29185744896] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[29188628733] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[29193475443] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[29201071416] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[29238178662] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[29250418923] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[29262873882] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[29265704622] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[29270296506] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[29278754670] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[29281251549] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[29285558874] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[29297186028] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[29298707361] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[29300508105] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29312491593] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=308 pred=0 subj_lo=0
[29319545904] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[29322823068] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[29325958860] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[29327445246] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[29330103660] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[29332701090] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[29333804445] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[29335186122] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[29336869980] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[29345025402] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29354107398] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 0 bytes on conn_handle=4
[29364796791] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[29372215653] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[29376453183] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[29380579569] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[29388080304] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=177
[29390035158] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 167 byte frame (171 encoded) to netd rx_port=21
[29394144285] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (171 bytes sent)
[29402974293] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[29405507967] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 167 bytes
[29410933959] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[29413708236] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:58712 on listener 1
[29443482915] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[29449971045] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=5
[29451167856] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[29452671336] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[29454323052] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[29456083008] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29457979650] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=309 pred=0 subj_lo=0
[29461988523] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 39 (user thread) assigned to CPU 1
[29464507149] [INFO] [anther] [CPU1] anther: Thread spawned TID=39 for conn_handle=5
[29506982934] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[29508658872] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[29510485785] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29512410411] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=338 pred=0 subj_lo=0
[29527432539] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
T:5EE0 [29561680236] [INFO] [anther] [CPU1] anther: Worker thread TID=39 starting for conn_handle=5
[29697372969] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29787710040] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=37, our_read=38)
[29791952223] [INFO] [anther] [CPU1] anther: Worker TID=39 connected to netd OK
[29809738398] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[29812554255] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[29837331810] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29838553371] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[29878467003] [INFO] [anther] [CPU1] anther: Worker TID=39 got first 113 bytes on conn_handle=5
[29881655265] [INFO] [anther] [CPU1] anther: POST /api/v1/query Http11
[29883084396] [INFO] [anther] [CPU1] anther: Request body size: 26 bytes
[29892789003] [INFO] [phloem::executor] [CPU1] phloem: calling find for kind: dev.Cpu
[29959068777] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30076297680] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30091415871] [INFO] [phloem::executor] [CPU1] phloem: find returned 4 candidates
[30193347030] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30306959826] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30312614244] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=1
[30392423556] [INFO] [phloem::executor] [CPU1] phloem: discovered 4 nodes
[30439521123] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 210 bytes - TCP ACK
[30443254644] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 210 bytes
[30452080461] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=70
[30454132995] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[30457299114] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30462645708] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30482166132] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[30513850059] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30558305514] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 222 bytes - TCP ACK
[30561474405] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 222 bytes
[30573111690] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=70
[30576091887] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[30578537319] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30583234935] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30589394187] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=70
[30591762399] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[30594849450] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30606375954] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30609451851] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[30611975163] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[30722022408] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30770930949] [INFO] [anther] [CPU1] anther: Worker TID=39 got first 0 bytes on conn_handle=5
[30864123741] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30928392627] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[30956259015] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[30959065302] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[30989789259] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[31085768802] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[31090455990] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[31183845495] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[31291966893] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[31433322393] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[31575008322] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[31680735768] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[31703453562] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[31764778947] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[32280200106] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[32360216988] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[32946536073] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[32984201250] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[33075755460] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=907 watches=19 history=1024 journal=1024 symbols=366 drops=0
[33631799718] [INFO] [flytrap] [CP
```
</details>
