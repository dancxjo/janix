# ❌ Scenario: Serve concurrent requests

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the anther server is ready | ❌ | 39349ms | - [📜](./01/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11965489206] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11971042776] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11974693665] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11976738180] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11977948554] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11978604396] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11979305679] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11979912879] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11980584000] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11981224530] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11981842125] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11982488859] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11983223373] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11983913238] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11984626731] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11985260232] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11985921618] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11986542513] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11987181459] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11987801991] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11988403317] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11989050282] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11989670583] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11990286759] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11990946198] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11991574947] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11992207623] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11992972233] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11993604381] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11994235902] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11995027143] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11995806570] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11996480859] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11997124854] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11997741129] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11998464687] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11999321466] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[12000100200] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[12000846759] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[12001607937] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[12002378619] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[12003125409] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[12004477155] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[12005948361] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[12006761349] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[12007329312] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[12007961097] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[12008508732] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[12009193548] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[12009743658] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[12010282779] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[12010827015] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[12011364816] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[12011942250] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[12012513249] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[12013102266] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[12013670691] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[12014259675] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[12014828496] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[12015432066] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[12016001382] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[12016588518] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[12017156448] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[12017744574] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[12018311679] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[12018907065] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[12019476018] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[12020089191] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[12020658738] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[12021245346] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[12021825915] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[12022413018] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[12022980750] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[12023567226] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[12024137169] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[12024723810] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[12025304181] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[12025892307] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[12026459643] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[12027047703] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[12027614940] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[12028202274] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[12028786176] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[12029377932] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[12029948304] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[12030540720] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[12031240518] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[12031867023] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[12032443236] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[12033031362] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[12033625230] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[12034216953] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[12034787589] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[12035451582] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[12036023802] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[12036698652] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[12037277010] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[12037873419] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[12038461182] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[12039055215] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[12039628722] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[12040221963] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[12040796658] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[12041391648] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[12041976408] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[12042570210] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[12043147446] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[12043973502] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12291762681] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[12303125538] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12308155893] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12309578589] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12310469226] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12314871261] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12316617852] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12317718996] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12318441168] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12319136016] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12319863600] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12320895939] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12321902901] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12322615767] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12323317677] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12324019851] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12324719253] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12325850955] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12326899992] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12327629853] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12329284044] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12330252231] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12331285461] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12332931369] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12334521144] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12335338818] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12335898201] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12336784251] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12721588143] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12722905173] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12726285957] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12727211277] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12728010570] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12729599916] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12743645310] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12745431039] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12746267490] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12748137303] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12748947354] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12752051499] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12760465377] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12762216093] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12775950396] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12776571753] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12793038291] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12793625097] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12795704130] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12797478969] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12798557244] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12800825202] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12801662445] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12836589777] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62349600 ticks/sec), init_cnt=623496 for 100Hz
[12838013892] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12838930434] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12840531792] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12846897360] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12877834167] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12879148986] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12880958838] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12882992760] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12884812512] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12890079840] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12891602757] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12909269604] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12910270296] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12911091699] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12911946465] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12913936860] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12915842544] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12916824723] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12941994549] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12943578516] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12944775492] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12946105161] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12947545413] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12948735624] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12949942599] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12950830233] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12958466433] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12959430792] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12961303707] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12962169429] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12965984625] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12967601361] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12968463849] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12969819885] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12970823481] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12971741640] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12983894946] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12987473598] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12989136369] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12990502635] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13016328105] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13036135563] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13038248949] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13042159812] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13043764107] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13046031240] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13048371699] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13050358761] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13051200789] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13052426178] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13058621235] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13060389804] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13062939285] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13067465070] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13070038179] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13070845854] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13071863046] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13083617844] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13084558113] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13094080230] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13094951298] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13123325688] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13124190915] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13494048843] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[14003124333] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[14034206802] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=499 journal=424 symbols=51 drops=0
[14068102653] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[15778962639] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=963 journal=771 symbols=97 drops=0
[16907494461] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[17054228037] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[17055452799] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[17167049196] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[17225615583] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[17259533082] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[17260714680] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[17261710191] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[17266231719] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[17285738283] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[17306028828] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[17309414067] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[17364877695] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[17385944169] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[17386732275] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[17390818302] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[17429542251] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[17450831013] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[17455102236] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[17456139030] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[17527083684] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[17528855685] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[17617594797] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[17685266082] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[17696768694] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[17700688500] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[17703015495] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[17727653295] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[17775425679] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[17780518470] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[17781529095] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[17782381023] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[17783385774] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[17784102006] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[17784772302] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[17785445172] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[17786091939] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[17786744481] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[17787411015] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[17788058376] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[17788717353] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[17789410452] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[17790125496] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[17790874893] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[17791550502] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[17792230071] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[17792894559] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[17793564921] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[17794214658] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[17794844760] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[17795482551] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[17796140769] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[17796818655] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[17797520499] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[17798208945] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[17799127335] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[17799891351] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[17800539438] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[17801198943] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[17801872605] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[17802561942] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[17803222866] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[17803890588] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[17804535540] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[17805628467] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[17806908174] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[17807704596] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[17808465411] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[17809279554] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[17810258829] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[17811073401] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[17812585131] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[17814376371] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[17815185003] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17823409065] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[17838349089] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[17843112111] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[17852416560] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[17853162492] [CONTRACT] [kernel] [CPU0] Spawning init process...
[17855371017] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[17858404047] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[17859126285] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [17865941148] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[17872195737] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[17890507338] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[17893793610] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[17895788394] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[17897737968] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[17907432048] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[17913959679] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[17918215029] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[17919987360] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[17924489946] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[17929111068] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[17930991837] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[17936414562] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[17938608237] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[17940503031] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[17941973907] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[17946648357] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[17948465667] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[17950375212] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[17952635877] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[17955056856] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[17957495919] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[17962714935] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[18010689444] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[18018317229] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[18025068996] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[18031390773] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[18035720637] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[18041067297] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[18047539785] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[18054298119] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[18060943494] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[18067993020] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[18074901009] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[18082227570] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[18089412858] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[18095498421] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[18101976519] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[18108149928] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[18114303273] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[18120393489] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[18126419718] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[18132843036] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[18139372812] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[18146095176] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[18153091440] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[18160278972] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[18167355459] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[18173906916] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[18180446130] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[18186643101] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[18192247953] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[18197839110] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[18202158678] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[18205995720] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[18211594764] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[18217169817] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[18223346823] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[18229799742] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[18236773170] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[18244679475] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[18253642836] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[18262476870] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[18271014366] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[18275096235] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[18304668624] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[18495185907] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[18505684395] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[18506944269] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18509599812] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18516959934] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18519030123] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18533702154] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[18539288592] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18540836589] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18542639049] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[18549705174] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[18550803909] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[18551770446] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18553712826] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18558687081] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18561050673] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18569222100] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[18573486921] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[18574445010] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[18576821538] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[18577763556] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[18585657288] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[18588395529] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[18591349821] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18596048427] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:46:48 = 1775436408 unix_secs
[18597896592] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775436408, mono_ns=9298717164, offset=1775436398701282836ns
[18599841942] [INFO] [rtc_cmos] [CPU1] System clock anchored
[18611710590] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[18653491428] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[18665906721] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[18667190586] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18670087656] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18678966999] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18682745598] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[18694353084] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[18700117095] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[18705732705] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18707967861] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[18715218225] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[18721732161] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[18724222374] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[18725347113] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18728103273] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18739166919] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18742581660] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18753577194] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[18758336817] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18759576264] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18761622264] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[18766010076] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[18769850187] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[18772097157] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[18779397252] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[18782298546] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[18783466152] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[18784434273] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[18785606598] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[18808046862] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[18809760651] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[19323045588] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19329064590] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[19331918364] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[19333285059] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[19337629674] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[19339202553] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[19341282708] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[19342714347] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[19343689662] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[19345976991] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[19346977782] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[19347936036] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[19348783641] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[19349574882] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[19350699489] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[19352006190] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[19356667506] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[19357900221] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19359940776] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19367777517] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19370244927] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19377759159] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[19380599733] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[19382084238] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[19384321506] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352768 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[19394376507] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[19412388996] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[19428625359] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[19468903641] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[19472329734] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[19480599171] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[19482168222] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[19492950147] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[19495098348] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[19496400066] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[19497306345] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[19499501340] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[19501522788] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[19502419959] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19504131240] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19507969305] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[19509734475] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19517414367] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[19520379615] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[19522500723] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[19523311104] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19525299288] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19526288892] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[19530398778] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19542355833] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19552783734] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[19557200256] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[19565938161] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[19568039139] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500432 RFLAGS_BEFORE=130 CR3_BEFORE=60207104 fs_base=0 gs_base=18446744071564586640
[19572798234] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[19573925250] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19576318872] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19577555910] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[19579294152] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[19584926889] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19586265864] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[19588487886] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[19589498577] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19591103532] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[19594469796] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[19596616809] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[19598716731] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[19601854635] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[19603215192] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[19604707188] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[19607671743] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[19610208156] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[19612897326] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[19615074765] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[19618387602] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[19619747631] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[19621098849] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[19622433237] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[19623694398] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[19625084160] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[19626772275] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[19636342110] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[19638800445] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434096 RFLAGS_BEFORE=134 CR3_BEFORE=60096512 fs_base=0 gs_base=18446744071564586608
[19647314412] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[20051218242] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
[20052590415] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[20054725185] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583120 RFLAGS_BEFORE=130 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[20058447882] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[20061521040] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[20062569087] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[20064319869] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[20067482391] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[20069622870] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20073920856] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[20075249502] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20078551812] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20084586951] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[20089166328] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[20092908495] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[20104344018] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[20109554850] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[20112761625] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[20115115944] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[20116277709] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20118855042] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20149527849] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20156115936] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[20187623247] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[20192092173] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[20193230475] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20195289279] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716848 RFLAGS_BEFORE=130 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[20202683391] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[20204327616] [INFO] [netd] [CPU3] NETD: Starting network stack service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[20206381041] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650480 RFLAGS_BEFORE=134 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[20211360147] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[20214796041] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[20217318429] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[20232255021] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[20235447573] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[20242511091] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[20244365295] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[20245884285] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[20247851382] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[20249331960] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[20279046975] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[20286573648] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[20288364624] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[20290282881] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[20292203547] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[20294522061] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[20295950334] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[20298946173] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4256000
[20301657486] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[20305322334] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[20308208448] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[20310489078] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[20323662315] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[20335283397] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[20348650707] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[20352074358] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[20354415114] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[20356211535] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[20357876715] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[20359748046] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[20361446622] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[20362833612] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[20364206577] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[20369753217] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=15, read=16)
[20375870658] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=17, read=18)
[20380650114] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[20383253946] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[20385603513] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[20386523883] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20388296577] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20457146391] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[20482927047] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[20490957069] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[20494437975] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[20495867469] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20497732728] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369914576 RFLAGS_BEFORE=130 CR3_BEFORE=69668864 fs_base=0 gs_base=18446744071564586576
[20502226536] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[20503416549] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20506423080] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20507544486] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[20512264443] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[20519299812] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20521624035] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20523573048] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20538651111] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20542093968] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[20549694231] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[20552809530] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[20555064783] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[20555943342] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20557956705] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20564643561] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20568237426] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20579257050] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[20583676806] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[20584858338] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20586658554] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370046960 RFLAGS_BEFORE=134 CR3_BEFORE=70602752 fs_base=0 gs_base=18446744071564586640
[20590287432] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[20591406165] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20593296339] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[20594209911] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20595334914] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[20605269201] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20606676288] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20611901013] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[20622006042] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[20624839686] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[20626851300] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20628936768] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113488 RFLAGS_BEFORE=130 CR3_BEFORE=70729728 fs_base=0 gs_base=18446744071564586576
[20633939469] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[20635055298] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20636972466] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20646553290] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[20649940872] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[20657431113] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[20660928420] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[20663814105] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[20664856905] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20666565216] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20692544532] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20697737841] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[20707248903] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[20710425747] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cac8
[20711865273] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20713619289] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370245856 RFLAGS_BEFORE=130 CR3_BEFORE=71032832 fs_base=0 gs_base=18446744071564586640
[20718407688] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[20719208070] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20720774646] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20721569121] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[20738882109] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[20743157787] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20749321560] [INFO] [fontd] [CPU3] FONTD: Service node created, req=19, resp=22
[20753784447] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[20758159686] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1120
[20759592711] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20761479552] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20763726060] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370331040 RFLAGS_BEFORE=134 CR3_BEFORE=71307264 fs_base=0 gs_base=18446744071564586576
[20768373483] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20770554453] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20772913656] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[20774629029] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[20775866661] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20777873160] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[20779792836] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20781171708] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[20789624757] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20791448205] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20793523707] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20795634684] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[20800913562] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20802564882] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20804186007] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20806232964] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[20831263464] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[20836621344] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[20837818584] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[20839766541] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[20841728292] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[20845406472] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[20854211103] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[20857677027] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[20867386518] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20870442054] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[20871930354] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20874554910] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369980976 RFLAGS_BEFORE=134 CR3_BEFORE=70340608 fs_base=0 gs_base=18446744071564586608
[20881086534] [INFO] [nectar] [CPU2] NECTAR: Started.
[20884513881] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20885855034] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20887112928] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
[20888670396] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20889940071] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20892419724] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370180064 RFLAGS_BEFORE=130 CR3_BEFORE=70877184 fs_base=0 gs_base=18446744071564586608
[20900166507] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[20902740210] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010dfc8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20904951804] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370398544 RFLAGS_BEFORE=134 CR3_BEFORE=71516160 fs_base=0 gs_base=18446744071564586608
[20912294898] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[20913979416] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[20915725842] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[20924126025] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[20927521230] [INFO] [fontd] [CPU3] FONTD: Service ready
[20964663423] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[20995821660] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21003935733] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[21011367201] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=15, RX port=18
[21024154041] [INFO] [netd] [CPU3] NETD: Driver TX port=15, RX port=18, link_up=true, mtu=1500
[21029372793] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21037683183] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[21039738357] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21042782046] [INFO] [netd] [CPU3] NETD: Created socket API port (write=23, read=24)
[21052100388] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21053472792] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21054989406] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21056844171] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[21067846932] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1251 backend=VirtIO-GPU
[21071227518] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[21072432777] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21075401721] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21090525588] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([229, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[21092870865] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21094484730] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21096155388] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21097842909] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=245 subj_lo=0
[21114989049] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[21115951560] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21117522789] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21119108439] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[21120222519] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21121891857] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=248 subj_lo=0
[21128256336] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[21131491953] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21132994377] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21134552274] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21136212801] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=249 subj_lo=0
[21145258332] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21146719506] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21148264533] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21149986209] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=250 subj_lo=0
[21159468066] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21160882380] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21162444600] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21164097801] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=251 pred=0 subj_lo=0
[21176488872] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[21208723668] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[21230961774] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=25, resp=28
[21232320879] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[21251950005] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[21261724605] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=29, our_read=30)
[21264273030] [INFO] [anther] [CPU1] anther: Connected to network stack
[21267522705] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[21275327733] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[21278208600] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db4d8
[21279600408] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e3
[21281422338] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370559056 RFLAGS_BEFORE=134 CR3_BEFORE=72433664 fs_base=0 gs_base=18446744071564586640
[21285816585] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[21287213475] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21290022303] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21291348705] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[21298334541] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[21300726579] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21312748776] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[21317421873] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[21320381049] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[21327859278] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[21333647445] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[21334890720] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db4d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[21337112742] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370624592 RFLAGS_BEFORE=134 CR3_BEFORE=73814016 fs_base=0 gs_base=18446744071564586576
[21343656279] [INFO] [echo] [CPU1] echo: starting up
[21345052971] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[21346977267] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[21365674902] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[21368901906] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[21384217701] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[21388020588] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[21396988272] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[21399246231] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[21406951995] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[21416194239] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[21417933933] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[21420074808] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[21422057250] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[21424601748] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[21441482832] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[21442873914] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[21445207146] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[21447876450] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[21449004489] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21451094346] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[21452406591] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21460609401] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21462740508] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[21463974213] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[21474236091] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[21476981394] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[21482010825] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[21484931589] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[21487294323] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[21489883503] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[21491120640] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21493652796] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21505043307] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21510684096] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[21521689233] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[21525999891] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[21528254913] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[21531210723] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[21532478748] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21534866793] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21536892960] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb01111e0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21539140953] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370759760 RFLAGS_BEFORE=134 CR3_BEFORE=74780672 fs_base=0 gs_base=18446744071564586640
[21543677298] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[21545539686] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[21555670818] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[21557841162] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[21560998305] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[21563556234] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[21565506303] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[21572267706] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[21574773000] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[21582187902] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f41e8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21583904760] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370825296 RFLAGS_BEFORE=134 CR3_BEFORE=74940416 fs_base=0 gs_base=18446744071564586576
[21587597328] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[21589650126] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[21594951807] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[21595916496] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[21599009685] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[21615870342] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[21620106024] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[21621975639] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[21626481327] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[21649377717] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[21651854136] [INFO] [bloom] [CPU3] bloom: creating surface...
[21652936668] [INFO] [bloom] [CPU3] bloom: surface created!
[21653811564] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[21659936100] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[21664763736] [INFO] [netd] [CPU3] NETD: DHCP configured ��� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[21668417562] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[21674360664] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[21677153025] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[21679905918] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[21680785764] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[21682066659] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[21687493476] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1263
[21688535154] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[21700732581] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[21703597905] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[21705010371] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[21706414752] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [21717360159] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[21730369782] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[21742651095] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[21755002170] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
T:07D0 T:0640 T:F0B0 [21771951102] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[21786802884] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[21789790671] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[21796094496] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=279
[21799015128] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[21800668032] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[21802227051] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21804100395] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21806100195] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=279 subj_lo=0
[21822704013] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[21836613414] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[21839119599] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[21840274302] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[21844004424] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[21847452792] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[21854383518] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1265)
[21856230264] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[21862217751] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[21865521084] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[21871106895] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
T:1220 [21885606963] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[21889299597] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[21902516097] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=251
[21905753298] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[21907339311] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[21909017691] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21911153055] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21912992046] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=251 pred=0 subj_lo=0
[21920826576] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1263
[21931893852] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[21934837848] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[21938240346] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1274)
[21939222129] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[21941412768] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[21949596900] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[21956558811] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[21959215773] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[21965081160] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=74
[21975568824] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=281
[21976850808] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=17
[21979081509] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[21980777973] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[21981908586] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[21983561292] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21985228881] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21987027711] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=281 subj_lo=0
[22001708223] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[22009520907] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[22032948465] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1276)
[22035188439] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[22037752308] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[22046741343] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[22048326993] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[22050252807] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[22053649926] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22059724005] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=149
[22062551082] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[22065910119] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[22073474181] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22192120104] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[22498286316] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[22609279869] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[22653023679] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[22738636239] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[22879464828] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[22912178355] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=663 watches=13 history=1024 journal=1024 symbols=316 drops=0
[23010372462] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[23236531065] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[23541484659] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[23796099129] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[24028441206] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[24134617485] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[24338364963] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[24633908277] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[24652669305] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[24664640418] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[24666540525] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[24668068722] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[24669722385] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[24671603847] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[24700538808] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[24702075387] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[24703616157] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[24705771651] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=340 pred=0 subj_lo=0
[24956262111] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[25143953802] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=721 watches=15 history=1024 journal=1024 symbols=342 drops=0
[25245180213] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[25247248917] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[25250436486] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[25259072355] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[25383985011] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[25391760933] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[25393993977] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:32822 on listener 1
[25425201846] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[25427719416] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[25540499391] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[25621386153] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[25623180528] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25624931541] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25627012191] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=311 pred=0 subj_lo=0
[25879263828] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[26024440926] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[26097802929] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[26150732256] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[26207723322] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26209075563] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26210408037] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26212038963] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=312 pred=0 subj_lo=0
[26278294086] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[26295873219] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[26303271423] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[26305544034] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[26321509500] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12227000
[26323030470] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[26325109734] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[26368530837] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=9d85f11f1afc2373)
[26388693342] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[26399584365] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[26405349630] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[26407945641] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[26410687413] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[26415522936] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[26421960708] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=149
[26424744225] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[26427918099] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[26437752660] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[26439867729] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[26443003257] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[26488408551] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x12228000
[26489762937] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[26730520938] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[27175685130] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=14aa2015f9906e3f)
[27288807447] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[27293305017] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=3
[27303095919] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 37 (user thread) assigned to CPU 1
[27305429019] [INFO] [anther] [CPU1] anther: Thread spawned TID=37 for conn_handle=3
T:5EF0 [27323894169] [INFO] [anther] [CPU1] anther: Worker thread TID=37 starting for conn_handle=3
[27337169013] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[27348746832] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=33, our_read=34)
[27351120654] [INFO] [anther] [CPU1] anther: Worker TID=37 connected to netd OK
[27373655991] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[27376987341] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[27390792924] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 85 bytes on conn_handle=3
[27402520266] [INFO] [anther] [CPU1] anther: GET /health Http11
[27619080522] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[27749138604] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=783 watches=17 history=1024 journal=1024 symbols=383 drops=0
[27852746526] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[27855352107] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[27857767113] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[27859274454] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[27861919866] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28005457821] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[28143104748] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28168543821] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[28195288308] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[28199932629] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[28202920812] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[28205392512] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[28209673107] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28220257725] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[28221896208] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28225173933] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[28228397439] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28229885838] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28235198673] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 0 bytes on conn_handle=3
[28416669303] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[28445715540] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[28447296372] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[28448677323] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28450007652] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=314 pred=0 subj_lo=0
[28547636370] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[28554762819] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[28557625404] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[28560653385] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28600008030] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28603933281] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[28606762635] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[28609499952] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[28631841150] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[28636033008] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[28639876023] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28647093387] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28654451232] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=149
[28658545938] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[28662759180] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[28678507110] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[28685467866] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[28689951774] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:32832 on listener 1
[28737736698] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[28739580210] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[28741407750] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28743368709] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=342 pred=0 subj_lo=0
[29121677442] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[29578118427] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[30029234301] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[30533505453] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[30919307331] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=853 watches=19 history=1024 journal=1024 symbols=385 drops=0
[31068944214] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[31574821806] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[31856459547] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[31860021864] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[31863595764] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[32124320217] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[32557424988] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[33014904318] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=149
[33018507456] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[33021498246] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[33043992927] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[33446511186] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=895 watches=19 history=1024 journal=1024 symbols=385 drops=0
[33636420477] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[33729481137] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2431 ops=1 watches=19
[34657320489] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[35254114896] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[35257370940] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[35260830891] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[35386313391] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[35823372090] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=918 watches=19 history=1024 journal=1024 symbols=386 drops=0
[36137713755] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[36745972098] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[37199550465] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[37374649686] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[37743995883] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[37872104094] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[37896032823] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=4
[37905230616] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 38 (user thread) assigned to CPU 1
[37907339514] [INFO] [anther] [CPU1] anther: Thread spawned TID=38 for conn_handle=4
T:5EF0 [37927511820] [INFO] [anther] [CPU1] anther: Worker thread TID=38 starting for conn_handle=4
[37958328342] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[37959765723] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=35, our_read=36)
[37961235081] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[37962451362] [INFO] [anther] [CPU1] anther: Worker TID=38 connected to netd OK
[37965146010] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[38079906348] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=954 watches=19 history=1024 journal=1024 symbols=390 drops=0
[38086946205] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 85 bytes on conn_handle=4
[38088317652] [INFO] [anther] [CPU1] anther: GET /health Http11
[38116126422] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[38118996663] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[38121363654] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=70
[38123052198] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[38126547129] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[38137150590] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[38168024763] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[38176142070] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[38185902777] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=70
[38189331873] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[38193805914] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[38199292362] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[38204177583] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=70
[38206849791] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[38212423953] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[38228508945] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[38232253323] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 0 bytes on conn_handle=4
[40894923762] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1037 watches=19 history=1024 journal=1024 symbols=455 drops=0
[41743761345] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[41808985779] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=70
[41819645769] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[41823599136] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[41866158180] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[41929614606] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[41931849927] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[41938010829] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[41939107221] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[41950199148] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=70
[41959404927] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[41961070536] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[41964474156] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[41970934533] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=149
[41973537969] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[41976874731] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[41992754694] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[41997039843] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[41999004663] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:45308 on listener 1
[42190902831] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[42192969720] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[42217174824] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[42992561337] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[43070235549] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[43071952110] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[43073885052] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[43075835682] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[43089683934] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[43091433825] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[43093204902] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[43095150813] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=234 pred=0 subj_lo=0
[43107175155] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[43108837530] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[43110656721] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[43112722785] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=473 pred=0 subj_lo=0
[43129420851] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[43131136851] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[43132993761] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[43134941982] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=472 pred=0 subj_lo=0
[43147210524] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[43148977179] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[43150735188] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[43152683739] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[43165287594] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[43241613756] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1084 watches=24 history=1024 journal=1024 symbols=474 drops=0
[45095797659] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=70
[45097611735] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[45100379841] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[45748460505] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1104 watches=24 history=1024 journal=1024 symbols=474 drops=0
[46178458623] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=149
[46182143865] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[46208490240] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[48095249103] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[48097128321] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=2 (16384b)
[48100174485] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[48102579162] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[48422451594] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=25 len=70
[48425707143] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[48429062286] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[48697970739] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=1138 watches=24 history=1024 journal=1024 symbols=474 drops=0
[50913639381] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=1171 watches=24 history=1024 journal=1024 symbols=474 drops=0
[53296056858] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=16000 nodes=1193 watches=24 history=1024 journal=1024 symbols=474 drops=0
[54421035537] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=26 len=70
[54423020289] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[54425788263] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[55060022721] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=27 len=70
[55063407267] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[55067079177] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[55242663168] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[55248588780] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=5
[55258409910] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 39 (user thread) assigned to CPU 1
[55260590055] [INFO] [anther] [CPU1] anther: Thread spawned TID=39 for conn_handle=5
T:5EF0 [55298376276] [INFO] [anther] [CPU1] anther: Worker thread TID=39 starting for conn_handle=5
[55347115923] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=37, our_read=38)
[55349788329] [INFO] [anther] [CPU1] anther: Worker TID=39 connected to netd OK
[55451992200] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=3455 ops=1 watches=24
[55527374529] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=17000 nodes=1213 watches=24 history=1024 journal=1024 symbols=474 drops=0
[55811676756] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[56060809431] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=28 len=149
[56064260043] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[56067297429] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[56073978774] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[56155138941] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[56159094981] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[56168068407] [INFO] [anther] [CPU1] anther: Worker TID=39 got first 85 bytes on conn_handle=5
[56170406292] [INFO] [anther] [CPU1] anther: GET /health Http11
[56198736198] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[56203693260] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[56212219173] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=29 len=70
[56215556199] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[56219631699] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[58195792380] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=18000 nodes=1244 watches=24 history=1024 journal=1024 symbols=474 drops=0
[59079495552] [INFO] [anther::net_client] [CPU1] anther: tcp_send chunk timeout
[59453507883] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[59879354205] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[59884785444] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[59890017759] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=30 len=70
[59893035939] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[59897081112] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[59905253496] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=31 len=70
[59908284711] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[59911895472] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[59925955353] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[59929260666] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[59932171497] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[59935853175] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=70
[59938310520] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[59942912370] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[59968241289] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[59970638112] [INFO] [anther::net_client] [CPU1] anther: tcp_recv got unexpected resp_type 0x0000 len=6
[60184478772] [INFO] [anther] [CPU1] anther: Worker TID=39 got first 0 bytes on conn_handle=5
[60786579150] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=19000 nodes=1272 watches=24 history=1024 journal=1024 symbols=474 drops=0
[61676382174] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=70
[61678899480] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[61682547465] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[62072260932] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[62077012239] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[62088029058] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[62091014568] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[62107403028] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[62110755168] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[62114288808] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[62124092580] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[62129157915] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=149
[62130506031] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[62134713333] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[62138426097] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[62139865062] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[62834957592] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[62842536768] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[62844687180] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:45322 on listener 1
[62850251376] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[63547784124] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=20000 nodes=1299 watches=24 history=1024 journal=1024 symbols=474 drops=0
[64964653281] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[64966878801] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[64969435806] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[66058414851] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=21000 nodes=1328 watches=24 history=1024 journal=1024 symbols=474 drops=0
[67616450286] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=70
[67619194269] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[67624639203] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[68292261378] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[68294459805] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[68298030009] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[68566954005] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=22000 nodes=1357 watches=24 history=1024 journal=1024 symbols=474 drops=0
[69269874663] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[69275456877] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[69279369588] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[70875357168] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=23000 nodes=1381 watches=24 history=1024 journal=1024 symbols=474 drops=0
[71671005417] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[73405121196] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=24000 nodes=1418 watches=24 history=1024 journal=1024 symbols=474 drops=0
[74192993490] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[74194954713] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[74198381268] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[74921403645] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[74925643848] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[74930451453] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[76268924325] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=25000 nodes=1444 watches=24 history=1024 journal=1024 symbols=474 drops=0
[76432123515] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[76442717175] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=6
[76444215441] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[76447848411] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[76454735346] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[76462064382] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 40 (user thread) assigned to CPU 1
[76466265216] [INFO] [anther] [CPU1] anther: Thread spawned TID=40 for conn_handle=6
T:5EF0 [76555610043] [INFO] [anther] [CPU1] anther: Worker thread TID=40 starting for conn_handle=6
[76614485013] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=39, our_read=40)
[76619289483] [INFO] [anther] [CPU1] anther: Worker TID=40 connected to netd OK
[77380582257] [INFO] [anther] [CPU1] anther: Worker TID=40 got first 85 bytes on conn_handle=6
[77384669472] [INFO] [anther] [CPU1] anther: GET /health Http11
[77672370765] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[77679475632] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[77705636382] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[77710445538] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[77715688149] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[78010893774] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[78022637319] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[78030798318] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[78034106040] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[78038729604] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[78045608685] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[78063187488] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[78066485211] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[78071072706] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[78253407210] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[78264867483] [INFO] [anther] [CPU1] anther: Worker TID=40 got first 0 bytes on conn_handle=6
[79960620498] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=26000 nodes=1470 watches=24 history=1024 journal=1024 symbols=474 drops=0
[81538979412] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[81541370262] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[81544220868] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[82521335820] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[82527373302] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[82531118901] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[82538811399] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[82539830010] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[82541537265] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[82544879010] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[82552563192] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=149
[82555411653] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[82558380135] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[82593225429] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[82599611556] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[82603840506] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:39186 on listener 1
[82987359312] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=27000 nodes=1501 watches=24 history=1024 journal=1024 symbols=474 drops=0
[84848256207] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[84852573795] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[84856582602] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[85560462372] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=28000 nodes=1523 watches=24 history=1024 journal=1024 symbols=474 drops=0
[87373892067] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=70
[87377286645] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[87382439793] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[88155243924] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=70
[88159668927] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[88164467688] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[88454792118] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=29000 nodes=1548 watches=24 history=1024 journal=1024 symbols=474 drops=0
[88955124753] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=2
[89807209734] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[89828114607] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=7
[89843839767] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 41 (user thread) assigned to CPU 1
[89847000540] [INFO] [anther] [CPU1] anther: Thread spawned TID=41 for conn_handle=7
[89871744501] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
T:5EF0 [89995323957] [INFO] [anther] [CPU1] anther: Worker thread TID=41 starting for conn_handle=7
[90192168396] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=41, our_read=42)
[90196404243] [INFO] [anther] [CPU1] anther: Worker TID=41 connected to netd OK
[91560311700] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=30000 nodes=1596 watches=24 history=1024 journal=1024 symbols=474 drops=0
[92304900699] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=149
[92316628371] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[92320240881] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[93681754980] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[93729310653] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[93740761983] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[93788948550] [INFO] [anther] [CPU1] anther: Worker TID=41 got first 85 bytes on conn_handle=7
[93791494302] [INFO] [anther] [CPU1] anther: GET /health Http11
[93953105697] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=70
[93957824796] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[93962260755] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[93973944108] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=70
[93977968128] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[93982462266] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[94039712052] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[94044316080] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[94050922086] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[94054228950] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[94060717410] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[94069875966] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=70
[94076725578] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[94085351349] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[94089671379] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[94118054613] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=149
[94122295905] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[94127363715] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[94138330077] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=70
[94142501871] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[94146757122] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[94156659564] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=25 len=70
[94160970189] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[94164936129] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[94174125144] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[94746847371] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=31000 nodes=1620 watches=24 history=1024 journal=1024 symbols=474 drops=0
[94791576858] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=26 len=70
[94795450167] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[94798118844] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[96014952528] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[96019163493] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP RST
[96023523453] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 204 bytes - TCP ACK
[96026513946] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[96031436226] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[96036669861] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:39178 on listener 1
[97477622451] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=4479 ops=1 watches=24
[97641487515] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=32000 nodes=1642 watches=24 history=1024 journal=1024 symbols=474 drops=0
[99375528252] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=27 len=70
[99380712783] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[99384774918] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[100216072011] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=33000 nodes=1671 watches=24 history=1024 journal=1024 symbols=474 drops=0
[102872972070] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=34000 nodes=1694 watches=24 history=1024 journal=1024 symbols=474 drops=0
[105340195389] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=35000 nodes=1723 watches=24 history=1024 journal=1024 symbols=474 drops=0
[105982103106] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=28 len=70
[105986020140] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[105989250939] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[107158426749] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=29 len=70
[107161897689] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[107165406381] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[107181159063] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=30 len=70
[107186815263] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[107192805060] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[108004569948] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=36000 nodes=1754 watches=24 history=1024 journal=1024 symbols=474 drops=0
[110868570318] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=37000 nodes=1780 watches=24 history=1024 journal=1024 symbols=474 drops=0
[112608893298] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=31 len=70
[112614449211] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[112619112012] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[113686270731] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=38000 nodes=1808 watches=24 history=1024 journal=1024 symbols=474 drops=0
[113752853247] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=70
[113757081636] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[113763466080] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[116555825925] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[116587550376] [INFO] [anther] [CPU1] anther: Worker TID=41 got first 0 bytes on conn_handle=7
[116602566036] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[116607565602] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=8
[116609273055] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 204 bytes - TCP ACK
[116613872562] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[116618792928] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 204 bytes
[116622059400] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 42 (user thread) assigned to CPU 1
[116624945745] [INFO] [anther] [CPU1] anther: Thread spawned TID=42 for conn_handle=8
[116627405763] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=70
[116630548287] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[116634908808] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[116643264903] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[116649301329] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[116652144411] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[116656491105] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[116662023951] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[116680949616] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
T:5EF0 [116708961666] [INFO] [anther] [CPU1] anther: Worker thread TID=42 starting for conn_handle=8
[116716968390] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[116765588115] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[116791835094] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=39000 nodes=1830 watches=24 history=1024 journal=1024 symbols=474 drops=0
[116799168123] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[116825718603] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[116843636580] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=43, our_read=44)
[116846735016] [INFO] [anther] [CPU1] anther: Worker TID=42 connected to netd OK
[116863064604] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[116910896817] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[116940475938] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[116977633212] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[117004752645] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[117030857922] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[117041371293] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[117056553669] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[117099081330] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[118626075117] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[118631503947] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[118633375476] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[118649007807] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=70
[118651769544] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[118655955759] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[118667098539] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[118669340262] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=149
[118672612146] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[118677656262] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[118691618859] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=70
[118697183781] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[118702626471] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[119239611744] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[119241870627] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[119245024932] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[119747162865] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[119761165887] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[119766997053] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:39214 on listener 1
[120579910869] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=40000 nodes=1871 watches=24 history=1024 journal=1024 symbols=474 drops=0
[124276098144] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[125284469442] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[125289843855] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[125296172529] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[125439575679] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=41000 nodes=1917 watches=24 history=1024 journal=1024 symbols=474 drops=0
[126504800862] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[126510138447] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=9
[126520273737] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 43 (user thread) assigned to CPU 1
[126521463816] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[126523719135] [INFO] [anther] [CPU1] anther: Thread spawned TID=43 for conn_handle=9
[126526274688] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[126529554624] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[126530949864] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[126533786379] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[126538220589] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[126542550519] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[126548286414] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[126560184498] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[126566553201] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=149
[126570498285] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=17
[126574790628] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[126589457313] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[126593557530] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[126598532445] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[126601756116] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[126609777921] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[126612482898] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:36166 on listener 1
T:5EF0 [126620626836] [INFO] [anther] [CPU1] anther: Worker thread TID=43 starting for conn_handle=9
[126675977043] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=45, our_read=46)
[126678359148] [INFO] [anther] [CPU1] anther: Worker TID=43 connected to netd OK
[126945623970] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[126949771476] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=17
[126954385404] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[128382175779] [IN
```
</details>
