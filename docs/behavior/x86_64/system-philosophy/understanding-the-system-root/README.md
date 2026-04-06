# ❌ Scenario: Understanding the system root

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 5266ms | - - - |
| 2 | And the anther server is ready | ✅ | 5661ms | - [📜](./02/serial.log) - |
| 3 | When I execute the GQL query "MATCH (n:svc.Root) RETURN n" | ❌ | 2953817ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11949908454] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11955251649] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11959939959] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11961870855] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11963071098] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11963691762] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11964348165] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11964921804] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11965541379] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11966142573] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11966723472] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11967317274] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11968006380] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11968696773] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11969394459] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11970006312] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11970615327] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11971208436] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11971814547] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11972455077] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11973026241] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11973602289] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11974184838] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11974777749] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11975497809] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11976095835] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11976690396] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11977327923] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11978038809] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11979016302] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11979931524] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11980893606] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11981533080] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11982164073] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11982748635] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11983436190] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11984130774] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11984847138] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11985554889] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11986275312] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11986976133] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11987681838] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11988994512] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11990425590] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11991186834] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11991745656] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11992252470] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11992767567] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11993319294] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11993836074] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11994344736] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11994858216] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11995391232] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11995924545] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11996465151] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11997022752] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11997564414] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11998123071] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11998678494] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11999243256] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11999784522] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[12000343014] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[12000884808] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[12001752015] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[12002581602] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[12003575991] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[12004340337] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[12004914372] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[12005457024] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[12006016968] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[12006564273] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[12007128573] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[12007670664] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[12008267931] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[12008810814] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[12009370626] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[12009939744] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[12010503879] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[12011045112] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[12011633865] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[12012177408] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[12012739365] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[12013340064] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[12013908126] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[12014451537] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[12015041214] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[12015582975] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[12016145163] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[12016688475] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[12017249013] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[12017790708] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[12018391374] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[12018934983] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[12019497468] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[12020036820] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[12020616993] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[12021158094] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[12021754866] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[12022295604] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[12022855746] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[12023399322] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[12024011406] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[12024760836] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[12025641738] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[12026417205] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[12027332823] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[12027966390] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[12028813599] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12275402733] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[12286134366] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12290811687] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12292043214] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12292934379] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12298189959] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12299853423] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12300927474] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12301635390] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12302431647] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12303156558] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12304148307] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12305218992] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12305910078] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12306594399] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12307255290] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12307938357] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12309102861] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12310118040] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12310823910] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12312443154] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12313359102] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12314341710] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12315892908] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12317385960] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12318401535] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12319235049] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12320278245] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12704042142] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12705051183] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12708223902] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12709083123] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12709869678] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12711367416] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12723998133] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12725645856] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12726415284] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12728045055] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12728579754] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12731007069] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12738603768] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12740267034] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12756488580] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12757054365] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12773122263] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12773725965] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12775653495] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12776833344] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12777847500] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12780024114] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12780811593] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12815680878] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62333800 ticks/sec), init_cnt=623338 for 100Hz
[12817115388] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12817898511] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12819054534] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12824413569] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12854689716] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12855807657] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12857986020] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12859373274] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12860528439] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12863175666] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12864378582] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12884797530] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12885468090] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12886380243] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12888333084] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12889444293] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12890539365] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12891394263] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12918710508] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12920476866] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12921375753] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12922079973] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12922887516] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12923551179] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12924310245] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12925055385] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12931802796] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12932723463] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12934463751] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12935536812] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12940679796] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12942205848] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12942892512] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12944085132] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12944976066] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12945805950] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12957370206] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12960181806] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12961367760] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12962136330] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12983186007] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13002222717] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13005105861] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13009508622] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13011167268] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13013406252] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13015614810] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13017712620] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13018687836] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13020169536] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13026842961] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13028433132] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13031106792] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13033467546] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13035714450] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13038304884] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13039086159] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13050825150] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13051662558] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13058185107] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13059040863] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13085774658] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13086718986] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13418482836] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13868274552] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13893431145] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[13926250173] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[15163724994] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=449 watches=0 history=963 journal=772 symbols=97 drops=0
[15867427356] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15960318330] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15961400862] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[16059074922] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[16114091466] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[16137721512] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[16138788303] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[16139611719] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[16143826380] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[16158393108] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[16174041906] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[16176155424] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[16227638889] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[16249206237] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[16249998006] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[16253603421] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[16282501257] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[16300512921] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[16307123778] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[16308208587] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[16371943104] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[16373832552] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[16461822333] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[16526093595] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[16538715732] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[16542387246] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[16544746713] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[16614569004] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[16620121584] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[16621533984] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[16622809929] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[16624048419] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[16625088051] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[16626128178] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[16627225758] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[16628163453] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[16629135699] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[16630331355] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[16631589480] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[16632790680] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[16633836582] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[16634925681] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[16636228257] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[16637246835] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[16638280098] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[16639373058] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[16640723154] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[16642042890] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[16643387640] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[16644319593] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[16644933228] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[16645543695] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[16646220393] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[16646847657] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[16647459906] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[16648129509] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[16648740339] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[16649379153] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[16650010575] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[16650646452] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[16651459077] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[16652354565] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[16653169764] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[16654096437] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[16654889955] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[16655726142] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[16656528900] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[16657358256] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[16658105607] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[16658850318] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[16660502925] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[16662289710] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[16663229319] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16671652437] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[16687786863] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[16692094749] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[16701020853] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[16701902448] [CONTRACT] [kernel] [CPU0] Spawning init process...
[16703944917] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[16706424240] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[16707162285] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
ThingOS Petals
type 'help' for commands
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000

petals> [16712956953] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[16716622494] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013440 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[16732668150] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[16737349266] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[16738569540] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[16739617554] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[16747268340] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[16751002125] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=184 drops=0
[16752577314] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[16756657929] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[16757717130] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[16761817545] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[16765726032] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[16766810445] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[16771334085] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[16772375367] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[16773396354] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[16774413975] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16777871253] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16778925306] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16780081098] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16781520987] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16782898539] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16784204613] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16788956118] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[16828723857] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[16835731506] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[16840044375] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[16844351304] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[16847060769] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[16850456436] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[16854596319] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[16858762932] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[16863027984] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[16867657719] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[16872380646] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[16877416149] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[16883340144] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[16888613313] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[16893162066] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[16898060916] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[16903098729] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[16908033648] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[16912121721] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[16916305197] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[16920536061] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[16924949514] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[16929669042] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[16934816481] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[16940101629] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[16944789081] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[16949330244] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[16953791877] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[16957944663] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[16963045044] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[16966450776] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[16969709196] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[16974012066] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[16978044435] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[16982757924] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[16987526028] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[16992597699] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[16999397547] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[17005059786] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[17010080538] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[17015379678] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[17018458083] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[17037188058] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[17165952738] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[17172403314] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[17173211385] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17174832741] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17179279491] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17180588502] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17188718745] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[17193811272] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[17195035539] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17196649371] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078976 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[17201696292] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[17202426021] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[17203268544] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17205000483] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17209362225] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[17211020937] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17217625458] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[17220161937] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[17221038813] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[17222895657] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144512 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[17225942085] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[17231499978] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[17233071669] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[17235103347] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17239293258] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:04:47 = 1775437487 unix_secs
[17240895012] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775437487, mono_ns=8620233022, offset=1775437478379766978ns
[17242513101] [INFO] [rtc_cmos] [CPU1] System clock anchored
[17252163357] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[17287620372] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[17298372267] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[17299196673] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17301088365] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17306761626] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[17309050011] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[17316009018] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[17318650272] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[17322311490] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17323860180] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210880 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[17329014054] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[17334563598] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[17336537163] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[17337648603] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17340008037] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17349187383] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17352407160] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17362042038] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[17366154828] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[17367124995] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17368874655] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277632 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[17373011502] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[17374072089] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[17379269061] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[17380332321] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[17381898567] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[17383301331] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[17388309972] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[17389628355] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[17404643817] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[17406138453] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[17972220618] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17976926154] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[17979651492] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[17981192559] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[17985240933] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[17986699731] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[17988587958] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[17989971912] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[17990949339] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[17993095131] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[17995534887] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[17996554950] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[17997394206] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[17998156308] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[17998909302] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[17999967711] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[18004026282] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[18004846563] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18006538671] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18013433559] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18015751809] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18022492026] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[18025017648] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fab68
[18026299203] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[18028170468] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352944 RFLAGS_BEFORE=134 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[18034548906] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[18042515535] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[18071189301] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[18081593475] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[18083552322] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[18088521759] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[18097069518] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[18098527689] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[18104781057] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[18106076274] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[18106928466] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[18107820093] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[18108882297] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[18110102109] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[18111428676] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[18112366239] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[18119141568] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[18120277824] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[18122619273] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[18124992138] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[18130405458] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[18131097468] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18132605997] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18136237614] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18137835870] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18144882393] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[18147254664] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[18148947333] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[18149656866] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18151010790] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18155738601] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18157035138] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18163517097] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[18165540030] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
[18166298568] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0105668
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[18169454589] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500224 RFLAGS_BEFORE=130 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[18172524480] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[18173447490] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18175324629] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18176046273] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[18177351588] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[18180860742] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[18181965120] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18182770881] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[18184188990] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[18184893771] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18186329601] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[18187714215] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[18188943399] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[18190551126] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[18191884029] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[18193212345] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[18194233002] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[18196737141] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[18198417303] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[18199757796] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[18201451818] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fab68
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[18202797822] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369433984 RFLAGS_BEFORE=130 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[18207860979] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[18646931292] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0105668
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[18648977721] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583008 RFLAGS_BEFORE=134 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[18652851096] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[18654170304] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[18654827499] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[18656074569] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[18656854227] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[18658625436] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[18663179964] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18665864910] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[18666695421] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18668438118] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18672895956] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[18674992446] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[18677279478] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[18684932706] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[18688286760] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[18690339756] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[18691857393] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[18692657709] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18694086939] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18715132161] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18719101137] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[18739201767] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[18742076133] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb01095c8
[18748672998] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18750339135] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716768 RFLAGS_BEFORE=130 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[18755488026] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fab68
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[18757676157] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650272 RFLAGS_BEFORE=134 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[18762006648] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[18764581869] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[18765778746] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[18768960969] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[18770055018] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[18772974759] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[18775038744] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[18781825425] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[18784304418] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[18786067212] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[18787972170] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[18795797559] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[18798222762] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[18800600346] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[18801599520] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18804508734] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18844135035] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[18845656665] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[18847046196] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18848192319] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[18872257536] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[18895289556] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[18902057361] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[18904649346] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fab68
[18906223842] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18908037555] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369782832 RFLAGS_BEFORE=134 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[18912748041] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[18913482390] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18914878488] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18916113414] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[18919442454] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[18923577321] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18925527720] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18938048184] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18941458503] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[18948071373] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[18950477568] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[18952313787] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[18953052558] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18954477135] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18959704863] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18961996812] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18968940738] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[18971179029] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010ac38
[18972127284] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18973409499] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915152 RFLAGS_BEFORE=130 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[18976223442] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18976923867] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18978074742] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[18978801105] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18979504962] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[18986589567] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18989954445] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18996730335] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[18999026508] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010ac38
[19000202958] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19001828604] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981552 RFLAGS_BEFORE=130 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[19006538265] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[19007252286] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19008615615] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19017525747] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19021295997] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[19027688757] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[19030254375] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[19032232560] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[19032900414] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19034297634] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19057809243] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[19062496662] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[19069037823] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[19071316110] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
[19072091412] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010ca80
[19074213477] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[19074965217] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19076671086] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19077424773] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113712 RFLAGS_BEFORE=130 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[19081689891] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[19093685061] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[19096675191] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19097402940] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[19102496160] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19103909748] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19105204833] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19106278059] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[19107284262] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[19109733885] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[19111229577] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19113227199] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370198000 RFLAGS_BEFORE=134 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[19116068829] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[19116794070] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19117962600] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[19118624646] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19119416349] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[19126842042] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19127937939] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19129151844] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19130501313] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[19134074652] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19135693566] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19137503418] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19139454939] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[19157046513] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[19166883021] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[19168118343] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19169142828] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19170190215] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19171276146] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[19174366266] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19175395140] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19176459357] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19177617030] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[19179275280] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[19181838456] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[19186429548] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fab68
[19187361138] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19188809838] [INFO] [fontd] [CPU3] FONTD: Service ready
[19189393311] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849072 RFLAGS_BEFORE=134 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[19194082611] [INFO] [nectar] [CPU2] NECTAR: Started.
[19195890780] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010ac38
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19197319185] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047632 RFLAGS_BEFORE=134 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[19201689210] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[19203786591] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
[19204796952] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19206553938] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19207975479] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370266752 RFLAGS_BEFORE=130 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[19211318280] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19212887133] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[19213880862] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[19215609732] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[19217182116] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[19229161710] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[19233347034] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[19245383652] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[19249726518] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[19251247323] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f4000
[19253065491] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f4000
[19253784660] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19254884022] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19255862769] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19257024732] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=239 subj_lo=0
[19265115507] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19266607404] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19272324786] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19273345575] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19274527965] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19275614325] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[19280964978] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19281919833] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19282889439] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19283948310] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[19290626586] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19291693806] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19292808381] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19293961071] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1252 backend=VirtIO-GPU
[19294893519] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[19296143361] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[19296878931] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19298344692] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19325575962] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[19341525588] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[19342615776] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[19391832405] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19393085910] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19429687794] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[19444511394] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[19452156273] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[19454396049] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f0fe8
[19456468284] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e4
[19458222795] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370357168 RFLAGS_BEFORE=130 CR3_BEFORE=72323072 fs_base=0 gs_base=18446744071564586640
[19463552757] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[19464680202] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f7000
[19466665449] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276783104)
[19468681617] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[19470827145] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f8000 phys=0x4607000
[19472194005] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[19474429128] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[19476686658] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[19478388831] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[19482601017] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19484244582] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19488713739] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[19497930969] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[19507197765] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[19509711771] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[19510714839] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[19511961315] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[19513608048] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[19515147828] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[19516784166] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[19518247650] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[19519425189] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[19520913258] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[19526302488] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[19530551469] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[19531672974] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[19532661159] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[19537477740] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[19538272215] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19540091175] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19544490669] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[19545911781] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19553013414] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[19556258568] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[19558181907] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[19560014265] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f0fe8
[19561466661] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[19563206652] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370566592 RFLAGS_BEFORE=134 CR3_BEFORE=73752576 fs_base=0 gs_base=18446744071564586576
[19566756990] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[19568568888] [INFO] [echo] [CPU1] echo: starting up
[19569554037] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19570885983] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[19576535748] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[19586628204] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[19587482112] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[19588492077] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[19589948862] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[19591906554] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[19596599649] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[19602386562] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[19607252874] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[19633190907] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19633995348] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[19634833911] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19636755798] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[19639123713] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[19640823114] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[19642300788] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[19643095725] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[19644045993] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19645150503] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19646728497] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19651796043] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19653628368] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[19661385282] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[19663955553] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[19665488436] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[19666421082] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[19668052767] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[19669566873] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19670314092] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19671721905] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19673245548] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[19675471530] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[19679263989] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[19680096150] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19683570258] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[19684219962] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19691260578] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[19693660437] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[19694672811] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[19696310436] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[19697077818] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19698470913] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19701331056] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[19702456257] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[19709533734] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[19711419618] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f28e8
[19712388894] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[19713792087] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[19714564254] [INFO] [bloom] [CPU3] bloom: creating surface...
[19715167659] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19716493995] [INFO] [bloom] [CPU3] bloom: surface created!
[19717113669] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370808256 RFLAGS_BEFORE=134 CR3_BEFORE=74153984 fs_base=0 gs_base=18446744071564586576
[19720377336] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[19722150954] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[19727202891] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f23b8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19728707229] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370742720 RFLAGS_BEFORE=134 CR3_BEFORE=74002432 fs_base=0 gs_base=18446744071564586640
[19734964524] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[19771874034] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1262
[19773880335] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[19799463255] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 3)
[19800649605] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[19803692832] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[19804672998] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[19805669763] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
[19814984079] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1262
T:0270 [19821136401] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[19829034390] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[19837393950] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[19845485220] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[19850314737] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
T:07D0 T:0640 T:F0B0 [19865280567] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[19867092630] [INFO] [anther] [CPU1] anther: Connected to network stack
[19876073943] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[19913712654] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[19914980382] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[19920664632] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[19948795350] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[19954849200] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[19956967635] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[19959619515] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[19962835101] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[19965558261] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[19975783542] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[20001815328] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[20004335538] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[20106704970] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[20199991581] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=632 watches=10 history=1024 journal=1024 symbols=291 drops=0
[20227102533] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[20316968430] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[20344034568] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[20354192760] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=279
[20355324396] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[20356191471] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20357150781] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20358235788] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20359318023] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=279 subj_lo=0
[20363243538] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[20369477931] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[20371534326] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[20375714898] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[20376964938] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[20378963385] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[20379886923] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[20382088815] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[20392360032] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1281)
[20393415867] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
T:1220 [20395420221] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[20396833116] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[20400850503] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[20404086219] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[20405611677] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[20414309388] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=244
[20415549198] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[20416414095] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20417449635] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20418587277] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20419794186] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[20429170113] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1283)
[20431241193] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[20440975764] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=296
[20442231018] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[20443131753] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20444209929] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20445333843] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20446574907] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=296 subj_lo=0
[20450080596] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[20453893515] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[20458470153] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[20460939873] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[20463385041] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[20465697549] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[20471952798] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[20477532537] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1285)
[20478708954] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[20483489070] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[20508446013] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[20510887452] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[20512993017] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[20586505500] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[20588600307] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[20591606376] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[20600158491] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[20605305204] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[20611067169] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[20620651821] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=74
[20622084615] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=25
[20624189685] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[20628978513] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[20634666129] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[20639589663] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[20644183197] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=70
[20645699151] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[20648043141] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[20652796659] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=149
[20654269350] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=25
[20656240638] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[20660937231] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[20681560614] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[20828326596] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[20830273167] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:46458 on listener 1
[20836026717] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[20877028656] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[20912006082] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[20919097155] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[20921490612] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=3
[20933157828] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 37 (user thread) assigned to CPU 1
[20935796640] [INFO] [anther] [CPU1] anther: Thread spawned TID=37 for conn_handle=3
T:5EE0 [20959622343] [INFO] [anther] [CPU1] anther: Worker thread TID=37 starting for conn_handle=3
[20982908265] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=33, our_read=34)
[20984878662] [INFO] [anther] [CPU1] anther: Worker TID=37 connected to netd OK
[21168602631] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[21346710990] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[21515496750] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[21692440143] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[21858045858] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[21866403207] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[21867593616] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[21868537449] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[21869584044] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[21870935724] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[21879766491] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[21880807179] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[21881907201] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[21883516677] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=338 pred=0 subj_lo=0
[21905067723] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[22020855285] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[22054734075] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[22208584629] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[22228219992] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=711 watches=15 history=1024 journal=1024 symbols=349 drops=0
[22370773458] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[22551744303] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[22739093718] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=fec1d29ef0678173)
[22920027801] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[23179237356] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=c1e9a7b90a789e5c)
[23181365856] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[23183863230] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[23224551339] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23283146337] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23284392549] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23285419509] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23286617508] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=317 pred=0 subj_lo=0
[23328947070] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23420422212] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23499878985] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[23540815353] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[23577611904] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[23595431409] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23618128050] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[23632308282] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23633467176] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23634541821] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23635713948] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=318 pred=0 subj_lo=0
[23701927425] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23709508977] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[23747449011] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[23749464948] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[23773089912] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[23811221973] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23815483131] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[23824069368] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23825214897] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23826476289] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23827705869] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=350 pred=0 subj_lo=0
[23835583101] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x122d4000
[23836693188] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[23837933592] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[23883875235] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[23885899620] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[23888045148] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[23896680291] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[23906534784] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[23910858939] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[23913085614] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[23916034593] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[23920606281] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[23931917889] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[23942319027] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[23944006878] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[23945607774] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[23960227698] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x122d5000
[23961688806] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[24051953574] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[24271583292] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=796 watches=18 history=1024 journal=1024 symbols=363 drops=0
[24278987172] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[24506111784] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[24731671173] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[24779265753] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=149
[24781501833] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=25
[24783813846] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[25017953466] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[25288265343] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[25341810516] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[25351656858] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 85 bytes on conn_handle=3
[25358756049] [INFO] [anther] [CPU1] anther: GET /health Http11
[25382539380] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[25385134929] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[25388176242] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[25392597648] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[25394425881] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[25395771555] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[25398343179] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[25571308950] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[25580186082] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[25584618312] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[25587591711] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[25589238147] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[25591975167] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[25598947803] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[25600639218] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[25602662613] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[25627532997] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[25809005904] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=831 watches=18 history=1024 journal=1024 symbols=364 drops=0
[25904551464] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[26178995436] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[26229770490] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[26239455165] [INFO] [anther] [CPU1] anther: Worker TID=37 got first 0 bytes on conn_handle=3
[26257157586] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26292473460] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[26384886759] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26397394914] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[26467897401] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26514269892] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[26649883128] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26651113863] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26652269490] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26653609983] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=351 pred=0 subj_lo=0
[26657495700] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26752870617] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26824523715] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26913899199] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26915734956] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[27022042014] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27206554485] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[27208829340] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[27211500195] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27218690433] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[27221373102] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[27224269413] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[27373986981] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[27808558899] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[27810615063] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[27812756103] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[27823127211] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=149
[27824995011] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=25
[27827064672] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[27895546206] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[27909004794] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=881 watches=19 history=1024 journal=1024 symbols=367 drops=0
[27944514081] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[27948223446] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[27950366268] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:46472 on listener 1
[27952447710] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[27971144124] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=4
[27979762668] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 38 (user thread) assigned to CPU 1
[27981523416] [INFO] [anther] [CPU1] anther: Thread spawned TID=38 for conn_handle=4
T:5EE0 [27998676651] [INFO] [anther] [CPU1] anther: Worker thread TID=38 starting for conn_handle=4
[28030660053] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=35, our_read=36)
[28032395358] [INFO] [anther] [CPU1] anther: Worker TID=38 connected to netd OK
[28340376240] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[28692081429] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2429 ops=1 watches=19
[29011352040] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[29154045063] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29394230811] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[29659570281] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[29695939944] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=917 watches=19 history=1024 journal=1024 symbols=368 drops=0
[29766812460] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[30073951644] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[30503327844] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[30505144626] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[30507368166] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30907959621] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[31380205263] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=149
[31382082798] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=25
[31384047387] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[31405460394] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[31416486684] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 85 bytes on conn_handle=4
[31424398698] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[31426843800] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[31441573581] [INFO] [anther] [CPU1] anther: GET /health Http11
[31662884451] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=978 watches=19 history=1024 journal=1024 symbols=420 drops=0
[32965927401] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[32995999641] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[33014043414] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[33019901013] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[33035277198] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[33038438895] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[33041210598] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[33045411135] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[33047795253] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[33049758324] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[33052763502] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=70
[33054778053] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[33057543420] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[33063179292] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=70
[33065222751] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[33067731147] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[33082054335] [INFO] [anther] [CPU1] anther: Worker TID=38 got first 0 bytes on conn_handle=4
[33157201803] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[33437844132] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[33439431531] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[33466926537] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[33655710693] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1043 watches=19 history=1024 journal=1024 symbols=451 drops=0
[33726700293] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[33823300599] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=70
[33825413952] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[33827609607] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[33938195973] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[33945035355] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[33950071716] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[33952877970] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[33958016367] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=70
[33959643960] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[33961763286] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[33962681115] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[33965146050] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=149
[33966928809] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 139 byte frame (143 encoded) to netd rx_port=25
[33970074930] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (143 bytes sent)
[33985331358] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[33987647595] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[33994464867] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 139 bytes
[34015927506] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[34026720651] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[34050191703] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34051388646] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34052674953] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34053966969] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[34061055336] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34062197202] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34063420710] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34064861985] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=235 pred=0 subj_lo=0
[34074014535] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[34076474619] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:46484 on listener 1
[34078685883] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34080007236] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[34081109238] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34082470983] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34083866883] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=453 pred=0 subj_lo=0
[34092837801] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34094101998] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34095347352] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34096689990] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=452 pred=0 subj_lo=0
[34102835844] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34103956095] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34105258473] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34106661666] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[34116915360] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[34120109331] [INFO] [anther] [CPU1] anther: Accepted connection, spawning thread for conn_handle=5
[34128975111] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 39 (user thread) assigned to CPU 1
[34130934057] [INFO] [anther] [CPU1] anther: Thread spawned TID=39 for conn_handle=5
[34168183665] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[34170486306] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=2 (16384b)
[34173292032] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[34175048787] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
T:5EE0 [34207066344] [INFO] [anther] [CPU1] anther: Worker thread TID=39 starting for conn_handle=5
[34328931582] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=37, our_read=38)
[34331069124] [INFO] [anther] [CPU1] anther: Worker TID=39 connected to netd OK
[34334418525] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[34344012714] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[34348614432] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[34408544709] [INFO] [anther] [CPU1] anther: Worker TID=39 got first 85 bytes on conn_handle=5
[34410691161] [INFO] [anther] [CPU1] anther: GET /health Http11
[35450447076] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 202 bytes - TCP ACK
[35453039391] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 202 bytes
[35462520423] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=70
[35463538968] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[35464513359] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[35467516260] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[35473806819] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[35476775631] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[35482734210] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[35487272370] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=70
[35489979195] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[35493533097] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[35496489204] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[35499848406] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=70
[35501747094] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[35504022114] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[35528122047] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=25 len=70
[35537415903] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[35539777350] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[35697160158] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[35699093034] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[35704617069] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[35716485882] [INFO] [anther] [CPU1] anther: Worker TID=39 got first 0 bytes on conn_handle=5
[35745604323] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1078 watches=24 history=1024 journal=1024 symbols=454 drops=0
[37445272161] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1106 watches=24 history=1024 journal=1024 symbols=454 drops=0
[37755793878] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[39441472686] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=1139 watches=24 history=1024 journal=1024 symbols=454 drops=0
[41025515817] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=1159 watches=24 history=1024 journal=1024 symbols=454 drops=0
[42747598212] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=16000 nodes=1193 watches=24 history=1024 journal=1024 symbols=454 drops=0
[44716280433] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=17000 nodes=1222 watches=24 history=1024 journal=1024 symbols=454 drops=0
[45373999803] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=3453 ops=1 watches=24
[46491165303] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=18000 nodes=1248 watches=24 history=1024 journal=1024 symbols=454 drops=0
[48236568270] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=19000 nodes=1275 watches=24 history=1024 journal=1024 symbols=454 drops=0
[50174095719] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=20000 nodes=1307 watches=24 history=1024 journal=1024 symbols=454 drops=0
[51941155596] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=21000 nodes=1331 watches=24 history=1024 journal=1024 symbols=454 drops=0
[53921107383] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=22000 nodes=1362 watches=24 history=1024 journal=1024 symbols=454 drops=0
[54315102567] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[54410425740] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=26 len=70
[54412109004] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[54414371682] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[56143333950] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=23000 nodes=1390 watches=24 history=1024 journal=1024 symbols=454 drops=0
[57902675490] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=24000 nodes=1412 watches=24 history=1024 journal=1024 symbols=454 drops=0
[57934641864] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[57938241273] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[57942988323] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[57946019934] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[57950900502] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=27 len=70
[57952725831] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[57955176708] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[57962775387] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=28 len=178
[57964757994] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 168 byte frame (172 encoded) to netd rx_port=25
[57968195736] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (172 bytes sent)
[57973538898] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[57995060310] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=SynReceived
[58006525599] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[58024386981] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[58027127433] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:46488 on listener 1
[58030094232] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Respawned listener on port 80 handle=1
[59866660458] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=25000 nodes=1440 watches=24 history=1024 journal=1024 symbols=454 drops=0
[61917185451] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=26000 nodes=1469 watches=24 history=1024 journal=1024 symbols=454 drops=0
[63445789473] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=2
[63716494677] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[64098187362] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[65087973060] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=27000 nodes=1525 watches=24 history=1024 journal=1024 symbols=454 drops=0
[71010924039] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=28000 nodes=1672 watches=24 history=1024 journal=1024 symbols=454 drops=0
[76780896390] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=4477 ops=1 watches=24
[77178535665] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=29000 nodes=1814 watches=24 history=1024 journal=1024 symbols=454 drops=0
[83102506080] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[84394450305] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=30000 nodes=1979 watches=24 history=1024 journal=1024 symbols=454 drops=0
[90712092999] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=31000 nodes=2140 watches=24 history=1024 journal=1024 symbols=454 drops=0
[97086765369] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=32000 nodes=2280 watches=24 history=1024 journal=1024 symbols=454 drops=0
[97227806181] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=29 len=178
[97232963883] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 168 byte frame (172 encoded) to netd rx_port=25
[97235540226] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (172 bytes sent)
[97238719248] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 168 bytes
[97266194883] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[97271546460] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[103592587428] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=33000 nodes=2431 watches=24 history=1024 journal=1024 symbols=454 drops=0
[109521349410] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=5501 ops=1 watches=24
[109559066001] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=34000 nodes=2573 watches=24 history=1024 journal=1024 symbols=454 drops=0
[117880966203] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=35000 nodes=2730 watches=24 history=1024 journal=1024 symbols=454 drops=0
[119647704660] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[124532101386] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=36000 nodes=2879 watches=24 history=1024 journal=1024 symbols=454 drops=0
[131482637088] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=37000 nodes=3017 watches=24 history=1024 journal=1024 symbols=454 drops=0
[138322994073] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=38000 nodes=3158 watches=24 history=1024 journal=1024 symbols=454 drops=0
[143528572968] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[145316691861] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=39000 nodes=3298 watches=24 history=1024 journal=1024 symbols=454 drops=0
[147392048958] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=6525 ops=1 watches=24
[152085646779] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=40000 nodes=3433 watches=24 history=1024 journal=1024 symbols=454 drops=0
[157317398007] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[159205878447] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=41000 nodes=3573 watches=24 history=1024 journal=1024 symbols=454 drops=0
[165914281302] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=42000 nodes=3710 watches=24 history=1024 journal=1024 symbols=454 drops=0
[173067789642] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=43000 nodes=3846 watches=24 history=1024 journal=1024 symbols=454 drops=0
[180288052890] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=44000 nodes=3989 watches=24 history=1024 journal=1024 symbols=454 drops=0
[186524306529] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=7549 ops=1 watches=24
[187887650973] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=45000 nodes=4125 watches=24 history=1024 journal=1024 symbols=454 drops=0
[192505372257] [INFO] [bloom] [CPU3] [bloom] graph pointer state diverged from streamed cursor: cursor=(960, 540) graph=(400, 300)
[192510633051] [INFO] [bloom] [CPU3] [cursor metrics] frame=120 moves=0 rasterizations=1 cursor_only_frames=0 damage_rects=1
[195147352719] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=46000 nodes=4268 watches=24 history=1024 journal=1024 symbols=454 drops=0
[202479832665] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=47000 nodes=4406 watches=24 history=1024 journal=1024 symbols=454 drops=0
[209954398929] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=48000 nodes=4551 watches=24 history=1024 journal=1024 symbols=454 drops=0
[217360167651] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=49000 nodes=4683 watches=24 history=1024 journal=1024 symbols=454 drops=0
[225029941818] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=50000 nodes=4822 watches=24 history=1024 journal=1024 symbols=454 drops=0
[227407568289] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=8573 ops=1 watches=24
[231457440918] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[231502855650] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[231550402248] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[232941517875] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=51000 nodes=4956 watches=24 history=1024 journal=1024 symbols=454 drops=0
[241262455005] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=52000 nodes=5101 watches=24 history=1024 journal=1024 symbols=454 drops=0
[250242065106] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=53000 nodes=5231 watches=24 history=1024 journal=1024 symbols=454 drops=0
[258548661657] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=54000 nodes=5378 watches=24 history=1024 journal=1024 symbols=454 drops=0
[264852681027] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[266758686810] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=55000 nodes=5506 watches=24 history=1024 journal=1024 symbols=454 drops=0
[280043101371] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=9597 ops=1 watches=24
[281411067696] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=56000 nodes=5659 watches=24 history=1024 journal=1024 symbols=454 drops=0
[289498756899] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=57000 nodes=5787 watches=24 history=1024 journal=1024 symbols=454 drops=0
[297786416961] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=58000 nodes=5917 watches=24 history=1024 journal=1024 symbols=454 drops=0
[306440900337] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=59000 nodes=6064 watches=24 history=1024 journal=1024 symbols=454 drops=0
[312564557547] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[314449206951] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=60000 nodes=6192 watches=24 history=1024 journal=1024 symbols=454 drops=0
[323803808097] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=61000 nodes=6333 watches=24 history=1024 journal=1024 symbols=454 drops=0
[327825267528] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=10621 ops=1 watches=24
[331667266953] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=62000 nodes=6451 watches=24 history=1024 journal=1024 symbols=454 drops=0
[340420085412] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=63000 nodes=6594 watches=24 history=1024 journal=1024 symbols=454 drops=0
[342103748976] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[349040643732] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=64000 nodes=6728 watches=24 history=1024 journal=1024 symbols=454 drops=0
[357819781671] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=65000 nodes=6871 watches=24 history=1024 journal=1024 symbols=454 drops=0
[366660626475] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=66000 nodes=7005 watches=24 history=1024 journal=1024 symbols=454 drops=0
[368518525089] [INFO] [bloom] [CPU3] [bloom] graph pointer state diverged from streamed cursor: cursor=(960, 540) graph=(400, 300)
[368529595203] [INFO] [bloom] [CPU3] [cursor metrics] frame=240 moves=0 rasterizations=1 cursor_only_frames=0 damage_rects=1
[375449919273] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=67000 nodes=7125 watches=24 history=1024 journal=1024 symbols=454 drops=0
[378308718480] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=11645 ops=1 watches=24
[385307952414] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=68000 nodes=7270 watches=24 history=1024 journal=1024 symbols=454 drops=0
[393309069285] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=69000 nodes=7382 watches=24 history=1024 journal=1024 symbols=454 drops=0
[402828382407] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=70000 nodes=7519 watches=24 history=1024 journal=1024 symbols=454 drops=0
[411216996960] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=71000 nodes=7647 watches=24 history=1024 journal=1024 symbols=454 drops=0
[416480838378] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[420631764498] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=72000 nodes=7780 watches=24 history=1024 journal=1024 symbols=454 drops=0
[430351682772] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=73000 nodes=7914 watches=24 history=1024 journal=1024 symbols=454 drops=0
[432059883750] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=12669 ops=1 watches=24
[439287628857] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=74000 nodes=8040 watches=24 history=1024 journal=1024 symbols=454 drops=0
[449859884958] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=75000 nodes=8195 watches=24 history=1024 journal=1024 symbols=454 drops=0
[457589040909] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=76000 nodes=8307 watches=24 history=1024 journal=1024 symbols=454 drops=0
[464469923610] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[467149869681] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=77000 nodes=8448 watches=24 history=1024 journal=1024 symbols=454 drops=0
[478327717032] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=78000 nodes=8582 watches=24 history=1024 journal=1024 symbols=454 drops=0
[487750466742] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=79000 nodes=8708 watches=24 history=1024 journal=1024 symbols=454 drops=0
[488663445699] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=13693 ops=1 watches=24
[498303384810] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=80000 nodes=8847 watches=24 history=1024 journal=1024 symbols=454 drops=0
[501714268407] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[506829409251] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=81000 nodes=8971 watches=24 history=1024 journal=1024 symbols=454 drops=0
[515631075003] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=82000 nodes=9104 watches=24 history=1024 journal=1024 symbols=454 drops=0
[518427762270] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[518479718922] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[518540038665] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[525536308044] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=83000 nodes=9230 watches=24 history=1024 journal=1024 symbols=454 drops=0
[534554490756] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=84000 nodes=9346 watches=24 history=1024 journal=1024 symbols=454 drops=0
[544550181648] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=14717 ops=1 watches=24
[544924330896] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=85000 nodes=9485 watches=24 history=1024 journal=1024 symbols=454 drops=0
[554773549671] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=86000 nodes=9607 watches=24 history=1024 journal=1024 symbols=454 drops=0
[564603381606] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=87000 nodes=9748 watches=24 history=1024 journal=1024 symbols=454 drops=0
[566607692739] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[575337970944] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=88000 nodes=9878 watches=24 history=1024 journal=1024 symbols=454 drops=0
[579519576174] [INFO] [bloom] [CPU3] [bloom] graph pointer state diverged from streamed cursor: cursor=(960, 540) graph=(400, 300)
[579527572371] [INFO] [bloom] [CPU3] [cursor metrics] frame=360 moves=0 rasterizations=1 cursor_only_frames=0 damage_rects=1
[584827189485] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=89000 nodes=9996 watches=24 history=1024 journal=1024 symbols=454 drops=0
[594834903465] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=90000 nodes=10131 watches=24 history=1024 journal=1024 symbols=454 drops=0
[602588964912] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[604283026743] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=15741 ops=1 watches=24
[604425238824] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=91000 nodes=10251 watches=24 history=1024 journal=1024 symbols=454 drops=0
[614178349125] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=92000 nodes=10371 watches=24 history=1024 journal=1024 symbols=454 drops=0
[616673805858] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[616749099681] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[617348327706] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[617448720735] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[628110908139] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=93000 nodes=10516 watches=24 history=1024 journal=1024 symbols=454 drops=0
[638080652484] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=94000 nodes=10642 watches=24 history=1024 journal=1024 symbols=454 drops=0
[639056055264] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[639109307628] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[648299280651] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=95000 nodes=10775 watches=24 history=1024 journal=1024 symbols=454 drops=0
[659370851700] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=96000 nodes=10903 watches=24 history=1024 journal=1024 symbols=454 drops=0
[668921355564] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=97000 nodes=11023 watches=24 history=1024 journal=1024 symbols=454 drops=0
[669623035323] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=16765 ops=1 watches=24
[681125371872] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=98000 nodes=11164 watches=24 history=1024 journal=1024 symbols=454 drops=0
[692780181819] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=99000 nodes=11288 watches=24 history=1024 journal=1024 symbols=454 drops=0
[693015628305] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[702484935669] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=100000 nodes=11410 watches=24 history=1024 journal=1024 symbols=454 drops=0
[713149913553] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[713857349535] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=101000 nodes=11553 watches=24 history=1024 journal=1024 symbols=454 drops=0
[723468574059] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=102000 nodes=11673 watches=24 history=1024 journal=1024 symbols=454 drops=0
[733700991639] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=103000 nodes=11790 watches=24 history=1024 journal=1024 symbols=454 drops=0
[734139404790] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=17789 ops=1 watches=24
[746038138032] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=104000 nodes=11930 watches=24 history=1024 journal=1024 symbols=454 drops=0
[756458578656] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=105000 nodes=12040 watches=24 history=1024 journal=1024 symbols=454 drops=0
[765247357677] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[768398319975] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=106000 nodes=12177 watches=24 history=1024 journal=1024 symbols=454 drops=0
[781045576806] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=107000 nodes=12303 watches=24 history=1024 journal=1024 symbols=454 drops=0
[792411352491] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=108000 nodes=12425 watches=24 history=1024 journal=1024 symbols=454 drops=0
[803632463799] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=109000 nodes=12556 watches=24 history=1024 journal=1024 symbols=454 drops=0
[804114150345] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=18813 ops=1 watches=24
[815154614901] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=110000 nodes=12678 watches=24 history=1024 journal=1024 symbols=454 drops=0
[823537631466] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[825268623564] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=111000 nodes=12788 watches=24 history=1024 journal=1024 symbols=454 drops=0
[837187687149] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=112000 nodes=12925 watches=24 history=1024 journal=1024 symbols=454 drops=0
[842081669154] [INFO] [bloom] [CPU3] [bloom] graph pointer state diverged from streamed cursor: cursor=(960, 540) graph=(400, 300)
[842089280703] [INFO] [bloom] [CPU3] [cursor metrics] frame=480 moves=0 rasterizations=1 cursor_only_frames=0 damage_rects=1
[842289782499] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[850614945873] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=113000 nodes=13057 watches=24 history=1024 journal=1024 symbols=454 drops=0
[861436565913] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=114000 nodes=13173 watches=24 history=1024 journal=1024 symbols=454 drops=0
[874129717362] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=115000 nodes=13308 watches=24 history=1024 journal=1024 symbols=454 drops=0
[877383675696] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=19837 ops=1 watches=24
[884839470603] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=116000 nodes=13428 watches=24 history=1024 journal=1024 symbols=454 drops=0
[886811296536] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[897303460713] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=117000 nodes=13546 watches=24 history=1024 journal=1024 symbols=454 drops=0
[911608424166] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=118000 nodes=13687 watches=24 history=1024 journal=1024 symbols=454 drops=0
[920603170707] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[920992406862] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[922478474346] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=119000 nodes=13805 watches=24 history=1024 journal=1024 symbols=454 drops=0
[933787127889] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=120000 nodes=13928 watches=24 history=1024 journal=1024 symbols=454 drops=0
[947535687681] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=121000 nodes=14074 watches=24 history=1024 journal=1024 symbols=454 drops=0
[951122125305] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=20861 ops=1 watches=24
[958463542245] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=122000 nodes=14196 watches=24 history=1024 journal=1024 symbols=454 drops=0
[970736924883] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=123000 nodes=14325 watches=24 history=1024 journal=1024 symbols=454 drops=0
[985818770640] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=124000 nodes=14453 watches=24 history=1024 journal=1024 symbols=454 drops=0
[986139030954] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[997260559680] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=125000 nodes=14575 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1002950706450] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1010573119809] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=126000 nodes=14716 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1025986520130] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=127000 nodes=14836 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1030603412553] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=21885 ops=1 watches=24
[1036945086606] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=128000 nodes=14956 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1050935520624] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=129000 nodes=15095 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1062781444395] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=130000 nodes=15209 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1075367533218] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=131000 nodes=15335 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1089845920152] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1090144578501] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=132000 nodes=15476 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1101668725773] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=133000 nodes=15596 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1108691398914] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=22909 ops=1 watches=24
[1114025874588] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1114319892840] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1115086684965] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=134000 nodes=15723 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1128203569317] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=135000 nodes=15845 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1139620967628] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=136000 nodes=15955 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1151243379429] [INFO] [bloom] [CPU3] [bloom] graph pointer state diverged from streamed cursor: cursor=(960, 540) graph=(400, 300)
[1151264763693] [INFO] [bloom] [CPU3] [cursor metrics] frame=600 moves=0 rasterizations=1 cursor_only_frames=0 damage_rects=1
[1151831567205] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=137000 nodes=16073 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1155031072899] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1166900993907] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=138000 nodes=16214 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1172261611290] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1178941242336] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=139000 nodes=16330 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1191509471955] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=23933 ops=1 watches=24
[1194084763674] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=140000 nodes=16461 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1200172676637] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1207082926005] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=141000 nodes=16587 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1219763743011] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=142000 nodes=16705 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1222516348812] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1233753230325] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=143000 nodes=16838 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1247512206402] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=144000 nodes=16960 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1259953010217] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=145000 nodes=17084 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1267324363437] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #4: 128 KB align=8 total=351MB
[1269165288765] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=24957 ops=1 watches=24
[1271226604131] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=146000 nodes=17213 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1277928109425] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #5: 128 KB align=8 total=353MB
[1281971320509] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=147000 nodes=17341 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1291659814335] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=148000 nodes=17457 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1303022940285] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1303117815384] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1306529769831] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=149000 nodes=17594 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1319442709695] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=150000 nodes=17718 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1323000570639] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1331523937029] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=151000 nodes=17822 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1349561879589] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=25981 ops=1 watches=24
[1350178439247] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=152000 nodes=17963 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1363608310944] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=153000 nodes=18083 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1375756314075] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=154000 nodes=18193 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1391299753833] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=155000 nodes=18328 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1404049745934] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=156000 nodes=18444 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1416897084735] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=157000 nodes=18562 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1431586083774] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1433991156972] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=158000 nodes=18705 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1435604046855] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=27005 ops=1 watches=24
[1446822992604] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=159000 nodes=18823 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1461347509512] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=160000 nodes=18952 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1477831177119] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=161000 nodes=19074 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1491565443654] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=162000 nodes=19184 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1493970375777] [INFO] [bloom] [CPU3] [bloom] graph pointer state diverged from streamed cursor: cursor=(960, 540) graph=(400, 300)
[1493990460237] [INFO] [bloom] [CPU3] [cursor metrics] frame=720 moves=0 rasterizations=1 cursor_only_frames=0 damage_rects=1
[1504324151517] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1506053822691] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=163000 nodes=19313 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1521862791339] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=164000 nodes=19441 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1527195125709] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=28029 ops=1 watches=24
[1535086455375] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=165000 nodes=19559 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1550826042216] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=166000 nodes=19694 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1551586043766] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1551654845103] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1551707266890] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1564731348873] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=167000 nodes=19810 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1575949712271] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1576013365344] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1577895340428] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=168000 nodes=19930 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1593208550274] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=169000 nodes=20055 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1598699335299] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1612401391656] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=170000 nodes=20179 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1621208200821] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=29053 ops=1 watches=24
[1626182489415] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=171000 nodes=20297 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1649776827633] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=172000 nodes=20432 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1671547867605] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=173000 nodes=20554 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1685178086097] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=174000 nodes=20668 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1699320522603] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=175000 nodes=20789 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1707002441430] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1731518889276] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=176000 nodes=20919 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1747683194763] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=30077 ops=1 watches=24
[1749110397837] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=177000 nodes=21039 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1763510299353] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=178000 nodes=21151 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1773758501922] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1774316997222] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1780543512858] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=179000 nodes=21286 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1794366234936] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=180000 nodes=21402 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1808600272083] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=181000 nodes=21510 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1820984728827] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1826649587730] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=182000 nodes=21643 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1840955049384] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=183000 nodes=21761 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1845567857529] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=31101 ops=1 watches=24
[1854763617201] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=184000 nodes=21873 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1862602722354] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=30 len=120
[1862624264787] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 110 byte frame (114 encoded) to netd rx_port=25
[1862649237405] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (114 bytes sent)
[1862696032428] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 110 bytes
[1872257340690] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=185000 nodes=22006 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1889017729896] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=186000 nodes=22122 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1903249184760] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=187000 nodes=22234 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1918048983477] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1918238295600] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1918393093551] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1918441031892] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1918524871725] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[1922783589144] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=188000 nodes=22359 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1939763081355] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=189000 nodes=22485 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1944628754772] [INFO] [bloom] [CPU3] [bloom] graph pointer state diverged from streamed cursor: cursor=(960, 540) graph=(400, 300)
[1944697965045] [INFO] [bloom] [CPU3] [cursor metrics] frame=840 moves=0 rasterizations=1 cursor_only_frames=0 damage_rects=1
[1950839282139] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=32125 ops=1 watches=24
[1956254030280] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=190000 nodes=22601 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1975578551034] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=191000 nodes=22740 watches=24 history=1024 journal=1024 symbols=454 drops=0
[1993682947740] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=192000 nodes=22856 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2008051242681] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=193000 nodes=22964 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2025754269924] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=194000 nodes=23099 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2042594826294] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=195000 nodes=23215 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2062046309103] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=196000 nodes=23331 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2062337677092] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=33149 ops=1 watches=24
[2082272031180] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=197000 nodes=23460 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2099499005934] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=198000 nodes=23574 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2111299051290] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2115525500979] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=199000 nodes=23684 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2138781904644] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=200000 nodes=23819 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2156906014395] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=201000 nodes=23939 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2163913042368] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2180081583207] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=202000 nodes=24051 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2189468109543] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=34173 ops=1 watches=24
[2198992390056] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2199698045490] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2200984433142] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=203000 nodes=24182 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2222089265319] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=204000 nodes=24306 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2233762229919] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2242713936276] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=205000 nodes=24416 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2261837710209] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2265864278766] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=206000 nodes=24543 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2290716545535] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=207000 nodes=24659 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2293227679644] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2293377456777] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2293477808391] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2316038953107] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=208000 nodes=24785 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2329817190591] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=35197 ops=1 watches=24
[2336903872446] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=209000 nodes=24905 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2357473785183] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=210000 nodes=25032 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2357812490088] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2357918538789] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2375286093213] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=211000 nodes=25148 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2382366462993] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2391064045953] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=212000 nodes=25260 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2403837657705] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2410517681253] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=213000 nodes=25391 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2428015876605] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=214000 nodes=25511 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2445843477033] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=215000 nodes=25627 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2446073495778] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=36221 ops=1 watches=24
[2463829239828] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu stats: frames=120, rects_in=124, transfers=126, flushes=126, union_flush=116, per_rect_flush=2, frame_pool=true
[2464236272784] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=216000 nodes=25760 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2482879609518] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=217000 nodes=25882 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2488622725380] [INFO] [bloom] [CPU3] [bloom] graph pointer state diverged from streamed cursor: cursor=(960, 540) graph=(400, 300)
[2488635603399] [INFO] [bloom] [CPU3] [cursor metrics] frame=960 moves=0 rasterizations=1 cursor_only_frames=0 damage_rects=1
[2498061526026] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=218000 nodes=25996 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2516126460309] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=219000 nodes=26123 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2533817691834] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=220000 nodes=26245 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2548873756704] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=221000 nodes=26355 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2554834549179] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=37245 ops=1 watches=24
[2566136950146] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=222000 nodes=26480 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2584705524654] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=223000 nodes=26598 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2593935972606] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2593985423073] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2594286700962] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2600877914514] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=224000 nodes=26712 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2620689614202] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=225000 nodes=26832 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2640466611090] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=226000 nodes=26963 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2658184486716] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=227000 nodes=27079 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2671520100942] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=38269 ops=1 watches=24
[2677733513532] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=228000 nodes=27193 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2681827809243] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2681959258638] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2698156512744] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=229000 nodes=27326 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2706354060795] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2717844870675] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=230000 nodes=27448 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2738614036389] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=231000 nodes=27558 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2762665264755] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=232000 nodes=27691 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2782858576608] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=233000 nodes=27803 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2798583596193] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=234000 nodes=27911 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2799002019363] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=39293 ops=1 watches=24
[2821309807239] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=235000 nodes=28038 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2839563503469] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=236000 nodes=28148 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2857028977803] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=237000 nodes=28260 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2878288907472] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=238000 nodes=28385 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2883785004903] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2898432282702] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=239000 nodes=28505 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2914972572204] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=240000 nodes=28613 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2924504003124] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=40317 ops=1 watches=24
[2935998007998] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2936058653649] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=241000 nodes=28740 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2936596112472] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2936716284381] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2936974253664] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[2957809373871] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=242000 nodes=28866 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2977708488051] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=243000 nodes=28970 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2994926115621] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=244000 nodes=29088 watches=24 history=1024 journal=1024 symbols=454 drops=0
[2996232283110] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3016651441023] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=245000 nodes=29219 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3029602654452] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3036118025061] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=246000 nodes=29327 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3045737721807] [INFO] [bloom] [CPU3] [bloom] graph pointer state diverged from streamed cursor: cursor=(960, 540) graph=(400, 300)
[3045774032532] [INFO] [bloom] [CPU3] [cursor metrics] frame=1080 moves=0 rasterizations=1 cursor_only_frames=0 damage_rects=1
[3051522164658] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3053892922707] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=41341 ops=1 watches=24
[3055182838323] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=247000 nodes=29441 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3074625920883] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=248000 nodes=29560 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3093965825280] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=249000 nodes=29678 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3116025893508] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=250000 nodes=29790 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3141728526024] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=251000 nodes=29929 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3161664916905] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=252000 nodes=30045 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3172873035156] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3179941389306] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=253000 nodes=30157 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3186169500360] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=42365 ops=1 watches=24
[3196979722563] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3197054654838] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3201378773472] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=254000 nodes=30288 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3225917258124] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=255000 nodes=30418 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3247888118409] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=256000 nodes=30532 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3267352268994] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3267668647155] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3268403474688] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=257000 nodes=30653 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3292641397287] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=258000 nodes=30773 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3310193032278] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=259000 nodes=30879 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3318683813049] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3318754303392] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3322235720628] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=43389 ops=1 watches=24
[3328962089145] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=260000 nodes=30991 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3355217359098] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=261000 nodes=31134 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3372385675791] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=262000 nodes=31244 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3391485639003] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3395388612792] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=263000 nodes=31358 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3414945039990] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=31 len=120
[3414970666767] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 110 byte frame (114 encoded) to netd rx_port=25
[3414990199599] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (114 bytes sent)
[3415047300786] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 110 bytes
[3419524045410] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=264000 nodes=31497 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3441600230541] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=265000 nodes=31611 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3443963206233] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3463853354214] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=266000 nodes=31721 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3463916128398] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=44413 ops=1 watches=24
[3494595049836] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=267000 nodes=31856 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3515389771938] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=268000 nodes=31976 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3542969599323] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=269000 nodes=32092 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3557594560245] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3558169801098] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3571441458783] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=270000 nodes=32221 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3586627801614] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3586701324195] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3587167772529] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3599584767867] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=271000 nodes=32343 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3634312939455] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=272000 nodes=32457 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3639624272757] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=45437 ops=1 watches=24
[3657732367707] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=273000 nodes=32592 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3674986137801] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3678434702844] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=274000 nodes=32710 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3697156738803] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=275000 nodes=32818 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3707288529186] [INFO] [bloom] [CPU3] [bloom] graph pointer state diverged from streamed cursor: cursor=(960, 540) graph=(400, 300)
[3707320040952] [INFO] [bloom] [CPU3] [cursor metrics] frame=1200 moves=0 rasterizations=1 cursor_only_frames=0 damage_rects=1
[3717250531764] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=276000 nodes=32943 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3722666821104] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3740751565155] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=277000 nodes=33071 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3759350390367] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=278000 nodes=33181 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3769943615229] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=46461 ops=1 watches=24
[3780155123976] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=279000 nodes=33305 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3787993717761] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3802814533056] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=280000 nodes=33426 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3821883063240] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=281000 nodes=33540 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3842760411171] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=282000 nodes=33656 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3861054060699] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #6: 256 KB align=8 total=691MB
[3865121870379] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=283000 nodes=33793 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3875118030927] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #7: 256 KB align=8 total=694MB
[3876625734927] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=284000 nodes=33909 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3886175234568] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=47485 ops=1 watches=24
[3889210155765] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=285000 nodes=34019 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3890995796457] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3904628573439] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=286000 nodes=34158 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3910876025232] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3917466115827] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=287000 nodes=34276 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3933476915808] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=288000 nodes=34392 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3958500769278] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=289000 nodes=34533 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3970808806965] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[3977702395194] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=290000 nodes=34647 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3995792897580] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=291000 nodes=34751 watches=24 history=1024 journal=1024 symbols=454 drops=0
[3997178210454] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=48509 ops=1 watches=24
[4020144559134] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=292000 nodes=34888 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4022829071118] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4022907934188] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4040589648777] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=293000 nodes=34998 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4047238276752] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4047557582376] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4060017209061] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=294000 nodes=35108 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4083394566240] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4088163586452] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=295000 nodes=35251 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4122915301296] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=296000 nodes=35381 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4130509963701] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4146865438650] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=297000 nodes=35497 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4149888012864] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=49533 ops=1 watches=24
[4159176930447] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4159333770405] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4159454990064] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4159868472837] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4160732800752] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4172876662845] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=298000 nodes=35636 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4190190594447] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4198458661452] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=299000 nodes=35766 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4219061879472] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=300000 nodes=35876 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4242833349084] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=301000 nodes=36003 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4253752470891] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4254222476559] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4254287468970] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4267830395268] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=302000 nodes=36123 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4283581950384] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4289107616409] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=303000 nodes=36237 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4295797260579] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=50557 ops=1 watches=24
[4302293334240] [INFO] [bloom] [CPU3] [bloom] graph pointer state diverged from streamed cursor: cursor=(960, 540) graph=(400, 300)
[4302341390787] [INFO] [bloom] [CPU3] [cursor metrics] frame=1320 moves=0 rasterizations=1 cursor_only_frames=0 damage_rects=1
[4311377360895] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=304000 nodes=36359 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4339575337185] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=305000 nodes=36500 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4360312536624] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=306000 nodes=36616 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4385292084591] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=307000 nodes=36744 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4415194568235] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=308000 nodes=36888 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4417921855248] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4418013056160] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4440238962951] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=309000 nodes=37013 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4444057507692] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=51581 ops=1 watches=24
[4463018110056] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=310000 nodes=37133 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4490313520572] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=311000 nodes=37270 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4513257594054] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=312000 nodes=37392 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4513953224319] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4536320975769] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=313000 nodes=37510 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4546561951701] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4546668848964] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4546763872728] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4546918727142] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4547438145987] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4548144173199] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4565854843836] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=314000 nodes=37657 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4589908634586] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=315000 nodes=37773 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4595380354653] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=52605 ops=1 watches=24
[4614317118378] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=316000 nodes=37895 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4618315585323] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4618409642022] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4642984105056] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=317000 nodes=38036 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4660714640637] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4661080779333] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4668208300206] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=318000 nodes=38160 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4692674433183] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=319000 nodes=38274 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4722073120545] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=320000 nodes=38413 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4747176467880] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=321000 nodes=38531 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4751734911726] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=53629 ops=1 watches=24
[4760961909165] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4771010568159] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=322000 nodes=38647 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4797939372930] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4797995767191] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4798101884466] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4800158623194] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=323000 nodes=38786 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4827569594793] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=324000 nodes=38912 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4852503051891] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=325000 nodes=39030 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4880129543124] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=326000 nodes=39165 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4908464646426] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=327000 nodes=39301 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4911501172875] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=54653 ops=1 watches=24
[4933317530559] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=328000 nodes=39421 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4961396853897] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=329000 nodes=39556 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4970477141982] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[4989440813247] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=330000 nodes=39686 watches=24 history=1024 journal=1024 symbols=454 drops=0
[4996477561251] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5013878083665] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=331000 nodes=39802 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5020920567498] [INFO] [bloom] [CPU3] [bloom] graph pointer state diverged from streamed cursor: cursor=(960, 540) graph=(400, 300)
[5020955202285] [INFO] [bloom] [CPU3] [cursor metrics] frame=1440 moves=0 rasterizations=1 cursor_only_frames=0 damage_rects=1
[5038773663000] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5040126977655] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5042709238380] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=332000 nodes=39937 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5070751786485] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=55677 ops=1 watches=24
[5072070342537] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=333000 nodes=40071 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5078537365803] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5096162421603] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=334000 nodes=40185 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5121167371320] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=335000 nodes=40297 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5153787277914] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=336000 nodes=40436 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5177223162375] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=337000 nodes=40550 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5184232231728] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=120
[5184304846248] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 110 byte frame (114 encoded) to netd rx_port=25
[5184335547534] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (114 bytes sent)
[5184396680826] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 110 bytes
[5201031487014] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=338000 nodes=40674 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5215738230813] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5216284282374] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5229657859734] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=339000 nodes=40809 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5232833543190] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=56701 ops=1 watches=24
[5254800189408] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=340000 nodes=40931 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5278425036060] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5279570961159] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=341000 nodes=41051 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5307548763693] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=342000 nodes=41186 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5340624570237] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=343000 nodes=41308 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5371015173150] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=344000 nodes=41432 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5401923065484] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=345000 nodes=41561 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5406602892240] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=57725 ops=1 watches=24
[5436335784198] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5439491567070] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=346000 nodes=41703 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5465753803224] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=347000 nodes=41817 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5484198313506] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5484339515556] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5484437986005] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5484507813972] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5484594975519] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5502250185498] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=348000 nodes=41927 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5538341855937] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5556114760416] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=349000 nodes=42082 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5589824340969] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=350000 nodes=42210 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5592426349389] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5592501110922] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5623099077750] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=351000 nodes=42328 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5632447449882] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=58749 ops=1 watches=24
[5633142556371] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5669906015169] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=352000 nodes=42471 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5701860901440] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=353000 nodes=42589 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5726729186244] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=354000 nodes=42701 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5754845615475] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=355000 nodes=42834 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5759965965699] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5783482606044] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=356000 nodes=42956 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5808933626100] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=357000 nodes=43076 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5817595629978] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=59773 ops=1 watches=24
[5840877390612] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=358000 nodes=43207 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5878945648950] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=359000 nodes=43345 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5887479709980] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5913489711591] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=360000 nodes=43471 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5917033725042] [INFO] [bloom] [CPU3] [bloom] graph pointer state diverged from streamed cursor: cursor=(960, 540) graph=(400, 300)
[5917058966742] [INFO] [bloom] [CPU3] [cursor metrics] frame=1560 moves=0 rasterizations=1 cursor_only_frames=0 damage_rects=1
[5944040171619] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=361000 nodes=43589 watches=24 history=1024 journal=1024 symbols=454 drops=0
[5959906318641] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5959990259784] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[5984367321042] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=362000 nodes=43728 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6015841476513] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=363000 nodes=43844 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6022474285641] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=60797 ops=1 watches=24
[6048799479525] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=364000 nodes=43968 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6055717157556] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[6083573173491] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=365000 nodes=44115 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6113673344031] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=366000 nodes=44237 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6129043754757] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[6139463809083] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=367000 nodes=44351 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6174034962018] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=368000 nodes=44494 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6204054646218] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=369000 nodes=44614 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6204912248832] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=61821 ops=1 watches=24
[6229866061155] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=370000 nodes=44736 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6259680169380] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=371000 nodes=44873 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6278488698471] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[6278553861921] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[6278623383120] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[6290920845924] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=372000 nodes=45005 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6317306088522] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=373000 nodes=45127 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6346897498293] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=374000 nodes=45262 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6362646552273] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[6377603660340] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=62845 ops=1 watches=24
[6379910758848] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=375000 nodes=45388 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6406788955182] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=376000 nodes=45510 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6435040694148] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=377000 nodes=45628 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6468188471019] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=378000 nodes=45769 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6493898552493] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=379000 nodes=45883 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6514874732256] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[6514940206269] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu stats: frames=120, rects_in=120, transfers=120, flushes=120, union_flush=120, per_rect_flush=0, frame_pool=true
[6521559307395] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=380000 nodes=46001 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6552561349587] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=63869 ops=1 watches=24
[6555570834186] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=381000 nodes=46142 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6560920978326] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[6583656262260] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=382000 nodes=46260 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6589473376233] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[6610604758692] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=383000 nodes=46378 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6633406390380] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[6633522770787] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[6641578513977] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=384000 nodes=46515 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6673149133464] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=385000 nodes=46645 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6677809836588] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[6677951935380] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[6698908258977] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=386000 nodes=46757 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6729806084265] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=387000 nodes=46892 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6730716991713] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=64893 ops=1 watches=24
[6752730725367] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[6754290451227] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[6763226496279] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=388000 nodes=47014 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6790023760740] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[6790771065111] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=389000 nodes=47136 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6805608865803] [INFO] [bloom] [CPU3] [bloom] graph pointer state diverged from streamed cursor: cursor=(960, 540) graph=(400, 300)
[6805634505384] [INFO] [bloom] [CPU3] [cursor metrics] frame=1680 moves=0 rasterizations=1 cursor_only_frames=0 damage_rects=1
[6819799018428] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=390000 nodes=47258 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6825859983657] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=120
[6825886831104] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 110 byte frame (114 encoded) to netd rx_port=25
[6825899953125] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (114 bytes sent)
[6825991370253] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 110 bytes
[6835466451693] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[6853302083859] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=391000 nodes=47393 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6882657061203] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=392000 nodes=47509 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6903129880119] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[6903870222909] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[6912661837092] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=393000 nodes=47631 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6920594324358] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=65917 ops=1 watches=24
[6944840855337] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=394000 nodes=47764 watches=24 history=1024 journal=1024 symbols=454 drops=0
[6974838303741] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=395000 nodes=47884 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7003739059944] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=396000 nodes=48008 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7030542185172] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[7030615384023] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[7036623779559] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=397000 nodes=48149 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7069959674844] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=398000 nodes=48277 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7100592416643] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=399000 nodes=48401 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7106062177269] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=66941 ops=1 watches=24
[7110824407566] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[7138756929612] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=400000 nodes=48534 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7175199024015] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[7194532079691] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=401000 nodes=48678 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7234859058570] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=402000 nodes=48800 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7270964326197] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=403000 nodes=48920 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7314690586434] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=404000 nodes=49059 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7327673597607] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[7328424108192] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[7354659268770] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=67965 ops=1 watches=24
[7355604500904] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=405000 nodes=49187 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7391012072070] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=406000 nodes=49305 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7433931981927] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=407000 nodes=49446 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7472152851405] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=408000 nodes=49570 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7512100223331] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=409000 nodes=49696 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7520419713015] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[7564582723071] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=410000 nodes=49847 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7599203194473] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=68989 ops=1 watches=24
[7615929733401] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=411000 nodes=49977 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7657246353339] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=412000 nodes=50097 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7681017833214] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[7682102217900] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[7703986670193] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=413000 nodes=50244 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7741201010235] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=414000 nodes=50370 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7760244706104] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=120
[7760292871155] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 110 byte frame (114 encoded) to netd rx_port=25
[7760321132190] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (114 bytes sent)
[7760335433136] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 110 bytes
[7784964008547] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=415000 nodes=50490 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7822387282545] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=416000 nodes=50625 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7848028863114] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=70013 ops=1 watches=24
[7854192293640] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[7868307422778] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=417000 nodes=50755 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7892825860029] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[7892893658661] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[7892943917991] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[7910150633715] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=418000 nodes=50877 watches=24 history=1024 journal=1024 symbols=454 drops=0
[7940096073954] [INFO] [bloom] [CPU3] [bloom] graph pointer state diverged from streamed cursor: cursor=(960, 540) graph=(400, 300)
[7940149500921] [INFO] [bloom] [CPU3] [cursor metrics] frame=1800 moves=0 rasterizations=1 cursor_only_frames=0 damage_rects=1
[7956479867061] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[7966450935135] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=419000 nodes=51018 watches=24 history=1024 journal=1024 symbols=454 drops=0
[8028371029338] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=420000 nodes=51164 watches=24 history=1024 journal=1024 symbols=454 drops=0
[8085687998289] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=421000 nodes=51296 watches=24 history=1024 journal=1024 symbols=454 drops=0
[8113543717851] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[8141443008435] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=422000 nodes=51428 watches=24 history=1024 journal=1024 symbols=454 drops=0
[8149997464521] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=71037 ops=1 watches=24
[8174349021267] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[8214232145229] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=423000 nodes=51591 watches=24 history=1024 journal=1024 symbols=454 drops=0
[8269232098083] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=424000 nodes=51721 watches=24 history=1024 journal=1024 symbols=454 drops=0
[8329884966777] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=425000 nodes=51865 watches=24 history=1024 journal=1024 symbols=454 drops=0
[8332491396042] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[8405872663947] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=426000 nodes=52034 watches=24 history=1024 journal=1024 symbols=454 drops=0
[8461656274269] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=427000 nodes=52164 watches=24 history=1024 journal=1024 symbols=454 drops=0
[8482062417939] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=72061 ops=1 watches=24
[8534469183999] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=428000 nodes=52292 watches=24 history=1024 journal=1024 symbols=454 drops=0
[8722724377389] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=429000 nodes=52469 watches=24 history=1024 journal=1024 symbols=454 drops=0
[8794173307953] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=430000 nodes=52607 watches=24 history=1024 journal=1024 symbols=454 drops=0
[8859409619379] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=431000 nodes=52739 watches=24 history=1024 journal=1024 symbols=454 drops=0
[8915887960782] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[8933466430086] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=432000 nodes=52896 watches=24 history=1024 journal=1024 symbols=454 drops=0
[8967475950642] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=73085 ops=1 watches=24
[8990850805365] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=433000 nodes=53028 watches=24 history=1024 journal=1024 symbols=454 drops=0
[9001353609732] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[9002595821109] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[9043313916483] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=434000 nodes=53156 watches=24 history=1024 journal=1024 symbols=454 drops=0
[9071216598114] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[9109535046678] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=435000 nodes=53305 watches=24 history=1024 journal=1024 symbols=454 drops=0
[9179094496737] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=436000 nodes=53441 watches=24 history=1024 journal=1024 symbols=454 drops=0
[9240047484624] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=437000 nodes=53567 watches=24 history=1024 journal=1024 symbols=454 drops=0
[9305013022470] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=438000 nodes=53697 watches=24 history=1024 journal=1024 symbols=454 drops=0
[9321722845908] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=74109 ops=1 watches=24
[9368523353607] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=439000 nodes=53854 watches=24 history=1024 journal=1024 symbols=454 drops=0
[9381498780258] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[9410157205548] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=440000 nodes=53970 watches=24 history=1024 journal=1024 symbols=454 drops=0
[9450911291052] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=441000 nodes=54092 watches=24 history=1024 journal=1024 symbols=454 drops=0
[9480049307553] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[9480351751959] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[9480489591639] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[9520742545842] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=442000 nodes=54241 watches=24 history=1024 journal=1024 symbols=454 drops=0
[9575397189864] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=443000 nodes=54367 watches=24 history=1024 journal=1024 symbols=454 drops=0
[9616607419947] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=120
[9616644342195] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 110 byte frame (114 encoded) to netd rx_port=25
[9616670139846] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (114 bytes sent)
[9616702523043] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 110 bytes
[9626792522424] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=75133 ops=1 watches=24
[9631478478864] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=444000 nodes=54497 watches=24 history=1024 journal=1024 symbols=454 drops=0
[9645846279960] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[9699549473691] [IN
```
</details>
