# ✅ Scenario: Serve serial requests

> Last run: 2026-04-05 18:40:52

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the anther server is ready | ✅ | 8220ms | - [📜](./01/serial.log) - |
| 2 | When I make a GET request to "/health" | ✅ | 132ms | - [📜](./02/serial.log) - |
| 3 | Then the response status should be 200 | ✅ | 12ms | - [📜](./03/serial.log) - |
| 4 | And the response body should contain "ok" | ✅ | 0ms | - - - |
| 5 | When I make a GET request to "/graph" | ✅ | 379ms | - [📜](./05/serial.log) - |
| 6 | Then the response status should be 200 | ✅ | 0ms | - - - |
| 7 | And the response body should contain "Graph index" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11975134512] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11980908720] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11984710221] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11986791828] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11988059259] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11988940029] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11989748793] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11990367279] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11990980914] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11991659262] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11992283028] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11992939827] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11993792580] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11994495183] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11995386150] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11996125680] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11996885769] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11997532107] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11998206363] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11998874250] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11999503857] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[12000117624] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[12000736935] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[12001353276] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[12002058420] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[12002709510] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[12003332913] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[12004009908] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[12004625556] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[12005281233] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[12005942619] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[12006590574] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[12007269021] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[12007919319] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[12008540973] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[12009323436] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[12010065837] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[12010825332] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[12011766492] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[12012608916] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[12013369170] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[12014120481] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[12015502983] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[12017053158] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[12017862219] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[12018434406] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[12018987948] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[12019571190] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[12020157831] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[12020781465] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[12021345105] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[12021893400] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[12022567392] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[12023312994] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[12023895246] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[12024488652] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[12025060146] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[12025668897] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[12026298735] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[12026900688] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[12027472248] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[12028061232] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[12028633485] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[12029245173] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[12029847027] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[12030439146] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[12031141848] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[12031734957] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[12032326119] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[12032914542] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[12033675588] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[12034296780] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[12034871046] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[12035486133] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[12036060564] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[12036656082] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[12037271136] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[12037867842] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[12038441085] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[12039050991] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[12039623838] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[12040239948] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[12040815072] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[12041406003] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[12041994657] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[12042585621] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[12043156983] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[12043773885] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[12044348646] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[12044939115] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[12045527538] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[12046119261] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[12046688412] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[12047303037] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[12047876577] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[12048571227] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[12049165458] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[12049871526] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[12050450148] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[12051185454] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[12051761700] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[12052388172] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[12052960920] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[12053549277] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[12054116910] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[12054704739] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[12055283757] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[12056113773] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12317583828] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[12329189433] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12334193883] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12335495172] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12336385776] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12340671651] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12342434412] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12343544202] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12344260962] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12344976765] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12345676167] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12346690521] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12347692599] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12348611814] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12349332303] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12350041671] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12350783280] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12352076814] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12353172084] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12354205347] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12356234154] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12357223098] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12358284840] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12359998596] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12361620216] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12362433435] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12362993874] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12363773961] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12763708188] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12764828538] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12768261528] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12769176024] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12770016006] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12771609081] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12785072586] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12786464724] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12787261641] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12788992326] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12789562038] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12792287013] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12800970600] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12802861236] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12816545709] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12817148520] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12835262715] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12835971555] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12838161369] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12839483613] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12840602775] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12842993361] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12843853671] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12879356820] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62502400 ticks/sec), init_cnt=625024 for 100Hz
[12880958937] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12881799447] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12883022196] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12889048788] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12920479407] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12921620976] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12923425680] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12925554741] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12927784353] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12932479791] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12934821372] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12952005924] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12953612199] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12955313316] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12956631402] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12957472407] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12958471317] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12960112902] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12986420832] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12987985494] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12989318529] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12990670077] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12992016015] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12993139269] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12994188306] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12995258991] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[13006342173] [INFO] [kernel::root] [CPU0] Spawning Root service...
[13008081405] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[13011514890] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[13012863270] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[13017995727] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[13020606687] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[13021578042] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[13023180621] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[13024318527] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[13025089803] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[13041220764] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[13045869276] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[13047763014] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[13048786278] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13078136940] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13109230992] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13112757603] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13117334472] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13120100565] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13123471548] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13126729902] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13129629051] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13130514441] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13139805954] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13142333391] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13146387738] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13149390210] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13156555401] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13157703075] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13177881288] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13179143142] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13188988725] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13190334465] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13236419526] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13237703391] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13637975406] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[14171397570] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[14201131890] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[14248249851] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[16042238652] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=446 watches=0 history=960 journal=768 symbols=94 drops=0
[16912518150] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[17017984269] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[17019524115] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[17137456677] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[17196453153] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[17225081214] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[17225988384] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[17226835956] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[17231693919] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[17249180916] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[17266779255] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[17269745955] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[17337418560] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[17356447713] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[17359750749] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[17365734375] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[17401682199] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[17420373366] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[17423636439] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[17424781803] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[17508192207] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[17509962657] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[17629394310] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[17735830035] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[17748450357] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[17753030097] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[17755299969] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[17759404575] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[17820582714] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[17825269077] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[17826369528] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[17827338243] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[17828270757] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[17829515715] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[17830210002] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[17830960488] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[17831889669] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[17832682527] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[17833363152] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[17834006982] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[17834666520] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[17835346089] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[17836147725] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[17837116110] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[17837877981] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[17838585171] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[17839259757] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[17839956882] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[17840635824] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[17841377301] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[17842020009] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[17842713240] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[17843360205] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[17844047859] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[17844700698] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[17845348323] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[17846068185] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[17846709771] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[17847364920] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[17848026966] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[17848694754] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[17849359275] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[17850020133] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[17850660465] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[17851524966] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[17852309211] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[17853233145] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[17853998283] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[17854788600] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[17855583108] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[17856378375] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[17857893372] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[17859840570] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[17861017647] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17871049911] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[17886236676] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[17890613169] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[17900436873] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[17901605568] [CONTRACT] [kernel] [CPU0] Spawning init process...
[17904051297] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[17907198705] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[17908042449] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [17914316904] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013360 RFLAGS_BEFORE=130 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[17918308914] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[17930786412] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[17935893459] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[17937293550] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[17938544811] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[17946860052] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[17952145431] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[17955956568] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[17957197335] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[17961871158] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[17965703910] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[17967004704] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[17971587381] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[17972854284] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[17974135839] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[17975385615] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[17979553812] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[17980961757] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[17982371583] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[17984021055] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[17985535755] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[17987005047] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[17992959930] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[18050978583] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[18060383187] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[18067851483] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[18075763299] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[18081207276] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[18087657819] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[18095282436] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[18116507409] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[18128702163] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[18136954110] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[18145784877] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[18158596401] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[18166935270] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[18175304070] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[18183792033] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[18192862479] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[18202809570] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[18210717888] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[18218184138] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[18226087935] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[18234806271] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[18242236221] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[18250837869] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[18268403703] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[18276292320] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[18284079627] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[18292535382] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[18300209829] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[18307646940] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[18315757185] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[18321598449] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[18327177165] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[18335046510] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[18342757356] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[18351599871] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[18359509872] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[18366533361] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[18373481742] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[18380893641] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[18388501164] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[18396178251] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[18401061261] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[18426853731] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[18588934914] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[18596590815] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[18598084296] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18600940314] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18606202692] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18608298885] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18617427246] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[18623633193] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18624699390] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18626619528] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078896 RFLAGS_BEFORE=130 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[18632098749] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[18633373605] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[18634415514] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18636888039] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18641812299] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18644048610] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18652241751] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[18656530530] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[18658544949] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[18660142611] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[18662298435] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144432 RFLAGS_BEFORE=130 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[18668843919] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[18670668357] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[18672866223] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18677377158] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:41:24 = 1775439684 unix_secs
[18679212156] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775439684, mono_ns=9339372702, offset=1775439674660627298ns
[18681076524] [INFO] [rtc_cmos] [CPU1] System clock anchored
[18694857324] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[18731628366] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[18744494175] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[18745877205] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18748678542] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18758014011] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18761777298] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[18773898363] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[18779513445] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[18784709592] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18787112454] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210544 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[18794242005] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[18800872662] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[18803489727] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[18804672612] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18807222027] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18817987188] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18821468391] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18832399014] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[18836533815] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18838055610] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18840195660] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277296 RFLAGS_BEFORE=130 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[18845614755] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[18847550403] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[18854045892] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[18855317778] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[18857041038] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[18859214550] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[18863882466] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[18865682286] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[18879045009] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[18881209182] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[19424718258] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19430838999] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[19434093822] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[19435934166] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[19440530373] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[19442334021] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[19444493640] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[19446139647] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[19447309431] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[19451941872] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[19453723971] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[19455235965] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[19456742943] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[19458127623] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[19459481712] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[19461124848] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[19467666768] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[19469286639] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19471473846] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19478784963] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19481165187] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19488203460] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[19491363309] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fab68
[19492955922] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[19495375812] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352768 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[19519143765] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[19532900442] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[19551490563] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[19564517016] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[19566954132] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[19572796353] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[19582449777] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[19584173466] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[19598770851] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[19599952713] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[19601393922] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[19603351581] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[19605459753] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[19606979040] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[19608297258] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[19609654350] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[19611078993] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[19621781289] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[19626473196] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[19631073792] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[19634420553] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[19635829653] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19638724809] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19644985635] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[19647799116] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19659818013] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[19664657265] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[19667913573] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[19669291191] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19672758699] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19680357114] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19682753046] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19694367495] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[19698973437] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[19700423721] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[19702582152] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369501360 RFLAGS_BEFORE=130 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[19709954748] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[19710966957] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19712580888] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[19713551649] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19714685892] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[19720783104] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[19723050666] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[19724111484] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19725497517] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[19728427191] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[19729533516] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19730660466] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[19732782069] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[19735679436] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[19737654816] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[19739819319] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[19742842548] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[19747339623] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[19749829539] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[19751821485] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[19754704068] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[19756956285] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369435824 RFLAGS_BEFORE=130 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[19764847344] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[20114416872] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
[20115872799] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[20117732910] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583280 RFLAGS_BEFORE=130 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[20121409803] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[20123454120] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[20124393267] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[20126660928] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[20127704751] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20129707092] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[20130663366] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[20131581822] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20133448170] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20140500171] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[20142970320] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[20147069382] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[20154900612] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[20160054090] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[20163830973] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[20166315873] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[20167432956] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20170295013] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20201626533] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20208115983] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[20249355192] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[20831153541] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[20832589338] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[20835014244] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[20837336421] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[21136690443] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[21140040966] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb01095c8
[21141837354] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21144049311] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716896 RFLAGS_BEFORE=134 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[21148101018] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[21150277071] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[21151253211] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650336 RFLAGS_BEFORE=134 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[21155047452] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[21157211031] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[21158677749] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[21162001575] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[21163801956] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[21167378991] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[21168920157] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[21170223723] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[21171949887] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[21181149891] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[21182920968] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[21184728444] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[21185565885] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21187639275] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21257366889] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[21282066498] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[21290279076] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[21293022201] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[21294283263] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21296007645] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369782832 RFLAGS_BEFORE=134 CR3_BEFORE=79892480 fs_base=0 gs_base=18446744071564586576
[21299603952] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[21300665694] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21302479572] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[21303437397] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21305423535] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[21310409901] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21312557442] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21332508450] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[21336124920] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[21343891107] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[21346933509] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[21349047918] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[21349776327] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21351282282] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21357795558] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[21360877692] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21368666418] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[21371518542] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010aff8
[21372884445] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21374629980] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915216 RFLAGS_BEFORE=134 CR3_BEFORE=80826368 fs_base=0 gs_base=18446744071564586640
[21378907242] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[21379761183] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21381546318] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21382495926] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[21384716001] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[21389744343] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21393433116] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[21401264049] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[21403792509] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010aff8
[21405215832] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21406933845] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981712 RFLAGS_BEFORE=130 CR3_BEFORE=80953344 fs_base=0 gs_base=18446744071564586576
[21411526620] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[21412267404] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21413810022] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21423859875] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[21427525581] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[21435252069] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[21438074691] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[21440374461] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[21441080925] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21442597836] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21469601472] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[21474558237] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[21482244234] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[21485694780] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
[21486664353] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cc18
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21489361047] [INFO] [kernel:00, 00, 00, 00]
[21492473475] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21494798688] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[21511056798] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[21514584300] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[21523033224] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[21525622437] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[21528366090] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[21529868547] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21531988368] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370202320 RFLAGS_BEFORE=130 CR3_BEFORE=81530880 fs_base=0 gs_base=18446744071564586576
[21536590812] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[21537831348] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21539878305] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[21541009314] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21542477385] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[21548185461] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21549837606] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21551423256] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21553060617] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21554553273] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21556401273] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21558143475] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=226 subj_lo=0
[21560198880] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[21582466059] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21583731312] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21585045768] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21586516248] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[21587844564] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[21598808682] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[21608853915] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[21611615223] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[21614580801] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21616338282] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849136 RFLAGS_BEFORE=134 CR3_BEFORE=80564224 fs_base=0 gs_base=18446744071564586608
[21622835157] [INFO] [nectar] [CPU2] NECTAR: Started.
[21630041532] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21631658037] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010aff8
[21633110565] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21635544381] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21637398552] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047968 RFLAGS_BEFORE=130 CR3_BEFORE=81100800 fs_base=0 gs_base=18446744071564586608
[21642275490] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[21645738609] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21647235192] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21648795135] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21650529054] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[21651507570] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[21654147966] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f10d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21656961315] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370267856 RFLAGS_BEFORE=130 CR3_BEFORE=81739776 fs_base=0 gs_base=18446744071564586608
[21664397568] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[21667209630] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[21668624406] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
[21670734921] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[21673675848] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21674642517] [INFO] [fontd] [CPU3] FONTD: Service ready
[21676317861] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21681252318] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[21686425860] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[21702701361] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[21707273280] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[21709089402] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f4000
[21710929020] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f4000
[21712083063] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21713295978] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21714565026] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21715968780] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=238 subj_lo=0
[21717967392] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f7000
[21729562866] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1250 backend=VirtIO-GPU
[21732589692] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[21733771257] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21736694991] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21740393070] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21741856092] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21743354523] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21744713958] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=240 subj_lo=0
[21758566137] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21760177263] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21761787399] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21763763802] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[21773600838] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21775174674] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21776719305] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21778636011] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[21795424266] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21796920750] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21798600549] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21800478711] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[21818591454] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21820911750] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21863041134] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[21897147360] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[21899164155] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[21911430354] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[21931044498] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[21941267931] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[21944595090] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f0fe8
[21946151304] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e2
[21948298218] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370360128 RFLAGS_BEFORE=130 CR3_BEFORE=82395136 fs_base=0 gs_base=18446744071564586640
[21952847103] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[21953971479] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21956511390] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21957782451] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[21963421722] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[21965658363] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21978148500] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[21982026330] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f0fe8
[21983501727] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[21985318938] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370425664 RFLAGS_BEFORE=130 CR3_BEFORE=83443712 fs_base=0 gs_base=18446744071564586576
[21989129349] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[21990906366] [INFO] [echo] [CPU1] echo: starting up
[21993283356] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[21995103603] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276783104)
[21996993216] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[21999606651] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f8000 phys=0x4fef000
[22002550845] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[22004170716] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[22006251993] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[22009120782] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[22011260502] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[22019950986] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[22022821887] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[22024300056] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[22035524049] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[22039240278] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[22041726927] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[22047171102] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[22048323363] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[22051571751] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[22054530366] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[22056747570] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[22058947449] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[22060947051] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[22063361199] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[22064477622] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[22065963216] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[22067142801] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[22068603150] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[22069779864] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[22075409103] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[22081561491] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[22086027480] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[22093483005] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[22116696987] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22133572593] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[22138174113] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[22141318881] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[22143542520] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[22145971881] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[22147456881] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22149921453] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22155327117] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[22156632465] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22159235901] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[22166428086] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22168909026] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[22171844409] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[22174079796] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[22176391050] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[22178702865] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[22179963399] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22182452193] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22188880626] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22190048562] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[22191268110] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22192896858] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22197216921] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[22205791641] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[22209016896] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010e3a8
[22210630761] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22212685176] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[22214295972] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370704192 RFLAGS_BEFORE=130 CR3_BEFORE=84336640 fs_base=0 gs_base=18446744071564586640
[22219043451] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[22220522214] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22223103177] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22225937877] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[22227041397] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[22228385223] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[22238760060] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[22242834372] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[22244822886] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[22249750644] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f23b8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22251738168] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370773824 RFLAGS_BEFORE=130 CR3_BEFORE=84484096 fs_base=0 gs_base=18446744071564586576
[22259059746] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[22277797872] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[22280812719] [INFO] [bloom] [CPU3] bloom: creating surface...
[22282481925] [INFO] [bloom] [CPU3] bloom: surface created!
[22284047808] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[22312831926] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[22322589729] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22333573086] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[22335158538] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[22350839808] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1261
[22352422026] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[22354182840] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[22371552192] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[22376281554] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[22378166052] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[22380056556] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [22389656157] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[22395279588] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 3)
[22401924864] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[22415592276] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[22427527386] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
T:07D0 T:0640 T:F0B0 [22450096614] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1261
[22478206905] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[22493346249] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[22496758878] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[22506190608] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22507547271] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22508785299] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[22512840867] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
T:1220 [22527941865] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[22530627702] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[22543376757] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[22546865814] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[22563847119] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[22567409832] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[22572659406] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[22578677286] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=276
[22581673422] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[22582904685] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[22584678864] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22586280024] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22587779181] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22589583654] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=276 subj_lo=0
[22606480347] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1272)
[22607951619] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[22622013909] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=243
[22623342357] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[22624426341] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22625728356] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22627084491] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22628652321] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[22643377911] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1275)
[22644585645] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[22659760233] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=279
[22661591370] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[22662778083] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22663905825] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22665120060] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22666511703] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=279 subj_lo=0
[22681850961] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1278)
[22683912999] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[22693348854] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([247, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[22741894131] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[22743490011] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[22750219371] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[22772164701] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=31, our_read=32)
[22775303595] [INFO] [anther] [CPU1] anther: Connected to network stack
[22784523663] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[22785330348] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[22789659453] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[22832156622] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[22836417681] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[22844218089] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[22850613291] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[22856248470] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[22867464081] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[22869722007] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[22873504929] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[22932312546] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[22935357621] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[22937829816] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[22940472357] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[23012911548] [INFO] [netd] [CPU3] NETD: DHCP configured ��� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[23017143468] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[23023716639] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[23026928661] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=1
[23029865397] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[23033729136] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=2
[23042327121] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=2)
[23084751228] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[23087593881] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[23095469133] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[23149337607] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[23225252424] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[23254382679] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[23274427143] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[23276574354] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[23279640615] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23398359072] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[23430011649] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=659 watches=13 history=1024 journal=1024 symbols=306 drops=0
[23649989418] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[23815955724] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[24042901839] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[24500395062] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[24531353121] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[24539783103] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[24547218630] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[24559892973] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=74
[24562307682] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=25
[24565335333] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[24569154522] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[24575015058] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[24582823947] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[24597994146] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[24612969414] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[24624009795] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[24625913202] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[24630698532] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=149
[24633523233] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=25
[24636555405] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[24797805153] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[25044141078] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[25252994943] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[25360835049] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[25369630869] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[25372116033] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[25373777220] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[25375674621] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[25377963864] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=226 subj_lo=0
[25388766117] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[25390421793] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[25392067239] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[25393504521] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=337 pred=0 subj_lo=0
[25476610566] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[25652302269] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=714 watches=15 history=1024 journal=1024 symbols=339 drops=0
[25701576483] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[25937220342] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[26174376393] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[26214691767] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26220787791] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[26232873777] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=Established
[26236676631] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:39792 on listener 2
[26246544225] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=3
[26260902096] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 37 (user thread) assigned to CPU 1
T:5EE0 [26284682061] [INFO] [anther] [CPU1] anther: Worker thread TID=37 starting for conn_handle=3
[26290864017] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[26293045218] [INFO] [anther] [CPU1] anther: Thread spawned TID=37 for conn_handle=3
[26302948221] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[26325114849] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[26329017132] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[26343966990] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=33, our_read=34)
[26346805485] [INFO] [anther] [CPU1] anther: Worker TID=37 connected to netd OK
[26371412133] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[26378169378] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[26405498427] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 85 bytes on conn_handle=3
[26417602827] [INFO] [anther] [CPU1] anther: GET /health Http11
[26435968680] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[26442281547] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[26463542061] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[26466082929] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[26470825854] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26481140862] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26534288583] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[26541949071] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[26572868124] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[26575365069] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[26578491423] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26580742155] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26586404955] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[26588861574] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[26594246613] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26608278048] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26611886070] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[26619519663] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[26643776874] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[26646455781] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[26650243785] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26665084776] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26668383951] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[26678113242] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26679488154] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[26680667508] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26682349650] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26683846167] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[26685227877] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=339 pred=0 subj_lo=0
[26687370138] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[26691987696] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26698903044] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26704778463] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=124
[26707705761] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 114 byte frame (118 encoded) to netd rx_port=25
[26711499111] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (118 bytes sent)
[26728964394] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 114 bytes
[26738783016] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 0 bytes on conn_handle=3
[26748479604] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[26760939249] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[26764242285] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[26781478152] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=Established
[26784230352] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:39798 on listener 2
[26810914119] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=4
[26820083334] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 38 (user thread) assigned to CPU 1
[26822208138] [INFO] [anther] [CPU1] anther: Thread spawned TID=38 for conn_handle=4
T:5EE0 [26840454498] [INFO] [anther] [CPU1] anther: Worker thread TID=38 starting for conn_handle=4
[26843038926] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[26885154549] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26927330859] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=35, our_read=36)
[26929230240] [INFO] [anther] [CPU1] anther: Worker TID=38 connected to netd OK
[26947891674] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[26955772569] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[26974986621] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 60 bytes on conn_handle=4
[26976813765] [INFO] [anther] [CPU1] anther: GET /health Http11
[26999038704] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[27006452649] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[27013405980] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[27018367497] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[27021914370] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27027833184] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[27049810788] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27062387979] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[27070505088] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[27087163686] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[27088501110] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[27089852559] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[27093333696] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27105222903] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[27107439150] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[27111277875] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27116665620] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[27120141939] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[27123496818] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[27127933140] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[27130207368] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[27132339630] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[27133822947] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27137608410] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[27139350909] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[27166593069] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[27170779284] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[27177284376] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[27178945431] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[27181005093] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[27183531771] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=340 pred=0 subj_lo=0
[27191936574] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[27208861746] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[27214751586] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[27216808278] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[27219353337] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27220774647] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[27222754779] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=123
[27224625747] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 113 byte frame (117 encoded) to netd rx_port=25
[27226833810] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[27228453747] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (117 bytes sent)
[27229932444] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[27231641184] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[27233405793] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=341 pred=0 subj_lo=0
[27242602365] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27248311101] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 113 bytes
[27252414057] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 0 bytes on conn_handle=4
[27273419118] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[27279512172] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[27283028685] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[27392941191] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=0e57f834b3afbcb1)
[27417485931] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27482918991] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[27500270655] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12222000
[27502043118] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[27503996850] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[27571959657] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[27583007397] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[27587568294] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[27590305842] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[27592768005] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[27597351969] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[27611329482] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=2 socket state=Established
[27613859031] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:39806 on listener 2
[27620148501] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=5
[27623360952] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[27625129488] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[27629856606] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 39 (user thread) assigned to CPU 1
[27631669890] [INFO] [anther] [CPU1] anther: Thread spawned TID=39 for conn_handle=5
[27636693513] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
T:5EE0 [27680143524] [INFO] [anther] [CPU1] anther: Worker thread TID=39 starting for conn_handle=5
[27688594956] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x12223000
[27690382797] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[27760385631] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=37, our_read=38)
[27763089750] [INFO] [anther] [CPU1] anther: Worker TID=39 connected to netd OK
[27875661066] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[28016460318] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[28017781011] [INFO] [anther] [CPU1] anther: Worker TID=39 got first 59 bytes on conn_handle=5
[28019221329] [INFO] [anther] [CPU1] anther: GET /graph Http11
[28024194990] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[28038144783] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 209 bytes - TCP ACK
[28044846489] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 209 bytes
[28050162096] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=70
[28052230041] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[28055360850] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[28346762070] [INFO] [n
```
</details>
