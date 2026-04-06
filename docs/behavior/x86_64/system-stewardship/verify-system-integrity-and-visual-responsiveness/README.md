# ❌ Scenario: Verify System Integrity and Visual Responsiveness

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booting | ✅ | 0ms | - - - |
| 2 | When I wait for the system to reach ready state | ✅ | 5058ms | - - - |
| 3 | Then the serial output should contain "SPROUT:" | ✅ | 0ms | - - - |
| 4 | And the serial output should contain "Supervisor starting" | ✅ | 0ms | - - - |
| 5 | And the serial output should contain "bloom: First frame rendered" | ❌ | 121115ms | - [📜](./05/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11178068187] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11183628720] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11187148665] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11189087019] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11190261621] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11190880239] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11191530999] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11192100678] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11192676264] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11193300129] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11193881688] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11194474698] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11195161362] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11195809713] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11196500568] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11197098561] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11197713186] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11198301246] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11198907819] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11199496539] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11200084731] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11200661340] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11201244912] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11201826042] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11202450303] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11203070043] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11203660380] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11204298303] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11204880786] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11205476337] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11206079049] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11206703310] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11207321433] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11207928105] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11208537186] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11209413237] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11210163624] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11210872233] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11211568962] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11212291497] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11213056635] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11213781381] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11215119630] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11216627730] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11217593178] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11218137216] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11218645185] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11219162625] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11219737650] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11220255717] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11220765270] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11221282875] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11221793319] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11222332209] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11222957130] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11223754608] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11224422792] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11224990392] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11225534430] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11226123975] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11226670422] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11227230960] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11227774569] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11228336856] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11228879178] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11229460704] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11230005831] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11230568415] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11231114565] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11231676753] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11232219372] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11232798324] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11233428657] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11234008335] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11234547291] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11235104364] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11235644607] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11236292826] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11236841682] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11237403078] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11237945697] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11238508050] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11239072878] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11239637904] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11240180490] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11240740797] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11241280974] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11241837717] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11242402215] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11242962126] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11243501742] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11244059145] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11244599883] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11245157649] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11245718715] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11246278329] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11246819463] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11247378318] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11247917175] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11248473885] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11249058645] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11249616972] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11250168369] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11250817017] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11251546119] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11252175990] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11252742468] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11253597564] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11489758335] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[11501523231] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11506283316] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11507507649] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11508332715] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11512566516] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11514188499] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11515232817] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11515920933] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11516574300] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11517224301] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11518166715] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11519123583] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11519797476] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11520456816] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11521111173] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11521766223] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11523004053] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11523997452] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11524715268] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11526767868] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11527702725] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11528684508] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11530413576] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11532124329] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11532935469] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11533465053] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11534218278] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[11901131352] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[11902129470] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[11905470225] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[11906328126] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[11907111018] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[11908617699] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[11920738830] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[11922001146] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[11922748761] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[11924471691] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[11925008733] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[11927369520] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[11935004037] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[11936704527] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[11950027914] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[11950641912] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[11966023806] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[11966576721] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[11968547679] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[11969739408] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[11970823392] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[11973011589] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[11973831738] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12008548497] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62275600 ticks/sec), init_cnt=622756 for 100Hz
[12009902256] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12010688844] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12011797281] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12017219775] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12047325180] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12048259740] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12051079260] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12052353522] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12053420808] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12056045298] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12057319230] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12075987462] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12076944561] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12077632314] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12078310464] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12079857603] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12081192156] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12081930069] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12106972944] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12108014952] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12109365741] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12110407485] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12111597498] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12112649406] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12114031479] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12115871526] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12123477069] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12124741002] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12127079052] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12128338200] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12132064329] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12133806894] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12134616285] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12135968196] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12137012745] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12138016902] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12150278877] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12153311643] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12154546998] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12155496210] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12176753424] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12202607769] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12205005285] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12209670429] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12211766061] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12215200008] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12217873107] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12220003323] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12220814760] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12222064998] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12230230650] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12232069377] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12234816198] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12237929715] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12242877372] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12243724218] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12244773057] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12257793669] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12258679917] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12266080959] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12266913087] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12293821749] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12294722550] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[12624042090] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13094913612] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13122741819] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[13161116133] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[14456727285] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=962 journal=770 symbols=96 drops=0
[15158763939] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15250330788] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15251375766] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[15345365673] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[15409681980] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[15434356080] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[15435308889] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[15436055679] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[15439475370] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[15454674675] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[15473427090] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[15475766823] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[15527248770] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[15552183834] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[15552959532] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[15558041565] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[15582972900] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[15601031622] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[15606486291] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[15607476291] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[15673961556] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[15675635844] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[15761815080] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[15824907846] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[15838683729] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[15843425961] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[15845625147] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[15871670562] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[15913166610] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[15918658305] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[15919677444] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[15920561052] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[15921438621] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[15922098753] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[15922742550] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[15923504619] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[15924238011] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[15924843528] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[15925477986] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[15926092083] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[15926738817] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[15927382350] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[15928060896] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[15928766073] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[15929407131] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[15930043338] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[15930651627] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[15931282158] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[15931892328] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[15932483028] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[15933095013] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[15933706965] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[15934319181] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[15934975485] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[15935596710] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[15936225855] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[15936984063] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[15937759068] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[15938657295] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[15939429693] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[15940145001] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[15940843974] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[15941526051] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[15942235188] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[15943079625] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[15943809156] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[15944536146] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[15945250002] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[15946028901] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[15946757739] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[15947936928] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[15949452948] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[15951170235] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[15951938706] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[15959866758] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[15973639605] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[15977851989] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[15986810070] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[15987486603] [CONTRACT] [kernel] [CPU0] Spawning init process...
[15989489538] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[15992183361] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[15992880387] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [15997646940] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[16001027526] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[16012263993] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[16014441531] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[16018065096] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[16019154888] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[16027976910] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[16032213714] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[16035454545] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[16036489161] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[16040781240] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[16044746916] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[16045938018] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[16050331440] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[16051654278] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[16053990447] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[16055909331] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16060390467] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16062148641] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16063997994] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16066250508] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16068417255] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16070601195] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16075733454] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[16117092321] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[16125058356] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[16129713633] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[16134195594] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[16137256707] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[16141390947] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[16146830271] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[16151523399] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[16156197981] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[16161114090] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[16166712705] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[16172515194] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[16179888945] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[16185399714] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[16191548604] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[16197706239] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[16202894763] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[16207783185] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[16212648936] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[16217978469] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[16222964307] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[16227763959] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[16233761148] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[16238753982] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[16246119780] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[16250921643] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[16255537683] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[16260794649] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[16265441148] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[16269727122] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[16273500078] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[16276882149] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[16281509772] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[16286144127] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[16290736440] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[16295164941] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[16299876978] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[16304309670] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[16309068996] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[16314037344] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[16319048790] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[16322174517] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[16341426816] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[16484419611] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[16493007366] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[16493865168] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16495489725] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16499868462] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[16501109757] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16509079620] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[16513846932] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16515205872] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16517111457] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[16521206262] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[16522070136] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16523774421] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[16524730728] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16529095605] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[16530960336] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16537737546] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[16540378206] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[16541340519] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[16542672366] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[16546217292] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[16549492344] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[16551199203] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[16553196264] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[16557414456] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:54:24 = 1775440464 unix_secs
[16559012679] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775440464, mono_ns=8279292004, offset=1775440455720707996ns
[16560742440] [INFO] [rtc_cmos] [CPU1] System clock anchored
[16572467571] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[16605550863] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[16618978332] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[16619833230] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16621432773] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16627003107] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[16629401052] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[16636708836] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[16640077674] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[16644108756] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16645817859] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[16650826038] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[16655970441] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[16657956018] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[16658788707] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16660511373] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16667146155] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[16669435761] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[16676817696] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[16680242304] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[16681845246] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16683330114] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16685186397] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[16689951069] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[16692148638] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[16696503945] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[16697848266] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[16701127938] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[16703219742] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[16705131102] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[16714606524] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[16716166896] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[17200634286] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17205479775] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[17206462119] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17208024471] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17214758682] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17216970870] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17224036467] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[17227537734] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[17228843973] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[17230752396] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352976 RFLAGS_BEFORE=130 CR3_BEFORE=59662336 fs_base=0 gs_base=18446744071564586576
[17236467303] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[17237382228] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[17239390311] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[17240883000] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[17244940152] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[17246324766] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[17247549066] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[17249820753] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[17251066305] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[17252153127] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[17254547178] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[17255435736] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[17256426924] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[17257225491] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[17258285187] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[17259094017] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[17260498068] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[17261369994] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[17262662373] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[17266199412] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[17274810861] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[17276113074] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[17300762721] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[17304107073] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[17306898477] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[17308254942] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[17309470761] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[17310747135] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17313343839] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17323686237] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[17326013991] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17333835717] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[17337316392] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[17339900622] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[17341253919] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17343655857] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17348859660] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17350918365] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17358231297] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[17361308547] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[17362392762] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[17363673789] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500192 RFLAGS_BEFORE=130 CR3_BEFORE=68235264 fs_base=0 gs_base=18446744071564586640
[17366446746] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[17367153243] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17369421069] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17370788985] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[17371912635] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[17375257119] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[17376099807] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17377416507] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[17378789274] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[17379514284] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17380860420] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[17382732510] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[17384530482] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[17386220247] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[17387468604] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[17388851238] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[17390392272] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[17392208361] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[17394458730] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[17396417676] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[17399702925] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1236 port=2 model='                                        ' rpc_port=5
[17424543312] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[17426015178] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[17427309900] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[17428562448] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[17429919342] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[17431233831] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[17432661906] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[17434201290] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[17440691103] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[17442473895] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369433952 RFLAGS_BEFORE=134 CR3_BEFORE=59830272 fs_base=0 gs_base=18446744071564586608
[17447636580] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[17752171668] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
[17753181072] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[17755412301] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369582976 RFLAGS_BEFORE=130 CR3_BEFORE=68349952 fs_base=0 gs_base=18446744071564586576
[17758306599] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[17760231357] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[17775888966] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[17776788942] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17777996280] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[17779452933] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[17780343702] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17781870348] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[17782571532] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17789491203] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[17791782591] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[17794256832] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[17799748197] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[17802534255] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[17804620845] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[17806301535] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[17807155971] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17808978990] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17828392197] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[17832383019] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[17870918901] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[17934530295] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[17935484754] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[17936818515] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[17938094724] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[18654287124] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[18656983224] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[18658036584] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18659255769] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369719104 RFLAGS_BEFORE=130 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[18662488713] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[18664446273] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653568 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[18667751487] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[18669221604] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[18670017366] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[18671428875] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[18675659574] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[18677119428] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[18680132592] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[18681494238] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[18682788762] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[18684360255] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[18698446503] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[18705436827] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[18707168436] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[18708686073] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[18709972875] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[18711889779] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[18713495328] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[18715365834] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4c32000
[18716746587] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[18718781202] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[18720792156] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[18722146707] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[18732125676] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[18741359274] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[18751175487] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[18753518421] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[18755523138] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[18756973521] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[18758328237] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[18759706119] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[18761085783] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[18762433404] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[18763642062] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[18769188372] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=15, read=16)
[18774868464] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=17, read=18)
[18782182683] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18800892231] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[18803036802] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[18805686603] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[18806559717] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18808308750] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18809410125] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18825890853] [INFO] [netd] [CPU3] NETD: Driver TX port=15, RX port=18, link_up=true, mtu=1500
[18828839766] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[18833774685] [INFO] [netd] [CPU3] NETD: Created socket API port (write=19, read=20)
[18859186896] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([219, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[18871071582] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[18872134347] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[18877165560] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[18880842981] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[18903553680] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[18910507209] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[18911757084] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[18915017946] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
[18915788100] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18917645538] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[18918433809] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18919749354] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369950912 RFLAGS_BEFORE=134 CR3_BEFORE=80007168 fs_base=0 gs_base=18446744071564586576
[18923257386] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18926524188] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[18929259690] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[18939487776] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=19, our_write=21, our_read=22)
[18941650299] [INFO] [anther] [CPU1] anther: Connected to network stack
[18949191360] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18952638507] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[18960622032] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[18964094457] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[18966215532] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[18967216158] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18969042807] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18974884995] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18977651682] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18985398003] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[18989291937] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
[18990261312] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18991950912] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370099616 RFLAGS_BEFORE=134 CR3_BEFORE=81375232 fs_base=0 gs_base=18446744071564586640
[18995447460] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18996489831] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18997985292] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[18998842863] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18999888930] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[19008497541] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19012170771] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19019555379] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[19022485053] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
[19024535046] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[19025336946] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19026955497] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19036794381] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19040164374] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[19041139887] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0102438
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19043443188] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370166016 RFLAGS_BEFORE=134 CR3_BEFORE=81600512 fs_base=0 gs_base=18446744071564586576
[19050111399] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[19053747372] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[19056486669] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[19057221084] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19058769378] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19084441134] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[19089449643] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[19097247972] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[19102252224] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
[19105510644] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[19106651355] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19118303094] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010aff8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19121282862] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370298336 RFLAGS_BEFORE=130 CR3_BEFORE=81903616 fs_base=0 gs_base=18446744071564586640
[19131801084] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19135413726] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[19152393315] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[19156356285] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19167683799] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[19169451213] [INFO] [fontd] [CPU3] FONTD: Service node created, req=23, resp=26
[19173135234] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[19175714250] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[19176933237] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19179560664] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19186878942] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19188860658] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19190475414] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19192878012] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[19194377070] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f2298
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19197900249] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370382288 RFLAGS_BEFORE=130 CR3_BEFORE=82178048 fs_base=0 gs_base=18446744071564586576
[19229636646] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[19251029226] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[19256872107] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19258214118] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19259324964] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19261264638] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=236 pred=0 subj_lo=0
[19263373602] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[19264306809] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[19267306509] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[19269538431] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[19274986335] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=15, RX port=18
[19278823641] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[19283244486] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19284942171] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370033408 RFLAGS_BEFORE=134 CR3_BEFORE=80945152 fs_base=0 gs_base=18446744071564586608
[19290256755] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19291595301] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19293060336] [INFO] [nectar] [CPU2] NECTAR: Started.
[19293630279] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19295276682] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19306556115] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0102438
[19308198459] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19310043291] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19317709290] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=238 pred=0 subj_lo=0
[19322142015] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=237 subj_lo=0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19325154816] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370232096 RFLAGS_BEFORE=130 CR3_BEFORE=81747968 fs_base=0 gs_base=18446744071564586608
[19332497778] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[19335001455] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f2298
[19336449396] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1249) for kind 'Asset'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19338117051] [INFO] [fontd] [CPU3] FONTD: Service ready
[19339025508] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370449616 RFLAGS_BEFORE=130 CR3_BEFORE=82386944 fs_base=0 gs_base=18446744071564586608
[19346216439] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[19348243662] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[19350403875] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[19355159373] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[19381560924] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[19384504887] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19389999420] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[19404583275] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[19415262702] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19416295470] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19417376418] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19418574780] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[19421573292] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1252 backend=VirtIO-GPU
[19423621272] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[19424467953] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19426157817] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19428613380] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19435407156] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[19438916739] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19440407019] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19442151894] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19443763944] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=246 subj_lo=0
[19456261077] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19457607147] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19459079871] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19460272854] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=247 subj_lo=0
[19467945288] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19468983171] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19469976702] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19471107018] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=248 subj_lo=0
[19477852647] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19478794929] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19479763347] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19481215182] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=249 subj_lo=0
[19490467326] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19491606519] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19492874016] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19494720762] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=250 pred=0 subj_lo=0
[19523217846] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[19541797011] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=27, resp=30
[19543666593] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[19578577821] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[19593680634] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[19601373165] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[19604101803] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db478
[19605382071] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e4
[19606856577] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370553008 RFLAGS_BEFORE=130 CR3_BEFORE=83304448 fs_base=0 gs_base=18446744071564586640
[19610787372] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[19611621711] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19612920162] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[19613896995] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19618882140] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[19619769939] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[19620624408] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19621665162] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[19629329280] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[19632078675] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db478
[19633241826] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[19634855625] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370618544 RFLAGS_BEFORE=130 CR3_BEFORE=84353024 fs_base=0 gs_base=18446744071564586576
[19638812457] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[19641761073] [INFO] [echo] [CPU1] echo: starting up
[19642834596] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[19646789052] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[19653733869] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[19662495567] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[19664332710] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[19674347814] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[19675979169] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[19679201817] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[19680590919] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[19682681502] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19685196498] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[19688868078] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19689803331] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[19691446500] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[19693001130] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[19693747392] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[19694448972] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19695692478] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[19696591497] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19701799326] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19703750946] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[19710916335] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[19711793574] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[19713018798] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[19714651110] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[19715616228] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[19717603488] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[19719022059] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[19720006284] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[19721093238] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19721884677] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19723489302] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19724990967] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[19730256315] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[19731533481] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19735318911] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[19736138631] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19743731832] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[19746397473] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[19747220097] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f2a98
[19748330217] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19750038429] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370757808 RFLAGS_BEFORE=130 CR3_BEFORE=85118976 fs_base=0 gs_base=18446744071564586640
[19753586622] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[19754910813] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19757365518] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19760837415] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[19762192725] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[19763645319] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[19769554398] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[19773475953] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[19775876604] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f2a98
[19777102455] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19778491062] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[19779526338] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370823344 RFLAGS_BEFORE=130 CR3_BEFORE=85266432 fs_base=0 gs_base=18446744071564586576
[19789173261] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[19790469732] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[19797838500] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[19807830075] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[19833114279] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[19835657358] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[19838131500] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[19840503012] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[19841571519] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[19843823868] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[19866168894] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[19868336466] [INFO] [bloom] [CPU3] bloom: creating surface...
[19869433947] [INFO] [bloom] [CPU3] bloom: surface created!
[19870298778] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[19876982301] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[19881359850] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1263
[19882325859] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[19897482825] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[19901122989] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[19902645477] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[19904272905] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [19911936132] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[19920638595] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[19930383462] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[19938935379] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[19943271810] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[19945399353] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=3
[19946808915] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 3)
[19947772614] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[19949971503] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[19952773929] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[19961727093] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1263
T:07D0 T:0640 T:F0B0 [19993778772] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[19995481473] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=3)
[19996264134] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[19999656369] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[20008123674] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[20009665995] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[20012118654] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[20014044930] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[20017229826] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[20027701518] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=279
[20028825399] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[20030109693] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20031257367] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20032332177] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20033482491] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=279 subj_lo=0
T:1220 [20038492518] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[20040528651] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[20051231343] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1276)
[20052646977] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[20066558655] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=250
[20067838527] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[20068714974] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20069724411] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20070887661] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20072112456] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=250 pred=0 subj_lo=0
[20083761786] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1279)
[20085764820] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[20097065274] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=286
[20098271457] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[20099492754] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20100451734] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20101504896] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20102540535] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=286 subj_lo=0
[20113221447] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1281)
[20114667870] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[20267794008] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[20469317847] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[20490477876] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[20498093385] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[20624496948] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=664 watches=13 history=1024 journal=1024 symbols=314 drops=0
[20646737496] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[20791402698] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[20936404335] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[21095505387] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[21344194905] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[21521694084] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[21606361590] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[21705044262] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[21864478647] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[22020451497] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[22025223693] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[22026521220] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[22027570257] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[22028769675] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[22030104789] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=237 subj_lo=0
[22046213706] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[22047466947] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[22048772196] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[22050171231] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=338 pred=0 subj_lo=0
[22082421042] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[22235938791] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[22339025874] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=712 watches=15 history=1024 journal=1024 symbols=339 drops=0
[22449184791] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[22655995461] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[22757959158] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[22759518243] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[22857509103] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[22896245328] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22897493256] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22898675943] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22899991092] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=312 pred=0 subj_lo=0
[22949218017] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22950440601] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22951647543] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22952999421] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=340 pred=0 subj_lo=0
[23056895301] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[23171811894] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=0e57f834b3afbcb1)
[23191786134] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23193025317] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23194127913] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23195363070] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=341 pred=0 subj_lo=0
[23203717812] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[23211747669] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[23225296512] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[23226919155] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[23250438519] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[23319624966] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23320723635] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23321856426] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23323408548] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=342 pred=0 subj_lo=0
[23488221966] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[23510657313] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x1222d000
[23512676550] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[23515392153] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[23543549667] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[23590579914] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[23601108762] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[23606640618] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[23609283984] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[23612131620] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[23616823989] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[23638863006] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[23640691041] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[23642386878] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[23657929317] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x1222e000
[23659125567] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[23831136615] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=c1e9a7b90a789e5c)
[24105598968] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[24344417262] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[24613945620] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[24635770005] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=802 watches=19 history=1024 journal=1024 symbols=363 drops=0
[24865355460] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[25135192401] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[25459627743] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[25729521708] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[26034943011] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[26358194148] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[26380958373] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=840 watches=19 history=1024 journal=1024 symbols=363 drops=0
[26627147019] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[26823838833] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[26943304542] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[26969228748] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[27235785159] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[27601875906] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[28139002617] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=880 watches=19 history=1024 journal=1024 symbols=366 drops=0
[28551796152] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[29071895886] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2445 ops=1 watches=19
[29098152105] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[29799729894] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[29874543303] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=899 watches=19 history=1024 journal=1024 symbols=366 drops=0
[30276638832] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[30641883459] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[30815912787] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[31191674349] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[32019416979] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=954 watches=19 history=1024 journal=1024 symbols=393 drops=0
[34111318527] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[34204987443] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1049 watches=19 history=1024 journal=1024 symbols=451 drops=0
[34209770694] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[34472180127] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[34473581307] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[34498566663] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[34946024421] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[34996693545] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34998374037] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35000142672] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35002064922] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[35011947861] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35013510510] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35015455464] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35017427610] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=240 pred=0 subj_lo=0
[35026212672] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35027838483] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35029632363] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35031060966] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=453 pred=0 subj_lo=0
[35042884206] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35044294230] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35045396397] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: cop start_seq=0
[35046793584] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=452 pred=0 subj_lo=0
[35054984943] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[35056610457] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[35058472977] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[35060390904] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[35068100397] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[36024136170] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1080 watches=24 history=1024 journal=1024 symbols=454 drops=0
[37575495507] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1104 watches=24 history=1024 journal=1024 symbols=454 drops=0
[39188451984] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=1141 watches=24 history=1024 journal=1024 symbols=454 drops=0
[40814606811] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=1165 watches=24 history=1024 journal=1024 symbols=454 drops=0
[42455753373] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=16000 nodes=1193 watches=24 history=1024 journal=1024 symbols=454 drops=0
[43685807319] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[44010010506] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=3469 ops=1 watches=24
[44304418719] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=17000 nodes=1222 watches=24 history=1024 journal=1024 symbols=454 drops=0
[46047601545] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=18000 nodes=1248 watches=24 history=1024 journal=1024 symbols=454 drops=0
[47792530599] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=19000 nodes=1274 watches=24 history=1024 journal=1024 symbols=454 drops=0
[49820944833] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=20000 nodes=1300 watches=24 history=1024 journal=1024 symbols=454 drops=0
[51812058045] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=21000 nodes=1331 watches=24 history=1024 journal=1024 symbols=454 drops=0
[53555714553] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=22000 nodes=1357 watches=24 history=1024 journal=1024 symbols=454 drops=0
[55419902934] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=23000 nodes=1381 watches=24 history=1024 journal=1024 symbols=454 drops=0
[57377462142] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=24000 nodes=1410 watches=24 history=1024 journal=1024 symbols=454 drops=0
[57828941907] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[57831855081] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=2 (16384b)
[57835802310] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[57839170191] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[57991508784] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[59422183788] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=25000 nodes=1446 watches=24 history=1024 journal=1024 symbols=454 drops=0
[61114892586] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=26000 nodes=1470 watches=24 history=1024 journal=1024 symbols=454 drops=0
[62915167755] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=27000 nodes=1505 watches=24 history=1024 journal=1024 symbols=454 drops=0
[64773098049] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=28000 nodes=1531 watches=24 history=1024 journal=1024 symbols=454 drops=0
[66525683169] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=29000 nodes=1553 watches=24 history=1024 journal=1024 symbols=454 drops=0
[68221015797] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=30000 nodes=1579 watches=24 history=1024 journal=1024 symbols=454 drops=0
[70040506404] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=4493 ops=1 watches=24
[70430613528] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=31000 nodes=1612 watches=24 history=1024 journal=1024 symbols=454 drops=0
[72161415711] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=32000 nodes=1636 watches=24 history=1024 journal=1024 symbols=454 drops=0
[73988135364] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=33000 nodes=1660 watches=24 history=1024 journal=1024 symbols=454 drops=0
[75929851404] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=34000 nodes=1695 watches=24 history=1024 journal=1024 symbols=454 drops=0
[77906014725] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=35000 nodes=1725 watches=24 history=1024 journal=1024 symbols=454 drops=0
[79879359582] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=36000 nodes=1751 watches=24 history=1024 journal=1024 symbols=454 drops=0
[81705840711] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=37000 nodes=1782 watches=24 history=1024 journal=1024 symbols=454 drops=0
[84044263545] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=38000 nodes=1822 watches=24 history=1024 journal=1024 symbols=454 drops=0
[85908669165] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=39000 nodes=1848 watches=24 history=1024 journal=1024 symbols=454 drops=0
[87710179029] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=40000 nodes=1879 watches=24 history=1024 journal=1024 symbols=454 drops=0
[89940713907] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=41000 nodes=1913 watches=24 history=1024 journal=1024 symbols=454 drops=0
[91641732600] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=42000 nodes=1937 watches=24 history=1024 journal=1024 symbols=454 drops=0
[93519918228] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=43000 nodes=1961 watches=24 history=1024 journal=1024 symbols=454 drops=0
[95413381140] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=44000 nodes=1994 watches=24 history=1024 journal=1024 symbols=454 drops=0
[96136112259] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=5517 ops=1 watches=24
[96720747618] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=2
[97582629936] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=45000 nodes=2036 watches=24 history=1024 journal=1024 symbols=454 drops=0
[99524468175] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=46000 nodes=2060 watches=24 history=1024 journal=1024 symbols=454 drops=0
[101828671164] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=47000 nodes=2105 watches=24 history=1024 journal=1024 symbols=454 drops=0
[104092742457] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=48000 nodes=2141 watches=24 history=1024 journal=1024 symbols=454 drops=0
[106535082852] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=49000 nodes=2191 watches=24 history=1024 journal=1024 symbols=454 drops=0
[108469936806] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=50000 nodes=2215 watches=24 history=1024 journal=1024 symbols=454 drops=0
[110542483293] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=51000 nodes=2248 watches=24 history=1024 journal=1024 symbols=454 drops=0
[112528899714] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=52000 nodes=2276 watches=24 history=1024 journal=1024 symbols=454 drops=0
[114414037782] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=53000 nodes=2308 watches=24 history=1024 journal=1024 symbols=454 drops=0
[116193535491] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=54000 nodes=2341 watches=24 history=1024 journal=1024 symbols=454 drops=0
[118917985065] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=55000 nodes=2371 watches=24 history=1024 journal=1024 symbols=454 drops=0
[121434971163] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=56000 nodes=2405 watches=24 history=1024 journal=1024 symbols=454 drops=0
[123496350747] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=57000 nodes=2429 watches=24 history=1024 journal=1024 symbols=454 drops=0
[124329387270] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=6541 ops=1 watches=24
[127373330733] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=58000 nodes=2474 watches=24 history=1024 journal=1024 symbols=454 drops=0
[129838942563] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=59000 nodes=2502 watches=24 history=1024 journal=1024 symbols=454 drops=0
[133469432877] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=60000 nodes=2546 watches=24 history=1024 journal=1024 symbols=454 drops=0
[135847172340] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=61000 nodes=2581 watches=24 history=1024 journal=1024 symbols=454 drops=0
[138644181027] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=62000 nodes=2611 watches=24 history=1024 journal=1024 symbols=454 drops=0
[141344963034] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=63000 nodes=2639 watches=24 history=1024 journal=1024 symbols=454 drops=0
[143570838477] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=64000 nodes=2661 watches=24 history=1024 journal=1024 symbols=454 drops=0
[146031265017] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=65000 nodes=2700 watches=24 history=1024 journal=1024 symbols=454 drops=0
[148217471688] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=66000 nodes=2728 watches=24 history=1024 journal=1024 symbols=454 drops=0
[150491847495] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=67000 nodes=2762 watches=24 history=1024 journal=1024 symbols=454 drops=0
[152915149662] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=68000 nodes=2795 watches=24 history=1024 journal=1024 symbols=454 drops=0
[155333573340] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=69000 nodes=2827 watches=24 history=1024 journal=1024 symbols=454 drops=0
[157881590922] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=70000 nodes=2855 watches=24 history=1024 journal=1024 symbols=454 drops=0
[158022547320] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=7565 ops=1 watches=24
[160594516368] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=71000 nodes=2899 watches=24 history=1024 journal=1024 symbols=454 drops=0
[163160747739] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=72000 nodes=2932 watches=24 history=1024 journal=1024 symbols=454 drops=0
[165412505907] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=73000 nodes=2958 watches=24 history=1024 journal=1024 symbols=454 drops=0
[168097489989] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=74000 nodes=2992 watches=24 history=1024 journal=1024 symbols=454 drops=0
[170500903551] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=75000 nodes=3023 watches=24 history=1024 journal=1024 symbols=454 drops=0
[173883712992] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=76000 nodes=3067 watches=24 history=1024 journal=1024 symbols=454 drops=0
[177115810014] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=77000 nodes=3103 watches=24 history=1024 journal=1024 symbols=454 drops=0
[179794695267] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=78000 nodes=3133 watches=24 history=1024 journal=1024 symbols=454 drops=0
[183019075569] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=79000 nodes=3182 watches=24 history=1024 journal=1024 symbols=454 drops=0
[186440398782] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=80000 nodes=3216 watches=24 history=1024 journal=1024 symbols=454 drops=0
[189885705504] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=81000 nodes=3242 watches=24 history=1024 journal=1024 symbols=454 drops=0
[193759904514] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=82000 nodes=3274 watches=24 history=1024 journal=1024 symbols=454 drops=0
[194709608214] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=8589 ops=1 watches=24
[198453741057] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=83000 nodes=3323 watches=24 history=1024 journal=1024 symbols=454 drops=0
[201990024159] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=84000 nodes=3347 watches=24 history=1024 journal=1024 symbols=454 drops=0
[206432933619] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=85000 nodes=3395 watches=24 history=1024 journal=1024 symbols=454 drops=0
[216344492364] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=86000 nodes=3530 watches=24 history=1024 journal=1024 symbols=454 drops=0
[226022658939] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=87000 nodes=3656 watches=24 history=1024 journal=1024 symbols=454 drops=0
[236665763400] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=88000 nodes=3803 watches=24 history=1024 journal=1024 symbols=454 drops=0
[245852253537] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=89000 nodes=3925 watches=24 history=1024 journal=1024 symbols=454 drops=0
[246466747098] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=9613 ops=1 watches=24
[253680785028] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=90000 nodes=4037 watches=24 history=1024 journal=1024 symbols=454 drops=0
[262482900636] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=91000 nodes=4158 watches=24 history=1024 journal=1024 symbols=454 drops=0
[271680770064] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=92000 nodes=4272 watches=24 history=1024 journal=1024 symbols=454 drops=0
[281776346643] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=93000 nodes=4404 watches=24 history=1024 journal=1024 symbols=454 drops=0
[284797884195] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[292063280322] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=94000 nodes=4535 watches=24 history=1024 journal=1024 symbols=454 drops=0
[300594007857] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=10637 ops=1 watches=24
[303046304385] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=95000 nodes=4669 watches=24 history=1024 journal=1024 symbols=454 drops=0
[312629851875] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=96000 nodes=4779 watches=24 history=1024 journal=1024 symbols=454 drops=0
[322542327987] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=97000 nodes=4908 watches=24 history=1024 journal=1024 symbols=454 drops=0
[333522381621] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=98000 nodes=5036 watches=24 history=1024 journal=1024 symbols=454 drops=0
[340803484362] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[344785166847] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=99000 nodes=5158 watches=24 history=1024 journal=1024 symbols=454 drops=0
[358881686064] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=100000 nodes=5299 watches=24 history=1024 journal=1024 symbols=454 drops=0
[363292964661] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=11661 ops=1 watches=24
[370857461733] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=101000 nodes=5425 watches=24 history=1024 journal=1024 symbols=454 drops=0
[382129280346] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=102000 nodes=5539 watches=24 history=1024 journal=1024 symbols=454 drops=0
[394833992520] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=103000 nodes=5674 watches=24 history=1024 journal=1024 symbols=454 drops=0
[
```
</details>
