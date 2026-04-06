# ❌ Scenario: Clock tick should be cheap

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 3946ms | - - - |
| 2 | Given the clock window is ticking | ❌ | 152946ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12348927261] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[12354413214] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[12358065357] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[12360194550] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[12361560981] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[12362274837] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[12363095118] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[12363900978] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[12364769868] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[12365701095] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[12366579093] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[12367471215] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[12368560281] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[12369551469] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[12370589121] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[12371505201] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[12372430554] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[12373321686] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[12374234169] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[12375140283] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[12375986139] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[12376854138] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[12377734380] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[12378630792] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[12379569510] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[12380471829] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[12381377613] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[12382340487] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[12383209542] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[12384088827] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[12385015731] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[12385931580] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[12386885643] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[12387796905] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[12388700214] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[12389724996] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[12390809277] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[12391870458] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[12393042057] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[12394117989] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[12395198970] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[12396237513] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[12398279157] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[12400446432] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[12401619813] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[12402432405] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[12403199457] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[12403974891] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[12404806656] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[12405579120] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[12406351980] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[12407128470] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[12407927301] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[12408742698] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[12409559118] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[12410402301] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[12411228423] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[12412053489] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[12412855059] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[12413681016] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[12414510999] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[12415356954] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[12416170701] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[12417011145] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[12417851325] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[12418689756] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[12419475420] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[12420295503] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[12421109745] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[12421936032] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[12422733543] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[12423558246] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[12424399350] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[12425240553] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[12426060075] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[12426906426] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[12427740006] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[12428587974] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[12429385056] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[12430232430] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[12431046540] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[12431894607] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[12432725250] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[12433571667] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[12434400759] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[12435242457] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[12436049142] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[12436893018] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[12437716698] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[12438562785] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[12439371483] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[12440199750] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[12441014982] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[12441837969] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[12442631190] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[12443466981] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[12444291750] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[12445137672] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[12445942047] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[12446779455] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[12447601782] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[12448450212] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[12449268909] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[12450107142] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[12450942438] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[12451787997] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[12452590524] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[12453810369] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12757972062] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[12769199289] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12776174796] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12777831066] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12778728699] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12782973522] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12784706121] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12785819145] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12786543594] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12787232535] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12787919166] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12789394695] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12790965000] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12792095151] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12793239063] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12794372646] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12795495009] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12797154711] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12798547212] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12799707987] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12801792630] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12803130351] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12804572583] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12806719035] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12808642473] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12809810376] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12810703290] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12811802454] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[13197805566] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[13199279412] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[13203044613] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[13204472325] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[13205754705] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[13207717446] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[13221689976] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[13223517285] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[13224718683] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[13226805240] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[13227700332] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[13230719139] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[13239104076] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[13241225349] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[13255674102] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[13256600049] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[13273241091] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[13274192613] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[13276671111] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[13278327480] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[13279766841] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[13284367140] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[13285615959] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[13321087725] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62413600 ticks/sec), init_cnt=624136 for 100Hz
[13323194940] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[13324527744] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[13326113427] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[13332337656] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[13363120122] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[13364445303] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[13365907401] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[13368135627] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[13369410879] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[13372383486] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[13373846112] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[13397788536] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[13399974621] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[13401209778] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[13402714974] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[13403863539] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[13406101632] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[13407316659] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[13433602875] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[13434937395] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[13436167701] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[13437635541] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[13438665075] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[13439463015] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[13440794631] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[13441464003] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[13447428918] [INFO] [kernel::root] [CPU0] Spawning Root service...
[13448538378] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[13450431324] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[13451288862] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[13456799202] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[13458336639] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[13459060065] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[13460422272] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[13461350001] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[13462248195] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[13474994841] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[13478431098] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[13479635862] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[13480421757] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13503163410] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13522946910] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13525210314] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13528622910] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13530669966] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13533136782] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13535628612] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13537728765] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13538924454] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13540410147] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13547850723] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13549697502] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13552377366] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13554858207] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13557121215] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13559767419] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13560732174] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13572924189] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13573974777] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13581220851] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13581978003] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13610879733] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13611677145] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13968441696] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[14473592463] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[14503678662] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[14541423369] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[15976333131] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=962 journal=770 symbols=96 drops=0
[17048374860] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[17211930153] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[17213396607] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[17371307613] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[17463696096] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[17496120675] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[17496980523] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[17497681146] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[17502794925] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[17531081040] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[17561910234] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[17564207958] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[17686373331] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[17713110888] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[17714313771] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[17720570010] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[17764094337] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[17797196637] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[17801152347] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[17802147462] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[17912382741] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[17914186620] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[18061717245] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[18129564057] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[18141918498] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[18147276906] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[18149576841] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[18183902748] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[18222249441] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[18226826409] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[18227833767] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[18228677412] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[18229575936] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[18230256429] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[18230955006] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[18231646785] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[18232262994] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[18232890951] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[18234058095] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[18234826929] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[18235500822] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[18236328462] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[18237448053] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[18238418715] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[18239093895] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[18239811348] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[18240463725] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[18241176162] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[18241825602] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[18242488176] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[18243122139] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[18243757653] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[18244408875] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[18245090853] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[18245738742] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[18246382110] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[18247077948] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[18247728114] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[18248421015] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[18249082896] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[18249750222] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[18250404645] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[18251080023] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[18251718705] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[18252475824] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[18253241127] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[18254009466] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[18254773086] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[18255674976] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[18256447803] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[18257222181] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[18258689790] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[18260386452] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[18261203532] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18269398884] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[18284718837] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[18289232478] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[18298545771] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[18299523891] [CONTRACT] [kernel] [CPU0] Spawning init process...
[18301834848] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[18305050467] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[18305939685] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [18313378545] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[18318403224] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[18337574211] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[18345076563] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[18347020494] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[18348750024] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[18360644445] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[18367722648] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[18372423201] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[18373542330] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[18377408742] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[18380793948] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[18382013364] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[18386534661] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[18387633297] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[18388719954] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[18389813904] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[18393916992] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[18395052225] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[18396241116] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[18397754331] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[18399525672] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[18400872633] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[18405027300] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[18451882878] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[18460351899] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[18467448879] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[18473459829] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[18476943903] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[18481090353] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[18487163673] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[18492979857] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[18499336779] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[18505492335] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[18512052306] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[18518343789] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[18524640024] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[18530767662] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[18537366837] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[18544377555] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[18551671908] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[18558599862] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[18564799605] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[18571312518] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[18578405076] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[18584325078] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[18590188023] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[18596511252] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[18602426601] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[18608510217] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[18614447313] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[18620146974] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[18625513698] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[18631816830] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[18635630970] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[18639604401] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[18645345279] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[18650995902] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[18656744469] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[18662890191] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[18668734821] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[18674605554] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[18680682273] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[18686923959] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[18693567981] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[18697831878] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[18721558944] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[18906402339] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[18917356227] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[18918528750] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18921210693] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18928229628] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18930195339] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18943219383] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[18952026159] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18953519541] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18955516437] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[18962768418] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[18967583844] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[18968779632] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18971475435] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18977825328] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18980447013] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18991129608] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[18997363077] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[18998652882] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[19001071815] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[19005746892] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[19010393985] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[19013012040] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[19016458527] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19023383874] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:57:40 = 1775437060 unix_secs
[19026203724] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775437060, mono_ns=9512738433, offset=1775437050487261567ns
[19028812374] [INFO] [rtc_cmos] [CPU1] System clock anchored
[19043946999] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[19100834313] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[19117618872] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[19119174426] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19122389517] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19131728715] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[19135394619] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[19147544823] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[19153304346] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[19158765681] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19161321696] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[19169936874] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[19177258749] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[19179940395] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[19181268546] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19184017479] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19194757032] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19198447719] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19210147110] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[19215304284] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[19216591317] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19218930654] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[19223791983] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[19226766009] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[19234217211] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[19240218888] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[19241310396] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[19242568785] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[19243636170] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[19244954586] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[19264155042] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[19266539028] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[19815110931] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[19816065654] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19817683776] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19824783561] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19827140553] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19834659636] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[19837088469] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[19838348277] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[19840289667] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352656 RFLAGS_BEFORE=130 CR3_BEFORE=59662336 fs_base=0 gs_base=18446744071564586576
[19844629893] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19847245506] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[19855127787] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[19856825439] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 0)
[19858271202] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 1)
[19860027759] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=1
[19864513878] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[19866387816] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[19868696265] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[19870322142] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[19871164335] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[19872553668] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[19873296300] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[19875766944] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[19876847958] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[19878893694] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[19879712028] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[19880826009] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[19881895737] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[19882840263] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[19884253158] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[19888691130] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[19890384459] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[19910949894] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[19915128651] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[19918492506] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[19920959784] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[19922027103] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19924480389] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19928422536] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[19929935619] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[19932556116] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19950658596] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[19954682253] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[19957530747] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[19958600574] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19960811112] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19967661318] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19969609803] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19979969031] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[19984113171] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0105370
[19985581506] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[19987594869] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369503696 RFLAGS_BEFORE=130 CR3_BEFORE=68235264 fs_base=0 gs_base=18446744071564586640
[19992139332] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[19993190745] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19995500811] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20003447871] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20007085956] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20009385594] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[20011191189] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[20017025424] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[20019647769] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[20021101518] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[20023894869] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[20026368615] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[20027447814] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[20029896282] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[20031065868] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[20033360127] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[20034501795] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[20037197136] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[20039465523] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[20041212180] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[20042603955] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[20044658502] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369438160 RFLAGS_BEFORE=130 CR3_BEFORE=59830272 fs_base=0 gs_base=18446744071564586608
[20052311235] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[20387450754] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1237 port=2 model='                                        ' rpc_port=5
[20389617204] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0105370
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[20392457448] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369585616 RFLAGS_BEFORE=130 CR3_BEFORE=68349952 fs_base=0 gs_base=18446744071564586576
[20418806364] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[20422578165] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[20427293370] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[20428294194] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[20429680128] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[20430834864] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[20432111040] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[20433875220] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[20435341641] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[20436701769] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[20438854854] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[20447417067] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[20450771286] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[20453864640] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[20456568726] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[20463167472] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20467428168] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[20468569242] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20471350383] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20481483891] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[20484929124] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[20496549447] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[20502161790] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[20505505845] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[20507954115] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[20509233327] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20511735816] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20541125781] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20548022979] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[20573769744] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[20578377336] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[20583009810] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[20584506261] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20586600573] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716688 RFLAGS_BEFORE=130 CR3_BEFORE=68890624 fs_base=0 gs_base=18446744071564586640
[20592507474] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0105370
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[20594966766] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369651152 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[20599751700] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[20601905016] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[20603580195] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[20606079582] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[20611636683] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[20614627572] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[20620565955] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[20622918855] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[20625260799] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[20627803515] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[20637504294] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[20640175050] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[20642772678] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[20643942462] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20646767130] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20692326270] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[20693620728] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[20695271982] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[20696764473] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[20746795278] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[20783432307] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[20794708242] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[20799845781] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0105370
[20801262108] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20803541055] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369783200 RFLAGS_BEFORE=134 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[20808405816] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[20809678527] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20812212762] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20813430363] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[20817951924] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[20825558028] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20828038902] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20849615655] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20854829358] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[20866140009] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[20871581643] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[20875067367] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[20876457822] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20879268399] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20888009175] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20891890635] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20903125980] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[20908176366] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[20909582925] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20911616187] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915072 RFLAGS_BEFORE=134 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[20915929089] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[20916999708] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20918716038] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[20920073724] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20921302743] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[20928930924] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20933223630] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[20940895701] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[20944075911] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[20945372250] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20947175205] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981632 RFLAGS_BEFORE=134 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[20951203977] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[20952499458] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20954860707] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20964399852] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[20968919763] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[20976496563] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[20979943248] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[20982664395] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[20983826919] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20986122135] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21011307867] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[21016511934] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[21023859153] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[21027087510] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
[21028171857] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010dfc8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21030538749] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[21031504428] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21032887689] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113872 RFLAGS_BEFORE=134 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[21037391331] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21040281504] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[21054510411] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[21057663627] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[21065013651] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[21066413247] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[21068711004] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[21070210392] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21071908077] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370198016 RFLAGS_BEFORE=134 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[21076714362] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[21077865666] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21080123064] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21102110238] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21104451192] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21106656186] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21109108086] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[21113322912] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[21115647366] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[21125420085] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21126736983] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21128122587] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21129578943] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[21133438788] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21134861913] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21136407897] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21138090963] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[21139432281] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[21153637296] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[21165179541] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[21169690542] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[21175128315] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0105370
[21176663112] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21178798608] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21180282849] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849280 RFLAGS_BEFORE=134 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[21185075076] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21186731973] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21187904298] [INFO] [nectar] [CPU2] NECTAR: Started.
[21188765994] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21190142193] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[21191997750] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21193444404] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[21195735990] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21198183303] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047888 RFLAGS_BEFORE=134 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[21206149008] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[21208996677] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
[21211041753] [INFO] [fontd] [CPU3] FONTD: Service ready
[21211917507] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f10c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21214344261] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370266640 RFLAGS_BEFORE=130 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[21222585813] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[21224906439] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[21228767274] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[21238586622] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21240765579] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21241819368] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[21259624122] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[21262903167] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21264141063] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21265335366] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21266880426] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=234 subj_lo=0
[21275039346] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[21285969210] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21287322903] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21288741738] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21290230269] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=240 subj_lo=0
[21299319393] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21300527061] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21301791588] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21303304242] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[21306742380] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1250 backend=VirtIO-GPU
[21309750990] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21311030433] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[21312175995] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21313565031] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21315628653] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21317039139] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21318259743] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[21324318510] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21325469484] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21326648046] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21328091598] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[21350157972] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[21363954150] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[21365198976] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[21504695124] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[21520537038] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[21528969396] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[21533506005] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[21536591967] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[21537585597] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21539857482] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21542975586] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21544867146] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21546230805] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[21547409961] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f0fe8
[21548858760] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e2
[21550909875] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370356480 RFLAGS_BEFORE=134 CR3_BEFORE=72052736 fs_base=0 gs_base=18446744071564586640
[21559368171] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[21563600454] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[21566707635] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[21569690505] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f0fe8
[21571052217] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[21572875797] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370431056 RFLAGS_BEFORE=134 CR3_BEFORE=73101312 fs_base=0 gs_base=18446744071564586576
[21578597040] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[21579697491] [INFO] [echo] [CPU1] echo: starting up
[21580711218] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f4000
[21582892881] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[21584143416] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f4000
[21586126023] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f7000
[21589187070] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276783104)
[21591060711] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[21593784795] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f8000 phys=0x45fb000
[21595693416] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[21598921476] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[21602087727] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[21604130295] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[21619214166] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[21620602047] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[21633405948] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[21643816128] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[21646522656] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[21647975778] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[21650583636] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[21653631021] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[21656493903] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[21658780407] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[21661018797] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[21663137364] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[21665296554] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[21667421292] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[21674661624] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[21675733035] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[21677063265] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[21683514435] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[21686390121] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[21717784407] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[21719288745] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[21721052067] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[21721914555] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21723795060] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21728706087] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21730687506] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[21738890910] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21741776364] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[21746023695] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[21749476518] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[21753155985] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[21755730447] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[21756853800] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21759307812] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21770613579] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21772587837] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[21775224471] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[21776518698] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[21777715278] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[21789465852] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[21793403313] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f1190
[21803086305] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21804253152] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21807905922] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[21809455041] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370705488 RFLAGS_BEFORE=134 CR3_BEFORE=73981952 fs_base=0 gs_base=18446744071564586640
[21814430418] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[21815657127] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21818142786] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21819517896] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21823876239] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[21825764433] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[21835194381] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[21837542892] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[21841855068] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[21843892224] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[21847086855] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1190
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21849514863] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370771024 RFLAGS_BEFORE=134 CR3_BEFORE=74129408 fs_base=0 gs_base=18446744071564586576
[21854021673] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21855971247] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21858424137] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[21866283054] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[21902514843] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[21904423035] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[21909753294] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[21913368048] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[21916397844] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[21923307252] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[21934925133] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21970614105] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[21977883543] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[21979123155] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21980917101] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21982149783] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[21986792421] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[21992901843] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[22020616134] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[22022945835] [INFO] [bloom] [CPU3] bloom: creating surface...
[22024683648] [INFO] [bloom] [CPU3] bloom: surface created!
[22025981439] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[22039809429] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[22056088395] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1263
[22057213926] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[22069704591] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[22072519194] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[22074013830] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[22075233543] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [22083154137] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[22092805416] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[22101555762] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[22107366666] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[22110333366] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
T:07D0 T:0640 T:F0B0 [22130484618] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1263
[22144546116] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([237, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[22153325469] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[22161042783] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[22165130262] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[22168057065] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[22171946412] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[22176485958] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[22181801400] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=31, our_read=32)
[22183897098] [INFO] [anther] [CPU1] anther: Connected to network stack
[22191085092] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[22193408061] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
T:1220 [22201291332] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[22203827646] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[22240720161] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=277
[22242139755] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[22243603371] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22244819751] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22246192188] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22247726160] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=277 subj_lo=0
[22262998395] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[22264539594] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[22271449002] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[22301996904] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[22306156257] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1269)
[22307378247] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[22318474563] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[22319779416] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=243
[22321064898] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[22322609991] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22324368000] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22326232797] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22328163132] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[22336281561] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[22340771244] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1276)
[22342379829] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[22355603259] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=281
[22357298997] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[22358713113] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22360211610] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22361969091] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22363875435] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=281 subj_lo=0
[22377265680] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1279)
[22378983099] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[22380053487] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[22383935805] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[22397627505] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[22404572454] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[22432022085] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[22454635929] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[22457776968] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[22461637077] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[22493434095] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[22500084915] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[22502904963] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[22535154939] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[22600652778] [INFO] [netd] [CPU3] NETD: DHCP configured ��� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[22605715143] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[22613517003] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[22616922273] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=1
[22620382026] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[22623873063] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=2
[22627690371] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[22629088086] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=2)
[22630911534] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[22633865232] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[22859026839] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[22969298088] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[22990674861] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=653 watches=13 history=1024 journal=1024 symbols=306 drops=0
[23001842820] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[23132713725] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[23362555788] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[23568793413] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[23749079970] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[24154330332] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[24181831443] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[24400920093] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[24639537582] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[24905244738] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[25008861768] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=706 watches=13 history=1024 journal=1024 symbols=338 drops=0
[25122197793] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[25157511324] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[25163039253] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[25165166796] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[25166702088] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[25168465014] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[25170567576] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[25182718407] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[25184097774] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[25185876276] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[25187953098] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=338 pred=0 subj_lo=0
[25360210227] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[25677358872] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[25801859292] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[25804507047] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[26027024595] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[26033749203] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26040151137] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26041896474] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26043660324] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26045573829] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=348 pred=0 subj_lo=0
[26153254908] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[26168550870] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26169838827] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26171121834] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26172585252] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=349 pred=0 subj_lo=0
[26212687116] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[26234702439] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26325857745] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26421623877] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[26449989324] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[26468373921] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26479838748] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[26480710080] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[26544233199] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26545661505] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26546920653] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26548326420] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=350 pred=0 subj_lo=0
[26581473732] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26607315603] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[26618093370] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x121fd000
[26620050765] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[26622295854] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[26693353599] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[26703463017] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[26708701998] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[26711228082] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[26713747071] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[26719423434] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[26729340165] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[26768666397] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[26770908186] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[26774559537] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[26777885112] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[26780499339] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[26783510490] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[26785330605] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=1 (16384b)
[26806413513] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x12223000
[26808298011] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[26863773387] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=fec1d29ef0678173)
[27216638196] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[27573422613] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=14aa2015f9906e3f)
[27868506105] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[28076443659] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=786 watches=18 history=1024 journal=1024 symbols=364 drops=0
[28186507305] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[28509133917] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[28572545166] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28804294299] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[29165036010] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[29487416475] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[29778001902] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[30068482389] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[30368060712] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[30560408142] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=836 watches=18 history=1024 journal=1024 symbols=364 drops=0
[30723925155] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[31027529478] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[31323715500] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[31601268006] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[31637397429] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[31707698484] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[31733548506] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[31785088533] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[31868902230] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[31870703601] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[31872556914] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[31874491572] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=351 pred=0 subj_lo=0
[31957337775] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[32482274121] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=886 watches=19 history=1024 journal=1024 symbols=367 drops=0
[32932824144] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2429 ops=1 watches=19
[32993734620] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[33011790405] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[33637203369] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[34151902785] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=904 watches=19 history=1024 journal=1024 symbols=367 drops=0
[34365014244] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[34838552199] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[35139139893] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[35254143342] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[35588513961] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[35613818922] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[36478600290] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=956 watches=19 history=1024 journal=1024 symbols=390 drops=0
[40642342851] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[40773263520] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1057 watches=19 history=1024 journal=1024 symbols=451 drops=0
[40784859588] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[41314281558] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[41316868197] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[41348187375] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[41909818401] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[41960123667] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[41961809406] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[41963636319] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[41965595661] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[41974944627] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[41976555654] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[41978348874] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[41980305411] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=235 pred=0 subj_lo=0
[41990925999] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[41992713048] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[41994496632] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[41996421588] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=453 pred=0 subj_lo=0
[42008867274] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[42010557039] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[42012375933] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[42014312274] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=452 pred=0 subj_lo=0
[42025763736] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[42027491814] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[42029369547] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[42031247775] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[42048499647] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[42775869807] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1086 watches=24 history=1024 journal=1024 symbols=454 drops=0
[45197471484] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1112 watches=24 history=1024 journal=1024 symbols=454 drops=0
[47242248471] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=1143 watches=24 history=1024 journal=1024 symbols=454 drops=0
[49339250928] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=1169 watches=24 history=1024 journal=1024 symbols=454 drops=0
[51385591653] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=16000 nodes=1193 watches=24 history=1024 journal=1024 symbols=454 drops=0
[52609649796] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=3453 ops=1 watches=24
[53462290233] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=17000 nodes=1224 watches=24 history=1024 journal=1024 symbols=454 drops=0
[55593118878] [INFO] [fontd] [CPU3] FONTD: Auto-importing font asset ThingId([31, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[56184082878] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=18000 nodes=1257 watches=24 history=1024 journal=1024 symbols=454 drops=0
[58523559963] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=19000 nodes=1286 watches=24 history=1024 journal=1024 symbols=454 drops=0
[60675666183] [INFO] [fontd] [CPU3] FONTD: Auto-imported font 'Noto Sans' style 'Regular' from asset ThingId([31, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[60679868766] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (1 fonts)
[60702010710] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=20000 nodes=1321 watches=24 history=1024 journal=1024 symbols=456 drops=0
[63002522187] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=21000 nodes=1349 watches=24 history=1024 journal=1024 symbols=456 drops=0
[65143267365] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=22000 nodes=1375 watches=24 history=1024 journal=1024 symbols=456 drops=0
[67315507347] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=23000 nodes=1408 watches=24 history=1024 journal=1024 symbols=456 drops=0
[70022748708] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=24000 nodes=1434 watches=24 history=1024 journal=1024 symbols=456 drops=0
[71506991820] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=1
[71820046551] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[71823858546] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[72778867392] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=25000 nodes=1476 watches=24 history=1024 journal=1024 symbols=456 drops=0
[74919131166] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=26000 nodes=1499 watches=24 history=1024 journal=1024 symbols=456 drops=0
[77338645428] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=27000 nodes=1531 watches=24 history=1024 journal=1024 symbols=456 drops=0
[79885513224] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=28000 nodes=1559 watches=24 history=1024 journal=1024 symbols=456 drops=0
[82354606257] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=29000 nodes=1583 watches=24 history=1024 journal=1024 symbols=456 drops=0
[85048232016] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=30000 nodes=1622 watches=24 history=1024 journal=1024 symbols=456 drops=0
[85056820893] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=4477 ops=1 watches=24
[87686929242] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=31000 nodes=1644 watches=24 history=1024 journal=1024 symbols=456 drops=0
[90258820443] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=32000 nodes=1666 watches=24 history=1024 journal=1024 symbols=456 drops=0
[93091523646] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=33000 nodes=1707 watches=24 history=1024 journal=1024 symbols=456 drops=0
[96169302834] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=34000 nodes=1735 watches=24 history=1024 journal=1024 symbols=456 drops=0
[99069320757] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=35000 nodes=1767 watches=24 history=1024 journal=1024 symbols=456 drops=0
[102246618870] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=36000 nodes=1804 watches=24 history=1024 journal=1024 symbols=456 drops=0
[105437728143] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=37000 nodes=1832 watches=24 history=1024 journal=1024 symbols=456 drops=0
[107545227636] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=38000 nodes=1856 watches=24 history=1024 journal=1024 symbols=456 drops=0
[109695949176] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=39000 nodes=1878 watches=24 history=1024 journal=1024 symbols=456 drops=0
[112159589949] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=40000 nodes=1911 watches=24 history=1024 journal=1024 symbols=456 drops=0
[114979717710] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=41000 nodes=1941 watches=24 history=1024 journal=1024 symbols=456 drops=0
[117772539093] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=42000 nodes=1973 watches=24 history=1024 journal=1024 symbols=456 drops=0
[119892960132] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=43000 nodes=2004 watches=24 history=1024 journal=1024 symbols=456 drops=0
[120896966520] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=5501 ops=1 watches=24
[122053264377] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=44000 nodes=2032 watches=24 history=1024 journal=1024 symbols=456 drops=0
[123911433954] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=45000 nodes=2058 watches=24 history=1024 journal=1024 symbols=456 drops=0
[125677194984] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=46000 nodes=2084 watches=24 history=1024 journal=1024 symbols=456 drops=0
[127615367121] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=47000 nodes=2119 watches=24 history=1024 journal=1024 symbols=456 drops=0
[129609228936] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=48000 nodes=2149 watches=24 history=1024 journal=1024 symbols=456 drops=0
[131313914325] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=49000 nodes=2173 watches=24 history=1024 journal=1024 symbols=456 drops=0
[133349687163] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=50000 nodes=2206 watches=24 history=1024 journal=1024 symbols=456 drops=0
[135523454121] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=51000 nodes=2238 watches=24 history=1024 journal=1024 symbols=456 drops=0
[137722006752] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=52000 nodes=2268 watches=24 history=1024 journal=1024 symbols=456 drops=0
[139586234469] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=53000 nodes=2294 watches=24 history=1024 journal=1024 symbols=456 drops=0
[141724788282] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=54000 nodes=2327 watches=24 history=1024 journal=1024 symbols=456 drops=0
[143607511146] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=55000 nodes=2357 watches=24 history=1024 journal=1024 symbols=456 drops=0
[145741040247] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=56000 nodes=2385 watches=24 history=1024 journal=1024 symbols=456 drops=0
[147946382331] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=6525 ops=1 watches=24
[148641178983] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=57000 nodes=2434 watches=24 history=1024 journal=1024 symbols=456 drops=0
[150829182900] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=58000 nodes=2462 watches=24 history=1024 journal=1024 symbols=456 drops=0
[152950686603] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=59000 nodes=2488 watches=24 history=1024 journal=1024 symbols=456 drops=0
[155173047249] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=60000 nodes=2516 watches=24 history=1024 journal=1024 symbols=456 drops=0
[157309992741] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=61000 nodes=2551 watches=24 history=1024 journal=1024 symbols=456 drops=0
[159334480173] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=62000 nodes=2577 watches=24 history=1024 journal=1024 symbols=456 drops=0
[161339379795] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=63000 nodes=2603 watches=24 history=1024 journal=1024 symbols=456 drops=0
[163527911382] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=64000 nodes=2634 watches=24 history=1024 journal=1024 symbols=456 drops=0
[165850032315] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=65000 nodes=2664 watches=24 history=1024 journal=1024 symbols=456 drops=0
[168034040955] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=66000 nodes=2694 watches=24 history=1024 journal=1024 symbols=456 drops=0
[169936578900] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=67000 nodes=2716 watches=24 history=1024 journal=1024 symbols=456 drops=0
[172033726821] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=68000 nodes=2753 watches=24 history=1024 journal=1024 symbols=456 drops=0
[174051167334] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=69000 nodes=2781 watches=24 history=1024 journal=1024 symbols=456 drops=0
[175880687928] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=70000 nodes=2809 watches=24 history=1024 journal=1024 symbols=456 drops=0
[177578438730] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=7549 ops=1 watches=24
[177805314522] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=71000 nodes=2833 watches=24 history=1024 journal=1024 symbols=456 drops=0
[180037601898] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=72000 nodes=2872 watches=24 history=1024 journal=1024 symbols=456 drops=0
[182165106057] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=73000 nodes=2900 watches=24 history=1024 journal=1024 symbols=456 drops=0
[184227990243] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=74000 nodes=2928 watches=24 history=1024 journal=1024 symbols=456 drops=0
[186242267013] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=75000 nodes=2950 watches=24 history=1024 journal=1024 symbols=456 drops=0
[188304185784] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=76000 nodes=2981 watches=24 history=1024 journal=1024 symbols=456 drops=0
[190151290626] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=77000 nodes=3011 watches=24 history=1024 journal=1024 symbols=456 drops=0
[192559912479] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=78000 nodes=3045 watches=24 history=1024 journal=1024 symbols=456 drops=0
[194975277915] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=79000 nodes=3078 watches=24 history=1024 journal=1024 symbols=456 drops=0
[197379881061] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=80000 nodes=3104 watches=24 history=1024 journal=1024 symbols=456 drops=0
[199989187002] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=81000 nodes=3132 watches=24 history=1024 journal=1024 symbols=456 drops=0
[202136158290] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=82000 nodes=3156 watches=24 history=1024 journal=1024 symbols=456 drops=0
[204067487019] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=83000 nodes=3189 watches=24 history=1024 journal=1024 symbols=456 drops=0
[206166446145] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=84000 nodes=3217 watches=24 history=1024 journal=1024 symbols=456 drops=0
[207175249611] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=8573 ops=1 watches=24
[208757404548] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=85000 nodes=3267 watches=24 history=1024 journal=1024 symbols=456 drops=0
[211000614429] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=86000 nodes=3295 watches=24 history=1024 journal=1024 symbols=456 drops=0
[213195406281] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=87000 nodes=3328 watches=24 history=1024 journal=1024 symbols=456 drops=0
[215444952690] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=88000 nodes=3352 watches=24 history=1024 journal=1024 symbols=456 drops=0
[217288515246] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=89000 nodes=3376 watches=24 history=1024 journal=1024 symbols=456 drops=0
[219566126538] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=90000 nodes=3412 watches=24 history=1024 journal=1024 symbols=456 drops=0
[221958596772] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=91000 nodes=3447 watches=24 history=1024 journal=1024 symbols=456 drops=0
[224186653218] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=92000 nodes=3473 watches=24 history=1024 journal=1024 symbols=456 drops=0
[226333373079] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=93000 nodes=3499 watches=24 history=1024 journal=1024 symbols=456 drops=0
[228596323395] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=94000 nodes=3529 watches=24 history=1024 journal=1024 symbols=456 drops=0
[230737808169] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=95000 nodes=3566 watches=24 history=1024 journal=1024 symbols=456 drops=0
[233256765885] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=96000 nodes=3594 watches=24 history=1024 journal=1024 symbols=456 drops=0
[235492467639] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=97000 nodes=3628 watches=24 history=1024 journal=1024 symbols=456 drops=0
[237326789931] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=9597 ops=1 watches=24
[237480208251] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=98000 nodes=3654 watches=24 history=1024 journal=1024 symbols=456 drops=0
[239714355705] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=99000 nodes=3691 watches=24 history=1024 journal=1024 symbols=456 drops=0
[242041691925] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=100000 nodes=3721 watches=24 history=1024 journal=1024 symbols=456 drops=0
[244265120253] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=101000 nodes=3751 watches=24 history=1024 journal=1024 symbols=456 drops=0
[246254736387] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=102000 nodes=3773 watches=24 history=1024 journal=1024 symbols=456 drops=0
[248760538752] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=103000 nodes=3816 watches=24 history=1024 journal=1024 symbols=456 drops=0
[251205496641] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=104000 nodes=3846 watches=24 history=1024 journal=1024 symbols=456 drops=0
[253447635672] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=105000 nodes=3874 watches=24 history=1024 journal=1024 symbols=456 drops=0
[255673902264] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=106000 nodes=3900 watches=24 history=1024 journal=1024 symbols=456 drops=0
[257996537205] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=107000 nodes=3935 watches=24 history=1024 journal=1024 symbols=456 drops=0
[260402191599] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=108000 nodes=3965 watches=24 history=1024 journal=1024 symbols=456 drops=0
[262830480429] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=109000 nodes=3993 watches=24 history=1024 journal=1024 symbols=456 drops=0
[265281029673] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=110000 nodes=4019 watches=24 history=1024 journal=1024 symbols=456 drops=0
[267788470026] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=111000 nodes=4063 watches=24 history=1024 journal=1024 symbols=456 drops=0
[267949429044] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=10621 ops=1 watches=24
[269892763536] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=112000 nodes=4086 watches=24 history=1024 journal=1024 symbols=456 drops=0
[272045275392] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=113000 nodes=4112 watches=24 history=1024 journal=1024 symbols=456 drops=0
[274156663209] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=114000 nodes=4144 watches=24 history=1024 journal=1024 symbols=456 drops=0
[276162909858] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=115000 nodes=4168 watches=24 history=1024 journal=1024 symbols=456 drops=0
[278495449221] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=116000 nodes=4205 watches=24 history=1024 journal=1024 symbols=456 drops=0
[280839067839] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=117000 nodes=4233 watches=24 history=1024 journal=1024 symbols=456 drops=0
[282753481329] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=118000 nodes=4257 watches=24 history=1024 journal=1024 symbols=456 drops=0
[285060094176] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=119000 nodes=4287 watches=24 history=1024 journal=1024 symbols=456 drops=0
[287498420121] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=120000 nodes=4320 watches=24 history=1024 journal=1024 symbols=456 drops=0
[289746232149] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=121000 nodes=4346 watches=24 history=1024 journal=1024 symbols=456 drops=0
[292151379630] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=122000 nodes=4374 watches=24 history=1024 journal=1024 symbols=456 drops=0
[294616457982] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=123000 nodes=4400 watches=24 history=1024 journal=1024 symbols=456 drops=0
[297547414725] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=124000 nodes=4441 watches=24 history=1024 journal=1024 symbols=456 drops=0
[300219641700] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=125000 nodes=4467 watches=24 history=1024 journal=1024 symbols=456 drops=0
[300369634719] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=11645 ops=1 watches=24
[302465695224] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=126000 nodes=4495 watches=24 history=1024 journal=1024 symbols=456 drops=0
[304746150918] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=127000 nodes=4523 watches=24 history=1024 journal=1024 symbols=456 drops=0
[306667832427] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=128000 nodes=4547 watches=24 history=1024 journal=1024 symbols=456 drops=0
[309268887609] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=129000 nodes=4582 watches=24 history=1024 journal=1024 symbols=456 drops=0
[311665033713] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=130000 nodes=4612 watches=24 history=1024 journal=1024 symbols=456 drops=0
[313806505353] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=131000 nodes=4640 watches=24 history=1024 journal=1024 symbols=456 drops=0
[316125421425] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=132000 nodes=4668 watches=24 history=1024 journal=1024 symbols=456 drops=0
[318696210921] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=133000 nodes=4705 watches=24 history=1024 journal=1024 symbols=456 drops=0
[321245071653] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=134000 nodes=4739 watches=24 history=1024 journal=1024 symbols=456 drops=0
[323404929111] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=135000 nodes=4777 watches=24 history=1024 journal=1024 symbols=456 drops=0
[325237932096] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=136000 nodes=4801 watches=24 history=1024 journal=1024 symbols=456 drops=0
[327335271582] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=137000 nodes=4825 watches=24 history=1024 journal=1024 symbols=456 drops=0
[329917948437] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=138000 nodes=4860 watches=24 history=1024 journal=1024 symbols=456 drops=0
[331693896171] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=12669 ops=1 watches=24
[332418681177] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=139000 nodes=4892 watches=24 history=1024 journal=1024 symbols=456 drops=0
[334516528236] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=140000 nodes=4920 watches=24 history=1024 journal=1024 symbols=456 drops=0
[337042612401] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=141000 nodes=4956 watches=24 history=1024 journal=1024 symbols=456 drops=0
[339384600258] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=142000 nodes=4993 watches=24 history=1024 journal=1024 symbols=456 drops=0
[342083104803] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=143000 nodes=5021 watches=24 history=1024 journal=1024 symbols=456 drops=0
[344498484363] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=144000 nodes=5043 watches=24 history=1024 journal=1024 symbols=456 drops=0
[346751375784] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=145000 nodes=5069 watches=24 history=1024 journal=1024 symbols=456 drops=0
[349246088235] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=146000 nodes=5102 watches=24 history=1024 journal=1024 symbols=456 drops=0
[352079323497] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=147000 nodes=5132 watches=24 history=1024 journal=1024 symbols=456 drops=0
[354656767839] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=148000 nodes=5162 watches=24 history=1024 journal=1024 symbols=456 drops=0
[356975988996] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=149000 nodes=5192 watches=24 history=1024 journal=1024 symbols=456 drops=0
[358970176554] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=150000 nodes=5218 watches=24 history=1024 journal=1024 symbols=456 drops=0
[361662059418] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=151000 nodes=5253 watches=24 history=1024 journal=1024 symbols=456 drops=0
[364267074897] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=152000 nodes=5277 watches=24 history=1024 journal=1024 symbols=456 drops=0
[365452490238] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=13693 ops=1 watches=24
[367331179017] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=153000 nodes=5313 watches=24 history=1024 journal=1024 symbols=456 drops=0
[369561127056] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=154000 nodes=5339 watches=24 history=1024 journal=1024 symbols=456 drops=0
[371949928317] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=155000 nodes=5370 watches=24 history=1024 journal=1024 symbols=456 drops=0
[374233340844] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=156000 nodes=5398 watches=24 history=1024 journal=1024 symbols=456 drops=0
[376668584853] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=157000 nodes=5426 watches=24 history=1024 journal=1024 symbols=456 drops=0
[379193601501] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=158000 nodes=5452 watches=24 history=1024 journal=1024 symbols=456 drops=0
[381636346740] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=159000 nodes=5487 watches=24 history=1024 journal=1024 symbols=456 drops=0
[385405478205] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=160000 nodes=5529 watches=24 history=1024 journal=1024 symbols=456 drops=0
[387808340139] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=161000 nodes=5557 watches=24 history=1024 journal=1024 symbols=456 drops=0
[390296116188] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=162000 nodes=5585 watches=24 history=1024 journal=1024 symbols=456 drops=0
[392517600189] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=163000 nodes=5609 watches=24 history=1024 journal=1024 symbols=456 drops=0
[394842541962] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=164000 nodes=5644 watches=24 history=1024 journal=1024 symbols=456 drops=0
[397669490823] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=165000 nodes=5678 watches=24 history=1024 journal=1024 symbols=456 drops=0
[400154141904] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=14717 ops=1 watches=24
[400411910844] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=166000 nodes=5708 watches=24 history=1024 journal=1024 symbols=456 drops=0
[403050486828] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=167000 nodes=5740 watches=24 history=1024 journal=1024 symbols=456 drops=0
[405892411947] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=168000 nodes=5775 watches=24 history=1024 journal=1024 symbols=456 drops=0
[409135109724] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=169000 nodes=5815 watches=24 history=1024 journal=1024 symbols=456 drops=0
[411233961699] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=170000 nodes=5839 watches=24 history=1024 journal=1024 symbols=456 drops=0
[413397111336] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=171000 nodes=5861 watches=24 history=1024 journal=1024 symbols=456 drops=0
[415848921213] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=172000 nodes=5891 watches=24 history=1024 journal=1024 symbols=456 drops=0
[418481348175] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=173000 nodes=5924 watches=24 history=1024 journal=1024 symbols=456 drops=0
[422783074032] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=174000 nodes=5960 watches=24 history=1024 journal=1024 symbols=456 drops=0
[425408213571] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=175000 nodes=5990 watches=24 history=1024 journal=1024 symbols=456 drops=0
[427973987991] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=176000 nodes=6014 watches=24 history=1024 journal=1024 symbols=456 drops=0
[430657814862] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=177000 nodes=6053 watches=24 history=1024 journal=1024 symbols=456 drops=0
[433920159453] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=178000 nodes=6097 watches=24 history=1024 journal=1024 symbols=456 drops=0
[436269297948] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=179000 nodes=6129 watches=24 history=1024 journal=1024 symbols=456 drops=0
[436310233788] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=15741 ops=1 watches=24
[438949337475] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=180000 nodes=6157 watches=24 history=1024 journal=1024 symbols=456 drops=0
[441623705259] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=181000 nodes=6188 watches=24 history=1024 journal=1024 symbols=456 drops=0
[444593895537] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=182000 nodes=6224 watches=24 history=1024 journal=1024 symbols=456 drops=0
[446933939892] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=183000 nodes=6252 watches=24 history=1024 journal=1024 symbols=456 drops=0
[449204798691] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=184000 nodes=6280 watches=24 history=1024 journal=1024 symbols=456 drops=0
[451285359624] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=185000 nodes=6304 watches=24 history=1024 journal=1024 symbols=456 drops=0
[453916166814] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=186000 nodes=6339 watches=24 history=1024 journal=1024 symbols=456 drops=0
[456995007348] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=187000 nodes=6371 watches=24 history=1024 journal=1024 symbols=456 drops=0
[459577946322] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=188000 nodes=6399 watches=24 history=1024 journal=1024 symbols=456 drops=0
[462209729256] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=189000 nodes=6427 watches=24 history=1024 journal=1024 symbols=456 drops=0
[465075642504] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=190000 nodes=6460 watches=24 history=1024 journal=1024 symbols=456 drops=0
[468068108673] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=191000 nodes=6486 watches=24 history=1024 journal=1024 symbols=456 drops=0
[470515238094] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=192000 nodes=6512 watches=24 history=1024 journal=1024 symbols=456 drops=0
[472416094062] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=16765 ops=1 watches=24
[473269424892] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=193000 nodes=6542 watches=24 history=1024 journal=1024 symbols=456 drops=0
[475669828722] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=194000 nodes=6564 watches=24 history=1024 journal=1024 symbols=456 drops=0
[478819029546] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=195000 nodes=6607 watches=24 history=1024 journal=1024 symbols=456 drops=0
[481959484446] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=196000 nodes=6637 watches=24 history=1024 journal=1024 symbols=456 drops=0
[484951759413] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=197000 nodes=6673 watches=24 history=1024 journal=1024 symbols=456 drops=0
[487168708983] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=198000 nodes=6695 watches=24 history=1024 journal=1024 symbols=456 drops=0
[490917849708] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=199000 nodes=6746 watches=24 history=1024 journal=1024 symbols=456 drops=0
[493877778075] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=200000 nodes=6774 watches=24 history=1024 journal=1024 symbols=456 drops=0
[496508317602] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=201000 nodes=6798 watches=24 history=1024 journal=1024 symbols=456 drops=0
[499292643036] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=202000 nodes=6826 watches=24 history=1024 journal=1024 symbols=456 drops=0
[502107984708] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=203000 nodes=6862 watches=24 history=1024 journal=1024 symbols=456 drops=0
[505587482730] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=204000 nodes=6899 watches=24 history=1024 journal=1024 symbols=456 drops=0
[508338360717] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=205000 nodes=6925 watches=24 history=1024 journal=1024 symbols=456 drops=0
[510566459931] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=17789 ops=1 watches=24
[510924168117] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=206000 nodes=6949 watches=24 history=1024 journal=1024 symbols=456 drops=0
[513820295802] [INFO] [k
```
</details>
