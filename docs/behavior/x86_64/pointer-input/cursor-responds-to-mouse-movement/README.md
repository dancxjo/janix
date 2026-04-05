# ✅ Scenario: Cursor responds to mouse movement

> Last run: 2026-04-05 12:26:57

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given a cursor is visible on the screen | ✅ | 6579ms | - [📜](./01/serial.log) - |
| 2 | When I move the mouse | ✅ | 3ms | - [📜](./02/serial.log) - |
| 3 | Then the serial log should contain 'CONTRACT: input pointer_move' | ✅ | 0ms | - [📜](./03/serial.log) - |
| 4 | And the cursor should move correspondingly on the screen | ✅ | 0ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12375468831] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[12381370650] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[12385013025] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[12387136542] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[12388680909] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=33264
[12389353251] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[12390028992] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[12390635796] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[12391238871] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=20976
[12391861944] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[12392485413] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=961032
[12393421557] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[12394185540] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[12394868508] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[12395573487] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[12396226392] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[12396882828] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[12397500357] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[12398132439] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[12398745480] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[12399355584] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[12399957537] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=578080
[12400567278] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[12401172498] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[12402021324] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[12403010103] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[12403820220] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[12404497974] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[12405108639] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[12405749433] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[12406376532] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[12407007822] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[12407703396] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[12408336105] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[12408970200] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[12409689369] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[12410414181] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[12411165954] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[12411890205] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[12413053224] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[12414061803] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[12414819384] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[12416220465] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[12417772422] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[12418565643] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[12419149908] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[12419679030] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[12420218316] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[12420788985] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[12421331571] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[12421937418] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[12422804460] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[12423535410] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[12424200558] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x77989000 (Usable)
[12424771887] [INFO] [kernel::memory] [CPU0]   [11] 0x77989000 - 0x779ed000 (Reserved)
[12425375589] [INFO] [kernel::memory] [CPU0]   [12] 0x779ed000 - 0x779ee000 (Other)
[12425939790] [INFO] [kernel::memory] [CPU0]   [13] 0x779ee000 - 0x779ef000 (Reserved)
[12426523461] [INFO] [kernel::memory] [CPU0]   [14] 0x779ef000 - 0x779f0000 (Other)
[12427089213] [INFO] [kernel::memory] [CPU0]   [15] 0x779f0000 - 0x779f1000 (Reserved)
[12427671762] [INFO] [kernel::memory] [CPU0]   [16] 0x779f1000 - 0x779f2000 (Other)
[12428236326] [INFO] [kernel::memory] [CPU0]   [17] 0x779f2000 - 0x779f3000 (Reserved)
[12428833956] [INFO] [kernel::memory] [CPU0]   [18] 0x779f3000 - 0x77a7e000 (Other)
[12429397959] [INFO] [kernel::memory] [CPU0]   [19] 0x77a7e000 - 0x77a7f000 (Reserved)
[12429979023] [INFO] [kernel::memory] [CPU0]   [20] 0x77a7f000 - 0x77f00000 (Other)
[12430543719] [INFO] [kernel::memory] [CPU0]   [21] 0x77f00000 - 0x77f01000 (Reserved)
[12431127060] [INFO] [kernel::memory] [CPU0]   [22] 0x77f01000 - 0x78022000 (Other)
[12431795574] [INFO] [kernel::memory] [CPU0]   [23] 0x78022000 - 0x78023000 (Reserved)
[12432720333] [INFO] [kernel::memory] [CPU0]   [24] 0x78023000 - 0x78823000 (Other)
[12433454715] [INFO] [kernel::memory] [CPU0]   [25] 0x78823000 - 0x78824000 (Reserved)
[12434115078] [INFO] [kernel::memory] [CPU0]   [26] 0x78824000 - 0x788e5000 (Other)
[12434684460] [INFO] [kernel::memory] [CPU0]   [27] 0x788e5000 - 0x788e6000 (Reserved)
[12435289152] [INFO] [kernel::memory] [CPU0]   [28] 0x788e6000 - 0x788f5000 (Other)
[12435855927] [INFO] [kernel::memory] [CPU0]   [29] 0x788f5000 - 0x788f6000 (Reserved)
[12436441512] [INFO] [kernel::memory] [CPU0]   [30] 0x788f6000 - 0x788fb000 (Other)
[12437007198] [INFO] [kernel::memory] [CPU0]   [31] 0x788fb000 - 0x788fc000 (Reserved)
[12437592552] [INFO] [kernel::memory] [CPU0]   [32] 0x788fc000 - 0x78901000 (Other)
[12438155796] [INFO] [kernel::memory] [CPU0]   [33] 0x78901000 - 0x78902000 (Reserved)
[12438754548] [INFO] [kernel::memory] [CPU0]   [34] 0x78902000 - 0x7892e000 (Other)
[12439319739] [INFO] [kernel::memory] [CPU0]   [35] 0x7892e000 - 0x78930000 (Reserved)
[12439902585] [INFO] [kernel::memory] [CPU0]   [36] 0x78930000 - 0x78939000 (Other)
[12440468040] [INFO] [kernel::memory] [CPU0]   [37] 0x78939000 - 0x7893b000 (Reserved)
[12441052602] [INFO] [kernel::memory] [CPU0]   [38] 0x7893b000 - 0x78943000 (Other)
[12441855558] [INFO] [kernel::memory] [CPU0]   [39] 0x78943000 - 0x78944000 (Reserved)
[12442761441] [INFO] [kernel::memory] [CPU0]   [40] 0x78944000 - 0x7894e000 (Other)
[12443436555] [INFO] [kernel::memory] [CPU0]   [41] 0x7894e000 - 0x7894f000 (Reserved)
[12444032469] [INFO] [kernel::memory] [CPU0]   [42] 0x7894f000 - 0x7895c000 (Other)
[12444600432] [INFO] [kernel::memory] [CPU0]   [43] 0x7895c000 - 0x7895e000 (Reserved)
[12445203078] [INFO] [kernel::memory] [CPU0]   [44] 0x7895e000 - 0x7896c000 (Other)
[12445771635] [INFO] [kernel::memory] [CPU0]   [45] 0x7896c000 - 0x7896d000 (Reserved)
[12446363358] [INFO] [kernel::memory] [CPU0]   [46] 0x7896d000 - 0x78979000 (Other)
[12446928021] [INFO] [kernel::memory] [CPU0]   [47] 0x78979000 - 0x7897a000 (Reserved)
[12447512352] [INFO] [kernel::memory] [CPU0]   [48] 0x7897a000 - 0x7897e000 (Other)
[12448080315] [INFO] [kernel::memory] [CPU0]   [49] 0x7897e000 - 0x7897f000 (Reserved)
[12448682796] [INFO] [kernel::memory] [CPU0]   [50] 0x7897f000 - 0x7898f000 (Other)
[12449247063] [INFO] [kernel::memory] [CPU0]   [51] 0x7898f000 - 0x78990000 (Reserved)
[12449831295] [INFO] [kernel::memory] [CPU0]   [52] 0x78990000 - 0x78a1e000 (Other)
[12450397179] [INFO] [kernel::memory] [CPU0]   [53] 0x78a1e000 - 0x78a1f000 (Reserved)
[12450984315] [INFO] [kernel::memory] [CPU0]   [54] 0x78a1f000 - 0x78a2a000 (Other)
[12451646757] [INFO] [kernel::memory] [CPU0]   [55] 0x78a2a000 - 0x78a2b000 (Reserved)
[12452568051] [INFO] [kernel::memory] [CPU0]   [56] 0x78a2b000 - 0x78a50000 (Other)
[12453243528] [INFO] [kernel::memory] [CPU0]   [57] 0x78a50000 - 0x78a51000 (Reserved)
[12453935340] [INFO] [kernel::memory] [CPU0]   [58] 0x78a51000 - 0x78a5d000 (Other)
[12454505349] [INFO] [kernel::memory] [CPU0]   [59] 0x78a5d000 - 0x78a5e000 (Reserved)
[12455109513] [INFO] [kernel::memory] [CPU0]   [60] 0x78a5e000 - 0x78a6b000 (Other)
[12455681271] [INFO] [kernel::memory] [CPU0]   [61] 0x78a6b000 - 0x78a6c000 (Reserved)
[12456270585] [INFO] [kernel::memory] [CPU0]   [62] 0x78a6c000 - 0x78ab4000 (Other)
[12456842508] [INFO] [kernel::memory] [CPU0]   [63] 0x78ab4000 - 0x78ab5000 (Reserved)
[12457735290] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12687054600] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485793 free frames
[12698246880] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12703171635] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12704435304] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12705324027] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12709706625] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12711417444] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12712554426] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12713260725] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12713947818] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12714635241] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12715634580] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12716644908] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12717354738] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12718052787] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12718763079] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12719457729] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12720507393] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12721560621] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12722325825] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12724072878] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12725024730] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12726067431] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12727605264] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12729106335] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12729894606] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12730480983] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12731252358] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[13107950526] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[13108972602] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[13112367114] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[13113317481] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[13114144065] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[13115671767] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[13128866718] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[13130241135] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[13131105075] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[13132893543] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[13133492559] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[13135932876] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[13143574818] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[13145390280] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[13158740628] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[13159354824] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[13175917128] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[13176590955] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[13178715462] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[13180042128] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[13181192112] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[13183637577] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[13184507919] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[13219522998] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62334400 ticks/sec), init_cnt=623344 for 100Hz
[13221090333] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[13221933120] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[13223142471] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[13228877079] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[13259291925] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[13260278328] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[13262174211] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[13263452928] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[13264556679] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[13267499553] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[13269061080] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[13289027829] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[13290614271] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[13291808937] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[13293085971] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[13294076499] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[13296137349] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[13297122927] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[13322552430] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[13324968954] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[13325940573] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[13327568430] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[13328684424] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[13329897867] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[13330793025] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[13332013266] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[13338788298] [INFO] [kernel::root] [CPU0] Spawning Root service...
[13339816116] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[13341772191] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[13342632270] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[13348019553] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[13350481716] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[13351824189] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[13353970905] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[13355663574] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[13357210482] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[13373893929] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[13377329394] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[13378757271] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[13379780832] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13402116618] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13421759175] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13424448312] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13428894204] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13430639640] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13432925979] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13435297161] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13437530172] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13438398468] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13439482584] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13446102450] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13448054004] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13450855110] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13453540386] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13456336608] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13458984495] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13460010564] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13472707578] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13473923925] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13481335560] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13482940779] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13512992031] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13513880490] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13886182431] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[14423693757] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[14455342935] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=499 journal=424 symbols=51 drops=0
[14493573633] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[16033084815] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=961 journal=769 symbols=95 drops=0
[16955929155] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[17061360558] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[17062527141] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[17176573491] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[17357579283] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[17386107618] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[17387028879] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[17387731218] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[17392209912] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[17412821052] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[17433679692] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[17436068001] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[17519283045] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[17544362550] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[17545191213] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[17549553087] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[17596229607] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[17626438500] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[17629836741] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[17631248613] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[17710044429] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[17712085248] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[17816795436] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[17888967591] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[17901711102] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[17905871577] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[17908292391] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[17913506523] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[17982066366] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[17988150972] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[17989342404] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[17990177964] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[17991092097] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[17991780840] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=33264
[17992465920] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[17993443083] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[17994070710] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[17994701802] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=20976
[17995380315] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[17996020713] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=961032
[17996673948] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[17997345432] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[17998113705] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[17998882374] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[17999542572] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[18000214386] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[18000943653] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[18001661997] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[18002341764] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[18002967906] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[18003600054] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=578080
[18004238406] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[18004934442] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[18005680275] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[18006332256] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[18006977010] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[18007733667] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[18008473296] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[18009141711] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[18009799401] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[18010464450] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[18011115375] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[18011813886] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[18012471015] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[18013222656] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[18013978290] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[18014745210] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[18015589482] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[18016472661] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[18017304492] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[18018084348] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[18019778667] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[18021961650] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[18022937526] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18031703184] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[18046124811] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[18050747715] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[18061460340] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[18062227326] [CONTRACT] [kernel] [CPU0] Spawning init process...
[18064374999] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[18067370310] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[18068116869] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [18074872761] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[18075672780] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564572728
[18093107142] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[18098327016] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[18099805152] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[18101163300] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[18112238199] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[18120186546] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[18124476645] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[18125732064] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[18130550757] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[18134866794] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[18136674336] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[18143299812] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[18144576219] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[18145928757] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[18147251430] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[18151297164] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[18156271320] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[18157637454] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[18159160074] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[18160641180] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[18162912174] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[18169980048] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[18214905753] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[18223703289] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[18230878941] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[18237822306] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[18242246154] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[18248110584] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[18255671445] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[18263277582] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[18269288829] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[18275399472] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[18282049863] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[18288593103] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[18295235145] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[18300790629] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[18306691293] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[18312642843] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[18318398604] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[18324629895] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[18329813007] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[18336263583] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[18341115870] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[18347570901] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[18353887794] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[18359863203] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[18365323845] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[18371066472] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[18376592520] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[18383017389] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[18388131234] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[18393452550] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[18397038066] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[18401841777] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[18407041521] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[18412444941] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[18417705735] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[18424228680] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[18429499869] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[18435815640] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[18441540612] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[18448184964] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[18455049360] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[18458777733] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[18483831333] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[18638645025] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[18646557765] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[18647624193] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18649622310] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18654496773] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18655920558] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18665600184] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[18671332152] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18673198632] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18674844837] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564572760
[18681192618] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[18682067778] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[18683129784] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18685301877] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18690209043] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18692582040] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18702715713] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[18707686536] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[18709447845] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[18711695673] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564572696
[18716026032] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[18719363553] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[18721364904] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[18723770274] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18729398721] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-05 19:27:15 = 1775417235 unix_secs
[18731377764] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775417235, mono_ns=9365456199, offset=1775417225634543801ns
[18733322817] [INFO] [rtc_cmos] [CPU1] System clock anchored
[18745513809] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[18792782382] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[18806319972] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[18807271725] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18809533974] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18816082725] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18818467536] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[18826833696] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[18830462442] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[18834067395] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18835823292] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369210752 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564572728
[18841983996] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[18846953499] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[18848949207] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[18849881688] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18851791365] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18859424892] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18861915699] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18869894703] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[18873412602] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[18875879187] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[18877162293] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18879610365] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369277440 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564572760
[18884038800] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[18886548912] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[18894693213] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[18895612230] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[18896455809] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[18897411522] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[18899076768] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[18923672163] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[18925892271] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[19427525733] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19442100579] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[19445179380] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[19447134366] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[19452918177] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[19454824092] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[19457063934] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[19458695883] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[19459846263] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[19463442999] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[19464633771] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[19466021091] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[19467076464] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[19468045641] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[19469026203] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[19470115929] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[19481237061] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[19482656721] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19484855907] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19493148213] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19495887312] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19504144374] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[19507502355] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[19508690388] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[19510593069] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369357232 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564572696
[19518505875] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[19527729837] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[19543652337] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[19552478715] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[19554278238] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[19560433068] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[19568255289] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[19569762168] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[19578265839] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[19593406338] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[19595091780] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[19596762240] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[19598333766] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[19599894270] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[19601462661] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[19603180740] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[19604895750] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[20076417009] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[20089905429] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[20097872784] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[20100533838] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[20107614351] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20118778944] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20125069965] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[20127326241] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[20136471102] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[20139711306] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[20141692329] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[20142479709] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20144183697] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20148939855] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20150357205] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20160055674] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[20174358633] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[20175921876] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[20177973057] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369503744 RFLAGS_BEFORE=134 CR3_BEFORE=68620288 fs_base=0 gs_base=18446744071564572760
[20184134388] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[20185624008] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[20186794254] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[20188082904] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20190775440] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20193304362] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[20200649865] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20203540566] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[20214913389] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[20218579458] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[20219710467] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[20222246814] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369585664 RFLAGS_BEFORE=134 CR3_BEFORE=68890624 fs_base=0 gs_base=18446744071564572696
[20226541038] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[20228246874] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[20230180872] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[20231690655] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[20232907497] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[20234221986] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[20236140078] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[20237203800] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369438208 RFLAGS_BEFORE=134 CR3_BEFORE=68399104 fs_base=0 gs_base=18446744071564572728
[20240894652] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[20241909930] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[20243627613] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[20245760007] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[20249294109] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[20252399805] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[20255814711] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[20258248659] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1241
[20259125106] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1240)
[20260395474] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[20261571066] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[20262944295] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[20264041149] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[20269047711] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20272031373] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[20273107569] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20275329855] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20282725188] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[20285461812] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[20293189917] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[20296594461] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[20313141024] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[20314929426] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[20315903586] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20318298561] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20340379125] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20344974342] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[20365712763] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[20369277357] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
[20370280293] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20372311641] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369717024 RFLAGS_BEFORE=130 CR3_BEFORE=69144576 fs_base=0 gs_base=18446744071564572760
[20378457957] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[20380235403] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369651200 RFLAGS_BEFORE=134 CR3_BEFORE=69009408 fs_base=0 gs_base=18446744071564572728
[20383751751] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[20385546126] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[20387502432] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[20390116824] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[20397441306] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[20400226374] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[20405236665] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[20407553199] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[20409455583] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[20411404596] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[20414344170] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[20416565730] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[20418411684] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[20419292421] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20421260574] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20485211736] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=266000 exec=false
[20509204782] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28c000 exec=false
[20517164679] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[20521486359] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[20523040527] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20525075670] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369783088 RFLAGS_BEFORE=134 CR3_BEFORE=69541888 fs_base=0 gs_base=18446744071564572696
[20529081474] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[20530513740] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20533188621] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20544308565] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[20547864579] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[20553894009] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20556078708] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20559615384] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20562985674] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[20570579700] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[20573955798] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[20576601111] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[20577389151] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20579050437] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20584887345] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20587677330] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20595062070] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[20598334416] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[20599977816] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20601952800] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369915408 RFLAGS_BEFORE=130 CR3_BEFORE=70467584 fs_base=0 gs_base=18446744071564572760
[20606972265] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[20607732354] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20609408457] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20610365688] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[20613011760] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[20617883649] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20621601066] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[20628762891] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[20631436980] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[20633827170] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20635814100] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369981808 RFLAGS_BEFORE=130 CR3_BEFORE=70594560 fs_base=0 gs_base=18446744071564572696
[20640763836] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[20641581048] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20643141123] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20652751053] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[20656231959] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[20664025767] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[20667230892] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[20669738991] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[20670516273] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20672090703] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20698630755] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20703702591] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[20711293845] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[20714566785] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010d510
[20716159134] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20717970834] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370114064 RFLAGS_BEFORE=130 CR3_BEFORE=70897664 fs_base=0 gs_base=18446744071564572760
[20723030823] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[20724405636] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20726733951] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[20728304124] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20746897215] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[20751159825] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20754547341] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[20760112791] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[20763489681] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[20764754703] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20766799713] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f10c8
[20768330088] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20770361271] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[20771653815] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20773036416] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370198304 RFLAGS_BEFORE=130 CR3_BEFORE=71172096 fs_base=0 gs_base=18446744071564572696
[20777908536] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[20779720797] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20782489695] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[20783553186] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20786945487] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[20800340913] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20801970618] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20803607550] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20805528084] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[20810189367] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20811730071] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20813494284] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20815199658] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[20825082927] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[20835863037] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[20844413271] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[20848509330] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[20854309806] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20857042833] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072369849328 RFLAGS_BEFORE=134 CR3_BEFORE=70205440 fs_base=0 gs_base=18446744071564572728
[20863904292] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20865705234] [INFO] [nectar] [CPU2] NECTAR: Started.
[20866608444] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20868471327] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20870510958] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[20872766475] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20874944937] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370047984 RFLAGS_BEFORE=134 CR3_BEFORE=70742016 fs_base=0 gs_base=18446744071564572728
[20880350238] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[20881293873] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20882812038] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20884322943] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20885906712] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[20888171238] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20889895389] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562103005 RSP_BEFORE=18446744072370266864 RFLAGS_BEFORE=134 CR3_BEFORE=71380992 fs_base=0 gs_base=18446744071564572728
[20895580662] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[20897808558] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[20899029921] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[20900316162] [INFO] [fontd] [CPU3] FONTD: Service ready
[20901087306] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[20916659214] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[20936058264] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[20940414528] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20942131485] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[20943424392] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20945402379] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20947399506] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=234 subj_lo=0
[20957651880] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[20958847635] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[20960637192] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[20962103316] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[20964256104] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1251 backend=VirtIO-GPU
[20966740344] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[20968003980] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20970562800] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20976776403] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20978668425] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20983097652] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20984381418] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20985552819] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20987246280] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[20994697614] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20995707513] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20996795754] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20997926334] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[21003365790] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21004361466] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21005529996] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21006660840] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[21018118539] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21019673565] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21021644556] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21023473812] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[21066393051] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[21110051226] [INF
```
</details>
