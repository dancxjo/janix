# ❌ Scenario: Discovering that visual elements are graph nodes

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 5057ms | - - - |
| 2 | And the anther server is ready | ✅ | 1827ms | - [📜](./02/serial.log) - |
| 3 | When I wait for 5 seconds | ✅ | 5001ms | - [📜](./03/serial.log) - |
| 4 | Then I should see the desktop wallpaper | ❌ | 1013ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11303219202] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11308652058] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11313223944] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11315146986] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11316411051] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11317038975] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11317710228] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11318281755] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11318881134] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11319483978] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11320064844] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11320658976] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11321347224] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11322017850] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11322692964] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11323291881] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11323903338] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11324489352] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11325107310] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11325693786] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11326262211] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11326837269] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11327418102] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11327997384] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11328633327] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11329224951] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11329811526] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11330450769] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11331028797] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11331621015] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11332552737] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11333497593] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11334423078] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11335225506] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11335806438] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11336490066] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11337180921] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11337880554] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11338593486] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11339308167] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11340003312] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11340704001] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11341969023] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11343374757] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11344212099] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11344753794] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11345289615] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11345806791] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11346353403] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11346869523] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11347376601] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11347890675] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11348412339] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11348946477] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11349484839] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11350041087] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11350584267] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11351140020] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11351691021] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11352246114] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11352783420] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11353340295] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11353877832] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11354524665] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11355383490] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11356207170] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11357205222] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11358195486] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11358902940] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11359474236] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11360019957] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11360584488] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11361128988] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11361709755] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11362309299] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11363135388] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11363937156] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11364838914] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11365520166] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11366085060] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11366629593] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11367189339] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11367729681] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11368313154] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11368869138] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11369428554] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11369973714] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11370534483] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11371154454] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11371745979] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11372290149] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11372851479] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11373399576] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11373962919] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11374525965] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11375087196] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11375628726] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11376190848] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11376732477] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11377657038] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11378520912] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11379342480] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11380106760] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11380665351] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11381235129] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11381795997] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11382336603] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11383189653] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11628982464] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[11641854081] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11646760719] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11648501238] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11649743556] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11653940628] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11655618315] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11656672302] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11657340684] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11658014346] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11658793377] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11660058762] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11661024342] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11661718299] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11662400640] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11663159409] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11663839209] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11665012326] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11666039385] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11666745057] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11668336317] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11669326977] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11670685917] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11673149631] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11674705944] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11675463228] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11676005847] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11676760821] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12060767994] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12061758324] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12064833000] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12065695125] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12066491250] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12068231208] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12080379201] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12081606306] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12082339500] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12083923005] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12084444042] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12086836773] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12093858447] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12095533956] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12108418047] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12108950007] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12124680612] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12125261478] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12127169208] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12128330511] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12129332292] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12131365653] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12132160854] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12167026575] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62339400 ticks/sec), init_cnt=623394 for 100Hz
[12168448083] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12169379409] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12170562657] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12175842426] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12206086002] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12206931363] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12209785269] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12211423686] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12212751738] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12215306466] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12216614817] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12235769997] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12236597241] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12237497712] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12238598592] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12239580144] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12240759696] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12241401777] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12266274504] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12267182895] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12269323539] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12270147879] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12271088940] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12271764318] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12272924532] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12273566349] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12279527601] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12280939044] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12282851064] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12283921650] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12289120635] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12290722389] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12291598704] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12293000478] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12293906790] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12294753009] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12306851997] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12309846021] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12310980957] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12311725041] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12334257210] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12351381603] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12353202048] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12356125155] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12357834621] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12359908275] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12362154156] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12363981465] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12364920810] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12366133131] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12371804313] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12373465005] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12376192422] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12378321417] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12380509812] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12382861392] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12383602242] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12398415084] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12399320175] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12408978846] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12409694352] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12439155696] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12440173911] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[12758107692] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13214505876] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13237780347] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=497 journal=422 symbols=51 drops=0
[13271208258] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[14435826471] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=446 watches=0 history=959 journal=768 symbols=93 drops=0
[15118411638] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15205947108] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15206882823] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[15301297011] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[15354471825] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[15379624161] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[15380414808] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[15381053490] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[15384721902] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[15399167454] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[15414357717] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[15416745432] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[15465678822] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[15483153543] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[15483870963] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[15487377444] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[15510948453] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[15528695523] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[15532202202] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[15533132934] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[15594157227] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[15598094655] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[15678224397] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[15739198959] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[15751742193] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[15756275370] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[15758426574] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[15765184215] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[15822257583] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[15827354301] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[15828341661] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[15829149270] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[15830038422] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[15830697861] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[15831344133] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[15831972519] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[15832566222] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[15833161344] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[15833781942] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[15834399636] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[15835016406] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[15835650732] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[15836316210] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[15837011751] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[15837654591] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[15838289346] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[15838896579] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[15839523150] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[15840130086] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[15840970134] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[15841792659] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[15842618913] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[15843479982] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[15844201626] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[15844816185] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[15845422263] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[15846079392] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[15846741966] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[15847422327] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[15848428629] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[15849186342] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[15849969168] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[15850720083] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[15851341539] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[15852055032] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[15852773706] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[15853496670] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[15854216928] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[15854963124] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[15855687375] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[15856553823] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[15858031035] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[15859657902] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[15860418684] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[15868942815] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[15881941845] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[15886299990] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[15895280907] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[15895931601] [CONTRACT] [kernel] [CPU0] Spawning init process...
[15897809862] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[15900244470] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[15901007925] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [15906521169] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[15907699005] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[15922343811] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[15926878176] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[15928067793] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[15929087691] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[15936023994] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[15940121538] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[15943039992] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[15944092692] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[15947580099] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[15950547261] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[15951602931] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[15955893591] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[15957032784] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[15958075221] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[15959043507] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[15962292720] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[15963581700] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[15964857480] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[15966354327] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[15967741449] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[15969050394] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[15972505923] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[16015135290] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[16022892105] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[16027862565] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[16032243249] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[16035055179] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[16040016498] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[16045984812] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[16051415391] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[16057579461] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[16063653771] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[16068479559] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[16073435895] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[16079808690] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[16085309493] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[16090741623] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[16096656345] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[16103080290] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[16108510341] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[16113821064] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[16119275370] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[16124469669] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[16129977039] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[16135652808] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[16142252808] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[16148702262] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[16154447397] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[16159816068] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[16165817679] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[16172110977] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[16177143048] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[16181081994] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[16184513763] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[16189772874] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[16194285525] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[16198688649] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[16204539351] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[16209583401] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[16215673386] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[16223662125] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[16229915559] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[16236123618] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[16239478233] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[16260611103] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[16390811547] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[16397118573] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[16397926215] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16399540608] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16403743983] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[16405002900] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16412779581] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[16417966818] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16418944905] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16420310214] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[16424770626] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[16425571371] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[16426212495] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16427863980] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16432414416] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[16434068706] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16440277920] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[16444167795] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[16444965009] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[16446462219] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[16448304114] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[16455844713] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[16457870253] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[16459857876] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[16464119496] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:04:28 = 1775437468 unix_secs
[16465890936] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775437468, mono_ns=8232730456, offset=1775437459767269544ns
[16467686499] [INFO] [rtc_cmos] [CPU1] System clock anchored
[16478727903] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[16513932303] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[16522086537] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[16523157387] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16524924273] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16531203876] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[16534370820] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[16541854461] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[16545670350] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[16548885936] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16550559597] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[16555744194] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[16559927637] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[16561796625] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[16563092865] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16565274429] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16571866410] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[16574186607] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[16581244713] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[16585076970] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16586428584] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16588379214] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[16594963638] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[16596611823] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[16602672768] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[16603371939] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[16605058206] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[16606760082] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[16614084597] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[16615954113] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[16630361913] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[16631892486] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[17079192306] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17084670834] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[17087338620] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[17088915756] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[17093246841] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[17094960531] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[17097008115] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[17098490079] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[17099486580] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[17101681872] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[17102677449] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[17103619929] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[17104427802] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[17105378994] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[17106317448] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[17107635237] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[17112600648] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[17113481352] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17115123630] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17122067094] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17124288060] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17131526511] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[17134642173] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fab68
[17136524988] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[17138763510] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352928 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[17145716907] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[17156694885] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[17193213939] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[17201867925] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[17204860563] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[17211453171] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[17224363695] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[17226868560] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[17234685567] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[17235640950] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[17236947354] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[17243031168] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[17244556263] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[17246027832] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[17247333510] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[17248670175] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[17249951235] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[17259580668] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[17264372730] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[17268296727] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[17270745657] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[17272310451] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17274659952] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17278327671] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[17280103929] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17287771578] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[17290213710] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[17292164109] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[17293066296] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17294816649] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17299501461] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17300999034] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17308898409] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[17311765548] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
[17313281304] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[17315448645] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500320 RFLAGS_BEFORE=134 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[17319377460] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[17320263642] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17321853450] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17323582155] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[17325228030] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[17327075733] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17329911456] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17331571950] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[17333733384] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[17335793970] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[17338157727] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[17339213496] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[17341633320] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[17343696414] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[17344633548] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[17346500556] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[17347348425] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[17348774685] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[17349766368] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[17351495370] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[17352672876] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[17354406795] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434240 RFLAGS_BEFORE=130 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[17359486584] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[17681290836] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
[17682702312] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[17684633802] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583104 RFLAGS_BEFORE=134 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[17689168959] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[17691685605] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[17694115923] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[17697014148] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17698771596] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[17700989592] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[17702412321] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17704345428] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[17705375985] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17714767785] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[17718386697] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[17723164602] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[17727501990] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[17730446778] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[17732995500] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[17734502115] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[17735432913] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17737264776] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17756014815] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[17760463083] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[17779900644] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[17782696074] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[17784653040] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17786580306] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369719024 RFLAGS_BEFORE=134 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[17791161300] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[17794217892] [INFO] [netd] [CPU3] NETD: Starting network stack service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[17796698634] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[17797616628] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653488 RFLAGS_BEFORE=134 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[17806075056] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[17808227184] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[17845486791] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[17848880643] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[17856190638] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[17858392068] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[17859834201] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[17862155916] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[17866199736] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[17868339390] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[17869416081] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[17870933619] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[17871802707] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17873577513] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17925214428] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[17926650522] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[17928777669] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[17930815716] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[17936896626] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[17959440213] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[17966123241] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[17968723476] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[17970368691] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17972183889] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369784560 RFLAGS_BEFORE=134 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[17975572230] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[17977239687] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17978438775] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[17979258594] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17980817481] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[17984330562] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[17986297197] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18002982657] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18005924541] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[18012635091] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[18015915786] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[18018308847] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[18019012803] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18020497308] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18025745925] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18027967320] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18034438257] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[18037316319] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010aff8
[18039499962] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18041271501] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915632 RFLAGS_BEFORE=134 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[18044551701] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18045535101] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18047202327] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[18048025083] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18049163022] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[18059820042] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18063369951] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18069776274] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[18072648891] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010aff8
[18074009316] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18075945756] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981872 RFLAGS_BEFORE=130 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[18079952583] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[18081281031] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18083590437] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18095090673] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18099298800] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[18105802044] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[18108509925] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[18110629416] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[18111356505] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18112915359] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18136246557] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18141014859] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[18147658980] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[18150339636] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cc18
[18151241724] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18152475429] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370114064 RFLAGS_BEFORE=130 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[18155313198] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[18156131169] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18158637783] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[18159571848] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18179989146] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[18181259943] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[18184413918] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18185817936] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18187646169] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18189030255] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18190621383] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[18196500300] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[18199006749] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[18200658432] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[18202106175] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[18202954143] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18204888075] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18205720632] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370199552 RFLAGS_BEFORE=134 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[18212070492] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[18213942879] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[18217231791] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18218260764] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18219820377] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18221408634] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18223050153] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18225060876] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18226364013] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[18228463803] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=229 pred=0 subj_lo=0
[18240692580] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[18250123023] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[18257505288] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[18260148885] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[18262630584] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18264471258] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18266247285] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18267865869] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18269443368] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[18270798678] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18272580810] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[18274540251] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18276561864] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[18278189259] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369850096 RFLAGS_BEFORE=134 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[18284504304] [INFO] [nectar] [CPU2] NECTAR: Started.
[18288398436] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010aff8
[18289729821] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18291713550] [INFO] [fontd] [CPU3] FONTD: Service ready
[18292411599] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370048080 RFLAGS_BEFORE=134 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[18299014437] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[18301793367] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010dfc8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18304055781] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370267104 RFLAGS_BEFORE=130 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[18308282817] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18309799563] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18313648188] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18314837541] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18315983631] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18317123913] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=234 subj_lo=0
[18321401241] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[18325069389] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[18326735229] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[18328400112] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[18348103158] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18349223838] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18350364384] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18351537006] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=236 subj_lo=0
[18354672963] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[18361502115] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[18365022588] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18366093933] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18367294902] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18368320575] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[18379607499] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18380743458] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1251 backend=VirtIO-GPU
[18382175295] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18383369796] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[18384407877] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18385648545] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18387441765] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[18389040153] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18396812412] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18397868874] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18399146898] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18400265862] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[18421978872] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[18435268632] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[18436451847] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[18484013031] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18485287656] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18516482919] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[18530700144] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[18538115541] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[18541069173] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[18542516586] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f0fe8
[18543466425] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e3
[18544754712] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18546016830] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370359824 RFLAGS_BEFORE=130 CR3_BEFORE=72052736 fs_base=0 gs_base=18446744071564586640
[18549304686] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18553314681] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18554609964] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18558298704] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[18562455483] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[18565389777] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[18567124125] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[18567940347] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f0fe8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[18569634138] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370433552 RFLAGS_BEFORE=130 CR3_BEFORE=73101312 fs_base=0 gs_base=18446744071564586576
[18582652671] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[18584124438] [INFO] [echo] [CPU1] echo: starting up
[18586142025] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[18604164051] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[18606562986] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[18610393098] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[18612721215] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f5000
[18614954028] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f5000
[18616761240] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f8000
[18620006988] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276787200)
[18621967188] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[18624581547] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f9000 phys=0x4656000
[18626561547] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[18629034699] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[18629831220] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[18630933948] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[18633149271] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[18635407989] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[18636270543] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[18650079855] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[18651221556] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18652564161] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18654377412] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[18656495814] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[18658518120] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[18662946918] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[18675316935] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[18676770453] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[18680689632] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[18683033490] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[18684417411] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[18686672136] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[18688093875] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[18689341341] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[18690678600] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[18691754268] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[18692890656] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[18698331267] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[18703612290] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[18710007261] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[18712026432] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[18715677882] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[18716885418] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18718423515] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18722657976] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[18723597090] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18724418526] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[18725394204] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18726495711] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[18732762807] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[18734987370] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[18736869789] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[18738275490] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[18739840218] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18740640765] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18742253343] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18749287689] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18753953922] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18757783275] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[18761264313] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[18763816170] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f1678
[18764964075] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18766330341] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[18767308824] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370707984 RFLAGS_BEFORE=130 CR3_BEFORE=73998336 fs_base=0 gs_base=18446744071564586640
[18771504972] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[18772783887] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18775018284] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18778589676] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[18779955282] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18785659662] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[18787509741] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[18790027971] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[18791540394] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[18798536691] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18813059925] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[18814206015] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[18819582309] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[18821595969] [INFO] [bloom] [CPU3] bloom: creating surface...
[18822752454] [INFO] [bloom] [CPU3] bloom: surface created!
[18823353021] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[18824363943] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[18825098490] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1688
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18827660676] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370773520 RFLAGS_BEFORE=130 CR3_BEFORE=74145792 fs_base=0 gs_base=18446744071564586576
[18834211176] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[18839460552] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18845296536] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[18851460243] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1261
[18852448791] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[18863306946] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18864607014] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18865370634] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[18867935955] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[18868948263] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[18870395049] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [18876475266] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[18890395260] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[18899219856] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[18900142272] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 3)
[18906859521] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[18913639932] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1261
T:07D0 T:0640 T:F0B0 [18923615403] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[18935252985] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[18939471243] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[18943061181] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[18952510962] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[18954925506] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[18957892932] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[18959765286] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[18966369213] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[18968757060] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
T:1220 [18975761904] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[18977544861] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[18987271083] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=276
[18988542078] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[18989631870] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[18990719583] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18991949625] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18993294870] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=276 subj_lo=0
[19006510149] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[19007813616] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19012249278] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1271)
[19013392497] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[19024843695] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=243
[19026522141] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[19027581969] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19028789934] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19029980904] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19031309880] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[19044588783] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1275)
[19045989039] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[19057448619] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=279
[19058638203] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[19059794193] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19060939194] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19062269358] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19063637076] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=279 subj_lo=0
[19068512562] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([246, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[19078248090] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1276)
[19079590827] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[19123604511] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[19125216726] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[19131057957] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[19145728074] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[19147910925] [INFO] [anther] [CPU1] anther: Connected to network stack
[19157391165] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[19158250848] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[19160175837] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[19185004476] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[19187502576] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19208997654] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19217018997] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[19223425716] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[19229378520] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[19231126431] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[19233627468] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19260735318] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[19267428939] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19271011650] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[19272648912] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[19309912875] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[19312996362] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[19317941478] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[19320369123] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[19322469804] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[19324445679] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[19330807881] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[19347807303] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[19349942337] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[19352490003] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[19354066743] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=638 watches=13 history=1024 journal=1024 symbols=299 drops=0
[19439005146] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[19449233694] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[19489127097] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[19524129009] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[19549759185] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[19594494216] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[19596444912] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[19598495697] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[19621032981] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[19774153971] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[19928397984] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[20117465841] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[20353527546] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[20506387704] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[20582896422] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[20682805341] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[20875298796] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[20960202813] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[20964923199] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[20966227161] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[20967190266] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[20968261974] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[20969679027] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[20978180949] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[20979245430] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[20980380531] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[20981842200] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=339 pred=0 subj_lo=0
[21064260657] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[21133800072] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=711 watches=15 history=1024 journal=1024 symbols=341 drops=0
[21238707765] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[21409773231] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[21566968566] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[21576047691] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[21585938286] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[21588035634] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[21595994805] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21602135379] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[21604105479] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[21606483294] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[21612556614] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=74
[21613834143] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=25
[21615634590] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[21624957981] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[21647083821] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[21657424767] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[21661011306] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[21664989654] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[21666171780] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[21674650173] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[21676757883] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[21679973007] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[21685483875] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[21690311049] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=149
[21692718762] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=25
[21696385986] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[21702988692] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[21707043864] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[21709187016] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:56580 on listener 1
[21716489157] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[21735305625] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[21739027629] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=3
[21740138904] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[21749163447] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 37 (user thread) assigned to CPU 1
[21751481565] [INFO] [anther] [CPU1] anther: Thread spawned TID=37 for conn_handle=3
[21780292578] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21830081196] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[21831219696] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21832480956] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21833629224] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=307 pred=0 subj_lo=0
[21850518558] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21853402626] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
T:5EE0 [21895356978] [INFO] [anther] [CPU1] anther: Worker thread TID=37 starting for conn_handle=3
[21936728286] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=33, our_read=34)
[21938588529] [INFO] [anther] [CPU1] anther: Worker TID=37 connected to netd OK
[21940628820] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[21970706340] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[21973820814] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[22007332677] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22112093091] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22118151627] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 85 bytes on conn_handle=3
[22126557684] [INFO] [anther] [CPU1] anther: GET /health Http11
[22142472264] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[22145790084] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[22152408399] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22153564323] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[22154609862] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22156327050] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[22157551284] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22159360608] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=350 pred=0 subj_lo=0
[22162510986] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22169878665] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[22174180743] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22208781441] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[22215998805] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[22237355415] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=fec1d29ef0678173)
[22238560872] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[22242989802] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[22258348398] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[22260171780] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[22262948466] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22266028191] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22270678419] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22273317066] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[22274780583] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[22276566246] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[22281708537] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[22284074241] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[22285988967] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[22334610771] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22335774219] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22336941462] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22338148371] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=351 pred=0 subj_lo=0
[22340885787] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 0 bytes on conn_handle=3
[22345953300] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[22356678267] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[22359153036] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[22370594763] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[22387280850] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12222000
[22388886993] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[22390348431] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[22455013185] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[22464736833] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[22469154477] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[22471430718] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[22473424479] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[22477696527] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[22499082276] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[22500966279] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[22502878629] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[22518054009] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x1222e000
[22519645533] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[22554652461] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[22804073160] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=c1e9a7b90a789e5c)
[23025724029] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[23201807145] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=791 watches=18 history=1024 journal=1024 symbols=363 drops=0
[23238850701] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[23491382475] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[23750870946] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[24018903315] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[24292902282] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[24596633160] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[24751668117] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=823 watches=18 history=1024 journal=1024 symbols=364 drops=0
[24871626318] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[25137902658] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[25212123585] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25431638694] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[25718669922] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[25994402082] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[26313741531] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=862 watches=18 history=1024 journal=1024 symbols=365 drops=0
[26360339247] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[27237094797] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[27465281580] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2429 ops=1 watches=18
[27662522085] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[27873934308] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=883 watches=18 history=1024 journal=1024 symbols=365 drops=0
[28290380316] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[28666155243] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[28912279539] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[29044389990] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[29406901755] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[29568830610] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=921 watches=18 history=1024 journal=1024 symbols=367 drops=0
[29581845315] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30912232527] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[31071098058] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[31112335056] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[31113661821] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[31114813818] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[31116016701] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=352 pred=0 subj_lo=0
[32147600892] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1036 watches=19 history=1024 journal=1024 symbols=445 drops=0
[32435311920] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[32551010547] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[32825282787] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[32826960111] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[32853433470] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[33355209492] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[33400362600] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[33401448465] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[33402561489] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[33404158854] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[33419253450] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[33420379014] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[33421548303] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[33422820519] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=235 pred=0 subj_lo=0
[33430828530] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[33432016959] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[33433251720] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[33434515620] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=453 pred=0 subj_lo=0
[33440850333] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[33442011471] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[33443171157] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[33444373776] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=452 pred=0 subj_lo=0
[33450598863] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[33451702647] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[33452883453] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[33454196094] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[33466344186] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[33885278625] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1072 watches=24 history=1024 journal=1024 symbols=454 drops=0
[35449430346] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1096 watches=24 history=1024 journal=1024 symbols=454 drops=0
[37302532872] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=1129 watches=24 history=1024 journal=1024 symbols=454 drops=0
[39026820690] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=1155 watches=24 history=1024 journal=1024 symbols=454 drops=0
[40549639647] [INFO] [kernel::roo
```
</details>
