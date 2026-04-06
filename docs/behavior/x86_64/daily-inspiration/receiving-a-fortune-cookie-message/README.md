# ❌ Scenario: Receiving a fortune cookie message

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for 60 seconds | ✅ | 60001ms | - [📜](./02/serial.log) - |
| 3 | Then I should see a window at 400, 200 with background color "#FDF5E6" | ❌ | 1012ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11155247070] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11160533076] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11164487070] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11166452715] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11167630848] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11168263458] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11168898642] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11169456210] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11170045227] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11170853298] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11171669289] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11172269955] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11172971403] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11173617609] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11174286156] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11174892399] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11175493131] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11176070400] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11176667073] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11177243121] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11177823591] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11178396339] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11178977469] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11179547049] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11180154843] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11180727096] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11181316674] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11181932982] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11182494576] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11183067159] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11183650038] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11184237933] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11184854406] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11185441443] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11186007327] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11186673729] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11187346863] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11188048146] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11188716264] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11189412597] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11190088965] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11190769920] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11192031246] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11193282078] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11194012599] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11194567263] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11195057577] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11195557461] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11196080379] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11196581616] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11197181358] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11197716585] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11198211717] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11198741565] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11199270093] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11199810699] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11200334475] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11200889106] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11201414928] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11201956689] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11202478782] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11203021137] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11203546035] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11204104923] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11204630877] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11205173562] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11205698955] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11206242003] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11206766934] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11207309553] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11207845770] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11208383934] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11208907248] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11209447524] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11209969518] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11210510916] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11211048981] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11211589653] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11212111713] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11212651758] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11213175567] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11213716272] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11214251400] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11214791742] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11215315419] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11215855431] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11216377524] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11217038976] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11217683400] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11218237404] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11218760520] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11219300697] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11219823021] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11220362571] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11220899547] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11221438767] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11221962642] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11222503281] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11223025869] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11223573801] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11224109952] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11224697022] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11225228289] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11225767344] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11226288513] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11226827139] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11227364016] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11228096847] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11458573269] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[11469036777] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11473822965] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11475090726] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11475942456] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11480170218] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11481848961] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11482898328] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11483560506] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11484248391] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11484903408] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11485863576] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11486786685] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11487534597] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11488333791] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11489002074] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11489663790] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11490831891] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11491833078] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11492531160] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11494078695] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11495048004] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11496155022] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11497811721] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11499441063] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11500209798] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11500793931] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11501556330] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[11859820533] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[11860824426] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[11864024436] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[11864871447] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[11865614508] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[11867122575] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[11878903344] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[11880263175] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[11881005477] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[11882581128] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[11883161565] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[11885619141] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[11893181685] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[11894828715] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[11907509724] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[11908069701] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[11923493208] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[11924043582] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[11926055625] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[11927211846] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[11928218841] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[11930392056] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[11931172077] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[11966101059] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62393800 ticks/sec), init_cnt=623938 for 100Hz
[11967601041] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[11968429770] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[11969581371] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[11975096166] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12004956942] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12005861868] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12008788440] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12010949940] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12012129591] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12014822688] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12016113351] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12034494813] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12036230481] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12038050893] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12038743035] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12039420756] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12039999015] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12040625388] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12066514581] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12068617242] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12069910842] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12070895034] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12072158769] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12073154940] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12074462829] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12075221499] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12081635973] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12082828725] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12084930858] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12086167929] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12091307481] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12092907024] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12093620880] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12094831716] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12095708493] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12096526299] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12108662940] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12111519519] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12112597959] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12113407284] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12134442870] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12153104898] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12155080047] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12158120139] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12159637578] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12161646387] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12163867617] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12165816465] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12166841544] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12167841312] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12173547375] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12175092732] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12179183808] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12181315707] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12183675636] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12184430082] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12195082086] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12195897318] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12202348059] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12203117553] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12228619755] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12229418916] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[12551131725] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13002334125] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13028302683] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[13062388020] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[14261548158] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=446 watches=0 history=959 journal=768 symbols=93 drops=0
[14969426043] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15062984970] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15063995496] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[15166363542] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[15223835088] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[15249856413] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[15252443151] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[15253064079] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[15256664313] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[15271837053] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[15289292898] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[15291618837] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[15341539653] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[15362164620] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[15363196002] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[15367339713] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[15391085556] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[15409354818] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[15415697022] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[15416660457] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[15483692598] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[15485661609] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[15578096589] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[15645929178] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[15658906725] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[15663270150] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[15665384295] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[15700684395] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[15737185431] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[15743008281] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[15744194235] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[15745005936] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[15745874991] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[15746570202] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[15747202614] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[15747895977] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[15748487007] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[15749103942] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[15750098133] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[15750961677] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[15751578381] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[15752212641] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[15752978274] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[15753683352] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[15754316490] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[15754954017] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[15755563032] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[15756206763] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[15756961671] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[15757697736] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[15758302692] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[15758911542] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[15759548739] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[15760205505] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[15760827753] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[15761446140] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[15762115281] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[15762746703] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[15763370403] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[15764002650] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[15764653872] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[15765279717] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[15765912459] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[15766541901] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[15767266746] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[15767999346] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[15768736665] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[15769470090] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[15770215692] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[15770989707] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[15771724518] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[15773240802] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[15774969045] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[15775743522] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[15784013190] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[15797844381] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[15802149693] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[15811275018] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[15811995309] [CONTRACT] [kernel] [CPU0] Spawning init process...
[15814191327] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[15817224225] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[15818935539] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
ThingOS Petals
type 'help' for commands

USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
petals> [15823121622] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[15824634639] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013360 RFLAGS_BEFORE=130 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[15844834170] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[15847204989] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[15848372694] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[15850097571] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[15859369350] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[15863882892] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[15867011094] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[15868196520] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[15872153847] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[15875874135] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[15876972243] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[15881224656] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[15882482088] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[15883564191] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[15884668404] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[15888159705] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[15889710738] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[15890907417] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[15892448715] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[15893850654] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[15895223124] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[15899940738] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[15939936507] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[15948617718] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[15954625731] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[15960378027] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[15963881142] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[15968024853] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[15973579413] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[15979200765] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[15984425886] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[15989127330] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[15993834549] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[15998611101] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[16003450485] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[16008689301] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[16015104534] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[16022784096] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[16029967734] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[16036284528] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[16043046921] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[16049687940] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[16054445946] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[16059314601] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[16064481510] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[16070891133] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[16075953597] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[16081894488] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[16087600617] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[16093603647] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[16099120785] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[16104917202] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[16109068140] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[16113128361] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[16118922138] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[16124114193] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[16129811940] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[16135958025] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[16141965114] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[16148162976] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[16154345196] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[16160318460] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[16166320665] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[16169393691] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[16190957079] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[16322732052] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[16329362643] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[16330254072] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16331972943] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16336787412] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[16338080352] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16346105853] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[16352794260] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16353893820] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16355470560] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078896 RFLAGS_BEFORE=130 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[16359523257] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[16360410891] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16361990568] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[16362884538] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16368388674] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[16370313135] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16377770838] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[16380983949] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[16382427567] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[16384125549] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144432 RFLAGS_BEFORE=130 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[16387415616] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[16390684497] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[16392586617] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[16394796462] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[16399421115] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:50:21 = 1775436621 unix_secs
[16401282348] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775436621, mono_ns=8200420437, offset=1775436612799579563ns
[16403204367] [INFO] [rtc_cmos] [CPU1] System clock anchored
[16417138881] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[16463826159] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[16465325217] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16468929378] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16470074181] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[16475507367] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[16478106084] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[16485699582] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[16489001001] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[16492328424] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16494042279] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210768 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[16499127447] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[16503358311] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[16505205684] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[16506083121] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16508187729] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16515402552] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[16517850591] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[16525133625] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[16528569981] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[16529463390] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16531147215] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277360 RFLAGS_BEFORE=130 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[16534419825] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[16536281751] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[16538422395] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[16543073151] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[16544620752] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[16546469412] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[16547637183] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[16551748983] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[16570005441] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[16571856510] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[17036028999] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17046863064] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[17049984105] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[17051806860] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[17056209225] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[17057899650] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[17059893015] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[17061541167] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[17062512654] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[17064882021] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[17065994979] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[17067041508] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[17067993723] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[17068902642] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[17069810439] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[17070848784] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[17080478844] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[17081442114] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17083170357] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17090210610] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17092688679] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17099719428] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[17102537793] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fab68
[17104384407] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[17106370578] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369356912 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[17113326615] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[17120422242] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[17131885155] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[17141241810] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[17143482576] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[17148787821] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[17157058149] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[17158428639] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[17161387287] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[17162428008] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[17163200802] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[17164224726] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[17165075895] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[17165891094] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[17166855849] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[17167654185] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[17169302370] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[17605156602] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[17608200621] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[17610750399] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[17612317074] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[17612996973] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17614496229] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17617940076] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[17619622911] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17627200140] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[17630168523] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[17632090014] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[17632795224] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17634209241] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17638609032] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17640062583] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17646963675] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[17649019608] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
[17649884835] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[17652358779] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500464 RFLAGS_BEFORE=134 CR3_BEFORE=68513792 fs_base=0 gs_base=18446744071564586640
[17655984390] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[17657069232] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17659308084] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[17660176512] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[17661418632] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17662305111] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[17667810105] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17668726251] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[17670684009] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17671559103] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[17674003743] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[17676949554] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[17679959847] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[17681392905] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[17682336474] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[17684279217] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[17685917733] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
[17688091839] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[17689691943] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[17690563308] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583408 RFLAGS_BEFORE=134 CR3_BEFORE=68894720 fs_base=0 gs_base=18446744071564586576
[17695213998] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[17696949171] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[17697650553] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[17698565412] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[17699477235] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[17701507824] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[17702258673] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[17703747600] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434672 RFLAGS_BEFORE=134 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[17708756670] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[17714230347] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1241
[17714965026] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1240)
[17716225461] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[17717725146] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[17721262680] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17723389926] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[17724095334] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17725612245] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17732054241] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[17734397802] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[17741189004] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[17743789569] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[17745591831] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[17746925790] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[17747573250] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17748912819] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17767506801] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[17771424891] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[17789991582] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[17792540469] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
[17793215418] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17796331971] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716480 RFLAGS_BEFORE=134 CR3_BEFORE=69156864 fs_base=0 gs_base=18446744071564586640
[17799805452] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[17801305929] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650496 RFLAGS_BEFORE=130 CR3_BEFORE=69021696 fs_base=0 gs_base=18446744071564586608
[17804084364] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[17805126042] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[17823640824] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[17825265513] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[17828624847] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[17830069917] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[17835185148] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[17836847094] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[17838108717] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[17840101917] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[17844116235] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[17845628427] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[17847420327] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[17848306410] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17850006999] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17915034687] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[17938759113] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[17945652285] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[17948368317] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[17950287795] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17951610831] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369782960 RFLAGS_BEFORE=130 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[17954599113] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[17955292245] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17956850967] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17957534595] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[17960169678] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[17963712327] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[17965018302] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[17981820054] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[17984835000] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[17991296961] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[17993722032] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[17995465092] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[17996166507] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17997618804] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18002830197] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18005065947] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18011540811] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[18013968192] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010aff8
[18015873612] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18017641455] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915120 RFLAGS_BEFORE=134 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[18021218358] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18022233933] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18023962803] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18027568317] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[18029032824] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[18033349257] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18036835476] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18043566684] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[18046523946] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010aff8
[18047885427] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18049917699] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981616 RFLAGS_BEFORE=130 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[18054217434] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[18055529712] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18057216771] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[18058348968] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18059634615] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[18061834230] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18064082157] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[18072357303] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18076478376] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[18083264298] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[18086155593] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[18088288977] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[18089115957] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18090680784] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18116971389] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18121992702] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[18129117798] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[18131775519] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cc18
[18133632264] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18135349353] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113664 RFLAGS_BEFORE=130 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[18138576555] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[18139293777] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18140444223] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[18141323277] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18155642373] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[18158208024] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[18159038205] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18160401996] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18161374869] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18162087042] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18163675332] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[18169110762] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[18171312027] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[18172893024] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[18173595264] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18175138971] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18178459167] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18180532557] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370198016 RFLAGS_BEFORE=134 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[18185518164] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[18186861891] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[18190388337] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18191768034] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18193285308] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18194618508] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=229 pred=0 subj_lo=0
[18195823107] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18196903362] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18197952894] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18199082253] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[18219607692] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[18229173864] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[18231350676] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18232792908] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18233826336] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18234952758] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18236517354] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18237951006] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18239497023] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[18241024131] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[18243647730] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[18247602318] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[18252059892] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[18256372134] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18257544261] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[18258802353] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18260711073] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18262031898] [INFO] [fontd] [CPU3] FONTD: Service ready
[18262790469] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[18264429645] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849040 RFLAGS_BEFORE=130 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[18271883916] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010aff8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18284303565] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18285734478] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18287091801] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047872 RFLAGS_BEFORE=130 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[18293665533] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010f930
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18295735821] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370266912 RFLAGS_BEFORE=130 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[18299781456] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18301104426] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18302495178] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[18303692946] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18305074953] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[18306463164] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=234 subj_lo=0
[18308053434] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[18315836781] [INFO] [nectar] [CPU2] NECTAR: Started.
[18317562846] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18318826317] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18319953894] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18321073551] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=236 subj_lo=0
[18323707908] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[18327034242] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[18340680567] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[18345290568] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[18347671320] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18348768240] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18349884663] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18351122031] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=240 subj_lo=0
[18357278379] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1252 backend=VirtIO-GPU
[18358662069] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18359714901] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[18360389784] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18361659228] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18363153897] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18364721661] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18365629788] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[18395293224] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[18408931035] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[18410185200] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[18500528145] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[18515979075] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[18523349625] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[18526279629] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f1030
[18527519571] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e4
[18529158615] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370356384 RFLAGS_BEFORE=134 CR3_BEFORE=72052736 fs_base=0 gs_base=18446744071564586640
[18533034498] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[18533839665] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18535679415] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18537188934] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[18539590839] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18540871965] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18547764774] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[18549959934] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[18550688607] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1030
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[18553199808] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[18553846575] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370422176 RFLAGS_BEFORE=134 CR3_BEFORE=73101312 fs_base=0 gs_base=18446744071564586576
[18558881946] [INFO] [echo] [CPU1] echo: starting up
[18560871945] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[18565459506] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[18567214512] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f4000
[18568443234] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[18569246388] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f4000
[18570431418] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f7000
[18572027133] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276783104)
[18573030003] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[18576045048] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f9000 phys=0x4615000
[18577226679] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[18579410949] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[18581418075] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[18582794868] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[18587050350] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[18588610854] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[18592122879] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[18600607740] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[18609357987] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[18611832822] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[18612528363] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[18614560239] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[18615302640] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[18618501495] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[18620424108] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[18621438627] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18622586466] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[18623785323] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18624632565] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[18625395492] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[18626122218] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[18627229731] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[18632185407] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[18639110226] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[18644400588] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[18646188726] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[18647871099] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[18664422084] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18668303181] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[18670567707] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[18672332316] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[18673890345] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[18674823057] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18676472430] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18677357952] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[18682233042] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18684066588] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18691116213] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[18693678498] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[18696541182] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[18698095746] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[18699543687] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[18700463958] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[18703179858] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[18704171904] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18704858007] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18706200414] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18713506350] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18717124338] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18719005668] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18725137761] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[18727548873] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[18728815017] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[18730363872] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[18731185440] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18732595596] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18733324698] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010f930
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18734731620] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370706752 RFLAGS_BEFORE=130 CR3_BEFORE=73998336 fs_base=0 gs_base=18446744071564586640
[18737750757] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[18738925821] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18741935289] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18743174604] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18746298120] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[18748534662] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[18750057249] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[18752790672] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[18753597885] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f25c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18754994016] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370772288 RFLAGS_BEFORE=130 CR3_BEFORE=74145792 fs_base=0 gs_base=18446744071564586576
[18759266757] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[18761097696] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18786294879] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[18801736338] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[18803058120] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[18817599141] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[18825152181] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[18826987674] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18854922273] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[18856817958] [INFO] [bloom] [CPU3] bloom: creating surface...
[18857700906] [INFO] [bloom] [CPU3] bloom: surface created!
[18858592566] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[18865041459] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[18871059075] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[18873620931] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[18877412136] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[18882035964] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 3)
[18885696654] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1261
[18886785489] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[18898802835] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[18901237773] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[18902285886] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[18903383961] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
[18909577764] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1261
T:0270 [18915422196] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[18921129183] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18922156638] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18924447399] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[18932376738] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[18939865626] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[18945417249] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
T:07D0 T:0640 T:F0B0 [18971997792] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[18980231952] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[18982170867] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[18983939238] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[18985979430] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
T:1220 [18996407232] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[18998162997] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[19005871632] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[19007892123] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[19010568126] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([239, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[19024025856] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=277
[19025049186] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[19026010179] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19027099410] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19028122113] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19029310575] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=277 subj_lo=0
[19044607164] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1274)
[19045955709] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[19058413671] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=243
[19059958995] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[19061027502] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19061970576] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19062951897] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19064086008] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[19066767654] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[19067924238] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[19073418408] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[19078328973] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=31, our_read=32)
[19080108234] [INFO] [anther] [CPU1] anther: Connected to network stack
[19099866786] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[19100706999] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[19103064288] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[19110959241] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1278)
[19112330820] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[19125383475] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=283
[19126505838] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[19127935695] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19129189761] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[19130447292] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19131708255] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19132731222] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19134018618] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=283 subj_lo=0
[19145445957] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19151568777] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[19153646193] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[19154487363] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1279)
[19156037703] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[19157045589] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[19158247944] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[19160313249] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19166382972] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19170234567] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[19172091741] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[19205177706] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[19208296140] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[19213127175] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[19215312336] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=1
[19218129513] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[19220772351] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=2
[19223966718] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=2)
[19247172912] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[19249474629] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[19253119875] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[19257783303] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[19447095393] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[19486741692] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[19509867036] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[19590714693] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[19667633502] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=664 watches=13 history=1024 journal=1024 symbols=313 drops=0
[19720969620] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[19839404739] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[19979889732] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[20187531408] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[20350525041] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[20499358968] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[20519495139] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[20671953357] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[20822558427] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[20830077642] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[20831306925] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[20832502647] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[20833592142] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[20834937321] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[20844173823] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[20845152306] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[20846369511] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[20847744324] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=337 pred=0 subj_lo=0
[20871567024] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[21018762666] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[21129764766] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=712 watches=15 history=1024 journal=1024 symbols=339 drops=0
[21200091231] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[21359798988] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[21527155419] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[21572614800] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[21574155438] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[21611658486] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21675493686] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21717692865] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[21718969404] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21720191493] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21721577130] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=339 pred=0 subj_lo=0
[21758008173] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21792385593] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=9d85f11f1afc2373)
[21813718674] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[21909784050] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21951124272] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[21980727351] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21987108330] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[22052586930] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[22064433633] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[22066070631] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[22070220876] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22112466750] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[22168859691] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[22183682070] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22191965433] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x1222d000
[22193591211] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[22195210653] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[22257137661] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[22267533783] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[22271900838] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[22274305779] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[22276997028] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[22281887859] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[22302749469] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[22305096528] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[22307065308] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[22322499639] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x122be000
[22323746577] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[22415683851] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=14aa2015f9906e3f)
[22650262866] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[22859910348] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[23067714912] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=793 watches=16 history=1024 journal=1024 symbols=354 drops=0
[23109686589] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[23385137985] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[23638290786] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[23933262936] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[24204474129] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[24486513414] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[24728061468] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=827 watches=16 history=1024 journal=1024 symbols=364 drops=0
[24749164968] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[25003459272] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[25192296591] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[25257666258] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[25399737297] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25436857776] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[25437925524] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25439003865] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25440114315] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=340 pred=0 subj_lo=0
[25455332793] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[25516002303] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25529950974] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[25612348509] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25658322690] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[25659377469] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25660490790] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25661612724] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=342 pred=0 subj_lo=0
[25793545140] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25819847691] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[25821067602] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25822353414] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25823744925] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=343 pred=0 subj_lo=0
[25904588160] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25907622576] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[26023959390] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26108579838] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26183657676] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26286946356] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26411469579] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26572248219] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26682903852] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=890 watches=19 history=1024 journal=1024 symbols=367 drops=0
[26724469662] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[26726055411] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=2 (16384b)
[26727975879] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[26729710326] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[26770644648] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26851477356] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26909994078] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2429 ops=1 watches=19
[26924534835] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[27022120686] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27245235864] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27330384873] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27431073681] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27468310419] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[27538750602] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27627039198] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27732232770] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=2
[27795355368] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27888148101] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27962626230] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28014408015] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28076621397] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28175603379] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28249027851] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28268524746] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[28350228786] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28447193280] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28735416336] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[28772114316] [INFO] [fontd] [CPU3] FONTD: Auto-importing font asset ThingId([70, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[29031249588] [INFO] [fontd] [CPU3] FONTD: Auto-imported font 'Noto Sans' style 'Regular' from asset ThingId([70, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[29034190449] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (1 fonts)
[29106703263] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[29262235167] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29266345746] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[29748333780] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[29768249610] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30216641367] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=1029 watches=19 history=1024 journal=1024 symbols=376 drops=0
[33987864075] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[34093764078] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[34119878463] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[34453866744] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[34455986433] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[34476633906] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[35161887453] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[35223041370] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35224295469] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35225480730] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35226888708] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[35238161277] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35239282650] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35240427057] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35241856980] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=235 pred=0 subj_lo=0
[35249903139] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35250996792] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35252109024] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35253370548] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=455 pred=0 subj_lo=0
[35262491022] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35263636287] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35264940612] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35266186065] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=454 pred=0 subj_lo=0
[35277622611] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35278780548] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35279985774] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35281242843] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[35291060607] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[35474365641] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=1266 watches=24 history=1024 journal=1024 symbols=456 drops=0
[39044767938] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1382 watches=24 history=1024 journal=1024 symbols=456 drops=0
[39615774429] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=3453 ops=1 watches=24
[42703925088] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1503 watches=24 history=1024 journal=1024 symbols=456 drops=0
[47005387803] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1617 watches=24 history=1024 journal=1024 symbols=456 drops=0
[51420881127] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=1744 watches=24 history=1024 journal=1024 symbols=456 drops=0
[55812994116] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=1858 watches=24 history=1024 journal=1024 symbols=456 drops=0
[60725521230] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=16000 nodes=1976 watches=24 history=1024 journal=1024 symbols=456 drops=0
[64448861169] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=4477 ops=1 watches=24
[65496297768] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=17000 nodes=2103 watches=24 history=1024 journal=1024 symbols=456 drops=0
[70144448517] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=18000 nodes=2219 watches=24 history=1024 journal=1024 symbols=456 drops=0
[74886996624] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=19000 nodes=2334 watches=24 history=1024 journal=1024 symbols=456 drops=0
[80300813175] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=20000 nodes=2456 watches=24 history=1024 journal=1024 symbols=456 drops=0
[85725395481] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=21000 nodes=2577 watches=24 history=1024 journal=1024 symbols=456 drops=0
[91029925965] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=22000 nodes=2699 watches=24 history=1024 journal=1024 symbols=456 drops=0
[93872503263] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=5501 ops=1 watches=24
[95975159115] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=23000 nodes=2817 watches=24 history=1024 journal=1024 symbols=456 drops=0
[101366903814] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=24000 nodes=2942 watches=24 history=1024 journal=1024 symbols=456 drops=0
[106659027552] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=25000 nodes=3060 watches=24 history=1024 journal=1024 symbols=456 drops=0
[111952193232] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=26000 nodes=3174 watches=24 history=1024 journal=1024 symbols=456 drops=0
[118944242472] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=27000 nodes=3307 watches=24 history=1024 journal=1024 symbols=456 drops=0
[124093203861] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=28000 nodes=3417 watches=24 history=1024 journal=1024 symbols=456 drops=0
[125564861730] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=6525 ops=1 watches=24
[130042486497] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=29000 nodes=3548 watches=24 history=1024 journal=1024 symbols=456 drops=0
[136573707963] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=30000 nodes=3664 watches=24 history=1024 journal=1024 symbols=456 drops=0
[140880439689] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[142556172792] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=31000 nodes=3789 watches=24 history=1024 journal=1024 symbols=456 drops=0
[148266266049] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=32000 nodes=3899 watches=24 history=1024 journal=1024 symbols=456 drops=0
[154327459275] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=33000 nodes=4015 watches=24 history=1024 journal=1024 symbols=456 drops=0
[160104673278] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=7549 ops=1 watches=24
[161119336620] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=34000 nodes=4136 watches=24 history=1024 journal=1024 symbols=456 drops=0
[167096610582] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=35000 nodes=4256 watches=24 history=1024 journal=1024 symbols=456 drops=0
[169244760102] [INFO] [bloom] [CPU3] [bloom] graph pointer state diverged from streamed cursor: cursor=(960, 540) graph=(400, 300)
[169251265821] [INFO] [bloom] [CPU3] [cursor metrics] frame=120 moves=0 rasterizations=1 cursor_only_frames=0 damage_rects=1
[172881000534] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=36000 nodes=4373 watches=24 history=1024 journal=1024 symbols=456 drops=0
[179214397659] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=37000 nodes=4493 watches=24 history=1024 journal=1024 symbols=456 drops=0
[185085490128] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=38000 nodes=4603 watches=24 history=1024 journal=1024 symbols=456 drops=0
[191582674668] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=39000 nodes=4732 watches=24 history=1024 journal=1024 symbols=456 drops=0
[196045688586] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=8573 ops=1 watches=24
[198441193566] [INFO] [kernel::r
```
</details>
