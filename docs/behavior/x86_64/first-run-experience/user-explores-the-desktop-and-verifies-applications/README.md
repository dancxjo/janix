# ❌ Scenario: User explores the desktop and verifies applications

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 5062ms | - - - |
| 2 | When I wait for 5 seconds | ✅ | 5001ms | - [📜](./02/serial.log) - |
| 3 | Then I should see the desktop wallpaper | ❌ | 1014ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11285723559] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11291210172] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11294751930] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11296683981] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11297868879] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11298497199] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11299145286] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11299720047] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11300301276] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11300906199] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11301538446] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11302152081] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11302873659] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11303536266] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11304220587] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11304847026] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11305466403] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11306062350] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11306670276] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11307411423] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11308001892] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11308576389] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11309157420] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11309737956] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11310362217] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11310971133] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11311558863] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11312195334] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11312774550] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11313366603] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11313987696] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11314621824] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11315339178] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11315954331] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11316540411] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11317227471] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11317952283] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11318671386] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11319364254] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11320086690] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11320807212] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11321520474] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11322812952] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11324180109] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11324925480] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11325467505] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11325970260] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11326483377] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11327022960] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11327556636] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11328066354] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11328580560] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11329090311] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11329622337] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11330160336] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11330732391] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11331273294] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11331831819] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11332391499] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11332951476] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11333489673] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11334073839] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11334614313] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11335174554] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11335716975] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11336275203] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11336815512] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11337386808] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11337926721] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11338486071] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11339026710] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11339585103] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11340125610] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11340697599] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11341249656] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11341842369] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11342438217] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11342998953] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11343538239] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11344116036] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11344654068] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11345210580] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11345749008] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11346308424] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11346849987] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11347424583] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11347965156] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11348524110] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11349064617] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11349624858] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11350164177] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11350733163] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11351269974] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11351826585] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11352364221] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11352921162] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11353457874] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11354028972] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11354568258] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11355124869] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11355661845] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11356217763] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11356756554] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11357324088] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11357861130] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11358417840] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11358957456] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11359784436] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11598791424] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[11609378880] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11614227141] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11615479095] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11616438537] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11620825029] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11622659532] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11623721406] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11624425824] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11625121761] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11625829842] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11626817004] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11627786841] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11628464628] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11629131987] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11629894584] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11630596395] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11631833268] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11632908375] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11633612892] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11635377006] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11636312622] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11637377829] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11639095314] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11640830982] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11641616415] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11642153226] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11642911236] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12012227865] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12013719300] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12018486216] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12019480671] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12020249901] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12021756087] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12035264703] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12036615723] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12037363437] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12039048615] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12039583974] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12042390393] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12049686825] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12051367812] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12065917413] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12066709446] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12084670191] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12085332204] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12087359856] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12088615539] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12089732061] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12092001999] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12092831223] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12127670313] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62369600 ticks/sec), init_cnt=623696 for 100Hz
[12129073770] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12129890223] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12131010342] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12136536159] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12168296910] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12169755246] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12171944697] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12173413131] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12174687030] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12177584199] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12178981914] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12202450227] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12204057756] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12205423197] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12206560707] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12207843582] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12208511403] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12209224038] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12235599750] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12236702049] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12237577275] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12238477581] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12239270373] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12240183615] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12241036269] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12241693926] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12248173608] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12249277854] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12251134698] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12251976660] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12255826473] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12257341404] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12258025560] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12259207851] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12260123601] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12260982525] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12272743065] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12275781903] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12277232781] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12278000790] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12298656216] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12319222509] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12321210660] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12324374997] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12326054598] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12328102809] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12330298332] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12332383503] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12333297438] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12334599288] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12341763819] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12343533444] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12346262445] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12349067973] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12351422688] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12354198582] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12355312299] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12366690930] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12368182761] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12375129921] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12376288881] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12406984821] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12408085239] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[12750263460] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13235486649] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13261798143] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[13295422701] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[14490511332] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=446 watches=0 history=959 journal=768 symbols=93 drops=0
[15205662879] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15300002817] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15301339053] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[15398673378] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[15462302526] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[15485924421] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[15487363353] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[15488376948] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[15492403542] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[15510134475] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[15526284015] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[15529058094] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[15583269768] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[15606522162] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[15607699272] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[15611868393] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[15636866784] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[15657150267] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[15661538442] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[15663031065] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[15729140262] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[15731744985] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[15819473043] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[15883489677] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[15894786732] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[15898879590] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[15901027362] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[15907720719] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[15968425143] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[15973474308] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[15974435004] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[15975275976] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[15976290726] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[15976985607] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[15977625939] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[15978292704] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[15978976761] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[15979611945] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[15980253729] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[15980872743] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[15981541719] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[15982199343] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[15983265243] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[15984010746] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[15984990384] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[15985734435] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[15986364108] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[15987010578] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[15987637413] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[15988262994] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[15988876464] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[15989493531] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[15990111324] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[15990776076] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[15991426341] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[15992057994] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[15992737101] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[15993358821] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[15993992487] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[15994631763] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[15995302884] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[15995938497] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[15996577113] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[15997194642] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[15997922985] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[15998683866] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[15999418116] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[16000139529] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[16000893348] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[16001654130] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[16002396828] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[16003844010] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[16005544797] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[16006397022] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16014602637] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[16028496033] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[16032975684] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[16041914229] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[16042589376] [CONTRACT] [kernel] [CPU0] Spawning init process...
[16044758004] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[16047155322] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[16048034013] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [16053991800] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[16054872603] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013584 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[16074866313] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[16076912940] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[16078062000] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[16079087574] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[16086636060] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[16093314567] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[16096669677] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[16097777817] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[16101536286] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[16104388509] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[16105520442] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[16109039562] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[16110098334] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[16111152090] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[16112147139] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16115878053] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16117421463] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16118899995] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16120602696] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16122216528] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16124591670] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16130020665] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[16172085138] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[16180876074] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[16186082187] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[16190639454] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[16193648922] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[16197390924] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[16201779660] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[16206196314] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[16210810539] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[16215771792] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[16220547948] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[16225863687] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[16230879588] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[16235168202] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[16239640857] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[16245455622] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[16251993879] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[16257507288] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[16263234933] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[16268128833] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[16273795098] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[16279748430] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[16286005824] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[16290997140] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[16295827746] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[16300461705] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[16305064182] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[16310138328] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[16314823008] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[16320676449] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[16325365254] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[16329284169] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[16334671815] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[16340590068] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[16345041504] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[16349596428] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[16355947707] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[16362025482] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[16367394285] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[16372700883] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[16378084107] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[16381772847] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[16401041877] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[16526152698] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[16533096855] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[16533851466] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16535583273] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16542070842] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[16543296891] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16551287016] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[16556850849] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16557962982] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16559468772] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079120 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[16564516452] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[16566778668] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[16567700523] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16569542781] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16573751502] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[16575616068] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16582627809] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[16586198475] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[16587605067] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[16594028649] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144656 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[16597353894] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[16600172292] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[16601975742] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[16604078568] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[16608663654] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:52:47 = 1775436767 unix_secs
[16610536239] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775436767, mono_ns=8305051887, offset=1775436758694948113ns
[16612271412] [INFO] [rtc_cmos] [CPU1] System clock anchored
[16624761714] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[16667535720] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[16680454164] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[16681535277] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16683468945] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16689301695] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[16691601234] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[16698747648] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[16701413058] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[16704471069] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16706088267] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369211024 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[16711074633] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[16715460333] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[16717268832] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[16718174946] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16719842634] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16727161176] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[16729589184] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[16736346627] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[16739178456] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[16740890331] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16742082423] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16743674970] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277488 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[16747559961] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[16749232962] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[16755311001] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[16756177086] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[16757073531] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[16757889885] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[16759342611] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[16774878021] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[16776679194] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[17258066100] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17263293069] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[17266655043] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[17268426978] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[17273011437] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[17274743607] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[17276894382] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[17278527717] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[17279671497] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[17281984962] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[17283162864] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[17284229259] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[17285235363] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[17286135702] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[17287087818] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[17288300238] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[17292770286] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[17293835394] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17295377946] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17302567293] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17304907785] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17311805049] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[17314396737] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[17317143822] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[17318910411] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352832 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[17325114807] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[17335471824] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[17346589722] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[17377422084] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[17380693572] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[17383727790] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[17384675748] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[17386098411] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[17387070657] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[17388555987] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17390347161] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17393358246] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[17394335211] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[17401770177] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17402948574] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[17405028729] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[17412354234] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[17415149499] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[17417186127] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[17417996046] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17419243743] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[17420722209] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17432916963] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17434939896] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17444860290] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[17447687664] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
[17448687465] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106698
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[17450898234] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500368 RFLAGS_BEFORE=130 CR3_BEFORE=59940864 fs_base=0 gs_base=18446744071564586640
[17454901728] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[17455815597] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17457471075] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17459090418] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[17460803382] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[17463302967] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17466098760] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17467214160] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[17469378498] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[17471112549] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[17473655100] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[17475575271] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[17476725585] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[17478655854] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[17480352318] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[17481330636] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[17483195070] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[17484042906] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[17485073133] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[17486220015] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[17510985558] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[17511844779] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[17512612425] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[17513375616] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[17514133692] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[17514873783] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[17515787190] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[17517004725] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[17522815893] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb01024a0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[17524567401] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434192 RFLAGS_BEFORE=134 CR3_BEFORE=59805696 fs_base=0 gs_base=18446744071564586608
[17529907032] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[17865914748] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106698
[17866968966] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[17868648072] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583056 RFLAGS_BEFORE=130 CR3_BEFORE=68349952 fs_base=0 gs_base=18446744071564586576
[17871831879] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[17874283449] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[17874990210] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[17876536161] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[17879142204] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[17880817713] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17884570077] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[17885751015] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17888335179] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17894258415] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[17895363717] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[17897668569] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[17905224447] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[17908275165] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[17910292917] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[17911764354] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[17912513058] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17914276281] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17933929761] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[17937975231] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[17957861328] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[17960309334] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1d0
[17961337317] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17962871520] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369719184 RFLAGS_BEFORE=134 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[17968391694] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb01024a0
[17969509833] [INFO] [netd] [CPU3] NETD: Starting network stack service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[17970972756] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[17971867584] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653648 RFLAGS_BEFORE=134 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[17977218435] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[17978861010] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[17985916542] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[17988144471] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[17990754639] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[17993951349] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[17995542147] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[17996777997] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[17998473009] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[18001418655] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[18003441522] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[18005243553] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[18005992125] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18007599819] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18073517451] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[18097295997] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[18104454885] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[18107016873] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb01024a0
[18108242229] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18109859625] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369784720 RFLAGS_BEFORE=134 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[18113752173] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[18114617070] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18116373396] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18117285780] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[18121149816] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[18125614584] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[18126603561] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[18128106942] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18129871650] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[18131564781] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18133648071] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18141954435] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18145178601] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[18152284326] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[18155305575] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[18157236801] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[18157949403] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18159407211] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18165369156] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18168220191] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18175123263] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[18177865827] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
[18178612848] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5c0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18181331487] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18182128173] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915792 RFLAGS_BEFORE=134 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[18185045802] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18186550371] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18187460841] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[18188947920] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[18195921183] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18199460136] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18206383734] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[18208754553] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5c0
[18209998686] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18211569387] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981936 RFLAGS_BEFORE=134 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[18215388609] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[18216201630] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18218015904] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18227398299] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18232749117] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[18240371655] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[18243225561] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[18245617698] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[18246316671] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18247752666] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18273390201] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18278200149] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[18285609573] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[18288714345] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cc30
[18291235446] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18292479051] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370114128 RFLAGS_BEFORE=134 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[18295396746] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[18296110833] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18297232503] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[18298159671] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18315368841] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[18318503610] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18319255647] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[18323121597] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18324499908] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18325759089] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18327322332] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[18328383975] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[18331080867] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[18332299293] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18333772545] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370198304 RFLAGS_BEFORE=130 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[18337636185] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[18338543157] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18340037133] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[18340968360] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18341988291] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[18354528060] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18355657617] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18357368568] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18359168883] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[18362925570] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18364056777] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18365561808] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18367096209] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[18378655449] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[18390423777] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[18397104990] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18398256525] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18399415716] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18400699317] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[18402568338] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[18405961200] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[18409266183] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18410622120] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18412195428] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18413769099] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[18418177998] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb01024a0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18421174530] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369850256 RFLAGS_BEFORE=134 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[18425974182] [INFO] [nectar] [CPU2] NECTAR: Started.
[18428421363] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18429404433] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18430507656] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18431653350] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5c0
[18432624309] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18434441223] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370048144 RFLAGS_BEFORE=134 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[18437619453] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[18439024032] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[18439674264] [INFO] [fontd] [CPU3] FONTD: Service ready
[18440748513] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18442732869] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370267232 RFLAGS_BEFORE=134 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[18447986106] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[18449971023] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[18451449489] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[18463093143] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18464146503] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18465154620] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18466344435] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=235 subj_lo=0
[18472798146] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18474316674] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18475001523] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[18484535190] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18485518128] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18486566340] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18487645176] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[18492602337] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18493560195] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18494556597] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18495640944] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=244 subj_lo=0
[18501068784] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18502053075] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18503048454] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18504155670] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=245 pred=0 subj_lo=0
[18512598258] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[18514377585] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[18516057945] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[18517453812] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[18519394080] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[18520565184] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[18522356457] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x44f7000
[18523613856] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[18525673122] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[18527704734] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[18529002492] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[18534352914] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[18538449270] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[18546698742] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[18547964985] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[18549193311] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[18555464565] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[18557721534] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[18559998039] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[18561433176] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[18562856334] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[18564130002] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[18565274343] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[18566164056] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[18567267576] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[18573014724] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[18578461275] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[18592101330] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[18595612299] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18598629984] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18606214077] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[18607630800] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[18612508563] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[18627653055] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18632468910] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1258 backend=VirtIO-GPU
[18634211607] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[18635095116] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18636896817] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18639221106] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18640578759] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18687249141] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18707191668] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18708256281] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18767461383] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[18781695867] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[18788933856] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[18791563626] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f0fc8
[18793249827] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4ea
[18795239199] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370508608 RFLAGS_BEFORE=130 CR3_BEFORE=72433664 fs_base=0 gs_base=18446744071564586640
[18798903948] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[18799605264] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18801165966] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18801917706] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[18805549455] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18806980335] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18814080780] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[18816434043] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f0fc8
[18817797834] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[18819395892] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370574144 RFLAGS_BEFORE=130 CR3_BEFORE=73482240 fs_base=0 gs_base=18446744071564586576
[18823491654] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[18825648864] [INFO] [echo] [CPU1] echo: starting up
[18828707469] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[18835018422] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[18835931169] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[18845951751] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[18847689861] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[18856422288] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[18858233460] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[18866293875] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[18869327037] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18873227571] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[18874884336] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[18877022637] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[18877816551] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18879277395] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18883956036] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18885729456] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18889288407] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[18890580687] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18891789807] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[18892998762] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18894068886] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[18895060503] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[18896574378] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[18898213917] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[18899816628] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[18901375416] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18902158704] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18903706173] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18911688576] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18915345207] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18921690843] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[18924168384] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[18925060704] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[18928172835] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[18929290446] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[18930854415] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[18931679580] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18933186360] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[18934270641] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18937016637] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[18938218431] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[18939406464] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18945262413] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f23b0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18948516081] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370705216 RFLAGS_BEFORE=130 CR3_BEFORE=73986048 fs_base=0 gs_base=18446744071564586640
[18952887129] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[18955948770] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[18957604413] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[18958496700] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[18962151846] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f23a0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18963685422] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370803520 RFLAGS_BEFORE=130 CR3_BEFORE=74133504 fs_base=0 gs_base=18446744071564586576
[18968545398] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[18987069420] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[18990010149] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[18996945000] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[19030937442] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[19037877837] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[19042792791] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[19065323145] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[19067681226] [INFO] [bloom] [CPU3] bloom: creating surface...
[19069069239] [INFO] [bloom] [CPU3] bloom: surface created!
[19070070294] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[19073869617] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[19076235156] [INFO] [anther] [CPU1] anther: Connected to network stack
[19090588803] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[19112093781] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[19114451598] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[19115763513] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[19121485482] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[19150560759] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[19153592634] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[19157156634] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1264
[19158057468] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 5)
[19158968334] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[19161452937] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[19170440553] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[19173448239] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[19174523511] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[19175692536] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [19183863765] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[19189138782] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[19191637311] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[19192660641] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19199802765] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[19207607595] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
T:07D0 T:0640 T:F0B0 [19231311429] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19237661850] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[19239667920] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[19246738005] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[19247975571] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[19249703583] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[19252655268] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19255692885] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[19258078290] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[19260602328] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[19262938761] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[19271564103] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1264
T:1220 [19280348472] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[19282173438] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[19288421427] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19292328132] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[19294505142] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[19300905360] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[19305585123] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[19321191252] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[19330647336] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=280
[19332380562] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[19333620702] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19335073362] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19336585653] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19338135168] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=280 subj_lo=0
[19363469598] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1274)
[19364644629] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[19379790276] [INFO] [netd] [CPU3] NETD: DHCP configured �� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[19385153238] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[19391080368] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[19394483856] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[19397473557] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[19400382936] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[19409326002] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=245
[19411332501] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[19412348835] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19413303624] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[19414066056] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19415143572] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19416299067] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=245 pred=0 subj_lo=0
[19432624299] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1277)
[19434219354] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[19447070643] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=289
[19449283326] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[19450548414] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19451595273] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19452647148] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19453750074] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=289 subj_lo=0
[19470822096] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1280)
[19472619936] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[19478560068] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[19481045661] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[19483444530] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[19605036132] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[19786284672] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[19803758205] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=659 watches=13 history=1024 journal=1024 symbols=306 drops=0
[19805897199] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[19813789974] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[19953609951] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[20088640671] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[20243768688] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[20494984191] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[20679386805] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[20891490444] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[20901287649] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[21075862599] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[21298577058] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[21375613281] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[21387575781] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[21389425827] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[21390500736] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[21391661181] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[21392987682] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[21403212732] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[21404324964] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[21405565269] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[21407051589] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=336 pred=0 subj_lo=0
[21499567254] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[21589845156] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=708 watches=15 history=1024 journal=1024 symbols=338 drops=0
[21678347823] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[21875043531] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[22015938780] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[22018157205] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[22073733693] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22101082278] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[22132198803] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22154638539] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22156382391] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22158239862] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22160271969] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=339 pred=0 subj_lo=0
[22216984746] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22268812830] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[22373154243] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22374490908] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22375663365] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22376763090] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=340 pred=0 subj_lo=0
[22380726621] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22445990952] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=fec1d29ef0678173)
[22471423953] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[22486432617] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[22491101325] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[22493335029] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[22498776267] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22535748675] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22537268754] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22538817774] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22540610763] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=341 pred=0 subj_lo=0
[22549457634] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[22603685280] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22649667414] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[22695391752] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12222000
[22697259915] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[22698909783] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[22741498989] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[22766512527] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[22776350883] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[22780985469] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[22784050608] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[22786577385] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[22791334368] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[22816170498] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[22817715987] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[22820273025] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[22834105701] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x1222e000
[22835955186] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[22850243955] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23042777208] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=14aa2015f9906e3f)
[23292499791] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[23541683682] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[23744247120] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=788 watches=18 history=1024 journal=1024 symbols=354 drops=0
[23764250631] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[24019957104] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[24128748402] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[24179949684] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24227878983] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[24229531359] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24231307023] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24233209209] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=342 pred=0 subj_lo=0
[24281349774] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[24309208935] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24384556911] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24386883015] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[24482321292] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24580311195] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24665617647] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24752927430] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24767664174] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[25105188603] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[25395647541] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[25715665008] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[25913192118] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=851 watches=19 history=1024 journal=1024 symbols=366 drops=0
[26014943394] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[26326356771] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[26631737616] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[26987658225] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[27446921667] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=884 watches=19 history=1024 journal=1024 symbols=367 drops=0
[27755360490] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27872852997] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[27986277759] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2429 ops=1 watches=19
[28298613420] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[28956738261] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=907 watches=19 history=1024 journal=1024 symbols=367 drops=0
[28979406225] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[29385222405] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[29646252174] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[29752243950] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[30322579353] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[31469793828] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=955 watches=19 history=1024 journal=1024 symbols=389 drops=0
[33810707766] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[33836973522] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[33875904711] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1047 watches=19 history=1024 journal=1024 symbols=451 drops=0
[33902733216] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[34164633987] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[34165977351] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[34194287952] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[34619099955] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[34657899903] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34659137832] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34660583826] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34661656293] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[34668218112] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34669306881] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34670344731] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34671472275] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=236 pred=0 subj_lo=0
[34679796756] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34680832560] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34681917699] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34683179718] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=453 pred=0 subj_lo=0
[34689430875] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34690599207] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34691721636] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34692964647] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=452 pred=0 subj_lo=0
[34702250847] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34703368623] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34704753897] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34706429241] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[34718310099] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[35734913148] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1084 watches=24 history=1024 journal=1024 symbols=454 drops=0
[35761106931] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[35763963906] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[35765771184] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=2 (16384b)
[35768304858] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[35770329936] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[35863158078] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[35937002739] [INFO] [fontd] [CPU3] 
```
</details>
