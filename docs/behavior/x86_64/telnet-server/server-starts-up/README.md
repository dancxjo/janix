# ✅ Scenario: Server starts up

> Last run: 2026-04-05 19:04:08

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 3951ms | - [📜](./01/serial.log) - |
| 2 | Then I should see a message in the serial output that says "telnetd: listening on guest port 2323" | ✅ | 3136ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12474639606] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[12480103317] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[12483708435] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[12485811030] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[12487031568] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[12487660317] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[12488310846] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[12488885574] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[12489547059] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=29168
[12490180296] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=33264
[12490790730] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[12491616324] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[12492505938] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[12493196991] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[12493884315] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[12494492736] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[12495115545] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[12495854283] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[12496610148] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[12497336874] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[12498043437] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[12498633477] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[12499224177] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[12499840716] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[12500472435] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[12501072342] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[12501820749] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[12502565460] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[12503425803] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[12504057390] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[12504671223] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[12505290897] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[12505947663] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[12506582880] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=172560
[12507183447] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[12508044879] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[12508862058] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[12509857107] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[12510565749] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[12511293003] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[12512001414] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[12512713323] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[12514224393] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[12516101829] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[12516978144] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[12517529607] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[12518039325] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[12518557920] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[12519099945] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[12519641046] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[12520153404] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[12520670481] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[12521343087] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[12522041499] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7795e000 (Usable)
[12522796242] [INFO] [kernel::memory] [CPU0]   [11] 0x7795e000 - 0x779c2000 (Reserved)
[12523359882] [INFO] [kernel::memory] [CPU0]   [12] 0x779c2000 - 0x779c3000 (Other)
[12523897848] [INFO] [kernel::memory] [CPU0]   [13] 0x779c3000 - 0x779c4000 (Reserved)
[12524453502] [INFO] [kernel::memory] [CPU0]   [14] 0x779c4000 - 0x779c5000 (Other)
[12524992557] [INFO] [kernel::memory] [CPU0]   [15] 0x779c5000 - 0x779c6000 (Reserved)
[12525547386] [INFO] [kernel::memory] [CPU0]   [16] 0x779c6000 - 0x779c7000 (Other)
[12526242300] [INFO] [kernel::memory] [CPU0]   [17] 0x779c7000 - 0x779c8000 (Reserved)
[12526911441] [INFO] [kernel::memory] [CPU0]   [18] 0x779c8000 - 0x77a53000 (Other)
[12527694234] [INFO] [kernel::memory] [CPU0]   [19] 0x77a53000 - 0x77a54000 (Reserved)
[12528261636] [INFO] [kernel::memory] [CPU0]   [20] 0x77a54000 - 0x77ed5000 (Other)
[12528806004] [INFO] [kernel::memory] [CPU0]   [21] 0x77ed5000 - 0x77ed6000 (Reserved)
[12529386771] [INFO] [kernel::memory] [CPU0]   [22] 0x77ed6000 - 0x77ff7000 (Other)
[12529929522] [INFO] [kernel::memory] [CPU0]   [23] 0x77ff7000 - 0x77ff8000 (Reserved)
[12530486232] [INFO] [kernel::memory] [CPU0]   [24] 0x77ff8000 - 0x787f8000 (Other)
[12531036639] [INFO] [kernel::memory] [CPU0]   [25] 0x787f8000 - 0x787f9000 (Reserved)
[12531728022] [INFO] [kernel::memory] [CPU0]   [26] 0x787f9000 - 0x788ba000 (Other)
[12532551999] [INFO] [kernel::memory] [CPU0]   [27] 0x788ba000 - 0x788bb000 (Reserved)
[12533172300] [INFO] [kernel::memory] [CPU0]   [28] 0x788bb000 - 0x788e6000 (Other)
[12533713665] [INFO] [kernel::memory] [CPU0]   [29] 0x788e6000 - 0x788e7000 (Reserved)
[12534273774] [INFO] [kernel::memory] [CPU0]   [30] 0x788e7000 - 0x788ec000 (Other)
[12534815436] [INFO] [kernel::memory] [CPU0]   [31] 0x788ec000 - 0x788ed000 (Reserved)
[12535375380] [INFO] [kernel::memory] [CPU0]   [32] 0x788ed000 - 0x788f2000 (Other)
[12535944663] [INFO] [kernel::memory] [CPU0]   [33] 0x788f2000 - 0x788f3000 (Reserved)
[12536694423] [INFO] [kernel::memory] [CPU0]   [34] 0x788f3000 - 0x7891f000 (Other)
[12537501306] [INFO] [kernel::memory] [CPU0]   [35] 0x7891f000 - 0x78921000 (Reserved)
[12538066794] [INFO] [kernel::memory] [CPU0]   [36] 0x78921000 - 0x7892a000 (Other)
[12538607697] [INFO] [kernel::memory] [CPU0]   [37] 0x7892a000 - 0x7892c000 (Reserved)
[12539167476] [INFO] [kernel::memory] [CPU0]   [38] 0x7892c000 - 0x78934000 (Other)
[12539728047] [INFO] [kernel::memory] [CPU0]   [39] 0x78934000 - 0x78935000 (Reserved)
[12540348249] [INFO] [kernel::memory] [CPU0]   [40] 0x78935000 - 0x7893f000 (Other)
[12540945648] [INFO] [kernel::memory] [CPU0]   [41] 0x7893f000 - 0x78940000 (Reserved)
[12541671087] [INFO] [kernel::memory] [CPU0]   [42] 0x78940000 - 0x7894d000 (Other)
[12542392236] [INFO] [kernel::memory] [CPU0]   [43] 0x7894d000 - 0x7894f000 (Reserved)
[12543115728] [INFO] [kernel::memory] [CPU0]   [44] 0x7894f000 - 0x7895d000 (Other)
[12543659502] [INFO] [kernel::memory] [CPU0]   [45] 0x7895d000 - 0x7895e000 (Reserved)
[12544254492] [INFO] [kernel::memory] [CPU0]   [46] 0x7895e000 - 0x7896a000 (Other)
[12544795593] [INFO] [kernel::memory] [CPU0]   [47] 0x7896a000 - 0x7896b000 (Reserved)
[12545351247] [INFO] [kernel::memory] [CPU0]   [48] 0x7896b000 - 0x7896f000 (Other)
[12545957754] [INFO] [kernel::memory] [CPU0]   [49] 0x7896f000 - 0x78970000 (Reserved)
[12546525750] [INFO] [kernel::memory] [CPU0]   [50] 0x78970000 - 0x78980000 (Other)
[12547066752] [INFO] [kernel::memory] [CPU0]   [51] 0x78980000 - 0x78981000 (Reserved)
[12547624650] [INFO] [kernel::memory] [CPU0]   [52] 0x78981000 - 0x78a11000 (Other)
[12548165685] [INFO] [kernel::memory] [CPU0]   [53] 0x78a11000 - 0x78a12000 (Reserved)
[12548776746] [INFO] [kernel::memory] [CPU0]   [54] 0x78a12000 - 0x78a1d000 (Other)
[12549389622] [INFO] [kernel::memory] [CPU0]   [55] 0x78a1d000 - 0x78a1e000 (Reserved)
[12550002498] [INFO] [kernel::memory] [CPU0]   [56] 0x78a1e000 - 0x78a43000 (Other)
[12550546338] [INFO] [kernel::memory] [CPU0]   [57] 0x78a43000 - 0x78a44000 (Reserved)
[12551106678] [INFO] [kernel::memory] [CPU0]   [58] 0x78a44000 - 0x78a50000 (Other)
[12551647614] [INFO] [kernel::memory] [CPU0]   [59] 0x78a50000 - 0x78a51000 (Reserved)
[12552207129] [INFO] [kernel::memory] [CPU0]   [60] 0x78a51000 - 0x78a5e000 (Other)
[12552769053] [INFO] [kernel::memory] [CPU0]   [61] 0x78a5e000 - 0x78a5f000 (Reserved)
[12553328106] [INFO] [kernel::memory] [CPU0]   [62] 0x78a5f000 - 0x78aa7000 (Other)
[12553869471] [INFO] [kernel::memory] [CPU0]   [63] 0x78aa7000 - 0x78aa8000 (Reserved)
[12554710509] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12795208569] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485750 free frames
[12807040521] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12811841790] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12813115887] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12813954714] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12818284974] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12820054896] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12821123337] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12821791686] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12822467460] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12823129572] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12824092281] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12825050766] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12825727827] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12826412115] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12827077131] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12827742840] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12828899490] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12829935822] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12830627832] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12832158867] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12833114316] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12834095472] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12835728807] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12837274428] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12838028742] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12838566015] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12839336235] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[13240456449] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[13241521689] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[13245237522] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[13246160895] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[13246950453] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[13248559929] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[13261786362] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[13263211896] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[13264030296] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[13265945451] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[13266511797] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[13269259971] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[13277634744] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[13279501191] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[13293703632] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[13294330533] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[13311578643] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[13312234848] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[13314357804] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[13315766112] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[13317131949] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[13319464785] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[13320330639] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[13355337501] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62450100 ticks/sec), init_cnt=624501 for 100Hz
[13356921303] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[13357807980] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[13359050100] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[13365319341] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[13398946671] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[13400021217] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[13401285645] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[13402543011] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[13403706261] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[13406744703] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[13408018140] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[13429479162] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[13431222486] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[13432070520] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[13432852620] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[13433807343] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[13434535257] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[13435230204] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[13461189687] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[13462468932] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[13463719137] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[13465067385] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[13466249082] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[13467461370] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[13468530669] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[13469532648] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[13476314544] [INFO] [kernel::root] [CPU0] Spawning Root service...
[13477310682] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[13479326223] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[13480155051] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[13485595530] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[13488061488] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[13489379805] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[13491415608] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[13492957566] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[13494590208] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[13507603164] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[13511299065] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[13513201185] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[13514559234] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13536061869] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13555987533] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13558251498] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13562161965] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13564870869] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13568943366] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13572778956] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13575914748] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13576886136] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13578036615] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13588060596] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13590743529] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13595389566] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13598966370] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13602578121] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13606512381] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13607426976] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13620975225] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13622423562] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13630174998] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13631564364] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13659239451] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13660674027] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[14021628885] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[14582237208] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[14613195399] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[14647766100] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[16005430188] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=446 watches=0 history=960 journal=768 symbols=94 drops=0
[16909521420] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[17008811457] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[17010057405] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[17115397959] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[17180904345] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[17214172998] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[17216187417] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[17217512103] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[17223315120] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[17243915205] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[17261891823] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[17265270693] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[17320645353] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[17343345327] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[17344136733] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[17348793297] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[17376415221] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[17396002899] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[17400329793] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[17401429617] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[17480534181] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[17482284996] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[17660222118] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[17733578643] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[17747806461] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[17752235589] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[17754465630] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[17769301473] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[17825762559] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[17830862808] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[17831977779] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[17832872310] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[17833757700] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[17834429448] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[17835085389] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[17835753309] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[17836354338] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[17836965036] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=29168
[17837602827] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=33264
[17838221214] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[17839254081] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[17840089773] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[17840784753] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[17841535866] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[17842235202] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[17843585925] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[17845462668] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[17846631594] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[17847459366] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[17848126494] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[17848741680] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[17849434944] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[17850286344] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[17851207044] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[17851997493] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[17852663961] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[17853342111] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[17853961950] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[17854595913] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[17855342901] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[17856035802] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[17856727515] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[17857372467] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=172560
[17857995375] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[17858771733] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[17859523110] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[17860273002] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[17861007252] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[17861806578] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[17862603759] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[17863363188] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[17864992629] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[17866806144] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[17868072684] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17877872298] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[17894479482] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[17899658436] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[17909900745] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[17911066074] [CONTRACT] [kernel] [CPU0] Spawning init process...
[17913508173] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[17916233181] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[17917039041] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
ThingOS Petals
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
type 'help' for commands

petals> [17924068107] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[17924869842] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[17940539232] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[17947566417] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[17948817282] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[17949877671] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[17958897924] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[17964509145] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[17967992130] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[17969098917] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[17972937015] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[17976210714] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[17977333572] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[17981628423] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[17982962580] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[17984321355] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[17985733755] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[17988769722] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[17990057415] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[17991588054] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[17993258778] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[17994658341] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[17996118096] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[18001312197] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[18044094552] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[18051911757] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[18057270990] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[18063366717] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[18068621109] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[18075081684] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[18082043133] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[18087787443] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[18093503142] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[18099353085] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[18104808810] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[18110559357] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[18116874831] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[18121652472] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[18126557493] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[18131034636] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[18136606785] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[18141180123] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[18145752240] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[18150474507] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[18154979106] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[18159687183] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[18164439909] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[18170164848] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[18176261235] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[18181968057] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[18187235814] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[18192327351] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[18196997379] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[18202431588] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[18205722216] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[18209324100] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[18214220211] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[18217815165] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/telnetd
[18221090547] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[18227766084] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[18232891578] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[18238041162] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[18243063267] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[18248570637] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[18253764771] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[18259702659] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[18263396250] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[18284949672] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[18433616388] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[18440712543] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[18441547575] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18443435505] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18448127280] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18449482095] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18457292337] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[18461813403] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18463187688] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18464620482] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[18470036673] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[18472773990] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[18473655024] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18475410921] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18479653995] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18481273239] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18488196441] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[18491211519] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[18492019524] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[18493754268] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[18494753046] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[18502151085] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[18504062280] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[18506325519] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18511033134] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 02:04:27 = 1775441067 unix_secs
[18512682870] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775441067, mono_ns=9256119213, offset=1775441057743880787ns
[18514327425] [INFO] [rtc_cmos] [CPU1] System clock anchored
[18524362725] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[18564167424] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[18573915723] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[18574889025] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18576617895] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18582655905] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18585538455] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[18593238642] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[18596507061] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[18599960742] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18602492469] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[18608973273] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[18613975479] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[18616010523] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[18616955313] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18619043817] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18626224287] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18628490463] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18635843853] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[18640191306] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[18642301029] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[18643369041] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18645508827] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[18649494204] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[18652743549] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[18656709951] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[18658154427] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[18662963649] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[18664648992] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[18666316779] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[18673030497] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[18674810385] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[19209832950] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19223806338] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[19225416111] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19231396041] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19244052663] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19247799054] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19277125527] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[19281867363] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
[19283147631] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fab68
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[19285142151] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352560 RFLAGS_BEFORE=130 CR3_BEFORE=59662336 fs_base=0 gs_base=18446744071564586576
[19294666479] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[19298716833] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[19301055972] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[19306524501] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[19307925483] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[19310187072] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[19313163672] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[19315299399] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[19316551584] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[19320175446] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[19322363478] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[19338579777] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[19342066260] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[19343557629] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[19344936039] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[19346435361] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[19347702165] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[19372553541] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[19376693787] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[19380277785] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=f=0x3000 mult=4
[19384675563] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19387284081] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[19389693345] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19395929850] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[19401284826] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[19402787547] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19415413974] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[19417925505] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[19423066773] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[19427503095] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[19430859195] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[19431957732] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19434404979] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19441403586] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19442904096] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[19444660026] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19460255067] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[19463481642] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[19465124679] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[19467137943] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500208 RFLAGS_BEFORE=134 CR3_BEFORE=59944960 fs_base=0 gs_base=18446744071564586640
[19472255253] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[19473157506] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19474548753] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[19475350719] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19476306993] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[19479939336] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[19481187528] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19482220329] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[19483809642] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[19484740077] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19486915668] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[19488978960] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[19490981565] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[19493325423] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[19495231998] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[19496633442] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[19498840977] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[19501221465] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[19503698544] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[19505629968] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[19510513704] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1236 port=2 model='                                        ' rpc_port=5
[19539376329] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[19540301946] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[19541064807] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[19541850603] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[19542639072] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[19543393419] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[19544248845] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[19545224424] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[19551252402] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0103240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[19553033148] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434128 RFLAGS_BEFORE=130 CR3_BEFORE=59805696 fs_base=0 gs_base=18446744071564586608
[19559306316] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[19897233246] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[19899257631] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369582992 RFLAGS_BEFORE=134 CR3_BEFORE=68354048 fs_base=0 gs_base=18446744071564586576
[19904973231] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[19905942012] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[19907885679] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[19909165914] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[19910220792] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[19912370676] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[19917050571] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19919602956] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[19920381888] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19921950840] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19924485867] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[19928717853] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[19931238954] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[19938367449] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[19940946333] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[19942602537] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[19944219999] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[19945141425] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19946738295] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19966069035] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19970574261] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[19987680438] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[20846101749] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[20849426697] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[20850817086] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20852747322] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716816 RFLAGS_BEFORE=134 CR3_BEFORE=68890624 fs_base=0 gs_base=18446744071564586640
[20859116421] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0103240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[20861608845] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[20862480705] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650480 RFLAGS_BEFORE=134 CR3_BEFORE=68747264 fs_base=0 gs_base=18446744071564586608
[20866750311] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[20887686633] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[20889647130] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[20892874695] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering interrupt-driven loop
[20897527200] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[20898927192] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[20900101035] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[20901683385] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[20904058791] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[20905781325] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[20908227219] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[20909263221] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20911668426] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20935973586] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[20937021105] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[20938667838] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[20940804489] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[20984381616] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[21008834880] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[21016416135] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[21019353894] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0103240
[21020720853] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21022016136] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369782832 RFLAGS_BEFORE=134 CR3_BEFORE=79896576 fs_base=0 gs_base=18446744071564586576
[21025656564] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[21026729328] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21029010882] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[21029769915] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21031934517] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[21038464590] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21040239000] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21056647689] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[21060295773] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[21068067966] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[21070907418] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[21072943518] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[21073677867] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21075110595] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21080869227] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[21083360760] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21093175620] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[21096554259] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010aff8
[21097600854] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21098982597] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915216 RFLAGS_BEFORE=134 CR3_BEFORE=80830464 fs_base=0 gs_base=18446744071564586640
[21104576658] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[21106176993] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[21113243382] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0103240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21115028781] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849136 RFLAGS_BEFORE=134 CR3_BEFORE=80568320 fs_base=0 gs_base=18446744071564586608
[21120157443] [INFO] [nectar] [CPU2] NECTAR: Started.
[21123754608] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[21124635675] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21126538752] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21134255802] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21138204219] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[21146019642] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[21149374719] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0103240
[21150889683] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21152714253] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369982784 RFLAGS_BEFORE=130 CR3_BEFORE=80957440 fs_base=0 gs_base=18446744071564586576
[21156979173] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[21157831398] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21159762921] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21169508943] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[21173259756] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[21180779499] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[21183782631] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[21185959542] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[21186796323] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21188365209] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21214871700] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[21220152855] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[21227546142] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[21230994411] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010c048
[21232315038] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21233601807] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370114928 RFLAGS_BEFORE=130 CR3_BEFORE=81260544 fs_base=0 gs_base=18446744071564586640
[21236673051] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[21237396477] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21238799043] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[21239713440] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21257224659] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[21259041804] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[21262556502] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21264079782] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[21264997083] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21266563824] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21268907715] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=226 pred=0 subj_lo=0
[21274028193] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[21277103100] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[21279411813] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[21280567803] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21282427221] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21286054845] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21288266010] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370199312 RFLAGS_BEFORE=130 CR3_BEFORE=81534976 fs_base=0 gs_base=18446744071564586576
[21294389127] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[21295498488] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[21299453736] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21301139013] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21303015459] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21304372782] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21306041031] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[21307612161] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21309354627] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21311180517] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[21331153602] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[21341735844] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[21347334723] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[21349033728] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21350253408] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21351362571] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[21353252184] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21354683295] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[21361213995] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[21362900625] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[21364202178] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[21369225372] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[21374383668] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[21376341327] [INFO] [fontd] [CPU3] FONTD: Service ready
[21377737788] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0103240
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21379342446] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370049136 RFLAGS_BEFORE=130 CR3_BEFORE=81104896 fs_base=0 gs_base=18446744071564586608
[21384691482] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[21386866149] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010d500
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21388579575] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370268128 RFLAGS_BEFORE=130 CR3_BEFORE=81743872 fs_base=0 gs_base=18446744071564586608
[21391922244] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21393440904] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21394956231] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21396227556] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=234 subj_lo=0
[21397679028] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[21399913722] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[21402153894] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[21418574595] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[21424734540] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[21433899333] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21436215702] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21444076170] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21445225032] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21446559288] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21447814311] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=239 subj_lo=0
[21458835222] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21460239471] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21461349987] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21462674838] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[21468409545] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[21469998561] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f4000
[21472124025] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f4000
[21473873850] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f7000
[21476294697] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276783104)
[21477632682] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[21480921924] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f8000 phys=0x4ed5000
[21482310993] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21483531003] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[21484405635] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21486094542] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21487608120] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[21488693523] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[21490821726] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[21492820173] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[21501734133] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[21503117427] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[21504046542] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[21504957705] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[21507998391] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[21519930432] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[21530975994] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[21534459540] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[21536912595] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[21539046012] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[21540556257] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[21542064720] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[21544016637] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[21546003303] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[21547660497] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[21553493742] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=19, read=20)
[21557749224] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[21560339724] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=21, read=22)
[21565891149] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[21573113397] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([234, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21582388179] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1252 backend=VirtIO-GPU
[21584400453] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[21585572481] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21587670357] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21600772380] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=23, resp=26
[21602661531] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[21617709036] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([234, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21621400581] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21622478328] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21667726872] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([234, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21758005599] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[21779076132] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[21790888251] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[21794513763] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[21795839670] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db5d0
[21797262993] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[21798377238] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e4
[21801556656] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21802704231] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370504528 RFLAGS_BEFORE=134 CR3_BEFORE=82776064 fs_base=0 gs_base=18446744071564586640
[21808926315] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21812550111] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[21814306041] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[21829505247] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[21832798086] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db5d0
[21833907051] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[21835721655] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370570064 RFLAGS_BEFORE=134 CR3_BEFORE=83824640 fs_base=0 gs_base=18446744071564586576
[21839202891] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[21841509327] [INFO] [echo] [CPU1] echo: starting up
[21843834375] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[21847321551] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[21864011895] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[21865509204] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[21866782278] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[21868041558] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[21872689344] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=19, RX port=22
[21879560307] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([234, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21891667248] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[21893471325] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[21899387235] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[21901145904] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[21906579948] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[21912939081] [INFO] [netd] [CPU3] NETD: Driver TX port=19, RX port=22, link_up=true, mtu=1500
[21915168000] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[21919410117] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[21926569005] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[21928241346] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[21930207783] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[21939885297] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[21941838765] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[21943528068] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[21944325513] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21945984522] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21948878127] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[21951985836] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21954486906] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[21959247486] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[21964262265] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[21967884411] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[21970440756] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[21972864309] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[21974930736] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[21975792597] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21977660595] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21985734408] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21989953227] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[21997736838] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[21998845473] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[22002003936] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[22002939519] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[22003944072] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[22005631791] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[22006484676] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22008003369] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[22009060722] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22010204271] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([236, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[22012952940] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[22016276370] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f2a98
[22017522879] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22019537265] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370742096 RFLAGS_BEFORE=134 CR3_BEFORE=84336640 fs_base=0 gs_base=18446744071564586640
[22029110499] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[22032963612] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[22035116499] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/telnetd'
[22037657004] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/telnetd
[22038746895] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22041060525] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22049330688] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f2a98
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22051809285] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370807632 RFLAGS_BEFORE=134 CR3_BEFORE=84500480 fs_base=0 gs_base=18446744071564586576
[22058944776] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[22060210128] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[22068204675] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[22075354224] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=221000 exec=false
[22076579019] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[22085051076] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=229000 exec=false
[22097157588] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 32 (user task/process) assigned to CPU 2
[22101455838] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=32)
[22103403498] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[22109243442] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[22111951818] [INFO] [anther] [CPU1] anther: Connected to network stack
[22112965941] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[22118388501] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[22121437668] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[22125088623] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f2a98
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22127428653] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=32 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370873168 RFLAGS_BEFORE=134 CR3_BEFORE=84602880 fs_base=0 gs_base=18446744071564586608
[22136495601] [INFO] [telnetd] [CPU2] telnetd: starting on port 2323
[22142191137] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[22166255496] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[22172403693] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[22182450543] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[22186233498] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[22196683278] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[22198841940] [INFO] [bloom] [CPU3] bloom: creating surface...
[22200941268] [INFO] [bloom] [CPU3] bloom: surface created!
[22202017629] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[22204758774] [INFO] [telnetd::net_client] [CPU2] telnetd: connected to socket API (netd=27, write=31, read=32)
[22209236445] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[22215070350] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1264
[22216714113] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[22230453102] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[22234830981] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=33)
[22236380991] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[22237707360] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:08C0 [22244184468] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[22253438856] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[22262499633] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[22271993931] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[22278765828] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[22285994973] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[22289112318] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
T:0E20 T:0C90 T:F700 [22303017594] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1264
[22320072192] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=77c80b059e863710)
[22328160063] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[22331752971] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[22333404885] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[22335725214] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[22339699668] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 37 (user thread) assigned to CPU 3
[22343090121] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=37 (priority=2)
[22348457406] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
T:1870 [22356171816] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[22358994174] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[22365279981] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[22367484843] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[22377550998] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=277
[22379260926] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[22381128792] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22382271120] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22383496080] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22384921218] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=277 subj_lo=0
[22397556060] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1269)
[22399458213] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[22412352237] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=244
[22413692070] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[22415316165] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22416861291] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22418673981] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22420676982] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[22437952944] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1276)
[22440726792] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[22451105226] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[22455678993] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[22466689641] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[22472216448] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[22474930335] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[22481626992] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=279
[22483485816] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[22485299694] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[22486762122] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22488353712] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22489933719] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=279 subj_lo=0
[22513960458] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1277)
[22516153407] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[22548422952] [INFO] [netd] [CPU3] NETD: DHCP configured ��� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[22551542772] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[22556449443] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[22560598698] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[22563657171] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 2323 with backlog 64
[22566379044] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 2323, handle=2
[22568202855] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[22570237272] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=3
[22573868559] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[22574767875] [INFO] [telnetd] [CPU2] telnetd: listening on guest port 2323 (handle=2)
[22607994684] [INFO] [netd::socket_api] [CPU3] SOCKET_AP
```
</details>
