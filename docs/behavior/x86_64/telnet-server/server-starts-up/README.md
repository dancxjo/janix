# ✅ Scenario: Server starts up

> Last run: 2026-04-05 19:25:00

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 4052ms | - [📜](./01/serial.log) - |
| 2 | Then I should see a message in the serial output that says "telnetd: listening on guest port 2323" | ✅ | 5269ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12817311276] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[12822931473] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[12827053800] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[12829145967] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[12830671128] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[12831349806] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[12832025316] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[12832620075] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[12833264202] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[12833879553] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=33264
[12834471309] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=969224
[12835092237] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[12835852260] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[12836566710] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[12837365343] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[12837999108] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[12838629672] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[12839317491] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[12840049431] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[12840665673] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[12841246506] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[12841831464] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[12842427906] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[12843062595] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[12843701178] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[12844304649] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[12844905051] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[12845669298] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[12846300819] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[12846911187] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[12847543962] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[12848163141] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[12848840268] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[12849459084] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=168464
[12850097139] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[12850793703] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[12851539338] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[12852333252] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[12853108455] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[12853841385] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[12854553657] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[12855270582] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[12856687470] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[12858286023] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[12859205040] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[12859788381] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[12860302818] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[12860826396] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[12861408747] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[12862004892] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[12862587012] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[12863209425] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[12863731485] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[12864273741] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7795f000 (Usable)
[12864823323] [INFO] [kernel::memory] [CPU0]   [11] 0x7795f000 - 0x779c3000 (Reserved)
[12865459068] [INFO] [kernel::memory] [CPU0]   [12] 0x779c3000 - 0x779c4000 (Other)
[12866036766] [INFO] [kernel::memory] [CPU0]   [13] 0x779c4000 - 0x779c5000 (Reserved)
[12866663238] [INFO] [kernel::memory] [CPU0]   [14] 0x779c5000 - 0x779c6000 (Other)
[12867275817] [INFO] [kernel::memory] [CPU0]   [15] 0x779c6000 - 0x779c7000 (Reserved)
[12867847212] [INFO] [kernel::memory] [CPU0]   [16] 0x779c7000 - 0x779c8000 (Other)
[12868396497] [INFO] [kernel::memory] [CPU0]   [17] 0x779c8000 - 0x779c9000 (Reserved)
[12868964724] [INFO] [kernel::memory] [CPU0]   [18] 0x779c9000 - 0x77a54000 (Other)
[12869552157] [INFO] [kernel::memory] [CPU0]   [19] 0x77a54000 - 0x77a55000 (Reserved)
[12870123321] [INFO] [kernel::memory] [CPU0]   [20] 0x77a55000 - 0x77ed6000 (Other)
[12870672837] [INFO] [kernel::memory] [CPU0]   [21] 0x77ed6000 - 0x77ed7000 (Reserved)
[12871241262] [INFO] [kernel::memory] [CPU0]   [22] 0x77ed7000 - 0x77ff8000 (Other)
[12871792362] [INFO] [kernel::memory] [CPU0]   [23] 0x77ff8000 - 0x77ff9000 (Reserved)
[12872364186] [INFO] [kernel::memory] [CPU0]   [24] 0x77ff9000 - 0x787f9000 (Other)
[12872927397] [INFO] [kernel::memory] [CPU0]   [25] 0x787f9000 - 0x787fa000 (Reserved)
[12873548391] [INFO] [kernel::memory] [CPU0]   [26] 0x787fa000 - 0x788bb000 (Other)
[12874101471] [INFO] [kernel::memory] [CPU0]   [27] 0x788bb000 - 0x788bc000 (Reserved)
[12874726986] [INFO] [kernel::memory] [CPU0]   [28] 0x788bc000 - 0x788e6000 (Other)
[12875328906] [INFO] [kernel::memory] [CPU0]   [29] 0x788e6000 - 0x788e7000 (Reserved)
[12875987322] [INFO] [kernel::memory] [CPU0]   [30] 0x788e7000 - 0x788ec000 (Other)
[12876536937] [INFO] [kernel::memory] [CPU0]   [31] 0x788ec000 - 0x788ed000 (Reserved)
[12877104768] [INFO] [kernel::memory] [CPU0]   [32] 0x788ed000 - 0x788f2000 (Other)
[12877651347] [INFO] [kernel::memory] [CPU0]   [33] 0x788f2000 - 0x788f3000 (Reserved)
[12878213304] [INFO] [kernel::memory] [CPU0]   [34] 0x788f3000 - 0x7891f000 (Other)
[12878792322] [INFO] [kernel::memory] [CPU0]   [35] 0x7891f000 - 0x78921000 (Reserved)
[12879397608] [INFO] [kernel::memory] [CPU0]   [36] 0x78921000 - 0x7892a000 (Other)
[12879946530] [INFO] [kernel::memory] [CPU0]   [37] 0x7892a000 - 0x7892c000 (Reserved)
[12880511919] [INFO] [kernel::memory] [CPU0]   [38] 0x7892c000 - 0x78934000 (Other)
[12881056353] [INFO] [kernel::memory] [CPU0]   [39] 0x78934000 - 0x78935000 (Reserved)
[12881671737] [INFO] [kernel::memory] [CPU0]   [40] 0x78935000 - 0x7893f000 (Other)
[12882220263] [INFO] [kernel::memory] [CPU0]   [41] 0x7893f000 - 0x78940000 (Reserved)
[12882865446] [INFO] [kernel::memory] [CPU0]   [42] 0x78940000 - 0x7894d000 (Other)
[12883428525] [INFO] [kernel::memory] [CPU0]   [43] 0x7894d000 - 0x7894f000 (Reserved)
[12884117268] [INFO] [kernel::memory] [CPU0]   [44] 0x7894f000 - 0x7895d000 (Other)
[12884740308] [INFO] [kernel::memory] [CPU0]   [45] 0x7895d000 - 0x7895e000 (Reserved)
[12885364965] [INFO] [kernel::memory] [CPU0]   [46] 0x7895e000 - 0x7896a000 (Other)
[12885939000] [INFO] [kernel::memory] [CPU0]   [47] 0x7896a000 - 0x7896b000 (Reserved)
[12886505643] [INFO] [kernel::memory] [CPU0]   [48] 0x7896b000 - 0x7896f000 (Other)
[12887054268] [INFO] [kernel::memory] [CPU0]   [49] 0x7896f000 - 0x78970000 (Reserved)
[12887621934] [INFO] [kernel::memory] [CPU0]   [50] 0x78970000 - 0x78980000 (Other)
[12888170427] [INFO] [kernel::memory] [CPU0]   [51] 0x78980000 - 0x78981000 (Reserved)
[12888736905] [INFO] [kernel::memory] [CPU0]   [52] 0x78981000 - 0x78a11000 (Other)
[12889307442] [INFO] [kernel::memory] [CPU0]   [53] 0x78a11000 - 0x78a12000 (Reserved)
[12889876758] [INFO] [kernel::memory] [CPU0]   [54] 0x78a12000 - 0x78a1d000 (Other)
[12890463135] [INFO] [kernel::memory] [CPU0]   [55] 0x78a1d000 - 0x78a1e000 (Reserved)
[12891034992] [INFO] [kernel::memory] [CPU0]   [56] 0x78a1e000 - 0x78a43000 (Other)
[12891660210] [INFO] [kernel::memory] [CPU0]   [57] 0x78a43000 - 0x78a44000 (Reserved)
[12892273086] [INFO] [kernel::memory] [CPU0]   [58] 0x78a44000 - 0x78a50000 (Other)
[12892853292] [INFO] [kernel::memory] [CPU0]   [59] 0x78a50000 - 0x78a51000 (Reserved)
[12893422641] [INFO] [kernel::memory] [CPU0]   [60] 0x78a51000 - 0x78a5e000 (Other)
[12893969682] [INFO] [kernel::memory] [CPU0]   [61] 0x78a5e000 - 0x78a5f000 (Reserved)
[12894537447] [INFO] [kernel::memory] [CPU0]   [62] 0x78a5f000 - 0x78aa7000 (Other)
[12895086534] [INFO] [kernel::memory] [CPU0]   [63] 0x78aa7000 - 0x78aa8000 (Reserved)
[12895975587] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[13152062451] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485751 free frames
[13167152130] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[13172085564] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[13173384312] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[13174240002] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[13178978802] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[13180829145] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[13182123405] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[13182867885] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[13183547916] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[13184221182] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[13185230553] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[13186307805] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[13187089839] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[13187850753] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[13188596685] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[13189326447] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[13190609652] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[13191673143] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[13192414092] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[13194003933] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[13195014492] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[13196206386] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[13198255356] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[13200103653] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[13201134177] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[13201685541] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[13202484108] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[13588948857] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[13589959746] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[13593582849] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[13594814574] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[13595613471] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[13597248390] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[13612358991] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[13614589890] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[13615715289] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[13619185371] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[13620063765] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[13623634035] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[13632229248] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[13634153577] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[13647726609] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[13648397169] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[13665806847] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[13666489782] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[13668577428] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[13669883931] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[13671027150] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[13673610654] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[13674500565] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[13709594580] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62444700 ticks/sec), init_cnt=624447 for 100Hz
[13711302396] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[13712218377] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[13713473400] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[13719789732] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[13750500852] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[13751520882] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[13754965422] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[13776288471] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[13782073998] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[13783434753] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[13785129369] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[13786250973] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[13788480552] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[13790036898] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[13791116724] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[13792755768] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[13794788766] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[13796366166] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[13822606314] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[13823935257] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[13828931985] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[13830250995] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[13831775265] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[13833356295] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[13834295145] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[13835162616] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[13846033773] [INFO] [kernel::root] [CPU0] Spawning Root service...
[13847769144] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[13850977338] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[13852268925] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[13861937463] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[13867607325] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[13868771532] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[13870777173] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[13872449118] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[13873827429] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[13888553976] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[13893078210] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[13895037321] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[13895949144] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13930093419] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13954824939] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13957484970] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13961114508] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13962937329] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13965761040] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13968917292] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13971136476] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13971966822] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13973431197] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13981325325] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13984677003] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13988289744] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13991665281] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13994815593] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13999630788] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[14001192150] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[14002818555] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14005707045] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14018095542] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[14019598527] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[14029413123] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[14030692533] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[14066710614] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[14068013091] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[14458372170] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[15050646963] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[15093982134] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=499 journal=424 symbols=51 drops=0
[15136961928] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[16698067023] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=961 journal=769 symbols=95 drops=0
[17636255142] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[17753241561] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[17754414183] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[17856051972] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[17935083870] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[17975669712] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[17976871869] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[17977706076] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[17983624461] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[18004643217] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[18022936338] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[18025444866] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[18081542919] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[18103440960] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[18104534283] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[18113094879] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[18144246285] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[18168484059] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[18172037367] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[18173067693] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[18276684756] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[18278530446] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[18384564198] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[18461943192] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[18475146360] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[18479282217] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[18481699335] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[18493866105] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[18569047563] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[18575085012] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[18576573840] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[18577892157] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[18578802462] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[18579474474] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[18580233969] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[18580947396] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[18581553969] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[18582185193] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[18582846018] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=33264
[18583471071] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=969224
[18584344911] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[18585110643] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[18585850140] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[18586755231] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[18587504925] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[18588195813] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[18588845352] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[18589542939] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[18590179938] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[18590885709] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[18591604086] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[18592252371] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[18593175282] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[18593866038] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[18594855312] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[18595518150] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[18596267778] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[18596898738] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[18597563721] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[18598246953] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[18598902861] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[18599572629] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[18600219330] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=168464
[18600867516] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[18601599060] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[18602339580] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[18603101946] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[18603882627] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[18604652319] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[18605402772] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[18606157449] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[18607869951] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[18609861633] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[18610696995] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18619219608] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[18635584341] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[18640403430] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[18651573303] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[18652656396] [CONTRACT] [kernel] [CPU0] Spawning init process...
[18655813011] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[18658838748] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[18659645367] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [18666263055] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013232 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[18672331722] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[18684704379] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[18690617748] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[18692672460] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[18694509834] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[18704899719] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[18711949113] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[18717543867] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[18719267457] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[18726195279] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[18731823132] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[18733624701] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[18739650864] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[18741401778] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[18743106294] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[18744740586] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[18750712365] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[18752563335] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[18754552740] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[18757027608] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[18758885178] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[18760699749] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[18765305955] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[18811767249] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[18819322335] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[18826833564] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[18831635724] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[18835378089] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[18840721713] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[18845832192] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[18851420412] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[18856319955] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[18862677372] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[18870752373] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[18877630266] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[18883682994] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[18889886136] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[18895697436] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[18901362150] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[18908803716] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[18914797638] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[18920583099] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[18926846301] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[18931929588] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[18937950372] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[18943249974] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[18950395926] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[18956420868] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[18961587282] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[18967849956] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[18974563740] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[18986448030] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[19029893751] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[19039761048] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[19052958012] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[19091319324] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[19098295095] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/telnetd
[19104383727] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[19113833706] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[19133051256] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[19141164933] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[19148016162] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[19154453769] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[19164393171] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[19173097878] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[19177426752] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[19203792630] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[19368619743] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[19377392463] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[19378707480] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19380860004] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19385892801] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19387458981] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19396386042] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[19402288818] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[19403595387] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19405534665] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078768 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[19411602936] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[19412505156] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[19413759651] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19415909337] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19421284245] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[19423343973] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19433974857] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[19437573870] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[19440003594] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[19442192484] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[19443367746] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144304 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[19452536070] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[19455064926] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[19458422214] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19463405808] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 02:25:12 = 1775442312 unix_secs
[19465249881] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775442312, mono_ns=9732397851, offset=1775442302267602149ns
[19467553248] [INFO] [rtc_cmos] [CPU1] System clock anchored
[19485743343] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[19527266253] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[19537076493] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[19538925054] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19541794734] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19550816142] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[19554346944] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[19568526813] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[19574908914] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[19583542044] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19588974471] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210736 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[19595842926] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[19604051808] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[19608079986] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[19610349693] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19615661208] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19625219922] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19628836161] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19644306462] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[19651765023] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[19655530620] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[19661933082] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[19663424088] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19670238951] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277488 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[19678250031] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[19685604444] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[19686708393] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[19688462244] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[19689262362] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[19690454454] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[19713522246] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[19717632660] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[20233757973] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20238836178] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[20241893529] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[20243793834] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[20248466502] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[20250358623] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[20252746008] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[20254378023] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[20255477814] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[20258444613] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[20259498138] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[20260595355] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[20261558757] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[20262791670] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[20263945053] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[20265687915] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[20271685170] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[20272866603] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[20275683285] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20283770430] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20286631761] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[20295849915] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[20299393125] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[20300606238] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[20302566999] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352800 RFLAGS_BEFORE=134 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[20311426344] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[20326016733] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[20345327409] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[20356374819] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[20359135302] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[20365420119] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[20375102913] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[20376933126] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[20384732511] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[20385671394] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[20386666014] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[20387577606] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[20388482466] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[20389265886] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[20389854276] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[20391125469] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[20392000596] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[20399086686] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[20402989827] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[20405707938] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[20408575242] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[20409604116] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20412039945] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20418155901] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[20420810256] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20431803513] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[20436346095] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[20439203004] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[20439998337] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20441604645] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20446287576] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20447659782] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20454818835] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[20457458274] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
[20458589778] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010ac50
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[20461042569] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500240 RFLAGS_BEFORE=134 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[20470366290] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[20475409416] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[20484399573] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
[20490225888] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[20492557041] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[20494108041] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[20495888919] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[20497379298] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[20499295674] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[20501209047] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434000 RFLAGS_BEFORE=134 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[20507048760] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[20509145646] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[20510685492] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[20511569892] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[20517035748] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[20518114518] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20519821410] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20525337690] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20527997556] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20535133740] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[20540163930] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[20543073243] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[20545293384] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[20863892973] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[20865930921] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[20867074734] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583024 RFLAGS_BEFORE=130 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[20871502674] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[20874253653] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[20875340277] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[20877971037] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[20880742806] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[20890128798] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20894022930] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[20895127935] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[20896375962] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20898900396] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20908797129] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[20912412048] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[20923232814] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[20927939109] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[20931425724] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[20933367543] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[20934185844] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20935705296] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20965717080] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20970700014] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[20972946489] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[21786986265] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[21788656065] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[21791049489] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[21793898280] [INFO] [ps2_mouse] [CPU3] ps2_mouse: using cooperative polling loop (2ms interval)
[21891101001] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[21894228510] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cc30
[21895764495] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21897623616] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716928 RFLAGS_BEFORE=134 CR3_BEFORE=68890624 fs_base=0 gs_base=18446744071564586640
[21902992749] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using cooperative polling loop (2ms interval)
[21905279748] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
[21906839856] [INFO] [netd] [CPU3] NETD: Starting network stack service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[21908892126] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650640 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[21912729630] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[21915098436] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[21917195718] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[21926678994] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[21928215771] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[21929450829] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[21930953979] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[21945827343] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[21949008576] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[21951669663] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[21952741767] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21955411830] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22026849603] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[22054429419] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[22062893127] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[22066396671] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[22067854974] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22069942653] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369782720 RFLAGS_BEFORE=134 CR3_BEFORE=79892480 fs_base=0 gs_base=18446744071564586576
[22074628884] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[22075788933] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22078142823] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22079253834] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[22084148724] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[22091448588] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22093333944] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22112759493] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[22116289107] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[22124004903] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[22127497920] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[22129656615] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[22130368062] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22131928434] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22138225956] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[22140862524] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22148838228] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[22152271416] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010e130
[22153824792] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22155728166] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915104 RFLAGS_BEFORE=130 CR3_BEFORE=80826368 fs_base=0 gs_base=18446744071564586640
[22160497062] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[22161362157] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22163518971] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22164600414] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[22166620707] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[22171739931] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22175770023] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[22183399722] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[22186472154] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010e130
[22187703516] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22189279761] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981600 RFLAGS_BEFORE=134 CR3_BEFORE=80953344 fs_base=0 gs_base=18446744071564586576
[22192976025] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[22193841021] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22195683312] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22205218398] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[22208755140] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[22217536704] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[22220912769] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[22224020907] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[22225090800] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22226900454] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22256366154] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[22263867747] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[22272729897] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[22276378509] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0110590
[22277350128] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22278721212] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113648 RFLAGS_BEFORE=130 CR3_BEFORE=81256448 fs_base=0 gs_base=18446744071564586640
[22282033917] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[22283157039] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22285675599] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22286797104] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[22305293868] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[22308985479] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[22310085501] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[22313712630] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[22316157105] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22317848817] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[22319924385] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[22321732224] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[22325677605] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[22326852768] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22328430432] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370197984 RFLAGS_BEFORE=130 CR3_BEFORE=81530880 fs_base=0 gs_base=18446744071564586576
[22333093860] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[22333842927] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22335052311] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[22335845466] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22336882755] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[22346247231] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22347711375] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22349389128] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22351055199] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[22356750636] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[22358210094] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22359706380] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[22361321202] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[22383062493] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[22395117096] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[22400786892] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22402528170] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22404428442] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22406428704] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[22408824141] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[22412605875] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[22415574522] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[22417103610] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[22418419419] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22419825516] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[22421197920] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[22422760899] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22424584413] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849024 RFLAGS_BEFORE=134 CR3_BEFORE=80564224 fs_base=0 gs_base=18446744071564586608
[22428977967] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22430519100] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22431895992] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[22433310834] [INFO] [nectar] [CPU2] NECTAR: Started.
[22434113295] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22435730625] [INFO] [fontd] [CPU3] FONTD: Service ready
[22436528763] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[22438912023] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010e130
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22441386858] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047856 RFLAGS_BEFORE=130 CR3_BEFORE=81100800 fs_base=0 gs_base=18446744071564586608
[22449609864] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[22452663057] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22455170034] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370266800 RFLAGS_BEFORE=130 CR3_BEFORE=81739776 fs_base=0 gs_base=18446744071564586608
[22462645425] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[22464970473] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[22467182232] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[22472949246] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22475002671] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22477722861] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[22481893137] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[22483377642] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[22485726879] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[22487392686] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[22489792446] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[22494812076] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22496661561] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22498537677] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22499939880] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=235 subj_lo=0
[22504163550] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[22506613767] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f8000 phys=0x4e93000
[22507846944] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[22510249014] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[22513063947] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[22514447340] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[22518354243] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22520096346] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22521930321] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22523356680] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=238 subj_lo=0
[22527303777] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[22532918892] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22534218828] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22535443986] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22536620238] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=239 subj_lo=0
[22540683165] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[22551800832] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[22554584118] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[22556942694] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[22558561080] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[22559914608] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[22561281435] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[22562682153] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[22564149960] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[22565154249] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[22572499059] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=19, read=20)
[22577065632] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22578181296] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22579446780] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=21, read=22)
[22584268014] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22585686552] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=240 pred=0 subj_lo=0
[22591788153] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[22599218565] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22611656430] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1251 backend=VirtIO-GPU
[22613994942] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[22614946464] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22617007743] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22653515181] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[22673310264] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=23, resp=26
[22674997686] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[22700538795] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22702159425] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22703017425] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22751122020] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22766640204] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d5000 exec=false
[22783022691] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2eb000 exec=false
[22791376806] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[22795255923] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db450
[22796847150] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e3
[22798595754] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370491104 RFLAGS_BEFORE=130 CR3_BEFORE=82505728 fs_base=0 gs_base=18446744071564586640
[22807160145] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[22841874264] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[22842892611] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22845498819] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22847458722] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[22852933521] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22855367040] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[22864150749] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[22867588062] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[22869020856] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[22870673694] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[22871743917] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[22873184202] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db450
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[22875119553] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370564832 RFLAGS_BEFORE=130 CR3_BEFORE=84090880 fs_base=0 gs_base=18446744071564586576
[22881501588] [INFO] [echo] [CPU1] echo: starting up
[22883763639] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[22884752022] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22911014643] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=19, RX port=22
[22913048103] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[22915136442] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[22923139404] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[22929298359] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[22931348187] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[22947543927] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[22949495580] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[22951628832] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[22952867421] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22954819998] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22960512828] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22961618229] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22963437420] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[22967805036] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[22970742696] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22972619241] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22975474764] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[22979941347] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[22982093343] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[22983447597] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[22984552986557385] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22989173394] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23000925981] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[23004622773] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[23007514365] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[23012766381] [INFO] [netd] [CPU3] NETD: Driver TX port=19, RX port=22, link_up=true, mtu=1500
[23016220326] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[23017775913] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[23020953351] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[23022181842] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[23024067396] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[23024956581] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[23026246155] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23028095838] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23030705412] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[23032145928] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[23033545095] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[23034501996] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0115df0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23037451305] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370708192 RFLAGS_BEFORE=130 CR3_BEFORE=84332544 fs_base=0 gs_base=18446744071564586640
[23045565939] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[23048925966] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[23050562238] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/telnetd'
[23052610911] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/telnetd
[23053356348] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23055083073] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23063295552] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f3418
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23065911924] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370806496 RFLAGS_BEFORE=130 CR3_BEFORE=84480000 fs_base=0 gs_base=18446744071564586576
[23072768235] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[23085825279] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=220000 exec=false
[23091689412] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[23094137121] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[23095925556] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=228000 exec=false
[23097225030] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[23108321412] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 32 (user task/process) assigned to CPU 2
[23111493636] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=32)
[23112892869] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[23125205466] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f3450
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23127274500] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=32 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370872032 RFLAGS_BEFORE=130 CR3_BEFORE=84582400 fs_base=0 gs_base=18446744071564586608
[23134208031] [INFO] [telnetd] [CPU2] telnetd: starting on port 2323
[23137545453] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[23154023904] [INFO] [telnetd::net_client] [CPU2] telnetd: connected to socket API (netd=27, write=29, read=30)
[23161750920] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[23162964066] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[23169164865] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[23178208581] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=31, our_read=32)
[23180258574] [INFO] [anther] [CPU1] anther: Connected to network stack
[23201580534] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[23205704709] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[23235969735] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[23255062974] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[23280112581] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[23282849898] [INFO] [bloom] [CPU3] bloom: creating surface...
[23284188741] [INFO] [bloom] [CPU3] bloom: surface created!
[23285335260] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[23290265592] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[23296106889] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1263
[23297610435] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[23313612333] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[23318122278] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=33)
[23319896754] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[23321958132] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0AF0 [23353081158] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[23425816920] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1263
[23449909428] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[23463331419] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[23475165879] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
T:1050 T:0EC0 T:F930 [23504534262] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[23507657283] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[23525130915] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=268
[23527517508] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[23528854371] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[23529935847] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[23532084048] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23533672107] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23535361146] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=268 subj_lo=0
[23551553586] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[23562026235] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 37 (user thread) assigned to CPU 3
[23567246307] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[23672090508] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=623 watches=11 history=1024 journal=1024 symbols=273 drops=0
[23697335178] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[23705087274] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1269)
[23707046220] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
T:1AA0 [23719666674] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[24324405798] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=37 (priority=2)
[24331245444] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=240
[24332686488] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[24333786015] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[24334845249] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24335956095] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24337340214] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=240 pred=0 subj_lo=0
[24349432239] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[24358599210] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1294)
[24360034974] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[24372731229] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=306
[24374397993] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[24375981234] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[24377292984] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24378569787] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24379798872] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=306 subj_lo=0
[24389323761] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1295)
[24390714051] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[24851363163] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[24853768698] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[24888761106] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[24920198721] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[25006571898] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[25039137618] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[25041969777] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[25050717483] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[26107478298] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[27279583980] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[27286872888] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[27308965035] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[27311626452] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[27411110529] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[27543929094] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[27556629144] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[27558348048] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[27571578870] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[27572895669] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[27574082844] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[27575244741] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=321 pred=0 subj_lo=0
[27658801797] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[27659986266] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[27661125954] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[27662271813] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=322 pred=0 subj_lo=0
[27769692456] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[27789130116] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[27790712400] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[27791820012] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[27793081866] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=323 pred=0 subj_lo=0
[27894232806] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=694 watches=16 history=1024 journal=1024 symbols=335 drops=0
[27968564217] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[27984991815] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12004000
[27986889513] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[27988762659] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[28066646157] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[28077958194] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[28082943966] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[28085736261] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[28087988478] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[28092623625] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[28101308499] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[28102538607] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[28104348591] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28106239359] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=324 pred=0 subj_lo=0
[28124072955] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[28126261284] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[28128920820] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[28148384880] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x12005000
[28150552914] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[28513759791] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[28520574060] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[29548778028] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[29873454633] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[29875449879] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[29878081332] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[29897743788] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[29901605943] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[29903407875] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[30058432767] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[30097470414] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[30102531030] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[30105549474] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[30110382291] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 2323 with backlog 64
[30114143136] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 2323, handle=1
[30116681298] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[30117866658] [INFO] [telnetd] [CPU2] telnetd: listening on guest port 2323 (handle=1)
[30119137422] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=2
[30120878403] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[30122361390] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=3
[30123684228] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[30124977003] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=4
[30126041319] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=2)
[30140667414] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30172087539] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[30175976787] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=5
[30204163770] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[30217591767] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[30220288824] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[30229661616
```
</details>
