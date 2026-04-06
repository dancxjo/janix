# ❌ Scenario: Input Subsystem Ready

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 5162ms | - - - |
| 2 | When I wait for the system to reach ready state | ✅ | 0ms | - - - |
| 3 | Then I should see a cursor centered on the screen | ❌ | 1011ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11436456075] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11441970342] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11445590079] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11447622417] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11448827775] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11449458405] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11450149260] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11450733855] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11451319770] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11451928257] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11452518165] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11453118963] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11453838825] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11454498330] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11455182651] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11455788927] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11456404938] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11457012237] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11457627621] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11458224525] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11458801134] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11459381340] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11459984349] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11460574983] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11461206636] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11461808061] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11462404965] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11463089979] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11463682329] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11464284876] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11464895739] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11465514951] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11466144393] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11466785913] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11467381827] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11468080602] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11468788650] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11469505047] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11470216791] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11470943517] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11471651763] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11472367929] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11473665951] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11475180684] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11476001526] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11476569291] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11477085378] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11477608659] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11478158769] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11478682644] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11479200744] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11479736994] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11480253312] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11480792829] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11481337857] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11481902751] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11482448802] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11483023827] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11483568624] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11484132462] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11484677490] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11485240602] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11485786257] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11486360226] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11486905749] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11487469455] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11488012371] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11488752396] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11489301450] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11489919441] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11490469386] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11491037514] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11491586337] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11492176575] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11492744109] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11493315306] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11493882114] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11494488060] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11495055429] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11495623359] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11496185250] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11496753774] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11497304577] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11497871583] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11498492379] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11499064533] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11499633453] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11500202703] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11500751856] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11501312295] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11501879697] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11502442908] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11503018890] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11503582068] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11504126139] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11504689119] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11505235401] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11505800130] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11506359315] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11506924275] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11507468544] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11508032613] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11508575265] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11509137024] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11509693173] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11510254404] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11510797815] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11511621957] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11747097846] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[11758553664] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11763422847] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11764650678] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11765491551] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11769641730] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11771339580] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11772407394] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11773112142] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11773776036] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11774440029] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11775421251] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11776404090] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11777090688] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11777765538] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11778436956] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11779108143] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11780327196] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11781336534] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11782034847] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11783655378] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11784591126] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11785582215] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11787269835] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11788880301] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11789668638] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11790204030] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11790969333] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12156665565] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12157649031] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12160986519] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12161887848] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12162656352] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12164165409] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12176450088] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12177793518] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12178582317] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12180342900] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12180896706] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12183387546] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12191464758] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12193224846] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12206293671] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12206924235] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12223586529] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12224266956] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12226371432] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12227666979] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12228792741] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12231120726] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12231980607] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12267122934] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62500400 ticks/sec), init_cnt=625004 for 100Hz
[12268761648] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12269594172] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12270822597] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12277186152] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12307672938] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12308671320] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12310874235] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12312144306] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12313247661] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12315932409] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12317194989] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12337498206] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12339602385] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12340783587] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12342012342] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12342966042] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12345164733] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12348051771] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12372334359] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12373558428] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12374691846] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12375999042] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12377198691] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12378304323] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12379507470] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12380471763] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12387225741] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12388225575] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12390116739] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12390975465] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12396543951] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12398176626] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12399097557] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12400601367] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12401649777] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12402549984] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12414664152] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12417611481] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12418837596] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12419591283] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12443096589] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12462377367] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12465200385] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12468526422] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12470239815] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12472558098] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12474812559] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12476774409] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12477582678] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12478637391] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12485164065] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12486785916] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12489328962] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12491718195] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12493944573] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12496427955] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12497208009] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12512223570] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12513035964] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12520629429] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12521431626] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12550481295] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12551288640] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[12898555923] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13361153817] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13385458614] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=497 journal=422 symbols=51 drops=0
[13424257011] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[14738250351] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=446 watches=0 history=959 journal=768 symbols=93 drops=0
[15507389832] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15608648517] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15609942249] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[15726562665] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[15788139312] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[15819158322] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[15820003584] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[15820640616] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[15824091723] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[15842491170] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[15857877288] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[15860186232] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[15911097015] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[15931255065] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[15932012844] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[15936279645] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[15965143788] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[15985845282] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[15989246988] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[15990213855] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[16055694534] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[16057669716] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[16151081364] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[16216048134] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[16230170847] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[16244192448] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[16246651047] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[16254369978] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[16310398368] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[16315128621] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[16316094201] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[16316920488] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[16317840330] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[16318585899] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[16319237550] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[16319890290] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[16320503265] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[16321173198] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[16321861941] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[16322492175] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[16323342981] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[16324122837] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[16324927674] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[16326000372] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[16326705351] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[16327380696] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[16328056437] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[16328716668] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[16329462666] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[16330179921] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[16330803423] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[16331525562] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[16332176454] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[16332867936] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[16333509390] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[16334145333] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[16334874105] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[16335504273] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[16336145166] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[16336793682] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[16337447544] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[16338124473] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[16338776487] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[16339402860] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[16340142093] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[16340890599] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[16341698538] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[16342446285] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[16343219640] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[16343973921] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[16344797832] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[16346282337] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[16348070079] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[16348881681] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16357219890] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[16372170705] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[16376719359] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[16386036777] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[16386753372] [CONTRACT] [kernel] [CPU0] Spawning init process...
[16389036477] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[16393648854] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[16394756334] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [16402135365] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[16403239677] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[16429058844] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[16432265256] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[16433536218] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[16434732534] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[16444204689] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[16450060737] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[16453395321] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[16454444688] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[16458343506] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[16461542361] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[16462621395] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[16466525031] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[16468136091] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[16469360094] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[16470611751] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16473772260] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16474876407] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16476038007] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16477477830] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16478849244] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16480287021] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16485088092] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[16526767356] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[16534139655] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[16538697846] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[16544189046] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[16547618109] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[16551484257] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[16556562198] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[16561780323] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[16566529155] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[16572870765] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[16578221220] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[16583731659] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[16589727231] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[16594193517] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[16599933867] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[16605245085] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[16610489940] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[16615159836] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[16619910021] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[16624684956] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[16629512988] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[16634885751] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[16639805127] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[16644695463] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[16649501253] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[16655425182] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[16660647465] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[16665474771] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[16669827702] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[16675605540] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[16678899303] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[16682585832] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[16687956945] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[16693224273] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[16697839785] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[16702772493] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[16707803145] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[16713217521] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[16719679911] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[16724765904] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[16731536415] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[16734910632] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[16755940740] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[16889690268] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[16899179022] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[16900320426] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16902894459] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16909434597] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[16911332031] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16919974566] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[16925425176] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[16927673961] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16929464442] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[16935555681] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[16937365071] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[16938821955] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16941590820] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16946317707] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[16948492011] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16957901631] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[16962910173] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[16964164767] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[16966408734] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[16970676459] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[16974896367] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[16976466507] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[16978502673] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[16982958465] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:02:28 = 1775437348 unix_secs
[16984680372] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775437348, mono_ns=8492116396, offset=1775437339507883604ns
[16986482469] [INFO] [rtc_cmos] [CPU1] System clock anchored
[16999044414] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[17041809279] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[17051525007] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[17052759504] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17055302715] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17062799688] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[17065509879] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[17073247422] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[17075787696] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[17078694270] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17080374036] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[17085432045] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[17089191966] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[17091086529] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[17091903906] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17094002574] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17101136250] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17103641676] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17110388130] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[17112885570] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[17114198904] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17115796335] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[17121876321] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[17123970699] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[17127959211] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[17130060255] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[17132275248] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[17134530798] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[17135706852] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[17137251252] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[17152992417] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[17154622782] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[17672693940] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17678420760] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[17681234241] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[17682826854] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[17686929414] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[17688423720] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[17690387649] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[17691798498] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[17692766322] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[17695083978] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[17696370846] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[17697592836] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[17698463013] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[17699235411] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[17699997183] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[17701090539] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[17705724762] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[17706656913] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17708514582] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17715387954] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17717792400] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17725446552] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[17728026063] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fbff0
[17729132652] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[17730967353] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352640 RFLAGS_BEFORE=134 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[17737914051] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[17746310340] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[17776830192] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[17783671752] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[17786702076] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[17793001380] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[17802194916] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[17804502903] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[17808749937] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[17810283513] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[17811169398] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[17812177086] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[17813019675] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[17814224571] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[17815085937] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[17816087487] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[17819081445] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[17822291256] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[17824705899] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[17827085463] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[17831194227] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[17832149445] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17833744731] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17837571477] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[17839381395] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17846676111] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[17849207574] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[17850957993] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[17851701054] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17853139128] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17857655805] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17859282870] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17866028235] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[17868348399] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
[17869287018] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[17871598800] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500336 RFLAGS_BEFORE=130 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[17875598928] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[17876498574] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17878163490] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17880146625] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[17882203317] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[17883449463] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17885872785] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17888169552] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[17889858096] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[17891251158] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[17893410381] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[17894769849] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[17895865053] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[17897184987] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[17898382887] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[17899603260] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[17900946789] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[17901703380] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[17903559696] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[17904722946] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[17906548374] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434256 RFLAGS_BEFORE=134 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[17909789271] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[17912141082] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[18289878915] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[18291507993] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
[18292678734] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[18294477201] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583120 RFLAGS_BEFORE=130 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[18297524355] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[18300462741] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[18302425416] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18303619521] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[18305067132] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[18305951268] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18308014791] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18308877048] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[18314815101] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[18317343528] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[18322287291] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[18324496080] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[18327612369] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[18329954280] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[18331349289] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[18332569068] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18334610151] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18354315177] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18358896336] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[18379250967] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[18382540671] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[18384342867] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18386202516] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369719072 RFLAGS_BEFORE=130 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[18390949797] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[18391935045] [INFO] [netd] [CPU3] NETD: Starting network stack service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[18393223761] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653536 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[18396141060] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[18398246130] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[18400400964] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[18411982050] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[18414983070] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[18418798167] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[18420294156] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[18421544922] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[18422553369] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[18423918117] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[18425687082] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[18427370379] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[18428859273] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[18429699354] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18431609427] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18489075264] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[18490331178] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[18492407472] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18494420868] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[18499882203] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[18524224653] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[18531259758] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[18534084591] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[18535836924] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18537153096] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369784608 RFLAGS_BEFORE=130 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[18540139497] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[18540839559] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18542300667] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18543063891] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[18545644194] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[18549689070] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18551011314] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18567578832] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18570707034] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[18578065473] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[18580689237] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[18582632673] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[18583446816] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18584872317] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18590185053] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18592477101] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18599478084] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[18602241834] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010aff8
[18603902526] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18605243217] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915680 RFLAGS_BEFORE=130 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[18608391417] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18609210543] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18610691352] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[18611530443] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18612447975] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[18619250694] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18622798557] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18630895734] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[18634287243] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010aff8
[18635530848] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18637100394] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981696 RFLAGS_BEFORE=134 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[18640190415] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[18640941561] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18642422040] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18651078996] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18655121595] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[18662720604] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[18666058488] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[18668271864] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[18668967603] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18670424652] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18695254380] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18699992619] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[18707022213] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[18709883709] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010ca80
[18711087813] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18712827705] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113952 RFLAGS_BEFORE=134 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[18715826415] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[18716593038] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18717747345] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[18718601583] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18733782144] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[18735592623] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[18738595755] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18739824543] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18742782696] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18744285879] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18745964919] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[18749209314] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[18751923498] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[18753527298] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18754842348] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370198240 RFLAGS_BEFORE=130 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[18758115684] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[18759030048] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18760469838] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[18761439246] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18762203988] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[18771705183] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18772743858] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18773921199] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18775347954] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=229 subj_lo=0
[18778722930] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18780367485] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18782187204] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18784037481] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[18800929917] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[18810966174] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[18818034477] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[18820640784] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[18825565143] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18827255931] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18828899859] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18830598204] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369850144 RFLAGS_BEFORE=130 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[18833998557] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18835094652] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18836221008] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18837259617] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[18838565658] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18840035841] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[18844208823] [INFO] [nectar] [CPU2] NECTAR: Started.
[18847533375] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[18848783745] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010aff8
[18850126119] [INFO] [fontd] [CPU3] FONTD: Service ready
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18851742954] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047872 RFLAGS_BEFORE=130 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[18856855182] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18858437664] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18859922037] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[18860876397] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18862379844] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=234 subj_lo=0
[18864815739] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18867019479] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370267040 RFLAGS_BEFORE=134 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[18873670464] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[18877420452] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[18879595317] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[18884237724] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18885834693] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18887361504] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18889310616] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=235 subj_lo=0
[18893242434] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[18914815623] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[18919508619] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[18928637343] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1252 backend=VirtIO-GPU
[18930422016] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[18931222761] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18932838111] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18938088972] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18939811275] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18942835692] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18943947132] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18944943402] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18946080549] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[18951585378] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18952582572] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18953665302] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18954715065] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[18960497556] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18961462773] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18962600250] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18963724428] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=243 pred=0 subj_lo=0
[18982510107] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[18996079905] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[18998831181] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[19070121246] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[19085556732] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[19092909297] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19094420202] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[19095821844] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19098726537] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[19101531009] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[19102719240] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19105192458] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19107782958] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db490
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e4
[19109327160] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370366192 RFLAGS_BEFORE=134 CR3_BEFORE=72052736 fs_base=0 gs_base=18446744071564586640
[19112215518] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[19114128759] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19115429520] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[19122571149] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[19125786141] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db490
[19126740105] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[19128437724] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370431888 RFLAGS_BEFORE=134 CR3_BEFORE=73101312 fs_base=0 gs_base=18446744071564586576
[19131759372] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[19133902029] [INFO] [echo] [CPU1] echo: starting up
[19136034423] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[19144452558] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[19160894412] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[19162694430] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[19167043269] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[19168650468] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f5000
[19170369273] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f5000
[19172317098] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f8000
[19174311816] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276787200)
[19176128169] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[19179905448] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f9000 phys=0x4656000
[19181530830] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[19184282172] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[19186865280] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[19188442251] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[19195466169] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[19197518934] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[19199045547] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[19202660268] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[19209423750] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[19213805325] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[19215425031] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[19217068761] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[19219173831] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[19221659127] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[19224042255] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[19226014203] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[19227380733] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[19229296779] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[19231589256] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[19233523551] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[19234570410] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[19238290203] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19240588917] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19241733885] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[19247190633] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[19248730512] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[19253551152] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[19255074564] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[19256582697] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[19257689913] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[19258818810] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19260810492] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19266106332] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19268392605] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[19276239444] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[19278722694] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[19280543700] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[19282071006] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[19283814198] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19284594747] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19286473437] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19297987335] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19299216651] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[19302160548] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[19303405341] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19309572843] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[19310818527] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19314468987] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[19319037672] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[19321186071] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[19323578769] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[19324498314] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19326130032] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19327079772] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f1688
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19328796201] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370710416 RFLAGS_BEFORE=134 CR3_BEFORE=73998336 fs_base=0 gs_base=18446744071564586640
[19332035184] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[19333173057] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[19341698376] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[19343948580] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[19344711012] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[19345472190] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[19351827099] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19377106518] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[19404309903] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[19406321088] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[19410071604] [INFO] [bloom::present] [CPU3] bloom: presenter stuck waiting for ACQUIRED, resending ACQUIRE...
[19417129677] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[19437375903] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19442558850] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1688
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19444105263] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370775952 RFLAGS_BEFORE=134 CR3_BEFORE=74145792 fs_base=0 gs_base=18446744071564586576
[19448779911] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[19468048347] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[19485513762] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19486513101] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19492328757] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[19494180123] [INFO] [bloom] [CPU3] bloom: creating surface...
[19495032645] [INFO] [bloom] [CPU3] bloom: surface created!
[19496267142] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[19503286077] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[19506430779] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[19509354942] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[19513322598] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[19519447992] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1262
[19520660346] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[19532330697] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[19535799888] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[19537037091] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[19538213343] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [19546023255] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[19550211714] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[19554799506] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[19563367626] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[19571446389] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
T:07D0 T:0640 T:F0B0 [19602002871] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[19608985836] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[19611220860] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[19613852214] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[19616608407] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[19620668562] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[19637154075] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1262
T:1220 [19645842381] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[19656950973] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=31, our_read=32)
[19659450426] [INFO] [anther] [CPU1] anther: Connected to network stack
[19770691512] [INFO] [flyt
```
</details>
