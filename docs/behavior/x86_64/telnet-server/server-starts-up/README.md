# ✅ Scenario: Server starts up

> Last run: 2026-04-05 19:22:36

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 3951ms | - [📜](./01/serial.log) - |
| 2 | Then I should see a message in the serial output that says "telnetd: listening on guest port 2323" | ✅ | 4859ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12522273126] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[12528915960] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[12533742243] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[12536691057] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[12538477776] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[12539358975] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[12540410058] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[12541222650] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[12542030886] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[12542877105] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=33264
[12543683856] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=969224
[12544301385] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[12545048538] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[12545728074] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[12546469188] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[12547092987] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[12547895811] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[12548788494] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[12549783708] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[12550758429] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[12551673024] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[12552573396] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[12553571976] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[12554490234] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[12555472743] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[12556421163] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[12557332524] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[12558335856] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[12559260780] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[12560205471] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[12561121518] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[12562044693] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[12563001198] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[12563858868] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=168464
[12564776796] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[12565776333] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[12566613477] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[12567350037] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[12568064091] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[12568803225] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[12569561763] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[12570288126] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[12571745736] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[12573257829] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[12574175196] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[12574727814] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[12575242614] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[12575768007] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[12576355011] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[12576881394] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[12577398504] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[12577919145] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[12578440611] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[12578984286] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7795f000 (Usable)
[12579572412] [INFO] [kernel::memory] [CPU0]   [11] 0x7795f000 - 0x779c3000 (Reserved)
[12580143576] [INFO] [kernel::memory] [CPU0]   [12] 0x779c3000 - 0x779c4000 (Other)
[12580692630] [INFO] [kernel::memory] [CPU0]   [13] 0x779c4000 - 0x779c5000 (Reserved)
[12581262870] [INFO] [kernel::memory] [CPU0]   [14] 0x779c5000 - 0x779c6000 (Other)
[12581815818] [INFO] [kernel::memory] [CPU0]   [15] 0x779c6000 - 0x779c7000 (Reserved)
[12582384309] [INFO] [kernel::memory] [CPU0]   [16] 0x779c7000 - 0x779c8000 (Other)
[12582976626] [INFO] [kernel::memory] [CPU0]   [17] 0x779c8000 - 0x779c9000 (Reserved)
[12583549374] [INFO] [kernel::memory] [CPU0]   [18] 0x779c9000 - 0x77a54000 (Other)
[12584099847] [INFO] [kernel::memory] [CPU0]   [19] 0x77a54000 - 0x77a55000 (Reserved)
[12584670483] [INFO] [kernel::memory] [CPU0]   [20] 0x77a55000 - 0x77ed6000 (Other)
[12585222276] [INFO] [kernel::memory] [CPU0]   [21] 0x77ed6000 - 0x77ed7000 (Reserved)
[12585821523] [INFO] [kernel::memory] [CPU0]   [22] 0x77ed7000 - 0x77ff8000 (Other)
[12586374471] [INFO] [kernel::memory] [CPU0]   [23] 0x77ff8000 - 0x77ff9000 (Reserved)
[12586943688] [INFO] [kernel::memory] [CPU0]   [24] 0x77ff9000 - 0x787f9000 (Other)
[12587493006] [INFO] [kernel::memory] [CPU0]   [25] 0x787f9000 - 0x787fa000 (Reserved)
[12588062652] [INFO] [kernel::memory] [CPU0]   [26] 0x787fa000 - 0x788bb000 (Other)
[12588612993] [INFO] [kernel::memory] [CPU0]   [27] 0x788bb000 - 0x788bc000 (Reserved)
[12589218378] [INFO] [kernel::memory] [CPU0]   [28] 0x788bc000 - 0x788e6000 (Other)
[12589771227] [INFO] [kernel::memory] [CPU0]   [29] 0x788e6000 - 0x788e7000 (Reserved)
[12590384895] [INFO] [kernel::memory] [CPU0]   [30] 0x788e7000 - 0x788ec000 (Other)
[12590937843] [INFO] [kernel::memory] [CPU0]   [31] 0x788ec000 - 0x788ed000 (Reserved)
[12591509799] [INFO] [kernel::memory] [CPU0]   [32] 0x788ed000 - 0x788f2000 (Other)
[12592061691] [INFO] [kernel::memory] [CPU0]   [33] 0x788f2000 - 0x788f3000 (Reserved)
[12592693674] [INFO] [kernel::memory] [CPU0]   [34] 0x788f3000 - 0x7891f000 (Other)
[12593247381] [INFO] [kernel::memory] [CPU0]   [35] 0x7891f000 - 0x78921000 (Reserved)
[12593816829] [INFO] [kernel::memory] [CPU0]   [36] 0x78921000 - 0x7892a000 (Other)
[12594367797] [INFO] [kernel::memory] [CPU0]   [37] 0x7892a000 - 0x7892c000 (Reserved)
[12594937509] [INFO] [kernel::memory] [CPU0]   [38] 0x7892c000 - 0x78934000 (Other)
[12595487949] [INFO] [kernel::memory] [CPU0]   [39] 0x78934000 - 0x78935000 (Reserved)
[12596094258] [INFO] [kernel::memory] [CPU0]   [40] 0x78935000 - 0x7893f000 (Other)
[12596645061] [INFO] [kernel::memory] [CPU0]   [41] 0x7893f000 - 0x78940000 (Reserved)
[12597224178] [INFO] [kernel::memory] [CPU0]   [42] 0x78940000 - 0x7894d000 (Other)
[12597863850] [INFO] [kernel::memory] [CPU0]   [43] 0x7894d000 - 0x7894f000 (Reserved)
[12598632717] [INFO] [kernel::memory] [CPU0]   [44] 0x7894f000 - 0x7895d000 (Other)
[12599223087] [INFO] [kernel::memory] [CPU0]   [45] 0x7895d000 - 0x7895e000 (Reserved)
[12599794680] [INFO] [kernel::memory] [CPU0]   [46] 0x7895e000 - 0x7896a000 (Other)
[12600343074] [INFO] [kernel::memory] [CPU0]   [47] 0x7896a000 - 0x7896b000 (Reserved)
[12600906681] [INFO] [kernel::memory] [CPU0]   [48] 0x7896b000 - 0x7896f000 (Other)
[12601453293] [INFO] [kernel::memory] [CPU0]   [49] 0x7896f000 - 0x78970000 (Reserved)
[12602015514] [INFO] [kernel::memory] [CPU0]   [50] 0x78970000 - 0x78980000 (Other)
[12602602848] [INFO] [kernel::memory] [CPU0]   [51] 0x78980000 - 0x78981000 (Reserved)
[12603167676] [INFO] [kernel::memory] [CPU0]   [52] 0x78981000 - 0x78a11000 (Other)
[12603717060] [INFO] [kernel::memory] [CPU0]   [53] 0x78a11000 - 0x78a12000 (Reserved)
[12604289478] [INFO] [kernel::memory] [CPU0]   [54] 0x78a12000 - 0x78a1d000 (Other)
[12604840908] [INFO] [kernel::memory] [CPU0]   [55] 0x78a1d000 - 0x78a1e000 (Reserved)
[12605408970] [INFO] [kernel::memory] [CPU0]   [56] 0x78a1e000 - 0x78a43000 (Other)
[12606005313] [INFO] [kernel::memory] [CPU0]   [57] 0x78a43000 - 0x78a44000 (Reserved)
[12606576543] [INFO] [kernel::memory] [CPU0]   [58] 0x78a44000 - 0x78a50000 (Other)
[12607127445] [INFO] [kernel::memory] [CPU0]   [59] 0x78a50000 - 0x78a51000 (Reserved)
[12607694319] [INFO] [kernel::memory] [CPU0]   [60] 0x78a51000 - 0x78a5e000 (Other)
[12608239083] [INFO] [kernel::memory] [CPU0]   [61] 0x78a5e000 - 0x78a5f000 (Reserved)
[12608803218] [INFO] [kernel::memory] [CPU0]   [62] 0x78a5f000 - 0x78aa7000 (Other)
[12609386592] [INFO] [kernel::memory] [CPU0]   [63] 0x78aa7000 - 0x78aa8000 (Reserved)
[12610226805] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12862293444] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485751 free frames
[12878429256] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12886155381] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12888371793] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12889864779] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12896536356] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12899201634] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12900814113] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12901794114] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12902753457] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12903681186] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12905146518] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12906593139] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12907590069] [INFO] [bran::arch::x86_64::acpi000, GSI base 0
[12913080279] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12920316090] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12921943650] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12924471813] [INFO] [bran::arch] [CPU0] > GSI 1 -> 0x21
[12928619682] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12929549292] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12930749502] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[13479859437] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[13484187882] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[13491483291] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[13494919845] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[13496330001] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[13500158958] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[13522484646] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[13524405741] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[13525656078] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[13528337724] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[13529264628] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[13532730024] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[13543060146] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[13545456837] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[13562878824] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[13563954327] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[13584584211] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[13585591536] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[13588247739] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[13590209622] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[13592210511] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[13595509026] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[13596814077] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[13633070682] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62694000 ticks/sec), init_cnt=626940 for 100Hz
[13637038470] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[13638541191] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[13640548878] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[13654311726] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[13693954263] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[13695432432] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[13696929147] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[13698806385] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[13701676098] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[13706994147] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[13717237017] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[13727474838] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[13729475133] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[13732715337] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[13734119421] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[13737418332] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[13744931574] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[13754138442] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[13764377880] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[13769327616] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[13770937290] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[13772742489] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[13773609465] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[13778163828] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[13780530654] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[13781751687] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[13784015454] [INFO] [kernel::root] [CPU0] Spawning Root service...
[13785712149] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[13790166918] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[13791617730] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[13805457930] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[13807852047] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[13808785155] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[13810625697] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[13812570519] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[13813712550] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[13835904951] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[13840315533] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[13848480030] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[13854215364] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13890707754] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13934541951] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13937988999] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13946976384] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13949915991] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13954858368] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13958988813] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13962991119] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13964627325] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13969930986] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13987238562] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13990755603] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14003422488] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14011424229] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14020069371] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[14021490879] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[14023396332] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14055248889] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[14056487280] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[14109353808] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[14110766670] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[14210031297] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[14211399246] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[14776773858] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[15370384293] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[15410643171] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=497 journal=422 symbols=51 drops=0
[15455684145] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[16887408285] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=446 watches=0 history=959 journal=768 symbols=93 drops=0
[17795851062] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[17906080797] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[17907320013] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[18019416426] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[18090907824] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[18127073415] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[18127875843] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[18128511555] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[18133771095] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[18153510408] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[18173768646] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[18176508240] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[18234564051] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[18256049031] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[18256952934] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[18261371238] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[18287750415] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[18311929251] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[18317075535] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[18318592644] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[18408991623] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[18411350925] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[18514420056] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[18588811329] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[18604495305] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[18609384189] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[18611795301] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[18620412987] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[18695364006] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[18701942556] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[18703097160] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[18704040036] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[18704943741] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[18705636609] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[18706378119] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[18707203284] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[18707832198] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[18708466095] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[18709250340] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=33264
[18709942185] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=969224
[18710819061] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[18711811173] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[18712545093] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[18713300925] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[18713949012] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[18714616437] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[18715271124] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[18715940331] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[18716725071] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[18717408534] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[18718032135] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[18718656000] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[18719571750] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[18720251187] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[18721134729] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[18721769979] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[18722475981] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[18723104268] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[18723764433] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[18724411893] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[18725172477] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[18725835282] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[18726483963] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=168464
[18727166436] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[18727906692] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[18728653977] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[18729423735] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[18730164288] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[18730934145] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[18731728719] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[18732513756] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[18734146398] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[18736137882] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[18737101944] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18745706100] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[18760424100] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[18765624735] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[18775858431] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[18776602680] [CONTRACT] [kernel] [CPU0] Spawning init process...
[18779288022] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[18782262081] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[18783995934] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
ThingOS Petals
type 'help'USER_TRAMPOLINE: PC=0x f200000 SP=0x800000 ARG0=0x600000
or commands

petals> [18790064040] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[18791112714] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[18813673263] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[18817434966] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[18819650190] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[18821699688] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[18830044926] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[18835106763] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[18838977762] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[18840057687] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[18844001649] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[18847402299] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[18848579046] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[18853970289] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[18855158223] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[18856232604] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[18857454726] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[18860740470] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[18862086837] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[18863621535] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[18865490391] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[18867155604] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[18868657698] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[18874385145] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[18928822110] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[18935906253] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[18942364947] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[18947954619] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[18951307650] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[18956442912] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[18962382153] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[18968878401] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[18975841203] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[18982884129] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[18990622827] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[18996547845] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[19002895131] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[19008158202] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[19015103448] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[19020554916] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[19026445416] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[19033527216] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[19041176748] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[19074203181] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[19085855745] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[19113919275] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[19126234611] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[19148023653] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[19155387405] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[19160432511] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[19179790674] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[19187909103] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[19193149008] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[19198266879] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[19202376864] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[19205875293] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[19211077809] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[19214648244] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/telnetd
[19217837496] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[19224492903] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[19231380135] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[19237751379] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[19245332337] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[19252907058] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[19260853392] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[19267160880] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[19270997658] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[19296802206] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[19456280910] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[19462936713] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[19463726238] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19465382838] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19470360360] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19471733490] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19480227954] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[19485574779] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[19486894746] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19488566427] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[19495101747] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[19496181969] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[19497126924] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19498921563] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19503103587] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[19504992408] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19513007316] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[19516852806] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[19518396216] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[19520182044] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[19524764127] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[19528283412] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[19530800454] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[19533518598] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19538576739] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 02:22:48 = 1775442168 unix_secs
[19540385898] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775442168, mono_ns=9769966470, offset=1775442158230033530ns
[19542017088] [INFO] [rtc_cmos] [CPU1] System clock anchored
[19555030440] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[19594558071] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[19603682076] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[19605137442] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19607695932] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19614390180] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[19617414597] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[19625535600] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[19629182595] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[19632345513] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19634269611] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[19639217697] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[19643836212] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[19645830897] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[19646661045] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19648413906] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19655769078] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19658238435] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19665865428] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[19669759989] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[19670863938] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19672942377] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[19673902215] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[19681033581] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[19682070804] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[19685496732] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[19687960512] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[19689563058] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[19690905861] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[19692321363] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[19705978842] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[19708030683] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[20229749925] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20233406094] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[20234469123] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[20236371078] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20244201351] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20246907945] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[20254446696] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[20257340532] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[20258631525] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[20260479393] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352688 RFLAGS_BEFORE=134 CR3_BEFORE=59662336 fs_base=0 gs_base=18446744071564586576
[20268031773] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[20270709393] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[20271906798] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[20274868482] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[20280017934] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[20282479107] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[20283725814] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[20285781318] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[20287644531] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[20289120423] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[20292423756] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[20293517277] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[20294563575] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[20295526779] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[20296458072] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[20297308416] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[20298459555] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[20314344468] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[20316875337] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[20323302450] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[20332707186] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[20334556506] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[20353338489] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[20358214866] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[20361715044] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[20364371379] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[20365439886] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20367972306] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20373065229] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[20374638867] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[20377398822] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20399720352] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[20404704441] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[20408061399] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[20409275502] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20411833398] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20419749504] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20422092537] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20432950230] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[20438321343] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[20439782517] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[20441563296] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500048 RFLAGS_BEFORE=134 CR3_BEFORE=68235264 fs_base=0 gs_base=18446744071564586640
[20445701727] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[20446614936] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20448374133] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20450367795] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[20452085148] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[20453746434] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20456515728] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[20457624396] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20458496256] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[20459919150] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[20461866975] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[20463861528] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[20464961913] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[20466463941] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[20469214029] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[20470436844] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[20471567127] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[20473351866] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[20474400474] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[20475269793] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[20478566988] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1236 port=2 model='                                        ' rpc_port=5
[20505994410] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[20506967382] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[20508218379] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[20509433472] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[20510261277] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[20511162606] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[20512180062] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[20513226063] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[20519596944] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[20522108145] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=744071564586608
[20526671286] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[20849298162] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[20851489725] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20853377490] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369582832 RFLAGS_BEFORE=134 CR3_BEFORE=68349952 fs_base=0 gs_base=18446744071564586576
[20856852654] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[20858210934] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20860220601] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[20861053224] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20862972273] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[20865922638] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[20872293189] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[20874989124] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[20878801350] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[20883809232] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[20887439199] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[20890060521] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[20891783385] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[20892660393] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20894675274] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20915910609] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20921003367] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[20965707774] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[21845574531] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[21848783682] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[21850293861] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21851996925] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369719008 RFLAGS_BEFORE=130 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[21861243129] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[21863140464] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[21864579066] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[21866078949] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[21867547317] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[21871957602] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[21874810584] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653472 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[21882794109] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[21885124569] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[21892322133] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[21894160530] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[21895702323] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[21897097398] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[21899736639] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using cooperative polling loop (2ms interval)
[21903704064] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[21905401452] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[21907158537] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[21908057193] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21909960996] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21925918707] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[21927591939] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[21929347935] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[21930902433] [INFO] [ps2_mouse] [CPU3] ps2_mouse: using cooperative polling loop (2ms interval)
[21981790710] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[22007084418] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[22014963762] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[22018104372] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[22019966793] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22021738035] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369784544 RFLAGS_BEFORE=130 CR3_BEFORE=79892480 fs_base=0 gs_base=18446744071564586576
[22028383014] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[22029704697] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22031043012] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[22032407001] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22040210049] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[22045801767] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22047237003] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22071361488] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[22074910374] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[22082709858] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[22085987154] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[22088166837] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[22089095127] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22090640352] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22096290579] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[22098744063] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22106061780] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[22108494177] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[22109544237] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22110913143] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915616 RFLAGS_BEFORE=130 CR3_BEFORE=80826368 fs_base=0 gs_base=18446744071564586640
[22113858756] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[22114645707] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22116535122] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22117321776] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[22118681706] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[22124321835] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22128101622] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[22135240611] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[22138101645] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[22139281659] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22140872457] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981760 RFLAGS_BEFORE=130 CR3_BEFORE=80953344 fs_base=0 gs_base=18446744071564586576
[22144545060] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[22145568159] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22147306962] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22156881549] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[22160612166] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[22168527942] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[22171951890] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[22174815861] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[22176160974] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22178768832] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22205703828] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[22210937760] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[22218973260] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[22223654475] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
[22226825247] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[22228074594] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22229662884] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22230887085] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010d500
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22233611631] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113920 RFLAGS_BEFORE=130 CR3_BEFORE=81256448 fs_base=0 gs_base=18446744071564586640
[22241223378] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[22247292210] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[22250582343] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[22258215078] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[22260720108] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1058
[22261757892] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22263055716] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370197952 RFLAGS_BEFORE=134 CR3_BEFORE=81530880 fs_base=0 gs_base=18446744071564586576
[22266592128] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[22267380894] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22269039870] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22271075904] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[22272987792] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[22281085695] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[22283185584] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22284621084] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22286205051] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22288895475] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=225 subj_lo=0
[22294322490] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[22295437098] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22296731292] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[22298259324] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=226 pred=0 subj_lo=0
[22308444114] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22309959441] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22311211956] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[22312956435] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[22314151497] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22315559739] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[22316874789] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[22322270652] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[22330763862] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[22333484778] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[22336816194] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22338707721] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369850080 RFLAGS_BEFORE=130 CR3_BEFORE=80564224 fs_base=0 gs_base=18446744071564586608
[22343103453] [INFO] [nectar] [CPU2] NECTAR: Started.
[22345082232] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22346705304] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047840 RFLAGS_BEFORE=134 CR3_BEFORE=81100800 fs_base=0 gs_base=18446744071564586608
[22351417077] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[22353722853] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db568
[22354754400] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22357204254] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22358888937] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370267936 RFLAGS_BEFORE=130 CR3_BEFORE=81739776 fs_base=0 gs_base=18446744071564586608
[22362119604] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[22364105280] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[22366248003] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[22368914700] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[22370591496] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[22372936344] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22374224928] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
[22375659207] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22376781735] [INFO] [fontd] [CPU3] FONTD: Service ready
[22377714612] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22378890171] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[22390556694] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[22397410992] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[22412992932] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[22418750607] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[22422217191] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f4000
[22424238507] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f4000
[22425923718] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f7000
[22427580318] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22428579822] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276783104)
[22429307901] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22430401950] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[22432453263] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22433980668] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f8000 phys=0x4e94000
[22434854871] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=239 subj_lo=0
[22436524638] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[22439496882] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[22442324553] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[22443967953] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[22450982631] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22452147894] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22456661865] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22457723673] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22458915930] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22460371230] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[22475168034] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22476370323] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22477620957] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22478990127] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[22490298402] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1251 backend=VirtIO-GPU
[22492206627] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[22493129934] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22495565037] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22498906749] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22500601827] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22502059932] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22503954198] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[22516832811] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[22518424797] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[22519959231] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22521607020] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[22565652648] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[22583685960] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[22584972597] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[22648171161] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d5000 exec=false
[22664362347] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2eb000 exec=false
[22665327597] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22666790355] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22672928223] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[22675933038] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[22678464666] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db568
[22683020316] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[22683922536] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e3
[22686191088] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22691441916] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370367264 RFLAGS_BEFORE=130 CR3_BEFORE=82681856 fs_base=0 gs_base=18446744071564586640
[22696165635] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22698920541] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[22699801377] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[22708843905] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[22712167236] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db568
[22713658407] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[22715588577] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370432800 RFLAGS_BEFORE=130 CR3_BEFORE=83734528 fs_base=0 gs_base=18446744071564586576
[22720625235] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[22723357866] [INFO] [echo] [CPU1] echo: starting up
[22726806267] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[22728209460] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[22740028773] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[22741168659] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[22751833467] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[22754570817] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[22757463300] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[22759492998] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[22761329184] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[22763158935] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[22764795801] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[22766783259] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[22768410357] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[22772613732] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[22774015869] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[22775623992] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[22780329825] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[22792534908] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22804868889] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[22808156712] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[22812532413] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[22814935077] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[22817183598] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[22818300120] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22820157261] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22823473992] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[22826127918] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22828380861] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[22833943572] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22836633567] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[22840716690] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[22844019066] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[22846528056] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[22849235739] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[22850319327] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22852660743] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22861834908] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[22864120719] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[22865437419] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22866651819] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[22870878525] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[22873407414] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22874724081] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22881018369] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[22884613323] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db568
[22885825479] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22887471585] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370711328 RFLAGS_BEFORE=130 CR3_BEFORE=84332544 fs_base=0 gs_base=18446744071564586640
[22890460593] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[22892962422] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[22893868404] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22895563053] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22896402540] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[22899822000] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[22901428902] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[22903656105] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[22912285473] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[22915653288] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db568
[22916848416] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22918328136] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/telnetd'
[22919416245] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370776864 RFLAGS_BEFORE=130 CR3_BEFORE=84480000 fs_base=0 gs_base=18446744071564586576
[22924165968] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/telnetd
[22925040666] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22926687828] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22941544725] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[22943753250] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[22947962070] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[22949554155] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=220000 exec=false
[22954258536] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[22955799372] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=228000 exec=false
[22963912323] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 32 (user task/process) assigned to CPU 2
[22966754184] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=32)
[22968356697] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[22973479056] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22981822776] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db568
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22984418556] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=32 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370842400 RFLAGS_BEFORE=130 CR3_BEFORE=84590592 fs_base=0 gs_base=18446744071564586608
[22993400694] [INFO] [telnetd] [CPU2] telnetd: starting on port 2323
[23004942345] [INFO] [telnetd] [CPU2] telnetd: waiting for network stack
[23015791392] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[23016835248] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[23019266919] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[23035078704] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[23071432164] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[23109268677] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[23111286198] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[23124093729] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[23126589552] [INFO] [bloom] [CPU3] bloom: creating surface...
[23127744486] [INFO] [bloom] [CPU3] bloom: surface created!
[23128640172] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[23134142328] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[23142890364] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[23156815440] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1261
[23158341558] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[23172526773] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[23175816345] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=33)
[23177347974] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[23180133339] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0AF0 [23188662585] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[23191283412] [INFO] [telnetd] [CPU2] telnetd: waiting for network stack
[23196567636] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[23200924824] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[23212299924] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[23221679910] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[23230463190] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1261
T:1050 T:0EC0 T:F930 [23260108113] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[23263637496] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[23268626469] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[23274502416] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[23284487391] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 37 (user thread) assigned to CPU 3
[23287461582] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=37 (priority=2)
[23290167582] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[23293232754] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
T:1AA0 [23313507459] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=77c80b059e863710)
[23318221377] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[23320879593] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[23332282710] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[23334800577] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[23335940628] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[23351140131] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=276
[23353544412] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[23355088416] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[23356180419] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23357240610] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23358743562] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=276 subj_lo=0
[23379250554] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1272)
[23383484355] [INFO] [telnetd::net_client] [CPU2] telnetd: connected to socket API (netd=27, write=29, read=30)
[23386566984] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[23398092597] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=244
[23399861925] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[23401131864] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[23402769654] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23404470507] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23406327813] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[23449123896] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([243, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[23465777907] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1273)
[23466821037] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[23487459402] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=279
[23489639283] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[23491167018] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[23492855001] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23494556580] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23496408705] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=279 subj_lo=0
[23515473531] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1277)
[23517800889] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[23535547200] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[23537956398] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[23546861217] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[23556032214] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=31, our_read=32)
[23557576152] [INFO] [anther] [CPU1] anther: Connected to network stack
[23590746432] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[23774185941] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=639 watches=13 history=1024 journal=1024 symbols=290 drops=0
[23935314843] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[23964802587] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[24639288861] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[24641153955] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[24864964014] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[25111155003] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[25856106672] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[25893921042] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[25896504480] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[26020657245] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=672 watches=13 history=1024 journal=1024 symbols=331 drops=0
[26288484849] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[26294800488] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[26308963164] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[26310739092] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[26406429093] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[26526530349] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26528193582] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26530496685] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26541842184] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=332 pred=0 subj_lo=0
[26697401973] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[26766670029] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26767976268] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26769277623] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26770620756] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=333 pred=0 subj_lo=0
[26844859107] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[26894496981] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[26895667491] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[26897062632] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26898516348] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=334 pred=0 subj_lo=0
[26943089382] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[26955401913] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[26957268657] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[26980075485] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[27004232640] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12004000
[27005405295] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[27006728892] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[27047188740] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[27048291039] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[27078480462] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[27089900277] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[27096644388] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[27099602046] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[27102643359] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[27109570620] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[27146958135] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[27149077989] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[27152209821] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[27173246529] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x12005000
[27175569960] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[27177346317] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[27763450737] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=710 watches=16 history=1024 journal=1024 symbols=349 drops=0
[27937180491] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[27939380931] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[27942222198] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[28184250270] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[28185978876] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[28207564935] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[28227223761] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[28232921739] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[28235840886] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[28286162619] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[28289497566] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[28292731665] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[28297209501] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 2323 with backlog 64
[28301251770] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 2323, handle=1
[28303869825] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[28304936451] [INFO] [telnetd] [CPU2] telnetd: listening on guest port 2323 (handle=1)
[28306722345] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=2
[28309411383] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[28311257007] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=3
[28313136324] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[28314922251] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=4
[28316922711] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[28318751340] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=5
[28341622320] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[28344011388] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[28350753882] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[28352010126] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[28353508821] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28355082888] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=335 pred=0 subj_lo=0
[28375860810] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[28383624720] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[28438023834] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[28439978259] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=6
[28456568646] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=6)
[28535796993] [INFO] [fontd
```
</details>
