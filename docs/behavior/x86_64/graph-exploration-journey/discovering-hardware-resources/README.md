# ✅ Scenario: Discovering Hardware Resources

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 5060ms | - - - |
| 2 | And the anther server is ready | ✅ | 1992ms | - [📜](./02/serial.log) - |
| 3 | When I execute the GQL query "MATCH (n:dev.Cpu) RETURN n" | ✅ | 1514ms | - [📜](./03/serial.log) - |
| 4 | Then the GQL result should have at least 1 row | ⏭️ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11141459604] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11147006772] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11150779596] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11152807578] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11153972445] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11154595650] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11155278255] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11155856679] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11156439030] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11157046230] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11157630957] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11158224297] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11158974156] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11159631648] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11160312207] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11160916668] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11161534329] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11162171691] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11162785689] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11163378600] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11163954087] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11164538781] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11165159610] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11165753148] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11166380709] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11166977118] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11167567356] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11168205873] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11168817792] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11169415455] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11170032159] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11170641603] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11171287875] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11171926458] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11172514716] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11173203558] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11173900584] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11174607180] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11175413205] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11176152933] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11176851114] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11177555895] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11178954468] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11180386470] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11181149199] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11181734982] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11182241367] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11182755210] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11183297070] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11183809791] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11184316737] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11184860082] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11185370988] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11185902552] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11186441112] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11187001089] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11187538494] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11188128204] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11188672605] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11189233803] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11189774739] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11190331845] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11190872517] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11191466517] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11192118366] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11192679861] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11193219543] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11193973626] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11194532712] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11195122059] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11195663061] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11196222807] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11196794664] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11197359921] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11197905015] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11198499807] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11199042228] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11199607056] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11200151391] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11200713018] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11201285040] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11201848614] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11202393906] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11202955764] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11203503135] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11204065026] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11204646222] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11205208575] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11205752580] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11206314702] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11206857354] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11207430762] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11208008394] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11208572628] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11209116534] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11209678425] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11210220285] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11210781351] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11211336906] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11211899853] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11212443330] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11213006376] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11213549226] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11214109665] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11214664461] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11215226847] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11215768872] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11216636904] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11455041807] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[11467539600] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11473101354] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11474356674] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11475261039] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11479971294] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11481725178] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11482812528] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11483496915] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11484169059] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11484883608] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11485869120] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11486826780] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11487513708] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11488441767] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11489139552] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11489824632] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11491028505] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11492102655] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11492824365] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11494406616] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11495360877] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11496355068] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11498055789] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11499626589] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11500404894] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11500953882] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11501770467] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[11875190943] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[11876185860] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[11879453718] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[11880362307] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[11881131669] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[11882650461] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[11895202869] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[11896525674] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[11897278173] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[11898950811] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[11899500987] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[11901983445] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[11910338154] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[11912101146] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[11926088097] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[11926713051] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[11944384221] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[11945122596] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[11947800249] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[11949370587] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[11950750845] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[11953672005] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[11954701011] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[11989805157] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62334700 ticks/sec), init_cnt=623347 for 100Hz
[11991394437] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[11992259037] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[11993424267] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[11999813199] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12031083933] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12032238108] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12034643544] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12036230811] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12037319976] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12040076136] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12042046533] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12061736841] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12063766044] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12064737663] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12066658098] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12067438383] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12068847549] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12069584934] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12094810728] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12095747829] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12096600252] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12097606455] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12098334369] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12099288894] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12099958299] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12100865964] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12107782698] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12109256016] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12111193776] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12112026003] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12115656333] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12117409986] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12118288116] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12119723220] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12120819183] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12121995204] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12134477256] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12139004790] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12141018648] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12142211829] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12168104355] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12187071039] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12189122682] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12192644871] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12194675691] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12197086968] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12199915563] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12201789864] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12202577277] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12203632155] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12210679404] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12212704053] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12215196906] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12217690782] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12220392558] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12223084533] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12224070408] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12235985421] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12236781843] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12243203148] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12243982938] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12274115073] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12275169951] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[12638287464] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13124898270] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13149075753] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=497 journal=422 symbols=51 drops=0
[13185598800] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[14410341858] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=446 watches=0 history=958 journal=767 symbols=92 drops=0
[15165371529] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15264494058] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15265868838] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[15369315126] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[15429656946] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[15454728663] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[15455657514] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[15456294744] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[15460349520] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[15475246578] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[15491501454] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[15497377863] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[15549905514] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[15568450590] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[15569289087] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[15573007296] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[15597275232] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[15614801103] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[15618769584] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[15619717476] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[15682387809] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[15684080676] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[15773186484] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[15837601329] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[15850746318] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[15855980052] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[15858271077] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[15860753007] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=583 watches=0 history=1024 journal=1024 symbols=181 drops=0
[15927506166] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[15933670203] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[15934707888] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[15935618886] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[15936511965] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[15937181502] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[15937849191] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[15938879385] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[15939684255] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[15940301982] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[15940944624] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[15941586870] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[15942241095] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[15942898323] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[15943691016] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[15944522220] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[15945169779] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[15945878322] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[15946602408] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[15947254356] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[15947906733] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[15948515253] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[15949153737] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[15949841358] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[15950498586] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[15951330120] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[15952011834] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[15952648569] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[15953332164] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[15953953851] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[15954618339] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[15955262499] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[15955911510] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[15956544879] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[15957182505] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[15957822045] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[15958554975] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[15959293119] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[15960039678] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[15960797160] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[15961569162] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[15962313444] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[15963065118] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[15964684164] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[15966414585] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[15967199556] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[15975312012] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[15990595467] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[15994858704] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[16004545425] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[16005327360] [CONTRACT] [kernel] [CPU0] Spawning init process...
[16007500707] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[16010286270] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[16011069426] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
ThingOS PetalsUSER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000

type 'help' for commands

petals> [16017287121] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[16017934812] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[16033935555] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[16036283703] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[16040458236] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[16041569511] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[16050669096] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[16056055785] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[16059519498] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[16061021889] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[16065497448] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[16068765966] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[16073234166] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[16077648015] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[16078712100] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[16079785755] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[16080913530] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16084333584] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16085540097] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16086777894] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16088549796] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16090051362] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16091376609] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16095906354] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[16142105628] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[16149551121] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[16156535340] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[16163207214] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[16166194275] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[16170432960] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[16174900203] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[16179273594] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[16183566366] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[16189131156] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[16194034890] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[16199317035] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[16204440483] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[16208774373] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[16212949203] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[16217936097] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[16222642656] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[16227901998] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[16234537704] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[16239203706] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[16243905579] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[16248508221] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[16255073076] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[16261340865] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[16266747222] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[16271963433] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[16277433348] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[16283023746] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[16288913025] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[16293892725] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[16297499196] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[16301177442] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[16306677717] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[16312100475] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[16316847459] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[16321433865] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[16326841707] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[16332681123] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[16338941586] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[16344379161] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[16350310845] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[16353467460] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[16372719759] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[16508793972] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[16516013613] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[16516790631] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16518488778] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16523012583] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[16524435576] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16533613404] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[16539642141] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16541108397] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16542915576] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[16550049879] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[16551135942] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[16551887847] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16553628432] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16558086105] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[16559893317] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16567175493] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[16570819551] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[16571575482] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[16572513705] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[16573859016] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[16579684605] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[16581302166] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[16583602299] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[16587909624] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:53:17 = 1775436797 unix_secs
[16589609454] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775436797, mono_ns=8294580541, offset=1775436788705419459ns
[16591228104] [INFO] [rtc_cmos] [CPU1] System clock anchored
[16603608021] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[16645473801] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[16655367729] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[16656266187] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16657998819] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16663954230] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[16666522323] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[16674117702] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[16677130701] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[16680502740] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16681997772] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210800 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[16686544347] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[16690952421] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[16692699375] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[16693463259] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16695353499] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16702452360] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[16704829944] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[16711798422] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[16714708230] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16716004536] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16717469934] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277264 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[16720357467] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[16722193554] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[16724172168] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[16731312312] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[16732572120] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[16733687883] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[16734863970] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[16736684052] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[16752497817] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[16754151084] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[17253028584] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17263903668] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[17266697316] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[17268450375] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[17272621278] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[17274051762] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[17275953123] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[17277577647] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[17278581540] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[17280842568] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[17281808148] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[17282737626] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[17283558501] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[17284323474] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[17285156856] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[17286364194] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[17298627126] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[17300076783] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17302512117] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17309626752] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17311896855] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17318900841] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[17322591099] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[17324121342] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[17326285878] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369356976 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[17335496871] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[17343926754] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[17358952083] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[17365768959] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[17367455424] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[17372503467] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[17380875237] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[17381984928] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[17389060458] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[17389905192] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[17390703462] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[17391487113] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[17392125069] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[17393221164] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[17393964588] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[17394763980] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[17395545420] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[17834861550] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[17839348098] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[17844205203] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[17847199359] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[17848618590] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17851327395] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17856977490] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[17859414111] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17870363082] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[17874241902] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[17877390960] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[17878535862] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17880961230] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17888363064] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17890624191] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17903034105] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[17906898009] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[17908803000] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[17910785541] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500656 RFLAGS_BEFORE=134 CR3_BEFORE=68513792 fs_base=0 gs_base=18446744071564586640
[17916067488] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[17917368579] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17919046530] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[17920317822] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[17921197239] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17922260796] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[17926801167] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[17928664842] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[17929352265] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17930658174] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[17932525116] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17933289000] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[17934638700] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[17936183562] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[17938546032] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[17940388653] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[17941605297] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[17944930740] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
[17946293574] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[17947068315] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[17949127053] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583312 RFLAGS_BEFORE=130 CR3_BEFORE=68894720 fs_base=0 gs_base=18446744071564586576
[17953373130] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[17955688707] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[17956930893] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[17959072032] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[17960739126] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[17962722459] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434608 RFLAGS_BEFORE=134 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[17967218709] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[17970555768] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[17980912125] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1241
[17981790717] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1240)
[17983021914] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[17984664819] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[17988911688] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17991197565] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[17992079787] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17993854560] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18000519141] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[18003297642] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[18010493424] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[18013550907] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[18016229286] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[18017655876] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[18018393987] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18019926045] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18039244542] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18044189493] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[18064584681] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[18068324967] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
[18069573357] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18071209530] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716448 RFLAGS_BEFORE=130 CR3_BEFORE=69156864 fs_base=0 gs_base=18446744071564586640
[18076002417] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[18077597307] [INFO] [netd] [CPU3] NETD: Starting network stack service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[18079224075] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[18080148702] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650656 RFLAGS_BEFORE=130 CR3_BEFORE=69021696 fs_base=0 gs_base=18446744071564586608
[18087074445] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[18088673427] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[18092110542] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[18093878187] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[18098656587] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[18100561974] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[18101905041] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[18103915170] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[18106474716] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[18108842697] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[18110648424] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[18111537279] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18113343039] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18179193384] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[18203992686] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[18211760490] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[18214453719] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[18215420091] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18216658350] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369782960 RFLAGS_BEFORE=130 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[18219489882] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[18220202055] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18221800146] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[18222567660] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18224239935] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[18228248214] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18229688829] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18246814047] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18249925353] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[18256891719] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[18259693419] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[18261746382] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[18262435323] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18263886432] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18269475246] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18271834548] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18278676933] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[18281288751] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b030
[18283212486] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18284526579] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369914992 RFLAGS_BEFORE=130 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[18287445231] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18288129618] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18289263168] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[18289958577] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18290667153] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[18297689982] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18301116273] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18307888038] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[18310764384] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b030
[18311907042] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18313766823] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981520 RFLAGS_BEFORE=134 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[18317379135] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[18318117477] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18319814106] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18325746153] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[18326935110] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[18328621674] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18329745555] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18330618471] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[18333729810] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[18340898037] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[18343957863] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[18346339770] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[18347188794] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18348867339] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18373959978] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18379035345] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[18385905615] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[18388533570] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010df80
[18390244389] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18392005764] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113568 RFLAGS_BEFORE=130 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[18395144097] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[18396533034] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18398506500] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[18399752910] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18415353891] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[18416905815] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[18420028803] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18422999067] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18425246400] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18427471227] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18429564945] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[18430486965] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[18433132014] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[18434623614] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18436489599] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370197920 RFLAGS_BEFORE=134 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[18440912094] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[18441720924] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18443021124] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[18443757750] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18444528366] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[18453068700] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18454080744] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18455266962] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18456277422] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[18459375858] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18460331637] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18461530395] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18463175049] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[18479330958] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[18489187134] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[18492420969] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18493534521] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18494750835] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18496110072] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[18498749115] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[18501390204] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[18502968429] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18504290046] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18505623906] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18507173652] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[18509340102] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18510960963] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369848752 RFLAGS_BEFORE=130 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[18518971713] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b030
[18522949434] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18523968375] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[18524777766] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18525846207] [INFO] [fontd] [CPU3] FONTD: Service ready
[18526438425] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18528217620] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[18532810296] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047776 RFLAGS_BEFORE=130 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[18539137749] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[18541214472] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18542804709] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370266672 RFLAGS_BEFORE=134 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[18549194730] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[18550657191] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[18552109092] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[18557807928] [INFO] [nectar] [CPU2] NECTAR: Started.
[18559611939] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18561398262] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[18563465613] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18581168562] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[18586281549] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[18595171749] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18596650479] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18598116174] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18599754195] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1250 backend=VirtIO-GPU
[18600642720] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=235 subj_lo=0
[18603585198] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[18604492302] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18606127551] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18610667229] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18612046068] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18613602942] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18615204696] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[18623708433] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18625102815] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18626742915] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18628367307] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[18638197611] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18639707493] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18641202261] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18642850017] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[18666385485] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[18683630394] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[18684746619] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[18736450623] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18738283311] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18742387851] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[18758482281] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[18766152570] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[18768629418] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db558
[18770108676] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e2
[18771788574] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370360784 RFLAGS_BEFORE=130 CR3_BEFORE=72052736 fs_base=0 gs_base=18446744071564586640
[18776557866] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[18777289014] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18778874235] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18780470544] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[18783213438] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18784523439] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18791295006] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[18793215111] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db558
[18794940681] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[18796886790] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370426320 RFLAGS_BEFORE=130 CR3_BEFORE=73101312 fs_base=0 gs_base=18446744071564586576
[18800255727] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[18801729408] [INFO] [echo] [CPU1] echo: starting up
[18804622485] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[18809840016] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[18811435665] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f4000
[18812763486] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[18813655476] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f4000
[18815122458] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f7000
[18817104207] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276783104)
[18818449452] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[18820341408] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f9000 phys=0x4614000
[18821860827] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[18824180034] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[18826596690] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[18828029616] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[18832802373] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[18834752772] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[18837866619] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[18848316432] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[18858174291] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18859073508] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[18860092449] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18860886363] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[18862750599] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[18863514021] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[18865797060] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[18867548337] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[18869282883] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[18871362180] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[18872204307] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[18873035148] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[18874243212] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[18875426493] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[18883912707] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[18886235148] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[18888124200] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[18890035626] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[18892706778] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[18906223644] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18911691843] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[18923380047] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[18925198413] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[18926522307] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[18928151979] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[18929079774] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18930773103] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18936194244] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18937951032] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18946000161] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[18949498392] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[18951118329] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[18952621347] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[18954341802] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18955157331] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18956705724] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18958963980] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[18960105186] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[18964595034] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18968207907] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18974644425] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[18976798302] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[18979478628] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[18980944851] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[18982600494] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18983647155] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[18984529344] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18986212971] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18987149973] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18987844293] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18989500365] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[18990623850] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18993999981] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f2288
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18996181281] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370710992 RFLAGS_BEFORE=130 CR3_BEFORE=73998336 fs_base=0 gs_base=18446744071564586640
[19000243449] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[19002852990] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[19004184342] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[19010114508] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19011949308] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f2288
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19013579079] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370776528 RFLAGS_BEFORE=130 CR3_BEFORE=74145792 fs_base=0 gs_base=18446744071564586576
[19022020941] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[19051682034] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[19055984211] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[19057314276] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[19069349310] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[19071248856] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19072579251] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[19098095874] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[19116408861] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19118129580] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19124348628] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[19131752475] [INFO] [bloom] [CPU3] bloom: creating surface...
[19133083134] [INFO] [bloom] [CPU3] bloom: surface created!
[19133962089] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[19140169158] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[19145769621] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[19148914554] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[19152815352] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[19155185676] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1261
[19156359354] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[19169500053] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[19172318154] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[19173410619] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[19174728276] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [19183195515] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[19192664271] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[19201208136] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[19209011316] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[19217754237] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1261
T:07D0 [19221901743] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
T:0640 T:F0B0 [19245376986] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[19261953480] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[19263612027] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[19266074058] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[19275204861] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[19277370453] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[19279523967] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[19282728762] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[19288730868] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([239, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[19293914706] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=275
[19295148180] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[19296024198] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19297105971] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19298286975] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19299862230] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=275 subj_lo=0
T:1220 [19302837972] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[19304941821] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[19319972793] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1274)
[19321342821] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[19336617663] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=243
[19338119691] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[19339012704] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19339998315] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19341163083] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19342337619] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[19353987048] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[19356730239] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[19360009152] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=31, our_read=32)
[19362123231] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[19363123890] [INFO] [anther] [CPU1] anther: Connected to network stack
[19389962691] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[19390852503] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[19393305822] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[19395128214] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1276)
[19396207776] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[19413058203] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=281
[19414956132] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[19416424995] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19417731960] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19419351765] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19421060472] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=281 subj_lo=0
[19426173228] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[19429298823] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19436046036] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1277)
[19437420783] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[19463744883] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19471520277] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[19473748305] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[19478545713] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[19480801263] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[19484259861] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19497946281] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19504341186] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[19506751968] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[19532261958] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[19558954932] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[19562768247] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[19568575125] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[19571885718] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=1
[19574872185] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[19577955111] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=2
[19584995661] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=2)
[19618890291] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[19621413009] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[19626011361] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[19776486477] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[19778345466] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[19832531037] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[19878343089] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[19905896472] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[19934748636] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[19936947162] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[19940009397] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[20003300460] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[20048670939] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=663 watches=13 history=1024 journal=1024 symbols=313 drops=0
[20160448176] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[20318531178] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[20495002440] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[20749496625] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[20936047077] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[21036772317] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[21129006657] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[21315307794] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[21475966215] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[21482872884] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[21484354551] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[21485473581] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[21486729990] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[21488308479] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[21501181185] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[21502334700] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[21503580879] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[21505071291] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=338 pred=0 subj_lo=0
[21526963722] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[21707514972] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[21748085865] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=710 watches=15 history=1024 journal=1024 symbols=340 drops=0
[21950272938] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[22126021269] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22131955461] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[22134909489] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[22136997597] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=SynReceived
[22141953042] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=74
[22143054681] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=25
[22144687290] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[22160689716] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[22162486731] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[22168567377] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22190284380] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=SynReceived
[22211450811] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[22214574921] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=SynReceived
[22220133441] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[22224461028] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[22227792477] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=SynReceived
[22231887447] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[22240146852] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[22241599875] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[22243837638] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22244905056] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22249280625] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=Established
[22251040119] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:59410 on listener 2
[22252207923] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=149
[22253815947] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=25
[22255759548] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[22257101196] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=2
[22260183528] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[22312650954] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22398815769] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=3
[22403375874] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22405160283] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22406855790] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22408706661] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=307 pred=0 subj_lo=0
[22410871131] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 37 (user thread) assigned to CPU 1
[22413963099] [INFO] [anther] [CPU1] anther: Thread spawned TID=37 for conn_handle=3
[22426034433] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[22428295956] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[22445685042] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
T:5EF0 [22455808320] [INFO] [anther] [CPU1] anther: Worker thread TID=37 starting for conn_handle=3
[22508575320] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[22531514214] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=33, our_read=34)
[22533614763] [INFO] [anther] [CPU1] anther: Worker TID=37 connected to netd OK
[22565099568] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[22611894426] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[22613463345] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 85 bytes on conn_handle=3
[22617686751] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[22622927712] [INFO] [anther] [CPU1] anther: GET /health Http11
[22647926532] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[22652324442] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[22656795315] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[22660172535] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[22664317566] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22676193738] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22707791502] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22725823494] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[22730317731] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[22742725269] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[22744389822] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[22746749190] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22748368500] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22753066050] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[22755532536] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[22758873753] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22778601120] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22783559799] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[22787082351] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[22804742631] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[22807169781] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[22810403748] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22811789715] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22814360184] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[22818745092] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[22830178437] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[22835268654] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[22838577465] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[22842270825] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22843458462] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 0 bytes on conn_handle=3
[22846551024] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22848066417] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22849614480] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22851229170] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=341 pred=0 subj_lo=0
[22857661398] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[22860625590] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[22862948592] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=177
[22865436231] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 167 byte frame (171 encoded) to netd rx_port=25
[22868044023] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (171 bytes sent)
[22870187736] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22885640481] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[22899625848] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=Established
[22902083754] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:59420 on listener 2
[22905884364] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=2
[22916046780] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22946595903] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=4
[22955338824] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 38 (user thread) assigned to CPU 1
[22957208538] [INFO] [anther] [CPU1] anther: Thread spawned TID=38 for conn_handle=4
[22961779269] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[22998196551] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=fec1d29ef0678173)
[23056184448] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
T:5EF0 [23136103254] [INFO] [anther] [CPU1] anther: Worker thread TID=38 starting for conn_handle=4
[23162339739] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23167745865] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[23182759281] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12222000
[23184028725] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[23185231377] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[23206814697] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=35, our_read=36)
[23208821427] [INFO] [anther] [CPU1] anther: Worker TID=38 connected to netd OK
[23251175937] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[23261307135] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[23266480677] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[23268704646] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[23271160671] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[23276233431] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[23312682888] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[23314859040] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[23317012323] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[23336022930] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x1222e000
[23338312701] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[23355854280] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[23638432455] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=14aa2015f9906e3f)
[23874535806] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[24164712759] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[24319263408] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=793 watches=17 history=1024 journal=1024 symbols=355 drops=0
[24483830184] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[24768457659] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[25057453047] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[25345234728] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[25698657996] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[26023204845] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[26092257510] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=825 watches=17 history=1024 journal=1024 symbols=364 drops=0
[26336915583] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[26490317139] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=177
[26499415602] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 167 byte frame (171 encoded) to netd rx_port=25
[26501856744] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (171 bytes sent)
[26517855078] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[26532188232] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26535154800] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 167 bytes
[26581356351] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 113 bytes on conn_handle=4
[26585519004] [INFO] [anther] [CPU1] anther: POST /api/v1/query Http11
[26586445545] [INFO] [anther] [CPU1] anther: Request body size: 26 bytes
[26594695215] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[26595565458] [INFO] [phloem::executor] [CPU1] phloem: calling find for kind: dev.Cpu
[26597491701] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[26695238691] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26702387910] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[26710232109] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[26734225287] [INFO] [phloem::executor] [CPU1] phloem: find returned 4 candidates
[26791965684] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26853094785] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26854251072] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26855415774] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26856569817] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=342 pred=0 subj_lo=0
[26897690424] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26899009896] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26900439588] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26901876738] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=343 pred=0 subj_lo=0
[26919945195] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26939875611] [INFO] [phloem::executor] [CPU1] phloem: discovered 4 nodes
[26970506772] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 210 bytes - TCP ACK
[26974065228] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 210 bytes
[26992174671] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[27002976528] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[27005305767] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27170499939] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[27462090084] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[27709459899] [INFO] [netd::ip
```
</details>
