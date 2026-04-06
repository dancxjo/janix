# ✅ Scenario: Server starts up

> Last run: 2026-04-05 17:56:38

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 3646ms | - [📜](./01/serial.log) - |
| 2 | Then I should see a message in the serial output that says "anther: Listening on port 80" | ✅ | 2328ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11357741208] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11363101596] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11366634015] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11368616391] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11369782974] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11370422349] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11371065123] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11371634241] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11372207979] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11372804190] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11373381360] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11373992850] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11374677171] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11375328624] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11376011394] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11376612324] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11377298262] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11377896948] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11378504742] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11379093957] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11379666870] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11380270803] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11380859028] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11381446065] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11382085275] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11382681552] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11383357920] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11384097978] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11384692770] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11385293007] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11385900108] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11386510542] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11387174139] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11387784177] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11388369630] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11389057317] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11389754673] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11390474700] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11391167436] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11391881754] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11392577526] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11393278182] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11394565182] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11395985601] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11396764368] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11397302730] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11397915738] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11398428129] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11398967976] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11399481093] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11400047175] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11400561315] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11401066776] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11401596393] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11402131521] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11402686713] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11403239298] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11403795315] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11404331235] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11404887483] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11405425614] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11405983413] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11406532038] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11407086405] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11407621434] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11408172864] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11408707431] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11409260247] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11409806826] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11410364130] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11410952520] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11411516424] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11412055710] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11412611397] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11413165533] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11413722177] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11414259186] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11414814774] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11415352707] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11415908031] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11416460550] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11417016534] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11417554863] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11418111474] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11418647460] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11419202454] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11419749825] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11420306436] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11420843148] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11421398637] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11421934194] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11422518789] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11423074212] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11423630493] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11424167205] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11424722727] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11425256601] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11425811232] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11426365797] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11426922276] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11427459186] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11428014081] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11428552146] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11429107998] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11429662893] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11430222804] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11430760935] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11431527459] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11666058723] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[11677429137] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11682293370] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11683553772] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11684412300] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11688813081] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11690511921] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11691593067] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11692272603] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11692986459] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11693673189] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11694651771] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11695605570] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11696289000] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11696977446] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11697647676] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11698346286] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11699489604] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11700520821] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11701208970] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11702873226] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11703827388] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11704793463] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11706502071] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11708077524] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11708832003] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11709371421] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11710140981] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12075978849] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12076983897] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12080319966] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12081181332] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12081951552] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12083435958] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12095680047] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12096938139] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12097664601] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12099453135] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12100001166] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12102416535] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12110058279] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12111746031] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12124849341] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12125399847] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12141109332] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12141662973] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12143671815] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12144934461] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12145976502] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12148200405] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12149030718] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12183860865] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62340600 ticks/sec), init_cnt=623406 for 100Hz
[12185286630] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12186081303] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12187230396] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12192620187] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12222738627] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12223720113] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12225669654] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12227074893] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12228165048] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12230896755] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12232120362] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12252367875] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12253183569] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12254091267] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12254785917] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12255751431] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12256997610] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12257668764] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12282819648] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12284329860] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12285045399] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12286172283] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12286951809] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12287701008] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12288509112] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12289171290] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12295600581] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12296537913] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12298336050] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12299173920] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12302577639] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12304031520] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12304711551] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12305916216] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12306814245] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12307620765] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12319112454] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12321919302] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12323044932] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12323866995] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12345938946] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12367361589] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12369892029] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12373488666] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12375015213] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12377142690] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12379284687] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12381197169] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12382242114] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12383278512] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12389679588] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12392081361] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12394798350] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12397483989] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12400116135] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12403082802] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12404207475] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12415904028] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12416750214] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12423702456] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12424545903] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12452647779] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12453492150] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[12795831048] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13283260386] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13317800727] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=500 journal=424 symbols=52 drops=0
[13348958271] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[14606470428] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=449 watches=0 history=963 journal=772 symbols=97 drops=0
[15300336810] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15390386814] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15391649196] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[15491246562] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[15548593599] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[15574420950] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[15575190213] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[15575813748] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[15579249147] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[15594097860] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[15610895784] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[15613219842] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[15664000077] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[15681662172] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[15682682763] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[15686417241] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[15711616074] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[15729208341] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[15732873519] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[15736917966] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[15799992219] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[15801962451] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[15885760176] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[15947529279] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[15960823230] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[15965595888] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[15968168106] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[16011310194] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[16036193712] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[16041416754] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[16042367880] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[16043170209] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[16044031905] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[16044710352] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[16045407378] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[16046048799] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[16046640258] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[16047246930] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[16047894390] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[16048505385] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[16049129976] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[16049773014] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[16050444993] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[16051162545] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[16051789446] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[16052428854] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[16053039915] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[16053674274] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[16054298799] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[16054890720] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[16055485743] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[16056089709] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[16056689352] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[16057350309] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[16057968168] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[16058579790] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[16059245532] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[16059850455] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[16060466961] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[16061415711] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[16062249456] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[16062876357] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[16063554045] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[16064403960] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[16065196059] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[16065921300] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[16066674228] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[16067406036] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[16068154740] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[16068884997] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[16069817478] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[16071352737] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[16073019369] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[16073808333] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16081664643] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[16095687234] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[16099964298] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[16109036262] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[16109739360] [CONTRACT] [kernel] [CPU0] Spawning init process...
[16111748334] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[16114238844] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[16115286462] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [16120665792] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[16121328795] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013040 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[16136907204] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[16141552449] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[16142746521] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[16143806646] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[16152798981] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[16157605497] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[16161729936] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[16163127321] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[16167140517] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[16171542882] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[16173550866] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[16177696689] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[16179033816] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[16180811295] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[16182462285] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16187656617] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16189333710] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16191203127] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16193514084] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16200625287] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16203458799] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16209335505] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[16251303684] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[16258619256] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[16263598461] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[16268787843] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[16272252480] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[16275836841] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[16280466312] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[16285534881] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[16290205074] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[16295220348] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[16300141374] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[16305433023] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[16310235711] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[16314795123] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[16319288040] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[16323827025] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[16329020928] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[16333727487] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[16338237333] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[16342501461] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[16346730180] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[16353621702] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[16358726868] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[16364653272] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[16370308878] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[16374831198] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[16381270059] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[16386312888] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[16390803198] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[16395675483] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[16399032771] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[16402511301] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[16409233764] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[16413662727] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[16418079414] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[16422561540] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[16427676870] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[16432617003] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[16438114869] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[16443246963] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[16449139608] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[16453423866] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[16472157966] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[16601915946] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[16609245114] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[16610177364] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16612139280] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16616762712] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[16618183230] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16627036107] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[16632333432] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16633403721] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16634844699] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078576 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[16639644417] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[16640515551] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[16641274386] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16642807896] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16646790864] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[16648402221] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16655216787] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[16658492697] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[16660196850] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[16661192163] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[16662516882] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144112 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[16668330888] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[16670284488] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[16672424868] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[16677101694] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:56:55 = 1775437015 unix_secs
[16678670778] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775437015, mono_ns=8339117242, offset=1775437006660882758ns
[16680330810] [INFO] [rtc_cmos] [CPU1] System clock anchored
[16691104980] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[16725041190] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[16734752232] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[16735658313] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16737383058] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16743088890] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[16745371368] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[16752428517] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[16755756138] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[16759281858] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[16760863680] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[16761577272] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16763328879] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16770099357] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[16772338605] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[16779069912] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[16781755320] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[16783227813] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00faa60
[16784368227] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16785903783] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369276752 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[16790233383] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
[16792380660] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16793803950] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210800 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[16799152095] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[16800716856] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[16802328345] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[16803724674] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[16807653918] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[16810277121] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[16811759382] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[16823128113] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[16824944796] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[17319517083] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[17321323371] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17324240175] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17334644712] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17338188615] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17347601139] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[17350313838] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[17351899752] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[17353653999] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352656 RFLAGS_BEFORE=130 CR3_BEFORE=59662336 fs_base=0 gs_base=18446744071564586576
[17360144076] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[17361218985] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17370786543] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 0)
[17372034966] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[17373261114] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 1)
[17375548212] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=1
[17381873916] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[17383545597] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[17385717657] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[17387094846] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[17388014589] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[17393582778] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[17394724710] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[17395433649] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[17396874660] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[17397806184] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[17398977024] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[17399752359] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[17400467139] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[17401486773] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[17403204192] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[17411948961] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[17413417032] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[17431994679] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[17434401105] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[17435717772] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[17436847824] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[17438507427] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[17439267747] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17440830693] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17450525565] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[17452245327] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17459114112] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[17461582050] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[17463816216] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[17465041044] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17466760641] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17471391069] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17472760899] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17480204214] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[17482962123] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0105668
[17484847149] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[17486165136] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500112 RFLAGS_BEFORE=130 CR3_BEFORE=68235264 fs_base=0 gs_base=18446744071564586640
[17491610895] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[17492231328] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[17492988348] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[17493699861] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17495402991] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17497295409] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[17498613594] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[17500089552] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[17500809381] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17502094698] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[17503391433] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17504225838] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[17505669885] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[17507697108] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[17509102182] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[17510439903] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[17511724626] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[17514475011] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[17516054391] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[17517404916] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[17520327726] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[17521943604] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369433872 RFLAGS_BEFORE=130 CR3_BEFORE=59830272 fs_base=0 gs_base=18446744071564586608
[17526914493] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[17900969889] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0105668
[17902011897] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1237 port=2 model='                                        ' rpc_port=5
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[17905414725] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369582896 RFLAGS_BEFORE=134 CR3_BEFORE=68349952 fs_base=0 gs_base=18446744071564586576
[17910512631] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[17912864805] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[17916107286] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[17924563833] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[17931789414] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[17933089449] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[17934340578] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[17935562997] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[17936708889] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[17937526728] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[17938538310] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[17939725089] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[17946855531] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[17948894601] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[17950525890] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[17955086226] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17957446881] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[17958512385] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17960325603] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17967261543] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[17969931540] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[17977376703] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[17980422009] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[17982275355] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[17983852425] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[17985012771] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17986949739] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17999135022] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[18007525107] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18011791512] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[18032023548] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[18036577779] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a670
[18037858740] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18039328824] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716800 RFLAGS_BEFORE=130 CR3_BEFORE=69054464 fs_base=0 gs_base=18446744071564586640
[18045340731] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[18046487448] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0105668
[18047460354] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[18048948192] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650480 RFLAGS_BEFORE=134 CR3_BEFORE=68755456 fs_base=0 gs_base=18446744071564586608
[18054609606] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[18056241654] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[18063680943] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[18066131688] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[18070318827] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[18071964207] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[18073386804] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[18075225696] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[18088050519] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[18094747143] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[18096799116] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[18098692854] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[18100311471] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[18102845046] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[18104318826] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[18106203060] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4256000
[18107939718] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[18110472732] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[18112737918] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[18114324393] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[18124832352] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[18133200030] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[18143005617] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[18145815732] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[18148157775] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[18149993763] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[18151786125] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[18153390354] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[18155140245] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[18156911487] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[18158235711] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[18164265999] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=15, read=16)
[18170253156] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=17, read=18)
[18173343540] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[18175347729] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[18177594336] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[18178643373] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18180513912] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18205303743] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18245776164] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[18269806170] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[18277865529] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[18280657263] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0105668
[18282541299] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18284182653] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369914560 RFLAGS_BEFORE=134 CR3_BEFORE=69668864 fs_base=0 gs_base=18446744071564586576
[18287003394] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[18287904789] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18289704543] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18290383848] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[18292875876] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[18296819409] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18298230456] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18314562519] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18318018081] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[18325053186] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[18327957186] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[18329957712] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[18330760206] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18332371728] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18337655556] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18339858702] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18346913079] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[18349407054] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[18351771240] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18353403288] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370046944 RFLAGS_BEFORE=130 CR3_BEFORE=70602752 fs_base=0 gs_base=18446744071564586640
[18358224258] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18358928313] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18360488685] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18361217358] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[18362621244] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[18368611140] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18372469830] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18380306010] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[18383496879] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
[18384302343] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18386877762] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113344 RFLAGS_BEFORE=130 CR3_BEFORE=70729728 fs_base=0 gs_base=18446744071564586576
[18389893830] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[18390582606] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18392022132] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18401003148] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18404003508] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[18410893050] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[18414098670] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[18416186316] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[18416900040] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18418279440] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18443332578] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18448119525] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[18455324316] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[18457907886] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cc18
[18458862576] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18460132713] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370245664 RFLAGS_BEFORE=130 CR3_BEFORE=71032832 fs_base=0 gs_base=18446744071564586640
[18462969261] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[18463766376] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18465015195] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[18466187289] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18482101968] [INFO] [fontd] [CPU3] FONTD: Service node created, req=19, resp=22
[18483237366] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[18485955048] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18487097046] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18487914093] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18489363255] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18491055891] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[18496311372] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[18498782412] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[18500516397] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[18501482241] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18503268135] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18505265295] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1108
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18507734619] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370330880 RFLAGS_BEFORE=134 CR3_BEFORE=71307264 fs_base=0 gs_base=18446744071564586576
[18539068548] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[18547201827] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18548810544] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18549915054] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18551132358] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[18551950098] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=229 pred=0 subj_lo=0
[18555314316] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18559290288] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[18561691632] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[18565774161] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18567936684] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0105668
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18569670273] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369980864 RFLAGS_BEFORE=134 CR3_BEFORE=70340608 fs_base=0 gs_base=18446744071564586608
[18574767717] [INFO] [nectar] [CPU2] NECTAR: Started.
[18576074748] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[18577173681] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[18578653137] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
[18579670956] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18581573670] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[18582448962] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370179872 RFLAGS_BEFORE=130 CR3_BEFORE=70877184 fs_base=0 gs_base=18446744071564586608
[18587579835] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[18589359162] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010d548
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18591043845] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370397984 RFLAGS_BEFORE=130 CR3_BEFORE=71516160 fs_base=0 gs_base=18446744071564586608
[18595933554] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[18597443799] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[18598758618] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[18603555696] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18605160189] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18606956775] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18608907735] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[18626174754] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
[18628751460] [INFO] [fontd] [CPU3] FONTD: Service ready
[18662752911] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[18666364332] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=15, RX port=18
[18677841237] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18679982673] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[18682469355] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[18691454595] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18692851881] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18694365393] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18695869632] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[18709080819] [INFO] [netd] [CPU3] NETD: Driver TX port=15, RX port=18, link_up=true, mtu=1500
[18710949972] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[18712286670] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[18716456517] [INFO] [netd] [CPU3] NETD: Created socket API port (write=23, read=24)
[18718000356] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18719226240] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18723574089] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1250 backend=VirtIO-GPU
[18725965797] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[18726823830] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18728688099] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18758343384] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18759396645] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18760473633] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18761607183] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=246 subj_lo=0
[18772358682] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18773404584] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18774524802] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18776256048] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=248 subj_lo=0
[18780624720] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([229, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[18783473313] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18784519479] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18785632866] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18786740577] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=249 subj_lo=0
[18794292693] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18795408753] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18796821747] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18798064989] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=251 subj_lo=0
[18802504545] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[18803983605] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[18806246580] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18807233709] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18808343796] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18809456127] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[18810313038] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=253 subj_lo=0
[18815813313] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18816822288] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18817927095] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18818999958] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=254 pred=0 subj_lo=0
[18837299283] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[18843359238] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[18859348365] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=25, resp=28
[18860539665] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[18873523482] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[18888699720] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[18897024729] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[18899899821] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[18901784154] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[18902643441] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18904839525] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18907408311] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e2
[18908996007] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370539568 RFLAGS_BEFORE=134 CR3_BEFORE=72167424 fs_base=0 gs_base=18446744071564586640
[18911440878] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18913349136] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18914972604] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[18922177626] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[18923577123] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=29, our_read=30)
[18925583226] [INFO] [anther] [CPU1] anther: Connected to network stack
[18926456274] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[18928275663] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[18933174183] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[18938499063] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[18942792957] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[18963641565] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[18966550152] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[18969223020] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[18970257636] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[18971942484] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370629680 RFLAGS_BEFORE=134 CR3_BEFORE=73748480 fs_base=0 gs_base=18446744071564586576
[18975166617] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[18976511037] [INFO] [echo] [CPU1] echo: starting up
[18977293797] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[18978706098] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[18979750977] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[18981200634] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[18983546472] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[18986543499] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[18987341340] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[18988391961] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[18989332098] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[18998727033] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19000979316] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[19002406566] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[19003263411] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[19004746332] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[19005596280] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[19006714551] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19008229713] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19013181891] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19015176444] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[19022940948] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[19025635332] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[19027688394] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[19029887646] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[19031994399] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19032870417] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19034532858] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19036175070] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[19038095307] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[19041946440] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19042965381] [INFO] [netd] [CPU3] NETD: DHCP configured ��� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[19045879479] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19047335637] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[19052729553] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[19055238840] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[19056095421] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[19056901875] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[19058605467] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[19059457098] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19060794456] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[19061702088] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19064161413] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[19065456564] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[19066775904] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[19071155466] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[19072735770] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f80f8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19074130218] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[19075501797] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370760752 RFLAGS_BEFORE=134 CR3_BEFORE=74780672 fs_base=0 gs_base=18446744071564586640
[19079004450] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[19080446880] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[19084106910] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f80f8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19086722391] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370826288 RFLAGS_BEFORE=134 CR3_BEFORE=74928128 fs_base=0 gs_base=18446744071564586576
[190932
```
</details>
