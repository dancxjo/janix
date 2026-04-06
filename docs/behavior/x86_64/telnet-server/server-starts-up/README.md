# ✅ Scenario: Server starts up

> Last run: 2026-04-05 19:06:17

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 3950ms | - [📜](./01/serial.log) - |
| 2 | Then I should see a message in the serial output that says "telnetd: listening on guest port 2323" | ✅ | 2834ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12220558779] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[12226046118] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[12229704399] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[12231763170] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[12232943052] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[12233586816] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[12234249126] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[12234825702] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[12235423068] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=29168
[12236036142] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=33264
[12236626479] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[12237250113] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[12237954762] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[12238915194] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[12239958555] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[12240777879] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[12241427946] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[12242030988] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[12242649606] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[12243248622] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[12243845658] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[12244433817] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[12245030391] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[12245624523] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[12246284358] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[12246908982] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[12247510638] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[12248158857] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[12248752230] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[12249358539] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[12249968511] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[12250602441] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[12251266038] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[12251884491] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=172560
[12252485520] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[12253185186] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[12253917819] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[12254644776] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[12255347742] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[12256078032] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[12256829541] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[12257548974] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[12258881118] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[12260383740] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[12261153465] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[12261700044] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[12262211346] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[12262733439] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[12263306781] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[12263828379] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[12264340968] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[12264859695] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[12265371327] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[12265907313] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7795e000 (Usable)
[12266447721] [INFO] [kernel::memory] [CPU0]   [11] 0x7795e000 - 0x779c2000 (Reserved)
[12267029313] [INFO] [kernel::memory] [CPU0]   [12] 0x779c2000 - 0x779c3000 (Other)
[12267575001] [INFO] [kernel::memory] [CPU0]   [13] 0x779c3000 - 0x779c4000 (Reserved)
[12268138806] [INFO] [kernel::memory] [CPU0]   [14] 0x779c4000 - 0x779c5000 (Other)
[12268684593] [INFO] [kernel::memory] [CPU0]   [15] 0x779c5000 - 0x779c6000 (Reserved)
[12269248827] [INFO] [kernel::memory] [CPU0]   [16] 0x779c6000 - 0x779c7000 (Other)
[12269809167] [INFO] [kernel::memory] [CPU0]   [17] 0x779c7000 - 0x779c8000 (Reserved)
[12270373764] [INFO] [kernel::memory] [CPU0]   [18] 0x779c8000 - 0x77a53000 (Other)
[12270920277] [INFO] [kernel::memory] [CPU0]   [19] 0x77a53000 - 0x77a54000 (Reserved)
[12271483950] [INFO] [kernel::memory] [CPU0]   [20] 0x77a54000 - 0x77ed5000 (Other)
[12272029077] [INFO] [kernel::memory] [CPU0]   [21] 0x77ed5000 - 0x77ed6000 (Reserved)
[12272593311] [INFO] [kernel::memory] [CPU0]   [22] 0x77ed6000 - 0x77ff7000 (Other)
[12273151902] [INFO] [kernel::memory] [CPU0]   [23] 0x77ff7000 - 0x77ff8000 (Reserved)
[12273716070] [INFO] [kernel::memory] [CPU0]   [24] 0x77ff8000 - 0x787f8000 (Other)
[12274262154] [INFO] [kernel::memory] [CPU0]   [25] 0x787f8000 - 0x787f9000 (Reserved)
[12274824771] [INFO] [kernel::memory] [CPU0]   [26] 0x787f9000 - 0x788ba000 (Other)
[12275367621] [INFO] [kernel::memory] [CPU0]   [27] 0x788ba000 - 0x788bb000 (Reserved)
[12275934132] [INFO] [kernel::memory] [CPU0]   [28] 0x788bb000 - 0x788e6000 (Other)
[12276493845] [INFO] [kernel::memory] [CPU0]   [29] 0x788e6000 - 0x788e7000 (Reserved)
[12277056759] [INFO] [kernel::memory] [CPU0]   [30] 0x788e7000 - 0x788ec000 (Other)
[12277601787] [INFO] [kernel::memory] [CPU0]   [31] 0x788ec000 - 0x788ed000 (Reserved)
[12278163909] [INFO] [kernel::memory] [CPU0]   [32] 0x788ed000 - 0x788f2000 (Other)
[12278708970] [INFO] [kernel::memory] [CPU0]   [33] 0x788f2000 - 0x788f3000 (Reserved)
[12279272346] [INFO] [kernel::memory] [CPU0]   [34] 0x788f3000 - 0x7891f000 (Other)
[12279834435] [INFO] [kernel::memory] [CPU0]   [35] 0x7891f000 - 0x78921000 (Reserved)
[12280399296] [INFO] [kernel::memory] [CPU0]   [36] 0x78921000 - 0x7892a000 (Other)
[12280943169] [INFO] [kernel::memory] [CPU0]   [37] 0x7892a000 - 0x7892c000 (Reserved)
[12281506710] [INFO] [kernel::memory] [CPU0]   [38] 0x7892c000 - 0x78934000 (Other)
[12282052596] [INFO] [kernel::memory] [CPU0]   [39] 0x78934000 - 0x78935000 (Reserved)
[12282613662] [INFO] [kernel::memory] [CPU0]   [40] 0x78935000 - 0x7893f000 (Other)
[12283171824] [INFO] [kernel::memory] [CPU0]   [41] 0x7893f000 - 0x78940000 (Reserved)
[12283736883] [INFO] [kernel::memory] [CPU0]   [42] 0x78940000 - 0x7894d000 (Other)
[12284280822] [INFO] [kernel::memory] [CPU0]   [43] 0x7894d000 - 0x7894f000 (Reserved)
[12284842878] [INFO] [kernel::memory] [CPU0]   [44] 0x7894f000 - 0x7895d000 (Other)
[12285387213] [INFO] [kernel::memory] [CPU0]   [45] 0x7895d000 - 0x7895e000 (Reserved)
[12285951348] [INFO] [kernel::memory] [CPU0]   [46] 0x7895e000 - 0x7896a000 (Other)
[12286511820] [INFO] [kernel::memory] [CPU0]   [47] 0x7896a000 - 0x7896b000 (Reserved)
[12287076681] [INFO] [kernel::memory] [CPU0]   [48] 0x7896b000 - 0x7896f000 (Other)
[12287623293] [INFO] [kernel::memory] [CPU0]   [49] 0x7896f000 - 0x78970000 (Reserved)
[12288203862] [INFO] [kernel::memory] [CPU0]   [50] 0x78970000 - 0x78980000 (Other)
[12288774894] [INFO] [kernel::memory] [CPU0]   [51] 0x78980000 - 0x78981000 (Reserved)
[12289343022] [INFO] [kernel::memory] [CPU0]   [52] 0x78981000 - 0x78a11000 (Other)
[12289912668] [INFO] [kernel::memory] [CPU0]   [53] 0x78a11000 - 0x78a12000 (Reserved)
[12290476242] [INFO] [kernel::memory] [CPU0]   [54] 0x78a12000 - 0x78a1d000 (Other)
[12291022458] [INFO] [kernel::memory] [CPU0]   [55] 0x78a1d000 - 0x78a1e000 (Reserved)
[12291584712] [INFO] [kernel::memory] [CPU0]   [56] 0x78a1e000 - 0x78a43000 (Other)
[12292128816] [INFO] [kernel::memory] [CPU0]   [57] 0x78a43000 - 0x78a44000 (Reserved)
[12292792743] [INFO] [kernel::memory] [CPU0]   [58] 0x78a44000 - 0x78a50000 (Other)
[12293392221] [INFO] [kernel::memory] [CPU0]   [59] 0x78a50000 - 0x78a51000 (Reserved)
[12293960052] [INFO] [kernel::memory] [CPU0]   [60] 0x78a51000 - 0x78a5e000 (Other)
[12294505377] [INFO] [kernel::memory] [CPU0]   [61] 0x78a5e000 - 0x78a5f000 (Reserved)
[12295067895] [INFO] [kernel::memory] [CPU0]   [62] 0x78a5f000 - 0x78aa7000 (Other)
[12295613319] [INFO] [kernel::memory] [CPU0]   [63] 0x78aa7000 - 0x78aa8000 (Reserved)
[12296428188] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12536621691] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485750 free frames
[12548036853] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12553000779] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12554333187] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12555272301] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12559584345] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12561348393] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12562447128] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12563148774] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12563820621] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12564514314] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12565566750] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12566623773] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12567315420] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12567990732] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12568659477] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12569327892] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12570637926] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12571735539] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12572435898] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12574088769] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12575084643] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12576200142] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12577846809] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12579469419] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12580333788] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12580874691] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12581702760] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12965283144] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12966741645] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12970633071] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12972017454] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12973277394] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12975229509] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12989310477] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12991360602] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12992593284] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12995215266] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12995813061] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12998593773] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[13007440281] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[13009217991] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[13022563521] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[13023127722] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[13039834236] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[13040403585] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[13042501164] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[13043730414] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[13044842448] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[13047018138] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[13047841257] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[13082840496] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62391900 ticks/sec), init_cnt=623919 for 100Hz
[13084331271] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[13085216232] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[13086354039] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[13092247773] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[13122851973] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[13123892100] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[13125775476] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[13127064918] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[13128185202] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[13131148734] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[13132420752] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[13152633714] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[13153841943] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[13154855472] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[13155899427] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[13157320440] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[13159038816] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[13160072937] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[13185433800] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[13186802739] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[13187828643] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[13189386375] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[13190263647] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[13191651429] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[13192542957] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[13193759535] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[13200743094] [INFO] [kernel::root] [CPU0] Spawning Root service...
[13201822293] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[13203841200] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[13204678212] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[13209673653] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[13212254451] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[13213866204] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[13216395423] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[13218075420] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[13219863393] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[13236754806] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[13240475886] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[13241637057] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[13242409983] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13292149893] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13294836819] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13298745834] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13300519353] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13304237628] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13307013621] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13309412358] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13310188056] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13311347478] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13318371396] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13320795840] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13324263876] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13327312449] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13330522425] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13333756128] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13334551692] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13337636598] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13348412088] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13349481222] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13357029015] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13357817088] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13383900519] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13384732878] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13710633948] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[14194473051] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[14238564021] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[14289483153] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[15773152065] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=449 watches=0 history=964 journal=773 symbols=98 drops=0
[16547562702] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[16676347611] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[16677940554] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[16787573121] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[16855108479] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[16884637176] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[16885573815] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[16886376804] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[16890196884] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[16910813733] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[16930507539] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[16935259473] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[16992227307] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[17016083436] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[17016880914] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[17021173191] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[17050542498] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[17072963061] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[17076850758] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[17077823367] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[17151926814] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[17153639712] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[17251634631] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[17324024124] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[17336556105] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[17346111585] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[17348826825] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[17387391681] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[17421373134] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[17426441835] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[17427596571] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[17428667124] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[17429556903] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[17430221853] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[17430867399] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[17431630590] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[17432239176] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[17432853768] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=29168
[17433679395] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=33264
[17434443378] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[17435084205] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[17435741334] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[17436433047] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[17437158981] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[17437825515] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[17438493006] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[17439126243] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[17439784890] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[17440420767] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[17441050275] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[17441672292] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[17442295893] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[17442921804] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[17443593849] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[17444229132] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[17444874282] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[17445559593] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[17446184085] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[17446826001] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[17447469600] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[17448133164] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[17448768975] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[17449412145] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=172560
[17450044524] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[17450785143] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[17451548169] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[17452305585] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[17453045742] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[17453818041] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[17454584532] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[17455345776] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[17456874435] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[17458662870] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[17459461899] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17467560792] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[17482947834] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[17487273012] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[17496553833] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[17497274025] [CONTRACT] [kernel] [CPU0] Spawning init process...
[17499309960] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[17502216600] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[17502932865] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [17508084792] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[17508820197] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013440 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[17526426753] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[17531823375] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[17533606662] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[17534743941] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[17543310411] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[17548744752] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[17552460486] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[17553621459] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[17557572549] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[17562009663] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[17563131828] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[17567413281] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[17568489939] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[17569730343] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[17570868183] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[17574161121] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[17575331829] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[17576599260] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[17578084491] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[17579620938] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[17581393665] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[17585320830] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[17632584915] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[17639500758] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[17645147916] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[17650339674] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[17653854240] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[17657766918] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[17663045136] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[17668224123] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[17673582960] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[17678576058] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[17684565294] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[17690610267] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[17696532315] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[17701786740] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[17706962856] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[17712169431] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[17718127152] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[17724053688] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[17729751105] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[17734868349] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[17740272297] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[17745691788] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[17751222225] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[17757026265] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[17763481527] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[17769008367] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[17774160261] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[17779349610] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[17784474708] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[17790150411] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[17794285707] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[17798057970] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[17803530723] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[17807606949] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/telnetd
[17811264207] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[17816805402] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[17822230767] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[17827631217] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[17833263096] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[17839218441] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[17845087392] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[17850953637] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[17855098107] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[17875509762] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[18067651734] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[18074799336] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[18075595197] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18077288163] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18082135434] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18083439330] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18092212149] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[18097371567] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18098457498] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18099758457] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078976 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[18104770827] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[18107082609] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[18107751750] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18109461777] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18114441147] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18116172294] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18125074275] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[18128391336] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[18129523698] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[18130836603] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144512 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[18133869996] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[18139982652] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[18141598662] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[18143652549] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18147958224] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 02:06:35 = 1775441195 unix_secs
[18149637561] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775441195, mono_ns=9074584860, offset=1775441185925415140ns
[18151601952] [INFO] [rtc_cmos] [CPU1] System clock anchored
[18166959393] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[18217286109] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[18225158490] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[18226781001] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18229204224] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18235244115] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18237644007] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[18245345118] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[18248182854] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[18251893011] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18253539447] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210944 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[18258756714] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[18264715458] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[18267105813] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[18267986121] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18269907744] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18277759236] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18280309443] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18287509317] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[18291409323] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18292355961] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18293643126] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277696 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[18296931774] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[18298923852] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[18301526199] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[18304958958] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[18306731190] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[18308517018] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[18309912357] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[18311503353] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[18325443114] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[18327591942] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[18834108414] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18839585886] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[18842470053] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[18844249578] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[18848756454] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[18850459122] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[18852550662] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[18854134068] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[18855166176] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[18857505942] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[18858541218] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[18859534782] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[18860343348] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[18861162276] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[18861914808] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[18863063406] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[18867626646] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[18868654431] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18870614763] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18878139654] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18880918386] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18888385956] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[18891426147] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[18893159835] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[18895398819] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352880 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[18903411714] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[18915496578] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[18944821236] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[18957061068] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[18959664999] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[18966482898] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[18976752927] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[18978624951] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[18979821630] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[18981094770] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[18981879609] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[18982684776] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[18983835981] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[18985169181] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[18986722788] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[18988059519] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[18995311665] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[18998641002] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[19002498900] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[19005816291] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[19010363559] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[19011524400] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19014358077] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19020912702] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[19024223889] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19032075711] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[19035441612] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[19037517114] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[19038225327] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19040596740] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19045867302] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19047452259] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19054921743] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[19058356086] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0105668
[19059578010] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[19061310345] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500512 RFLAGS_BEFORE=134 CR3_BEFORE=68505600 fs_base=0 gs_base=18446744071564586640
[19065221538] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[19066239753] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19068267537] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19069868169] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[19071868299] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[19073955945] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19076656335] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19077474966] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[19079857401] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[19082086254] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[19084964118] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[19086580260] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[19087682658] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[19090071462] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[19091987937] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[19093180920] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[19094614968] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[19095626187] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[19096489038] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[19097946219] [INFO]744071564586608
[19104313239] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[19482529770] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0105668
[19483646820] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[19485293421] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583296 RFLAGS_BEFORE=134 CR3_BEFORE=68620288 fs_base=0 gs_base=18446744071564586576
[19488311436] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[19490799207] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[19491517749] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[19492946781] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[19495721421] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[19496697693] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19498880115] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[19500074286] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19502132397] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19510893435] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[19512136182] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[19514247060] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[19524325953] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[19529401848] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[19533315285] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[19535341947] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[19536523743] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19538937594] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19560897543] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19565100588] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[19601237568] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[19683784428] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[19684791984] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[19686108882] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[19687497324] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[20449169400] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[20452288923] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb01095c8
[20453819496] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20455607667] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369719216 RFLAGS_BEFORE=130 CR3_BEFORE=68882432 fs_base=0 gs_base=18446744071564586640
[20461838529] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[20462799258] [INFO] [netd] [CPU3] NETD: Starting network stack service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[20464138530] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653680 RFLAGS_BEFORE=130 CR3_BEFORE=68747264 fs_base=0 gs_base=18446744071564586608
[20467220202] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[20469256170] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[20470405692] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[20474065062] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[20475428523] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[20476509504] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[20477986386] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[20480762346] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering interrupt-driven loop
[20491050096] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[20497381509] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[20498970393] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[20500527399] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[20501805324] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[20503742160] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[20504996028] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[20506677114] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4c33000
[20508025560] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[20510164257] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[20512246986] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[20513606916] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[20523918492] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[20533379394] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[20543343513] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[20545925961] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[20548192038] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[20549852433] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[20551687002] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[20553218334] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[20554809693] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[20556118539] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[20557357260] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[20562458928] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=15, read=16)
[20568513900] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=17, read=18)
[20576548377] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20596706724] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=15, RX port=18
[20652402804] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[20654060295] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[20655737850] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[20656596444] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20658190080] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20658914496] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20672338467] [INFO] [netd] [CPU3] NETD: Driver TX port=15, RX port=18, link_up=true, mtu=1500
[20674782612] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[20678979156] [INFO] [netd] [CPU3] NETD: Created socket API port (write=19, read=20)
[20701658802] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([219, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[20714147289] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[20715514644] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[20722012476] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[20726119953] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[20750261169] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[20751555495] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[20759281851] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[20762067546] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[20763347715] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20764674084] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369953088 RFLAGS_BEFORE=130 CR3_BEFORE=80011264 fs_base=0 gs_base=18446744071564586576
[20767608246] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[20768385528] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20769996918] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20770727076] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[20773294179] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[20783121645] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=19, our_write=21, our_read=22)
[20785760127] [INFO] [anther] [CPU1] anther: Connected to network stack
[20796739689] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20800044045] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[20807832870] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[20810675160] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[20812844382] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[20813584770] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20815519527] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20821533546] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20823981486] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20831918679] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[20835175812] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
[20836653552] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20839159572] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370100544 RFLAGS_BEFORE=130 CR3_BEFORE=81379328 fs_base=0 gs_base=18446744071564586640
[20842563225] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[20843412348] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20844756900] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[20845491711] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20846223288] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[20853538002] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20857438635] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[20866208814] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[20869629858] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
[20870729847] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20872441326] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370166400 RFLAGS_BEFORE=130 CR3_BEFORE=81604608 fs_base=0 gs_base=18446744071564586576
[20875651236] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[20876689053] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20878447128] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20889115698] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[20892427974] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[20900838288] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[20904182772] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[20906980545] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[20907910551] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20909599953] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20937906330] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20943258600] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[20951873118] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[20955173415] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010ac38
[20956505460] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20958145362] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370298400 RFLAGS_BEFORE=130 CR3_BEFORE=81907712 fs_base=0 gs_base=18446744071564586640
[20963573829] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[20964671541] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20967398430] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20968556730] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[20992751241] [INFO] [fontd] [CPU3] FONTD: Service node created, req=23, resp=26
[20993991579] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[20998728960] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[21001576035] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21003805614] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21005935137] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21009080829] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[21011655390] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[21014926218] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f25e8
[21016428543] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21018370032] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370382384 RFLAGS_BEFORE=134 CR3_BEFORE=82182144 fs_base=0 gs_base=18446744071564586576
[21023034912] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[21024578157] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21026387712] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21065051139] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[21067925274] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21069080472] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21070508910] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21071911014] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=236 pred=0 subj_lo=0
[21076203060] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[21082514178] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[21084633900] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[21085649574] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[21088471305] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[21090243801] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21091401870] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21092647026] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21093819978] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21095064375] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21096311346] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[21097165518] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21099164856] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
[21100245903] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=237 pred=0 subj_lo=0
[21101662098] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=238 subj_lo=0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21103878312] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370035008 RFLAGS_BEFORE=130 CR3_BEFORE=80949248 fs_base=0 gs_base=18446744071564586608
[21109242231] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1249) for kind 'Asset'
[21110970705] [INFO] [nectar] [CPU2] NECTAR: Started.
[21113078349] [INFO] [fontd] [CPU3] FONTD: Service ready
[21114634497] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1190
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21116463159] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370232320 RFLAGS_BEFORE=134 CR3_BEFORE=81752064 fs_base=0 gs_base=18446744071564586608
[21121779690] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[21123571590] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f25e8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21125306235] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370449568 RFLAGS_BEFORE=134 CR3_BEFORE=82391040 fs_base=0 gs_base=18446744071564586608
[21130716915] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[21132343386] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[21133976424] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[21137699847] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[21146138310] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21147328257] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21148608987] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21150053496] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[21159849777] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21160974186] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21162163935] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21163510269] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[21167090571] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[21169405785] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[21172128021] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21173410797] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21174615891] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21176098152] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[21177648393] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[21179377032] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[21187057551] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[21191512716] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[21203809242] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[21206846331] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[21208396077] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[21210447786] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21211718319] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[21212972913] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21214351125] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21215888397] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=247 subj_lo=0
[21226619073] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1256 backend=VirtIO-GPU
[21229207428] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[21230488488] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[21231327315] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21233064237] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[21233828418] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21235085256] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[21239850555] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21240851016] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21242080761] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21243449238] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=250 subj_lo=0
[21255477837] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21256539282] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21257676561] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21258965805] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=251 pred=0 subj_lo=0
[21264300453] [INFO] [netd] [CPU3] NETD: DHCP configured �� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[21268113966] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[21273219891] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[21280692114] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[21285253539] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[21307003311] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[21322357056] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=27, resp=30
[21323475195] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[21394737639] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[21416239350] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[21440213157] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[21443337960] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db4c8
[21444674922] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e8
[21446325087] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370556832 RFLAGS_BEFORE=134 CR3_BEFORE=83308544 fs_base=0 gs_base=18446744071564586640
[21450316668] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[21451130250] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21453457146] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21454583007] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[21460534293] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21463347345] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[21474924240] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[21480923673] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db4c8
[21482949708] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[21484993926] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370622368 RFLAGS_BEFORE=134 CR3_BEFORE=84357120 fs_base=0 gs_base=18446744071564586576
[21488239047] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[21490422723] [INFO] [echo] [CPU1] echo: starting up
[21493643655] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[21495111297] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[21504584574] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[21506645853] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[21537097296] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[21539476662] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[21540376803] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[21542578761] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[21543418380] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[21544455207] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21547011255] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21551812062] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[21554901357] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21557729721] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[21564746412] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[21566833431] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[21569608863] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[21571399542] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[21574511475] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[21576518931] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[21578266941] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[21580230573] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[21581187474] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21582769692] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21590931978] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21595153965] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[21604244079] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[21605374593] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[21608480619] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[21609861009] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[21611490780] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[21612705279] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21614549748] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21617296404] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f8128
[21618423684] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21619916208] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[21620710551] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370757536 RFLAGS_BEFORE=134 CR3_BEFORE=85131264 fs_base=0 gs_base=18446744071564586640
[21626686719] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[21628355265] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[21630945501] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[21632261409] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/telnetd'
[21634466931] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/telnetd
[21635594112] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21637154748] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f8128
[21638390070] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21639895101] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370823072 RFLAGS_BEFORE=134 CR3_BEFORE=85278720 fs_base=0 gs_base=18446744071564586576
[21645980103] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[21668497488] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[21671765313] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[21673451283] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[21676533945] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=221000 exec=false
[21683357322] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=229000 exec=false
[21687482025] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[21691157367] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 32 (user task/process) assigned to CPU 2
[21693746217] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=32)
[21695145681] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[21703852236] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[21706129467] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f8128
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21708119994] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=32 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370888608 RFLAGS_BEFORE=134 CR3_BEFORE=85381120 fs_base=0 gs_base=18446744071564586608
[21715135101] [INFO] [telnetd] [CPU2] telnetd: starting on port 2323
[21728201682] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[21729161718] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[21734668494]
```
</details>
