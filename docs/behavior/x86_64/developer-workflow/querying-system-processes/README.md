# ❌ Scenario: Querying System Processes

> Last run: 2026-04-05 18:40:52

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 5265ms | - - - |
| 2 | And the anther server is ready | ✅ | 2361ms | - [📜](./02/serial.log) - |
| 3 | When I execute the GQL query "MATCH (n:proc.Task) RETURN n.name" | ✅ | 757ms | - [📜](./03/serial.log) - |
| 4 | Then the response status should be 200 | ✅ | 0ms | - - - |
| 5 | And the response body should contain "sprout" | ❌ | 1013ms | - [📜](./05/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11694668337] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11700349617] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11704050930] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11706093168] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11707326840] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11707990173] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11708699574] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11709311691] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11709927306] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11710566714] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11711187246] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11711819955] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11712565062] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11713260471] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11713979409] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11714616804] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11715281721] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11715903738] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11716543674] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11717165031] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11717764938] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11718371511] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11718999336] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11719612608] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11720273070] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11720900400] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11721523770] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11722207266] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11722821165] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11723450442] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11724083283] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11724723945] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11725384242] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11726023155] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11726638473] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11727362163] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11728093014] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11728864026] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11729588277] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11730346155] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11731080933] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11731877355] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11733195672] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11734561608] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11735362056] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11735932527] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11736466665] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11737010571] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11737576983] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11738121846] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11738711985] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11739259323] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11739793956] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11740355319] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11740923513] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11741512860] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11742096828] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11742685251] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11743254402] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11743840251] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11744407686] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11745010398] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11745579714] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11746167576] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11746735308] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11747322906] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11747889879] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11748489555] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11749077813] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11749667919] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11750233605] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11750821203] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11751388638] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11751990261] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11752559115] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11753147076] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11753715171] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11754301944] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11754881457] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11755476249] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11756070051] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11756661213] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11757231288] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11757818985] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11758406484] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11759004543] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11759688270] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11760283689] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11760856635] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11761464627] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11762035890] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11762667939] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11763243360] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11763836931] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11764408590] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11765015955] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11765586492] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11766176136] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11766748389] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11767340970] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11767913520] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11768521182] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11769092544] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11769682683] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11770256124] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11770847715] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11771432442] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11772251568] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12023795751] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[12034904706] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12039906615] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12041206485] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12042115734] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12046389168] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12048141765] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12049256736] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12050010126] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12050716524] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12051434505] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12052474731] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12053483706] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12054205251] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12054931152] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12055636758] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12056341605] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12057484659] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12058550559] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12059292498] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12060890523] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12061877322] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12062911245] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12064555833] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12066139965] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12066948927] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12067509894] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12068312388] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12459171813] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12460437132] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12463831083] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12464747394] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12465553782] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12467209524] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12480247395] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12481564854] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12482340090] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12484157994] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12484730973] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12487277451] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12494953416] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12496768119] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12510363921] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12510936141] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12527821317] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12528400665] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12530490357] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12531766797] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12532887576] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12535181835] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12536072142] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12570914037] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62293200 ticks/sec), init_cnt=622932 for 100Hz
[12572410785] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12573316998] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12574696167] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12580444140] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12610926042] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12611895318] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12613810968] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12616008306] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12617921382] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12622385919] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12624657177] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12641919774] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12643407414] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12644097246] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12644961978] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12646805061] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12648242937] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12650330154] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12674616273] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12676279539] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12677499450] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12678919374] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12680376984] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12681737211] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12682931514] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12684044175] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12691477557] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12692370372] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12694182930] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12695046606] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12701699208] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12704315448] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12705566676] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12707734083] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12709130214] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12710492091] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12730551207] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12735553809] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12736869123] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12737845197] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12759425349] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12777919770] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12780067410] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12783444036] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12785177427] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12787509834] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12790038459] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12792219033] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12793432839] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12794899260] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12801255852] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12803134278] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12805811766] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12809722200] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12813418464] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12816544257] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12817699257] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12830117784] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12831275820] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12838918785] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12840384810] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12868564632] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12869492691] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13211484858] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13722469827] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13749406308] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[13786041687] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[15131199930] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=449 watches=0 history=963 journal=772 symbols=97 drops=0
[15863453166] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15958619358] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15960054330] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[16072570767] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[16132440258] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[16156761390] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[16160191905] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[16160906619] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[16164415476] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[16180050249] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[16199289513] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[16201535460] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[16253205507] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[16274663889] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[16275856047] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[16280472912] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[16306432230] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[16325924901] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[16331307168] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[16332333633] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[16405433946] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[16407961053] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[16500162723] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[16568628021] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[16581115485] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[16588077165] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[16590957801] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[16649829471] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[16698068508] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[16703733090] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[16705234227] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[16706584983] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[16707974283] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[16709107371] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[16710184623] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[16711261908] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[16712276031] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[16713408294] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[16714207620] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[16714857456] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[16715558442] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[16716242598] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[16716958797] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[16717932165] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[16718621172] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[16719303447] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[16719955494] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[16720630014] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[16721283513] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[16721948628] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[16722592590] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[16723242129] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[16723893120] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[16724589915] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[16725283509] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[16725940143] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[16726652217] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[16727302647] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[16727965650] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[16728670794] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[16729352640] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[16730015115] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[16730686236] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[16731333762] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[16732118535] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[16732892220] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[16733673462] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[16734443154] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[16735371576] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[16736164401] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[16736960427] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[16738488459] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[16740285078] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[16741108395] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16749433173] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[16765513710] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[16770107904] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[16779435783] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[16780151223] [CONTRACT] [kernel] [CPU0] Spawning init process...
[16782212568] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[16785034926] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[16785765381] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [16791929583] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[16794074880] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013440 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[16818883455] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[16823735973] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[16825143588] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[16826486919] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[16836052431] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[16842166968] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[16845821652] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[16847120004] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[16850907414] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[16854329184] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[16855625028] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[16860343731] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[16861636341] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[16862891364] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[16864181499] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16867842420] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16869152322] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16870473906] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16872079389] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16873640388] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16875180861] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16880279097] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[16931224959] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[16940098461] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[16951500951] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[16959835431] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[16964509221] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[16970399457] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[16978933983] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[16985380071] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[16991334096] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[16997741871] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[17003984250] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[17010899136] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[17017096767] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[17022914271] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[17028397221] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[17034111501] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[17040571548] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[17046813531] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[17052591006] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[17058678945] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[17064327159] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[17070534327] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[17077252995] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[17083098879] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[17088997134] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[17094866283] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[17101489053] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[17107826175] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[17113888506] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[17120135274] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[17125872423] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[17131511727] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[17138325864] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[17150655588] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[17155818240] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[17161527603] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[17167140210] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[17172908577] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[17178507357] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[17184908598] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[17192559384] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[17197067514] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[17221290867] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[17374760964] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[17381837484] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[17382769767] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17384791149] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17389616376] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17391077715] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17399551818] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[17404293687] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[17405698596] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17407602894] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078976 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[17413863852] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[17415872298] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[17416785804] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17418537609] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17424552981] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[17426957229] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17436401103] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[17440916988] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[17443389018] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[17444866164] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[17446932030] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144512 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[17454384057] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[17456187771] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[17458370457] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17462931321] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:44:00 = 1775439840 unix_secs
[17464752063] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775439840, mono_ns=8732151466, offset=1775439831267848534ns
[17466663720] [INFO] [rtc_cmos] [CPU1] System clock anchored
[17481149268] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[17526141831] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[17533687149] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[17534628177] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17537411628] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17546446335] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[17549995518] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[17561346561] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[17566579272] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[17570750736] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17572916691] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[17579897610] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[17586823089] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[17588882421] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[17589827244] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17591600037] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17599068333] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17601696684] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17609255334] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[17612848902] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[17615093166] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[17616147483] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17618550675] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[17625850605] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[17627143050] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[17634241119] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[17635632333] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[17636735523] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[17638519602] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[17640276621] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[17654556777] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[17656324950] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[18162603591] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18181294065] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[18185752365] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[18188297094] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[18195334674] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[18197693184] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[18200806998] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[18203053209] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[18204629421] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[18208316445] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[18209811543] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[18211290372] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[18212634495] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[18213943638] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[18215279973] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[18216756228] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[18231427632] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[18232407369] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18234390372] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18241820916] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18244534308] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18251894232] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[18254527929] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fab68
[18256076553] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[18258201951] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369357056 RFLAGS_BEFORE=134 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[18268071228] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[18279599316] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[18298276920] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[18309711321] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[18312631656] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[18320125494] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[18333032685] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[18334826994] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[18340828440] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[18341898234] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[18342971757] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[18343900542] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[18344922684] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[18345876483] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[18347192853] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[18348222981] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[18352027221] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[18841392174] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[18846218391] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[18851016954] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[18853660551] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[18854839773] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18857434959] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18863313579] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18865950873] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18884847597] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[18889430769] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[18892174125] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[18893308500] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18897871410] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18905248692] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18907436064] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18918768033] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[18923027343] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[18924241545] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[18926088687] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500624 RFLAGS_BEFORE=130 CR3_BEFORE=68513792 fs_base=0 gs_base=18446744071564586640
[18932812701] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[18934080330] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[18934821015] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[18936049836] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18938157513] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18939118308] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[18940896216] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[18942611193] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[18944343462] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18945324948] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[18947047779] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[18947886903] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18948798330] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[18951034806] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[18952430376] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[18953864622] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[18957064830] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[18958212768] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[18961730139] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[18963491679] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[18964970904] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[18966775641] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[18968493918] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434832 RFLAGS_BEFORE=130 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[18974258523] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[18981606435] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[18982507566] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
[18983883270] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[18985895148] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[18986765127] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583632 RFLAGS_BEFORE=130 CR3_BEFORE=68771840 fs_base=0 gs_base=18446744071564586576
[18993653712] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[18994703871] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18996847815] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[18998119140] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[18999085710] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19000848504] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19001870844] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[19009884465] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[19011232614] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[19013619768] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[19024377471] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[19029199728] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[19032494679] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[19034841606] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[19035950109] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19038480516] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19068514080] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19074814836] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[19476713388] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[19478157435] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[19480457205] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[19482671142] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[19970016660] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[19973041308] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[19974532479] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19976317647] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716736 RFLAGS_BEFORE=134 CR3_BEFORE=69156864 fs_base=0 gs_base=18446744071564586640
[19983519039] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[19985347008] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[19986432444] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650400 RFLAGS_BEFORE=130 CR3_BEFORE=69021696 fs_base=0 gs_base=18446744071564586608
[19989874839] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[19992251697] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[19993644000] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[19997838069] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[19999496385] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[20003110611] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[20004485655] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[20005556505] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[20007083712] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[20010150963] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[20011946889] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[20013625929] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[20014341369] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20015847390] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20083770762] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[20108627550] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[20116486500] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[20119350306] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[20120896257] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20122882857] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369782672 RFLAGS_BEFORE=134 CR3_BEFORE=79892480 fs_base=0 gs_base=18446744071564586576
[20127394320] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[20128667163] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20131348215] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20132572020] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[20137090743] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[20143051665] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20145264876] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20164359105] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20167783482] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[20175766017] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[20178373842] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[20180392287] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[20181110004] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20182561245] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20188668192] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20191126494] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20198632641] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[20201142258] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010aff8
[20202583335] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20204304318] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915056 RFLAGS_BEFORE=130 CR3_BEFORE=80826368 fs_base=0 gs_base=18446744071564586640
[20207741730] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[20208818025] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20210624082] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[20211506832] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20212591245] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[20222947470] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20228521038] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[20239665501] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[20243570754] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010aff8
[20245334604] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20247477162] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981616 RFLAGS_BEFORE=130 CR3_BEFORE=80953344 fs_base=0 gs_base=18446744071564586576
[20250940809] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[20252221836] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20254580742] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20269149087] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[20274851421] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[20286083631] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[20288796759] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[20291108343] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[20291880576] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20293352541] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20321285886] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20326335249] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[20336582079] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[20340729750] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cc18
[20342106048] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20344096014] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113904 RFLAGS_BEFORE=130 CR3_BEFORE=81256448 fs_base=0 gs_base=18446744071564586640
[20348462046] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[20349859728] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20352075018] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[20353511838] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20376519669] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[20381079510] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[20382291468] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20384605692] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20386483920] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20387789466] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20390386236] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[20399293035] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[20403745494] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[20405313885] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[20406813933] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[20407992726] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20410621704] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20411852142] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370202320 RFLAGS_BEFORE=130 CR3_BEFORE=81530880 fs_base=0 gs_base=18446744071564586576
[20418823260] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[20420374194] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[20425440420] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20427131835] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20428686465] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20430287658] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20431981779] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20433642240] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20435360517] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[20437352793] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=229 pred=0 subj_lo=0
[20460184833] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[20471415162] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[20480104524] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[20482859694] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[20487337167] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20489953836] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369848976 RFLAGS_BEFORE=134 CR3_BEFORE=80564224 fs_base=0 gs_base=18446744071564586608
[20495280729] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20496556410] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20497698243] [INFO] [nectar] [CPU2] NECTAR: Started.
[20498571621] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20500095660] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[20502038106] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010aff8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20504508024] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047920 RFLAGS_BEFORE=130 CR3_BEFORE=81100800 fs_base=0 gs_base=18446744071564586608
[20512102974] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db478
[20517357663] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
[20518507779] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20520034722] [INFO] [fontd] [CPU3] FONTD: Service ready
[20520651657] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20523466590] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20525224368] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[20530575582] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370269200 RFLAGS_BEFORE=130 CR3_BEFORE=81739776 fs_base=0 gs_base=18446744071564586608
[20538848946] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[20541239499] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[20544200325] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[20555172891] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[20569651212] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[20588376237] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[20592193644] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20594191035] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20600545251] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[20612056608] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20613729147] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20615699115] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20617719276] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=235 subj_lo=0
[20637208680] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20639048001] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20641055457] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20643136569] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[20659515855] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20661080484] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20662718703] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20664482751] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[20666600526] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1251 backend=VirtIO-GPU
[20669407407] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[20670533532] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20672323188] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20680856097] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20682301332] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20683833753] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20685596382] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[20697828690] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20699361474] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20700933495] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20702770902] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[20744321631] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20746382019] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20766161361] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[20800503537] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[20802490071] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[20817039606] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[20831689362] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[20840373411] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[20843039283] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db478
[20845306416] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e3
[20847097161] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370358928 RFLAGS_BEFORE=134 CR3_BEFORE=82657280 fs_base=0 gs_base=18446744071564586640
[20850778509] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[20851663338] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20852945883] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[20854105866] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20859244758] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[20861320425] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20870096412] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[20873501649] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db478
[20874862965] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[20876900418] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370424464 RFLAGS_BEFORE=134 CR3_BEFORE=83705856 fs_base=0 gs_base=18446744071564586576
[20881325718] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[20883318423] [INFO] [echo] [CPU1] echo: starting up
[20886859521] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[20888108571] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[20890154043] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f4000
[20891635875] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[20892564990] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f4000
[20894207367] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f7000
[20896258944] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276783104)
[20897765559] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[20899500864] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f9000 phys=0x5031000
[20901736086] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[20904441591] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[20906813598] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[20908029615] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[20909183559] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[20910095019] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[20925373755] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[20928933201] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[20931401073] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[20937955500] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20939112051] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[20940258009] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20943403305] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[20953889055] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[20955809193] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[20956831236] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[20958140676] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[20960446716] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[20962705533] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[20964916632] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[20966414964] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[20967993024] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[20969671173] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[20971095651] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[20972365029] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[20975105844] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[20977573749] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[20979991230] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[20983296939] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[20992024845] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21001360611] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[21002441757] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[21003599892] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[21004651107] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[21005880522] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21008022090] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[21009134157] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21014719935] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21017110059] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[21020223774] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[21025559841] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[21029150142] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[21031158621] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[21032678931] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[21034441593] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[21035365923] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21037033974] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21045138675] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21049137087] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[21057179154] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[21060021246] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[21061449222] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[21062516673] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db478
[21064096185] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[21065032164] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21067198647] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21068122053] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370703616 RFLAGS_BEFORE=134 CR3_BEFORE=84336640 fs_base=0 gs_base=18446744071564586640
[21072983811] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[21074448945] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[21079080825] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21083062341] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[21085879881] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[21087474771] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[21100325169] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[21103860558] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[21110490918] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[21126639765] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[21127931253] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db478
[21129432060] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21131604714] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370773248 RFLAGS_BEFORE=134 CR3_BEFORE=84484096 fs_base=0 gs_base=18446744071564586576
[21137913885] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[21150621129] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[21154297461] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[21157174038] [INFO] [bloom] [CPU3] bloom: creating surface...
[21158665902] [INFO] [bloom] [CPU3] bloom: surface created!
[21159553371] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[21168053313] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[21175845009] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1261
[21178039278] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[21191127705] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21192404079] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21198297087] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[21202431822] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[21203908737] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[21205646484] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [21213650568] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[21225537597] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[21238354929] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[21250635351] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
T:07D0 T:0640 T:F0B0 [21275713503] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 3)
[21277079241] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1261
[21290684613] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21304042716] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[21313106100] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[21316012641] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[21318259875] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[21321363030] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[21332173302] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[21334046877] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
T:1220 [21341202663] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[21343564110] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[21353747844] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=274
[21354903801] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[21358726356] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21360339759] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21370440960] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[21371771553] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21373123266] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21374646975] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=274 subj_lo=0
[21388806879] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1271)
[21390059658] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[21391438464] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[21400182474] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[21402828876] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[21407218734] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[21413915259] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=244
[21415677162] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[21416767086] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[21417900603] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21419136156] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21420501993] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[21435410007] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1275)
[21436635231] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[21453149058] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=276
[21454517502] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[21455637027] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[21456818328] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21458328606] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21459677943] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=276 subj_lo=0
[21474105048] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1276)
[21475552923] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[21508059738] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([250, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[21538745052] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[21541609551] [INFO] [anther] [CPU1] anther: Connected to network stack
[21588358935] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[21590338506] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[21596760108] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[21627291477] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[21628068858] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[21646161141] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[21673706307] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[21676193715] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[21693627153] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[21700578108] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[21703361196] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[21720846345] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[21722466843] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[21724779714] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[21735714759] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[21740045613] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[21742527807] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[21766252365] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[21787292439] [INFO] [netd] [CPU3] NETD: DHCP configured ��� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[21791325633] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[21796477032] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[21801842964] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[21805449501] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[21808766991] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[21818406984] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[21827219997] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[21830028957] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[21832337373] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[21924862773] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21939167778] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[21941206122] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[21944435865] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[21950378571] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[21956439054] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[21964440003] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[21985041012] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=74
[21986848950] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=25
[21989904849] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[21999484353] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[22004776563] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[22007295684] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[22010695245] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[22012724877] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[22015658940] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22028817855] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22031687469] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=149
[22033243782] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=25
[22034813394] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[22036312353] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[22037722344] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:33888 on listener 1
[22044244464] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[22066169103] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=3
[22073223414] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[22076984820] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 37 (user thread) assigned to CPU 1
[22079589939] [INFO] [anther] [CPU1] anther: Thread spawned TID=37 for conn_handle=3
[22098007767] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[22107092964] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
T:5EE0 [22115939406] [INFO] [anther] [CPU1] anther: Worker thread TID=37 starting for conn_handle=3
[22117437969] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[22140367392] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[22149304353] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=33, our_read=34)
[22151579340] [INFO] [anther] [CPU1] anther: Worker TID=37 connected to netd OK
[22183302999] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=651 watches=13 history=1024 journal=1024 symbols=307 drops=0
[22324293750] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[22492465611] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[22644803412] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[22813324809] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[23154570681] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[23348598636] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[23383806006] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[23560662081] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[23799079524] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[24024075702] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[24032368635] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[24034591548] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[24036132318] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[24037858911] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[24040064367] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[24058803450] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[24060383688] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[24062186412] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[24064234458] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=337 pred=0 subj_lo=0
[24133778922] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[24378108843] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[24535010808] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 85 bytes on conn_handle=3
[24542854479] [INFO] [anther] [CPU1] anther: GET /health Http11
[24558568254] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=716 watches=15 history=1024 journal=1024 symbols=339 drops=0
[24561030384] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[24565022064] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[24566261412] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[24568846005] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[24577114485] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[24579151146] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[24581444976] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[24582714222] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[24583894665] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[24592879311] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24596907060] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[24609778743] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[24612440556] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[24628441662] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[24631798917] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[24636732450] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[24648800385] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[24651128733] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[24653088900] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[24654475758] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[24656827206] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[24660939171] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[24664474098] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[24688567761] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[24694906731] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[24697580127] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[24702458385] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[24708178374] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[24711727458] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[24716277036] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[24728780076] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[24731739945] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[24734562072] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24735649356] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[24747192426] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=184
[24748940601] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[24750134442] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 174 byte frame (178 encoded) to netd rx_port=25
[24753127839] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (178 bytes sent)
[24755084574] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 174 bytes
[24775845006] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[24778297005] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[24858924519] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24897715029] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[24899377272] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24900959160] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24902586258] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=339 pred=0 subj_lo=0
[24952568553] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 0 bytes on conn_handle=3
[24997567089] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[25016321682] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[25018685802] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:33904 on listener 1
[25050957063] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25170374097] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[25176398544] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25189816443] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=4
[25203554376] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 38 (user thread) assigned to CPU 1
[25206017958] [INFO] [anther] [CPU1] anther: Thread spawned TID=38 for conn_handle=4
[25237487154] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[25260552405] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[25288841358] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[25290167661] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25291489377] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25292931807] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=340 pred=0 subj_lo=0
[25321114335] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[25340322579] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
T:5EE0 [25364163198] [INFO] [anther] [CPU1] anther: Worker thread TID=38 starting for conn_handle=4
[25391553000] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[25393034931] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25394782875] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25396492836] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=341 pred=0 subj_lo=0
[25463667570] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25470754386] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[25494768354] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x121f1000
[25496482077] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[25498189002] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[25538453127] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=35, our_read=36)
[25540790088] [INFO] [anther] [CPU1] anther: Worker TID=38 connected to netd OK
[25600545333] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[25615702101] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[25622153667] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[25624034205] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[25626773964] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[25629547383] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[25637147646] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[25647615477] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[25656604347] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 120 bytes on conn_handle=4
[25658660115] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[25659615828] [INFO] [anther] [CPU1] anther: POST /api/v1/query Http11
[25660706082] [INFO] [anther] [CPU1] anther: Request body size: 33 bytes
[25662939753] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[25664258004] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[25665420429] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[25666497186] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[25672478799] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[25673518200] [INFO] [phloem::executor] [CPU1] phloem: calling find for kind: proc.Task
[25674666996] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[25677232218] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[25678473645] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=1 (16384b)
[25723129707] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x121fe000
[25724096046] [INFO] [phloem::executor] [CPU1] phloem: find returned 0 candidates
[25725296553] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[25726403175] [INFO] [phloem::executor] [CPU1] phloem: discovered 0 nodes
[25929402246] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=0e57f834b3afbcb1)
[26120254083] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[26379894123] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=c1e9a7b90a789e5c)
[26619062580] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[26758671621] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 209 bytes - TCP ACK
[26765615052] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 209 bytes
[26775134166] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[26777300748] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[26780320050] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[26972983158] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[27155227539] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[27157084251] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[27158974128] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[27160951290] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=342 pred=0 subj_lo=0
[27171277452] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[27174029817] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 124 bytes - TCP ACK
[27182430363] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 124 bytes
[27194763420] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[27204460932] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[27207640713] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27214169598] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[27217304103] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[27220820616] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27363910761] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[27451101645] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=788 watches=19 history=1024 journal=1024 symbols=356 drops=0
[27667928541] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[27939900615] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[27980600505] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[27997262205] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28007199396] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28010288790] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[28013821110] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[28025092458] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[28035921837] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 0 bytes on conn_handle=4
[28205048685] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28214257467] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[28329838053] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[28371042348] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28663959060] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[29010579840] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[29394554145] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[29827330170] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[30144995034] [INFO] [kernel
```
</details>
