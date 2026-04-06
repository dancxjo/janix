# ✅ Scenario: Network Stack Initialization

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 5264ms | - - - |
| 2 | When I wait for the system to reach ready state | ✅ | 0ms | - - - |
| 3 | Then the serial output should contain "NETD: Starting network stack service..." | ✅ | 606ms | - [📜](./03/serial.log) - |
| 4 | And the serial output should contain "NETD: Network stack ready" | ✅ | 507ms | - [📜](./04/serial.log) - |
| 5 | And the serial output should contain "FETCHD: Got IP address" | ✅ | 1016ms | - [📜](./05/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11298536667] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11304019353] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11307672618] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11309706969] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11310889491] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11311517745] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11312169693] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11312787981] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11313417291] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11314026240] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11314617270] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11315219883] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11315928888] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11316666174] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11317360461] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11317971357] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11318592846] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11319185262] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11319831039] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11320458204] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11321039928] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11321624424] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11322218292] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11322851199] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11323489947] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11324092164] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11324692467] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11325339663] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11325971844] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11326576965] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11327189577] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11327807766] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11328465753] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11329082886] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11329713978] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11330412621] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11331116313] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11331833898] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11332572009] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11333301969] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11334012558] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11334728658] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11336108421] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11337606324] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11338380702] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11338931142] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11339484354] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11340011298] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11340684564] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11341436304] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11341983147] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11342565960] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11343091881] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11343637272] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11344192266] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11344764552] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11345320701] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11345919915] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11346474711] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11347088412] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11347698054] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11348272683] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11348825631] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11349435999] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11349990630] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11350654458] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11351209056] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11351781870] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11352525294] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11353102431] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11353655610] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11354227731] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11354852916] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11355425070] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11356012536] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11356586241] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11357138727] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11357710617] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11358261651] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11358847632] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11359398600] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11359972074] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11360524659] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11361095526] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11361645999] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11362243167] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11362795488] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11363368599] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11363919567] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11364492348] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11365045329] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11365644411] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11366196468] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11366768358] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11367320844] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11367890259] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11368441227] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11369064597] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11369617842] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11370187356] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11370738621] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11371307409] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11371858311] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11372478018] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11373244608] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11373961269] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11374518111] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11375494515] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11622399987] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[11633178546] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11638092807] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11639373174] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11640231108] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11644513221] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11646222027] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11647296771] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11647979838] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11648671518] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11649341319] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11650315677] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11651273304] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11651987589] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11652663132] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11653342602] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11654019003] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11655189876] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11656260660] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11656967652] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11658573663] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11659519311] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11660519541] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11662190925] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11663747535] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11664519504] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11665054434] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11665869666] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12041155203] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12042165069] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12045693990] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12046563573] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12047368113] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12048877995] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12061366878] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12062668992] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12063410139] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12065115711] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12065667636] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12068112837] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12075507015] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12077221497] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12090617517] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12091197558] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12107055807] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12107634528] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12109731843] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12111050061] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12112565124] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12115864596] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12117030453] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12152431104] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62421300 ticks/sec), init_cnt=624213 for 100Hz
[12154497663] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12156128391] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12158384403] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12164094096] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12194763669] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12195822738] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12198440298] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12199802274] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12200916750] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12203647203] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12204883449] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12223801194] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12224704404] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12225416181] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12226263687] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12228448848] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12230463333] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12231329781] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12256503633] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12257857524] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12259074729] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12260425914] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12261667737] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12262908273] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12263810592] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12265110957] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12271995021] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12273297267] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12275876118] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12276931029] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12282804699] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12284478195] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12285221157] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12286439121] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12287365299] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12288239469] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12300393897] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12303457551] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12304678980] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12305569452] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12327323217] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12345847206] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12348128628] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12351380877] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12352941150] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12355031469] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12357828912] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12360185079] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12360933684] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12361953351] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12368843817] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12370662612] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12373429629] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12375897369] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12380870535] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12381707679] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12395146368] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12396092082] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12403317894] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12404120784] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12433495008] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12434391222] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[12776380980] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13249028364] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13275080280] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[13312472382] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[14538651204] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=961 journal=769 symbols=95 drops=0
[15544014453] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15655999590] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15657092616] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[15769843848] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[15856938108] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[15903293637] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[15904723395] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[15905735802] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[15913975011] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[15949156905] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[15986378001] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[15990633384] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[16115637846] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[16142727216] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[16144854033] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[16154026020] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[16209651678] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[16233652182] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[16238938353] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[16240360059] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[16317241347] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[16319913291] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[16437000492] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[16512483735] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[16527675252] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[16531781673] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[16534249215] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[16550883393] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[16603372005] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[16609047840] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[16610166672] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[16611000681] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[16611891285] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[16612592832] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[16613241249] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[16613887224] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[16614489540] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[16615102152] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[16615761162] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[16616392452] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[16617031926] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[16617685161] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[16618372419] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[16619110860] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[16619760795] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[16620491382] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[16621154187] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[16621831677] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[16622484351] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[16623099933] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[16623718056] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[16624356771] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[16624984926] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[16625674098] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[16626408447] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[16627043994] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[16627733958] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[16628360133] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[16629016371] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[16629661257] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[16630312314] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[16630952019] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[16631594298] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[16632629805] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[16633382667] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[16634120052] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[16634859087] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[16635601059] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[16636364316] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[16637231061] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[16637993658] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[16639652700] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[16641395331] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[16642204788] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16650453402] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[16664732832] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[16669486713] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[16679256660] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[16679956920] [CONTRACT] [kernel] [CPU0] Spawning init process...
[16682076147] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[16684640082] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[16685578833] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [16691402112] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[16692345582] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[16714219170] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[16716388293] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[16717543062] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[16718729610] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[16727994624] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[16733403885] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[16736801598] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[16737875616] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[16741907820] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[16745166801] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[16746290682] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[16750623021] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[16751765646] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[16752802935] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[16753880220] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16757551965] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16758802434] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16759957731] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16761407190] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16762803519] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16764707685] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16768907562] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[16812435255] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[16820609652] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[16826974461] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[16833776916] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[16837408104] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[16841882574] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[16847173794] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[16852603812] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[16857857412] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[16863452826] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[16868951121] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[16876126806] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[16880909397] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[16885466598] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[16890047328] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[16894465929] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[16899868425] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[16907274681] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[16915987836] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[16923340137] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[16928891925] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[16933597032] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[16940203896] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[16945281969] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[16950253221] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[16955002185] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[16959645945] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[16964309274] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[16969179645] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[16973682264] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[16976767071] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[16979956752] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[16984372350] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[16988862429] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[16993330464] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[16998418998] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[17003103117] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[17007728727] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[17012489307] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[17016944604] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[17022366933] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[17025387918] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[17048271240] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[17199683358] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[17206999293] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[17207802909] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17209482312] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17213903718] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17215182963] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17223135996] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[17228573175] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[17229580071] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17231048307] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[17234865582] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[17236679724] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[17237371767] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17238929169] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17243168514] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[17244930120] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17251593150] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[17254152729] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[17256679572] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[17257639971] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[17259530277] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[17265840504] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[17267785161] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[17269979331] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17274268506] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:01:52 = 1775437312 unix_secs
[17275923786] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775437312, mono_ns=8637741387, offset=1775437303362258613ns
[17277639126] [INFO] [rtc_cmos] [CPU1] System clock anchored
[17288347725] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[17320080195] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[17328184401] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[17329050453] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17330781897] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17336653917] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[17339227059] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[17346261372] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[17348725977] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[17351369541] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17352890148] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[17357885292] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[17361946206] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[17363540568] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[17364326925] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17365975275] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17372911413] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17375033775] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17381361162] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[17383822632] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[17384622321] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17386138803] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[17386905063] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[17392353066] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[17393154603] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[17398680090] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[17400422358] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[17401153737] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[17401883598] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[17402615472] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[17415750231] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[17417252160] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[17991608250] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17996608905] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[17999446212] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[18001215408] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[18005524911] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[18007260381] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[18009587310] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[18011155833] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[18012248199] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[18014793060] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[18015878298] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[18016945155] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[18017797743] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[18018687060] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[18019609344] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[18020663100] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[18025417509] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[18026433249] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18028294713] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18035408589] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18037921539] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18045300207] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[18048560607] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[18049683861] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[18051468501] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352560 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[18058059723] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[18070479603] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[18096994443] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[18108320604] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[18110228268] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[18115284627] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[18124414737] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[18126030087] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[18131420373] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[18132843465] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[18134435286] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[18135725058] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[18136801155] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[18137715948] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[18138671265] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[18139946715] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[18140950641] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[18147351816] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[18149848266] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[18152273667] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[18154318017] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[18155087511] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18156674646] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18160309761] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18161994543] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18169170822] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[18171283317] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[18173081487] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[18173898699] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18175320273] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18180149328] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18181526583] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18188468859] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[18191117373] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
[18191925015] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[18194632665] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500368 RFLAGS_BEFORE=130 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[18200368263] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[18201718326] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[18202467657] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[18203318661] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18204990639] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18207187053] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[18208719870] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[18210321525] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18211281759] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[18213032970] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18213904401] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[18215637000] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[18217133847] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[18218897961] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[18220340787] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[18221827206] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[18222956136] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[18225485421] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[18227106117] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[18228569799] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[18230504193] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[18232085817] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434288 RFLAGS_BEFORE=130 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[18236939721] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[18682493214] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
[18683604456] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[18685309533] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583152 RFLAGS_BEFORE=134 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[18688348305] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[18690295767] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[18691063941] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[18692562702] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[18695414727] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[18696999189] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18700779009] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[18701868075] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18704747787] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18712440714] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[18713254494] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[18715050651] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[18722206503] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[18725179572] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[18727796010] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[18729423009] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[18730210851] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18731739477] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18751328112] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18755586894] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[18775739202] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[18778256178] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[18780178725] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18781938648] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369719072 RFLAGS_BEFORE=130 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[18785253927] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[18786778065] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653536 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[18789865050] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[18791308140] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[18792113208] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[18793598670] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[18797943219] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[18799695816] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[18804069306] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[18805257933] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[18806120421] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[18807170316] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[18808898295] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[18827008431] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[18831930150] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[18834083037] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[18835712808] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[18836598957] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18838337496] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18902887113] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[18926620119] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[18933904539] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[18937316343] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[18938559684] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18940202061] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369784608 RFLAGS_BEFORE=130 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[18944276736] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[18945496251] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18947269935] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[18948494136] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[18949353357] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[18950301678] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18951376455] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18952285044] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[18953136642] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[18956675727] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18958169406] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18979216245] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18982479252] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[18989445651] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[18992263950] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[18994206132] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[18994959423] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18996483462] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19001726568] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19004036040] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19010849847] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[19013285412] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[19014530238] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19016096979] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915680 RFLAGS_BEFORE=130 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[19019880660] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19020930060] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19022588244] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[19023959295] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19024964607] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[19035226650] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19040344257] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19050493935] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[19054354836] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[19055644113] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19057309689] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981824 RFLAGS_BEFORE=130 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[19060713177] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[19061767758] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19064037003] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19074524535] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19078509054] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[19085639892] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[19088350743] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[19090623024] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[19091481948] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19093247547] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19118300223] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[19123111458] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[19130091585] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[19133140389] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cac8
[19134587637] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19135877673] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370114016 RFLAGS_BEFORE=130 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[19138726167] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[19139447613] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19140869055] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[19141858857] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19158768684] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[19161866922] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19163532300] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[19167305388] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19168744287] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19170058215] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19171169952] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[19172184834] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[19175187240] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[19176352866] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19177862418] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370198112 RFLAGS_BEFORE=134 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[19181833044] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[19182751005] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19184434170] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19186297251] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[19188295566] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[19196976480] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19198134120] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19199352612] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19200624399] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[19204390062] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19206169422] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19208036166] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19209988974] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[19222740306] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[19232705481] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[19240277991] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[19241275449] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19242535455] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19244190834] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19245645573] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[19248638574] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[19251549438] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19253013615] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19254417204] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19255939659] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[19257714102] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19259497389] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369850144 RFLAGS_BEFORE=130 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[19265183256] [INFO] [nectar] [CPU2] NECTAR: Started.
[19267631196] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19269464643] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[19270689042] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370048032 RFLAGS_BEFORE=130 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[19274698707] [INFO] [fontd] [CPU3] FONTD: Service ready
[19275624489] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[19277444571] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19279052826] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370267120 RFLAGS_BEFORE=134 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[19284432585] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[19285861155] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19287512739] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[19288455384] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19290144885] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[19291205835] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19293145509] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[19300627269] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[19303203546] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[19305423159] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[19307315445] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[19308100515] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19309270563] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[19311327948] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19312240596] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[19314286860] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x44b6000
[19316233266] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[19318499409] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[19320588540] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[19322209566] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[19323829800] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19324894677] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19325980641] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19327077330] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=236 subj_lo=0
[19332017067] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[19332843387] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19333838139] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19334835795] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19335903081] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=237 subj_lo=0
[19340675310] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[19341801006] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19342747710] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19343779026] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19344852384] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=238 subj_lo=0
[19349872245] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[19350852345] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19351823535] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19352868018] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19354051068] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[19354861779] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=239 pred=0 subj_lo=0
[19356244908] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[19357751589] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[19359146961] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[19360694760] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[19361974368] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[19363535070] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[19364593413] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[19369573377] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=19, read=20)
[19374613929] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=21, read=22)
[19379025237] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[19384455090] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([230, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19395877479] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[19405098900] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[19406873112] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1256 backend=VirtIO-GPU
[19408563933] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[19409345637] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19410874263] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19421531811] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=23, resp=26
[19422649818] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[19474354284] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([230, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19504278717] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([230, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19510437969] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19511762259] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19537415073] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([230, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19540614093] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[19555186893] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[19563419832] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[19566358251] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[19569075438] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[19569918852] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19571512356] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19576296234] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[19577175189] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f0fe8
[19578570330] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e8
[19580335005] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370504768 RFLAGS_BEFORE=130 CR3_BEFORE=72167424 fs_base=0 gs_base=18446744071564586640
[19585614873] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[19587661005] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[19588590879] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[19590414426] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[19591445940] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f0fe8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[19593915330] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370574400 RFLAGS_BEFORE=130 CR3_BEFORE=73216000 fs_base=0 gs_base=18446744071564586576
[19601587995] [INFO] [echo] [CPU1] echo: starting up
[19603612380] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[19619515443] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[19622868045] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([230, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19636223706] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=19, RX port=22
[19638293895] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[19640348343] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[19651226892] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[19652802213] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[19654614507] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[19655491383] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19657070862] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19661964531] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19664031420] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[19665520116] [INFO] [netd] [CPU3] NETD: Driver TX port=19, RX port=22, link_up=true, mtu=1500
[19669432233] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[19670538954] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19672639602] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19675217958] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[19676361936] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[19679146410] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[19681359951] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[19683675231] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[19684628337] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[19685894085] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19686645231] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19688173329] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19689207912] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[19695684261] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19699300896] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19700834901] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[19707097806] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[19709958081] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[19711203963] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[19713215280] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[19714037013] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19716200229] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19718102811] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f35c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19719920715] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[19720738884] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370738240 RFLAGS_BEFORE=130 CR3_BEFORE=73981952 fs_base=0 gs_base=18446744071564586640
[19723886457] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[19731872028] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[19734493119] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[19735807608] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[19746349590] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[19747328766] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[19748742024] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[19750111194] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f2a88
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19751928075] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[19753797228] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[19755555105] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370803776 RFLAGS_BEFORE=130 CR3_BEFORE=74129408 fs_base=0 gs_base=18446744071564586576
[19762902786] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[19816947546] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[19839732891] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[19865447844] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[19873915578] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[19875628344] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[19883608602] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[19909925607] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[19912398693] [INFO] [anther] [CPU1] anther: Connected to network stack
[19923798906] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[19925810157] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[19929973998] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[19942995732] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[19947097863] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[19959173124] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[19968693393] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[19972731834] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19980537753] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[19988041392] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[20012810037] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[20015602299] [INFO] [bloom] [CPU3] bloom: creating surface...
[20016662127] [INFO] [bloom] [CPU3] bloom: surface created!
[20017618467] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[20022158607] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[20026848534] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1264
[20028204999] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[20043587553] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[20046655134] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[20048188677] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[20049258999] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [20054237841] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[20055517416] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[20067584460] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[20083047864] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[20093935752] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[20098207371] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[20108449779] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[20123729373] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
T:07D0 [20130055308] [INFO] T:0640 [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
T:F0B0 [20148341070] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1264
[20149542798] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[20172910593] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[20177134923] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[20181419511] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[20189102439] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[20192852295] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[20198936934] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[20203711605] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
T:1220 [20215146666] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[20218173855] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[20226839589] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[20234096883] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[20237216076] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[20245464624] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[20248538871] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[20266542945] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=278
[20268358935] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[20270369262] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20272033881] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20273884950] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20275938474] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=278 subj_lo=0
[20297176284] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1271)
[20299595745] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[20309325993] [INFO] [netd] [CPU3] NETD: DHCP configured ��� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[20313806535] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[20320314597] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[20324314890] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[20327654721] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[20330888358] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[20336689857] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[20342125287] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=239
[20344338003] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[20345833596] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20347374168] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20349267477] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20351265495] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=239 pred=0 subj_lo=0
[20372100969] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1274)
[20374144428] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[20387149299] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=283
[20389167018] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[20390896482] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20392526484] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20394444774] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20396407977] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=283 subj_lo=0
[20402467206] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[20406188682] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[20414637705] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[20426820909] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1277)
[20428997259] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[20600082558] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[20951651754] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[20984900442] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[20995348968] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[21243308823] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[21310556355] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=664 watches=13 history=1024 journal=1024 symbols=313 drops=0
[21466783173] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[21639647733] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[21827563494] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[22058334243] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[22200450261] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[22253170995] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[22418918313] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[22588175709] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[22785678696] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[22791237315] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[22792688226] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[22793668062] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[22794734622] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[22796323407] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[22800665052] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[22808611221] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[22809704412] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[22810922211] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[22812333720] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=337 pred=0 subj_lo=0
[22975917888] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[23087378424] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=712 watches=15 history=1024 journal=1024 symbols=338 drops=0
[23137619406] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[23296156224] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[23298377718] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[23319203061] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[23338918845] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23418041955] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23472944715] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[23528150580] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23529251658] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23530345245] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23531426655] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=340 pred=0 subj_lo=0
[23553736536] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23585296713] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[23606709357] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[23618736306] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[23620012548] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[23629525392] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23714926554] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[23746483101] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23753703798] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[23809665495] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23810888079] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23812005657] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23813254179] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=341 pred=0 subj_lo=0
[23835727443] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23875852209] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=fec1d29ef0678173)
[23898733485] [DEBUG] [blo
```
</details>
