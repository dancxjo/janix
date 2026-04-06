# ❌ Scenario: Serve serial requests

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the anther server is ready | ❌ | 39115ms | - [📜](./01/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[13994316798] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[13999978113] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[14004126180] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[14006723148] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[14008459839] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[14009665923] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[14010799539] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[14011774161] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[14012799306] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[14013865008] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[14014879989] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[14015940081] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[14017135341] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[14018267637] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[14019431382] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[14020457880] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[14021668551] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[14022705147] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[14023742799] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[14024790516] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[14025834405] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[14026820742] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[14027812293] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[14028836118] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[14029903899] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[14030930265] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[14031951285] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[14033071998] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[14034081105] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[14035104600] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[14036292732] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[14037343452] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[14038452516] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[14039508681] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[14040515742] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[14041726512] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[14042944971] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[14044167786] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[14045422314] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[14046658263] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[14047897116] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[14049188868] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[14051068944] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[14052942981] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[14054262123] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[14055236085] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[14056104744] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[14056986438] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[14057940600] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[14058849057] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[14059718112] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[14060635479] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[14061507570] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[14062443846] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[14063507601] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[14064480177] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[14065521195] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[14066477403] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[14067426549] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[14068426680] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[14069379192] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[14070335598] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[14071257255] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[14072237256] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[14073202671] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[14074154259] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[14075090139] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[14076039582] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[14076955035] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[14077906953] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[14078852007] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[14079811119] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[14080728453] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[14081697795] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[14082613149] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[14083561569] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[14084496723] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[14085465834] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[14086385709] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[14087334723] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[14088270438] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[14089242321] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[14090161866] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[14091238095] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[14092177902] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[14093163579] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[14094090714] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[14095069230] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[14096016924] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[14096986827] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[14097952011] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[14098910034] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[14099830536] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[14100776943] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[14101715034] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[14102660550] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[14103579402] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[14104544850] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[14105469873] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[14106424893] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[14107353777] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[14108344173] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[14109278073] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[14110240452] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[14111199597] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[14112458382] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[14113425810] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[14114731191] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[14402254251] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[14421387288] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[14429908152] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[14432073150] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[14433525447] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[14441403042] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[14443863819] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[14445673770] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[14446660536] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[14447551800] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[14448402474] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[14449848567] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[14451384552] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[14452379271] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[14453133816] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[14454044121] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[14454852687] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[14456303004] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[14457557664] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[14458380915] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[14460113580] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[14461220763] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[14462397873] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[14464257852] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[14466035166] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[14466978042] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[14467562505] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[14468399913] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[14979427221] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[14980471209] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[14983798104] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[14985064941] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[14986136187] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[14988319302] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[15003347535] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[15005241240] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[15006422211] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[15008597076] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[15009482235] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[15013058412] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[15022131795] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[15024353190] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[15039475671] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[15040421088] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[15058722822] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[15059726880] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[15062477925] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[15064314639] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[15065924115] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[15068770695] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[15070089375] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[15105300111] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62589500 ticks/sec), init_cnt=625895 for 100Hz
[15107215629] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[15108247176] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[15109815402] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[15116279409] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[15148954062] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[15150352800] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[15152033193] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[15153875583] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[15155724870] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[15160497825] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[15162678795] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[15180632280] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[15182245914] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[15183706659] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[15184800345] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[15186131730] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[15187200171] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[15188366127] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[15217053621] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[15218885418] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[15220214493] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[15221326329] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[15222582672] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[15223597356] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[15224719158] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[15225822249] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[15236928927] [INFO] [kernel::root] [CPU0] Spawning Root service...
[15238941729] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[15242152332] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[15243657858] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[15250370652] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[15253125195] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[15254381076] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[15256566501] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[15258250854] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[15259787763] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[15282073356] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[15287627949] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[15289712394] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[15291142284] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[15330912234] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15366529827] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15370580247] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15377211993] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15380246838] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15385459353] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15390403116] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15394181847] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[15395521383] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[15397463532] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15410115864] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15414966435] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15420615573] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15425170068] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[15431795313] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[15433257081] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[15449944785] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[15451493046] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[15461211150] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[15462766308] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[15499090629] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[15500135937] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[15947692629] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[16761632514] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[16793174541] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=497 journal=422 symbols=51 drops=0
[16848098091] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[18818145423] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=961 journal=769 symbols=95 drops=0
[19882006485] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[19993520514] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[19995032211] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[20118133068] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[20185104324] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[20221676046] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[20222542197] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[20223245163] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[20228921229] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[20251426305] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[20276324937] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[20280163761] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[20350190388] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[20376585141] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[20377576263] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[20383813362] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[20413379712] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[20437986459] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[20442908706] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[20444699451] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[20525619114] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[20528467542] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[20630414607] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[20707293651] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[20721240672] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[20728098897] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[20730969831] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[20743576392] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[20827323297] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[20834244783] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[20836033317] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[20837059617] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[20838056448] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[20838874716] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[20839581180] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[20840319687] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[20840979819] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[20841970710] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[20843023674] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[20843937675] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[20844875931] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[20845647966] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[20846522862] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[20847602655] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[20848603215] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[20849496987] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[20850236847] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[20851234041] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[20852239881] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[20853013269] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[20853736992] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[20854772697] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[20855576247] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[20856317658] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[20857097514] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[20858033724] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[20859375306] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[20860096521] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[20860800939] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[20861536476] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[20862260628] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[20862965673] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[20863694049] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[20864383155] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[20865211884] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[20866051140] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[20867048367] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[20868206535] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[20869137399] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[20870260950] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[20871409647] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[20873381595] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[20875845408] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[20877181545] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20887764150] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[20905987344] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[20911499400] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[20923935813] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[20925510045] [CONTRACT] [kernel] [CPU0] Spawning init process...
[20928616104] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[20932425261] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[20933283426] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [20941686183] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013184 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[20946917079] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[20980761219] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[21002221119] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[21004403871] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[21005874714] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[21032959365] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[21043475838] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[21054540771] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[21056650329] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[21077250612] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[21083828172] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[21086154804] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[21095673885] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[21097845813] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[21099584055] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[21101732520] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[21108324798] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[21111970011] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[21127713321] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[21131271546] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[21133829706] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[21136212207] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[21148095078] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[21218369964] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[21226784931] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[21235162113] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[21240613581] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[21244046274] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[21248223315] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[21254182191] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[21260226471] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[21268203198] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[21275493426] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[21283654788] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[21292494696] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[21301477890] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[21308149929] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[21315617367] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[21325707348] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[21332322924] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[21338801979] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[21346074816] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[21352142361] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[21359292570] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[21366235704] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[21373933515] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[21380632515] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[21387455958] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[21394434204] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[21401857521] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[21409066107] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[21416373891] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[21423623595] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[21428817729] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[21432875046] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[21438529200] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[21444372114] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[21451229910] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[21458931252] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[21465625104] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[21472374066] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[21478855596] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[21485347653] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[21491858058] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[21495890130] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[21521486052] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[21713412105] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[21724317417] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[21725611149] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21728421957] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21735545568] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21737690436] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[21751439292] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[21760082916] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[21761621409] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21763848711] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078720 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[21771182400] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[21772227015] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[21773502993] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[21776465139] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21783682734] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[21786732858] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[21799341300] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[21803088153] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[21804852168] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[21807033171] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144256 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[21811984293] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[21815559084] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[21817981614] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[21820394607] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21825394074] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:46:04 = 1775436364 unix_secs
[21827452713] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775436364, mono_ns=10913477074, offset=1775436353086522926ns
[21829525245] [INFO] [rtc_cmos] [CPU1] System clock anchored
[21841430556] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[21887975142] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[21895824852] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[21896779872] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[21898792740] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21905033832] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[21907570047] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[21919987353] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[21926306820] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[21931784655] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21934468611] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[21946945911] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[21954701142] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[21957694803] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[21959063247] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[21961968600] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21973929549] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21977456853] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[21986067114] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[21990473373] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[21992009259] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21993785847] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[21995203065] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[22001037828] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[22003659744] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[22010435436] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[22011835626] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[22013279739] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[22014799818] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[22016797638] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[22035290310] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[22037603940] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[22604741511] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22613771466] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[22618338699] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[22620922434] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[22626965658] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[22629485109] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[22632607833] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[22635006966] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[22636830744] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[22639651749] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[22640891460] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[22641991911] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[22643006760] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[22643995044] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[22644913500] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[22646222544] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[22651618638] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[22652787069] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[22654936458] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22664380794] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22667464017] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[22676081340] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[22679689197] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[22680954087] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[22683751398] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352688 RFLAGS_BEFORE=134 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[22694691723] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[22717465749] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[22751657181] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[22771194963] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[22774997487] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[22784312991] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[22788780696] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[22790275497] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[22791695784] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[22793150787] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[22794483855] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[22795961199] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[22797455241] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[22798913115] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[22801261824] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[22803731049] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[22808738106] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[22812634713] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[22816082289] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[22818608538] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[22819982955] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22822856166] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22826122605] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[22828606086] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[22830826986] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22846854492] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[22850558544] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[22853417268] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[22854710670] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22857348096] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22862976840] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22864887573] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[22873220601] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[22876803147] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb01080c8
[22878284814] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[22880600391] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500176 RFLAGS_BEFORE=130 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[22885378197] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[22886731626] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22889345028] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22890616287] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[22892651298] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[22897826721] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[22899104052] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[22901659506] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[22902819885] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22905830343] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[22908904656] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[22911183801] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[22913520498] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[22915959693] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[22917461721] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[22920802476] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[22923609918] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[22924949091] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[22927098579] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[22928977830] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[22931264301] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102630
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[22933601658] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369433936 RFLAGS_BEFORE=134 CR3_BEFORE=60096512 fs_base=0 gs_base=18446744071564586608
[22942830735] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[23338902972] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb01080c8
[23340642732] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[23342985105] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369582960 RFLAGS_BEFORE=130 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[23348180922] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[23351382219] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[23352743073] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[23357207379] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[23358410130] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[23361796557] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[23362822890] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[23364149391] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23367206049] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23375556237] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[23378742090] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[23384273913] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[23389893153] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[23394865362] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[23398661253] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[23401230930] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[23402397381] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23404946532] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23436343623] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[23443238544] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[23477422056] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[23482071525] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a670
[23483521380] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23485821150] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369718768 RFLAGS_BEFORE=134 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[23491369242] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102630
[23493130518] [INFO] [netd] [CPU3] NETD: Starting network stack service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[23494719336] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[23495578524] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653232 RFLAGS_BEFORE=134 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[23504007120] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[23506430937] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[23515780860] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[23519224278] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[23524306377] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[23526190677] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[23529164967] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[23531186712] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[23533742991] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[23555592258] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[23558324460] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[23561667921] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[23562886710] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23565786519] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23645992722] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[23647249989] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[23648823495] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[23650594044] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[23670109716] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[23699026923] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[23707942170] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[23711989950] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0102630
[23713646286] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23715826398] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369784304 RFLAGS_BEFORE=134 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[23721134481] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[23721986838] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23724079170] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23725469559] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[23730423288] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[23737978836] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[23740650747] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[23754711387] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[23759126886] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[23767788495] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[23771169906] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[23773573593] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[23774667576] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23776653450] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23783891175] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[23787287073] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[23796326994] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[23800140408] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010c010
[23801572344] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23803217955] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915376 RFLAGS_BEFORE=134 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[23807271378] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[23808462018] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23810654208] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[23811817656] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23814662520] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[23821449399] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[23825910900] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[23834359494] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[23837990649] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010c010
[23839548975] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23841994275] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981616 RFLAGS_BEFORE=130 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[23847084525] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[23848686213] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23851320405] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23863560435] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[23868920394] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[23877566625] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[23881620939] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[23886157317] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[23887846686] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23889969708] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23919902193] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[23924831469] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[23933228583] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[23936129613] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010e118
[23937679755] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23939466573] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113904 RFLAGS_BEFORE=130 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[23944391361] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[23945197551] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23947271766] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23948181477] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[23966553237] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[23969885511] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[23971365561] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[23976333084] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[23978704497] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23980880847] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[23982306249] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[23984970702] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[23988077949] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[23989209024] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23990872191] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370198080 RFLAGS_BEFORE=130 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[23994614523] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[23995646763] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23997200634] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[23998042893] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23999112687] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[24010624539] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24012088881] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24013592328] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24015284766] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[24021079236] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[24022616871] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24024289278] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[24026104245] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[24043341432] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[24055127151] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[24065494299] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[24070252008] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[24072337014] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[24074062650] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24075944541] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[24078024696] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[24082740363] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[24084559653] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
[24085932387] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24086967300] [INFO] [fontd] [CPU3] FONTD: Service ready
[24087841239] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24089215392] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24091112991] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[24094460511] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102630
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24097055136] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849840 RFLAGS_BEFORE=134 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[24104679390] [INFO] [nectar] [CPU2] NECTAR: Started.
[24106565868] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[24112685058] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010c010
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24115279320] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047824 RFLAGS_BEFORE=134 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[24124221066] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[24128028276] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24130805358] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370267152 RFLAGS_BEFORE=130 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[24142612065] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[24145406736] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[24147919125] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[24157714548] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[24171642198] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[24173949129] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[24176546658] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[24178130691] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[24180883155] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[24182558763] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[24186491109] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x44b7000
[24188329770] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[24191648910] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[24194919342] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[24196945278] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[24213514017] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[24227357055] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[24243183096] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[24244912956] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[24247710729] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[24249025449] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24250763526] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24252288819] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[24253916841] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24256292577] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=235 subj_lo=0
[24259493511] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[24261891588] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[24264588480] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[24267194919] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[24269374965] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[24271462050] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[24272506896] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24273752151] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24275040933] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24276480690] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=236 subj_lo=0
[24279707628] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=19, read=20)
[24285700230] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24287318979] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24289388673] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24291444078] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=237 subj_lo=0
[24301084863] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=21, read=22)
[24307636386] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[24326796681] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[24339893424] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24341648100] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1249 backend=VirtIO-GPU
[24342984435] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24344832765] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[24346053534] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24347888598] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24349922817] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[24352046136] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24368171883] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24369721464] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24371506368] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24372976716] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[24416070624] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[24417117780] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[24442799502] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[24444606747] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[24453191202] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=23, resp=26
[24456055206] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[24558605874] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[24573331365] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[24581324295] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[24589736721] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[24591927591] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[24593458725] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[24598739649] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[24602287875] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[24603471387] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24605847750] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24607626582] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db478
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e1
[24609992781] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370500928 RFLAGS_BEFORE=130 CR3_BEFORE=72167424 fs_base=0 gs_base=18446744071564586640
[24615455898] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[24617556909] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[24621481170] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[24627191787] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[24631157859] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[24632523564] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db478
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[24635335923] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[24636568638] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370566464 RFLAGS_BEFORE=130 CR3_BEFORE=73216000 fs_base=0 gs_base=18446744071564586576
[24645173751] [INFO] [echo] [CPU1] echo: starting up
[24649083987] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[24677441514] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[24687509022] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[24709680831] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[24713647068] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[24724861128] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[24727377048] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[24730858218] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[24732244746] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24734972856] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24744112800] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[24747579219] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[24759920262] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[24762404667] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[24764309823] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[24769155840] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[24770710866] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[24773102442] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[24776126265] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[24778240839] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[24779577009] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24782381646] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24786746193] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[24793692165] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[24795776280] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[24797041995] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[24801236823] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[24810352545] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[24814512525] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[24816274494] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[24818115036] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f25e8
[24819647292] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24821266668] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24822821925] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370709824 RFLAGS_BEFORE=130 CR3_BEFORE=73981952 fs_base=0 gs_base=18446744071564586640
[24827040084] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[24830617779] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[24832226265] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[24836919360] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[24838769340] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[24841028322] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[24842304234] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[24845107485] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[24847667790] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[24854205090] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f25e8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[24856673358] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370775360 RFLAGS_BEFORE=130 CR3_BEFORE=74129408 fs_base=0 gs_base=18446744071564586576
[24865996221] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[24869263155] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[24895365066] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[24897369849] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=19, RX port=22
[24914111838] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[24937067991] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[24938700402] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[24972463758] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[24973994925] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[24977614101] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[24992426052] [INFO] [netd] [CPU3] NETD: Driver TX port=19, RX port=22, link_up=true, mtu=1500
[24996303354] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[24997718163] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[24999910386] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[25002407166] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[25022056257] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[25045144641] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([237, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[25053131235] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[25085193045] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[25087352334] [INFO] [bloom] [CPU3] bloom: creating surface...
[25088414076] [INFO] [bloom] [CPU3] bloom: surface created!
[25089292371] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[25099339782] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[25101978429] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[25103429538] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[25110055509] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[25112690163] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[25130376150] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[25133362287] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[25136054262] [INFO] [anther] [CPU1] anther: Connected to network stack
[25143383595] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[25144616376] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[25148476254] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[25152586866] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1264
[25153928580] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[25169250117] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[25172995419] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[25174752933] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[25176750753] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [25181370852] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[25191988206] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[25194003483] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[25196070273] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
T:07D0 [25220716488] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[25231887483] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[25237282719] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[25242254598] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[25253526276] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[25266226194] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1264
T:0640 [25274768970] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[25277981817] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[25280563077] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[25284924720] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[25289406615] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[25292501025] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[25296624078] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[25301682780] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[25304628624] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
T:F0B0 T:1220 [25319507169] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[25322346093] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[25343210607] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[25348605414] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[25363008231] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[25366038126] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[25380407349] [INFO] [netd] [CPU3] NETD: DHCP configured �� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[25385247822] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[25391297943] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[25395287313] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[25399768284] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[25403048352] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[25409544831] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[25414706526] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=282
[25416392661] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[25417519677] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[25418899077] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25420163670] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25421822448] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=282 subj_lo=0
[25436306577] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[25439475336] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[25448190570] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1273)
[25449733254] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[25451183142] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[25468171839] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=244
[25470253677] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[25471906581] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[25473671355] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25474815465] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25476112827] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[25490721234] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1276)
[25492572006] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[25519688964] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=284
[25522411035] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[25524062256] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[25525686417] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25527509436] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25529536428] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=284 subj_lo=0
[25546292772] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1282)
[25548885483] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[25597875171] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[25603843221] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[25608418110] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[25614599769] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[25625322327] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[25634731452] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[25645421670] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=74
[25649400810] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=21
[25653208515] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[25666536555] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[25668446298] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[25675980165] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[25681553106] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[25695429375] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[25708953237] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[25711099854] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[25716586335] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[25727957739] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=149
[25729485144] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[25730754852] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=21
[25733870943] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[25737133059] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[25898506260] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[25903290699] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[25906702041] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:41266 on listener 1
[25909549512] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[26045574951] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[26178335667] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26205658512] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[26253372519] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[26326638426] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=664 watches=13 history=1024 journal=1024 symbols=325 drops=0
[26332884270] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[26566012572] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[26812326123] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[27082150656] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[27412490886] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[27556613337] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[27635119974] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[27918467313] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[28186095135] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[28534440429] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[28560622761] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[28565264112] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[28567668624] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[28569273480] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[28571041884] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[28573205991] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[28597082580] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[28598899923] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[28600849299] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[28603215597] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=349 pred=0 subj_lo=0
[28780386096] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=724 watches=15 history=1024 journal=1024 symbols=351 drops=0
[28895853063] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[28898383899] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[28901380728] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28950346821] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[29088455253] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[29090813202] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[29149889703] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29251596825] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29280476115] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[29281948806] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[29283383019] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29284845249] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=353 pred=0 subj_lo=0
[29317765059] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[29319568014] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[29321242071] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29323489173] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=354 pred=0 subj_lo=0
[29363288328] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29443340586] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[29503668051] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[29508421404] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=3
[29520533889] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 37 (user thread) assigned to CPU 1
[29523664830] [INFO] [anther] [CPU1] anther: Thread spawned TID=37 for conn_handle=3
T:5EF0 [29565490581] [INFO] [anther] [CPU1] anther: Worker thread TID=37 starting for conn_handle=3
[29570220702] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[29574499812] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[29587668990] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[29592507384] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29647494261] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[29649583788] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[29651768982] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29653484751] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=356 pred=0 subj_lo=0
[29685751953] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[29759217675] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=33, our_read=34)
[29761654659] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[29763731778] [INFO] [anther] [CPU1] anther: Worker TID=37 connected to netd OK
[29765088276] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[29766505758] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[29768399925] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=357 pred=0 subj_lo=0
[29775365433] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29783871777] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[29875750476] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 85 bytes on conn_handle=3
[29888493426] [INFO] [anther] [CPU1] anther: GET /health Http11
[29911575705] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[29915101920] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[29920320507] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[29922269289] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[29925757950] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[29939716983] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29945413938] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[29962857045] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[30033935250] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[30071764602] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[30122849988] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[30143483073] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[30145165215] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[30147819009] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[30150907479] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30156717294] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30158316804] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[30161385111] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[30163764279] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30169511361] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30187224837] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 0 bytes on conn_handle=3
[30205138029] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x121f6000
[30208093212] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[30212453601] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[30323566581] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[30339952302] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[30347748024] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[30351214443] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[30354498537] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[30362404908] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[30373492248] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[30406865808] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[30410481519] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[30414202269] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[30442421130] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[30445188213] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[30449117391] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[30451151049] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=1 (16384b)
[30457936938] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x12203000
[30461816253] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[30475817427] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[30628669599] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30901074600] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=9d85f11f1afc2373)
[31312427289] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[31849086060] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=14aa2015f9906e3f)
[32221916991] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[32223783273] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[32226547089] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[32271743592] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[32550281742] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=798 watches=19 history=1024 journal=1024 symbols=384 drops=0
[32571362769] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[32607519285] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[32610775263] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[32617184391] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[32619020841] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[32626639848] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[32629344429] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[32643565185] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[32653391859] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=149
[32657139900] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=21
[32661059112] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[32672004717] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[32679853173] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[32683763904] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:41278 on listener 1
[32725724130] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[32764472664] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[32828848932] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[32923231506] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[33105613464] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[33232083225] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[33329842161] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[33346871580] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[33488630703] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[33598729956] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[33726040095] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[33737759253] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=4
[33752049243] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 38 (user thread) assigned to CPU 1
[33755060064] [INFO] [anther] [CPU1] anther: Thread spawned TID=38 for conn_handle=4
[33761368905] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[33886568232] [INFO] [flytrap] [CPU2] FLYTRAP: PubliT:5EF0 shed new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[33890935056] [INFO] [anther] [CPU1] anther: Worker thread TID=38 starting for conn_handle=4
[33892428075] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[33960162786] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=1
[34022343267] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=35, our_read=36)
[34026556047] [INFO] [anther] [CPU1] anther: Worker TID=38 connected to netd OK
[34219454346] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[34389779688] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[34454184369] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[34523224659] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[34611032181] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[34615574268] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[34711508997] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[34875955785] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[35051044875] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[35098855770] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[35199045156] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[35327271672] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[35520359853] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[35525259066] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[35530124058] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[35536067292] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[35847893796] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[35865274104] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[36464045574] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=149
[36467909610] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=21
[36472028340] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[36479892042] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[36499497672] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[36501424938] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[36631585485] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[36691768212] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[36763474803] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 85 bytes on conn_handle=4
[36766010655] [INFO] [anther] [CPU1] anther: GET /health Http11
[36784575663] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[36791595027] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[36805457370] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[36808710180] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[36812615895] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[36814013643] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[36914901639] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[36920279319] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[36940667742] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[36944087763] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[36948578007] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[36961344420] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[36966043521] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=70
[36969094800] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[36973150071] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[36982560945] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[37061244165] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 0 bytes on conn_handle=4
[37400246763] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[37501152381] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[38118606009] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[38193960156] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[38817805389] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[38830973511] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[38841922152] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=70
[38844398934] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[38848644945] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[38861340441] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[38865973971] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[38870346603] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[38879545617] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=70
[38882792586] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[38885658570] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[38895181776] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[38903162298] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=149
[38905847178] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=21
[38909510475] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[38922133536] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[38927378028] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[38930712678] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:41292 on listener 1
[39440163378] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[39468610368] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2429 ops=1 watches=19
[39519147096] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[40195630167] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[40433697414] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=962 watches=19 history=1024 journal=1024 symbols=387 drops=0
[41573241732] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[42139432665] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=70
[42142814934] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[42147006660] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[42222371334] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[43013488155] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=993 watches=19 history=1024 journal=1024 symbols=387 drops=0
[43094936742] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=149
[43097510379] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=21
[43101100548] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[43230535920] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[43854908163] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[44192938449] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[44214780456] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[44220308220] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=5
[44231310849] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 39 (user thread) assigned to CPU 1
[44234244846] [INFO] [anther] [CPU1] anther: Thread spawned TID=39 for conn_handle=5
T:5EF0 [44295774237] [INFO] [anther] [CPU1] anther: Worker thread TID=39 starting for conn_handle=5
[44324542317] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[44327466447] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[44338342950] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[44461954647] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[44470874712] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=37, our_read=38)
[44474046672] [INFO] [anther] [CPU1] anther: Worker TID=39 connected to netd OK
[44958706947] [INFO] [anther] [CPU1] anther: Worker TID=39 got first 85 bytes on conn_handle=5
[44960511684] [INFO] [anther] [CPU1] anther: GET /health Http11
[45093282564] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[45144245322] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[45147748767] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[45166705188] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=70
[45170507745] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[45176375112] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[45474929016] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=70
[45477812490] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[45480992271] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[45500825898] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[45517517661] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[45523094859] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[45524418324] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[45537852690] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=25 len=70
[45551318406] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[45555802314] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[45565702215] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=26 len=70
[45568818141] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[45573172161] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[45711293133] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[45752786442] [INFO] [anther] [CPU1] anther: Worker TID=39 got first 0 bytes on conn_handle=5
[46353790395] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=1039 watches=19 history=1024 journal=1024 symbols=406 drops=0
[49542158358] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[49716045819] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[50314704924] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=1130 watches=19 history=1024 journal=1024 symbols=470 drops=0
[50348500554] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[50509278765] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[50881327431] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[50883542424] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[50919464376] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[51684338376] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[51747915516] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[51749535453] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[51751362069] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[51753489414] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[51767345718] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[51768850848] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[51770627601] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[51772465965] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=234 pred=0 subj_lo=0
[51784137966] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[51785560398] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[51787378434] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[51789248709] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=473 pred=0 subj_lo=0
[51800359083] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[51802246683] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[51804248199] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[51806401020] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=472 pred=0 subj_lo=0
[51814838097] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[51816692565] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[51818818524] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[51820988505] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[51830236425] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[52115029230] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=27 len=70
[52117654644] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[52130712315] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[53102452128] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1158 watches=24 history=1024 journal=1024 symbols=474 drops=0
[53166302244] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[53171696886] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[53176906992] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[53181828381] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[53190660864] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=28 len=70
[53193256380] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[53196783288] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[53205956397] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=29 len=149
[53208281049] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=21
[53211093507] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[53233006662] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[53241181686] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[54356028078] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[54359295375] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:41312 on listener 1
[54364459974] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[55406576148] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=30 len=70
[55410933303] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[55414822683] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[55912352760] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1188 watches=24 history=1024 journal=1024 symbols=474 drops=0
[58552926234] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1218 watches=24 history=1024 journal=1024 symbols=474 drops=0
[58734745113] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=31 len=70
[58739097912] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[58742719794] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[60792635178] [INFO] [fontd] [CPU3] FONTD: Auto-importing font asset ThingId([117, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[61602188571] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=1257 watches=24 history=1024 journal=1024 symbols=475 drops=0
[62836934853] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=149
[62840123544] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=21
[62843980386] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[64040039085] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=1279 watches=24 history=1024 journal=1024 symbols=476 drops=0
[64174920348] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=3453 ops=1 watches=24
[64485954192] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=70
[64496663121] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[64510592817] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[65044952775] [INFO] [fontd] [CPU3] FONTD: Auto-imported font 'Noto Sans' style 'Regular' from asset ThingId([117, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[65049515850] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (1 fonts)
[65363796762] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[65367747225] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[65370939744] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[66797386623] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=16000 nodes=1314 watches=24 history=1024 journal=1024 symbols=476 drops=0
[69497139063] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=17000 nodes=1334 watches=24 history=1024 journal=1024 symbols=476 drops=0
[70534601148] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[71995408287] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=70
[71998457883] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[72002610108] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[72047389326] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=18000 nodes=1358 watches=24 history=1024 journal=1024 symbols=476 drops=0
[74726522442] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=19000 nodes=1378 watches=24 history=1024 journal=1024 symbols=476 drops=0
[77683573473] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[77686812621] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[77691865251] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[77971622223] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=20000 nodes=1413 watches=24 history=1024 journal=1024 symbols=476 drops=0
[78646757739] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=70
[78650130405] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[78656469276] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[78663243582] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[81122185452] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=21000 nodes=1439 watches=24 history=1024 journal=1024 symbols=476 drops=0
[82630127316] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=149
[82634443287] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=21
[82640589174] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[87094371606] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[87098151525] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[87102753573] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[90224849979] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[90235746513] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[90243097956] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[93862727739] [INFO] [nectar] [CPU2] NECTAR: timeout waiting for Socket API port space (need 20 bytes)
[93867040542] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[93870615498] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[93875370270] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[93887781141] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[93897415392] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[93901067931] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[93982784346] [INFO] [anther::net_client] [CPU1] anther: timeout waiting for Socket API port space (need 22 bytes)
[95257611405] [INFO] [anther::net_client] [CPU1] anther: timeout waiting for Socket API port space (need 20 bytes)
[99445906989] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[99484283415] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[99497148630] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[99502009398] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[99506219274] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[99520971891] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[99524892423] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[99539816640] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[99546946389] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=6
[99558850842] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 40 (user thread) assigned to CPU 1
[99561515295] [INFO] [anther] [CPU1] anther: Thread spawned TID=40 for conn_handle=6
[99563195226] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[99566663328] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[99570054738] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[99573277221] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[99574565640] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=22000 nodes=1469 watches=24 history=1024 journal=1024 symbols=476 drops=0
[99576762747] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[99581822142] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[99586368321] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[99599312934] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=149
[99603088728] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=21
[99607004970] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[99613884282] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[99620244702] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[99623722506] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[99628137015] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[99645821253] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
T:5EF0 [99657752271] [INFO] [anther] [CPU1] anther: Worker thread TID=40 starting for conn_handle=6
[99665239674] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[99669844989] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:53730 on listener 1
[99802497201] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=39, our_read=40)
[99805259466] [INFO] [anther] [CPU1] anther: Worker TID=40 connected to netd OK
[103218225726] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=23000 nodes=1505 watches=24 history=1024 journal=1024 symbols=476 drops=0
[104074908894] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[104081898063] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[104092136115] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[104109985617] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=70
[104114459262] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[104118815262] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[105163633509] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=70
[105167157711] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[105171035805] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[106973520588] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=24000 nodes=1531 watches=24 history=1024 journal=1024 symbols=476 drops=0
[109841117001] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=25000 nodes=1557 watches=24 history=1024 journal=1024 symbols=476 drops=0
[110676355548] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=70
[110681696664] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[110687794965] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[111799542360] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=70
[111807957096] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[111812770179] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[121898656341] [INFO] [nectar] [CPU2] NECTAR: timeout waiting for Socket API port space (need 20 bytes)
[121927928496] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=70
[121932508038] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[121936809489] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[121950443604] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=70
[121960174710] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[121969208130] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[122006799882] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=70
[122012821194] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=21
[122023404030] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[122224671228] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=149
[122227720527] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=21
[122231172855] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[122542396284] [INFO] [anther::net_client] [CPU1] anther: timeout waiting for Socket API port space (need 20 bytes)
[123869152341] [
```
</details>
