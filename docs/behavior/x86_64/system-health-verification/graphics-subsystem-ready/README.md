# ❌ Scenario: Graphics Subsystem Ready

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 5165ms | - - - |
| 2 | When I wait for the system to reach ready state | ✅ | 0ms | - - - |
| 3 | Then the serial output should contain "[bloom] First frame rendered" | ✅ | 2029ms | - [📜](./03/serial.log) - |
| 4 | And I should see the desktop wallpaper | ❌ | 1015ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11342487717] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11347982415] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11351500611] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11353407483] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11354579412] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11355202815] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11355853377] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11356425498] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11357005572] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11357607261] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11358206541] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11358803940] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11359490505] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11360143443] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11360817930] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11361433314] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11362046487] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11362634184] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11363259402] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11363849211] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11364435555] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11365011372] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11365605207] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11366186799] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11366812479] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11367404400] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11368010049] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11368647906] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11369229762] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11369825379] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11370430269] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11371056378] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11371685556] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11372292426] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11372974569] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11373656118] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11374418352] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11375118777] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11375800788] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11376515733] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11377211241] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11378086698] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11379510648] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11381042211] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11381787186] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11382317826] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11382819261] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11383328022] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11383862919] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11384390919] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11384896875] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11385408177] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11385913935] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11386441341] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11386975974] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11387548392] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11388083553] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11388639900] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11389179978] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11389767708] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11390363556] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11390965278] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11391506841] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11392063056] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11392599966] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11393191194] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11393744175] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11394323787] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11394865185] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11395420905] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11395959465] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11396514129] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11397051468] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11397634479] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11398386219] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11399076183] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11399618736] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11400266955] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11400832443] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11401391265] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11401931112] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11402486172] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11403023775] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11403600021] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11404157820] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11404717005] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11405257380] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11405818677] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11406358656] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11406922362] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11407503855] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11408065152] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11408604702] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11409165141] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11409705912] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11410267044] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11410843719] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11411406666] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11411948889] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11412505269] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11413044687] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11413603575] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11414207376] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11414770389] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11415311655] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11415869190] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11416411875] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11417284890] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[11657000454] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[11668128120] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[11673152700] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[11674410792] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[11675351358] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[11679799890] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[11681456985] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[11682525855] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[11683216281] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[11683898325] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[11684569512] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[11685543837] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[11686463580] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[11687185191] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[11687847996] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[11688505521] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[11689162122] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[11690282604] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[11691276861] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[11692029228] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11693649165] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[11694566103] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[11695555707] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[11697259959] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[11698842408] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11699712915] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11700270021] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[11701040472] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12073632549] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12074661291] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12078235059] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12079162062] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12079936242] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12081439788] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12094190988] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12095524782] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12096279492] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12098025324] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12098562399] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12100976250] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12108972876] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12110781078] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12124383711] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12124988403] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12141193713] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12141866451] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12144207933] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12145498431] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12146627889] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12148903041] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12149740812] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12184639731] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62399900 ticks/sec), init_cnt=623999 for 100Hz
[12186166410] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12187106811] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12188365596] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12194036811] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12225068163] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12226042950] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12227937447] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12229172736] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12230254212] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12232971333] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12234283776] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12254743017] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12256389684] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12257323815] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12258875244] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12259568640] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12261716577] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12263924376] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12288700875] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12290488320] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12291403542] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12293369352] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12294243951] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12295057566] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12295890255] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12296562201] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12303289350] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12304368846] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12306580341] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12307617597] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12313197534] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12314716260] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12315426948] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12316688802] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12317589570] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12318407673] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12331495473] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12334468608] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12335680665] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12336683502] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[12359289195] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12377883045] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12380717151] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12383976561] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12385612932] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12388001010] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12390417567] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12392411955] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[12393313284] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[12394305363] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12400569291] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12402227871] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12404814576] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12408089034] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[12412346067] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[12413190108] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[12425421591] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[12426245535] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[12433444155] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[12434371554] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[12462374067] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[12463226523] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[12808307721] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[13284310743] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[13311094434] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[13345096281] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[14658861459] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=961 journal=769 symbols=95 drops=0
[15438987432] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[15539149329] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[15540270372] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[15649398270] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[15715220466] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[15741390852] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[15742394052] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[15743030193] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[15746529282] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[15767211999] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[15784843602] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[15787524852] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[15840857967] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[15864423894] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[15865650009] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[15870298455] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[15905014323] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[15934292715] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[15938259678] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[15940034748] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[16010858655] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[16013806545] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[16107264723] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[16173982968] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[16187359749] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[16191305460] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[16194110097] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[16223513031] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[16286799012] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[16292719839] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[16293717792] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[16294814580] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[16295931135] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[16296622683] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[16297272816] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[16298025546] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[16298841669] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[16299508863] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[16300567701] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[16301842788] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[16303531827] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[16305169122] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[16306836777] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[16307919375] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[16308848886] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[16309807800] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[16310692332] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[16311680187] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[16312549803] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[16313444004] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[16314311541] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[16315254483] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[16316117070] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[16317080670] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[16317997476] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[16318932993] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[16319929593] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[16320867387] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[16321755417] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[16322456898] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[16323113763] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[16323810888] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[16324663311] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[16325351823] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[16326092013] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[16326934437] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[16327871934] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[16328813061] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[16329598230] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[16330548465] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[16331667165] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[16334175594] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[16336788798] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[16337600235] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16346111892] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[16361542560] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[16366496982] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[16376687778] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[16377511359] [CONTRACT] [kernel] [CPU0] Spawning init process...
[16379615076] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[16382481753] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[16383246429] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [16390664730] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[16396027923] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[16425054657] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[16428743133] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[16430549949] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[16432446063] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[16442805522] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[16449497163] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[16454230716] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[16455846594] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[16461724653] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[16466526813] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[16468108800] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[16473736257] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[16475272902] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[16477032726] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[16478773080] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[16483555572] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[16485417102] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[16487138085] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[16489361229] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[16491594471] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[16494195465] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[16500279774] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[16542885447] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[16550501385] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[16556649285] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[16562632317] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[16566506649] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[16571148396] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[16577694672] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[16583328036] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[16589073237] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[16595683434] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[16603126782] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[16610961510] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[16617293451] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[16623336081] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[16629785007] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[16636434969] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[16642447833] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[16648180692] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[16653855075] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[16658871108] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[16665530508] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[16671030486] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[16676167563] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[16681468947] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[16686205008] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[16691050101] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[16695884073] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[16700760813] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[16705167897] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[16709940192] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[16713147165] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[16716538278] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[16721280378] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[16726346406] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[16731577104] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[16736500473] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[16741073844] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[16748152377] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[16753463364] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[16758583809] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[16763945253] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[16767248949] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[16786550550] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[16937973129] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[16945322163] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[16946123997] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[16947832407] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16952576883] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[16954039905] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[16963418439] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[16969368207] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
[16970381703] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[16973117172] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[16980400140] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[16981125381] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[16982252628] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[16984614504] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[16991277864] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[16993766592] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17004333225] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[17009795946] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[17011240092] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[17013545472] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[17017192302] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[17020608528] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[17022418644] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[17024619447] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17030423850] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:02:14 = 1775437334 unix_secs
[17032178262] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775437334, mono_ns=8515863774, offset=1775437325484136226ns
[17033827008] [INFO] [rtc_cmos] [CPU1] System clock anchored
[17046568572] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[17085634203] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[17098960791] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[17100211887] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17102884656] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17111766936] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[17115440958] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[17126840115] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[17132254755] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[17137286265] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17139482844] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210624 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[17146790298] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[17153125440] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[17155190184] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[17156196123] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17158308519] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17165953761] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17168499513] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17175482775] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[17178459936] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[17179618500] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17181225831] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[17185610640] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[17186764716] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[17192844273] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[17194482228] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[17196163941] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[17198528226] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[17202460770] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[17203943559] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[17216334267] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[17218092276] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[17717516883] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17722382106] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[17725160046] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[17726939274] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[17731432752] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[17733225510] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[17735409714] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[17736991107] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[17738130399] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[17740445052] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[17741399907] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[17742366411] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[17743189827] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[17743942194] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[17744725713] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[17745810819] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[17750472267] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[17751377952] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17753023365] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17759920926] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17762343720] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[17769213693] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[17772152112] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fab68
[17774147292] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[17776179630] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352816 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[17782411977] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[17803383246] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[17805840987] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[17816809527] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[17818833285] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[17824439424] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[17832914286] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[17835084498] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[17861825124] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[17864721171] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[17867541153] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[17868700014] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[17870183859] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[17871140595] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17872844055] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17884007361] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[17885828037] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17893175025] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[17896326756] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[17898562440] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[17899534521] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17901321636] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17906310510] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17907874479] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17914892589] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[17917809294] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
[17918650497] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[17921203377] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500416 RFLAGS_BEFORE=134 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[17925886671] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[17926726092] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17928308904] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17930204787] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[17932189869] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[17933604876] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17936086575] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[17938890684] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[17940291072] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[17941757097] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[17942979021] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[17943949023] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[17946135801] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[17947705479] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[17948723958] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[17950061910] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[17951675940] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[17952668877] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[17954216049] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[17955874728] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[17956826844] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[17958098433] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[17959237164] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[17960261913] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[17961194856] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[17962110837] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[17963147928] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[17964284778] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[17970273189] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[17972062317] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434336 RFLAGS_BEFORE=130 CR3_BEFORE=60096512 fs_base=0 gs_base=18446744071564586608
[17978159892] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[18317577498] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[18320027484] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583200 RFLAGS_BEFORE=134 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[18323911617] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18325832085] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[18327924252] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[18329056977] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[18330150432] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18332035623] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[18332871051] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18342207906] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[18345333930] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[18347428605] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[18352829055] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[18355719591] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[18357645900] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[18359522445] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[18360448194] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18362467464] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18381800019] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18386108928] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[18406742178] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[18409270968] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
[18409977036] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18413669736] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716640 RFLAGS_BEFORE=134 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[18418244823] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[18420324549] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[18421700550] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[18422909604] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[18423853734] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[18425572869] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[18427831884] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650208 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[18437356212] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[18439527711] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[18442524540] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[18444234501] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[18448256046] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[18449025903] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[18450376329] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[18451922016] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[18453768663] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[18477559914] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[18479351748] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[18481507473] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[18482496417] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18484710123] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18560674506] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[18585349035] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[18593518449] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[18596853660] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[18598264476] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18600017898] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369783472 RFLAGS_BEFORE=130 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[18604126827] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[18605067657] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18607159032] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18608410722] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[18613024848] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[18619987518] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18622156245] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18634886325] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18638859492] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[18646243044] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[18649200570] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[18651238386] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[18651947655] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18653392428] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18658850067] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18661231083] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18668845899] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[18672023403] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
[18672971031] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010aff8
[18674510118] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[18675233610] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18677001123] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18677862423] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915472 RFLAGS_BEFORE=134 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[18682580994] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[18684415992] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[18685842417] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18689249502] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[18697062549] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[18699192930] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010aff8
[18700697004] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18702114717] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981872 RFLAGS_BEFORE=130 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[18705884967] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[18706764615] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18708274035] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18717287523] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18721771332] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[18728775483] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[18731302590] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[18733767261] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[18734749869] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18736344429] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18760297182] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[18765099705] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[18773629182] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[18776967198] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
[18778925451] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010c048
[18780199284] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[18781001514] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18783106551] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18783960987] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113920 RFLAGS_BEFORE=130 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[18789505449] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[18799896687] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[18803379408] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[18811197735] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[18813337752] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[18814928385] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[18816054873] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18817336758] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370198240 RFLAGS_BEFORE=130 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[18820271580] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[18821089155] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18822366519] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[18823189209] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18823957449] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[18827263587] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18829140165] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18830221509] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18832371459] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18833962455] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18835765938] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18837429039] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[18839475930] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=226 subj_lo=0
[18863979255] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[18875455203] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[18879824568] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18881391078] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18882507039] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18883820208] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18885593265] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18886806411] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18888704274] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[18890490696] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[18893168877] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[18897647901] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[18903521901] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[18908855130] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18911018808] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849392 RFLAGS_BEFORE=134 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[18914962077] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[18916572840] [INFO] [nectar] [CPU2] NECTAR: Started.
[18917179248] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[18918967749] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[18920818059] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
[18922999854] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010aff8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18924832278] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370048128 RFLAGS_BEFORE=130 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[18929870487] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[18932818113] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010d548
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18934323606] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370267056 RFLAGS_BEFORE=130 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[18937734948] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[18939647958] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[18940627860] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18941983599] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[18942990297] [INFO] [fontd] [CPU3] FONTD: Service ready
[18943839189] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18945290232] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[18946457178] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18948150078] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[18961127559] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[18962773005] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[18965037333] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[18966941004] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[18968956017] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[18970019145] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[18971968752] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x44b6000
[18974400357] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[18975962643] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[18977870175] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[18978817869] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[18979553043] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[18980455263] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[18981772755] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18982614354] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[18983563104] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[18984450342] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[18991881579] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[18993264081] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[18994731525] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18996648858] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=236 subj_lo=0
[18998448513] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[19004971788] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19006882158] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19008714747] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[19009651815] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19011803547] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=237 subj_lo=0
[19018114665] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[19019392623] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19020343914] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19021618011] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19022714238] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[19023506865] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=238 subj_lo=0
[19024980150] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[19026836136] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[19028399016] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[19030128051] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[19031818179] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[19032627042] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19033607835] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[19034305455] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19035333471] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[19035959448] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19037025216] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=239 pred=0 subj_lo=0
[19040584497] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=19, read=20)
[19045838196] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=21, read=22)
[19051115226] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[19055367936] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19070992710] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[19107180675] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1257 backend=VirtIO-GPU
[19108602777] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19110004980] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[19110789621] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19112691675] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19145373423] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[19163127390] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=23, resp=26
[19164889029] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[19183207230] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19194029184] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19195605363] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19215794037] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19250424072] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19253883099] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[19271913441] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[19279844595] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19280748498] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19282134300] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19283645601] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[19286658765] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[19288591080] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[19289395455] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19290979455] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db690
[19296150324] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e9
[19301182692] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[19302602715] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19305625812] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370509504 RFLAGS_BEFORE=134 CR3_BEFORE=72433664 fs_base=0 gs_base=18446744071564586640
[19311331776] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[19312400184] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[19315290687] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db600
[19316479776] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[19318466937] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370575040 RFLAGS_BEFORE=134 CR3_BEFORE=73482240 fs_base=0 gs_base=18446744071564586576
[19322645793] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[19338776556] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[19339779624] [INFO] [echo] [CPU1] echo: starting up
[19340634357] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=19, RX port=22
[19342976829] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[19360625196] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[19361817387] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[19362846888] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[19364120424] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[19383209736] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[19384843863] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[19389636750] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[19406356728] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[19408689102] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[19411111170] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[19421998266] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[19426871574] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[19439341878] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[19444473510] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[19446023685] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[19446727080] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19448202444] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19450578972] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19451706054] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19453781193] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19455631965] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[19463439600] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[19467510909] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[19470072237] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[19471589808] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[19472506647] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[19474631253] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[19475836776] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19477112424] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19479119022] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[19480465521] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19482563331] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([232, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19487955927] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19491665523] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19498677000] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[19501321488] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[19502592219] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010d548
[19503593241] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19505241558] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370710208 RFLAGS_BEFORE=134 CR3_BEFORE=74002432 fs_base=0 gs_base=18446744071564586640
[19508445693] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[19509711177] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19511490570] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19514808060] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[19516006620] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[19518252204] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[19523225733] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[19525981365] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[19527269553] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[19528569027] [INFO] [netd] [CPU3] NETD: Driver TX port=19, RX port=22, link_up=true, mtu=1500
[19531155204] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[19535286606] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[19544004843] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[19550443803] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f2a40
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19552141488] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370775744 RFLAGS_BEFORE=134 CR3_BEFORE=74149888 fs_base=0 gs_base=18446744071564586576
[19556320344] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[19571880273] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[19573976796] [INFO] [bloom] [CPU3] bloom: creating surface...
[19575290163] [INFO] [bloom] [CPU3] bloom: surface created!
[19576500735] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[19581031206] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[19585296060] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1263
[19586185278] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[19598123985] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[19600872390] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[19602123948] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[19603206414] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [19607785923] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[19613154231] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[19616002857] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[19625355552] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[19633671123] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
T:07D0 T:0640 T:F0B0 [19646425260] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([237, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[19662090393] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[19663886682] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[19666257171] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[19667455269] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[19672923435] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[19696440423] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[19699674423] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[19700511072] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[19703951157] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[19704832554] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=278
[19706175588] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[19707899046] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19709534823] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19711452024] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19713397209] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=278 subj_lo=0
[19722492504] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[19731396036] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[19734135663] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[19737291090] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[19738881888] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[19740474600] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[19741795689] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19744855086] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=29, our_read=30)
[19746573561] [INFO] [anther] [CPU1] anther: Connected to network stack
[19749014043] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19755345324] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[19757985324] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[19760016870] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[19761164082] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1266)
[19762447584] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=21
[19763546781] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[19764664161] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[19767530343] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1263
T:1220 [19775074902] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[19777263660] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[19792278462] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=239
[19794147318] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[19795262487] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19796340696] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 4)
[19797192261] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19798411182] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19799650662] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=239 pred=0 subj_lo=0
[19812499938] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1271)
[19813604250] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[19818701430] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[19822582923] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[19825540713] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[19837334649] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=279
[19839778695] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[19840942110] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[19842078300] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19843383846] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19844765622] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=279 subj_lo=0
[19861151607] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1274)
[19863075012] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[19881886068] [INFO] [netd] [CPU3] NETD: DHCP configured �� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[19885616388] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[19890491709] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[19893243975] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=1
[19896543678] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[19900117116] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=2
[19926816195] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=1)
[19963794807] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[19966267167] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[19970435958] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[20086390533] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[20350526196] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[20385115773] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[20396472030] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[20502214986] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=661 watches=13 history=1024 journal=1024 symbols=314 drops=0
[20597532318] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[20741705061] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[20879079474] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[21120884301] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[21275813988] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[21478077159] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[21504350571] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[21665507820] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[21849708177] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[22031786733] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[22037266779] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[22038912687] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[22040045940] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[22041138273] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[22042379040] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=226 subj_lo=0
[22052458527] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[22053534327] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[22054737738] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[22056197625] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=340 pred=0 subj_lo=0
[22095707502] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[22205982876] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=710 watches=15 history=1024 journal=1024 symbols=342 drops=0
[22274608488] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[22457005032] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[22553355594] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[22555014108] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[22558067268] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22621901478] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[22676778729] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22753184817] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[22790693013] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22792298595] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22793515437] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22795637238] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=307 pred=0 subj_lo=0
[22800548958] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22888760169] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[22958961861] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=fec1d29ef0678173)
[22970852157] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[22972125132] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[22973252478] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22974512055] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=308 pred=0 subj_lo=0
[23021878275] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23082851154] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[23093522265] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[23095487547] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[23109917589] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23111019327] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23112151590] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23113401498] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=351 pred=0 subj_lo=0
[23122315260] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23127552327] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[23170242282] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[23222859000] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[23224013604] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23225163753] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23226269682] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=352 pred=0 subj_lo=0
[23260378812] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[23280916857] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x1222d000
[23282833530] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[23285128053] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[23311012560] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[23353461648] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[23364231693] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[23370097245] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[23373573762] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[23376686355] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[23381931309] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[23393098344] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[23408828454] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[23411445354] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[23413548675] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[23434262808] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x1222e000
[23435746191] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[23616483429] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=c1e9a7b90a789e5c)
[23884279947] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[24175802244] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[24450233511] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[24459413616] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=795 watches=19 history=1024 journal=1024 symbols=363 drops=0
[24714368778] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[24984338907] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[25284074211] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[25584814662] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[25902513648] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[26153836269] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=829 watches=19 history=1024 journal=1024 symbols=364 drops=0
[26238289440] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[26535952410] 
```
</details>
