# ❌ Scenario: Receiving a fortune cookie message

> Last run: 2026-04-05 18:40:52

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for 60 seconds | ✅ | 60001ms | - [📜](./02/serial.log) - |
| 3 | Then I should see a window at 400, 200 with background color "#FDF5E6" | ❌ | 1014ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11928394368] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11934163527] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11937897345] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11939987136] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11941350828] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11942052144] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11942755836] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11943374751] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11943993534] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11944635747] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11945275320] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11945915124] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11946650397] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11947351020] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11948088372] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11948748372] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11949401904] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11950040421] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11950702764] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11951332899] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11951959239] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11952573897] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11953197333] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11953821165] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11954500338] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11955158589] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11955789615] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11956473408] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11957097966] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11957737539] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11958392655] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11959040016] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11959698366] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11960358498] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11960982825] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11961735489] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11962489308] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11963247021] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11963987970] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11964754131] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11965510689] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11966269920] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11967580218] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11968958595] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11969761782] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11970337962] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11970876258] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11971427754] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11972022876] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11972574636] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11973117189] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11973669015] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11974213020] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11974795635] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11975410029] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11976021849] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11976598161] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11977195164] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11977770189] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11978382306] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11978957991] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11979552387] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11980128204] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11980722798] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11981297955] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11981906508] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11982481962] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11983076556] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11983650162] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11984245548] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11984832123] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11985426783] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11986002072] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11986595313] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11987170701] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11987766516] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11988469317] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11989272240] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11989855614] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11990453112] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11991028335] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11991641904] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11992216797] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11992809741] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11993391927] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11993987511] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11994676683] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11995278636] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11995864617] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11996634507] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11997417564] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11998094229] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11998739808] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11999344632] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11999924244] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[12000521181] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[12001100595] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[12001711326] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[12002289255] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[12002884113] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[12003460491] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[12004054392] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[12004641165] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[12005239323] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[12005818341] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[12006410658] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[12006985221] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[12007875627] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12260232006] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[12271297203] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12276362340] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12277738506] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12278667357] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12282905580] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12284659299] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12285789285] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12286510698] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12287217426] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12287936727] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12288941214] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12289959924] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12290691567] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12291423210] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12292137363] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12292850295] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12294029187] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12295116240] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12295851018] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12297449901] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12298439604] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12299471151] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12301118478] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12302827779] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12303646377] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12304245789] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12305036766] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12695037696] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12696106995] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12699637731] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12700607106] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12701432601] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12703068015] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12715796610] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12717201519] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12717994476] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12719752914] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12720331536] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12722860194] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12730861176] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12733060791] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12748114368] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12748700448] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12765963045] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12766562160] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12768694356] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12769941096] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12771035376] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12773307954] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12774167043] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12809072529] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62287300 ticks/sec), init_cnt=622873 for 100Hz
[12810457671] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12811298775] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12812520831] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12818221548] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12848719323] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12849653487] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12851626128] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12853825479] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12855777231] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12860359314] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12862270047] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12879781728] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12882180201] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12883066350] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12884729550] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12885626787] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12887490693] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12888776736] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12915088626] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12917456673] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12918353547] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12919863000] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12920752944] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12922128219] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12922764624] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12924060831] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12931349178] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12932278590] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12934159458] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12935042703] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12941794932] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12944433249] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12945695433] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12947908215] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12949480599] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12951161289] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12971565849] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12976488129] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12978527595] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12979982334] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13037638020] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13039899774] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13043161989] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13044805983] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13047089253] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13049471523] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13051423341] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13052478516] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13053757398] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13063807284] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13066719171] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13070911095] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13074080382] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13078309530] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13079119383] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13080430935] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13082652297] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13092454716] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13093250247] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13099855659] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13100687787] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13127937075] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13128713565] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13471747047] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13964943828] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13995235122] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=499 journal=424 symbols=51 drops=0
[14028224496] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[15302995554] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=449 watches=0 history=963 journal=772 symbols=97 drops=0
[16006754280] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[16100515959] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[16101667296] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[16205959737] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[16266812925] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[16290818082] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[16291681560] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[16292354265] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[16297451643] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[16313365695] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[16329899652] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[16334342277] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[16390464123] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[16411922736] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[16412969793] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[16418533560] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[16445378631] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[16466642313] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[16471793976] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[16473320919] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[16544992992] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[16546811325] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[16634400486] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[16701972111] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[16712814030] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[16716625629] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[16718895435] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[16778677509] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=184 drops=0
[16785955230] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[16791707526] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[16793196123] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[16794546813] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[16795849290] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[16797029436] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[16798117413] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[16799199582] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[16800228093] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[16801258056] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[16802328081] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[16803363192] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[16804452225] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[16805555382] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[16806717807] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[16807945275] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[16809029721] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[16810150104] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[16811204025] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[16812292167] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[16813344306] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[16814473401] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[16815511185] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[16816555140] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[16817630478] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[16818747660] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[16820188836] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[16821237048] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[16822565595] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[16823846556] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[16825263147] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[16826348814] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[16827455469] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[16828517937] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[16829588919] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[16830651255] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[16831873278] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[16833104376] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[16834371873] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[16835595348] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[16836890499] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[16838135622] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[16839386949] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[16841307219] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[16843562109] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[16844841684] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16854230844] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[16870043091] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[16874956032] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[16884844548] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[16885990011] [CONTRACT] [kernel] [CPU0] Spawning init process...
[16888570941] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[16893015249] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[16894166091] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [16899412398] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013360 RFLAGS_BEFORE=130 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[16903878816] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[16915708623] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[16920681690] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[16922229885] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[16923550116] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[16932058638] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[16937281713] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[16941238380] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[16942587552] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[16946950515] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[16950555336] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[16951984434] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[16956509460] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[16958112996] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[16959465435] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[16960706697] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16964660064] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16966017981] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16967573271] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16969826577] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16971580494] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16973110374] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16977896694] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[17019632190] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[17027206251] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[17033251983] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[17039099220] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[17042968107] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[17047430070] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[17053663902] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[17059699206] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[17065517238] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[17071533369] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[17077711827] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[17084349150] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[17090648058] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[17096456982] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[17102381439] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[17108453274] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[17115073767] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[17121497646] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[17127361944] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[17133285609] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[17139249270] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[17145456240] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[17151988821] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[17158379898] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[17164369761] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[17170475652] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[17176477461] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[17182512897] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[17188730163] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[17194592184] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[17198826546] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[17202838026] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[17208857424] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[17214799470] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[17221055049] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[17227197867] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[17233283265] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[17239399914] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[17245965990] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[17252958558] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[17260015641] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[17264062332] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[17287596711] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[17456784972] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[17464083582] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[17465012664] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17466962436] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17471855115] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17473367571] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17482377825] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[17487594663] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[17488984854] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17491106391] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078896 RFLAGS_BEFORE=130 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[17497712430] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[17498827995] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[17500102323] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17502657942] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17508166731] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[17510543952] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17518732209] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[17522512227] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[17524074513] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[17526319899] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144432 RFLAGS_BEFORE=130 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[17531168160] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[17540847423] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[17543520126] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[17547131019] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17551856487] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:42:25 = 1775439745 unix_secs
[17553577800] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775439745, mono_ns=8776558263, offset=1775439736223441737ns
[17555298849] [INFO] [rtc_cmos] [CPU1] System clock anchored
[17566663290] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[17615401914] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[17632032957] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[17633316129] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17636207457] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17645536854] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[17648809629] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[17659311714] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[17664328539] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[17672011995] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[17674814982] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[17676049644] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17678471646] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17688219516] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17691358476] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17701336719] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[17704924215] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00faa60
[17706340971] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17708113038] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369276784 RFLAGS_BEFORE=130 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[17712300408] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[17714579883] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17716722408] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[17717775636] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210544 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[17725483446] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[17729719854] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[17732211222] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[17734382886] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[17736154854] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[17741366346] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[17743563156] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[17765670582] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[17768098029] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[18306750297] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18310980666] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[18311868894] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18313504044] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18320741868] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18323111631] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18330298008] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[18333747729] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[18335181843] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[18337507023] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352736 RFLAGS_BEFORE=130 CR3_BEFORE=59662336 fs_base=0 gs_base=18446744071564586576
[18343930110] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[18346550013] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[18348429000] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[18351123549] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[18358385529] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[18360352164] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[18361994112] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[18365461455] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[18367485345] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[18369117855] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[18373272984] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[18374796561] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[18376519194] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[18378420951] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[18379784181] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[18381681153] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[18382835262] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[18384598782] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[18385949274] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[18393443541] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[18402398388] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[18403758714] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[18429289164] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[18433510260] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[18437532102] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[18438720795] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[18440539359] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[18441756102] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18444211599] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18461053776] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18464041002] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18476839920] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[18481236081] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[18484438401] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[18485822619] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18488444502] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18496851384] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18499279326] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18511434513] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[18515950497] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[18517316961] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[18519431337] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369499968 RFLAGS_BEFORE=130 CR3_BEFORE=68235264 fs_base=0 gs_base=18446744071564586640
[18523979859] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[18525123144] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18527497263] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18528706020] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[18537530187] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18541795041] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18559079352] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[18560630121] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[18564416706] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[18567487950] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[18569506824] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[18570720960] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[18573219918] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[18574276149] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1236 port=2 model='                                        ' rpc_port=5
[18576506751] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[18579385506] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[18581519055] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[18583617162] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[18586403418] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[18588432720] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[18590385363] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[18617640360] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[18619183275] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[18620621613] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[18622100013] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[18623544489] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[18625103772] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[18626627217] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[18628395126] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[18637688421] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[18640271463] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369433728 RFLAGS_BEFORE=130 CR3_BEFORE=59830272 fs_base=0 gs_base=18446744071564586608
[18648861858] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[18977790810] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
[18979262874] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[18981456813] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369586848 RFLAGS_BEFORE=134 CR3_BEFORE=68349952 fs_base=0 gs_base=18446744071564586576
[18986042361] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[18987800106] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[18990097005] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[18991087170] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[18993033147] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[18997611633] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19001768115] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[19003087653] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19005736794] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19010067846] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[19016633856] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[19020630288] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[19032343242] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[19037224602] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[19040148171] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[19042402401] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[19043574528] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19046039694] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19076991219] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19083423645] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[19133226123] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[19876394010] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[19877888844] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[19880136870] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[19882210491] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[20003736621] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[20007530334] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[20008968804] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20010685728] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369718976 RFLAGS_BEFORE=134 CR3_BEFORE=68882432 fs_base=0 gs_base=18446744071564586640
[20017779771] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[20020109802] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[20021303940] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653440 RFLAGS_BEFORE=134 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[20026124250] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[20028969642] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[20030720952] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[20035618152] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[20037426453] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[20041093578] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[20042676951] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[20044253823] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[20046439941] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[20061828369] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[20069800410] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[20071780806] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[20074315239] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[20076135717] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[20078585637] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[20080390836] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[20082834717] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4c32000
[20084782113] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[20087451879] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[20090106432] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[20091986508] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[20103240465] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[20113419777] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[20124206190] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[20127347889] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[20130004455] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[20132128665] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[20134090482] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[20136180669] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[20137830108] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[20139270723] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[20140513767] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[20146309359] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=15, read=16)
[20152209825] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=17, read=18)
[20162389698] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20186066736] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=15, RX port=18
[20564778960] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[20566565184] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[20568340089] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[20569304910] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20571423873] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20579200587] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20605672065] [INFO] [netd] [CPU3] NETD: Driver TX port=15, RX port=18, link_up=true, mtu=1500
[20608924083] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[20615137257] [INFO] [netd] [CPU3] NETD: Created socket API port (write=19, read=20)
[20643189963] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[20646295296] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([219, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[20664461565] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[20666044542] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[20669202840] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[20674550589] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[20677031331] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[20679673245] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[20681073204] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20682778710] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369950512 RFLAGS_BEFORE=134 CR3_BEFORE=80007168 fs_base=0 gs_base=18446744071564586576
[20687456955] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[20688554964] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20691081081] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20692160346] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[20696244129] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[20707573359] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=19, our_write=21, our_read=22)
[20710485048] [INFO] [anther] [CPU1] anther: Connected to network stack
[20718934203] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[20729936106] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20733778659] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[20742959259] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[20745835803] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[20748038751] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[20748970110] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20750728218] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20757094578] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20759852652] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20767619928] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[20770732851] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
[20771650878] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20773959723] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370099312 RFLAGS_BEFORE=130 CR3_BEFORE=81371136 fs_base=0 gs_base=18446744071564586640
[20778283845] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[20779261536] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20781096303] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20786937336] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[20789239053] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20790169620] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[20793185259] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[20801219868] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[20804249268] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
[20806271376] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0102438
[20807774526] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20809315593] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20810701164] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370165840 RFLAGS_BEFORE=134 CR3_BEFORE=81600512 fs_base=0 gs_base=18446744071564586576
[20815070958] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20825729628] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[20829252840] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[20836982397] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[20839956885] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[20842388688] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[20843374431] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20845344168] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20871777828] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20877049281] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[20885469099] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[20888681121] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
[20890322673] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[20891796420] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[20892802260] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20894908914] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20895857103] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370297888 RFLAGS_BEFORE=130 CR3_BEFORE=81903616 fs_base=0 gs_base=18446744071564586640
[20905095090] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[20913113463] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[20916463095] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20924291619] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[20927787078] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[20929540797] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[20930459814] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20932513338] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f2298
[20933956659] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20935754400] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370382176 RFLAGS_BEFORE=134 CR3_BEFORE=82178048 fs_base=0 gs_base=18446744071564586576
[20942809701] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[20944985457] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[20953606212] [INFO] [fontd] [CPU3] FONTD: Service node created, req=23, resp=26
[20954782068] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20957465100] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20959321350] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20961326661] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[20965976427] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20967532245] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20969334210] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20971214220] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=233 pred=0 subj_lo=0
[20974851810] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[20985491340] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[20993353689] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[20996052165] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[20999970222] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[21002925669] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21004686219] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370033264 RFLAGS_BEFORE=130 CR3_BEFORE=80728064 fs_base=0 gs_base=18446744071564586608
[21009497190] [INFO] [nectar] [CPU2] NECTAR: Started.
[21011548602] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1180
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21013885728] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21015468705] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370232096 RFLAGS_BEFORE=130 CR3_BEFORE=81747968 fs_base=0 gs_base=18446744071564586608
[21018948621] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21020530839] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[21021396561] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21023266209] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=237 pred=0 subj_lo=0
[21026370519] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010c010
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21028107276] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370449456 RFLAGS_BEFORE=134 CR3_BEFORE=82386944 fs_base=0 gs_base=18446744071564586608
[21031362066] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21032954415] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21034547325] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21036295203] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[21037299591] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=238 subj_lo=0
[21039108750] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[21040845738] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[21044513622] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[21048599022] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21050234172] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21052048347] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21053939346] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=241 pred=0 subj_lo=0
[21059697120] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21061350453] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21063194922] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21065186010] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[21067238016] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1251) for kind 'Asset'
[21069538743] [INFO] [fontd] [CPU3] FONTD: Service ready
[21075610908] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[21078328425] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[21080720595] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21082367691] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21084225228] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21086152824] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[21120401808] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21122116257] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21123880503] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21125630724] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=248 subj_lo=0
[21127565976] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[21137293683] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[21144707331] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21146309151] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21148154775] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21150085902] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=250 subj_lo=0
[21156443880] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[21161080380] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[21166596693] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[21168350610] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[21170325297] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21171546462] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[21172709118] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21174080004] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21175428054] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=251 pred=0 subj_lo=0
[21180255030] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[21191137440] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1258 backend=VirtIO-GPU
[21192914820] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[21193824102] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21195582606] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21214878036] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[21229845615] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=27, resp=30
[21231077835] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[21256572843] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[21261701406] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[21264293787] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[21288905352] [INFO] [netd] [CPU3] NETD: DHCP configured �� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[21293561784] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[21300098688] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[21305477655] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[21310569258] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[21335842011] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[21350522160] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[21358171989] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[21360728400] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f4208
[21362107602] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4ea
[21363907059] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370554352 RFLAGS_BEFORE=134 CR3_BEFORE=83570688 fs_base=0 gs_base=18446744071564586640
[21368223162] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[21369120597] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21370788417] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21371672058] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[21375447852] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[21377030367] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21385927134] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[21388464570] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f4208
[21389631450] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[21391036854] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370619888 RFLAGS_BEFORE=134 CR3_BEFORE=84619264 fs_base=0 gs_base=18446744071564586576
[21395112816] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[21395981046] [INFO] [echo] [CPU1] echo: starting up
[21398056251] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[21404968365] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[21406157652] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[21415210443] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[21421694481] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[21425109882] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[21446362344] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[21448203051] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[21450088539] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[21450838332] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21452228424] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[21453652308] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21455090415] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[21458855880] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21459980883] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[21461462451] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[21464048958] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[21469319553] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[21472003278] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[21474216984] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[21476319414] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[21478053663] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[21478951428] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21480924696] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21487531758] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[21489076092] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21490059162] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[21493324776] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[21495966294] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[21501517422] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[21504227514] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[21505071192] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f41c0
[21506404590] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21508179429] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[21509075841] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370755056 RFLAGS_BEFORE=134 CR3_BEFORE=85118976 fs_base=0 gs_base=18446744071564586640
[21513239682] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21515135961] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21518370654] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[21519710487] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[21527412918] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[21529206072] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[21530273490] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[21536032683] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[21537695652] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[21538521675] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[21541028157] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[21550388277] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db418
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21552968646] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370824688 RFLAGS_BEFORE=134 CR3_BEFORE=85266432 fs_base=0 gs_base=18446744071564586576
[21560677347] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[21610236087] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[21642673800] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[21710006505] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[21714181467] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[21722506542] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[21758448294] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[21776063793] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[21784953828] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[21814811601] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[21818004450] [INFO] [bloom] [CPU3] bloom: creating surface...
[21819401307] [INFO] [bloom] [CPU3] bloom: surface created!
[21820638213] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[21838903779] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[21857112915] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1267
[21858615405] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[21880205886] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[21884995374] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[21886491561] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[21887978178] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [21906367098] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[21919803312] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[21927560919] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 5)
[21933362814] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[21945674454] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[21966715848] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1267
T:07D0 T:0640 T:F0B0 [22018740711] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[22036696308] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[22050241587] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[22053731634] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[22056655401] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[22059672129] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[22072339773] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[22074863448] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
T:1220 [22083292275] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[22085013192] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[22099925760] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=284
[22101436104] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[22102547379] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22103782074] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22105031157] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22106427816] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=284 subj_lo=0
[22120417836] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1279)
[22122404865] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[22134273117] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=251
[22135775112] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[22137114153] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22138175664] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22139363169] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22140697821] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=251 pred=0 subj_lo=0
[22155652695] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1282)
[22156794033] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[22170190812] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=287
[22171879620] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[22172909517] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22174039998] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22175194338] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22176647328] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=287 subj_lo=0
[22191261048] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1283)
[22192796340] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[22372689669] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[22554927120] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=645 watches=13 history=1024 journal=1024 symbols=306 drops=0
[22620988599] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[22645580199] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[22650866865] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[22843576107] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[23005438368] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[23177732556] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[23592642381] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[23800898847] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[23914448844] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[23996215914] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[24218959908] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[24437731461] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[24540856131] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[24547446660] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[24549076761] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[24550108968] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[24551414679] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[24553099230] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[24563473836] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[24564823602] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[24566330151] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[24567833202] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=336 pred=0 subj_lo=0
[24653137707] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[24800296554] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=718 watches=15 history=1024 journal=1024 symbols=337 drops=0
[24851847504] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[25073977170] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[25270497813] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[25407185067] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[25409342442] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[25583022069] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=0e57f834b3afbcb1)
[25610503413] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[25612195059] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25613768697] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25615553931] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=348 pred=0 subj_lo=0
[25776215322] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[25920277944] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[25984368333] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[25999311624] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26001019308] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26002669308] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26004543675] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=349 pred=0 subj_lo=0
[26011012731] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[26015593065] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[26124017403] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[26194012548] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[26304688740] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=c1e9a7b90a789e5c)
[26354076969] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[26383237353] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x122bd000
[26385560883] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[26387625231] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[26487557019] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[26502662703] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[26509936365] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[26513085588] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[26516500758] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[26522963775] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[26597040360] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[26599096392] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[26600904792] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[26621803098] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x122ce000
[26624152302] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[26628276840] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[26899752693] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[27254415573] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[27656983794] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[27907221210] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=793 watches=17 history=1024 journal=1024 symbols=363 drops=0
[28024757112] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[28359547128] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[28396192572] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[28542635649] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[28570749537] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[28572035811] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[28573390857] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28574835960] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=350 pred=0 subj_lo=0
[28667757888] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[28669132767] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[28670426103] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28671908496] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=351 pred=0 subj_lo=0
[28795402548] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[29229008028] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[29674810803] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[29715698760] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[29717156535] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=2 (16384b)
[29718815247] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[29720441190] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[30177750141] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[30721418904] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[31309098579] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[32018188719] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=878 watches=19 history=1024 journal=1024 symbols=366 drops=0
[32041413261] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[33800033091] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[34364226732] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[34462357116] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=900 watches=19 history=1024 journal=1024 symbols=366 drops=0
[35061219336] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2445 ops=1 watches=19
[35144895489] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[35629961301] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[35918449941] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[36043673919] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[36106857468] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[36363376269] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[36443842842] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[36691844013] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=943 watches=19 history=1024 journal=1024 symbols=369 drops=0
[39671512980] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=1039 watches=19 history=1024 journal=1024 symbols=439 drops=0
[40311218772] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[40411755648] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[40638090108] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[40639625202] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[40667465157] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[41281690593] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[41317370457] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[41318539482] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[41319840474] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[41321144172] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[41327360778] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[41328455421] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[41329692162] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[41331071727] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=240 pred=0 subj_lo=0
[41337245037] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[41338368324] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[41339959386] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[41341731123] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=453 pred=0 subj_lo=0
[41355882249] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[41357397312] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[41359109418] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[41360843271] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=452 pred=0 subj_lo=0
[41370236688] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[41371847946] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[41373636249] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[41375380398] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[41384802657] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[42009083589] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1088 watches=24 history=1024 journal=1024 symbols=454 drops=0
[44401732914] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1114 watches=24 history=1024 journal=1024 symbols=454 drops=0
[44468322624] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[46277841126] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1138 watches=24 history=1024 journal=1024 symbols=454 drops=0
[47191703691] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[48901138836] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=1177 watches=24 history=1024 journal=1024 symbols=454 drops=0
[50698730016] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=1199 watches=24 history=1024 journal=1024 symbols=454 drops=0
[52710232245] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=16000 nodes=1229 watches=24 history=1024 journal=1024 symbols=454 drops=0
[53118653643] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=3469 ops=1 watches=24
[54246794646] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[55761542331] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=17000 nodes=1272 watches=24 history=1024 journal=1024 symbols=454 drops=0
[57822107244] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[57989983722] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=18000 nodes=1296 watches=24 history=1024 journal=1024 symbols=454 drops=0
[58950796905] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=2
[61400008494] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=19000 nodes=1349 watches=24 history=1024 journal=1024 symbols=454 drops=0
[64110596319] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=20000 nodes=1371 watches=24 history=1024 journal=1024 symbols=454 drops=0
[66623611296] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=21000 nodes=1399 watches=24 history=1024 journal=1024 symbols=454 drops=0
[69519968760] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=22000 nodes=1427 watches=24 history=1024 journal=1024 symbols=454 drops=0
[72337692570] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=23000 nodes=1464 watches=24 history=1024 journal=1024 symbols=454 drops=0
[74588226108] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=24000 nodes=1488 watches=24 history=1024 journal=1024 symbols=454 drops=0
[76591629543] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=25000 nodes=1510 watches=24 history=1024 journal=1024 symbols=454 drops=0
[78746147205] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=26000 nodes=1538 watches=24 history=1024 journal=1024 symbols=454 drops=0
[80837852865] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=27000 nodes=1573 watches=24 history=1024 journal=1024 symbols=454 drops=0
[83059621656] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=28000 nodes=1601 watches=24 history=1024 journal=1024 symbols=454 drops=0
[85798987593] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=29000 nodes=1633 watches=24 history=1024 journal=1024 symbols=454 drops=0
[86502626727] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=4493 ops=1 watches=24
[88681157862] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[89616381195] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=30000 nodes=1684 watches=24 history=1024 journal=1024 symbols=454 drops=0
[92062742178] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=31000 nodes=1714 watches=24 history=1024 journal=1024 symbols=454 drops=0
[94073593317] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=32000 nodes=1740 watches=24 history=1024 journal=1024 symbols=454 drops=0
[96785933508] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=33000 nodes=1776 watches=24 history=1024 journal=1024 symbols=454 drops=0
[99211275075] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=34000 nodes=1809 watches=24 history=1024 journal=1024 symbols=454 drops=0
[101275281393] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=35000 nodes=1837 watches=24 history=1024 journal=1024 symbols=454 drops=0
[103946742834] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=36000 nodes=1867 watches=24 history=1024 journal=1024 symbols=454 drops=0
[106307016222] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=37000 nodes=1902 watches=24 history=1024 journal=1024 symbols=454 drops=0
[109031645481] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=38000 nodes=1938 watches=24 history=1024 journal=1024 symbols=454 drops=0
[111774790446] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=39000 nodes=1962 watches=24 history=1024 journal=1024 symbols=454 drops=0
[114257741202] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=40000 nodes=1990 watches=24 history=1024 journal=1024 symbols=454 drops=0
[117768189363] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=41000 nodes=2041 watches=24 history=1024 journal=1024 symbols=454 drops=0
[119999602833] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=5517 ops=1 watches=24
[120633150363] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=42000 nodes=2073 watches=24 history=1024 journal=1024 symbols=454 drops=0
[123242955660] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=43000 nodes=2097 watches=24 history=1024 journal=1024 symbols=454 drops=0
[126726082791] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=44000 nodes=2140 watches=24 history=1024 journal=1024 symbols=454 drops=0
[129207865083] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=45000 nodes=2176 watches=24 history=1024 journal=1024 symbols=454 drops=0
[132134960529] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=46000 nodes=2202 watches=24 history=1024 journal=1024 symbols=454 drops=0
[134372840646] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=47000 nodes=2237 watches=24 history=1024 journal=1024 symbols=454 drops=0
[137287972041] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=48000 nodes=2261 watches=24 history=1024 journal=1024 symbols=454 drops=0
[140552555649] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=49000 nodes=2299 watches=24 history=1024 journal=1024 symbols=454 drops=0
[143465124924] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=50000 nodes=2329 watches=24 history=1024 journal=1024 symbols=454 drops=0
[146207754462] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=51000 nodes=2364 watches=24 history=1024 journal=1024 symbols=454 drops=0
[148873537626] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=52000 nodes=2390 watches=24 history=1024 journal=1024 symbols=454 drops=0
[151619378691] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=53000 nodes=2426 watches=24 history=1024 journal=1024 symbols=454 drops=0
[154195611141] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=54000 nodes=2452 watches=24 history=1024 journal=1024 symbols=454 drops=0
[155686717395] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=6541 ops=1 watches=24
[157676772495] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=55000 nodes=2493 watches=24 history=1024 journal=1024 symbols=454 drops=0
[160563226329] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=56000 nodes=2527 watches=24 history=1024 journal=1024 symbols=454 drops=0
[162975440397] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=57000 nodes=2553 watches=24 history=1024 journal=1024 symbols=454 drops=0
[166116427224] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=58000 nodes=2588 watches=24 history=1024 journal=1024 symbols=454 drops=0
[170380953567] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=59000 nodes=2628 watches=24 history=1024 journal=1024 symbols=454 drops=0
[173229819642] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=60000 nodes=2654 watches=24 history=1024 journal=1024 symbols=454 drops=0
[176943998022] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=61000 nodes=2686 watches=24 history=1024 journal=1024 symbols=454 drops=0
[180097249992] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=62000 nodes=2721 watches=24 history=1024 journal=1024 symbols=454 drops=0
[182555772036] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=63000 nodes=2747 watches=24 history=1024 journal=1024 symbols=454 drops=0
[185063342178] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=64000 nodes=2781 watches=24 history=1024 journal=1024 symbols=454 drops=0
[188201630001] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=65000 nodes=2824 watches=24 history=1024 journal=1024 symbols=454 drops=0
[191195850087] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=66000 nodes=2846 watches=24 history=1024 journal=1024 symbols=454 drops=0
[194916492672] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=67000 nodes=2888 watches=24 history=1024 journal=1024 symbols=454 drops=0
[195210264381] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=7565 ops=1 watches=24
[198562530309] [INFO]
```
</details>
