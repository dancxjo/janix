# ❌ Scenario: Dynamic Asset Loading from Graph

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 3746ms | - - - |
| 2 | Then I should see a message in the serial output that says "[asset_bank] promoting wallpaper" within 30s | ✅ | 3553ms | - [📜](./02/serial.log) - |
| 3 | And I should see a message in the serial output that says "[asset_bank] promoting cursor" within 30s | ✅ | 1ms | - - - |
| 4 | And I should see a message in the serial output that says "[asset_bank] promoting font" within 30s | ✅ | 11416ms | - [📜](./04/serial.log) - |
| 5 | And I should see the wallpaper on the screen within 250 seconds | ❌ | 1012ms | - [📜](./05/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11887458660] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11892816540] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11896390242] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11898342720] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11899548210] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11900175837] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11901204084] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11902155441] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11903119932] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11904104157] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11905058781] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11906042379] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11907159693] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11908199886] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11909330235] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11910301194] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11911284132] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11912286408] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11913262020] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11914211859] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11915132625] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11915946405] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11916550008] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11917133514] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11917759623] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11918358342] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11918967324] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11919636069] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11920219509] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11920825257] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11921424669] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11922033684] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11922690120] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11923332927] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11923918611] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11924605836] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11925300057] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11926021008] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11926708068] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11927423277] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11928121458] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11928839043] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11930131950] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11931590979] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11932376412] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11932921341] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11933429079] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11933946123] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11934487917] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11935012155] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11935550781] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11936067759] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11936574870] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11937107325] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11937647733] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11938206522] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11938776993] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11939337993] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11939879325] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11940438840] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11941288260] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11942041485] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11942586810] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11943149394] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11943695709] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11944258128] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11944801605] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11945377356] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11945920998] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11946484374] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11947031415] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11947592811] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11948137344] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11948709036] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11949253305] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11949823611] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11950367154] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11950929540] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11951471367] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11952045171] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11952587097] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11953155225] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11953698636] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11954257986] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11954801859] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11955378534] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11955921483] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11956480932] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11957024871] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11957586729] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11958128886] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11958707310] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11959248708] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11959810632] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11960352591] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11960912733] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11961453504] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11962024668] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11962568376] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11963129541] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11963672622] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11964231906] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11964774030] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11965349451] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11965891575] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11966452608] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11966997768] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11967758154] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12202036671] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[12213117378] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12217967256] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12219333687] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12220221090] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12224476737] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12226163961] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12227272992] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12227951241] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12228630249] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12229287906] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12230352552] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12231321465] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12232024629] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12232693935] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12233552628] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12234236124] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12235443957] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12236463030] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12237152598] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12238705545] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12239625255] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12240586611] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12242182821] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12243741906] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12244536579] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12245096325] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12245873739] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12615660948] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12616693155] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12620027739] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12620931609] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12621693942] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12623214153] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12635458242] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12636726597] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12637522227] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12639184305] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12639723987] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12642505590] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12649679493] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12651348237] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12664262391] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12664817781] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12680670783] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12681251781] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12683329659] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12684511455] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12685593393] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12687861912] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12688716018] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12723500889] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62302000 ticks/sec), init_cnt=623020 for 100Hz
[12724792476] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12725660376] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12726816135] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12732210051] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12762377562] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12763390002] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12765588792] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12766797648] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12767812959] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12770470020] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12772358280] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12792329880] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12794721258] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12795507978] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12796196424] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12797364723] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12798367032] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12799069635] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12824145576] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12826108449] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12827255892] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12828628428] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12829496757] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12830390034] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12831153423] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12832022082] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12838162722] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12838974291] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12840684417] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12841581753] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12847092819] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12849456873] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12850693053] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12852591477] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12854074068] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12855677142] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12867830778] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12871286769] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12873058869] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12874548225] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12895138146] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12912853503] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12915144132] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12918254448] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12919858611] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12922039185] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12924489303] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12926362779] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12927165900] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12928172367] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12935499687] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12937920435] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12940214958] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12942327618] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12944784501] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12945572046] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12946594287] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12959937375] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12960751584] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12967195923] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12968001882] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12996832365] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12997599351] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13345384107] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13811064135] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13856770092] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=500 journal=424 symbols=52 drops=0
[13886533551] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[15157645140] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=450 watches=0 history=966 journal=774 symbols=98 drops=0
[15832404984] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15927405483] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15928513623] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[16031215431] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[16087355229] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[16113219342] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[16117331208] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[16117980153] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[16121788584] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[16138980924] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[16162909521] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[16165093824] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[16215090375] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[16235892981] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[16236750618] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[16240423254] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[16265968983] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[16284074598] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[16287542601] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[16290622524] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[16355242695] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[16356980904] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[16441375005] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[16503049530] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[16514825217] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[16518559035] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[16520706279] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[16573857729] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=183 drops=0
[16588524645] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[16593797748] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[16594767420] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[16595589846] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[16596495465] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[16597177476] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[16597913706] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[16598564664] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[16599162393] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[16599789789] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[16600732896] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[16601666730] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[16602360390] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[16603024680] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[16603707912] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[16604425497] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[16605069459] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[16605719196] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[16606357350] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[16607003820] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[16607977056] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[16608585609] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[16609214193] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[16609834461] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[16610456247] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[16611124959] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[16611754005] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[16612376154] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[16613067075] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[16613683284] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[16614328731] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[16614964509] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[16615606722] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[16616249826] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[16616881512] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[16617494388] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[16618218903] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[16618952196] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[16619704530] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[16620428748] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[16621185867] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[16621919358] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[16622674101] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[16624124220] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[16625860284] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[16626646542] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16634775366] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[16648820958] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[16652959752] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[16662043695] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[16662737355] [CONTRACT] [kernel] [CPU0] Spawning init process...
[16664808138] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[16667691183] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[16668905814] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [16674338868] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[16675059918] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[16696953933] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[16699232583] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[16701001482] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[16702157934] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[16709664081] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[16714134393] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[16717328826] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[16718421357] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[16722148113] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[16725208830] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[16726295124] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[16729721613] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[16730796786] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[16732050291] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[16733286669] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16736216013] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16737295080] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16738484433] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16739873139] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16741262439] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16742519310] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16746187689] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[16787879724] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[16795109562] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[16799709366] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[16804159548] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[16807153110] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[16810967184] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[16815369054] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[16819927113] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[16824552129] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[16829231958] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[16834554594] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[16839552411] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[16844425686] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[16848749775] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[16853803362] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[16858594467] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[16863811536] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[16868311746] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[16872597093] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[16876887522] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[16881128220] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[16885664532] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[16890203682] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[16894943835] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[16899824040] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[16904678571] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[16909900887] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[16915383837] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[16920740661] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[16926130848] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[16930328382] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[16934069856] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[16939811328] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[16946219268] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[16952215401] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[16958004591] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[16964391279] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[16970000223] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[16975001604] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[16980151188] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[16985477289] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[16988521407] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[17007303258] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[17138665236] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[17145194649] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[17145961305] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17147651862] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17152175634] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17153416962] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17161838034] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[17168938743] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[17170294779] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17172454728] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[17179211016] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[17180364333] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[17181704760] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17184511509] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17190374949] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[17192053956] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17199305640] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[17202374244] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[17203746879] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[17205321573] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[17209827756] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[17215109472] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[17217721389] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[17220851670] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17227234728] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:04:03 = 1775437443 unix_secs
[17230456089] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775437443, mono_ns=8614791421, offset=1775437434385208579ns
[17234659629] [INFO] [rtc_cmos] [CPU1] System clock anchored
[17245456173] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[17278166235] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[17289132201] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[17289992346] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17291931657] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17298759093] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[17302027314] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[17310169998] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[17313183822] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[17316548172] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17318116761] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[17322969510] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[17326798533] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[17328723456] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[17329487538] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17331157041] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17338450536] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17341034832] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17348016609] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[17350654530] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[17351426763] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17352938064] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[17357824473] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[17359183941] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[17362743387] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[17366973162] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[17368545018] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[17369856207] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[17370897324] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[17372223132] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[17382805209] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[17384386701] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[17880687594] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17888776257] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[17893233534] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[17895884853] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[17902088127] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[17904838083] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[17907731424] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[17909611632] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[17910764652] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[17913217080] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[17914911036] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[17916411645] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[17917865757] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[17919212751] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[17920575354] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[17922205026] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[17927260560] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[17928890661] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17931486375] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17938840359] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17941806201] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17949563742] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[17953606704] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fbff0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[17955234528] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
[17956854762] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352816 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[17964892044] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[17973125181] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[18001935105] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[18013701024] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[18015710097] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[18021283434] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[18030068298] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[18031586991] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[18042346938] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[18043798410] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[18045047625] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[18047370990] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[18048657066] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[18049938192] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[18051382338] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[18053102100] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[18054391971] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[18064613325] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[18067529139] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[18070306023] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[18072230022] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[18072932229] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18074406108] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18077950638] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18079738446] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18086899776] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[18090157866] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[18092249076] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[18093007647] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18094575147] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18099323121] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18100764000] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18107632257] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[18110096070] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
[18111247704] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[18113569485] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500512 RFLAGS_BEFORE=134 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[18116652840] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[18117866019] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18120100119] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[18120787608] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18121650393] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[18125115558] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[18126272109] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18127190433] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[18128852181] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18129761661] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[18132440568] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[18133986552] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[18135384201] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[18137182206] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[18138374925] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[18139905729] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[18140752410] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[18142500288] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[18143358816] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[18148165200] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[18149885820] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[18151469259] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434432 RFLAGS_BEFORE=130 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[18156383223] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[18500299059] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
[18501552762] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[18503381985] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583296 RFLAGS_BEFORE=134 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[18506295258] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[18508861470] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[18509823651] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[18512150514] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[18512926443] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18515093817] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[18515753850] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[18516493446] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18518028243] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18525247059] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[18527679885] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[18528513399] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[18535705881] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[18538733829] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[18541716897] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[18543422667] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[18544239417] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18545818203] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18565570848] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18569762112] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[18590084964] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[18592596759] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[18594118851] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18595923423] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716944 RFLAGS_BEFORE=130 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[18600557382] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[18602644368] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650480 RFLAGS_BEFORE=134 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[18605868138] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[18607968885] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[18609015612] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[18611499159] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[18614338479] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[18615852684] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[18618448497] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[18620009661] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[18621130539] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[18622053945] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[18623470074] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[18634178046] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[18635972058] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[18638251632] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[18639050892] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18640688154] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18706884900] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[18733812273] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[18742138965] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[18746066625] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[18747612774] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18749421966] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369783088 RFLAGS_BEFORE=134 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[18754327284] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[18755035035] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18756632268] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18757749318] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[18760513662] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[18764538672] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18765978528] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18781647885] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18784827138] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[18792607482] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[18795626124] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[18797603682] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[18798445248] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18799973346] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18805636938] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18808131606] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18815987982] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[18818694642] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010aff8
[18819676458] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18820982598] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915472 RFLAGS_BEFORE=134 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[18823906299] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18824629659] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18825769677] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[18826460862] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18827205870] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[18834433926] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18838248594] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18845053029] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[18847588716] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010aff8
[18848889939] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18850488129] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981872 RFLAGS_BEFORE=130 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[18854381304] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[18855286857] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18857597055] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18866708619] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18870631197] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[18873620568] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[18874413228] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[18875499291] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18876445533] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[18878643597] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[18881776254] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[18884171229] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[18884886042] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18886325568] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18911423124] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18916440411] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[18923418822] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[18926695293] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010c048
[18927891015] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18929255631] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370114064 RFLAGS_BEFORE=130 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[18932226060] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[18932941830] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18934075314] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[18935526291] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18951656790] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[18953808192] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[18955930026] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18957288009] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18958099578] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18959446440] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18961025952] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[18965999184] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[18969072837] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[18970923675] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[18971751018] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18973200312] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18974143749] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18975649143] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370198208 RFLAGS_BEFORE=134 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[18980742858] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[18982496379] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[18986882376] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18987974478] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18988978140] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18990031401] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=229 pred=0 subj_lo=0
[18991419975] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18992489934] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18993719646] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18995419476] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[19009066395] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[19019184855] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[19027363476] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[19030131780] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[19032713931] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19033860714] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19034918067] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19036015119] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19037346735] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19038448308] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19039831404] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[19041137973] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[19042631091] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19044709662] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849392 RFLAGS_BEFORE=134 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[19053091068] [INFO] [nectar] [CPU2] NECTAR: Started.
[19057418226] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010aff8
[19058857719] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19061070435] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370048080 RFLAGS_BEFORE=134 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[19065286350] [INFO] [fontd] [CPU3] FONTD: Service ready
[19067219028] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[19069495335] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010df80
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19071288984] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370267120 RFLAGS_BEFORE=134 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[19076234463] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[19077812028] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[19079848821] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[19082663556] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19084137336] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19091106342] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[19099503984] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19100471643] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19101468507] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19102706337] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=234 subj_lo=0
[19105129395] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[19106770815] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[19108548822] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[19109996730] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[19111943367] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[19112708175] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19113666759] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[19114426419] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19115485521] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19116563631] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=240 subj_lo=0
[19117797600] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x44b6000
[19119138225] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[19121303784] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[19122585768] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19123580949] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19124518974] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[19125309390] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19126444854] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[19127085912] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[19132516128] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19133457783] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19134439830] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19135717722] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[19137721515] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[19142277495] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19143228621] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19144223637] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19145294223] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[19146607788] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[19155869799] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[19158330411] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[19160648364] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[19162585662] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[19164128841] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[19165474482] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[19166787783] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[19167928857] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[19169221731] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[19174700523] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=19, read=20)
[19180501065] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=21, read=22)
[19187694504] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[19193714331] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19202325384] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[19216081863] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19221604215] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=23, resp=26
[19223147493] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[19241192652] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[19252720971] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1257 backend=VirtIO-GPU
[19253760405] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19255054203] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[19255777662] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19257239991] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19259454324] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19261146366] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19308193113] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19358496663] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19368939480] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19370778372] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19395383601] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[19410633528] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[19418019555] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[19420788156] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db6a0
[19422411261] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e9
[19424351100] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370509488 RFLAGS_BEFORE=130 CR3_BEFORE=72433664 fs_base=0 gs_base=18446744071564586640
[19427823228] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[19428551802] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19430211372] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19431104088] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[19435324095] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[19436851500] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19444874757] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[19447699359] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[19449953919] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[19458225666] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[19460114718] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db6a0
[19461054096] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=19, RX port=22
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[19462628427] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370575024 RFLAGS_BEFORE=130 CR3_BEFORE=73482240 fs_base=0 gs_base=18446744071564586576
[19467249516] [INFO] [echo] [CPU1] echo: starting up
[19469246082] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[19474450347] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[19476387942] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[19477364346] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19502100288] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[19504577697] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[19509993657] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19511359560] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19526546853] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[19529139432] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[19533449595] [INFO] [netd] [CPU3] NETD: Driver TX port=19, RX port=22, link_up=true, mtu=1500
[19536735339] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[19542566208] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[19546753479] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[19563054786] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[19565630502] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[19571590467] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[19572936009] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[19574515620] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[19575320655] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19577243037] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19580665830] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[19582674969] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19584496272] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[19591618002] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[19594185567] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[19595921004] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[19597854309] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[19599278853] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([237, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[19600863645] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19601807577] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19603923537] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19611072723] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19614896994] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19617605931] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[19622895996] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[19624249854] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[19627200450] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[19628680203] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f43f0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19630904766] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[19631834805] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370742960 RFLAGS_BEFORE=130 CR3_BEFORE=73986048 fs_base=0 gs_base=18446744071564586640
[19636351845] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[19637190276] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19638688047] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19641911718] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[19643125887] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[19646667018] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[19648215180] [INFO] [anther] [CPU1] anther: Connected to network stack
[19651708230] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[19653944739] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[19655221872] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[19674627555] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[19677676557] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[19684904844] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[19686357438] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[19692361887] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[19698096858] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f41b0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19699858431] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370824880 RFLAGS_BEFORE=130 CR3_BEFORE=74133504 fs_base=0 gs_base=18446744071564586576
[19704466419] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[19720662555] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[19721495904] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[19727575857] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[19734829224] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[19743319695] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[19763588064] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[19769688609] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[19772176908] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19777722195] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19783794690] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[19788245631] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[19793980800] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[19804796418] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[19806619107] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[19809338241] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19820685852] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[19823161974] [INFO] [bloom] [CPU3] bloom: creating surface...
[19824339084] [INFO] [bloom] [CPU3] bloom: surface created!
[19825232328] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[19830159789] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19833681615] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[19835392368] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[19840222776] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[19853174121] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1264
[19854901968] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[19867095567] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[19869953202] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[19871439159] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[19872842187] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [19880592501] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[19889174382] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[19898513481] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[19906701705] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[19916631570] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[19920264969] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[19925330238] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[19926476394] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 6)
[19928052936] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[19931003730] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[19933479324] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[19936723554] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
T:07D0 T:0640 T:F0B0 [19955273217] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[19960742967] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1264
[19968410682] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[19977103608] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[19979596527] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[19981605237] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[19984435482] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[19996239417] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[19998619905] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[20002799190] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[20004072792] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[20004829119] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
T:1220 [20009702295] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[20011599399] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[20026922520] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=280
[20028307728] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[20029346667] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20030433918] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20031762168] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20033046825] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=280 subj_lo=0
[20049886395] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1277)
[20050966353] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[20061933804] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=243
[20063872422] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[20065309869] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20066444508] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20067556377] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20068838922] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[20079484359] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1280)
[20080655232] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[20093345349] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=283
[20094822858] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[20096103984] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20097125301] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20098333629] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20099772033] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=283 subj_lo=0
[20112564186] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1283)
[20114182506] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[20230090155] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[20457634758] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[20461889019] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[20470631247] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[20493431244] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[20561521200] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=662 watches=13 history=1024 journal=1024 symbols=314 drops=0
[20619527643] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[20738335827] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[20878516395] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[21113539524] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[21284267268] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[21445882095] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[21518141073] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[21620556540] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[21786545088] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[21906388680] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[21919628280] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[21925398231] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[21926709882] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[21928027242] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[21929598141] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[21938564175] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[21939706470] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[21940978158] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[21942501273] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=338 pred=0 subj_lo=0
[21978085437] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[22016789058] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=708 watches=15 history=1024 journal=1024 symbols=339 drops=0
[22215933333] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[22377310725] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[22531939353] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[22633343238] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[22635270603] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[22669789329] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22732933938] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22769531103] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22770862158] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22772051940] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22773445530] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=340 pred=0 subj_lo=0
[22782677709] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=fec1d29ef0678173)
[22824079575] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22925011857] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23015262534] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23016492576] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23017630878] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23018942430] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=341 pred=0 subj_lo=0
[23034930303] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[23053676019] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23087005029] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[23093431581] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[23160844212] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23163775767] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23165035476] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23166133881] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23167526580] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=342 pred=0 subj_lo=0
[23184182076] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23185425186] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23186712615] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23188152933] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=343 pred=0 subj_lo=0
[23248041795] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[23279581479] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[23327542458] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12232000
[23329767879] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[23332319010] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[23409960453] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[23419748055] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[23423990271] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[23426164905] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[23428323963] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[23432606175] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[23441808621] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[23446679454] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=c1e9a7b90a789e5c)
[23453511741] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[23455122339] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[23456920641] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[23463378741] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[23465393391] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[23468113416] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[23469687516] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=1 (16384b)
[23479563855] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x122c3000
[23481011037] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[23492612220] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23698796616] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[23937590721] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[24169515513] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=794 watches=19 history=1024 journal=1024 symbols=363 drops=0
[24179449998] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[24419799195] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[24506356875] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[24576892890] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24613644231] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[24678364722] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24730557192] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[24767542932] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25028102286] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[25344962214] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[25635443889] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[25950485979] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=840 watches=19 history=1024 journal=1024 symbols=366 drops=0
[26041260696] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[26363683170] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[26645539470] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[26949003378] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[27314675454] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[27623182653] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=884 watches=19 history=1024 journal=1024 symbols=367 drops=0
[28197960813] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[28490236467] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2429 ops=1 watches=19
[28654212411] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[29126168973] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=906 watches=19 history=1024 journal=1024 symbols=367 drops=0
[29352008400] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[29771581884] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[30058907307] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[30187932258] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[30550416369] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[30932110440] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=948 watches=19 history=1024 journal=1024 symbols=379 drops=0
[32106227241] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[33068571063] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1046 watches=19 history=1024 journal=1024 symbols=442 drops=0
[33409455882] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[33497207865] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[33785809728] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[33787507347] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[33813643974] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[34286333334] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[34327056687] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34328182911] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34329274056] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34330816773] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[34339460892] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34340509269] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34341717069] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34343051292] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=235 pred=0 subj_lo=0
[34349164080] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34350207606] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34351342146] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34352575686] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=453 pred=0 subj_lo=0
[34358665506] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34359746322] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34360947258] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34362120672] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=452 pred=0 subj_lo=0
[34369395720] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34370414166] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34371518511] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34372651863] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[34381330731] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[34716997689] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1084 watches=24 history=1024 journal=1024 symbols=454 drops=0
[36340453017] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1106 watches=24 history=1024 journal=1024 symbols=454 drops=0
[37916174439] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=1132 watches=24 history=1024 journal=1024 symbols=454 drops=0
[39560029179] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=1165 watches=24 history=1024 journal=1024 symbols=454 drops=0
[41297865378] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=16000 nodes=1195 watches=24 history=1024 journal=1024 symbols=454 drops=0
[42488436870] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=3453 ops=1 watches=24
[42865388016] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=17000 nodes=1215 watches=24 history=1024 journal=1024 symbols=454 drops=0
[44518754148] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=18000 nodes=1248 watches=24 history=1024 journal=1024 symbols=454 drops=0
[46278835284] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=19000 nodes=1276 watches=24 history=1024 journal=1024 symbols=454 drops=0
[47849650332] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=20000 nodes=1302 watches=24 history=1024 journal=1024 symbols=454 drops=0
[49528622781] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=21000 nodes=1333 watches=24 history=1024 journal=1024 symbols=454 drops=0
[51376487415] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=22000 nodes=1363 watches=24 history=1024 journal=1024 symbols=454 drops=0
[53014702719] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=23000 nodes=1385 watches=24 history=1024 journal=1024 symbols=454 drops=0
[54539289519] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=24000 nodes=1414 watches=24 history=1024 journal=1024 symbols=454 drops=0
[56468109159] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=25000 nodes=1444 watches=24 history=1024 journal=1024 symbols=454 drops=0
[58114750749] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=26000 nodes=1470 watches=24 history=1024 journal=1024 symbols=454 drops=0
[59739100179] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=27000 nodes=1496 watches=24 history=1024 journal=1024 symbols=454 drops=0
[60597923001] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=1
[60875253780] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[60878072376] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[61889499276] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=28000 nodes=1537 watches=24 history=1024 journal=1024 symbols=454 drops=0
[63698540301] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=
```
</details>
