# ❌ Scenario: Clock updates over time

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 3950ms | - - - |
| 2 | Then I should see a clock window displaying a ticking clock | ❌ | 31060ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12497161809] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[12503585787] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[12507481305] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[12509829123] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[12511401936] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[12512382267] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[12513399360] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[12514292901] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[12515121762] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[12515724309] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[12516293955] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[12516868782] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[12517552146] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[12518226006] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[12518884191] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[12519467928] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[12520076844] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[12520652529] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[12521250687] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[12521846733] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[12522409977] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[12522980844] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[12523563888] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[12524152113] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[12524784723] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[12525392583] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[12525974307] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[12526604310] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[12527178378] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[12527773005] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[12528384099] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[12528986052] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[12529595034] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[12530194611] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[12530778546] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[12531474351] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[12532161741] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[12532854774] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[12533535630] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[12534246219] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[12534968160] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[12535694886] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[12536965551] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[12538297728] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[12539041383] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[12539572881] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[12540073095] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[12540584067] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[12541124640] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[12541676796] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[12542182092] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[12542693196] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[12543196941] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[12543725370] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[12544260597] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[12544829682] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[12545363985] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[12545918220] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[12546452622] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[12547002996] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[12547537035] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[12548097243] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[12548623461] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[12549166278] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[12549692826] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[12550234851] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[12550760376] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[12551312400] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[12551839245] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[12552385857] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[12552913197] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[12553459710] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[12553988271] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[12554548578] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[12555077337] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[12555624312] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[12556152807] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[12556700574] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[12557227947] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[12557788089] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[12558315396] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[12558862305] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[12559388721] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[12559935366] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[12560462211] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[12561018294] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[12561546426] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[12562090563] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[12562617705] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[12563193555] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[12563724723] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[12564288330] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[12564877413] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[12565483557] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[12566028222] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[12566589354] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[12567127155] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[12567705513] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[12568239750] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[12568809396] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[12569356701] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[12569907735] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[12570441444] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[12571008120] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[12571538595] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[12572080125] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[12572769066] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[12573850278] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12803926740] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[12815302533] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12820093506] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12821358462] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12822213294] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12826657767] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12828358785] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12829405248] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12830079603] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12830731980] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12831401616] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12832351653] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12833408610] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12834095208] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12834785007] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12835461705] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12836121870] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12837371019] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12838549515] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12839261160] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12840815097] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12841755366] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12842726919] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12844393650] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12846038535] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12846946860] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12847629663] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12848411004] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[13211913594] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[13212910095] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[13216411230] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[13217286159] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[13218041397] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[13219538112] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[13232002443] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[13233290136] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[13234027488] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[13235755500] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[13236310065] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[13238674053] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[13246462317] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[13248332559] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[13261760919] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[13262312844] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[13278733710] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[13279356222] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[13281370014] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[13282595832] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[13283664174] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[13285906062] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[13286737431] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[13321586520] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62400200 ticks/sec), init_cnt=624002 for 100Hz
[13323193323] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[13324070232] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[13325217147] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[13330719402] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[13361365578] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[13362336405] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[13363842393] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[13365813483] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[13367626734] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[13370945511] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[13372795557] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[13394930142] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[13397020824] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[13398146322] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[13398863115] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[13399571757] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[13400200275] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[13400949870] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[13426782501] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[13427524968] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[13429289874] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[13430297067] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[13431299541] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[13432277265] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[13433452791] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[13434126882] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[13439416551] [INFO] [kernel::root] [CPU0] Spawning Root service...
[13440376587] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[13442625834] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[13443598641] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[13448659356] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[13451017569] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[13452286056] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[13454328789] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[13455921831] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[13457440095] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[13469833476] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[13473487005] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[13475293788] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[13476781923] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13499065833] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13520716638] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13522826625] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13527243609] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13529514207] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13531584528] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13533480180] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13534348839] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13535355504] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13541599005] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13543219932] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13545594381] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13547915304] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13550130924] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13553027169] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13553802867] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13555020303] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13567815624] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13568726952] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13575261216] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13576079385] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13602874197] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13603612836] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13933578087] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[14423178330] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[14454936408] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=500 journal=424 symbols=52 drops=0
[14486889846] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[15788028894] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=963 journal=771 symbols=97 drops=0
[16547801424] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[16649404299] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[16650784656] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[16755849792] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[16824602256] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[16852677435] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[16853715912] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[16854624963] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[16860360231] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[16880115912] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[16897060554] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[16903417443] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[16959162198] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[16980054663] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[16980956091] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[16985287176] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[17013598569] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[17032612575] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[17039173668] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[17040158553] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[17108915340] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[17111056050] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[17200478361] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[17268810339] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[17280228075] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[17284949352] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[17287174938] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[17306941839] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[17354188533] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[17360111208] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[17361150708] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[17361976830] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[17362898058] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[17363595414] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[17364254952] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[17364923268] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[17365554096] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[17366276928] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[17367264123] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[17368047774] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[17368693089] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[17369351604] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[17370065625] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[17370791163] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[17371440009] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[17372102979] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[17372736381] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[17373596130] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[17374464360] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[17375084529] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[17375698857] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[17376565767] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[17377315230] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[17377982853] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[17378618598] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[17379245895] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[17379944835] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[17380565235] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[17381199858] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[17381854347] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[17382505272] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[17383157385] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[17383797651] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[17384416995] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[17385149628] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[17385889950] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[17386655946] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[17387390460] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[17388157314] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[17388957828] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[17389831107] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[17391297132] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[17393071344] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[17393915484] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17401906995] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[17417499297] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[17422138305] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[17431251321] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[17431952208] [CONTRACT] [kernel] [CPU0] Spawning init process...
[17434382031] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[17437132779] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[17438214090] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
ThingOS Petals
type 'help' foUSER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARrG0=0x600000
 commands

petals> [17443717566] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[17445407463] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013248 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[17468040777] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[17471237751] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[17473509669] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[17475393738] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[17483902491] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[17489085306] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[17492676828] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[17494380552] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[17498699262] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[17502470931] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[17504257188] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[17509173264] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[17510890287] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[17512732380] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[17514446169] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[17521099530] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[17522248491] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[17523437118] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[17525066724] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[17526523806] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[17527819122] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[17533527165] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[17576960445] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[17585325978] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[17590927398] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[17596092558] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[17599734174] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[17604377439] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[17609726574] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[17615385249] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[17620877670] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[17626139487] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[17631964779] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[17638273752] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[17644042020] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[17649296181] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[17654407188] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[17659746423] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[17665831260] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[17671753440] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[17676853227] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[17682108279] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[17687691318] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[17695211952] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[17701699884] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[17706830625] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[17712093300] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[17716587372] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[17721127215] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[17726047251] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[17731576302] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[17737585602] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[17741510853] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[17745060696] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[17750739105] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[17756040522] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[17761698504] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[17767203135] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[17773121520] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[17777820555] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[17782976343] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[17787893871] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[17793021048] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[17796315537] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[17815469463] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[17957367780] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[17964430242] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[17965333056] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17967342657] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17972254146] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17973561969] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17981856156] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[17987023626] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[17988335706] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17990052333] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078784 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[17996476509] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[18000070605] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[18000839868] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18003032421] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18007385121] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18009107292] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18016217934] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[18018560736] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[18020162556] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[18021994518] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144320 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[18025079787] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[18029150106] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[18031786707] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[18035264379] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18039453069] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:03:23 = 1775437403 unix_secs
[18041047761] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775437403, mono_ns=9020306658, offset=1775437393979693342ns
[18042769503] [INFO] [rtc_cmos] [CPU1] System clock anchored
[18053313036] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[18094607190] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[18106819071] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[18107764158] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18109390431] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18115538529] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18117833844] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[18125019099] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[18127693386] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[18131160729] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18132791325] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369211008 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[18138390996] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[18142752672] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[18144461313] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[18145297236] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18146918064] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18153855885] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18156076950] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18163407834] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[18166727667] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18167853033] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18169248735] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277472 RFLAGS_BEFORE=130 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[18173387100] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[18174615294] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[18178544340] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[18182564136] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[18184380786] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[18185399859] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[18186440085] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[18187489023] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[18204116139] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[18205627209] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[18741867969] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18746590995] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[18751043058] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[18752829282] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[18757508286] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[18759343086] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[18761456175] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[18763079709] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[18764268666] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[18767117655] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[18769482303] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[18770584734] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[18771545859] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[18772499229] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[18774031056] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[18776486487] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[18782220270] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[18783538983] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18786248547] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18793404135] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18795705720] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18802547511] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[18805731483] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fb2d0
[18806610570] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[18808453983] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369353104 RFLAGS_BEFORE=134 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[18815559477] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[18824077041] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[18838686009] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[18845212221] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[18847386954] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[18855075789] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[18864613218] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[18866101980] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[18872846223] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[18874263738] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[18875268786] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[18876168498] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[18876997722] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[18877761573] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[18878959473] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[18880236672] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[18883320423] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[18886432026] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[18889140138] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[18891513003] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[18894581640] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[18895305165] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18896823099] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18900454947] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18902172267] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18909502887] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[18911708409] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[18913655937] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[18914970195] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18917348175] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18922827264] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18924897288] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18932439372] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[18935811675] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
[18937010004] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0105370
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[18939424350] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500672 RFLAGS_BEFORE=134 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[18944772726] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[18946157703] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18948453579] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[18949126515] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18950391075] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[18954166836] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[18955565178] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18956805747] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[18958374303] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[18959079810] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18961366809] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[18962717862] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[18964081653] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[18965796597] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[18967220349] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[18968840319] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[18969605556] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[18973039701] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[18974737815] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[18976069002] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[18978042798] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fe7c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[18979554528] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434592 RFLAGS_BEFORE=130 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[18984872742] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[19383434235] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[19384934250] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0105370
[19386943191] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[19388446473] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583456 RFLAGS_BEFORE=134 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[19391408520] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[19406984223] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[19408152753] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19409805327] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[19411440609] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[19412358702] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19413559473] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[19414638276] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19421168382] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[19423731822] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[19425981069] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[19434845991] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[19438215687] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[19440271785] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[19441682073] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[19442353062] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19443794634] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19462654464] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19466659410] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[19487101986] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[19489561047] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0108b48
[19490699382] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19492176033] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369717136 RFLAGS_BEFORE=134 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[19497165732] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fe7c8
[19498206420] [INFO] [netd] [CPU3] NETD: Starting network stack service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[19499584797] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650528 RFLAGS_BEFORE=134 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[19502780418] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[19504730850] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[19506260268] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[19516300749] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[19519882470] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[19523800758] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[19525176594] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[19526722017] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[19528075083] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[19529175501] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[19531392672] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[19532891103] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[19534682376] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[19535514966] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19537253505] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19603652706] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[19626946680] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[19633957035] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[19636997589] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fe7c8
[19639005111] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19640560005] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369783200 RFLAGS_BEFORE=134 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[19645235973] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[19646069949] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19647410772] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[19648416678] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19650104001] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[19653775317] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19655285826] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19672556475] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[19675670256] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[19682653485] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[19685182374] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[19687179633] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[19688065881] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19689642159] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19695089799] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19697404122] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19703953236] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[19706062893] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
[19706803446] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a670
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19709599932] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915584 RFLAGS_BEFORE=134 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[19712381238] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19713964314] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19715160135] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[19716075060] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19717684503] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[19723843689] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19727316477] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19734081543] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[19736432496] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010a670
[19737674550] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19739251752] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981984 RFLAGS_BEFORE=130 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[19743510633] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[19744466148] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19746451494] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19755819666] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19759804152] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[19766957661] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[19769514138] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[19771723125] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[19772542647] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19774985769] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19803718770] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[19808577789] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[19815628701] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[19818300645] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010bc50
[19820184054] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19821572793] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370114144 RFLAGS_BEFORE=134 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[19825230678] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[19826080527] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19827689343] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19828601001] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[19844245905] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[19847195445] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19850646156] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[19854064263] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19855291929] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[19856233122] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19857505668] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19859492367] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[19861994196] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[19863581991] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19865581758] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370198208 RFLAGS_BEFORE=134 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[19869829221] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[19870889478] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19872622275] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[19873507368] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19874582046] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[19884413043] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19885434426] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19886611206] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19887848541] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[19889327469] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19890376407] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19891591830] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19892793327] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[19918882170] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[19923731190] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19925299647] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19926377295] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19927519128] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19929250869] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19930494804] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[19931270304] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19933202223] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[19934510805] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[19940263662] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[19942187595] [INFO] [fontd] [CPU3] FONTD: Service ready
[19943548944] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[19946123703] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[19949008893] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fe7c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19950384729] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849504 RFLAGS_BEFORE=134 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[19954602261] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19955609553] [INFO] [nectar] [CPU2] NECTAR: Started.
[19956111318] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19957230711] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19958382444] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[19959909519] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010a670
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19961346240] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370048064 RFLAGS_BEFORE=130 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[19965964062] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[19967693625] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19969205289] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370267344 RFLAGS_BEFORE=130 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[19972411701] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19973430807] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19974549771] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19975740741] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=235 subj_lo=0
[19977527427] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[19979237817] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[19980920355] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[19983591705] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19984545834] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19986122673] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19987313907] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=236 subj_lo=0
[19992746301] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[19994094351] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[19995640467] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[19996479261] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[19998576147] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[20007690780] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[20011496010] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[20021768712] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1253 backend=VirtIO-GPU
[20023320504] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[20024052675] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20025829560] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20026660995] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20027883942] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20029419993] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20030601525] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20031874038] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20033385669] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[20040202677] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20041189509] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20042189211] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20043250128] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[20063463684] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[20076258741] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[20077210791] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[20159791674] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[20174886831] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[20181826566] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[20184628860] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db4e8
[20185896291] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e5
[20187755214] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370358192 RFLAGS_BEFORE=130 CR3_BEFORE=72052736 fs_base=0 gs_base=18446744071564586640
[20192426298] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[20193719040] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20195756955] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[20197014090] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20201165358] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[20202578286] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20209447104] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[20211512640] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db4e8
[20212967148] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[20214601770] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370423728 RFLAGS_BEFORE=130 CR3_BEFORE=73101312 fs_base=0 gs_base=18446744071564586576
[20219341230] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[20220918168] [INFO] [echo] [CPU1] echo: starting up
[20223210216] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[20229471471] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[20236039461] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[20237894391] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[20248025556] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[20249035455] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[20250232926] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[20251130625] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f5000
[20252576454] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f5000
[20254193388] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f8000
[20256793623] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276787200)
[20258546154] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[20261274165] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f9000 phys=0x4656000
[20263129326] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[20264295414] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[20266125528] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[20269243038] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[20271039261] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[20282159766] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[20284418319] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[20291215956] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[20292719040] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[20294148699] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[20299474536] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20301160869] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20310006849] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[20313000642] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[20314981137] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[20315811879] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[20317038720] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20319077592] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20324225493] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20325222489] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[20326676403] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[20337609204] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[20342014143] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[20344606194] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[20346747135] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[20349004170] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[20350116336] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20352280014] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20362578588] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[20364001713] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20365512948] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[20367587988] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[20369293923] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[20376896595] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[20379623913] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[20381081820] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[20382720138] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[20383495968] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20385396042] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20387437488] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f1548
[20389063035] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20390852394] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[20392018911] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370576608 RFLAGS_BEFORE=130 CR3_BEFORE=73994240 fs_base=0 gs_base=18446744071564586640
[20400851724] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[20404555941] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[20406235047] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[20410748259] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1548
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20413260615] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370642144 RFLAGS_BEFORE=130 CR3_BEFORE=74141696 fs_base=0 gs_base=18446744071564586576
[20417429274] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[20420310009] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[20427765006] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[20428810347] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[20430209844] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[20431729032] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[20433450708] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[20435086947] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[20436552906] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[20437759386] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[20438999394] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[20444293353] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[20449486233] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[20460179982] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20463377913] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20479449738] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[20481302490] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[20495196612] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[20500296894] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[20524705443] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20526229581] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[20528504997] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20529607692] [INFO] [bloom] [CPU3] bloom: creating surface...
[20538673782] [INFO] [bloom] [CPU3] bloom: surface created!
[20539793934] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[20542458486] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20549306283] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[20558633007] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1261
[20559709731] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[20572391598] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[20575763406] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[20577662028] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[20579012487] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [20586496293] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[20595432990] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[20605830927] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[20612410929] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20613350802] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20614782870] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
T:07D0 T:0640 T:F0B0 [20629784010] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1261
[20635546899] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[20647291533] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[20649963345] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[20653865496] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[20654651424] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[20661189219] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[20664516345] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[20673724434] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[20676324966] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[20678740005] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
T:1220 [20694730617] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[20696871294] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[20734447371] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[20736626790] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[20748216423] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=278
[20750010402] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[20750907606] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20751909024] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20753054784] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20754370032] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=278 subj_lo=0
[20771251050] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1274)
[20773301901] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[20777688228] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([242, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[20791277067] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=243
[20792816649] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[20793920037] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20794979766] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20796348738] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20797590198] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[20814159267] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1278)
[20816065743] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[20830122456] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=282
[20831686524] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[20832885810] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20833872378] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20835525117] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20837164689] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=282 subj_lo=0
[20841830790] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[20843416605] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[20849557806] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[20865149415] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[20867394372] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=31, our_read=32)
[20870056416] [INFO] [anther] [CPU1] anther: Connected to network stack
[20880915759] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[20881752606] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[20884793688] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[20892130479] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1280)
[20893739460] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[20916492333] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[20919368448] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[20939490693] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[20945963940] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[20948747853] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[20953159326] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[20955356070] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[20957691810] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[21011532234] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[21015292980] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[21017878761] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[21065133837] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[21068394468] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[21073862469] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[21075846264] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[21077218371] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=1
[21079609881] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[21082005417] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=2
[21085994391] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[21087018150] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=2)
[21087933141] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[21090289902] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[21289615281] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[21303191052] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x11824000
[21304308267] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[21306310971] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[21308562891] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[21375872892] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[21388185621] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[21392788593] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[21394893069] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[21397550724] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[21404215833] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[21407559426] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[21408970374] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[21410805966] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[21443714259] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21515432499] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21527742225] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[21528950157] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21530105619] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21531325860] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=306 pred=0 subj_lo=0
[21544693038] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[21599674932] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21706906221] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[21724815222] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=676 watches=14 history=1024 journal=1024 symbols=335 drops=0
[21749869053] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21788328804] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[21823660551] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[21825395031] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21827222406] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21829212339] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=307 pred=0 subj_lo=0
[21843573774] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[21862267416] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[21864055686] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21865922100] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21867904542] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=308 pred=0 subj_lo=0
[21888002301] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[21892371270] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[21900189168] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[21914611356] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 16384)
[21916219578] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=1 (16384b)
[22021000551] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22022721633] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22024527261] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22026498582] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=309 pred=0 subj_lo=0
[22035307965] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22058247156] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[22087582341] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[22104803556] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[22341869583] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[22518197559] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[22691127261] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[22869950928] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[23076394671] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[23126364162] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[23128311195] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[23132385309] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[23133797577] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[23134719795] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[23135746293] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[23137104309] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[23146505877] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[23147631639] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[23148662427] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[23149953321] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=351 pred=0 subj_lo=0
[23262167016] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[23425375149] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[23582211939] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[23607057474] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=742 watches=19 history=1024 journal=1024 symbols=354 drops=0
[23757018252] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[23948876490] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=fec1d29ef0678173)
[24148786134] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[24220426857] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[24222189057] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[24239946225] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x1222e000
[24241312227] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[24405697701] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=c1e9a7b90a789e5c)
[24627230001] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[24841359543] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[25071388155] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[25096484952] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=791 watches=19 history=1024 journal=1024 symbols=354 drops=0
[25307740161] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[25590486042] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[25845232941] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[26116153932] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[26307992766] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[26371452492] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26405765232] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[26461651161] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[26478153768] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26559308688] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26771338242] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[26912587680] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=845 watches=19 history=1024 journal=1024 symbols=366 drops=0
[27023353170] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[27285464349] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[27593068701] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[27942513555] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[28557791295] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=878 watches=19 history=1024 journal=1024 symbols=367 drops=0
[28788245772] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[29060824419] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2429 ops=1 watches=19
[29273942280] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[29913012591] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[29987152206] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[30196022901] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=901 watches=19 history=1024 journal=1024 symbols=367 drops=0
[30382109835] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[30617390067] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[30731949666] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[31087190409] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[32171456394] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=963 watches=19 history=1024 journal=1024 symbols=402 drops=0
[33884907276] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[33996113118] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[34247289417] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1047 watches=19 history=1024 journal=1024 symbols=451 drops=0
[34271064300] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[34272947247] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[34296366819] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[34796968668] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[34843761777] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34844976639] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34846175826] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34847496057] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[34855212645] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34856335965] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34857438264] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34858621875] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=237 pred=0 subj_lo=0
[34864548114] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34865579991] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34866626487] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34867851678] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=453 pred=0 subj_lo=0
[34875387096] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34876427190] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34877542227] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34878746199] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=452 pred=0 subj_lo=0
[34887309600] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[34888450938] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[34889573895] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34890763083] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[34904217282] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[35863100526] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1078 watches=24 history=1024 journal=1024 symbols=454 drops=0
[37498456479] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1104 watches=24 history=1024 journal=1024 symbols=454 drops=0
[39076058241] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=1135 watches=24 history=1024 journal=1024 symbols=454 drops=0
[40637249037] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=1161 watches=24 history=1024 journal=1024 symbols=454 drops=0
[42266811411] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=16000 nodes=1187 watches=24 history=1024 journal=1024 symbols=454 drops=0
[42821851149] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=3453 ops=1 watches=24
[43291773525] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[44115862758] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=17000 nodes=1226 watches=24 history=1024 journal=1024 symbols=454 drops=0
[45736269051] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=18000 nodes=1250 watches=24 history=1024 journal=1024 symbols=454 drops=0
[47302865115] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=19000 nodes=1272 watches=24 history=1024 journal=1024 symbols=454 drops=0
[49141172982] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=20000 nodes=1303 watches=24 history=1024 journal=1024 symbols=454 drops=0
[50959355040] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=21000 nodes=1331 watches=24 history=1024 journal=1024 symbols=454 drops=0
[52567737717] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=22000 nodes=1359 watches=24 history=1024 journal=1024 symbols=454 drops=0
[54358829052] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=23000 nodes=1394 watches=24 history=1024 journal=1024 symbols=454 drops=0
[55774116816] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[55826913879] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=1
[56134502919] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11198464)
[56136519450] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=2 (11182080b)
[56140652172] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[56144468325] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[56520576453] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=24000 nodes=1428 watches=24 history=1024 journal=1024 symbols=454 drops=0
[58143689538] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=25000 nodes=1454 watches=24 history=1024 journal=1024 symbols=454 drops=0
[59883973281] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=26000 nodes=1491 watches=24 history=1024 journal=1024 symbols=454 drops=0
[61725687903] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=27000 nodes=1519 watches=24 history=1024 journal=1024 symbols=454 drops=0
[63396063225] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=28000 nodes=1543 watches=24 history=1024 journal=1024 symbols=454 drops=0
[65229705651] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=29000 nodes=1573 watches=24 history=1024 journal=1024 symbols=454 drops=0
[66403560300] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=4477 ops=1 watches=24
[67087311819] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=30000 nodes=1608 watches=24 history=1024 journal=1024 symbols=454 drops=0
[68956321137] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=31000 nodes=1634 watches=24 history=1024 journal=1024 symbols=454 drops=0
[70979603640] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=32000 nodes=1664 watches=24 history=1024 journal=1024 symbols=454 drops=0
[73058842560] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=33000 nodes=1701 watches=24 history=1024 journal=1024 symbols=454 drops=0
[75009098109] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=34000 nodes=1727 watches=24 history=1024 journal=1024 symbols=454 drops=0
[76585081551] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=35000 nodes=1751 watches=24 history=1024 journal=1024 symbols=454 drops=0
[79071484239] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=36000 nodes=1798 watches=24 history=1024 journal=1024 symbols=454 drops=0
[81205785474] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=37000 nodes=1828 watches=24 history=1024 journal=1024 symbols=454 drops=0
[82982618763] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=38000 nodes=1852 watches=24 history=1024 journal=1024 symbols=454 drops=0
[84715665672] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=39000 nodes=1883 watches=24 history=1024 journal=1024 symbols=454 drops=0
[87008941173] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=40000 nodes=1909 watches=24 history=1024 journal=1024 symbols=454 drops=0
[90505544712] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=41000 nodes=1943 watches=24 history=1024 journal=1024 symbols=454 drops=0
[92465420553] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=42000 nodes=1965 watches=24 history=1024 journal=1024 symbols=454 drops=0
[94628525706] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=43000 nodes=2004 watches=24 history=1024 journal=1024 symbols=454 drops=0
[94957460598] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=5501 ops=1 watches=24
[96447223191] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=44000 nodes=2028 watches=24 history=1024 journal=1024 symbols=454 drops=0
[98202257934] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=45000 nodes=2052 watches=24 history=1024 journal=1024 symbols=454 drops=0
[100111345587] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=46000 nodes=2085 watches=24 history=1024 journal=1024 symbols=454 drops=0
[102170066361] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=47000 nodes=2111 watches=24 history=1024 journal=1024 symbols=454 drops=0
[104009069868] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=48000 nodes=2137 watches=24 history=1024 journal=1024 symbols=454 drops=0
[106000895055] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=49000 nodes=2163 watches=24 history=1024 journal=1024 symbols=454 drops=0
[107825270157] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=50000 nodes=2196 watches=24 history=1024 journal=1024 symbols=454 drops=0
[109783616448] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=51000 nodes=2222 watches=24 history=1024 journal=1024 symbols=454 drops=0
[111536911926] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=52000 nodes=2244 watches=24 history=1024 journal=1024 symbols=454 drops=0
[113386334028] [INFO] [ker
```
</details>
